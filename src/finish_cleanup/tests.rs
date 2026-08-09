use super::{marker_paths, prepare_quarantine, CleanupStep, QuarantineCleanup};
use std::fs;
use std::io;
use std::os::unix::fs::symlink;
use std::os::unix::fs::MetadataExt;
use tempfile::TempDir;

const FINISH_HANDLE: &str = "finish-k2";
const ATTEMPT: &str = "11111111111111111111111111111111";

struct Fixture {
    _temporary: TempDir,
    grove_root: std::path::PathBuf,
    control_directory: std::path::PathBuf,
    quarantine: std::path::PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temporary = TempDir::new().unwrap();
        let grove_root = temporary.path().join(".grove");
        let control_directory = temporary.path().join("control");
        fs::create_dir(&grove_root).unwrap();
        fs::create_dir(&control_directory).unwrap();
        fs::write(grove_root.join("FORMAT"), "session-kinds-v1\n").unwrap();
        fs::create_dir(grove_root.join("FINISHING-finish-k2")).unwrap();
        fs::write(
            grove_root.join("FINISHING-finish-k2/MANIFEST.json"),
            "manifest\n",
        )
        .unwrap();
        let quarantine = control_directory.join(format!("FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
        Self {
            _temporary: temporary,
            grove_root,
            control_directory,
            quarantine,
        }
    }

    fn prepare_and_handoff(&self) -> QuarantineCleanup {
        let cleanup =
            prepare_quarantine(&self.grove_root, &self.quarantine, FINISH_HANDLE, ATTEMPT).unwrap();
        cleanup.handoff(&self.grove_root).unwrap();
        cleanup
    }
}

#[test]
fn a_marked_quarantine_is_removed_with_its_marker() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();

    cleanup.dispose().unwrap();

    assert!(!fixture.quarantine.exists());
    assert!(!marker.exists());
}

#[test]
fn handoff_does_not_replace_a_foreign_quarantine_destination() {
    let fixture = Fixture::new();
    let cleanup = prepare_quarantine(
        &fixture.grove_root,
        &fixture.quarantine,
        FINISH_HANDLE,
        ATTEMPT,
    )
    .unwrap();
    fs::create_dir(&fixture.quarantine).unwrap();
    let foreign_inode = fs::symlink_metadata(&fixture.quarantine).unwrap().ino();

    let error = cleanup.handoff(&fixture.grove_root).unwrap_err();

    assert!(format!("{error:#}").contains("handing off marked task root"));
    assert!(fixture.grove_root.is_dir());
    assert_eq!(
        fs::symlink_metadata(&fixture.quarantine).unwrap().ino(),
        foreign_inode
    );
    assert!(cleanup.marker_path().is_file());
}

#[test]
fn failed_proof_restore_does_not_replace_a_new_task_root() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    fs::create_dir(&fixture.grove_root).unwrap();
    let replacement_inode = fs::symlink_metadata(&fixture.grove_root).unwrap().ino();

    let error = cleanup.restore(&fixture.grove_root).unwrap_err();

    assert!(format!("{error:#}").contains("restoring marked task root"));
    assert_eq!(
        fs::symlink_metadata(&fixture.grove_root).unwrap().ino(),
        replacement_inode
    );
    assert!(fixture.quarantine.is_dir());
    assert!(cleanup.marker_path().is_file());
}

#[test]
fn interrupted_cleanup_keeps_the_marker_and_retries_from_the_claimed_name() {
    let fixture = Fixture::new();
    fs::write(fixture.grove_root.join("first"), "first\n").unwrap();
    fs::write(fixture.grove_root.join("second"), "second\n").unwrap();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let mut removed_one = false;

    let error = cleanup
        .dispose_with(|step| {
            if let CleanupStep::BeforeEntry(_) = step {
                if removed_one {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "simulated persistent cleanup refusal",
                    ));
                }
                removed_one = true;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("simulated persistent cleanup refusal"));
    assert!(marker.is_file());
    assert!(!fixture.quarantine.exists());
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    assert!(claimed.is_dir());

    QuarantineCleanup::from_marker(&marker)
        .unwrap()
        .dispose()
        .unwrap();

    assert!(!claimed.exists());
    assert!(!marker.exists());
}

#[test]
fn persistent_cleanup_failure_keeps_retryable_claim_and_evidence() {
    let fixture = Fixture::new();
    fs::write(fixture.grove_root.join("blocked"), "blocked\n").unwrap();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));

    for _ in 0..2 {
        let error = QuarantineCleanup::from_marker(&marker)
            .unwrap()
            .dispose_with(|step| {
                if let CleanupStep::BeforeEntry(_) = step {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "simulated persistent cleanup refusal",
                    ));
                }
                Ok(())
            })
            .unwrap_err();

        assert!(format!("{error:#}").contains("persistent cleanup refusal"));
        assert!(claimed.join("blocked").is_file());
        assert!(marker.is_file());
    }
}

#[test]
fn a_replacement_at_the_quarantine_path_is_never_deleted() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let original = fixture.control_directory.join("preserved-original");
    fs::rename(&fixture.quarantine, &original).unwrap();
    fs::create_dir(&fixture.quarantine).unwrap();
    fs::write(fixture.quarantine.join("replacement"), "keep\n").unwrap();

    let error = cleanup.dispose().unwrap_err();

    assert!(format!("{error:#}").contains("identity"));
    assert_eq!(
        fs::read_to_string(fixture.quarantine.join("replacement")).unwrap(),
        "keep\n"
    );
    assert!(original.is_dir());
    assert!(cleanup.marker_path().is_file());
}

#[test]
fn a_replacement_racing_the_atomic_claim_is_never_deleted() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let original = fixture.control_directory.join("preserved-original");
    let replacement_body = fixture.quarantine.join("replacement");
    let mut replaced = false;

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BeforeClaim && !replaced {
                fs::rename(&fixture.quarantine, &original)?;
                fs::create_dir(&fixture.quarantine)?;
                fs::write(&replacement_body, "keep\n")?;
                replaced = true;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("identity changed"));
    assert!(original.is_dir());
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    assert_eq!(
        fs::read_to_string(fixture.quarantine.join("replacement")).unwrap(),
        "keep\n"
    );
    assert!(!claimed.exists());
    assert!(cleanup.marker_path().is_file());
}

#[test]
fn a_reaping_destination_created_during_claim_is_not_replaced() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    let mut foreign_inode = None;

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BeforeClaim {
                fs::create_dir(&claimed)?;
                foreign_inode = Some(fs::symlink_metadata(&claimed)?.ino());
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("claiming finish quarantine"));
    assert_eq!(
        fs::symlink_metadata(&claimed).unwrap().ino(),
        foreign_inode.unwrap()
    );
    assert!(fixture.quarantine.is_dir());
    assert!(cleanup.marker_path().is_file());
}

#[test]
fn a_root_replaced_before_final_removal_is_not_deleted() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    let original = fixture.control_directory.join("preserved-empty-original");
    let mut replacement_inode = None;

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BeforeRootRemoval {
                fs::rename(&claimed, &original)?;
                fs::create_dir(&claimed)?;
                replacement_inode = Some(fs::symlink_metadata(&claimed)?.ino());
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("identity changed"));
    assert!(original.is_dir());
    assert_eq!(
        fs::symlink_metadata(&claimed).unwrap().ino(),
        replacement_inode.unwrap()
    );
    assert!(cleanup.marker_path().is_file());
}

#[test]
fn quarantine_cleanup_unlinks_symlinks_without_following_targets() {
    let fixture = Fixture::new();
    let external = fixture.control_directory.join("external");
    fs::create_dir(&external).unwrap();
    fs::write(external.join("preserved"), "keep\n").unwrap();
    symlink(&external, fixture.grove_root.join("external-link")).unwrap();
    let cleanup = fixture.prepare_and_handoff();

    cleanup.dispose().unwrap();

    assert_eq!(
        fs::read_to_string(external.join("preserved")).unwrap(),
        "keep\n"
    );
}

#[test]
fn corrupt_cleanup_markers_leave_the_quarantine_untouched() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    fs::write(&marker, "not a cleanup marker\n").unwrap();

    let error = QuarantineCleanup::from_marker(&marker).unwrap_err();

    assert!(format!("{error:#}").contains("parsing Grove cleanup marker"));
    assert!(fixture.quarantine.is_dir());
}

#[test]
fn a_marker_moved_to_a_foreign_attempt_name_cannot_authorize_cleanup() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let foreign_marker = fixture
        .control_directory
        .join("GROVE-FINISH-CLEANUP-22222222222222222222222222222222.json");
    fs::rename(cleanup.marker_path(), &foreign_marker).unwrap();

    let error = QuarantineCleanup::from_marker(&foreign_marker).unwrap_err();

    assert!(format!("{error:#}").contains("attempt does not match its path"));
    assert!(fixture.quarantine.is_dir());
    assert!(foreign_marker.is_file());
}

#[test]
fn a_marker_corrupted_after_loading_cannot_authorize_cleanup() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    fs::write(&marker, "corrupted after loading\n").unwrap();

    let error = cleanup.dispose().unwrap_err();

    assert!(
        format!("{error:#}").contains("parsing Grove cleanup marker"),
        "{error:#}"
    );
    assert!(fixture.quarantine.is_dir());
    assert!(marker.is_file());
}

#[test]
fn marker_discovery_ignores_unmarked_and_temporary_entries() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    fs::create_dir(
        fixture
            .control_directory
            .join("FINISHED-foreign-k9-attempt"),
    )
    .unwrap();
    fs::write(
        fixture.control_directory.join(".GROVE-CLEANUP.partial.tmp"),
        "partial\n",
    )
    .unwrap();

    assert_eq!(
        marker_paths(&fixture.control_directory).unwrap(),
        vec![cleanup.marker_path().to_path_buf()]
    );
}
