//! Lease-owned observation locks; records and admission remain the lease's.

use super::{ensure_close_on_exec, hex_nonce, random_nonce};
use anyhow::{bail, Context, Result};
use std::fs::{self, File, OpenOptions};
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

// Same accepted collision bound as keyed-launch's completion channel.
const DRAW_RETRY_LIMIT: usize = 8;
const PREFIX: &str = "witness-";

#[derive(Debug)]
pub(super) struct LaunchWitnesses {
    private: Option<File>,
    pub(super) root: crate::TreeLifetime,
    path: Option<PathBuf>,
}

impl LaunchWitnesses {
    pub(super) fn new(root: crate::TreeLifetime) -> Self {
        Self {
            private: None,
            root,
            path: None,
        }
    }

    pub(super) fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Called only under the exclusive, already-invalidated epoch.
    pub(super) fn prepare(&mut self, directory: &Path) -> Result<()> {
        self.prepare_with(directory, random_nonce, |_, _| Ok(()), lock)
    }

    fn prepare_with(
        &mut self,
        directory: &Path,
        mut draw: impl FnMut() -> Result<[u8; 16]>,
        mut after_create: impl FnMut(&Path, &File) -> Result<()>,
        mut acquire: impl FnMut(&File) -> Result<()>,
    ) -> Result<()> {
        acquire(self.root.directory()).context("locking task-root witness")?;
        let result = (|| {
            for _ in 0..DRAW_RETRY_LIMIT {
                let path = directory.join(format!("{PREFIX}{}", hex_nonce(draw()?)?));
                // Atomic exclusive creation also refuses an existing symlink.
                // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
                let file = match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .custom_flags(libc::O_CLOEXEC | libc::O_NONBLOCK)
                    .open(&path)
                {
                    Ok(file) => file,
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(error) => return Err(error).context("allocating private witness"),
                };
                ensure_close_on_exec(file.as_raw_fd())?;
                after_create(&path, &file)?;
                acquire(&file).context("locking private witness")?;
                self.private = Some(file);
                self.path = Some(path);
                return Ok(());
            }
            bail!("could not allocate private witness after {DRAW_RETRY_LIMIT} occupied draws")
        })();
        if result.is_err() {
            // The local private descriptor has closed. Keep the selected pin,
            // but release its observation lock on partial-setup failure.
            if unsafe { libc::flock(self.root.directory().as_raw_fd(), libc::LOCK_UN) } != 0 {
                return Err(std::io::Error::last_os_error())
                    .context("releasing partial task-root witness");
            }
        }
        result
    }

    fn close_private(&mut self) {
        self.private.take();
    }
}

impl Drop for LaunchWitnesses {
    fn drop(&mut self) {
        // Drop runs before field destructors, including the root descriptor.
        // https://doc.rust-lang.org/reference/destructors.html#destructors.operation
        self.close_private();
    }
}

fn lock(file: &File) -> Result<()> {
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error()).context("acquiring nonblocking exclusive witness")
    }
}

/// Only the driver calls this, after invalidation. Never remove its unreaped file.
pub(super) fn discard_abandoned(directory: &Path, retained: Option<&Path>) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let name = entry.file_name();
        let Some(suffix) = name.to_str().and_then(|name| name.strip_prefix(PREFIX)) else {
            continue;
        };
        if suffix.len() == 32
            && suffix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            && retained != Some(entry.path().as_path())
        {
            fs::remove_file(entry.path()).context("removing abandoned private witness")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::TempDir;

    fn fixture() -> (TempDir, LaunchWitnesses, PathBuf) {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let control = temp.path().join("control");
        fs::create_dir(&control).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        (temp, LaunchWitnesses::new(root), control)
    }

    fn shared(file: &File) -> bool {
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) } == 0 {
            assert_eq!(unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) }, 0);
            true
        } else {
            assert!(matches!(std::io::Error::last_os_error().raw_os_error(),
                Some(code) if code == libc::EAGAIN || code == libc::EWOULDBLOCK));
            false
        }
    }

    #[test]
    fn paired_owner_closes_private_before_root_and_sets_exec_flags() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        let private = File::open(owner.path().unwrap()).unwrap();
        assert!(!shared(&directory));
        assert!(!shared(&private));
        for descriptor in [owner.root.directory(), owner.private.as_ref().unwrap()] {
            let flags = unsafe { libc::fcntl(descriptor.as_raw_fd(), libc::F_GETFD) };
            assert_ne!(flags, -1);
            assert_ne!(flags & libc::FD_CLOEXEC, 0);
        }
        assert_eq!(
            private.metadata().unwrap().permissions().mode() & 0o777,
            0o600
        );
        // This is the operation Drop invokes before Rust drops the root field.
        owner.close_private();
        assert!(shared(&private));
        assert!(!shared(&directory));
        drop(owner);
        assert!(shared(&directory));
    }

    #[test]
    fn paired_owner_retries_only_occupied_draws_and_never_truncates() {
        let (_temp, mut owner, control) = fixture();
        let occupied = control.join(format!("{PREFIX}{}", "00".repeat(16)));
        fs::write(&occupied, "occupied").unwrap();
        let mut draws = 0;
        owner
            .prepare_with(
                &control,
                || {
                    draws += 1;
                    Ok([u8::from(draws == 3); 16])
                },
                |_, _| Ok(()),
                lock,
            )
            .unwrap();
        assert_eq!(draws, 3);
        assert_eq!(fs::read(&occupied).unwrap(), b"occupied");
        assert!(fs::read(owner.path().unwrap()).unwrap().is_empty());
    }

    #[test]
    fn paired_owner_exhaustion_and_allocation_errors_release_directory() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for failure in ["collision", "random", "open"] {
            let (temp, mut owner, control) = fixture();
            let occupied = control.join(format!("{PREFIX}{}", "00".repeat(16)));
            fs::write(&occupied, "preserve").unwrap();
            let target = if failure == "open" {
                occupied.clone()
            } else {
                control
            };
            let mut draws = 0;
            let result = owner.prepare_with(
                &target,
                || {
                    draws += 1;
                    if failure == "random" {
                        bail!("injected random source failure");
                    }
                    Ok([0; 16])
                },
                |_, _| Ok(()),
                lock,
            );
            assert!(result.is_err(), "{failure}");
            assert_eq!(draws, if failure == "collision" { 8 } else { 1 });
            assert!(owner.private.is_none());
            assert!(shared(&File::open(temp.path().join(".grove")).unwrap()));
            assert_eq!(fs::read(&occupied).unwrap(), b"preserve");
        }
    }

    #[test]
    fn paired_owner_foreign_holders_and_lock_errors_roll_back() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for failure in [
            "directory-holder",
            "private-holder",
            "directory-error",
            "private-error",
        ] {
            let (temp, mut owner, control) = fixture();
            let directory = File::open(temp.path().join(".grove")).unwrap();
            if failure == "directory-holder" {
                assert_eq!(
                    unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                    0
                );
            }
            let mut foreign = None;
            let mut locks = 0;
            let result = owner.prepare_with(
                &control,
                random_nonce,
                |path, _| {
                    if failure == "private-holder" {
                        let file = File::open(path)?;
                        assert_eq!(
                            unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                            0
                        );
                        foreign = Some(file);
                    }
                    Ok(())
                },
                |file| {
                    locks += 1;
                    if (failure == "directory-error" && locks == 1)
                        || (failure == "private-error" && locks == 2)
                    {
                        bail!("injected reported lock failure");
                    }
                    lock(file)
                },
            );
            assert!(result.is_err(), "{failure}");
            assert!(owner.private.is_none());
            drop(foreign);
            assert!(shared(&directory));
            lock(&directory).unwrap();
            for entry in fs::read_dir(&control).unwrap() {
                assert!(shared(&File::open(entry.unwrap().path()).unwrap()));
            }
        }
    }

    #[test]
    fn paired_owner_cleanup_recognizes_only_its_names_and_preserves_retained() {
        let (_temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        let retained = owner.path().unwrap().to_path_buf();
        let abandoned = control.join(format!("{PREFIX}{}", "ff".repeat(16)));
        fs::write(&abandoned, "old").unwrap();
        for name in [
            "witness-short",
            "witness-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "driver.lease",
        ] {
            fs::write(control.join(name), "untouched").unwrap();
        }
        discard_abandoned(&control, Some(&retained)).unwrap();
        assert!(retained.exists());
        assert!(!abandoned.exists());
        assert_eq!(fs::read_dir(&control).unwrap().count(), 4);
        drop(owner);
        discard_abandoned(&control, None).unwrap();
        assert!(!retained.exists());
        assert_eq!(fs::read_dir(&control).unwrap().count(), 3);
    }
}
