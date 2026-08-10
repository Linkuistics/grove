use super::{
    auxiliary_marker_paths, prepare_auxiliary, recover_auxiliary, recover_auxiliary_marker,
    AuxiliaryCleanup, AuxiliaryRole, MarkerReplacementStep, ReplacementPublicationStep,
};
use crate::finish_cleanup::{reap_orphaned, FileIdentity};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
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

fn replacement_artifact_path(artifact: &std::path::Path) -> std::path::PathBuf {
    let mut name = artifact.file_name().unwrap().as_encoded_bytes().to_vec();
    name.extend_from_slice(b".filtered");
    artifact.with_file_name(OsString::from_vec(name))
}

/// A deterministic name derived from the artifact that no live document
/// describes. Grove must never move or unlink whatever sits here.
fn unclaimed_replacement_path(artifact: &std::path::Path) -> std::path::PathBuf {
    replacement_artifact_path(artifact)
}

const STAGING_NONCE: &str = "abcdef0123456789abcdef0123456789";

/// Publish a replacement copy the way `replace_artifact_from` does: under a
/// name only this role and attempt could have drawn, carried by identity rather
/// than derived by a reader.
fn stage_replacement(
    artifact: &Path,
    nonce: &str,
    contents: &str,
) -> (OsString, FileIdentity, PathBuf) {
    let name = OsString::from_vec(
        [
            artifact.file_name().unwrap().as_encoded_bytes(),
            b".staging-",
            nonce.as_bytes(),
        ]
        .concat(),
    );
    let path = artifact.with_file_name(&name);
    fs::write(&path, contents).unwrap();
    let metadata = fs::symlink_metadata(&path).unwrap();
    (
        name,
        FileIdentity {
            device: metadata.dev(),
            inode: metadata.ino(),
        },
        path,
    )
}

fn staging_entries(directory: &Path) -> Vec<OsString> {
    let mut names = fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .filter(|name| String::from_utf8_lossy(name.as_encoded_bytes()).contains(".staging-"))
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn entry_identity(path: &Path) -> FileIdentity {
    let metadata = fs::symlink_metadata(path).unwrap();
    FileIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
    }
}

fn staged_marker_path(replacement_state: &std::path::Path) -> std::path::PathBuf {
    let body: serde_json::Value =
        serde_json::from_slice(&fs::read(replacement_state).unwrap()).unwrap();
    let staged_name: Vec<u8> = serde_json::from_value(body["staged_name"].clone()).unwrap();
    replacement_state.with_file_name(OsString::from_vec(staged_name))
}

fn staged_artifact_path(replacement_state: &std::path::Path) -> std::path::PathBuf {
    let body: serde_json::Value =
        serde_json::from_slice(&fs::read(replacement_state).unwrap()).unwrap();
    let staged_name: Vec<u8> =
        serde_json::from_value(body["staged_artifact_name"].clone()).unwrap();
    replacement_state.with_file_name(OsString::from_vec(staged_name))
}

fn prepared_rebinding() -> (TempDir, PathBuf, AuxiliaryCleanup, PathBuf, PathBuf) {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "old index\n").unwrap();
    let mut prepared = prepare_auxiliary(
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
    let (staging_name, staging_identity, _) =
        stage_replacement(&artifact, STAGING_NONCE, "new index\n");
    prepared
        .bind_artifact_replacement(&staging_name, staging_identity)
        .unwrap();
    (temporary, target, prepared, marker, replacement_state)
}

#[test]
fn recovery_publishes_only_the_exact_bound_replacement_inode() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    fs::write(&source, "old index\n").unwrap();
    let mut prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let (staging_name, staging_identity, replacement) =
        stage_replacement(&artifact, STAGING_NONCE, "filtered index\n");
    let expected_inode = staging_identity.inode;

    prepared
        .bind_artifact_replacement(&staging_name, staging_identity)
        .unwrap();
    let recovered = recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();

    assert_eq!(
        fs::symlink_metadata(&artifact).unwrap().ino(),
        expected_inode
    );
    assert!(!replacement.exists());
    recovered.dispose().unwrap();
    assert!(!artifact.exists());
}

#[test]
fn an_interruption_before_artifact_exchange_preserves_both_bound_inodes_for_recovery() {
    let (_temporary, target, mut prepared, _marker, replacement_state) = prepared_rebinding();
    let artifact = prepared.artifact_path().to_path_buf();
    let replacement = staged_artifact_path(&replacement_state);
    let old_inode = fs::symlink_metadata(&artifact).unwrap().ino();
    let replacement_inode = fs::symlink_metadata(&replacement).unwrap().ino();

    let interruption = prepared
        .rebind_artifact_identity_with(|step| {
            if step == MarkerReplacementStep::BeforeArtifactExchange {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "simulated interruption before artifact exchange",
                ));
            }
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{interruption:#}").contains("simulated interruption"));
    assert_eq!(fs::symlink_metadata(&artifact).unwrap().ino(), old_inode);
    assert_eq!(
        fs::symlink_metadata(&replacement).unwrap().ino(),
        replacement_inode
    );
    recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    assert_eq!(
        fs::symlink_metadata(&artifact).unwrap().ino(),
        replacement_inode
    );
    assert!(!replacement.exists());
}

#[test]
fn recovery_rejects_an_unbound_regular_replacement_and_preserves_both_artifacts() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    let preserved_intended = temporary.path().join("preserved-intended-index");
    fs::write(&source, "old index\n").unwrap();
    let mut prepared = prepare_auxiliary(
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
    let (staging_name, staging_identity, intended) =
        stage_replacement(&artifact, STAGING_NONCE, "filtered index\n");
    prepared
        .bind_artifact_replacement(&staging_name, staging_identity)
        .unwrap();
    fs::rename(&intended, &preserved_intended).unwrap();
    fs::write(&intended, "external index\n").unwrap();

    let error =
        recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT).unwrap_err();

    assert!(format!("{error:#}").contains("artifact replacement identity"));
    assert_eq!(fs::read_to_string(&artifact).unwrap(), "old index\n");
    assert_eq!(fs::read_to_string(&intended).unwrap(), "external index\n");
    assert_eq!(
        fs::read_to_string(&preserved_intended).unwrap(),
        "filtered index\n"
    );
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
}

#[test]
fn recovery_rejects_a_symlink_in_place_of_the_bound_replacement() {
    use std::os::unix::fs::symlink;

    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    let preserved_intended = temporary.path().join("preserved-intended-index");
    let victim = temporary.path().join("victim-index");
    fs::write(&source, "old index\n").unwrap();
    fs::write(&victim, "external index\n").unwrap();
    let mut prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let (staging_name, staging_identity, intended) =
        stage_replacement(&artifact, STAGING_NONCE, "filtered index\n");
    prepared
        .bind_artifact_replacement(&staging_name, staging_identity)
        .unwrap();
    fs::rename(&intended, &preserved_intended).unwrap();
    symlink(&victim, &intended).unwrap();

    let error =
        recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT).unwrap_err();

    assert!(format!("{error:#}").contains("without following symlinks"));
    assert_eq!(fs::read_to_string(&victim).unwrap(), "external index\n");
    assert_eq!(fs::read_to_string(&artifact).unwrap(), "old index\n");
    assert_eq!(
        fs::read_to_string(&preserved_intended).unwrap(),
        "filtered index\n"
    );
    assert!(intended
        .symlink_metadata()
        .unwrap()
        .file_type()
        .is_symlink());
}

#[test]
fn binding_refuses_a_replacement_outside_this_attempts_staging_namespace() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    let victim = temporary.path().join("victim-index");
    fs::write(&source, "old index\n").unwrap();
    fs::write(&victim, "precious user bytes\n").unwrap();
    let mut prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let victim_inode = fs::symlink_metadata(&victim).unwrap().ino();

    let error = prepared
        .bind_artifact_replacement(OsStr::new("victim-index"), entry_identity(&victim))
        .unwrap_err();

    assert!(
        format!("{error:#}").contains("staging namespace"),
        "{error:#}"
    );
    assert_eq!(
        fs::read_to_string(&victim).unwrap(),
        "precious user bytes\n"
    );
    assert_eq!(fs::symlink_metadata(&victim).unwrap().ino(), victim_inode);
    assert_eq!(fs::read_to_string(&artifact).unwrap(), "old index\n");
    assert!(!replacement_state_path(&marker_path(&artifact)).exists());
}

#[test]
fn recovery_refuses_a_replacement_state_that_redirects_the_artifact_exchange() {
    let (temporary, target, prepared, marker, replacement_state) = prepared_rebinding();
    let artifact = prepared.artifact_path().to_path_buf();
    let staged_artifact = staged_artifact_path(&replacement_state);
    let victim = temporary.path().join("victim-index");
    fs::write(&victim, "precious user bytes\n").unwrap();
    let victim_metadata = fs::symlink_metadata(&victim).unwrap();
    let artifact_inode = fs::symlink_metadata(&artifact).unwrap().ino();
    let staged_inode = fs::symlink_metadata(&staged_artifact).unwrap().ino();

    let mut body: serde_json::Value =
        serde_json::from_slice(&fs::read(&replacement_state).unwrap()).unwrap();
    body["staged_artifact_name"] = serde_json::to_value(b"victim-index".to_vec()).unwrap();
    body["replacement_artifact"]["device"] = victim_metadata.dev().into();
    body["replacement_artifact"]["inode"] = victim_metadata.ino().into();
    fs::write(
        &replacement_state,
        serde_json::to_vec_pretty(&body).unwrap(),
    )
    .unwrap();

    let error =
        recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT).unwrap_err();

    assert_eq!(
        fs::read_to_string(&victim).unwrap(),
        "precious user bytes\n",
        "{error:#}"
    );
    assert_eq!(
        fs::symlink_metadata(&victim).unwrap().ino(),
        victim_metadata.ino(),
        "{error:#}"
    );
    assert_eq!(
        fs::symlink_metadata(&artifact).unwrap().ino(),
        artifact_inode,
        "{error:#}"
    );
    assert_eq!(
        fs::symlink_metadata(&staged_artifact).unwrap().ino(),
        staged_inode,
        "{error:#}"
    );
    assert!(
        format!("{error:#}").contains("does not match its role and attempt"),
        "{error:#}"
    );
    assert!(marker.is_file());
}

#[test]
fn recovery_refuses_a_substituted_staged_marker_before_exchanging_artifacts() {
    let (temporary, target, prepared, marker, replacement_state) = prepared_rebinding();
    let artifact = prepared.artifact_path().to_path_buf();
    let staged_marker = staged_marker_path(&replacement_state);
    let staged_artifact = staged_artifact_path(&replacement_state);
    let preserved = temporary.path().join("preserved-staged-marker");
    let artifact_inode = fs::symlink_metadata(&artifact).unwrap().ino();
    let staged_inode = fs::symlink_metadata(&staged_artifact).unwrap().ino();
    fs::rename(&staged_marker, &preserved).unwrap();
    fs::write(&staged_marker, "foreign marker bytes\n").unwrap();

    let error =
        recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT).unwrap_err();

    assert_eq!(
        fs::symlink_metadata(&artifact).unwrap().ino(),
        artifact_inode,
        "{error:#}"
    );
    assert_eq!(
        fs::symlink_metadata(&staged_artifact).unwrap().ino(),
        staged_inode,
        "{error:#}"
    );
    assert!(
        format!("{error:#}").contains("parsing finish auxiliary marker"),
        "{error:#}"
    );
    assert!(marker.is_file());
    assert!(replacement_state.is_file());
}

/// A process death between the staging copy and the state document that owns it
/// is the one interruption that leaves a replacement nothing describes. Because
/// the staging name is drawn rather than derived, that leftover blocks nothing:
/// the same attempt recovers, disposes and prepares again.
#[test]
fn an_interrupted_replacement_publication_leaves_a_clean_same_attempt_retry() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    let filtered = temporary.path().join("filtered-index");
    fs::write(&source, "old index\n").unwrap();
    fs::write(&filtered, "filtered index\n").unwrap();
    let mut prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let marker = marker_path(&artifact);
    let artifact_inode = entry_identity(&artifact).inode;

    // A returned error still unwinds its own staging entry, so death — not an
    // error — is what leaves one behind.
    let staged = stage_replacement(&artifact, STAGING_NONCE, "abandoned copy\n").2;
    let interruption = prepared
        .replace_artifact_from_with(&filtered, |step| {
            if step == ReplacementPublicationStep::BeforeStatePublication {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "simulated death before the state document",
                ));
            }
            Ok(())
        })
        .unwrap_err();

    assert!(
        format!("{interruption:#}").contains("simulated death"),
        "{interruption:#}"
    );
    assert!(!replacement_state_path(&marker).exists());
    assert_eq!(entry_identity(&artifact).inode, artifact_inode);

    let recovered = recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    recovered.dispose().unwrap();
    assert!(!artifact.exists());
    assert!(!marker.exists());
    assert!(
        staged.is_file(),
        "an entry no live document describes must be left alone"
    );
    prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
}

/// The replacement is published under a drawn name, never one a reader can
/// derive from the role and attempt, so an unrelated entry sitting at the name
/// the earlier protocol claimed neither blocks the publication nor loses its
/// bytes — and the completed exchange leaves no staging entry behind.
#[test]
fn publication_claims_no_name_derivable_from_the_role_and_attempt() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    let filtered = temporary.path().join("filtered-index");
    fs::write(&source, "old index\n").unwrap();
    fs::write(&filtered, "filtered index\n").unwrap();
    let mut prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let foreign = unclaimed_replacement_path(&artifact);
    fs::write(&foreign, "precious user bytes\n").unwrap();
    let foreign_inode = entry_identity(&foreign).inode;

    prepared.replace_artifact_from(&filtered).unwrap();

    assert_eq!(
        fs::read_to_string(&artifact).unwrap(),
        "filtered index\n",
        "the artifact name must have adopted the replacement"
    );
    assert_eq!(
        fs::read_to_string(&foreign).unwrap(),
        "precious user bytes\n"
    );
    assert_eq!(entry_identity(&foreign).inode, foreign_inode);
    assert!(!replacement_state_path(&marker_path(&artifact)).exists());
    assert!(
        staging_entries(temporary.path()).is_empty(),
        "a settled replacement leaves no staging entry: {:?}",
        staging_entries(temporary.path())
    );
}

/// An interruption after the state document is durable is recovered forward:
/// the document names both inodes, so recovery completes the exchange it
/// describes rather than guessing from a name.
#[test]
fn an_interruption_after_the_state_document_recovers_the_new_identity() {
    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    let filtered = temporary.path().join("filtered-index");
    fs::write(&source, "old index\n").unwrap();
    fs::write(&filtered, "filtered index\n").unwrap();
    let mut prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();

    let interruption = prepared
        .replace_artifact_from_with(&filtered, |step| {
            if step == ReplacementPublicationStep::AfterStatePublication {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "simulated death after the state document",
                ));
            }
            Ok(())
        })
        .unwrap_err();

    assert!(
        format!("{interruption:#}").contains("simulated death"),
        "{interruption:#}"
    );
    assert!(replacement_state_path(&marker_path(&artifact)).is_file());

    let recovered = recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();

    assert_eq!(fs::read_to_string(&artifact).unwrap(), "filtered index\n");
    assert!(!replacement_state_path(&marker_path(&artifact)).exists());
    assert!(staging_entries(temporary.path()).is_empty());
    recovered.dispose().unwrap();
    assert!(!artifact.exists());
}

/// Disposal must remove only the two entries it validated. `.filtered` is the
/// name an earlier protocol claimed before publishing the document that owned
/// it, so an entry there was never proven to be Grove's — a substitution landing
/// in `replace_artifact_from`'s post-copy identity check leaves a foreign one.
#[test]
fn disposal_never_removes_a_replacement_entry_the_auxiliary_did_not_create() {
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
    let foreign = unclaimed_replacement_path(&artifact);
    fs::write(&foreign, "precious user bytes\n").unwrap();
    let foreign_inode = fs::symlink_metadata(&foreign).unwrap().ino();

    prepared.dispose().unwrap();

    assert!(
        foreign.exists(),
        "disposal unlinked an entry the auxiliary never created"
    );
    assert_eq!(
        fs::read_to_string(&foreign).unwrap(),
        "precious user bytes\n"
    );
    assert_eq!(fs::symlink_metadata(&foreign).unwrap().ino(), foreign_inode);
}

#[test]
fn recovery_never_follows_a_symlink_at_a_derivable_replacement_name() {
    use std::os::unix::fs::symlink;

    let temporary = TempDir::new().unwrap();
    let source = temporary.path().join("source-index");
    let target = temporary.path().join("index");
    let victim = temporary.path().join("victim-index");
    fs::write(&source, "old index\n").unwrap();
    fs::write(&victim, "precious user bytes\n").unwrap();
    let prepared = prepare_auxiliary(
        &source,
        &target,
        AuxiliaryRole::GitIndexSuccess,
        HANDLE,
        ATTEMPT,
    )
    .unwrap();
    let artifact = prepared.artifact_path().to_path_buf();
    let unbound = replacement_artifact_path(&artifact);
    symlink(&victim, &unbound).unwrap();

    let recovered = recover_auxiliary(&target, AuxiliaryRole::GitIndexSuccess, HANDLE, ATTEMPT)
        .unwrap()
        .unwrap();
    recovered.dispose().unwrap();

    assert_eq!(
        fs::read_to_string(&victim).unwrap(),
        "precious user bytes\n"
    );
    assert!(unbound.symlink_metadata().unwrap().file_type().is_symlink());
    assert!(!artifact.exists());
    assert!(!marker_path(&artifact).exists());
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
