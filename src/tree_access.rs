use anyhow::{bail, Context, Result};

use crate::task_name::TaskNameError;
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

#[cfg(test)]
thread_local! {
    static ACQUISITION_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

// The two transaction sentinels. `pub(crate)` rather than private because
// `task_name` classifies them as `Verdict::Reserved` and a token spelled in two
// modules is a token that can drift in one of them.
pub(crate) const FINISHING_PREFIX: &str = "FINISHING-";
pub(crate) const PREPARING_FINISH_PREFIX: &str = "PREPARING-FINISH-";

pub struct TreeReadGuard {
    _worktree_directory: File,
    root: PathBuf,
}

pub struct TreeWriteGuard {
    _worktree_directory: File,
    root: PathBuf,
}

pub(crate) struct LifecycleWriteGuard {
    _worktree_directory: File,
}

impl TreeReadGuard {
    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl TreeWriteGuard {
    pub fn root(&self) -> &Path {
        &self.root
    }
}

pub fn read(grove_root: &Path) -> Result<TreeReadGuard> {
    let (root, worktree_directory) = acquire(grove_root, libc::LOCK_SH)?;
    require_grove_root(&root)?;
    refuse_pending(&root)?;
    Ok(TreeReadGuard {
        _worktree_directory: worktree_directory,
        root,
    })
}

pub fn write(grove_root: &Path) -> Result<TreeWriteGuard> {
    let (root, worktree_directory) = acquire(grove_root, libc::LOCK_EX)?;
    require_grove_root(&root)?;
    refuse_pending(&root)?;
    Ok(TreeWriteGuard {
        _worktree_directory: worktree_directory,
        root,
    })
}

/// Lifecycle transitions take the ordinary exclusive tree lock before `.grove/`
/// necessarily exists. The caller owns root classification and every mutation
/// performed while this guard lives.
pub(crate) fn write_for_lifecycle(worktree: &Path) -> Result<LifecycleWriteGuard> {
    let worktree_directory = acquire_worktree(worktree, libc::LOCK_EX)?;
    Ok(LifecycleWriteGuard {
        _worktree_directory: worktree_directory,
    })
}

fn acquire(grove_root: &Path, operation: libc::c_int) -> Result<(PathBuf, File)> {
    let worktree = grove_root.parent().with_context(|| {
        format!(
            "grove root {} has no working-tree parent",
            grove_root.display()
        )
    })?;
    let worktree_directory = acquire_worktree(worktree, operation)?;
    // Keep the caller's spelling for returned paths. On macOS `/var` and
    // `/private/var` name the same inode; canonicalising here would make adding
    // locking observably rewrite every `pick` path even though the descriptor
    // lock itself already follows the filesystem identity.
    let root = grove_root.to_path_buf();
    Ok((root, worktree_directory))
}

fn acquire_worktree(worktree: &Path, operation: libc::c_int) -> Result<File> {
    #[cfg(test)]
    ACQUISITION_COUNT.with(|count| count.set(count.get() + 1));

    if !worktree.is_dir() {
        bail!("working tree root not found: {}", worktree.display());
    }
    let worktree_directory = File::open(worktree).with_context(|| {
        format!(
            "opening working tree root {} for tree access",
            worktree.display()
        )
    })?;
    let descriptor = worktree_directory.as_raw_fd();
    let first = unsafe { libc::flock(descriptor, operation | libc::LOCK_NB) };
    if first != 0 {
        let error = std::io::Error::last_os_error();
        if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
        {
            eprintln!("waiting for active Grove tree operation");
            let waited = unsafe { libc::flock(descriptor, operation) };
            if waited != 0 {
                return Err(std::io::Error::last_os_error())
                    .context("waiting for the active Grove tree operation");
            }
        } else {
            return Err(error).context("locking the Grove working tree");
        }
    }
    Ok(worktree_directory)
}

#[cfg(test)]
pub(crate) fn reset_acquisition_count() {
    ACQUISITION_COUNT.with(|count| count.set(0));
}

#[cfg(test)]
pub(crate) fn acquisition_count() -> usize {
    ACQUISITION_COUNT.with(std::cell::Cell::get)
}

/// Assert the lock-neutral helper contract in unit tests: callers must already
/// hold either the shared or exclusive working-tree lock. A second non-blocking
/// exclusive acquisition succeeds only when no guard is live.
#[cfg(test)]
pub(crate) fn assert_guard_held(grove_root: &Path) {
    let worktree = grove_root
        .parent()
        .expect("a Grove root used by an unlocked helper must have a worktree parent");
    let directory = File::open(worktree).expect("opening worktree to verify the tree guard");
    let result = unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if result == 0 {
        unsafe {
            libc::flock(directory.as_raw_fd(), libc::LOCK_UN);
        }
        panic!("lock-neutral tree helper called without a live tree guard");
    }

    let error = std::io::Error::last_os_error();
    assert!(
        matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN),
        "checking for a live tree guard failed unexpectedly: {error}"
    );
}

fn require_grove_root(grove_root: &Path) -> Result<()> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    Ok(())
}

/// Refuse a tree held by the one transaction grove still runs — the finish
/// cycle's. Migration's witness was the other, and went with migration
/// (`delete-migration-k6`).
pub(crate) fn refuse_pending(grove_root: &Path) -> Result<()> {
    let mut entries = fs::read_dir(grove_root)
        .with_context(|| format!("reading task-tree root {}", grove_root.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    if let Some(witness) = entries.into_iter().find(|entry| {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        name.starts_with(FINISHING_PREFIX) || name.starts_with(PREPARING_FINISH_PREFIX)
    }) {
        // The wording is the **domain's**, not a second one written here.
        // `task_name` classifies these sentinels `Verdict::Reserved` carrying the
        // same error, so the library halting on one mid-tree and this pre-check
        // meeting one at the root say the same sentence. The `name` field is
        // whatever names the witness, and here that is its path: the library has
        // only the filename to give, and a reader who ran a verb from elsewhere
        // wants to know where it is.
        bail!(TaskNameError::PendingFinish {
            name: witness.path().display().to_string()
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_refuses_a_preparing_finish_witness() {
        let worktree = tempfile::tempdir().unwrap();
        let grove_root = worktree.path().join(".grove");
        let witness =
            grove_root.join("PREPARING-FINISH-finish-k2-11111111111111111111111111111111");
        fs::create_dir_all(&witness).unwrap();

        let error = match read(&grove_root) {
            Ok(_) => panic!("reader admitted a preparing finish witness"),
            Err(error) => error,
        };

        let diagnostic = error.to_string();
        assert!(diagnostic.contains("pending Grove finish transaction"));
        assert!(diagnostic.contains(&witness.display().to_string()));
    }

    #[test]
    fn worktree_lock_descriptor_is_close_on_exec() {
        let worktree = tempfile::tempdir().unwrap();
        let guard = write_for_lifecycle(worktree.path()).unwrap();
        let flags = unsafe { libc::fcntl(guard._worktree_directory.as_raw_fd(), libc::F_GETFD) };

        assert_ne!(flags, -1, "F_GETFD failed");
        assert_ne!(flags & libc::FD_CLOEXEC, 0, "tree lock leaked across exec");
    }
}
