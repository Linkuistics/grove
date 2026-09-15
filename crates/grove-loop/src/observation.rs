//! One captured tree and selected file, with no advisory guard returned.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context};
use ordinal_fs_tree::Snapshot;

use crate::{entry_path, Error, Parts, Reading, TaskName, TryReading};

/// A retained directory descriptor prevents inode reuse while a capture lives.
/// This value holds no tree lock, epoch guard or driver authority.
#[derive(Debug)]
pub struct TreeLifetime(File);

impl TreeLifetime {
    pub(crate) fn directory(&self) -> &File {
        &self.0
    }

    /// Open the exact task-root directory without creating or locking anything.
    ///
    /// # Errors
    /// An existing directory cannot be opened. Missing/nondirectory roots are None.
    pub fn open(worktree: &Path) -> Result<Option<Self>, Error> {
        // custom_flags adds OS flags without changing read-only access.
        // https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.custom_flags
        match OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NONBLOCK | libc::O_CLOEXEC)
            .open(worktree.join(".grove"))
        {
            Ok(file) => Ok(Some(Self(file))),
            Err(error) if matches!(error.raw_os_error(), Some(libc::ENOENT | libc::ENOTDIR)) => {
                Ok(None)
            }
            Err(error) => Err(anyhow::Error::new(error).into()),
        }
    }

    /// Whether this retained directory still occupies the observed task-root path.
    ///
    /// # Errors
    /// Metadata cannot be read for the descriptor or current path.
    pub fn at(&self, worktree: &Path) -> Result<bool, Error> {
        let current = match std::fs::metadata(worktree.join(".grove")) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(anyhow::Error::new(error).into()),
        };
        let old = self.0.metadata().map_err(anyhow::Error::new)?;
        Ok(current.is_dir() && old.dev() == current.dev() && old.ino() == current.ino())
    }

    /// Compare two still-pinned directory identities, independent of path spelling.
    ///
    /// # Errors
    /// Metadata cannot be read for either descriptor.
    pub fn same(&self, other: &Self) -> Result<bool, Error> {
        let a = self.0.metadata().map_err(anyhow::Error::new)?;
        let b = other.0.metadata().map_err(anyhow::Error::new)?;
        Ok(a.dev() == b.dev() && a.ino() == b.ino())
    }
}

/// The tree portion of one quiet observation. Errors remain a separate Result.
pub enum TreeObservation {
    Ready(CapturedTree),
    Vacant,
    Busy,
}

/// Names and selected bytes copied under the tree guard, then released from it.
/// The snapshot's paths use the caller's spelling. The lifetime pins that capture.
pub struct CapturedTree {
    root: PathBuf,
    snapshot: Snapshot<TaskName>,
    pub lifetime: TreeLifetime,
    /// First surviving requested key, or the root (None).
    pub selected: Option<u32>,
    /// A file failure does not discard a readable tree.
    pub content: Result<Vec<u8>, String>,
}

impl CapturedTree {
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub fn snapshot(&self) -> &Snapshot<TaskName> {
        &self.snapshot
    }
}

/// Runtime evidence at this sample. Legacy active records cannot identify a mandate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivityObservation {
    Idle,
    Busy(String),
    Unavailable(String),
}

/// Independent results, captured values and a tree pin; no advisory lock escapes.
pub struct ObservationGuard {
    pub tree: Result<TreeObservation, Error>,
    pub activity: ActivityObservation,
}

/// Capture the tree and selected bytes, release its guard, then sample runtime.
/// Candidate keys are ordered by preference; None requests the root brief.
/// Runtime failures preserve the tree. Neither operation grants session authority.
pub fn try_observe(worktree: &Path, candidates: &[Option<u32>]) -> ObservationGuard {
    observe_with(worktree, candidates, || {}, || {})
}

pub(crate) fn observe_with(
    worktree: &Path,
    candidates: &[Option<u32>],
    after_capture: impl FnOnce(),
    in_epoch: impl FnMut(),
) -> ObservationGuard {
    let tree = capture(worktree, candidates).map_err(Error::from);
    after_capture();
    let activity = crate::driver_lease::observation::observe(worktree, in_epoch);
    ObservationGuard { tree, activity }
}

fn capture(worktree: &Path, candidates: &[Option<u32>]) -> anyhow::Result<TreeObservation> {
    // Pin before reading so a changed root cannot be attached to old names.
    let lifetime = TreeLifetime::open(worktree)?;
    let tree = match crate::try_read(worktree)? {
        TryReading::Busy => return Ok(TreeObservation::Busy),
        TryReading::Ready(Reading::Vacant) => return Ok(TreeObservation::Vacant),
        TryReading::Ready(Reading::Tree(tree)) => tree,
    };
    let lifetime = lifetime.context("tree changed during observation; retrying")?;
    ensure!(
        lifetime.at(worktree)?,
        "tree changed during observation; retrying"
    );
    crate::select_snapshot(tree.root(), tree.snapshot(), None)?;
    let brief = tree
        .snapshot()
        .root()
        .distinguished()
        .context("root has no brief")?;
    let mut files = vec![(None, entry_path(tree.root(), brief))];
    for entry in tree.walk() {
        let Some(triple) = entry.triple() else {
            continue;
        };
        let file = match triple.parts {
            Parts::Leaf { .. } => entry,
            Parts::Node => entry
                .contents()
                .and_then(|level| level.distinguished())
                .context("branch has no brief")?,
        };
        files.push((Some(triple.key.get()), entry_path(tree.root(), file)));
    }
    let (selected, path) = candidates
        .iter()
        .find_map(|key| files.iter().find(|(candidate, _)| candidate == key))
        .unwrap_or(&files[0]);
    let content = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()));
    ensure!(
        lifetime.at(worktree)?,
        "tree changed during observation; retrying"
    );
    let root = tree.root().to_path_buf();
    Ok(TreeObservation::Ready(CapturedTree {
        root,
        snapshot: tree.into_snapshot(),
        lifetime,
        selected: *selected,
        content,
    }))
}
