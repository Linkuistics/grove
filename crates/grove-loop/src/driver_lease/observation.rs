//! Advisory runtime reads share admission's grammar, never its authority path.

use super::*;
use crate::observation::RuntimeIdentity;
use crate::{ActivityObservation, LaunchTreeIdentity, RunningMandate, TreeLifetime, TreeRelation};

const RECORD_LIMIT: u64 = 64 * 1024;

pub(crate) fn observe(
    worktree: &Path,
    tree: Option<&TreeLifetime>,
    in_epoch: impl FnMut(),
) -> ActivityObservation {
    match read_runtime(worktree, tree, in_epoch, &mut NativeWitnessIo) {
        Ok(activity) => activity,
        Err(error) => ActivityObservation::Unavailable(format!("{error:#}")),
    }
}

fn read_runtime(
    worktree: &Path,
    tree: Option<&TreeLifetime>,
    mut in_epoch: impl FnMut(),
    io: &mut impl WitnessIo,
) -> Result<ActivityObservation> {
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
                observe_private(&namespace, &epoch_bytes, &epoch_record, tree, io)?
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
fn observation_witness(record: &str, epoch: &EpochRecord) -> Result<RunningMandate> {
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
    let kind = crate::Kind::new(&text("observation-kind-hex")?)?;
    let number = |name| -> Result<u64> {
        record_field(record, name)?
            .parse::<u64>()
            .with_context(|| format!("parsing {name}"))
    };
    let tree_identity = LaunchTreeIdentity((
        number("observation-tree-device")?,
        number("observation-tree-inode")?,
    ));
    let name = text("observation-witness-name-hex")?;
    anyhow::ensure!(
        !name.is_empty()
            && name != "."
            && name != ".."
            && !name.contains('/')
            && !name.contains('\0'),
        "observation witness name is not a plain basename"
    );
    Ok(RunningMandate {
        handle,
        kind,
        tree_identity,
        relation: TreeRelation::NoReadableTree,
        runtime: RuntimeIdentity {
            worktree: (
                epoch.process.worktree_identity.device,
                epoch.process.worktree_identity.inode,
            ),
            nonce: epoch.process.nonce.clone(),
            signal: epoch
                .signal_path
                .clone()
                .context("observation epoch is inactive")?,
            witness_name: PathBuf::from(name),
            witness: (
                number("observation-witness-device")?,
                number("observation-witness-inode")?,
            ),
        },
    })
}

/// Native operations are replaceable at the filesystem/lock boundary, not at
/// the status-provider boundary. A probe returns true only after unlocking.
trait WitnessIo {
    fn open(&mut self, path: &Path) -> Result<File> {
        open(path, false)
    }

    fn marker(&mut self, file: &mut File) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        file.take(9).read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    fn current(&mut self, file: &File, path: &Path) -> Result<bool> {
        current(file, path)
    }

    fn probe(&mut self, file: &File) -> Result<bool> {
        // Independent shared probes are compatible; only exclusive ownership
        // can contend. Release success immediately, before validation or I/O.
        // https://man7.org/linux/man-pages/man2/flock.2.html
        // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flock.2.html
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) } == 0 {
            if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) } != 0 {
                return Err(std::io::Error::last_os_error())
                    .context("releasing shared witness probe");
            }
            Ok(true)
        } else {
            let error = std::io::Error::last_os_error();
            if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
            {
                Ok(false)
            } else {
                Err(error).context("probing shared witness")
            }
        }
    }
}

struct NativeWitnessIo;
impl WitnessIo for NativeWitnessIo {}

fn tree_relation(
    tree: Option<&TreeLifetime>,
    expected: LaunchTreeIdentity,
    io: &mut impl WitnessIo,
) -> Result<Option<TreeRelation>> {
    let Some(tree) = tree else {
        return Ok(Some(TreeRelation::NoReadableTree));
    };
    let metadata = tree.directory().metadata()?;
    if (metadata.dev(), metadata.ino()) != expected.0 {
        return Ok(Some(TreeRelation::PreviousTree));
    }
    // Probe the accepted pin itself: reopening its path could bind another tree.
    if io
        .probe(tree.directory())
        .context("probing task-root witness")?
    {
        Ok(None)
    } else {
        Ok(Some(TreeRelation::SameTree))
    }
}

fn observe_private(
    namespace: &Path,
    record: &str,
    epoch: &EpochRecord,
    tree: Option<&TreeLifetime>,
    io: &mut impl WitnessIo,
) -> Result<ActivityObservation> {
    let mut mandate = observation_witness(record, epoch)?;
    // Save errors and unverified binding until the final private probe: release
    // establishes Idle even if the directory probe or marker read failed.
    let relation = tree_relation(tree, mandate.tree_identity, io);
    let path = namespace.join(&mandate.runtime.witness_name);
    for _ in 0..IDENTITY_RETRY_LIMIT {
        let mut file = io.open(&path)?;
        if !io.current(&file, &path)? {
            continue;
        }
        anyhow::ensure!(
            {
                let metadata = file.metadata()?;
                (metadata.dev(), metadata.ino()) == mandate.runtime.witness
            },
            "private witness identity does not match the epoch"
        );
        let marker = io.marker(&mut file);
        let released = io.probe(&file);
        if !io.current(&file, &path)? {
            continue;
        }
        if released.context("probing private witness")? {
            return Ok(ActivityObservation::Idle);
        }
        let marker = marker.context("reading private witness marker")?;
        if marker != b"started\n" {
            anyhow::ensure!(
                b"started\n".starts_with(&marker),
                "invalid private witness marker"
            );
            return Ok(ActivityObservation::Busy(
                "launch publication is in progress".into(),
            ));
        }
        let Some(relation) = relation? else {
            return Ok(ActivityObservation::Busy(
                "task-root witness is not held".into(),
            ));
        };
        mandate.relation = relation;
        return Ok(ActivityObservation::Running(mandate));
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
    fn witness_started_public_observation_identifies_the_prepared_launch() {
        let (work, mut lease) = fixture();
        let root = crate::TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                &lease.control_dir.join("signal-test"),
            )
            .unwrap();
        lease.launch.as_mut().unwrap().started().unwrap();
        let observed = sample(work.path());
        let ActivityObservation::Running(mandate) = observed else {
            panic!("{observed:?}");
        };
        assert_eq!(mandate.handle.to_string(), "work-k1");
        assert_eq!(mandate.handle.key().get(), 1);
        assert_eq!(mandate.kind.label(), "impl");
        assert_eq!(mandate.relation, TreeRelation::SameTree);
        assert_eq!(mandate.runtime.nonce, lease.nonce);
        assert_eq!(sample(work.path()), ActivityObservation::Running(mandate));
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
            ActivityObservation::Running(_)
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

    fn started_fixture() -> (TempDir, DriverLease) {
        let (work, mut lease) = fixture();
        let root = TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                &lease.control_dir.join("signal-test"),
            )
            .unwrap();
        lease.launch.as_mut().unwrap().started().unwrap();
        (work, lease)
    }

    fn running(activity: ActivityObservation) -> RunningMandate {
        let ActivityObservation::Running(mandate) = activity else {
            panic!("expected Running, got {activity:?}")
        };
        mandate
    }

    #[test]
    fn witness_real_launch_started_and_reaped_reach_public_observation() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = fixture();
        let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
        let root = TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                channel.path(),
            )
            .unwrap();
        assert!(matches!(sample(work.path()), ActivityObservation::Busy(_)));
        let config = work.path().join("launch.kdl");
        fs::write(&config, "test \"/bin/sh -c true\"\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        let mut events = Vec::new();
        lease
            .supervise_launch(|notify| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(work.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    &mut |event| {
                        notify(event);
                        events.push(event);
                        let before = contents(work.path());
                        let activity = sample(work.path());
                        assert_eq!(contents(work.path()), before);
                        match event {
                            keyed_launch::LaunchEvent::Started => {
                                let mandate = running(activity);
                                assert_eq!(mandate.handle.to_string(), "work-k1");
                                assert_eq!(mandate.relation, TreeRelation::SameTree);
                                assert_eq!(mandate.runtime.signal, channel.path());
                            }
                            keyed_launch::LaunchEvent::Reaped => {
                                assert_eq!(activity, ActivityObservation::Idle)
                            }
                        }
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
    }

    #[test]
    fn witness_private_open_and_identity_io_errors_release_the_epoch_guard() {
        struct FailingIo {
            fail_at: usize,
            calls: usize,
        }
        impl FailingIo {
            fn check(&mut self) -> Result<()> {
                self.calls += 1;
                anyhow::ensure!(
                    self.calls != self.fail_at,
                    "injected witness filesystem error"
                );
                Ok(())
            }
        }
        impl WitnessIo for FailingIo {
            fn open(&mut self, path: &Path) -> Result<File> {
                self.check()?;
                NativeWitnessIo.open(path)
            }
            fn current(&mut self, file: &File, path: &Path) -> Result<bool> {
                self.check()?;
                NativeWitnessIo.current(file, path)
            }
        }
        let (work, _lease) = started_fixture();
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        for fail_at in 1..=3 {
            let mut io = FailingIo { fail_at, calls: 0 };
            let error = read_runtime(work.path(), Some(&pin), || {}, &mut io).unwrap_err();
            assert!(error
                .to_string()
                .contains("injected witness filesystem error"));
            assert_eq!(io.calls, fail_at);
            assert!(acquire_epoch_file(
                &work.path().join(".jj/grove/session.epoch"),
                LockMode::Exclusive,
                "test"
            )
            .is_ok());
            assert_eq!(
                running(sample(work.path())).relation,
                TreeRelation::SameTree
            );
        }
    }

    #[test]
    fn witness_capture_uses_accepted_pin_after_root_path_replacement() {
        let (work, _lease) = started_fixture();
        let original = running(sample(work.path()));
        let captured = crate::observation::observe_with(
            work.path(),
            &[],
            || {
                fs::rename(work.path().join(".grove"), work.path().join("old")).unwrap();
                fs::create_dir(work.path().join(".grove")).unwrap();
                fs::write(work.path().join(".grove/_BRIEF.md"), "replacement").unwrap();
            },
            || {},
        );
        assert_eq!(running(captured.activity), original);
        let crate::TreeObservation::Ready(tree) = captured.tree.unwrap() else {
            panic!()
        };
        assert!(!tree.lifetime.at(work.path()).unwrap());
        assert_eq!(tree.content.unwrap(), b"root");
        let replacement = running(sample(work.path()));
        assert_eq!(replacement.relation, TreeRelation::PreviousTree);
        assert_eq!(replacement.tree_identity, original.tree_identity);
        assert_eq!(replacement.runtime, original.runtime);
    }

    #[test]
    fn witness_tree_absence_failure_and_contention_preserve_runtime_identity() {
        let (work, _lease) = started_fixture();
        let original = running(sample(work.path()));
        let check = || {
            let mandate = running(crate::try_observe(work.path(), &[]).activity);
            assert_eq!(mandate.relation, TreeRelation::NoReadableTree);
            assert_eq!(mandate.runtime, original.runtime);
            assert_eq!(mandate.tree_identity, original.tree_identity);
        };
        let writer = File::open(work.path()).unwrap();
        assert_eq!(
            unsafe { libc::flock(writer.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0
        );
        check();
        drop(writer);
        fs::write(work.path().join(".grove/01-impl--work-k1.md"), "").unwrap();
        fs::write(work.path().join(".grove/02-impl--duplicate-k1.md"), "").unwrap();
        assert!(crate::try_observe(work.path(), &[]).tree.is_err());
        check();
        fs::remove_dir_all(work.path().join(".grove")).unwrap();
        check();
    }

    #[test]
    fn witness_marker_prefixes_invalid_bytes_and_release_have_exact_precedence() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = started_fixture();
        let path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        for length in 0..8 {
            fs::write(&path, &b"started\n"[..length]).unwrap();
            assert!(
                matches!(sample(work.path()), ActivityObservation::Busy(_)),
                "{length}"
            );
        }
        for marker in [
            b"started\n!".as_slice(),
            b"started",
            b"stax",
            b"\xff",
            b"started\0",
        ] {
            fs::write(&path, marker).unwrap();
            let activity = sample(work.path());
            if marker == b"started" {
                assert!(matches!(activity, ActivityObservation::Busy(_)));
            } else {
                assert!(
                    matches!(activity, ActivityObservation::Unavailable(_)),
                    "{marker:?}: {activity:?}"
                );
            }
        }
        fs::write(&path, vec![b'x'; 100_000]).unwrap();
        assert_eq!(
            NativeWitnessIo
                .marker(&mut File::open(&path).unwrap())
                .unwrap()
                .len(),
            9
        );
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        lease.launch.take();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

    #[derive(Clone, Copy, Debug)]
    enum ProbeResult {
        Released,
        Held,
        Error,
    }

    struct ProbeCases {
        directory: ProbeResult,
        private: ProbeResult,
        marker_error: bool,
        events: Vec<&'static str>,
    }

    impl WitnessIo for ProbeCases {
        fn marker(&mut self, file: &mut File) -> Result<Vec<u8>> {
            self.events.push("marker");
            if self.marker_error {
                bail!("injected marker read error");
            }
            NativeWitnessIo.marker(file)
        }
        fn probe(&mut self, file: &File) -> Result<bool> {
            let result = if file.metadata()?.is_dir() {
                self.events.push("directory");
                self.directory
            } else {
                self.events.push("private");
                self.private
            };
            match result {
                ProbeResult::Released => Ok(true),
                ProbeResult::Held => Ok(false),
                ProbeResult::Error => Err(std::io::Error::from_raw_os_error(libc::ENOLCK).into()),
            }
        }
    }

    #[test]
    fn witness_directory_private_and_marker_error_matrix_obeys_final_probe() {
        let (work, _lease) = started_fixture();
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        for directory in [ProbeResult::Released, ProbeResult::Held, ProbeResult::Error] {
            for private in [ProbeResult::Released, ProbeResult::Held, ProbeResult::Error] {
                for marker_error in [false, true] {
                    let mut io = ProbeCases {
                        directory,
                        private,
                        marker_error,
                        events: vec![],
                    };
                    let result = read_runtime(work.path(), Some(&pin), || {}, &mut io);
                    assert_eq!(io.events, ["directory", "marker", "private"]);
                    match (private, marker_error, directory) {
                        (ProbeResult::Released, _, _) => {
                            assert_eq!(result.unwrap(), ActivityObservation::Idle)
                        }
                        (ProbeResult::Error, _, _) | (_, true, _) | (_, _, ProbeResult::Error) => {
                            assert!(result.is_err())
                        }
                        (_, _, ProbeResult::Released) => {
                            assert!(matches!(result.unwrap(), ActivityObservation::Busy(_)))
                        }
                        (_, _, ProbeResult::Held) => {
                            assert_eq!(running(result.unwrap()).relation, TreeRelation::SameTree)
                        }
                    }
                    // Every result drops the shared epoch guard.
                    assert!(acquire_epoch_file(
                        &work.path().join(".jj/grove/session.epoch"),
                        LockMode::Exclusive,
                        "test"
                    )
                    .is_ok());
                }
            }
        }
    }

    #[test]
    fn witness_unlocked_matching_directory_is_busy_and_probes_escape_no_locks() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = started_fixture();
        let path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        let directory = lease.launch.as_ref().unwrap().root.directory();
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_UN) },
            0
        );
        let capture = crate::try_observe(work.path(), &[]);
        assert!(matches!(capture.activity, ActivityObservation::Busy(_)));
        let probe = File::open(work.path().join(".grove")).unwrap();
        assert_eq!(
            unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0
        );
        drop(probe);
        lease.launch.take();
        let idle = crate::try_observe(work.path(), &[]);
        assert_eq!(idle.activity, ActivityObservation::Idle);
        for path in [
            path,
            work.path().join(".grove"),
            work.path().join(".jj/grove/session.epoch"),
        ] {
            let probe = File::open(path).unwrap();
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum ReplacementPoint {
        Open,
        Probe,
    }

    struct ReplacingIo {
        point: ReplacementPoint,
        path: PathBuf,
        original: PathBuf,
        substitute: PathBuf,
        remaining: usize,
        opens: usize,
        recover: bool,
    }

    impl ReplacingIo {
        fn replace(&mut self) {
            if self.remaining == 0 {
                return;
            }
            self.remaining -= 1;
            // Alternate paths while keeping both objects alive. Restoring the
            // published object on the second attempt proves retry can recover.
            if self.original.exists() {
                fs::rename(&self.path, &self.substitute).unwrap();
                fs::rename(&self.original, &self.path).unwrap();
            } else {
                fs::rename(&self.path, &self.original).unwrap();
                fs::rename(&self.substitute, &self.path).unwrap();
            }
        }
    }

    impl WitnessIo for ReplacingIo {
        fn open(&mut self, path: &Path) -> Result<File> {
            self.opens += 1;
            if matches!(self.point, ReplacementPoint::Probe)
                && self.recover
                && self.original.exists()
            {
                fs::rename(&self.path, &self.substitute)?;
                fs::rename(&self.original, &self.path)?;
            }
            let file = NativeWitnessIo.open(path)?;
            if matches!(self.point, ReplacementPoint::Open) {
                self.replace();
            }
            Ok(file)
        }
        fn probe(&mut self, file: &File) -> Result<bool> {
            let result = NativeWitnessIo.probe(file);
            if !file.metadata()?.is_dir() && matches!(self.point, ReplacementPoint::Probe) {
                self.replace();
            }
            result
        }
    }

    #[test]
    fn witness_open_and_probe_replacements_reject_stale_descriptors_and_bound_retries() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for point in [ReplacementPoint::Open, ReplacementPoint::Probe] {
            for remaining in [1, 2, 8] {
                let (work, _lease, path, epoch_path, record) = released_fixture();
                let original = path.with_extension("original");
                let substitute = path.with_extension("substitute");
                fs::write(&substitute, "started\n").unwrap();
                let mut io = ReplacingIo {
                    point,
                    path,
                    original,
                    substitute,
                    remaining,
                    opens: 0,
                    recover: remaining > 1,
                };
                let result = read_runtime(work.path(), None, || {}, &mut io);
                if remaining == 2 {
                    assert_eq!(result.unwrap(), ActivityObservation::Idle);
                    assert_eq!(io.opens, 3);
                } else {
                    let message = format!("{:#}", result.unwrap_err());
                    assert!(
                        message.contains(if remaining == 8 {
                            "8 observation attempts"
                        } else {
                            "identity does not match"
                        }),
                        "{message}"
                    );
                    assert_eq!(io.opens, if remaining == 8 { 8 } else { 2 });
                }
                assert_eq!(fs::read_to_string(epoch_path).unwrap(), record);
            }
        }
    }

    /// Reproduce independent descriptor teardown with real native locks. The
    /// epoch/marker come from DriverLease; only the holders are test-controlled.
    fn release_fixture() -> (TempDir, DriverLease, Option<File>, Option<File>) {
        let (work, mut lease) = started_fixture();
        fs::write(work.path().join(".grove/01-impl--work-k1.md"), "old").unwrap();
        let private = File::open(lease.launch.as_ref().unwrap().path().unwrap()).unwrap();
        let directory = File::open(work.path().join(".grove")).unwrap();
        lease.launch.take();
        for file in [&directory, &private] {
            assert_eq!(
                unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
        assert_eq!(
            running(sample(work.path())).relation,
            TreeRelation::SameTree
        );
        (work, lease, Some(directory), Some(private))
    }

    fn assert_observation_releases_guards(work: &Path, private: &Path) {
        // Keep the returned capture (and therefore its tree pin) alive while
        // independently acquiring every advisory lock it could have retained.
        let capture = crate::try_observe(work, &[]);
        assert_eq!(capture.activity, ActivityObservation::Idle);
        for path in [
            work.to_path_buf(),
            work.join(".grove"),
            work.join(".jj/grove/session.epoch"),
            private.to_path_buf(),
        ] {
            let probe = File::open(path).unwrap();
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
        drop(capture);
    }

    #[test]
    fn witness_release_directory_first_reused_identity_and_key_cannot_attach() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease, mut directory, mut private) = release_fixture();
        let original = running(sample(work.path()));
        directory.take();
        fs::remove_dir_all(work.path().join(".grove")).unwrap();
        fs::create_dir(work.path().join(".grove")).unwrap();
        fs::write(work.path().join(".grove/_BRIEF.md"), "replacement").unwrap();
        fs::write(work.path().join(".grove/01-impl--replacement-k1.md"), "new").unwrap();
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        let metadata = pin.directory().metadata().unwrap();
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        let record = fs::read_to_string(&epoch_path).unwrap();
        // Force the ABA premise: old launch numbers equal the replacement's.
        // This is a fixture-only identity model, not observed host inode reuse.
        let record = record
            .replace(
                &format!("observation-tree-device={}", original.tree_identity.0 .0),
                &format!("observation-tree-device={}", metadata.dev()),
            )
            .replace(
                &format!("observation-tree-inode={}", original.tree_identity.0 .1),
                &format!("observation-tree-inode={}", metadata.ino()),
            );
        fs::write(&epoch_path, record).unwrap();
        let before = contents(work.path());
        let capture = crate::try_observe(work.path(), &[]);
        let crate::TreeObservation::Ready(tree) = capture.tree.unwrap() else {
            panic!("replacement tree was not captured")
        };
        assert_eq!(tree.content.unwrap(), b"replacement");
        assert!(
            matches!(capture.activity, ActivityObservation::Busy(_)),
            "released directory falsely attached old work-k1 to replacement-k1: {:?}",
            capture.activity
        );
        private.take();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        assert_observation_releases_guards(
            work.path(),
            &lease.control_dir.join(original.runtime.witness_name),
        );
        assert_eq!(contents(work.path()), before);
    }

    #[test]
    fn witness_release_private_first_and_both_released_are_idle() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease, mut directory, mut private) = release_fixture();
        let original = running(sample(work.path()));
        let before = contents(work.path());
        private.take();
        assert!(!NativeWitnessIo
            .probe(&File::open(work.path().join(".grove")).unwrap())
            .unwrap());
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        directory.take();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        assert_observation_releases_guards(
            work.path(),
            &lease.control_dir.join(original.runtime.witness_name),
        );
        assert_eq!(contents(work.path()), before);
    }

    #[test]
    fn witness_release_after_directory_verification_obeys_final_private_probe() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        struct ReleaseAfterDirectory {
            directory: Option<File>,
            private: Option<File>,
            events: Vec<&'static str>,
        }
        impl WitnessIo for ReleaseAfterDirectory {
            fn probe(&mut self, file: &File) -> Result<bool> {
                let released = NativeWitnessIo.probe(file)?;
                if file.metadata()?.is_dir() {
                    assert!(!released, "positive directory evidence is required");
                    self.events.push("directory verified");
                    self.directory.take();
                    self.private.take();
                    self.events.push("both released");
                } else {
                    assert!(released, "private release must be observed");
                    self.events.push("private released");
                }
                Ok(released)
            }
        }
        let (work, lease, directory, private) = release_fixture();
        let original = running(sample(work.path()));
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        let before = contents(work.path());
        let mut io = ReleaseAfterDirectory {
            directory,
            private,
            events: vec![],
        };
        assert_eq!(
            read_runtime(work.path(), Some(&pin), || {}, &mut io).unwrap(),
            ActivityObservation::Idle
        );
        assert_eq!(
            io.events,
            ["directory verified", "both released", "private released"]
        );
        assert_observation_releases_guards(
            work.path(),
            &lease.control_dir.join(original.runtime.witness_name),
        );
        assert_eq!(contents(work.path()), before);
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
