//! Lease-owned observation locks; records and admission remain the lease's.

use super::{encode_path, ensure_close_on_exec, hex_nonce, random_nonce, FileIdentity};
use anyhow::{bail, Context, Result};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
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
    started: bool,
}

impl LaunchWitnesses {
    pub(super) fn new(root: crate::TreeLifetime) -> Self {
        Self {
            private: None,
            root,
            path: None,
            started: false,
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

    /// Append only observation fields after mandatory activation. Admission
    /// ignores this namespace, including a partially written extension.
    pub(super) fn publish(
        &self,
        epoch: &mut impl Write,
        selected: &crate::Selection,
    ) -> Result<()> {
        let Some(private) = &self.private else {
            return Ok(());
        };
        let root = FileIdentity::from_metadata(&self.root.directory().metadata()?);
        let witness = FileIdentity::from_metadata(&private.metadata()?);
        let basename = self
            .path
            .as_ref()
            .and_then(|path| path.file_name())
            .context("prepared witness has no basename")?;
        // Hex text cannot inject mandatory fields. The enclosing epoch supplies
        // the nonce/signal binding; its exclusive guard spans this append.
        let extension = format!(
            "observation-version=1\nobservation-key={}\nobservation-handle-hex={}\nobservation-kind-hex={}\nobservation-tree-device={}\nobservation-tree-inode={}\nobservation-witness-name-hex={}\nobservation-witness-device={}\nobservation-witness-inode={}\n",
            selected.handle.key(),
            encode_path(Path::new(&selected.handle.to_string()))?,
            encode_path(Path::new(selected.kind.label()))?,
            root.device, root.inode,
            encode_path(Path::new(basename))?,
            witness.device, witness.inode,
        );
        epoch
            .write_all(extension.as_bytes())
            .context("appending observation extension")?;
        epoch.flush().context("flushing observation extension")
    }

    /// Attempt the sole marker publication once, even if a short write fails.
    /// Neither this callback nor failure releases either witness or takes a guard.
    pub(super) fn started(&mut self) -> Result<()> {
        if std::mem::replace(&mut self.started, true) {
            return Ok(());
        }
        if let Some(private) = &mut self.private {
            private
                .write_all(b"started\n")
                .context("publishing Started witness")?;
        }
        Ok(())
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
    use std::io::{BufRead, BufReader, Read};
    use std::os::unix::fs::PermissionsExt;
    use std::process::{Child, Command, Stdio};
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    const FOREIGN_PATH: &str = "GROVE_TEST_FOREIGN_WITNESS_PATH";
    const WAIT: Duration = Duration::from_secs(10);

    #[test]
    fn witness_foreign_shared_holder_process() {
        let Some(path) = std::env::var_os(FOREIGN_PATH) else {
            return;
        };
        let file = File::open(path).unwrap();
        assert_eq!(
            unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        println!("locked");
        std::io::stdout().flush().unwrap();
        let mut release = [0];
        std::io::stdin().read_exact(&mut release).unwrap();
        assert_eq!(release, [b'x']);
        drop(file);
    }

    struct ForeignHolder(Child);

    impl ForeignHolder {
        fn start(path: &Path) -> Self {
            let mut holder = Self(
                Command::new(std::env::current_exe().unwrap())
                    .args([
                        "--exact",
                        "driver_lease::witnesses::tests::witness_foreign_shared_holder_process",
                        "--nocapture",
                    ])
                    .env(FOREIGN_PATH, path)
                    .env_remove("GROVE_SIGNAL_FILE")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap(),
            );
            let stdout = holder.0.stdout.take().unwrap();
            let (ready_tx, ready_rx) = mpsc::channel();
            thread::spawn(move || {
                for line in BufReader::new(stdout).lines() {
                    if line.unwrap() == "locked" {
                        ready_tx.send(()).unwrap();
                    }
                }
            });
            ready_rx
                .recv_timeout(WAIT)
                .expect("foreign shared lock readiness");
            holder
        }

        fn release_and_reap(&mut self) {
            self.0.stdin.take().unwrap().write_all(b"x").unwrap();
            let deadline = Instant::now() + WAIT;
            loop {
                if let Some(status) = self.0.try_wait().unwrap() {
                    assert!(status.success(), "foreign holder failed: {status}");
                    return;
                }
                assert!(Instant::now() < deadline, "foreign holder did not exit");
                thread::yield_now();
            }
        }
    }

    impl Drop for ForeignHolder {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    #[test]
    fn witness_foreign_directory_holder_preserves_launch_and_admission() {
        foreign_holder_preserves_launch(false);
    }

    #[test]
    fn witness_foreign_private_holder_preserves_launch_and_admission() {
        foreign_holder_preserves_launch(true);
    }

    fn foreign_holder_preserves_launch(private: bool) {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        use super::super::{
            acquire_epoch_file, admit_session, tests::witness_selection, DriverLease, LockMode,
        };
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let workspace = jj_workspace::Workspace::resolve(temp.path()).unwrap();
        let mut lease = DriverLease::acquire(&workspace).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let channel = keyed_launch::Channel::allocate(lease.control_dir()).unwrap();
        let signal = channel.path().to_path_buf();
        let directory = temp.path().join(".grove");
        let mut holder = (!private).then(|| ForeignHolder::start(&directory));
        let (path_tx, path_rx) = mpsc::channel();
        let (ready_tx, ready_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            let mut preparation_error = None;
            let result = lease.prepare_launch_using(
                root,
                &signal,
                |path| acquire_epoch_file(path, LockMode::Exclusive, "foreign holder test"),
                |launch, control| {
                    let result = launch.prepare_with(
                        control,
                        random_nonce,
                        |path, _| {
                            assert!(private, "directory contention must prevent allocation");
                            path_tx.send(path.to_path_buf()).unwrap();
                            ready_rx.recv_timeout(WAIT).unwrap();
                            Ok(())
                        },
                        lock,
                    );
                    preparation_error = result.as_ref().err().map(|error| format!("{error:#}"));
                    result
                },
                |launch, epoch| launch.publish(epoch, &witness_selection()),
            );
            done_tx.send((lease, result, preparation_error)).unwrap();
        });
        let target = if private {
            let path = path_rx
                .recv_timeout(WAIT)
                .expect("private witness allocation");
            holder = Some(ForeignHolder::start(&path));
            path
        } else {
            directory.clone()
        };
        let probe = File::open(&target).unwrap();
        assert!(
            shared(&probe),
            "foreign shared locks permit shared observation"
        );
        // Keep the independent probe nonblocking even if production locking regresses.
        let assert_foreign_lock = || {
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                -1
            );
            assert!(matches!(std::io::Error::last_os_error().raw_os_error(),
                Some(code) if code == libc::EAGAIN || code == libc::EWOULDBLOCK));
        };
        assert_foreign_lock();
        if private {
            // Preparation already holds the directory, without blocking mutations
            // guarded by its containing directory.
            assert!(!shared(&File::open(&directory).unwrap()));
            lock(&File::open(temp.path()).unwrap()).unwrap();
            ready_tx.send(()).unwrap();
        }
        let (mut lease, result, error) = done_rx
            .recv_timeout(WAIT)
            .expect("preparation must finish before the foreign holder releases");
        worker.join().unwrap();
        result.unwrap();
        let error = error.expect("foreign holder must defeat preparation");
        assert!(
            error.contains(if private {
                "locking private witness"
            } else {
                "locking task-root witness"
            }),
            "{error}"
        );
        assert!(lease.launch.as_ref().unwrap().private.is_none());
        assert!(lease.launch.as_ref().unwrap().path().is_none());
        assert!(
            shared(&File::open(&directory).unwrap()),
            "partial directory lock rolled back"
        );
        lock(&File::open(temp.path()).unwrap()).unwrap();

        let config = temp.path().join("launch.kdl");
        fs::write(&config, "test \"/bin/sh -c 'echo launched > proof'\"\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        let mut events = Vec::new();
        lease
            .supervise_launch(|observer| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(temp.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    &mut |event| {
                        observer(event);
                        events.push(event);
                        assert!(admit_session(
                            temp.path(),
                            "test",
                            Some(channel.path().to_path_buf())
                        )
                        .is_ok());
                        assert!(matches!(
                            crate::try_observe(temp.path(), &[None]).activity,
                            crate::ActivityObservation::Unavailable(_)
                        ));
                        assert!(shared(&probe));
                        assert_foreign_lock();
                    },
                )
            })
            .unwrap();
        assert_eq!(
            events,
            [
                keyed_launch::LaunchEvent::Started,
                keyed_launch::LaunchEvent::Reaped
            ]
        );
        assert_eq!(fs::read(temp.path().join("proof")).unwrap(), b"launched\n");
        assert!(lease.launch.is_none());
        holder.as_mut().unwrap().release_and_reap();
        lock(&probe).unwrap();
        drop(probe);
        lock(&File::open(&directory).unwrap()).unwrap();
        lease.invalidate_session_epoch().unwrap();
        if private {
            assert!(
                !target.exists(),
                "invalidation cleans the abandoned private file"
            );
        }
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &witness_selection(), channel.path())
            .unwrap();
        assert!(
            lease.launch.as_ref().unwrap().private.is_some(),
            "preparation recovers after reap"
        );
        drop(lease);
        channel.discard().unwrap();
    }

    #[test]
    fn witnessed_started_is_exact_and_never_retried_after_success_or_failure() {
        let (_temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        owner.started().unwrap();
        owner.started().unwrap();
        assert_eq!(fs::read(owner.path().unwrap()).unwrap(), b"started\n");

        let (_temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        let path = owner.path().unwrap().to_path_buf();
        // Read-only descriptor injects a real write failure without unsafe fd reuse.
        owner.private = Some(File::open(&path).unwrap());
        assert!(owner.started().is_err());
        owner.private = Some(OpenOptions::new().write(true).open(&path).unwrap());
        owner.started().unwrap();
        assert!(
            fs::read(path).unwrap().is_empty(),
            "failed publication was retried"
        );
    }

    #[test]
    fn witnessed_marker_failure_preserves_real_launch_and_both_locks_until_reap() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        use super::super::{tests::witness_selection, DriverLease, EPOCH_FILE_NAME};
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let workspace = jj_workspace::Workspace::resolve(temp.path()).unwrap();
        let mut lease = DriverLease::acquire(&workspace).unwrap();
        let channel = keyed_launch::Channel::allocate(lease.control_dir()).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &witness_selection(), channel.path())
            .unwrap();
        let owner = lease.launch.as_mut().unwrap();
        let path = owner.path().unwrap().to_path_buf();
        owner.private = Some(File::open(&path).unwrap());
        lock(owner.private.as_ref().unwrap()).unwrap();
        let private = File::open(&path).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        let epoch_path = lease.control_dir().join(EPOCH_FILE_NAME);
        let config = temp.path().join("launch.kdl");
        fs::write(&config, "test \"/bin/sh -c true\"\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        let mut events = Vec::new();
        let result = lease.supervise_launch(|observer| {
            keyed_launch::run_observed(
                keyed_launch::Launch {
                    argv: &argv,
                    channel: &channel,
                    channel_var: "GROVE_SIGNAL_FILE",
                    scrub: &[],
                    cwd: Some(temp.path()),
                    escalation: keyed_launch::Escalation {
                        grace: std::time::Duration::ZERO,
                        kill_grace: std::time::Duration::ZERO,
                    },
                },
                &mut |event| {
                    observer(event);
                    events.push(event);
                    assert!(fs::read(&path).unwrap().is_empty());
                    let reaped = event == keyed_launch::LaunchEvent::Reaped;
                    assert_eq!(shared(&private), reaped);
                    assert_eq!(shared(&directory), reaped);
                    // Callbacks must have neither epoch nor containing-tree guard.
                    lock(&File::open(&epoch_path).unwrap()).unwrap();
                    lock(&File::open(temp.path()).unwrap()).unwrap();
                    assert!(super::super::admit_session(
                        temp.path(),
                        "test",
                        Some(channel.path().to_path_buf())
                    )
                    .is_ok());
                },
            )
        });
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(
            events,
            [
                keyed_launch::LaunchEvent::Started,
                keyed_launch::LaunchEvent::Reaped
            ]
        );
        assert!(lease.launch.is_none());
    }

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
