use anyhow::{bail, Context, Result};

use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

#[cfg(test)]
thread_local! {
    static ACQUISITION_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

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
    Ok(TreeReadGuard {
        _worktree_directory: worktree_directory,
        root,
    })
}

pub fn write(grove_root: &Path) -> Result<TreeWriteGuard> {
    let (root, worktree_directory) = acquire(grove_root, libc::LOCK_EX)?;
    require_grove_root(&root)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worktree_lock_descriptor_is_close_on_exec() {
        let worktree = tempfile::tempdir().unwrap();
        let guard = write_for_lifecycle(worktree.path()).unwrap();
        let flags = unsafe { libc::fcntl(guard._worktree_directory.as_raw_fd(), libc::F_GETFD) };

        assert_ne!(flags, -1, "F_GETFD failed");
        assert_ne!(flags & libc::FD_CLOEXEC, 0, "tree lock leaked across exec");
    }
}
