use super::{
    auxiliary_marker_paths, prepare_auxiliary, recover_auxiliary, recover_auxiliary_marker,
    AuxiliaryRole,
};
use crate::finish_cleanup::reap_orphaned;
use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use tempfile::TempDir;

const HANDLE: &str = "finish-k2";
const ATTEMPT: &str = "11111111111111111111111111111111";
const OTHER_ATTEMPT: &str = "22222222222222222222222222222222";

fn marker_path(artifact: &std::path::Path) -> std::path::PathBuf {
    let mut name = artifact.file_name().unwrap().as_encoded_bytes().to_vec();
    name.extend_from_slice(b".json");
    artifact.with_file_name(OsString::from_vec(name))
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
