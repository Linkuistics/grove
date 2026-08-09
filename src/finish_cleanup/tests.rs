use super::{
    marker_paths, prepare_quarantine, prepare_quarantine_with, CleanupOutcome, CleanupOwner,
    CleanupStep, QuarantineCleanup,
};
use std::fs;
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::fs::symlink;
use std::os::unix::fs::MetadataExt;
use std::sync::Mutex;
use std::time::Duration;
use tempfile::TempDir;

const FINISH_HANDLE: &str = "finish-k2";
const ATTEMPT: &str = "11111111111111111111111111111111";
static CLEANUP_TEST_ENVIRONMENT: Mutex<()> = Mutex::new(());

#[test]
fn cleanup_owner_requires_a_canonical_finish_handle() {
    for invalid in ["", "finish", "build-k2", "finish-k0", "finish-k02"] {
        let error = CleanupOwner::new(invalid.to_owned(), ATTEMPT.to_owned()).unwrap_err();
        assert!(
            format!("{error:#}").contains("finish handle"),
            "{invalid}: {error:#}"
        );
    }
}

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

    assert_eq!(cleanup.dispose().unwrap(), CleanupOutcome::Disposed);

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
        assert!(
            format!("{error:#}").contains(&claimed.to_string_lossy().into_owned()),
            "{error:#}"
        );
        assert!(
            format!("{error:#}").contains(&marker.to_string_lossy().into_owned()),
            "{error:#}"
        );
        assert!(claimed.join("blocked").is_file());
        assert!(marker.is_file());
    }
}

#[test]
fn failure_immediately_after_claim_names_only_retryable_paths() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::AfterClaim {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "simulated failure immediately after claim",
                ));
            }
            Ok(())
        })
        .unwrap_err();
    let diagnostic = format!("{error:#}");

    assert!(diagnostic.contains("simulated failure immediately after claim"));
    assert!(diagnostic.contains(claimed.to_string_lossy().as_ref()));
    assert!(diagnostic.contains(marker.to_string_lossy().as_ref()));
    assert!(!diagnostic.contains(fixture.quarantine.to_string_lossy().as_ref()));
    assert!(claimed.is_dir());
    assert!(marker.is_file());
}

#[test]
fn a_stale_marker_reports_that_nothing_was_disposed() {
    let fixture = Fixture::new();
    let cleanup = prepare_quarantine(
        &fixture.grove_root,
        &fixture.quarantine,
        FINISH_HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let marker = cleanup.marker_path().to_path_buf();

    assert_eq!(cleanup.dispose().unwrap(), CleanupOutcome::NothingToDispose);
    assert!(!marker.exists());
    assert!(fixture.grove_root.is_dir());
}

#[test]
fn a_quarantine_created_between_empty_owner_probes_is_not_orphaned() {
    let fixture = Fixture::new();
    let cleanup = prepare_quarantine(
        &fixture.grove_root,
        &fixture.quarantine,
        FINISH_HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let marker = cleanup.marker_path().to_path_buf();

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BetweenOwnerProbes {
                fs::create_dir(&fixture.quarantine)?;
                fs::write(fixture.quarantine.join("replacement"), "keep\n")?;
            }
            Ok(())
        })
        .unwrap_err();
    let diagnostic = format!("{error:#}");

    assert!(
        diagnostic.contains("cleanup owner changed during observation"),
        "{error:#}"
    );
    assert!(
        diagnostic.contains("observed owners (false, false), then (true, false)"),
        "{error:#}"
    );
    assert!(!diagnostic.contains("changed from false to false"));
    assert_eq!(
        fs::read_to_string(fixture.quarantine.join("replacement")).unwrap(),
        "keep\n"
    );
    assert!(marker.is_file());
}

#[test]
fn a_quarantine_created_before_empty_owner_marker_removal_keeps_its_evidence() {
    let fixture = Fixture::new();
    let cleanup = prepare_quarantine(
        &fixture.grove_root,
        &fixture.quarantine,
        FINISH_HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let marker = cleanup.marker_path().to_path_buf();

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BeforeEmptyOwnerMarkerRemoval {
                fs::create_dir(&fixture.quarantine)?;
                fs::write(fixture.quarantine.join("replacement"), "keep\n")?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(
        format!("{error:#}").contains("cleanup owner appeared before marker removal"),
        "{error:#}"
    );
    assert_eq!(
        fs::read_to_string(fixture.quarantine.join("replacement")).unwrap(),
        "keep\n"
    );
    assert!(marker.is_file());
}

#[test]
fn a_quarantine_created_while_a_claimed_owner_exists_is_not_orphaned() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    fs::rename(&fixture.quarantine, &claimed).unwrap();
    let claimed_inode = fs::symlink_metadata(&claimed).unwrap().ino();

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BetweenOwnerProbes {
                fs::create_dir(&fixture.quarantine)?;
                fs::write(fixture.quarantine.join("replacement"), "keep\n")?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(
        format!("{error:#}").contains("cleanup owner changed during observation"),
        "{error:#}"
    );
    assert_eq!(
        fs::read_to_string(fixture.quarantine.join("replacement")).unwrap(),
        "keep\n"
    );
    assert_eq!(fs::symlink_metadata(&claimed).unwrap().ino(), claimed_inode);
    assert!(marker.is_file());
}

#[test]
fn simultaneous_quarantine_names_are_refused_without_mutation() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    let interruption = cleanup
        .dispose_with(|step| match step {
            CleanupStep::BeforeEntry(_) => Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "leave claimed quarantine intact",
            )),
            _ => Ok(()),
        })
        .unwrap_err();
    assert!(format!("{interruption:#}").contains("leave claimed quarantine intact"));
    fs::create_dir(&fixture.quarantine).unwrap();
    let quarantine_inode = fs::symlink_metadata(&fixture.quarantine).unwrap().ino();
    let claimed_inode = fs::symlink_metadata(&claimed).unwrap().ino();

    let error = QuarantineCleanup::from_marker(&marker)
        .unwrap()
        .dispose()
        .unwrap_err();

    assert!(format!("{error:#}").contains("ambiguous Grove cleanup owner"));
    assert_eq!(
        fs::symlink_metadata(&fixture.quarantine).unwrap().ino(),
        quarantine_inode
    );
    assert_eq!(fs::symlink_metadata(&claimed).unwrap().ino(), claimed_inode);
    assert!(marker.is_file());
}

#[test]
fn well_formed_foreign_marker_versions_and_kinds_are_refused() {
    for (field, replacement, expected) in [
        (
            "version",
            serde_json::json!(2),
            "unsupported Grove cleanup marker version",
        ),
        (
            "kind",
            serde_json::json!("foreign"),
            "unsupported Grove cleanup marker kind",
        ),
    ] {
        let fixture = Fixture::new();
        let cleanup = fixture.prepare_and_handoff();
        let marker = cleanup.marker_path().to_path_buf();
        let mut body: serde_json::Value =
            serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
        body[field] = replacement;
        fs::write(&marker, serde_json::to_vec_pretty(&body).unwrap()).unwrap();

        let error = QuarantineCleanup::from_marker(&marker).unwrap_err();

        assert!(format!("{error:#}").contains(expected), "{error:#}");
        assert!(fixture.quarantine.is_dir());
        assert!(marker.is_file());
    }
}

#[test]
fn prepare_refuses_an_overlong_claim_name_before_publishing_a_marker() {
    let fixture = Fixture::new();
    let control_directory = fs::File::open(&fixture.control_directory).unwrap();
    // SAFETY: the descriptor is an open directory and `_PC_NAME_MAX` is a
    // read-only query for that filesystem.
    let name_max = unsafe { libc::fpathconf(control_directory.as_raw_fd(), libc::_PC_NAME_MAX) };
    assert!(name_max > 64, "unexpected NAME_MAX {name_max}");
    let quarantine_overhead = b"FINISHED-".len() + 1 + ATTEMPT.len();
    let finish_handle = "h".repeat(name_max as usize - quarantine_overhead - 4);
    let quarantine = fixture
        .control_directory
        .join(format!("FINISHED-{finish_handle}-{ATTEMPT}"));

    let error =
        prepare_quarantine(&fixture.grove_root, &quarantine, &finish_handle, ATTEMPT).unwrap_err();

    assert!(
        format!("{error:#}").contains("claimed finish quarantine name"),
        "{error:#}"
    );
    assert!(marker_paths(&fixture.control_directory).unwrap().is_empty());
    assert!(fixture.grove_root.is_dir());
}

#[test]
fn marker_publication_stays_with_the_validated_control_directory() {
    let fixture = Fixture::new();
    let validated_control_directory = fixture.control_directory.with_extension("validated");
    let mut replaced = false;

    prepare_quarantine_with(
        &fixture.grove_root,
        &fixture.quarantine,
        FINISH_HANDLE,
        ATTEMPT,
        |step| {
            if step == CleanupStep::BeforeMarkerPublication && !replaced {
                fs::rename(&fixture.control_directory, &validated_control_directory)?;
                fs::create_dir(&fixture.control_directory)?;
                replaced = true;
            }
            Ok(())
        },
    )
    .unwrap();

    assert_eq!(marker_paths(&validated_control_directory).unwrap().len(), 1);
    assert!(marker_paths(&fixture.control_directory).unwrap().is_empty());
}

#[test]
fn a_substituted_freshly_published_marker_is_rejected() {
    let fixture = Fixture::new();
    let marker = fixture
        .control_directory
        .join(format!("GROVE-FINISH-CLEANUP-{ATTEMPT}.json"));

    let error = prepare_quarantine_with(
        &fixture.grove_root,
        &fixture.quarantine,
        FINISH_HANDLE,
        ATTEMPT,
        |step| {
            if step == CleanupStep::AfterMarkerPublication {
                let mut body: serde_json::Value =
                    serde_json::from_slice(&fs::read(&marker)?).unwrap();
                body["finish_handle"] = serde_json::json!("foreign-k9");
                body["quarantine_name"] =
                    serde_json::json!(format!("FINISHED-foreign-k9-{ATTEMPT}").as_bytes());
                body["claimed_name"] =
                    serde_json::json!(format!("REAPING-FINISHED-foreign-k9-{ATTEMPT}").as_bytes());
                fs::remove_file(&marker)?;
                fs::write(&marker, serde_json::to_vec_pretty(&body).unwrap())?;
            }
            Ok(())
        },
    )
    .unwrap_err();

    assert!(
        format!("{error:#}").contains("finish cleanup marker collision"),
        "{error:#}"
    );
    assert!(fixture.grove_root.is_dir());
    assert!(marker.is_file());
}

#[test]
fn cleanup_pause_timeout_names_the_unreleased_barrier() {
    let _environment = CLEANUP_TEST_ENVIRONMENT
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let fixture = TempDir::new().unwrap();
    let barrier = fixture.path().join("unreleased-cleanup-barrier");
    fs::write(&barrier, "paused\n").unwrap();
    let previous_pause = std::env::var_os("GROVE_TEST_FINISH_CLEANUP_PAUSE_AT");
    let previous_barrier = std::env::var_os("GROVE_TEST_FINISH_CLEANUP_BARRIER");
    std::env::set_var(
        "GROVE_TEST_FINISH_CLEANUP_PAUSE_AT",
        "before-marker-publication",
    );
    std::env::set_var("GROVE_TEST_FINISH_CLEANUP_BARRIER", &barrier);

    let error = super::cleanup_test_checkpoint_with_timeout(
        CleanupStep::BeforeMarkerPublication,
        Duration::ZERO,
    )
    .unwrap_err();

    match previous_pause {
        Some(value) => std::env::set_var("GROVE_TEST_FINISH_CLEANUP_PAUSE_AT", value),
        None => std::env::remove_var("GROVE_TEST_FINISH_CLEANUP_PAUSE_AT"),
    }
    match previous_barrier {
        Some(value) => std::env::set_var("GROVE_TEST_FINISH_CLEANUP_BARRIER", value),
        None => std::env::remove_var("GROVE_TEST_FINISH_CLEANUP_BARRIER"),
    }

    assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    assert!(
        error
            .to_string()
            .contains(barrier.to_string_lossy().as_ref()),
        "{error}"
    );
}

#[test]
fn publication_and_temporary_cleanup_failures_are_both_reported() {
    let fixture = Fixture::new();
    let marker = fixture
        .control_directory
        .join(format!("GROVE-FINISH-CLEANUP-{ATTEMPT}.json"));
    let preserved_temporary = fixture.control_directory.join("preserved-temporary");
    let replacement_temporary = std::cell::RefCell::new(None);

    let error = prepare_quarantine_with(
        &fixture.grove_root,
        &fixture.quarantine,
        FINISH_HANDLE,
        ATTEMPT,
        |step| {
            if let CleanupStep::BeforeTemporaryMarkerPublication(temporary_name) = step {
                let temporary = fixture.control_directory.join(&temporary_name);
                fs::rename(&temporary, &preserved_temporary)?;
                fs::write(&temporary, "replacement temporary\n")?;
                fs::write(&marker, "force publication collision\n")?;
                replacement_temporary.replace(Some(temporary));
            }
            Ok(())
        },
    )
    .unwrap_err();
    let diagnostic = format!("{error:#}");

    assert!(diagnostic.contains("publishing Grove cleanup marker"));
    assert!(
        diagnostic.contains("removing unpublished temporary Grove cleanup marker"),
        "{diagnostic}"
    );
    assert!(diagnostic.contains("identity changed"), "{diagnostic}");
    assert!(preserved_temporary.is_file());
    assert!(replacement_temporary.into_inner().unwrap().is_file());
    assert!(marker.is_file());
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
fn a_regular_file_substituted_before_unlink_is_not_deleted() {
    let fixture = Fixture::new();
    fs::write(fixture.grove_root.join("entry"), "original\n").unwrap();
    let cleanup = fixture.prepare_and_handoff();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    let preserved = claimed.join("preserved-original");
    let replacement = claimed.join("entry");

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BeforeNonDirectoryUnlink("entry".into()) {
                fs::rename(&replacement, &preserved)?;
                fs::write(&replacement, "replacement\n")?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(
        format!("{error:#}").contains("identity changed"),
        "{error:#}"
    );
    assert_eq!(fs::read_to_string(&replacement).unwrap(), "replacement\n");
    assert_eq!(fs::read_to_string(&preserved).unwrap(), "original\n");
    assert!(cleanup.marker_path().is_file());
}

#[test]
fn a_symlink_substituted_before_unlink_is_not_deleted() {
    let fixture = Fixture::new();
    symlink("original-target", fixture.grove_root.join("entry")).unwrap();
    let cleanup = fixture.prepare_and_handoff();
    let claimed = fixture
        .control_directory
        .join(format!("REAPING-FINISHED-{FINISH_HANDLE}-{ATTEMPT}"));
    let preserved = claimed.join("preserved-original");
    let replacement = claimed.join("entry");

    let error = cleanup
        .dispose_with(|step| {
            if step == CleanupStep::BeforeNonDirectoryUnlink("entry".into()) {
                fs::rename(&replacement, &preserved)?;
                symlink("replacement-target", &replacement)?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(
        format!("{error:#}").contains("identity changed"),
        "{error:#}"
    );
    assert_eq!(
        fs::read_link(&replacement).unwrap(),
        std::path::Path::new("replacement-target")
    );
    assert_eq!(
        fs::read_link(&preserved).unwrap(),
        std::path::Path::new("original-target")
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
fn marker_quarantine_name_must_be_one_non_empty_path_component() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let mut body: serde_json::Value = serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
    body["finish_handle"] = serde_json::json!("nested/finish-k2");
    body["quarantine_name"] =
        serde_json::json!(format!("FINISHED-nested/finish-k2-{ATTEMPT}").as_bytes());
    body["claimed_name"] =
        serde_json::json!(format!("REAPING-FINISHED-nested/finish-k2-{ATTEMPT}").as_bytes());
    fs::write(&marker, serde_json::to_vec_pretty(&body).unwrap()).unwrap();

    let error = QuarantineCleanup::from_marker(&marker).unwrap_err();

    assert!(
        format!("{error:#}").contains("quarantine name is not a single non-empty path component"),
        "{error:#}"
    );
    assert!(fixture.quarantine.is_dir());
    assert!(marker.is_file());
}

#[test]
fn marker_claimed_name_must_be_one_non_empty_path_component() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let mut body: serde_json::Value = serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
    body["claimed_name"] = serde_json::json!(b"nested/claim");
    fs::write(&marker, serde_json::to_vec_pretty(&body).unwrap()).unwrap();

    let error = QuarantineCleanup::from_marker(&marker).unwrap_err();

    assert!(
        format!("{error:#}").contains("claimed name is not a single non-empty path component"),
        "{error:#}"
    );
    assert!(fixture.quarantine.is_dir());
    assert!(marker.is_file());
}

#[test]
fn marker_quarantine_name_cannot_be_empty() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let mut body: serde_json::Value = serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
    body["quarantine_name"] = serde_json::json!(b"");
    fs::write(&marker, serde_json::to_vec_pretty(&body).unwrap()).unwrap();

    let error = QuarantineCleanup::from_marker(&marker).unwrap_err();

    assert!(
        format!("{error:#}").contains("quarantine name is not a single non-empty path component"),
        "{error:#}"
    );
    assert!(fixture.quarantine.is_dir());
    assert!(marker.is_file());
}

#[test]
fn marker_claimed_name_cannot_be_empty() {
    let fixture = Fixture::new();
    let cleanup = fixture.prepare_and_handoff();
    let marker = cleanup.marker_path().to_path_buf();
    let mut body: serde_json::Value = serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
    body["claimed_name"] = serde_json::json!(b"");
    fs::write(&marker, serde_json::to_vec_pretty(&body).unwrap()).unwrap();

    let error = QuarantineCleanup::from_marker(&marker).unwrap_err();

    assert!(
        format!("{error:#}").contains("claimed name is not a single non-empty path component"),
        "{error:#}"
    );
    assert!(fixture.quarantine.is_dir());
    assert!(marker.is_file());
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
