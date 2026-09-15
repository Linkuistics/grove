use std::collections::HashSet;
use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use grove_loop::{entry_path, Handle, Kind, Outcome, Parts, Reading, TryReading};

#[derive(PartialEq, Eq)]
pub(crate) struct Row {
    pub key: Item,
    pub path: PathBuf,
    pub handle: Option<Handle>,
    pub kind: Option<Kind>,
    pub lifecycle: Lifecycle,
    pub depth: usize,
    pub branch: bool,
    pub expanded: bool,
    pub counts: [usize; 3],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Lifecycle {
    Live,
    Done,
    Abandoned,
    Empty,
}

/// Root has its own identity, separate from permanent task keys.
pub(crate) type Item = Option<u32>;

/// A retained descriptor prevents inode reuse while an old lifetime is visible.
/// It never carries Grove's tree lock.
pub(crate) struct Root(File);

impl Root {
    pub fn open(worktree: &Path) -> std::io::Result<Option<Self>> {
        use rustix::fs::{open, Mode, OFlags};
        // Reject a raced-in nondirectory (including FIFOs) without blocking.
        // https://docs.rs/crate/rustix/0.38.44/source/src/fs/abs.rs
        match open(
            worktree.join(".grove"),
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NONBLOCK | OFlags::CLOEXEC,
            Mode::empty(),
        ) {
            Ok(fd) => Ok(Some(Self(File::from(fd)))),
            Err(rustix::io::Errno::NOENT | rustix::io::Errno::NOTDIR) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn at(&self, worktree: &Path) -> std::io::Result<bool> {
        let current = match std::fs::metadata(worktree.join(".grove")) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error),
        };
        let old = self.0.metadata()?;
        Ok(current.is_dir() && old.dev() == current.dev() && old.ino() == current.ino())
    }

    pub fn same(&self, other: &Self) -> std::io::Result<bool> {
        let a = self.0.metadata()?;
        let b = other.0.metadata()?;
        // Device + inode identify an open object on supported Unix targets.
        // https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html
        Ok(a.dev() == b.dev() && a.ino() == b.ino())
    }
}

type Content = Result<Vec<u8>, String>;
#[derive(PartialEq, Eq)]
pub(crate) enum Observation<T> {
    Ready(T),
    Vacant,
    Busy,
}

/// Two bounded captures detect visible non-cooperating edits without claiming
/// atomicity. Each drops its guard before another acquisition. Never loop until
/// stable here: busy/changing trees must leave input and quit responsive.
pub(crate) fn capture(
    worktree: &Path,
    candidates: &[Item],
) -> Result<Observation<(Vec<Row>, usize, Content)>> {
    let first = capture_once(worktree, candidates)?;
    if !matches!(first, Observation::Ready(_)) {
        return Ok(first);
    }
    let second = capture_once(worktree, candidates)?;
    // Contention on the verification read is still Busy, not malformed data.
    if !matches!(second, Observation::Ready(_)) {
        return Ok(second);
    }
    anyhow::ensure!(first == second, "tree changed during observation; retrying");
    Ok(second)
}

/// Copy rows and selected bytes while guarded. No guard escapes this function.
fn capture_once(
    worktree: &Path,
    candidates: &[Item],
) -> Result<Observation<(Vec<Row>, usize, Content)>> {
    let tree = match grove_loop::try_read(worktree)? {
        TryReading::Busy => return Ok(Observation::Busy),
        TryReading::Ready(Reading::Vacant) => return Ok(Observation::Vacant),
        TryReading::Ready(Reading::Tree(tree)) => tree,
    };
    let root = tree.snapshot().root();
    let brief = root.distinguished().context("root has no brief")?;
    let path = entry_path(tree.root(), brief);

    let mut keys = HashSet::new();
    let mut rows = vec![Row {
        key: None,
        path,
        handle: None,
        kind: None,
        lifecycle: Lifecycle::Empty,
        depth: 0,
        branch: true,
        expanded: true,
        counts: [0; 3],
    }];
    for entry in tree.walk() {
        let Some(triple) = entry.triple() else {
            continue;
        };
        anyhow::ensure!(
            keys.insert(triple.key.get()),
            "duplicate key k{}",
            triple.key
        );
        let (path, handle, kind, counts, branch) = match triple.parts {
            Parts::Leaf { outcome, kind, .. } => {
                let handle = Handle::of_leaf(entry.name()).context("leaf has no handle")?;
                let index = match outcome {
                    Outcome::Live => 0,
                    Outcome::Done => 1,
                    Outcome::Abandoned => 2,
                };
                let mut counts = [0; 3];
                counts[index] = 1;
                (
                    entry_path(tree.root(), entry),
                    handle,
                    Some(kind.clone()),
                    counts,
                    false,
                )
            }
            Parts::Node => {
                let brief = entry
                    .contents()
                    .and_then(|level| level.distinguished())
                    .context("branch has no brief")?;
                let handle =
                    Handle::of_node(entry.name(), brief.name()).context("branch has no handle")?;
                (entry_path(tree.root(), brief), handle, None, [0; 3], true)
            }
        };
        rows.push(Row {
            key: Some(triple.key.get()),
            path,
            handle: Some(handle),
            kind,
            lifecycle: Lifecycle::Empty,
            depth: entry.depth(),
            branch,
            expanded: true,
            counts,
        });
    }
    // Reverse pre-order folds each subtree into its immediate parent, including
    // collapsed descendants and empty branches (which contribute no leaves).
    let mut parents = vec![0];
    let mut parent_of = vec![0; rows.len()];
    for i in 1..rows.len() {
        while parents.len() > rows[i].depth {
            parents.pop();
        }
        parent_of[i] = *parents.last().context("entry has no parent")?;
        if rows[i].branch {
            parents.push(i);
        }
    }
    for i in (1..rows.len()).rev() {
        let counts = rows[i].counts;
        for (n, count) in counts.into_iter().enumerate() {
            rows[parent_of[i]].counts[n] += count;
        }
    }
    for row in &mut rows {
        let [live, done, abandoned] = row.counts;
        row.lifecycle = if live > 0 {
            Lifecycle::Live
        } else if done > 0 {
            Lifecycle::Done
        } else if abandoned > 0 {
            Lifecycle::Abandoned
        } else {
            Lifecycle::Empty
        };
    }
    let selected = candidates
        .iter()
        .find_map(|key| rows.iter().position(|row| row.key == *key))
        .unwrap_or(0);
    let content = read_file(&rows[selected].path);
    Ok(Observation::Ready((rows, selected, content)))
}

fn read_file(path: &Path) -> Content {
    std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))
}

/// Never put control bytes from names, paths, diagnostics or file contents on
/// the terminal. Keep source newlines, render tabs as spaces and controls inert.
pub(crate) fn safe_text(text: &str) -> String {
    text.chars()
        .flat_map(|ch| match ch {
            '\n' => "\n".chars().collect::<Vec<_>>(),
            '\t' => "    ".chars().collect(),
            ch if ch.is_control() => vec!['�'],
            ch => vec![ch],
        })
        .collect()
}
