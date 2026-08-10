use super::{
    auxiliary_marker_paths, prepare_auxiliary, recover_auxiliary, recover_auxiliary_marker,
    AuxiliaryCleanup, AuxiliaryRole, MarkerReplacementStep,
};
use crate::finish_cleanup::reap_orphaned;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use tempfile::TempDir;

const HANDLE: &str = "finish-k2";
const ATTEMPT: &str = "11111111111111111111111111111111";
const OTHER_ATTEMPT: &str = "22222222222222222222222222222222";

fn marker_path(artifact: &std::path::Path) -> std::path::PathBuf {
    let mut name = artifact.file_name().unwrap().as_encoded_bytes().to_vec();
    name.extend_from_slice(b".json");
    artifact.with_file_name(OsString::from_vec(name))
}

fn replacement_state_path(marker: &std::path::Path) -> std::path::PathBuf {
    let mut name = marker.file_name().unwrap().as_encoded_bytes().to_vec();
    name.extend_from_slice(b".replacing");
    marker.with_file_name(OsString::from_vec(name))
}

fn staged_marker_path(replacement_state: &std::path::Path) -> std::path::PathBuf {
    let body: serde_json::Value =
        serde_json::from_slice(&fs::read(replacement_state).unwrap()).unwrap();
    let staged_name: Vec<u8> = serde_json::from_value(body["staged_name"].clone()).unwrap();
    replacement_state.with_file_name(OsString::from_vec(staged_name))
}

fn prepared_rebinding() -> (TempDir, PathBuf, AuxiliaryCleanup, PathBuf, PathBuf) {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "old index\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let marker = marker_path(&artifact);
    let replacement_state = replacement_state_path(&marker);
    let replacement_artifact = temporary.path().join("replacement-index");
    fs::write(&replacement_artifact, "new index\n").unwrap();
    fs::rename(replacement_artifact, &artifact).unwrap();
    (temporary, target, prepared, marker, replacement_state)
}

#[test]
fn an_interrupted_marker_replacement_recovers_before_exchange() {
    let (_temporary, target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let artifact = prepared.artifact_path().to_path_buf();

    let interruption = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeExchange {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "simulated interruption before marker exchange",
                ));
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{interruption:#}").contains("simulated interruption"));
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
    let staged_marker = staged_marker_path(&replacement_state);
    assert!(staged_marker.is_file());
    let recovered = recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    assert!(!replacement_state.exists());
    assert!(!staged_marker.exists());
    recovered.dispose().unwrap();
    assert!(!marker.exists());
    assert!(!artifact.exists());
}

#[test]
fn an_interrupted_marker_replacement_recovers_after_exchange() {
    let (_temporary, target, mut prepared, marker, replacement_state) = prepared_rebinding();

    let interruption = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::AfterExchange {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "simulated interruption after marker exchange",
                ));
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{interruption:#}").contains("simulated interruption"));
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
    let staged_marker = staged_marker_path(&replacement_state);
    assert!(staged_marker.is_file());
    recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    assert!(marker.is_file());
    assert!(!replacement_state.exists());
    assert!(!staged_marker.exists());
}

#[test]
fn an_interrupted_marker_replacement_recovers_after_retired_marker_removal() {
    let (_temporary, target, mut prepared, marker, replacement_state) = prepared_rebinding();

    let interruption = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::AfterRetiredMarkerRemoval {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "simulated interruption after retired marker removal",
                ));
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{interruption:#}").contains("simulated interruption"));
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
    let staged_marker = staged_marker_path(&replacement_state);
    assert!(!staged_marker.exists());
    recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    assert!(marker.is_file());
    assert!(!replacement_state.exists());
}

#[test]
fn marker_replacement_completes_without_leaving_transition_state() {
    let (_temporary, target, mut prepared, marker, replacement_state) = prepared_rebinding();

    prepared.rebind_artifact_identity().unwrap();

    assert!(marker.is_file());
    assert!(!replacement_state.exists());
    recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
}

#[test]
fn every_published_marker_replacement_state_names_its_handle_and_attempt() {
    let (_temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();

    prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeExchange {
                let staged_marker = staged_marker_path(&replacement_state);
                for path in [&marker, &staged_marker, &replacement_state] {
                    let body: serde_json::Value = serde_json::from_slice(&fs::read(path)?).unwrap();
                    assert_eq!(body["finish_handle"], HANDLE);
                    assert_eq!(body["attempt_identity"], ATTEMPT);
                }
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "leave both parseable states",
                ));
            }
            Ok(())
        })
        .unwrap_err();
}

#[test]
fn a_canonical_marker_substituted_before_exchange_is_preserved() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-marker");
    let foreign = b"foreign marker bytes\n";

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeExchange {
                fs::rename(&marker, &preserved)?;
                fs::write(&marker, foreign)?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("parsing finish auxiliary marker"));
    let staged_marker = staged_marker_path(&replacement_state);
    assert_ne!(fs::read(&staged_marker).unwrap(), foreign);
    assert_eq!(fs::read(&marker).unwrap(), foreign);
    assert!(preserved.is_file());
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn a_replacement_marker_substituted_before_exchange_is_preserved() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-replacement-marker");
    let foreign = b"foreign replacement marker bytes\n";

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeExchange {
                let staged_marker = staged_marker_path(&replacement_state);
                fs::rename(&staged_marker, &preserved)?;
                fs::write(&staged_marker, foreign)?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("parsing finish auxiliary marker"));
    let staged_marker = staged_marker_path(&replacement_state);
    assert_eq!(fs::read(&staged_marker).unwrap(), foreign);
    assert!(preserved.is_file());
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn a_byte_identical_replacement_marker_substitution_is_not_adopted() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-published-marker");
    let mut external_inode = None;

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeExchange {
                let staged_marker = staged_marker_path(&replacement_state);
                let body = fs::read(&staged_marker)?;
                fs::rename(&staged_marker, &preserved)?;
                fs::write(&staged_marker, body)?;
                external_inode = Some(fs::symlink_metadata(&staged_marker)?.ino());
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("identity changed"));
    let staged_marker = staged_marker_path(&replacement_state);
    assert_eq!(
        fs::symlink_metadata(&staged_marker).unwrap().ino(),
        external_inode.unwrap()
    );
    assert!(preserved.is_file());
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn a_byte_identical_replacement_state_substituted_before_exchange_is_not_adopted() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-replacement-state");
    let mut external_inode = None;

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeExchange {
                let body = fs::read(&replacement_state)?;
                fs::rename(&replacement_state, &preserved)?;
                fs::write(&replacement_state, body)?;
                external_inode = Some(fs::symlink_metadata(&replacement_state)?.ino());
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("state identity changed"));
    assert_eq!(
        fs::symlink_metadata(&replacement_state).unwrap().ino(),
        external_inode.unwrap()
    );
    assert!(preserved.is_file());
    assert!(marker.is_file());
    assert!(staged_marker_path(&replacement_state).is_file());
}

#[test]
fn a_canonical_marker_substituted_after_exchange_is_preserved() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-new-marker");
    let foreign = b"foreign canonical marker bytes\n";

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::AfterExchange {
                fs::rename(&marker, &preserved)?;
                fs::write(&marker, foreign)?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("parsing finish auxiliary marker"));
    assert_eq!(fs::read(&marker).unwrap(), foreign);
    assert!(preserved.is_file());
    assert!(staged_marker_path(&replacement_state).is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn a_byte_identical_canonical_marker_substitution_is_not_adopted() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-canonical-marker");
    let mut external_inode = None;

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::AfterExchange {
                let body = fs::read(&marker)?;
                fs::rename(&marker, &preserved)?;
                fs::write(&marker, body)?;
                external_inode = Some(fs::symlink_metadata(&marker)?.ino());
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("identity changed"));
    assert_eq!(
        fs::symlink_metadata(&marker).unwrap().ino(),
        external_inode.unwrap()
    );
    assert!(preserved.is_file());
    assert!(staged_marker_path(&replacement_state).is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn a_byte_identical_replacement_state_substituted_after_exchange_is_not_adopted() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-exchanged-state");
    let mut external_inode = None;

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::AfterExchange {
                let body = fs::read(&replacement_state)?;
                fs::rename(&replacement_state, &preserved)?;
                fs::write(&replacement_state, body)?;
                external_inode = Some(fs::symlink_metadata(&replacement_state)?.ino());
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("state identity changed"));
    assert_eq!(
        fs::symlink_metadata(&replacement_state).unwrap().ino(),
        external_inode.unwrap()
    );
    assert!(preserved.is_file());
    assert!(marker.is_file());
    assert!(staged_marker_path(&replacement_state).is_file());
}

#[test]
fn a_retired_marker_substituted_after_exchange_is_preserved() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-old-marker");
    let foreign = b"foreign retired marker bytes\n";

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::AfterExchange {
                let staged_marker = staged_marker_path(&replacement_state);
                fs::rename(&staged_marker, &preserved)?;
                fs::write(&staged_marker, foreign)?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("parsing finish auxiliary marker"));
    assert_eq!(
        fs::read(staged_marker_path(&replacement_state)).unwrap(),
        foreign
    );
    assert!(preserved.is_file());
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn a_retired_marker_substituted_before_removal_is_preserved() {
    let (temporary, _target, mut prepared, marker, replacement_state) = prepared_rebinding();
    let preserved = temporary.path().join("preserved-retired-marker");
    let foreign = b"foreign retired marker bytes\n";

    let error = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeRetiredMarkerRemoval {
                let staged_marker = staged_marker_path(&replacement_state);
                fs::rename(&staged_marker, &preserved)?;
                fs::write(&staged_marker, foreign)?;
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{error:#}").contains("parsing finish auxiliary marker"));
    assert_eq!(
        fs::read(staged_marker_path(&replacement_state)).unwrap(),
        foreign
    );
    assert!(preserved.is_file());
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn recovery_finishes_an_interrupted_disposal() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "index bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let marker = marker_path(&artifact);
    fs::remove_file(&artifact).unwrap();

    let recovered = recover_auxiliary(&target, AuxiliaryRole::GitIndexBackup, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    recovered.dispose().unwrap();

    assert!(!artifact.exists());
    assert!(!marker.exists());
}

#[test]
fn recovery_finishes_an_interrupted_activation_only_for_the_marked_target() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "successful index\n").unwrap();
    fs::write(&target, "original index\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let marker = marker_path(&artifact);
    fs::rename(&artifact, &target).unwrap();

    let recovered = recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    recovered.activate().unwrap();

    assert_eq!(fs::read_to_string(target).unwrap(), "successful index\n");
    assert!(!artifact.exists());
    assert!(!marker.exists());
}

#[test]
fn recovery_rejects_the_wrong_handle_role_and_target_binding() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "index bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let marker = marker_path(prepared.artifact_path());

    let wrong_handle = recover_auxiliary(
        &target,
        AuxiliaryRole::GitIndexBackup,
        "other-finish-k9",
        ATTEMPT,
    )
    .unwrap_err();
    assert!(format!("{wrong_handle:#}").contains("role, handle, or attempt"));

    let wrong_target = recover_auxiliary(
        &temporary.path().join("other-index"),
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap_err();
    assert!(format!("{wrong_target:#}").contains("does not match"));

    let mut body: serde_json::Value = serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
    body["role"] = serde_json::Value::String("git-index-success".to_owned());
    fs::write(&marker, serde_json::to_vec_pretty(&body).unwrap()).unwrap();
    let wrong_role =
        recover_auxiliary(&target, AuxiliaryRole::GitIndexBackup, HANDLE, ATTEMPT).unwrap_err();
    assert!(format!("{wrong_role:#}").contains("role, handle, or attempt"));
}

#[test]
fn recovery_rejects_replaced_and_symlinked_artifacts_without_touching_them() {
    use std::os::unix::fs::symlink;

    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "index bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let original = temporary.path().join("original-artifact");
    fs::rename(&artifact, &original).unwrap();
    fs::write(&artifact, "replacement\n").unwrap();

    let replacement_error =
        recover_auxiliary(&target, AuxiliaryRole::GitIndexBackup, HANDLE, ATTEMPT).unwrap_err();
    assert!(format!("{replacement_error:#}").contains("identity does not match"));
    assert_eq!(fs::read_to_string(&artifact).unwrap(), "replacement\n");

    fs::remove_file(&artifact).unwrap();
    let victim = temporary.path().join("victim");
    fs::write(&victim, "victim\n").unwrap();
    symlink(&victim, &artifact).unwrap();
    let symlink_error =
        recover_auxiliary(&target, AuxiliaryRole::GitIndexBackup, HANDLE, ATTEMPT).unwrap_err();
    assert!(format!("{symlink_error:#}").contains("without following symlinks"));
    assert_eq!(fs::read_to_string(victim).unwrap(), "victim\n");
    assert!(artifact
        .symlink_metadata()
        .unwrap()
        .file_type()
        .is_symlink());
}

#[test]
fn a_new_attempt_never_adopts_an_old_attempts_artifact() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "index bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();

    assert!(recover_auxiliary(
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        OTHER_ATTEMPT,
    )
    .unwrap()
    .is_none());
    assert!(prepared.artifact_path().is_file());
    assert!(marker_path(prepared.artifact_path()).is_file());
}

#[test]
fn marker_discovery_exposes_only_validated_auxiliaries_for_reaping() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "index bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    fs::write(temporary.path().join("unrelated.json"), "{}\n").unwrap();
    let expected_marker = marker_path(prepared.artifact_path());

    let markers = auxiliary_marker_paths(temporary.path()).unwrap();
    assert_eq!(markers, vec![expected_marker.clone()]);
    let recovered = recover_auxiliary_marker(&expected_marker).unwrap();
    assert_eq!(recovered.marker.role, AuxiliaryRole::GitIndexBackup);
    assert_eq!(recovered.marker.finish_handle, HANDLE);
    assert_eq!(recovered.marker.attempt_identity, ATTEMPT);
    recovered.dispose().unwrap();

    assert!(auxiliary_marker_paths(temporary.path()).unwrap().is_empty());
}

#[test]
fn driver_reaping_discovers_auxiliaries_beside_the_resolved_git_index() {
    let temporary = TempDir::new().unwrap();
    let worktree = temporary.path();
    let initialized = std::process::Command::new("git")
        .current_dir(worktree)
        .args(["init", "-q", "."])
        .status()
        .unwrap();
    assert!(initialized.success());
    fs::create_dir(worktree.join(".git/grove")).unwrap();
    let source = worktree.join(".git/source-index");
    let target = worktree.join(".git/index");
    fs::write(&source, "index bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let marker = marker_path(&artifact);

    reap_orphaned(worktree).unwrap();

    assert!(!artifact.exists());
    assert!(!marker.exists());
}

#[test]
fn failed_restoration_keeps_evidence_without_leaking_an_unmarked_temporary() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "index bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexBackup,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    fs::create_dir(&target).unwrap();

    prepared.restore_target().unwrap_err();

    assert!(prepared.artifact_path().is_file());
    assert!(marker_path(prepared.artifact_path()).is_file());
    let names: Vec<_> = fs::read_dir(temporary.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert!(!names
        .iter()
        .any(|name| { name.as_encoded_bytes().starts_with(b".grove-cleanup-") }));
}
