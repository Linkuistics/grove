use anyhow::{bail, Context, Result};

use crate::task_name::TaskNameError;
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

#[cfg(test)]
thread_local! {
    static ACQUISITION_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

// The three transaction sentinels. `pub(crate)` rather than private because
// `task_name` classifies them as `Verdict::Reserved` and a token spelled in two
// modules is a token that can drift in one of them.
pub(crate) const FINISHING_PREFIX: &str = "FINISHING-";
pub(crate) const PREPARING_FINISH_PREFIX: &str = "PREPARING-FINISH-";
pub(crate) const MIGRATION_TRANSACTION: &str = "MIGRATING-session-kinds";

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
    crate::tree_format::require_current(&root)?;
    Ok(TreeReadGuard {
        _worktree_directory: worktree_directory,
        root,
    })
}

pub fn write(grove_root: &Path) -> Result<TreeWriteGuard> {
    let (root, worktree_directory) = acquire(grove_root, libc::LOCK_EX)?;
    require_grove_root(&root)?;
    refuse_pending(&root)?;
    crate::tree_format::require_current(&root)?;
    Ok(TreeWriteGuard {
        _worktree_directory: worktree_directory,
        root,
    })
}

/// Session-kind migration owns the exclusive tree lock across planning,
/// recovery, landing, and commit. It must admit both a legacy tree and its own
/// pending witness, so the caller owns those validations while this guard lives.
#[cfg(test)]
pub(crate) fn write_for_migration(grove_root: &Path) -> Result<TreeWriteGuard> {
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

pub(crate) fn refuse_pending(grove_root: &Path) -> Result<()> {
    refuse_pending_finish(grove_root)?;
    refuse_pending_migration(grove_root)
}

pub(crate) fn refuse_pending_finish(grove_root: &Path) -> Result<()> {
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

pub(crate) fn refuse_pending_migration(grove_root: &Path) -> Result<()> {
    let migration = grove_root.join(MIGRATION_TRANSACTION);
    match fs::symlink_metadata(&migration) {
        // The domain's wording, for the reason `refuse_pending_finish` gives.
        Ok(_) => bail!(TaskNameError::PendingMigration {
            name: migration.display().to_string()
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error)
            .with_context(|| format!("checking migration witness {}", migration.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_refuses_a_pending_session_kind_migration_before_format_validation() {
        let worktree = tempfile::tempdir().unwrap();
        let grove_root = worktree.path().join(".grove");
        let witness = grove_root.join("MIGRATING-session-kinds");
        fs::create_dir_all(&witness).unwrap();

        let error = match read(&grove_root) {
            Ok(_) => panic!("reader admitted a pending migration"),
            Err(error) => error,
        };
        let diagnostic = error.to_string();

        assert!(diagnostic.contains("pending Grove session-kind migration"));
        assert!(diagnostic.contains(&witness.display().to_string()));
        assert!(diagnostic.contains("rerun bare `grove`"));
    }

    #[test]
    fn reader_refuses_a_malformed_session_kind_migration_witness() {
        let worktree = tempfile::tempdir().unwrap();
        let grove_root = worktree.path().join(".grove");
        fs::create_dir(&grove_root).unwrap();
        fs::write(grove_root.join(MIGRATION_TRANSACTION), "not a directory\n").unwrap();

        let error = match read(&grove_root) {
            Ok(_) => panic!("reader admitted a malformed migration witness"),
            Err(error) => error,
        };

        assert!(error
            .to_string()
            .contains("pending Grove session-kind migration"));
    }

    #[test]
    fn migration_writer_admits_its_pending_legacy_transaction() {
        let worktree = tempfile::tempdir().unwrap();
        let grove_root = worktree.path().join(".grove");
        fs::create_dir_all(grove_root.join(MIGRATION_TRANSACTION)).unwrap();

        let guard = write_for_migration(&grove_root).unwrap();

        assert_eq!(guard.root(), grove_root);
    }

    #[test]
    fn writer_refuses_a_pending_session_kind_migration() {
        let worktree = tempfile::tempdir().unwrap();
        let grove_root = worktree.path().join(".grove");
        fs::create_dir_all(grove_root.join(MIGRATION_TRANSACTION)).unwrap();

        let error = match write(&grove_root) {
            Ok(_) => panic!("the mutator admitted a pending migration"),
            Err(error) => error,
        };

        assert!(error
            .to_string()
            .contains("pending Grove session-kind migration"));
    }

    #[test]
    fn reader_refuses_a_preparing_finish_before_format_validation() {
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
