//! Advisory runtime reads share admission's grammar, never its authority path.

use super::*;
use crate::ActivityObservation;

const RECORD_LIMIT: u64 = 64 * 1024;

pub(crate) fn observe(worktree: &Path, in_epoch: impl FnMut()) -> ActivityObservation {
    match read_runtime(worktree, in_epoch) {
        Ok(activity) => activity,
        Err(error) => ActivityObservation::Unavailable(format!("{error:#}")),
    }
}

fn read_runtime(worktree: &Path, mut in_epoch: impl FnMut()) -> Result<ActivityObservation> {
    // Pin before discovery so a retargeted workspace alias cannot mix namespaces.
    let root = match open(worktree, true) {
        Ok(root) => root,
        Err(error) if missing(&error) => return Ok(ActivityObservation::Idle),
        Err(error) => return Err(error),
    };
    let namespace = Workspace::discover_control_dir(worktree, CONTROL_NAMESPACE)?;
    anyhow::ensure!(
        current(&root, worktree)?,
        "workspace changed during runtime discovery"
    );
    let Some(namespace) = namespace else {
        return Ok(ActivityObservation::Idle);
    };
    for _ in 0..IDENTITY_RETRY_LIMIT {
        let directory = open(&namespace, true)?;
        let lease_path = namespace.join(LEASE_FILE_NAME);
        let epoch_path = namespace.join(EPOCH_FILE_NAME);
        let mut lease = match open(&lease_path, false) {
            Ok(file) => file,
            Err(error) if missing(&error) => {
                if current(&root, worktree)? && current(&directory, &namespace)? {
                    return Ok(ActivityObservation::Idle);
                }
                continue;
            }
            Err(error) => return Err(error),
        };
        let mut epoch = open(&epoch_path, false)?;
        let locked = unsafe { libc::flock(epoch.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) };
        let lock_error = (locked != 0).then(std::io::Error::last_os_error);
        // Check after the probe: a replaced descriptor cannot authorize even Busy.
        if !current(&root, worktree)?
            || !current(&directory, &namespace)?
            || !current(&lease, &lease_path)?
            || !current(&epoch, &epoch_path)?
        {
            continue;
        }
        if let Some(error) = lock_error {
            return if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
            {
                Ok(ActivityObservation::Busy(
                    "session epoch publication is in progress".into(),
                ))
            } else {
                Err(error).context("probing shared session epoch")
            };
        }
        in_epoch();
        // The only guarded work is bounded record copying and identity checking.
        // File drop releases the shared flock on every return/retry/error path.
        let result = (|| {
            let lease_record = parse_process_record(&bounded_record(&mut lease)?)?;
            let epoch_bytes = bounded_record(&mut epoch)?;
            let epoch_record = parse_epoch_record(&epoch_bytes)?;
            anyhow::ensure!(
                lease_record == epoch_record.process,
                "lease and epoch records do not match"
            );
            anyhow::ensure!(
                lease_record.worktree_identity == FileIdentity::from_metadata(&root.metadata()?),
                "runtime records belong to a different working tree"
            );
            Ok(if epoch_record.signal_path.is_none() {
                ActivityObservation::Idle
            } else {
                observe_private(&namespace, &epoch_bytes)?
            })
        })();
        if current(&root, worktree)?
            && current(&directory, &namespace)?
            && current(&lease, &lease_path)?
            && current(&epoch, &epoch_path)?
        {
            return result;
        }
    }
    bail!("runtime paths changed during all {IDENTITY_RETRY_LIMIT} observation attempts")
}

/// Validate the entire extension before trusting even evidence of release.
fn observation_witness(record: &str) -> Result<(PathBuf, FileIdentity)> {
    anyhow::ensure!(
        record_field(record, "observation-version")
            .context("active epoch record has no supported observation witness")?
            == "1",
        "active epoch record has no supported observation witness"
    );
    let text = |name| -> Result<String> {
        decode_path(record_field(record, name)?)?
            .into_os_string()
            .into_string()
            .map_err(|_| anyhow::anyhow!("{name} is not UTF-8"))
    };
    let handle = crate::Handle::parse(&text("observation-handle-hex")?)?;
    anyhow::ensure!(
        record_field(record, "observation-key")? == handle.key().to_string(),
        "observation key does not match its handle"
    );
    crate::Kind::new(&text("observation-kind-hex")?)?;
    for name in ["observation-tree-device", "observation-tree-inode"] {
        record_field(record, name)?
            .parse::<u64>()
            .with_context(|| format!("parsing {name}"))?;
    }
    let name = text("observation-witness-name-hex")?;
    anyhow::ensure!(
        !name.is_empty()
            && name != "."
            && name != ".."
            && !name.contains('/')
            && !name.contains('\0'),
        "observation witness name is not a plain basename"
    );
    let identity = FileIdentity {
        device: record_field(record, "observation-witness-device")?.parse()?,
        inode: record_field(record, "observation-witness-inode")?.parse()?,
    };
    Ok((PathBuf::from(name), identity))
}

fn observe_private(namespace: &Path, record: &str) -> Result<ActivityObservation> {
    let (name, expected) = observation_witness(record)?;
    let path = namespace.join(name);
    for _ in 0..IDENTITY_RETRY_LIMIT {
        let file = open(&path, false)?;
        if !current(&file, &path)? {
            continue;
        }
        anyhow::ensure!(
            FileIdentity::from_metadata(&file.metadata()?) == expected,
            "private witness identity does not match the epoch"
        );
        // Shared probes cannot contend with one another. Unlock before all I/O.
        // https://man7.org/linux/man-pages/man2/flock.2.html
        let locked = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) };
        let error = (locked != 0).then(std::io::Error::last_os_error);
        if locked == 0 && unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) } != 0 {
            return Err(std::io::Error::last_os_error()).context("releasing shared witness probe");
        }
        if !current(&file, &path)? {
            continue;
        }
        return match error {
            None => Ok(ActivityObservation::Idle),
            Some(error) if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN) =>
            {
                // Contention alone cannot bind a mandate to the captured tree.
                Ok(ActivityObservation::Unavailable(
                    "launch witness is not verified".into(),
                ))
            }
            Some(error) => Err(error).context("probing shared private witness"),
        };
    }
    bail!("private witness changed during all {IDENTITY_RETRY_LIMIT} observation attempts")
}

fn missing(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<std::io::Error>()
        .is_some_and(|error| error.kind() == std::io::ErrorKind::NotFound)
}

fn open(path: &Path, directory: bool) -> Result<File> {
    // O_NONBLOCK prevents a substituted FIFO from waiting before fstat rejects it.
    // https://man7.org/linux/man-pages/man2/open.2.html
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(
            libc::O_NONBLOCK | libc::O_CLOEXEC | if directory { libc::O_DIRECTORY } else { 0 },
        )
        .open(path)
        .with_context(|| format!("opening runtime path {}", path.display()))?;
    let metadata = file.metadata()?;
    anyhow::ensure!(
        if directory {
            metadata.is_dir()
        } else {
            metadata.is_file()
        },
        "runtime path has the wrong file type: {}",
        path.display()
    );
    Ok(file)
}

fn current(file: &File, path: &Path) -> Result<bool> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error).context("validating runtime path identity"),
    };
    Ok(FileIdentity::from_metadata(&file.metadata()?) == FileIdentity::from_metadata(&metadata))
}

fn bounded_record(file: &mut File) -> Result<String> {
    let mut record = String::new();
    file.take(RECORD_LIMIT + 1).read_to_string(&mut record)?;
    anyhow::ensure!(
        record.len() as u64 <= RECORD_LIMIT,
        "runtime record exceeds 64 KiB"
    );
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use tempfile::TempDir;

    fn fixture() -> (TempDir, DriverLease) {
        let work = TempDir::new().unwrap();
        fs::create_dir(work.path().join(".jj")).unwrap();
        fs::create_dir(work.path().join(".grove")).unwrap();
        fs::write(work.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let workspace = Workspace::resolve(work.path()).unwrap();
        let lease = DriverLease::acquire(&workspace).unwrap();
        (work, lease)
    }

    fn sample(work: &Path) -> ActivityObservation {
        let capture = crate::try_observe(work, &[]);
        assert!(matches!(
            capture.tree.unwrap(),
            crate::TreeObservation::Ready(_)
        ));
        capture.activity
    }

    #[test]
    fn witness_released_active_epoch_is_idle_despite_leftover_bytes() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = fixture();
        let root = crate::TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                &lease.control_dir.join("signal-test"),
            )
            .unwrap();
        let witness = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        lease.launch.as_mut().unwrap().started().unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        lease.launch.take();
        for marker in [
            b"started\n".as_slice(),
            b"",
            b"sta",
            b"invalid",
            b"started\nextra",
        ] {
            fs::write(&witness, marker).unwrap();
            let before = contents(work.path());
            assert_eq!(sample(work.path()), ActivityObservation::Idle);
            assert_eq!(contents(work.path()), before);
            // No successful observer probe may escape into the returned value.
            let file = File::open(&witness).unwrap();
            assert_eq!(
                unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
    }

    #[test]
    fn idle_legacy_active_contention_and_recovery_preserve_tree_and_admission() {
        let (work, lease) = fixture();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        let signal = lease.control_dir.join("signal-first");
        lease.activate_session_epoch(&signal).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        let admitted = admit_session(work.path(), "test", Some(signal.clone())).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        drop(admitted);
        let lock = acquire_epoch_file(
            &lease.control_dir.join(EPOCH_FILE_NAME),
            LockMode::Exclusive,
            "test",
        )
        .unwrap();
        assert!(matches!(sample(work.path()), ActivityObservation::Busy(_)));
        drop(lock);
        lease.invalidate_session_epoch().unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        lease
            .activate_session_epoch(&lease.control_dir.join("signal-second"))
            .unwrap();
        assert!(admit_session(work.path(), "old session", Some(signal)).is_err());
    }

    fn released_fixture() -> (TempDir, DriverLease, PathBuf, PathBuf, String) {
        let (work, mut lease) = fixture();
        let signal = lease.control_dir.join("signal-test");
        let root = crate::TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &super::super::tests::witness_selection(), &signal)
            .unwrap();
        let witness = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        lease.launch.take();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        let record = fs::read_to_string(&epoch).unwrap();
        (work, lease, witness, epoch, record)
    }

    #[test]
    fn witness_extension_validation_cannot_be_replaced_by_blanket_unavailability() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease, _, epoch, original) = released_fixture();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        // Enumerate the writer's extension: every published field is mandatory
        // and unique for observation, while admission ignores the extension.
        for line in original
            .lines()
            .filter(|line| line.starts_with("observation-"))
        {
            let (name, _) = line.split_once('=').unwrap();
            for invalid in [
                original.replace(&format!("{line}\n"), ""),
                format!("{original}{line}\n"),
                original.replace(line, &format!("{name}=invalid")),
            ] {
                fs::write(&epoch, invalid).unwrap();
                assert!(
                    matches!(sample(work.path()), ActivityObservation::Unavailable(_)),
                    "{name}"
                );
                assert!(admit_session(
                    work.path(),
                    "test",
                    Some(lease.control_dir.join("signal-test"))
                )
                .is_ok());
                fs::write(&epoch, &original).unwrap();
                assert_eq!(sample(work.path()), ActivityObservation::Idle);
            }
        }
        for (name, value) in [
            ("observation-version", "0"),
            ("observation-version", "2"),
            ("observation-key", "2"),
            ("observation-key", "01"),
            ("observation-key", "0"),
            ("observation-handle-hex", "776f726b2d6b3031"), // work-k01
            ("observation-handle-hex", "ff"),
            ("observation-kind-hex", "49"), // I
            ("observation-tree-device", "18446744073709551616"),
            ("observation-witness-device", "18446744073709551615"),
            ("observation-witness-inode", "0"),
        ] {
            let old = record_field(&original, name).unwrap();
            fs::write(
                &epoch,
                original.replace(&format!("{name}={old}"), &format!("{name}={value}")),
            )
            .unwrap();
            assert!(
                matches!(sample(work.path()), ActivityObservation::Unavailable(_)),
                "{name}={value}"
            );
        }
        let name_field = "observation-witness-name-hex";
        let old = record_field(&original, name_field).unwrap();
        for name in [
            "",
            ".",
            "..",
            "../witness",
            "/tmp/witness",
            "sub/witness",
            "witness/",
            "witness\0",
        ] {
            let encoded = encode_path(Path::new(name)).unwrap();
            fs::write(
                &epoch,
                original.replace(
                    &format!("{name_field}={old}"),
                    &format!("{name_field}={encoded}"),
                ),
            )
            .unwrap();
            assert!(
                matches!(sample(work.path()), ActivityObservation::Unavailable(_)),
                "{name:?}"
            );
        }
        fs::write(&epoch, &original).unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

    #[test]
    fn witness_missing_replaced_and_nonregular_evidence_is_unavailable() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        use std::os::unix::fs::PermissionsExt;
        let (work, _lease, witness, _, _) = released_fixture();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        let retained = File::open(&witness).unwrap();
        assert_eq!(
            unsafe { libc::flock(retained.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        fs::set_permissions(&witness, fs::Permissions::from_mode(0o0)).unwrap();
        if unsafe { libc::geteuid() } != 0 {
            assert!(matches!(
                sample(work.path()),
                ActivityObservation::Unavailable(_)
            ));
        }
        fs::set_permissions(&witness, fs::Permissions::from_mode(0o600)).unwrap();
        let saved = witness.with_extension("saved");
        fs::rename(&witness, &saved).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::write(&witness, "started\n").unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&witness).unwrap();
        fs::create_dir(&witness).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_dir(&witness).unwrap();
        let path = std::ffi::CString::new(witness.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&witness).unwrap();
        fs::rename(saved, &witness).unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

    #[test]
    fn absent_controls_are_idle_but_missing_or_bad_records_are_unavailable() {
        let (work, lease) = fixture();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        let original = fs::read(&epoch).unwrap();
        for bytes in [
            b"bad".to_vec(),
            vec![b'x'; 65537],
            b"\xff".to_vec(),
            String::from_utf8(original.clone())
                .unwrap()
                .replace(&lease.nonce, "00000000000000000000000000000000")
                .into_bytes(),
            String::from_utf8(original.clone())
                .unwrap()
                .replace("state=inactive", "state=other")
                .into_bytes(),
            String::from_utf8(original.clone())
                .unwrap()
                .replace("state=inactive", "state=inactive\nsignal-path-hex=61")
                .into_bytes(),
        ] {
            fs::write(&epoch, bytes).unwrap();
            assert!(matches!(
                sample(work.path()),
                ActivityObservation::Unavailable(_)
            ));
        }
        fs::remove_file(&epoch).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::create_dir(&epoch).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_dir(&epoch).unwrap();
        let path = std::ffi::CString::new(epoch.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&epoch).unwrap();
        fs::write(&epoch, original).unwrap();
        fs::write(&lease.lease_path, vec![b'x'; 65537]).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&lease.lease_path).unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

    #[test]
    fn aliases_match_by_identity_and_subdirectories_do_not_borrow_runtime() {
        let (work, lease) = fixture();
        let alias_parent = TempDir::new().unwrap();
        let alias = alias_parent.path().join("alias");
        std::os::unix::fs::symlink(work.path(), &alias).unwrap();
        assert_eq!(sample(&alias), ActivityObservation::Idle);
        lease
            .activate_session_epoch(&lease.control_dir.join("signal"))
            .unwrap();
        assert!(matches!(
            sample(&alias),
            ActivityObservation::Unavailable(_)
        ));
        let sub = work.path().join("sub");
        fs::create_dir(&sub).unwrap();
        assert_eq!(
            crate::try_observe(&sub, &[]).activity,
            ActivityObservation::Idle
        );
        let other = TempDir::new().unwrap();
        fs::create_dir(other.path().join(".jj")).unwrap();
        std::os::unix::fs::symlink(&lease.control_dir, other.path().join(".jj/grove")).unwrap();
        assert!(matches!(
            crate::try_observe(other.path(), &[]).activity,
            ActivityObservation::Unavailable(_)
        ));
    }

    #[test]
    fn descriptor_flags_and_bounded_reads_are_enforced() {
        let (work, lease) = fixture();
        for (path, directory) in [
            (work.path(), true),
            (lease.control_dir.as_path(), true),
            (lease.lease_path.as_path(), false),
        ] {
            let file = open(path, directory).unwrap();
            let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFL) };
            assert_eq!(flags & libc::O_ACCMODE, libc::O_RDONLY);
            assert_ne!(flags & libc::O_NONBLOCK, 0);
            assert_ne!(
                unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFD) } & libc::FD_CLOEXEC,
                0
            );
            assert!(current(&file, path).unwrap());
        }
        fs::write(&lease.lease_path, vec![b'a'; 65536]).unwrap();
        assert_eq!(
            bounded_record(&mut open(&lease.lease_path, false).unwrap())
                .unwrap()
                .len(),
            65536
        );
        fs::write(&lease.lease_path, vec![b'a'; 65537]).unwrap();
        assert!(bounded_record(&mut open(&lease.lease_path, false).unwrap()).is_err());
    }

    #[test]
    fn capture_pause_and_returned_values_hold_no_epoch_or_tree_lock() {
        let (work, lease) = fixture();
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        std::thread::scope(|scope| {
            let path = work.path();
            let observer = scope.spawn(move || {
                crate::observation::observe_with(
                    path,
                    &[],
                    || {
                        ready_tx.send(()).unwrap();
                        release_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                    },
                    || {},
                )
            });
            ready_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            lease
                .activate_session_epoch(&lease.control_dir.join("signal"))
                .unwrap();
            let tree_writer = File::open(work.path()).unwrap();
            assert_eq!(
                unsafe { libc::flock(tree_writer.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
            drop(tree_writer);
            release_tx.send(()).unwrap();
            let capture = observer.join().unwrap();
            lease.invalidate_session_epoch().unwrap();
            assert!(matches!(
                capture.activity,
                ActivityObservation::Unavailable(_)
            ));
            assert!(matches!(
                capture.tree.unwrap(),
                crate::TreeObservation::Ready(_)
            ));
        });
    }

    #[test]
    fn paused_runtime_reader_allows_shared_observers_and_bounds_driver_handoff() {
        let (work, lease) = fixture();
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        std::thread::scope(|scope| {
            let path = work.path();
            let observer = scope.spawn(move || {
                crate::observation::observe_with(
                    path,
                    &[],
                    || {},
                    || {
                        ready_tx.send(()).unwrap();
                        release_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                    },
                )
            });
            ready_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            assert_eq!(sample(work.path()), ActivityObservation::Idle);
            let tree_writer = File::open(work.path()).unwrap();
            assert_eq!(
                unsafe { libc::flock(tree_writer.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
            drop(tree_writer);
            let elapsed = std::cell::Cell::new(Duration::ZERO);
            let reports = std::cell::Cell::new(0);
            let start = Instant::now();
            let error = acquire_epoch_file_with(
                &lease.control_dir.join(EPOCH_FILE_NAME),
                LockMode::Exclusive,
                "post-reap invalidation",
                EPOCH_HANDOFF_TIMEOUT,
                || start + elapsed.get(),
                || elapsed.set(elapsed.get() + Duration::from_secs(10)),
                |_, _| Ok(()),
                |_, _| Ok(()),
                || reports.set(reports.get() + 1),
            )
            .unwrap_err();
            assert_eq!(elapsed.get(), Duration::from_secs(30));
            assert_eq!(reports.get(), 1);
            assert!(error.to_string().contains("timed out after 30s"));
            release_tx.send(()).unwrap();
            let retained = observer.join().unwrap();
            lease.invalidate_session_epoch().unwrap();
            assert_eq!(retained.activity, ActivityObservation::Idle);
        });
    }
    #[test]
    fn identity_replacements_retry_and_stop_after_eight_attempts() {
        let (work, lease) = fixture();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        let bytes = fs::read(&epoch).unwrap();
        for replacements in [1, 8] {
            let mut attempts = 0;
            let result = crate::observation::observe_with(
                work.path(),
                &[],
                || {},
                || {
                    attempts += 1;
                    if attempts <= replacements {
                        fs::rename(&epoch, epoch.with_extension("old")).unwrap();
                        fs::write(&epoch, &bytes).unwrap();
                    }
                },
            );
            if replacements == 1 {
                assert_eq!(result.activity, ActivityObservation::Idle);
                assert_eq!(attempts, 2);
            } else {
                assert!(
                    matches!(result.activity, ActivityObservation::Unavailable(ref reason) if reason.contains("8 observation attempts"))
                );
                assert_eq!(attempts, 8);
            }
        }
    }

    fn contents(path: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                files.push((path.clone(), Vec::new()));
                files.extend(contents(&path));
            } else {
                files.push((path.clone(), fs::read(path).unwrap()));
            }
        }
        files.sort();
        files
    }

    #[test]
    fn observations_preserve_all_tree_and_administration_bytes() {
        let (work, lease) = fixture();
        for active in [false, true] {
            if active {
                lease
                    .activate_session_epoch(&lease.control_dir.join("signal"))
                    .unwrap();
            }
            let before = contents(work.path());
            let _ = sample(work.path());
            let _ = sample(work.path());
            assert_eq!(contents(work.path()), before);
        }
        drop(lease);
        let work = TempDir::new().unwrap();
        for namespace in [false, true] {
            if namespace {
                fs::create_dir(work.path().join(".jj")).unwrap();
            }
            let before = contents(work.path());
            assert_eq!(
                crate::try_observe(work.path(), &[]).activity,
                ActivityObservation::Idle
            );
            assert_eq!(contents(work.path()), before);
        }
    }

    #[test]
    fn unreadable_epoch_and_nondirectory_namespace_are_unavailable() {
        use std::os::unix::fs::PermissionsExt;
        let (work, lease) = fixture();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        fs::set_permissions(&epoch, fs::Permissions::from_mode(0o0)).unwrap();
        if unsafe { libc::geteuid() } != 0 {
            assert!(matches!(
                sample(work.path()),
                ActivityObservation::Unavailable(_)
            ));
        }
        fs::set_permissions(&epoch, fs::Permissions::from_mode(0o600)).unwrap();
        fs::rename(&lease.control_dir, lease.control_dir.with_extension("old")).unwrap();
        fs::write(&lease.control_dir, "not a namespace").unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
    }
}
