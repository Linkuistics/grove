use crate::{driver_lease, finish_cleanup, repo};
use anyhow::{bail, Context, Result};
use finish_cleanup::unix::{
    create_directory_at, create_new_file_at, directory_names, metadata_at, open_directory_at,
    open_file_at, read_link_at, rename_at_noreplace, unlink_at,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

const WITNESS_PREFIX: &[u8] = b"FINISHING-";
const PREPARING_WITNESS_PREFIX: &[u8] = b"PREPARING-FINISH-";
const MANIFEST_FILE: &str = "MANIFEST.json";
const ORIGINAL_TREE_DIRECTORY: &str = "original";
const READY_FILE: &str = "READY";

pub(crate) fn finish(worktree: &Path, grove_root: &Path, finish_handle: &str) -> Result<()> {
    let preflight = preflight_root(worktree, grove_root)?;
    let attempt_identity = finish_attempt_identity()?;
    let preparing = begin_preparing_witness(
        grove_root,
        &preflight.grove_root_directory,
        finish_handle,
        &attempt_identity,
    )?;
    if let Err(error) = finish_test_checkpoint("after-preparing-witness") {
        return Err(remove_preparing_after_error(preparing, error));
    }
    let prepared_commit = match repo::prepare_finish(worktree, finish_handle, &attempt_identity) {
        Ok(prepared_commit) => prepared_commit,
        Err(error) => return Err(remove_preparing_after_error(preparing, error)),
    };
    if let Err(error) = finish_test_checkpoint("after-repository-preparation") {
        return Err(abort_prepared_finish_with_preparing(
            prepared_commit,
            preparing,
            error,
        ));
    }
    let transaction = prepare_transaction(
        grove_root,
        preflight,
        finish_handle,
        attempt_identity,
        preparing,
        prepared_commit,
    )?;
    let EvacuatedTransaction {
        transaction,
        directories,
        prepared_commit,
    } = evacuate(transaction)?;
    finish_test_checkpoint("after-evacuation")?;
    revalidate_transaction_directories(&transaction, &directories)?;
    match prepared_commit.commit(finish_handle, &transaction.manifest.attempt_identity) {
        repo::FinishCommitOutcome::Committed(proof) => {
            finish_test_checkpoint("after-commit")?;
            quarantine_and_dispose(
                grove_root,
                &transaction.quarantine_path,
                &transaction.manifest.finish_handle,
                &transaction.manifest.attempt_identity,
                &proof,
            )
        }
        repo::FinishCommitOutcome::NotCommitted { proof, error } => {
            match rollback(&transaction, &directories, proof) {
                Ok(()) => Err(error),
                Err(rollback_error) => Err(error.context(format!(
                    "finish rollback failed: {rollback_error:#}; recovery remains pending at {}",
                    transaction.witness_path.display()
                ))),
            }
        }
        repo::FinishCommitOutcome::RecoveryPending(error) => Err(error.context(format!(
            "Recovery pending at {}; preserve divergent work, restore the recorded start or the exact teardown result, then retry",
            transaction.witness_path.display()
        ))),
    }
}

/// Deterministic process-interruption seam for black-box recovery tests. The
/// variable is deliberately test-prefixed and is not user configuration.
fn finish_test_checkpoint(name: &str) -> Result<()> {
    if std::env::var("GROVE_TEST_FINISH_EXIT_AT").as_deref() == Ok(name) {
        std::process::exit(86);
    }
    if std::env::var("GROVE_TEST_FINISH_FAIL_AT").as_deref() == Ok(name) {
        bail!("injected finish interruption at {name}");
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct RootEntryDigest {
    path: Vec<u8>,
    entry_type: EntryType,
    mode: u32,
    digest: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
enum EntryType {
    Directory,
    File,
    Symlink,
}

struct FinishPreflight {
    grove_root_directory: File,
    entry_digests: Vec<RootEntryDigest>,
    quarantine_directory: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FinishManifest {
    version: u32,
    finish_handle: String,
    attempt_identity: String,
    repository_anchor: repo::FinishStartAnchor,
    deletion_fingerprint: [u8; 32],
    entries: Vec<RootEntryDigest>,
}

struct FinishTransaction {
    grove_root: PathBuf,
    witness_path: PathBuf,
    original_tree: PathBuf,
    quarantine_path: PathBuf,
    manifest: FinishManifest,
}

struct PreparedTransaction {
    transaction: FinishTransaction,
    preparing: PreparingFinish,
    prepared_commit: repo::PreparedFinish,
}

struct EvacuatedTransaction {
    transaction: FinishTransaction,
    directories: TransactionDirectories,
    prepared_commit: repo::PreparedFinish,
}

struct TransactionDirectories {
    grove_root: File,
    witness: File,
    original_tree: File,
    manifest: File,
    ready: File,
    witness_name: OsString,
}

struct PendingFinish {
    witness_path: PathBuf,
    manifest: FinishManifest,
    directories: TransactionDirectories,
    layout: ReadyWitnessLayout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReadyWitnessLayout {
    NotEvacuated,
    Evacuated,
}

struct PreparingFinish {
    finish_handle: String,
    attempt_identity: String,
    grove_root_directory: File,
    witness_directory: File,
    original_tree_directory: Option<File>,
    manifest_file: Option<File>,
    ready_file: Option<File>,
    witness_name: OsString,
    witness_path: PathBuf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FinishRecovery {
    None,
    RolledBack,
    Committed,
}

pub(crate) fn cleanup_owner(grove_root: &Path) -> Result<Option<finish_cleanup::CleanupOwner>> {
    if let Some(preparing) = pending_preparing(grove_root)? {
        return finish_cleanup::CleanupOwner::new(
            preparing.finish_handle,
            preparing.attempt_identity,
        )
        .map(Some);
    }
    let Some(pending) = pending_manifest(grove_root)? else {
        return Ok(None);
    };
    finish_cleanup::CleanupOwner::new(
        pending.manifest.finish_handle,
        pending.manifest.attempt_identity,
    )
    .map(Some)
}

pub(crate) fn recover_pending(worktree: &Path, grove_root: &Path) -> Result<FinishRecovery> {
    if let Some(preparing) = pending_preparing(grove_root)? {
        repo::abort_preparing_finish(
            worktree,
            &preparing.finish_handle,
            &preparing.attempt_identity,
        )
        .with_context(|| {
            format!(
                "Recovery pending: aborting repository preparation owned by {}",
                preparing.witness_path.display()
            )
        })?;
        remove_preparing_witness(preparing)?;
        return Ok(FinishRecovery::RolledBack);
    }
    let Some(pending) = pending_manifest(grove_root)? else {
        return Ok(FinishRecovery::None);
    };
    let witness_path = pending.witness_path;
    let manifest = pending.manifest;
    let directories = pending.directories;
    let layout = pending.layout;
    let original_tree = witness_path.join(ORIGINAL_TREE_DIRECTORY);
    let quarantine_directory = repo::workspace_control(worktree)?
        .control_dir()
        .to_path_buf();
    let quarantine_path = quarantine_directory.join(format!(
        "FINISHED-{}-{}",
        manifest.finish_handle, manifest.attempt_identity
    ));
    let transaction = FinishTransaction {
        grove_root: grove_root.to_path_buf(),
        witness_path: witness_path.clone(),
        original_tree,
        quarantine_path,
        manifest,
    };
    match repo::recover_finish(
        worktree,
        &transaction.manifest.repository_anchor,
        &transaction.manifest.finish_handle,
        &transaction.manifest.attempt_identity,
        transaction.manifest.deletion_fingerprint,
    ) {
        repo::FinishRecoveryOutcome::Committed(proof) => {
            if layout == ReadyWitnessLayout::NotEvacuated {
                bail!(
                    "Recovery pending: repository has the exact finish commit, but the ready witness is not-yet-evacuated at {}; preserve the live task tree and witness for manual reconciliation",
                    witness_path.display()
                );
            }
            quarantine_and_dispose(
                grove_root,
                &transaction.quarantine_path,
                &transaction.manifest.finish_handle,
                &transaction.manifest.attempt_identity,
                &proof,
            )?;
            Ok(FinishRecovery::Committed)
        }
        repo::FinishRecoveryOutcome::NotCommitted(proof) => {
            match layout {
                ReadyWitnessLayout::NotEvacuated => {
                    rollback_not_evacuated(&transaction, &directories, proof)?;
                }
                ReadyWitnessLayout::Evacuated => rollback(&transaction, &directories, proof)?,
            }
            Ok(FinishRecovery::RolledBack)
        }
        repo::FinishRecoveryOutcome::RecoveryPending(error) => Err(error.context(format!(
            "Recovery pending at {}; preserve divergent work, restore the recorded start or the exact teardown result, then retry",
            witness_path.display()
        ))),
    }
}

fn pending_preparing(grove_root: &Path) -> Result<Option<PreparingFinish>> {
    let metadata = match fs::symlink_metadata(grove_root) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "inspecting task root for a preparing finish transaction: {}",
                    grove_root.display()
                )
            })
        }
    };
    if !metadata.file_type().is_dir() {
        bail!(
            "Recovery pending: task root is not a real directory while checking finish preparation: {}",
            grove_root.display()
        );
    }
    let grove_root_directory = open_task_root(grove_root).with_context(|| {
        format!(
            "Recovery pending: opening task root without following symlinks at {}",
            grove_root.display()
        )
    })?;
    revalidate_task_root(grove_root, &grove_root_directory)?;
    let root_entries = directory_names(&grove_root_directory).with_context(|| {
        format!(
            "scanning task root for a preparing finish transaction: {}",
            grove_root.display()
        )
    })?;
    let mut witnesses = root_entries
        .iter()
        .filter(|name| name.as_bytes().starts_with(PREPARING_WITNESS_PREFIX))
        .cloned()
        .collect::<Vec<_>>();
    if witnesses.is_empty() {
        return Ok(None);
    }
    let ready_witnesses = root_entries
        .iter()
        .filter(|name| name.as_bytes().starts_with(WITNESS_PREFIX))
        .collect::<Vec<_>>();
    if !ready_witnesses.is_empty() {
        bail!(
            "Recovery pending: coexisting preparing and ready finish witnesses prevent exact ownership classification; preparing {witnesses:?}, ready {ready_witnesses:?}"
        );
    }
    if witnesses.len() != 1 {
        bail!(
            "Recovery pending: multiple preparing finish witnesses exist: {}",
            witnesses
                .iter()
                .map(|name| grove_root.join(name).display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let witness_name = witnesses.pop().expect("one preparing finish witness");
    let (finish_handle, attempt_identity) = parse_preparing_witness_name(&witness_name)?;
    let witness_path = grove_root.join(&witness_name);
    let witness_directory =
        open_directory_at(&grove_root_directory, &witness_name).with_context(|| {
            format!(
                "Recovery pending: opening preparing finish witness without following symlinks at {}",
                witness_path.display()
            )
        })?;
    revalidate_open_directory_at(
        &grove_root_directory,
        &witness_name,
        &witness_directory,
        "preparing finish witness",
    )?;
    let witness_entries = directory_names(&witness_directory)
        .context("listing the preparing finish witness through its retained descriptor")?;
    let original_name = OsString::from(ORIGINAL_TREE_DIRECTORY);
    let manifest_name = OsString::from(MANIFEST_FILE);
    let ready_name = OsString::from(READY_FILE);
    let valid_entries = witness_entries.is_empty()
        || witness_entries == [original_name.clone()]
        || witness_entries == [manifest_name.clone(), original_name.clone()]
        || witness_entries
            == [
                manifest_name.clone(),
                ready_name.clone(),
                original_name.clone(),
            ];
    if !valid_entries {
        bail!(
            "Recovery pending: preparing finish witness contains unclassified entries at {}",
            witness_path.display()
        );
    }
    let original_tree_directory = witness_entries
        .contains(&original_name)
        .then(|| {
            open_directory_at(&witness_directory, OsStr::new(ORIGINAL_TREE_DIRECTORY))
                .context("opening preparing finish recovery tree without following symlinks")
        })
        .transpose()?;
    if let Some(original_tree_directory) = &original_tree_directory {
        revalidate_open_directory_at(
            &witness_directory,
            OsStr::new(ORIGINAL_TREE_DIRECTORY),
            original_tree_directory,
            "preparing finish recovery tree",
        )?;
        if !directory_names(original_tree_directory)
            .context("checking the preparing finish recovery tree")?
            .is_empty()
        {
            bail!(
                "Recovery pending: preparing finish recovery tree is not empty at {}",
                witness_path.join(ORIGINAL_TREE_DIRECTORY).display()
            );
        }
    }
    let manifest_file = witness_entries
        .contains(&manifest_name)
        .then(|| {
            read_regular_file_at(
                &witness_directory,
                OsStr::new(MANIFEST_FILE),
                "preparing finish manifest",
            )
            .map(|(_, file)| file)
        })
        .transpose()?;
    let ready_file = witness_entries
        .contains(&ready_name)
        .then(|| {
            read_regular_file_at(
                &witness_directory,
                OsStr::new(READY_FILE),
                "preparing finish ready marker",
            )
            .map(|(_, file)| file)
        })
        .transpose()?;
    revalidate_task_root(grove_root, &grove_root_directory)?;
    revalidate_open_directory_at(
        &grove_root_directory,
        &witness_name,
        &witness_directory,
        "preparing finish witness",
    )?;
    Ok(Some(PreparingFinish {
        finish_handle,
        attempt_identity,
        grove_root_directory,
        witness_directory,
        original_tree_directory,
        manifest_file,
        ready_file,
        witness_name,
        witness_path,
    }))
}

fn parse_preparing_witness_name(name: &OsStr) -> Result<(String, String)> {
    let bytes = name.as_bytes();
    let body = bytes
        .strip_prefix(PREPARING_WITNESS_PREFIX)
        .context("preparing finish witness has an invalid reserved prefix")?;
    let split = body
        .len()
        .checked_sub(33)
        .filter(|split| body[*split] == b'-')
        .context("preparing finish witness does not name a handle and 128-bit attempt identity")?;
    let finish_handle = std::str::from_utf8(&body[..split])
        .context("preparing finish witness handle is not UTF-8")?
        .to_owned();
    let attempt_identity = std::str::from_utf8(&body[split + 1..])
        .context("preparing finish witness attempt identity is not UTF-8")?
        .to_owned();
    finish_cleanup::CleanupOwner::new(finish_handle.clone(), attempt_identity.clone())
        .context("preparing finish witness has an invalid cleanup owner")?;
    Ok((finish_handle, attempt_identity))
}

fn preparing_witness_name(finish_handle: &str, attempt_identity: &str) -> OsString {
    OsString::from(format!(
        "PREPARING-FINISH-{finish_handle}-{attempt_identity}"
    ))
}

fn begin_preparing_witness(
    grove_root: &Path,
    grove_root_directory: &File,
    finish_handle: &str,
    attempt_identity: &str,
) -> Result<PreparingFinish> {
    finish_cleanup::CleanupOwner::new(finish_handle.to_owned(), attempt_identity.to_owned())
        .context("finish preparation has an invalid cleanup owner")?;
    let witness_name = preparing_witness_name(finish_handle, attempt_identity);
    create_directory_at(grove_root_directory, &witness_name).with_context(|| {
        format!(
            "creating preparing finish witness {}",
            grove_root.join(&witness_name).display()
        )
    })?;
    pending_preparing(grove_root)?.context("preparing finish witness was not published")
}

fn remove_preparing_witness(preparing: PreparingFinish) -> Result<()> {
    let mut expected_entries = Vec::new();
    if preparing.original_tree_directory.is_some() {
        expected_entries.push(OsString::from(ORIGINAL_TREE_DIRECTORY));
    }
    if preparing.manifest_file.is_some() {
        expected_entries.push(OsString::from(MANIFEST_FILE));
    }
    if preparing.ready_file.is_some() {
        expected_entries.push(OsString::from(READY_FILE));
    }
    expected_entries.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    let observed_entries = directory_names(&preparing.witness_directory)
        .context("checking the preparing finish witness before removal")?;
    if observed_entries != expected_entries {
        bail!(
            "Recovery pending: preparing finish witness changed at {}; expected {expected_entries:?}, observed {observed_entries:?}",
            preparing.witness_path.display()
        );
    }
    if let Some(original_tree_directory) = &preparing.original_tree_directory {
        revalidate_open_directory_at(
            &preparing.witness_directory,
            OsStr::new(ORIGINAL_TREE_DIRECTORY),
            original_tree_directory,
            "preparing finish recovery tree",
        )?;
        if !directory_names(original_tree_directory)
            .context("checking the preparing finish recovery tree before removal")?
            .is_empty()
        {
            bail!(
                "Recovery pending: preparing finish recovery tree is not empty at {}",
                preparing
                    .witness_path
                    .join(ORIGINAL_TREE_DIRECTORY)
                    .display()
            );
        }
    }
    if let Some(manifest_file) = &preparing.manifest_file {
        revalidate_open_file_at(
            &preparing.witness_directory,
            OsStr::new(MANIFEST_FILE),
            manifest_file,
            "preparing finish manifest",
        )?;
    }
    if let Some(ready_file) = &preparing.ready_file {
        revalidate_open_file_at(
            &preparing.witness_directory,
            OsStr::new(READY_FILE),
            ready_file,
            "preparing finish ready marker",
        )?;
    }
    revalidate_open_directory_at(
        &preparing.grove_root_directory,
        &preparing.witness_name,
        &preparing.witness_directory,
        "preparing finish witness",
    )?;
    if preparing.ready_file.is_some() {
        unlink_at(&preparing.witness_directory, OsStr::new(READY_FILE), 0)?;
    }
    if preparing.manifest_file.is_some() {
        unlink_at(&preparing.witness_directory, OsStr::new(MANIFEST_FILE), 0)?;
    }
    if preparing.original_tree_directory.is_some() {
        unlink_at(
            &preparing.witness_directory,
            OsStr::new(ORIGINAL_TREE_DIRECTORY),
            libc::AT_REMOVEDIR,
        )?;
    }
    if !directory_names(&preparing.witness_directory)
        .context("checking the emptied preparing finish witness")?
        .is_empty()
    {
        bail!(
            "Recovery pending: preparing finish witness is not empty after cleanup at {}",
            preparing.witness_path.display()
        );
    }
    unlink_at(
        &preparing.grove_root_directory,
        &preparing.witness_name,
        libc::AT_REMOVEDIR,
    )
    .with_context(|| {
        format!(
            "removing preparing finish witness {}",
            preparing.witness_path.display()
        )
    })
}

fn remove_preparing_after_error(preparing: PreparingFinish, error: anyhow::Error) -> anyhow::Error {
    match remove_preparing_witness(preparing) {
        Ok(()) => error,
        Err(cleanup_error) => error.context(format!(
            "Recovery pending: removing the preparing finish witness failed: {cleanup_error:#}; preserve it for exact ownership recovery"
        )),
    }
}

fn pending_manifest(grove_root: &Path) -> Result<Option<PendingFinish>> {
    pending_manifest_with_checkpoint(grove_root, |_| Ok(()))
}

fn pending_manifest_with_checkpoint(
    grove_root: &Path,
    mut checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<Option<PendingFinish>> {
    let metadata = match fs::symlink_metadata(grove_root) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "inspecting task root for a pending finish transaction: {}",
                    grove_root.display()
                )
            })
        }
    };
    if !metadata.file_type().is_dir() {
        bail!(
            "Recovery pending: task root is not a real directory while checking cleanup ownership: {}",
            grove_root.display()
        );
    }
    let grove_root_directory = open_task_root(grove_root).with_context(|| {
        format!(
            "Recovery pending: opening task root without following symlinks at {}",
            grove_root.display()
        )
    })?;
    revalidate_task_root(grove_root, &grove_root_directory)?;
    let root_entries = directory_names(&grove_root_directory).with_context(|| {
        format!(
            "scanning task root for a pending finish transaction: {}",
            grove_root.display()
        )
    })?;
    let mut witnesses = root_entries
        .iter()
        .filter(|name| name.as_bytes().starts_with(WITNESS_PREFIX))
        .cloned()
        .collect::<Vec<_>>();
    if witnesses.is_empty() {
        return Ok(None);
    }
    if witnesses.len() != 1 {
        bail!(
            "Recovery pending: multiple finish transaction witnesses exist: {}",
            witnesses
                .iter()
                .map(|name| grove_root.join(name).display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let witness_name = witnesses.pop().expect("one finish witness");
    let witness_path = grove_root.join(&witness_name);
    let witness_directory =
        open_directory_at(&grove_root_directory, &witness_name).with_context(|| {
            format!(
                "Recovery pending: opening finish witness without following symlinks at {}",
                witness_path.display()
            )
        })?;
    revalidate_open_directory_at(
        &grove_root_directory,
        &witness_name,
        &witness_directory,
        "finish witness",
    )?;
    let witness_entries = directory_names(&witness_directory)
        .context("listing the finish witness through its retained descriptor")?;
    let mut expected_witness_entries = vec![
        OsString::from(MANIFEST_FILE),
        OsString::from(ORIGINAL_TREE_DIRECTORY),
        OsString::from(READY_FILE),
    ];
    expected_witness_entries.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    if witness_entries != expected_witness_entries {
        bail!(
            "Recovery pending: finish witness contains foreign or missing entries at {}; expected {expected_witness_entries:?}, observed {witness_entries:?}",
            witness_path.display()
        );
    }
    let (ready, ready_file) = read_regular_file_at(
        &witness_directory,
        OsStr::new(READY_FILE),
        "finish cleanup ownership ready marker",
    )
    .with_context(|| {
        format!(
            "Recovery pending: validating finish cleanup ownership ready marker at {}",
            witness_path.join(READY_FILE).display()
        )
    })?;
    if ready != b"ready\n" {
        bail!(
            "Recovery pending: finish witness has an invalid ready marker at {}",
            witness_path.display()
        );
    }
    let (manifest_body, manifest_file) = read_regular_file_at(
        &witness_directory,
        OsStr::new(MANIFEST_FILE),
        "finish cleanup ownership manifest",
    )
    .with_context(|| {
        format!(
            "Recovery pending: validating finish cleanup ownership manifest at {}",
            witness_path.join(MANIFEST_FILE).display()
        )
    })?;
    let manifest: FinishManifest = serde_json::from_slice(&manifest_body).with_context(|| {
        format!(
            "Recovery pending: parsing finish manifest at {}",
            witness_path.display()
        )
    })?;
    if manifest.version != 1 {
        bail!(
            "Recovery pending: unsupported finish manifest version {} at {}",
            manifest.version,
            witness_path.display()
        );
    }
    let expected_witness = OsString::from(format!("FINISHING-{}", manifest.finish_handle));
    if witness_name != expected_witness {
        bail!(
            "Recovery pending: finish manifest handle {} does not match witness {}",
            manifest.finish_handle,
            witness_path.display()
        );
    }
    finish_cleanup::CleanupOwner::new(
        manifest.finish_handle.clone(),
        manifest.attempt_identity.clone(),
    )
    .with_context(|| {
        format!(
            "Recovery pending: invalid finish manifest identity at {}",
            witness_path.display()
        )
    })?;
    validate_manifest_entries(&manifest, &witness_path)?;
    let original_tree = witness_path.join(ORIGINAL_TREE_DIRECTORY);
    let original_tree_directory =
        open_directory_at(&witness_directory, OsStr::new(ORIGINAL_TREE_DIRECTORY)).with_context(
            || {
                format!(
            "Recovery pending: opening finish recovery tree without following symlinks at {}",
            original_tree.display()
        )
            },
        )?;
    revalidate_open_directory_at(
        &witness_directory,
        OsStr::new(ORIGINAL_TREE_DIRECTORY),
        &original_tree_directory,
        "finish recovery tree",
    )?;
    checkpoint("after-original-tree-open")?;
    let observed_original_entries =
        digest_root_entries_at(&original_tree_directory).with_context(|| {
            format!(
                "Recovery pending: validating finish recovery tree at {}",
                original_tree.display()
            )
        })?;
    let mut expected_unmoved_root = manifest
        .entries
        .iter()
        .map(|entry| OsString::from_vec(entry.path.clone()))
        .collect::<Vec<_>>();
    expected_unmoved_root.push(witness_name.clone());
    expected_unmoved_root.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    let layout = if root_entries == expected_unmoved_root {
        if !observed_original_entries.is_empty() {
            bail!(
                "Recovery pending: ready finish transaction has both live task-root entries and evacuated recovery-tree entries at {}; preserve the partial evacuation for exact recovery",
                witness_path.display()
            );
        }
        validate_unmoved_root_entries(
            &grove_root_directory,
            &manifest,
            &witness_name,
            &witness_path,
        )?;
        ReadyWitnessLayout::NotEvacuated
    } else if root_entries == [witness_name.clone()] {
        if observed_original_entries != manifest.entries {
            bail!(
                "Recovery pending: evacuated finish tree does not match its manifest at {}",
                witness_path.display()
            );
        }
        ReadyWitnessLayout::Evacuated
    } else {
        bail!(
            "Recovery pending: finish task root contains foreign task-root entries beside the ready witness; observed {root_entries:?}; no repository classification was attempted"
        );
    };
    revalidate_task_root(grove_root, &grove_root_directory)?;
    revalidate_open_directory_at(
        &grove_root_directory,
        &witness_name,
        &witness_directory,
        "finish witness",
    )?;
    revalidate_open_directory_at(
        &witness_directory,
        OsStr::new(ORIGINAL_TREE_DIRECTORY),
        &original_tree_directory,
        "finish recovery tree",
    )?;
    revalidate_open_file_at(
        &witness_directory,
        OsStr::new(MANIFEST_FILE),
        &manifest_file,
        "finish manifest",
    )?;
    revalidate_open_file_at(
        &witness_directory,
        OsStr::new(READY_FILE),
        &ready_file,
        "finish ready marker",
    )?;
    Ok(Some(PendingFinish {
        witness_path,
        manifest,
        layout,
        directories: TransactionDirectories {
            grove_root: grove_root_directory,
            witness: witness_directory,
            original_tree: original_tree_directory,
            manifest: manifest_file,
            ready: ready_file,
            witness_name,
        },
    }))
}

fn validate_unmoved_root_entries(
    grove_root_directory: &File,
    manifest: &FinishManifest,
    witness_name: &OsStr,
    witness_path: &Path,
) -> Result<()> {
    let mut expected_root_entries = manifest
        .entries
        .iter()
        .map(|entry| OsString::from_vec(entry.path.clone()))
        .collect::<Vec<_>>();
    expected_root_entries.push(witness_name.to_os_string());
    expected_root_entries.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    let observed_root_entries = directory_names(grove_root_directory)
        .context("validating the not-yet-evacuated finish task root")?;
    if observed_root_entries != expected_root_entries {
        bail!(
            "Recovery pending: not-yet-evacuated task root changed beside the ready witness at {}; expected {expected_root_entries:?}, observed {observed_root_entries:?}",
            witness_path.display()
        );
    }
    for expected in &manifest.entries {
        let name = OsString::from_vec(expected.path.clone());
        let observed =
            digest_entry_at(grove_root_directory, &name, Path::new("")).with_context(|| {
                format!(
                    "Recovery pending: validating not-yet-evacuated task-root entry {:?} beside {}",
                    name,
                    witness_path.display()
                )
            })?;
        if observed != *expected {
            bail!(
                "Recovery pending: not-yet-evacuated task-root entry {:?} does not match the ready finish manifest at {}",
                name,
                witness_path.display()
            );
        }
    }
    Ok(())
}

fn validate_manifest_entries(manifest: &FinishManifest, witness_path: &Path) -> Result<()> {
    let mut previous_path: Option<&[u8]> = None;
    for entry in &manifest.entries {
        let path = entry.path.as_slice();
        if path.is_empty()
            || path == b"."
            || path == b".."
            || path.contains(&b'/')
            || path.contains(&b'\0')
        {
            bail!(
                "Recovery pending: finish manifest has an invalid root entry path {:?} at {}",
                OsStr::from_bytes(path),
                witness_path.display()
            );
        }
        if previous_path.is_some_and(|previous| previous >= path) {
            bail!(
                "Recovery pending: finish manifest has a duplicate or unsorted root entry path {:?} at {}",
                OsStr::from_bytes(path),
                witness_path.display()
            );
        }
        previous_path = Some(path);
    }
    Ok(())
}

fn read_regular_file_at(parent: &File, name: &OsStr, description: &str) -> Result<(Vec<u8>, File)> {
    let mut file = open_file_at(parent, name)
        .with_context(|| format!("opening {description} {name:?} without following symlinks"))?;
    if !file
        .metadata()
        .with_context(|| format!("reading identity for {description} {name:?}"))?
        .file_type()
        .is_file()
    {
        bail!("finish transaction {description} is not a regular file: {name:?}");
    }
    let mut body = Vec::new();
    file.read_to_end(&mut body)
        .with_context(|| format!("reading {description} {name:?}"))?;
    Ok((body, file))
}

fn preflight_root(worktree: &Path, grove_root: &Path) -> Result<FinishPreflight> {
    let grove_directory = open_task_root(grove_root)?;
    let descriptor_metadata = revalidate_task_root(grove_root, &grove_directory)?;

    let control = repo::workspace_control(worktree)?;
    let control_parent = control.control_dir().parent().with_context(|| {
        format!(
            "workspace-control quarantine has no parent: {}",
            control.control_dir().display()
        )
    })?;
    let control_metadata = fs::metadata(control_parent).with_context(|| {
        format!(
            "reading workspace-control quarantine device: {}",
            control_parent.display()
        )
    })?;
    ensure_same_device(
        descriptor_metadata.dev(),
        control_metadata.dev(),
        grove_root,
        control.control_dir(),
    )?;

    for name in directory_names(&grove_directory)
        .with_context(|| format!("reading finish transaction input {}", grove_root.display()))?
    {
        if name.as_bytes().starts_with(WITNESS_PREFIX)
            || name.as_bytes().starts_with(PREPARING_WITNESS_PREFIX)
        {
            bail!(
                "reserved finish transaction path already exists: {}",
                grove_root.join(name).display()
            );
        }
    }
    let entry_digests = digest_root_entries_at(&grove_directory)?;
    revalidate_task_root(grove_root, &grove_directory)?;
    Ok(FinishPreflight {
        grove_root_directory: grove_directory,
        entry_digests,
        quarantine_directory: control.control_dir().to_path_buf(),
    })
}

fn open_task_root(grove_root: &Path) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(grove_root)
        .with_context(|| format!("opening task root directory: {}", grove_root.display()))
}

fn revalidate_task_root(grove_root: &Path, grove_directory: &File) -> Result<fs::Metadata> {
    let descriptor_metadata = grove_directory.metadata().with_context(|| {
        format!(
            "reading opened task root identity: {}",
            grove_root.display()
        )
    })?;
    let path_metadata = match fs::symlink_metadata(grove_root) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            bail!(
                "Recovery pending: task root changed while the finish transaction held it open: recorded directory device/inode {}/{}, observed a missing path at {}; no finish commit was attempted, so restore the original task root at that path and retry",
                descriptor_metadata.dev(),
                descriptor_metadata.ino(),
                grove_root.display()
            )
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!("revalidating task root identity: {}", grove_root.display())
            })
        }
    };
    if path_metadata.file_type().is_dir()
        && descriptor_metadata.dev() == path_metadata.dev()
        && descriptor_metadata.ino() == path_metadata.ino()
    {
        return Ok(descriptor_metadata);
    }
    let observed_kind = if path_metadata.file_type().is_dir() {
        "directory"
    } else if path_metadata.file_type().is_symlink() {
        "symlink"
    } else if path_metadata.file_type().is_file() {
        "file"
    } else {
        "special entry"
    };
    bail!(
        "Recovery pending: task root changed while the finish transaction held it open: recorded directory device/inode {}/{}, observed {observed_kind} device/inode {}/{} at {}; no finish commit was attempted, so restore the original task root at that path and retry",
        descriptor_metadata.dev(),
        descriptor_metadata.ino(),
        path_metadata.dev(),
        path_metadata.ino(),
        grove_root.display()
    )
}

fn finish_attempt_identity() -> Result<String> {
    let from_session = std::env::var_os("GROVE_SIGNAL_FILE")
        .filter(|value| !value.is_empty())
        .and_then(|value| PathBuf::from(value).file_name().map(OsString::from))
        .and_then(|name| name.into_string().ok())
        .and_then(|name| name.strip_prefix("signal-").map(str::to_owned));
    match from_session {
        Some(identity)
            if identity.len() == 32 && identity.bytes().all(|byte| byte.is_ascii_hexdigit()) =>
        {
            Ok(identity)
        }
        Some(_) => {
            bail!("active Grove signal path does not carry a valid 128-bit finish attempt identity")
        }
        None => driver_lease::fresh_nonce(),
    }
}

fn prepare_transaction(
    grove_root: &Path,
    preflight: FinishPreflight,
    finish_handle: &str,
    attempt_identity: String,
    preparing: PreparingFinish,
    prepared_commit: repo::PreparedFinish,
) -> Result<PreparedTransaction> {
    if let Err(error) = revalidate_task_root(grove_root, &preflight.grove_root_directory) {
        return Err(abort_prepared_finish_with_preparing(
            prepared_commit,
            preparing,
            error,
        ));
    }
    let repository_anchor = prepared_commit.start_anchor();
    let deletion_fingerprint = prepared_commit.deletion_fingerprint();
    let witness_path = grove_root.join(format!("FINISHING-{finish_handle}"));
    let original_tree = witness_path.join(ORIGINAL_TREE_DIRECTORY);
    let quarantine_path = preflight
        .quarantine_directory
        .join(format!("FINISHED-{finish_handle}-{attempt_identity}"));
    let manifest = FinishManifest {
        version: 1,
        finish_handle: finish_handle.to_owned(),
        attempt_identity,
        repository_anchor,
        deletion_fingerprint,
        entries: preflight.entry_digests.clone(),
    };
    let transaction = (|| {
        fs::create_dir_all(&preflight.quarantine_directory).with_context(|| {
            format!(
                "creating workspace-control quarantine directory {}",
                preflight.quarantine_directory.display()
            )
        })?;
        if fs::symlink_metadata(&quarantine_path).is_ok() {
            bail!(
                "finish cleanup quarantine already exists: {}",
                quarantine_path.display()
            );
        }
        Ok(FinishTransaction {
            grove_root: grove_root.to_path_buf(),
            witness_path,
            original_tree,
            quarantine_path,
            manifest,
        })
    })();
    match transaction {
        Ok(transaction) => Ok(PreparedTransaction {
            transaction,
            preparing,
            prepared_commit,
        }),
        Err(error) => Err(abort_prepared_finish_with_preparing(
            prepared_commit,
            preparing,
            error,
        )),
    }
}

fn abort_prepared_finish_with_preparing(
    prepared_commit: repo::PreparedFinish,
    preparing: PreparingFinish,
    error: anyhow::Error,
) -> anyhow::Error {
    match prepared_commit.abort() {
        Ok(()) => remove_preparing_after_error(preparing, error),
        Err(abort_error) => error.context(format!(
            "Recovery pending: cancelling repository preparation failed: {abort_error:#}; preserve the preparing finish witness and its named attempt-bound auxiliary evidence, then rerun bare Grove"
        )),
    }
}

fn abort_prepared_finish(
    prepared_commit: repo::PreparedFinish,
    error: anyhow::Error,
) -> anyhow::Error {
    match prepared_commit.abort() {
        Ok(()) => error,
        Err(abort_error) => error.context(format!(
            "Recovery pending: cancelling repository preparation failed: {abort_error:#}; preserve the named attempt-bound auxiliary evidence, restore the original task root, then exit and rerun Grove so the lease-owning driver can validate and reap orphan auxiliaries before retrying"
        )),
    }
}

#[derive(Clone, Copy)]
struct EntryIdentity {
    device: u64,
    inode: u64,
}

impl EntryIdentity {
    fn from_file(directory: &File, description: &str) -> Result<Self> {
        let metadata = directory
            .metadata()
            .with_context(|| format!("reading {description} identity"))?;
        Ok(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }

    fn from_stat(metadata: &libc::stat) -> Self {
        Self {
            device: metadata.st_dev as u64,
            inode: metadata.st_ino,
        }
    }
}

fn validate_opened_entry_identity(
    expected: EntryIdentity,
    file: &File,
    description: &str,
) -> Result<fs::Metadata> {
    let metadata = file
        .metadata()
        .with_context(|| format!("reading {description} identity"))?;
    let observed = EntryIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
    };
    if observed.device != expected.device || observed.inode != expected.inode {
        bail!(
            "Recovery pending: {description} identity changed while opening it; expected device/inode {}/{}, observed {}/{}",
            expected.device,
            expected.inode,
            observed.device,
            observed.inode
        );
    }
    Ok(metadata)
}

fn revalidate_entry_at(
    parent: &File,
    name: &OsStr,
    expected_identity: EntryIdentity,
    expected_mode: libc::mode_t,
    description: &str,
) -> Result<()> {
    let observed = metadata_at(parent, name).with_context(|| {
        format!("revalidating {description} {name:?} without following symlinks")
    })?;
    let observed_identity = EntryIdentity::from_stat(&observed);
    let relevant_mode = libc::S_IFMT | 0o7777;
    if observed_identity.device != expected_identity.device
        || observed_identity.inode != expected_identity.inode
        || observed.st_mode & relevant_mode != expected_mode & relevant_mode
    {
        bail!(
            "Recovery pending: {description} changed at {name:?}; expected device/inode {}/{}, observed {}/{}",
            expected_identity.device,
            expected_identity.inode,
            observed_identity.device,
            observed_identity.inode
        );
    }
    Ok(())
}

fn revalidate_open_directory_at(
    parent: &File,
    name: &OsStr,
    directory: &File,
    description: &str,
) -> Result<()> {
    let expected = EntryIdentity::from_file(directory, description)?;
    let observed = open_directory_at(parent, name)
        .with_context(|| format!("reopening {description} {name:?} without following symlinks"))?;
    let observed = EntryIdentity::from_file(&observed, description)?;
    if observed.device != expected.device || observed.inode != expected.inode {
        bail!(
            "Recovery pending: {description} identity changed at {name:?}; expected directory device/inode {}/{}, observed {}/{}",
            expected.device,
            expected.inode,
            observed.device,
            observed.inode
        );
    }
    Ok(())
}

fn revalidate_open_file_at(
    parent: &File,
    name: &OsStr,
    file: &File,
    description: &str,
) -> Result<()> {
    let expected = EntryIdentity::from_file(file, description)?;
    let observed = open_file_at(parent, name)
        .with_context(|| format!("reopening {description} {name:?} without following symlinks"))?;
    if !observed
        .metadata()
        .with_context(|| format!("reading {description} identity"))?
        .file_type()
        .is_file()
    {
        bail!("Recovery pending: {description} is not a regular file at {name:?}");
    }
    let observed = EntryIdentity::from_file(&observed, description)?;
    if observed.device != expected.device || observed.inode != expected.inode {
        bail!(
            "Recovery pending: {description} identity changed at {name:?}; expected file device/inode {}/{}, observed {}/{}",
            expected.device,
            expected.inode,
            observed.device,
            observed.inode
        );
    }
    Ok(())
}

fn revalidate_transaction_directories(
    transaction: &FinishTransaction,
    directories: &TransactionDirectories,
) -> Result<()> {
    revalidate_task_root(&transaction.grove_root, &directories.grove_root)?;
    revalidate_open_directory_at(
        &directories.grove_root,
        &directories.witness_name,
        &directories.witness,
        "finish witness",
    )?;
    revalidate_open_directory_at(
        &directories.witness,
        OsStr::new(ORIGINAL_TREE_DIRECTORY),
        &directories.original_tree,
        "finish recovery tree",
    )?;
    revalidate_open_file_at(
        &directories.witness,
        OsStr::new(MANIFEST_FILE),
        &directories.manifest,
        "finish manifest",
    )?;
    revalidate_open_file_at(
        &directories.witness,
        OsStr::new(READY_FILE),
        &directories.ready,
        "finish ready marker",
    )
}

fn write_new_file_at(parent: &File, name: &OsStr, body: &[u8], description: &str) -> Result<File> {
    let mut file = create_new_file_at(parent, name)
        .with_context(|| format!("creating {description} {name:?}"))?;
    file.write_all(body)
        .with_context(|| format!("writing {description} {name:?}"))?;
    Ok(file)
}

fn materialize_witness_with_checkpoint(
    transaction: &FinishTransaction,
    preparing: PreparingFinish,
    mut checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<TransactionDirectories> {
    let manifest_body = serde_json::to_vec_pretty(&transaction.manifest)
        .context("serializing finish transaction manifest")?;
    let final_witness_name = transaction
        .witness_path
        .file_name()
        .context("finish transaction witness has no file name")?
        .to_os_string();
    let PreparingFinish {
        grove_root_directory,
        witness_directory,
        witness_name: preparing_witness_name,
        witness_path: preparing_witness_path,
        ..
    } = preparing;
    let mut original_tree_directory = None;
    let mut manifest_file = None;
    let mut ready_file = None;
    let mut published = false;
    let result = (|| {
        checkpoint("after-witness-directory")?;
        revalidate_open_directory_at(
            &grove_root_directory,
            &preparing_witness_name,
            &witness_directory,
            "preparing finish witness",
        )?;
        create_directory_at(&witness_directory, OsStr::new(ORIGINAL_TREE_DIRECTORY)).with_context(
            || {
                format!(
                    "creating finish transaction recovery tree {}",
                    transaction.original_tree.display()
                )
            },
        )?;
        let directory = open_directory_at(&witness_directory, OsStr::new(ORIGINAL_TREE_DIRECTORY))
            .context("opening finish recovery tree without following symlinks")?;
        original_tree_directory = Some(directory);
        checkpoint("after-recovery-tree")?;
        revalidate_open_directory_at(
            &witness_directory,
            OsStr::new(ORIGINAL_TREE_DIRECTORY),
            original_tree_directory
                .as_ref()
                .expect("recovery tree was opened"),
            "finish recovery tree",
        )?;
        manifest_file = Some(write_new_file_at(
            &witness_directory,
            OsStr::new(MANIFEST_FILE),
            &manifest_body,
            "finish transaction manifest",
        )?);
        checkpoint("after-manifest")?;
        ready_file = Some(write_new_file_at(
            &witness_directory,
            OsStr::new(READY_FILE),
            b"ready\n",
            "finish transaction ready marker",
        )?);
        checkpoint("after-ready")?;
        revalidate_open_directory_at(
            &grove_root_directory,
            &preparing_witness_name,
            &witness_directory,
            "preparing finish witness",
        )?;
        rename_at_noreplace(
            &grove_root_directory,
            &preparing_witness_name,
            &grove_root_directory,
            &final_witness_name,
        )
        .with_context(|| {
            format!(
                "publishing ready finish witness {}",
                transaction.witness_path.display()
            )
        })?;
        published = true;
        checkpoint("after-ready-witness")
    })();
    match result {
        Ok(()) => Ok(TransactionDirectories {
            grove_root: grove_root_directory,
            witness: witness_directory,
            original_tree: original_tree_directory.expect("recovery tree was opened"),
            manifest: manifest_file.expect("manifest was created"),
            ready: ready_file.expect("ready marker was created"),
            witness_name: final_witness_name,
        }),
        Err(error) => match remove_incomplete_witness(
            transaction,
            &grove_root_directory,
            &witness_directory,
            if published {
                &final_witness_name
            } else {
                &preparing_witness_name
            },
            original_tree_directory.as_ref(),
            manifest_file.as_ref(),
            ready_file.as_ref(),
        ) {
            Ok(()) => Err(error),
            Err(cleanup_error) => {
                let cleanup_path = if published {
                    &transaction.witness_path
                } else {
                    &preparing_witness_path
                };
                Err(error.context(format!(
                    "Recovery pending: removing the incomplete finish witness {} failed: {cleanup_error:#}; preserve it for exact ownership recovery",
                    cleanup_path.display()
                )))
            }
        },
    }
}

fn remove_incomplete_witness(
    transaction: &FinishTransaction,
    grove_root_directory: &File,
    witness_directory: &File,
    witness_name: &OsStr,
    original_tree_directory: Option<&File>,
    manifest_file: Option<&File>,
    ready_file: Option<&File>,
) -> Result<()> {
    revalidate_task_root(&transaction.grove_root, grove_root_directory)?;
    revalidate_open_directory_at(
        grove_root_directory,
        witness_name,
        witness_directory,
        "finish witness",
    )?;
    let mut expected_entries = Vec::new();
    if original_tree_directory.is_some() {
        expected_entries.push(OsString::from(ORIGINAL_TREE_DIRECTORY));
    }
    if manifest_file.is_some() {
        expected_entries.push(OsString::from(MANIFEST_FILE));
    }
    if ready_file.is_some() {
        expected_entries.push(OsString::from(READY_FILE));
    }
    expected_entries.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    let observed_entries = directory_names(witness_directory).with_context(|| {
        format!(
            "reading incomplete finish witness {}",
            transaction.witness_path.display()
        )
    })?;
    if observed_entries != expected_entries {
        bail!(
            "incomplete finish witness contains a foreign or missing entry at {}; expected {expected_entries:?}, observed {observed_entries:?}",
            transaction.witness_path.display()
        );
    }
    if let Some(original_tree_directory) = original_tree_directory {
        revalidate_open_directory_at(
            witness_directory,
            OsStr::new(ORIGINAL_TREE_DIRECTORY),
            original_tree_directory,
            "finish recovery tree",
        )?;
        if !directory_names(original_tree_directory)?.is_empty() {
            bail!(
                "incomplete finish recovery tree is not empty at {}",
                transaction.original_tree.display()
            );
        }
    }
    if let Some(manifest_file) = manifest_file {
        revalidate_open_file_at(
            witness_directory,
            OsStr::new(MANIFEST_FILE),
            manifest_file,
            "finish transaction manifest",
        )?;
    }
    if let Some(ready_file) = ready_file {
        revalidate_open_file_at(
            witness_directory,
            OsStr::new(READY_FILE),
            ready_file,
            "finish transaction ready marker",
        )?;
    }
    if ready_file.is_some() {
        unlink_at(witness_directory, OsStr::new(READY_FILE), 0)?;
    }
    if manifest_file.is_some() {
        unlink_at(witness_directory, OsStr::new(MANIFEST_FILE), 0)?;
    }
    if original_tree_directory.is_some() {
        unlink_at(
            witness_directory,
            OsStr::new(ORIGINAL_TREE_DIRECTORY),
            libc::AT_REMOVEDIR,
        )?;
    }
    revalidate_task_root(&transaction.grove_root, grove_root_directory)?;
    revalidate_open_directory_at(
        grove_root_directory,
        witness_name,
        witness_directory,
        "finish witness",
    )?;
    if !directory_names(witness_directory)?.is_empty() {
        bail!(
            "incomplete finish witness is not empty after verified cleanup at {}",
            transaction.witness_path.display()
        );
    }
    unlink_at(grove_root_directory, witness_name, libc::AT_REMOVEDIR).with_context(|| {
        format!(
            "removing incomplete finish witness {}",
            transaction.witness_path.display()
        )
    })
}

fn entry_path(root: &Path, path: &[u8]) -> PathBuf {
    root.join(OsString::from_vec(path.to_vec()))
}

fn evacuate(prepared: PreparedTransaction) -> Result<EvacuatedTransaction> {
    evacuate_with_checkpoint(prepared, finish_test_checkpoint)
}

fn validate_ready_root_entries(
    transaction: &FinishTransaction,
    directories: &TransactionDirectories,
) -> Result<()> {
    let mut expected = transaction
        .manifest
        .entries
        .iter()
        .map(|entry| OsString::from_vec(entry.path.clone()))
        .collect::<Vec<_>>();
    expected.push(directories.witness_name.clone());
    expected.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    let observed = directory_names(&directories.grove_root)
        .context("listing the finish task root through its retained descriptor")?;
    if observed != expected {
        bail!(
            "Recovery pending: finish task root contains foreign task-root entries or is missing prepared entries; expected {expected:?}, observed {observed:?}; no source entry was moved"
        );
    }
    for prepared in &transaction.manifest.entries {
        let name = OsString::from_vec(prepared.path.clone());
        let observed = digest_entry_at(&directories.grove_root, &name, Path::new(""))
            .with_context(|| {
                format!(
                    "validating prepared finish source entry {}",
                    entry_path(&transaction.grove_root, &prepared.path).display()
                )
            })?;
        if observed != *prepared {
            bail!(
                "Recovery pending: prepared finish source entry does not match its manifest at {}; no source entry was moved",
                entry_path(&transaction.grove_root, &prepared.path).display()
            );
        }
    }
    let observed_after_digest = directory_names(&directories.grove_root)
        .context("rechecking the finish task root after source digest validation")?;
    if observed_after_digest != expected {
        bail!(
            "Recovery pending: finish task root changed during source digest validation; expected {expected:?}, observed {observed_after_digest:?}; no source entry was moved"
        );
    }
    Ok(())
}

fn evacuate_with_checkpoint(
    prepared: PreparedTransaction,
    checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<EvacuatedTransaction> {
    let PreparedTransaction {
        transaction,
        preparing,
        prepared_commit,
    } = prepared;
    if let Err(error) =
        revalidate_task_root(&transaction.grove_root, &preparing.grove_root_directory)
    {
        return Err(abort_prepared_finish_with_preparing(
            prepared_commit,
            preparing,
            error,
        ));
    }
    let directories = match materialize_witness_with_checkpoint(&transaction, preparing, checkpoint)
    {
        Ok(directories) => directories,
        Err(error) => return Err(abort_prepared_finish(prepared_commit, error)),
    };
    revalidate_transaction_directories(&transaction, &directories)?;
    validate_ready_root_entries(&transaction, &directories)?;
    for entry in &transaction.manifest.entries {
        let name = OsString::from_vec(entry.path.clone());
        rename_at_noreplace(
            &directories.grove_root,
            &name,
            &directories.original_tree,
            &name,
        )
        .with_context(|| {
            format!(
                "evacuating finish transaction entry {} to {}",
                entry_path(&transaction.grove_root, &entry.path).display(),
                entry_path(&transaction.original_tree, &entry.path).display()
            )
        })?;
    }
    Ok(EvacuatedTransaction {
        transaction,
        directories,
        prepared_commit,
    })
}

fn rollback(
    transaction: &FinishTransaction,
    directories: &TransactionDirectories,
    proof: repo::FinishStartProof,
) -> Result<()> {
    proof.revalidate_before_rollback()?;
    revalidate_task_root(&transaction.grove_root, &directories.grove_root)?;
    revalidate_open_directory_at(
        &directories.grove_root,
        &directories.witness_name,
        &directories.witness,
        "finish witness",
    )?;
    revalidate_open_directory_at(
        &directories.witness,
        OsStr::new(ORIGINAL_TREE_DIRECTORY),
        &directories.original_tree,
        "finish recovery tree",
    )?;
    for expected in &transaction.manifest.entries {
        let name = OsString::from_vec(expected.path.clone());
        rename_at_noreplace(
            &directories.original_tree,
            &name,
            &directories.grove_root,
            &name,
        )
        .with_context(|| {
            format!(
                "restoring finish transaction entry {} to {}",
                entry_path(&transaction.original_tree, &expected.path).display(),
                entry_path(&transaction.grove_root, &expected.path).display()
            )
        })?;
        let destination = entry_path(&transaction.grove_root, &expected.path);
        let name = OsString::from_vec(expected.path.clone());
        let actual = digest_entry_at(&directories.grove_root, &name, Path::new(""))?;
        if actual != *expected {
            bail!(
                "restored finish transaction entry does not match its manifest: {}",
                destination.display()
            );
        }
    }
    revalidate_open_file_at(
        &directories.witness,
        OsStr::new(MANIFEST_FILE),
        &directories.manifest,
        "finish transaction manifest",
    )?;
    revalidate_open_file_at(
        &directories.witness,
        OsStr::new(READY_FILE),
        &directories.ready,
        "finish transaction ready marker",
    )?;
    proof.revalidate_after_rollback()?;
    if !directory_names(&directories.original_tree)
        .context("checking the restored finish recovery tree")?
        .is_empty()
    {
        bail!(
            "Recovery pending: finish recovery tree is not empty after rollback at {}",
            transaction.original_tree.display()
        );
    }
    remove_ready_witness(transaction, directories)
}

fn rollback_not_evacuated(
    transaction: &FinishTransaction,
    directories: &TransactionDirectories,
    proof: repo::FinishStartProof,
) -> Result<()> {
    rollback_not_evacuated_with_checkpoint(transaction, directories, proof, |_| Ok(()))
}

fn rollback_not_evacuated_with_checkpoint(
    transaction: &FinishTransaction,
    directories: &TransactionDirectories,
    proof: repo::FinishStartProof,
    mut checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<()> {
    proof.revalidate_before_rollback()?;
    revalidate_task_root(&transaction.grove_root, &directories.grove_root)?;
    revalidate_open_directory_at(
        &directories.grove_root,
        &directories.witness_name,
        &directories.witness,
        "finish witness",
    )?;
    revalidate_open_directory_at(
        &directories.witness,
        OsStr::new(ORIGINAL_TREE_DIRECTORY),
        &directories.original_tree,
        "finish recovery tree",
    )?;
    if !directory_names(&directories.original_tree)
        .context("checking the not-yet-evacuated finish recovery tree")?
        .is_empty()
    {
        bail!(
            "Recovery pending: not-yet-evacuated finish recovery tree is not empty at {}",
            transaction.original_tree.display()
        );
    }
    validate_unmoved_root_entries(
        &directories.grove_root,
        &transaction.manifest,
        &directories.witness_name,
        &transaction.witness_path,
    )?;
    revalidate_open_file_at(
        &directories.witness,
        OsStr::new(MANIFEST_FILE),
        &directories.manifest,
        "finish transaction manifest",
    )?;
    revalidate_open_file_at(
        &directories.witness,
        OsStr::new(READY_FILE),
        &directories.ready,
        "finish transaction ready marker",
    )?;
    checkpoint("before-final-repository-proof")?;
    proof.revalidate_after_rollback()?;
    remove_ready_witness(transaction, directories)
}

fn remove_ready_witness(
    transaction: &FinishTransaction,
    directories: &TransactionDirectories,
) -> Result<()> {
    unlink_at(&directories.witness, OsStr::new(READY_FILE), 0)
        .context("removing the rolled-back finish ready marker")?;
    unlink_at(&directories.witness, OsStr::new(MANIFEST_FILE), 0)
        .context("removing the rolled-back finish manifest")?;
    unlink_at(
        &directories.witness,
        OsStr::new(ORIGINAL_TREE_DIRECTORY),
        libc::AT_REMOVEDIR,
    )
    .context("removing the empty finish recovery tree")?;
    revalidate_task_root(&transaction.grove_root, &directories.grove_root)?;
    revalidate_open_directory_at(
        &directories.grove_root,
        &directories.witness_name,
        &directories.witness,
        "finish witness",
    )?;
    if !directory_names(&directories.witness)
        .context("checking the rolled-back finish witness")?
        .is_empty()
    {
        bail!(
            "Recovery pending: finish witness contains foreign entries after rollback at {}",
            transaction.witness_path.display()
        );
    }
    unlink_at(
        &directories.grove_root,
        &directories.witness_name,
        libc::AT_REMOVEDIR,
    )
    .with_context(|| {
        format!(
            "removing rolled-back finish transaction witness {}",
            transaction.witness_path.display()
        )
    })
}

fn quarantine_and_dispose(
    grove_root: &Path,
    quarantine_path: &Path,
    finish_handle: &str,
    attempt_identity: &str,
    proof: &repo::FinishProof,
) -> Result<()> {
    proof.revalidate()?;
    let cleanup = finish_cleanup::prepare_quarantine(
        grove_root,
        quarantine_path,
        finish_handle,
        attempt_identity,
    )?;
    cleanup.handoff(grove_root)?;
    if let Err(error) = proof.revalidate() {
        cleanup.restore(grove_root).with_context(|| {
            format!(
                "restoring finish quarantine {} after repository proof changed",
                quarantine_path.display()
            )
        })?;
        return Err(error.context("Recovery pending: Git finish proof changed after quarantine"));
    }
    if let Err(error) = cleanup.dispose() {
        eprintln!("warning: {error:#}");
    }
    Ok(())
}

fn ensure_same_device(
    grove_device: u64,
    control_device: u64,
    grove_root: &Path,
    quarantine_directory: &Path,
) -> Result<()> {
    if grove_device != control_device {
        bail!(
            "finish requires an atomic quarantine handoff, but task root {} and workspace-control quarantine {} are on different filesystems",
            grove_root.display(),
            quarantine_directory.display()
        );
    }
    Ok(())
}

#[cfg(test)]
fn digest_root_entries(grove_root: &Path) -> Result<Vec<RootEntryDigest>> {
    let directory = open_task_root(grove_root)?;
    let digests = digest_root_entries_at(&directory)?;
    revalidate_task_root(grove_root, &directory)?;
    Ok(digests)
}

fn digest_root_entries_at(directory: &File) -> Result<Vec<RootEntryDigest>> {
    let names = directory_names(directory).context("listing finish transaction root entries")?;
    let mut digests = Vec::with_capacity(names.len());
    for name in &names {
        digests.push(digest_entry_at(directory, name, Path::new(""))?);
    }
    let observed = directory_names(directory)
        .context("relisting finish transaction root entries after digesting")?;
    if observed != names {
        bail!("Recovery pending: finish transaction root entry set changed while digesting");
    }
    Ok(digests)
}

fn digest_entry_at(parent: &File, name: &OsStr, relative_parent: &Path) -> Result<RootEntryDigest> {
    let relative = relative_parent.join(name);
    let path_bytes = relative.as_os_str().as_bytes().to_vec();
    let stat = metadata_at(parent, name).with_context(|| {
        format!(
            "reading finish transaction entry {:?} without following symlinks",
            relative
        )
    })?;
    let expected_identity = EntryIdentity::from_stat(&stat);
    let kind = stat.st_mode & libc::S_IFMT;
    let (entry_type, type_bytes, mode) = if kind == libc::S_IFDIR {
        (
            EntryType::Directory,
            b"directory".as_slice(),
            u32::from(stat.st_mode & 0o7777),
        )
    } else if kind == libc::S_IFREG {
        (
            EntryType::File,
            b"file".as_slice(),
            u32::from(stat.st_mode & 0o7777),
        )
    } else if kind == libc::S_IFLNK {
        (EntryType::Symlink, b"symlink".as_slice(), 0)
    } else {
        bail!(
            "unsupported task-tree entry in finish transaction: {:?}",
            relative
        );
    };

    let mut hasher = Sha256::new();
    update_field(&mut hasher, &path_bytes);
    update_field(&mut hasher, type_bytes);
    hasher.update(u64::from(mode).to_le_bytes());
    match entry_type {
        EntryType::Directory => {
            let directory = open_directory_at(parent, name).with_context(|| {
                format!("opening finish transaction directory entry {:?}", relative)
            })?;
            let metadata =
                validate_opened_entry_identity(expected_identity, &directory, "directory entry")?;
            if !metadata.file_type().is_dir() {
                bail!(
                    "finish transaction entry is not a directory: {:?}",
                    relative
                );
            }
            let names = directory_names(&directory)
                .with_context(|| format!("listing finish transaction directory {:?}", relative))?;
            hasher.update((names.len() as u64).to_le_bytes());
            for child_name in &names {
                let child = digest_entry_at(&directory, child_name, &relative)?;
                update_field(&mut hasher, &child.digest);
            }
            let observed_names = directory_names(&directory).with_context(|| {
                format!("relisting finish transaction directory {:?}", relative)
            })?;
            if observed_names != names {
                bail!(
                    "Recovery pending: finish transaction directory entry set changed while digesting {:?}",
                    relative
                );
            }
        }
        EntryType::File => {
            let mut file = open_file_at(parent, name).with_context(|| {
                format!(
                    "opening finish transaction file {:?} without following symlinks",
                    relative
                )
            })?;
            let metadata =
                validate_opened_entry_identity(expected_identity, &file, "regular file entry")?;
            if !metadata.file_type().is_file() {
                bail!(
                    "finish transaction entry is not a regular file: {:?}",
                    relative
                );
            }
            let mut body = Vec::new();
            file.read_to_end(&mut body)
                .with_context(|| format!("reading finish transaction file {:?}", relative))?;
            update_field(&mut hasher, &body);
        }
        EntryType::Symlink => {
            let target = read_link_at(parent, name)
                .with_context(|| format!("reading finish transaction symlink {:?}", relative))?;
            update_field(&mut hasher, target.as_os_str().as_bytes());
        }
    }
    revalidate_entry_at(
        parent,
        name,
        expected_identity,
        stat.st_mode,
        "finish transaction entry",
    )?;

    Ok(RootEntryDigest {
        path: path_bytes,
        entry_type,
        mode,
        digest: hasher.finalize().into(),
    })
}

fn update_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

#[cfg(test)]
mod tests {
    use super::{
        begin_preparing_witness, digest_root_entries, ensure_same_device, evacuate,
        evacuate_with_checkpoint, materialize_witness_with_checkpoint, pending_manifest,
        pending_manifest_with_checkpoint, preflight_root, prepare_transaction, recover_pending,
        rollback_not_evacuated_with_checkpoint, FinishRecovery, PreparedTransaction, MANIFEST_FILE,
        READY_FILE,
    };
    use crate::repo;
    use serde_json::Value;
    use std::fs;
    use std::os::unix::ffi::OsStringExt;
    use std::os::unix::fs::{symlink, PermissionsExt};
    use std::os::unix::net::UnixListener;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use tempfile::TempDir;

    type ManifestMutationCase = (&'static str, fn(&mut Value), &'static str);

    fn run_git(repository: &Path, arguments: &[&str]) -> String {
        let output = Command::new("git")
            .current_dir(repository)
            .args(arguments)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn prepared_plain_git_transaction() -> (TempDir, PreparedTransaction) {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path();
        run_git(repository, &["init", "-q", "."]);
        run_git(repository, &["config", "user.name", "Grove Test"]);
        run_git(
            repository,
            &["config", "user.email", "grove-test@example.com"],
        );
        let grove_root = repository.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        fs::write(grove_root.join("FORMAT"), "session-kinds-v1\n").unwrap();
        fs::write(grove_root.join("BRIEF.md"), "# fixture — brief\n").unwrap();
        fs::write(grove_root.join("01-DONE-impl-work-k1.md"), "# work-k1\n").unwrap();
        symlink("FORMAT", grove_root.join("current-format")).unwrap();
        run_git(repository, &["add", "-A"]);
        run_git(repository, &["commit", "-q", "-m", "fixture"]);
        fs::write(grove_root.join("02-finish-finish-k2.md"), "# finish-k2\n").unwrap();

        let preflight = preflight_root(repository, &grove_root).unwrap();
        let preparing = begin_preparing_witness(
            &grove_root,
            &preflight.grove_root_directory,
            "finish-k2",
            "11111111111111111111111111111111",
        )
        .unwrap();
        let prepared =
            repo::prepare_finish(repository, "finish-k2", "11111111111111111111111111111111")
                .unwrap();
        let transaction = prepare_transaction(
            &grove_root,
            preflight,
            "finish-k2",
            "11111111111111111111111111111111".to_owned(),
            preparing,
            prepared,
        )
        .unwrap();
        (fixture, transaction)
    }

    fn evacuated_plain_git_transaction() -> TempDir {
        let (fixture, transaction) = prepared_plain_git_transaction();
        evacuate(transaction).unwrap();
        fixture
    }

    fn preparing_witness(repository: &Path) -> PathBuf {
        repository.join(".grove/PREPARING-FINISH-finish-k2-11111111111111111111111111111111")
    }

    #[test]
    fn restart_removes_an_empty_preparing_witness_before_repository_preparation() {
        let (fixture, prepared) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let PreparedTransaction {
            prepared_commit, ..
        } = prepared;
        prepared_commit.abort().unwrap();
        let preparing = preparing_witness(repository);

        let recovery = recover_pending(repository, &repository.join(".grove")).unwrap();

        assert_eq!(recovery, FinishRecovery::RolledBack);
        assert!(!preparing.exists());
        assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());
    }

    #[test]
    fn restart_aborts_plain_git_preparation_before_removing_its_witness() {
        let (fixture, prepared) = prepared_plain_git_transaction();
        let repository = fixture.path();
        drop(prepared);
        let preparing = preparing_witness(repository);

        let recovery = recover_pending(repository, &repository.join(".grove")).unwrap();

        assert_eq!(recovery, FinishRecovery::RolledBack);
        assert!(!preparing.exists());
        repo::prepare_finish(repository, "finish-k2", "11111111111111111111111111111111")
            .unwrap()
            .abort()
            .unwrap();
    }

    #[test]
    fn restart_refuses_coexisting_preparing_and_ready_witnesses() {
        let (fixture, prepared) = prepared_plain_git_transaction();
        let repository = fixture.path();
        drop(prepared);
        let preparing = preparing_witness(repository);
        let ready = repository.join(".grove/FINISHING-finish-k2");
        fs::create_dir(&ready).unwrap();

        let error = recover_pending(repository, &repository.join(".grove"))
            .expect_err("coexisting finish publication states must fail closed");

        assert!(format!("{error:#}").contains("coexisting"), "{error:#}");
        assert!(preparing.is_dir());
        assert!(ready.is_dir());
    }

    #[test]
    fn restart_preserves_an_unclassified_preparing_witness() {
        let (fixture, prepared) = prepared_plain_git_transaction();
        let repository = fixture.path();
        drop(prepared);
        let preparing = preparing_witness(repository);
        fs::write(preparing.join("foreign"), "preserve\n").unwrap();

        let error = recover_pending(repository, &repository.join(".grove"))
            .expect_err("an unclassified preparing witness must fail closed");

        assert!(
            format!("{error:#}").contains("unclassified entries"),
            "{error:#}"
        );
        assert_eq!(
            fs::read_to_string(preparing.join("foreign")).unwrap(),
            "preserve\n"
        );
    }

    #[test]
    fn restart_does_not_follow_a_substituted_preparing_witness() {
        let (fixture, prepared) = prepared_plain_git_transaction();
        let repository = fixture.path();
        drop(prepared);
        let preparing = preparing_witness(repository);
        let displaced = repository.join("displaced-preparing-witness");
        let external = TempDir::new().unwrap();
        fs::write(external.path().join("preserve"), "external\n").unwrap();
        fs::rename(&preparing, &displaced).unwrap();
        symlink(external.path(), &preparing).unwrap();

        let error = recover_pending(repository, &repository.join(".grove"))
            .expect_err("a substituted preparing witness must fail closed");

        assert!(
            format!("{error:#}").contains("opening preparing finish witness"),
            "{error:#}"
        );
        assert_eq!(
            fs::read_to_string(external.path().join("preserve")).unwrap(),
            "external\n"
        );
        assert!(displaced.is_dir());
    }

    #[test]
    fn committed_repository_with_a_not_evacuated_witness_fails_closed() {
        let (fixture, prepared) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let PreparedTransaction {
            transaction,
            preparing,
            prepared_commit,
        } = prepared;
        materialize_witness_with_checkpoint(&transaction, preparing, |_| Ok(())).unwrap();
        let saved_root = repository.join("saved-not-evacuated-root");
        fs::create_dir(&saved_root).unwrap();
        for entry in &transaction.manifest.entries {
            let name = std::ffi::OsString::from_vec(entry.path.clone());
            fs::rename(transaction.grove_root.join(&name), saved_root.join(&name)).unwrap();
        }
        let outcome = prepared_commit.commit("finish-k2", &transaction.manifest.attempt_identity);
        assert!(matches!(outcome, repo::FinishCommitOutcome::Committed(_)));
        for entry in &transaction.manifest.entries {
            let name = std::ffi::OsString::from_vec(entry.path.clone());
            fs::rename(saved_root.join(&name), transaction.grove_root.join(&name)).unwrap();
        }

        let error = recover_pending(repository, &repository.join(".grove"))
            .expect_err("a committed result cannot own a live, not-evacuated tree");

        assert!(
            format!("{error:#}").contains("not-yet-evacuated"),
            "{error:#}"
        );
        assert!(repository.join(".grove/FORMAT").is_file());
        assert!(repository.join(".grove/FINISHING-finish-k2").is_dir());
    }

    #[test]
    fn not_evacuated_rollback_retains_its_witness_when_the_final_repository_proof_changes() {
        let (fixture, prepared) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let PreparedTransaction {
            transaction,
            preparing,
            prepared_commit,
        } = prepared;
        let directories =
            materialize_witness_with_checkpoint(&transaction, preparing, |_| Ok(())).unwrap();
        drop(prepared_commit);
        let proof = match repo::recover_finish(
            repository,
            &transaction.manifest.repository_anchor,
            &transaction.manifest.finish_handle,
            &transaction.manifest.attempt_identity,
            transaction.manifest.deletion_fingerprint,
        ) {
            repo::FinishRecoveryOutcome::NotCommitted(proof) => proof,
            _ => panic!("the ready, not-evacuated fixture must classify as not committed"),
        };

        let error = rollback_not_evacuated_with_checkpoint(
            &transaction,
            &directories,
            proof,
            |checkpoint| {
                if checkpoint == "before-final-repository-proof" {
                    fs::write(repository.join("divergent"), "preserve\n")?;
                    run_git(repository, &["add", "divergent"]);
                    run_git(repository, &["commit", "-q", "-m", "divergent"]);
                }
                Ok(())
            },
        )
        .expect_err("changed repository topology must block witness removal");

        assert!(format!("{error:#}").contains("recorded start"), "{error:#}");
        assert!(transaction.witness_path.is_dir());
        assert!(transaction.grove_root.join("FORMAT").is_file());
    }

    fn finish_witness(repository: &Path) -> PathBuf {
        repository.join(".grove/FINISHING-finish-k2")
    }

    fn mutate_manifest(repository: &Path, mutation: impl FnOnce(&mut Value)) {
        let manifest_path = finish_witness(repository).join(MANIFEST_FILE);
        let mut manifest: Value =
            serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
        mutation(&mut manifest);
        fs::write(manifest_path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
    }

    fn pending_manifest_error(repository: &Path) -> String {
        let grove_root = repository.join(".grove");
        let error = pending_manifest(&grove_root)
            .err()
            .expect("tampered finish state must be refused before repository classification");
        format!("{error:#}")
    }

    fn assert_original_tree_is_retained(repository: &Path) {
        let original_tree = finish_witness(repository).join("original");
        assert!(original_tree.join("02-finish-finish-k2.md").is_file());
        assert!(!repository.join(".grove/02-finish-finish-k2.md").exists());
    }

    fn fixture(created_in_reverse: bool) -> TempDir {
        let fixture = TempDir::new().unwrap();
        let root = fixture.path();
        let names = if created_in_reverse {
            ["zeta", "alpha"]
        } else {
            ["alpha", "zeta"]
        };
        for name in names {
            fs::write(root.join(name), format!("{name}\n")).unwrap();
        }
        fs::create_dir(root.join("nested")).unwrap();
        fs::write(root.join("nested/body"), "body\n").unwrap();
        symlink("alpha", root.join("link")).unwrap();
        fixture
    }

    #[test]
    fn canonical_digests_cover_content_modes_and_symlink_targets() {
        let left = fixture(false);
        let right = fixture(true);
        let expected = digest_root_entries(left.path()).unwrap();

        assert_eq!(digest_root_entries(right.path()).unwrap(), expected);

        fs::write(right.path().join("alpha"), "changed\n").unwrap();
        assert_ne!(digest_root_entries(right.path()).unwrap(), expected);
        fs::write(right.path().join("alpha"), "alpha\n").unwrap();

        let original_mode = fs::metadata(right.path().join("alpha"))
            .unwrap()
            .permissions()
            .mode();
        fs::set_permissions(
            right.path().join("alpha"),
            fs::Permissions::from_mode(original_mode ^ 0o100),
        )
        .unwrap();
        assert_ne!(digest_root_entries(right.path()).unwrap(), expected);
        fs::set_permissions(
            right.path().join("alpha"),
            fs::Permissions::from_mode(original_mode),
        )
        .unwrap();

        fs::remove_file(right.path().join("link")).unwrap();
        symlink("beta", right.path().join("link")).unwrap();
        assert_ne!(digest_root_entries(right.path()).unwrap(), expected);
    }

    #[test]
    fn recovery_rejects_symlinked_ready_and_manifest_objects_before_classification() {
        for file_name in [READY_FILE, MANIFEST_FILE] {
            let fixture = evacuated_plain_git_transaction();
            let repository = fixture.path();
            let object = finish_witness(repository).join(file_name);
            let displaced = repository.join(format!("displaced-{file_name}"));
            fs::rename(&object, &displaced).unwrap();
            symlink(&displaced, &object).unwrap();

            let message = pending_manifest_error(repository);

            assert!(message.contains(file_name), "{message}");
            assert!(message.contains("without following symlinks"), "{message}");
            assert!(displaced.is_file());
            assert_original_tree_is_retained(repository);
        }
    }

    #[test]
    fn recovery_rejects_malformed_or_extended_manifest_documents() {
        for case in ["malformed", "top-level extension", "entry extension"] {
            let fixture = evacuated_plain_git_transaction();
            let repository = fixture.path();
            let manifest_path = finish_witness(repository).join(MANIFEST_FILE);
            match case {
                "malformed" => fs::write(&manifest_path, b"{").unwrap(),
                "top-level extension" => mutate_manifest(repository, |manifest| {
                    manifest["foreign"] = Value::from("unexpected");
                }),
                "entry extension" => mutate_manifest(repository, |manifest| {
                    manifest["entries"][0]["foreign"] = Value::from("unexpected");
                }),
                _ => unreachable!(),
            }

            let message = pending_manifest_error(repository);

            assert!(
                message.contains("parsing finish manifest"),
                "{case}: {message}"
            );
            assert_original_tree_is_retained(repository);
        }
    }

    #[test]
    fn recovery_rejects_manifest_version_and_identity_mismatches() {
        let cases: [ManifestMutationCase; 3] = [
            (
                "version",
                |manifest| manifest["version"] = Value::from(2),
                "unsupported finish manifest version",
            ),
            (
                "finish handle",
                |manifest| manifest["finish_handle"] = Value::from("finish-k999"),
                "does not match witness",
            ),
            (
                "attempt identity",
                |manifest| manifest["attempt_identity"] = Value::from("not-an-attempt"),
                "invalid finish manifest identity",
            ),
        ];
        for (case, mutation, expected) in cases {
            let fixture = evacuated_plain_git_transaction();
            let repository = fixture.path();
            mutate_manifest(repository, mutation);

            let message = pending_manifest_error(repository);

            assert!(message.contains(expected), "{case}: {message}");
            assert_original_tree_is_retained(repository);
        }
    }

    #[test]
    fn recovery_rejects_noncanonical_manifest_entry_paths() {
        let cases: [ManifestMutationCase; 2] = [
            (
                "duplicate",
                |manifest| {
                    manifest["entries"][1] = manifest["entries"][0].clone();
                },
                "duplicate or unsorted root entry path",
            ),
            (
                "traversal",
                |manifest| manifest["entries"][0]["path"] = serde_json::json!([46, 46]),
                "invalid root entry path",
            ),
        ];
        for (case, mutation, expected) in cases {
            let fixture = evacuated_plain_git_transaction();
            let repository = fixture.path();
            mutate_manifest(repository, mutation);

            let message = pending_manifest_error(repository);

            assert!(message.contains(expected), "{case}: {message}");
            assert_original_tree_is_retained(repository);
        }
    }

    #[test]
    fn recovery_rejects_content_mode_and_link_target_tampering() {
        for case in ["content", "mode", "link target"] {
            let fixture = evacuated_plain_git_transaction();
            let repository = fixture.path();
            let original_tree = finish_witness(repository).join("original");
            match case {
                "content" => fs::write(original_tree.join("FORMAT"), "tampered\n").unwrap(),
                "mode" => {
                    let path = original_tree.join("FORMAT");
                    let mode = fs::metadata(&path).unwrap().permissions().mode();
                    fs::set_permissions(path, fs::Permissions::from_mode(mode ^ 0o100)).unwrap();
                }
                "link target" => {
                    let path = original_tree.join("current-format");
                    fs::remove_file(&path).unwrap();
                    symlink("BRIEF.md", path).unwrap();
                }
                _ => unreachable!(),
            }

            let message = pending_manifest_error(repository);

            assert!(
                message.contains("does not match its manifest"),
                "{case}: {message}"
            );
            assert_original_tree_is_retained(repository);
        }
    }

    #[test]
    fn recovery_rejects_missing_and_foreign_evacuated_entries() {
        for case in ["missing", "foreign"] {
            let fixture = evacuated_plain_git_transaction();
            let repository = fixture.path();
            let original_tree = finish_witness(repository).join("original");
            match case {
                "missing" => fs::remove_file(original_tree.join("FORMAT")).unwrap(),
                "foreign" => fs::write(original_tree.join("foreign"), "preserve\n").unwrap(),
                _ => unreachable!(),
            }

            let message = pending_manifest_error(repository);

            assert!(
                message.contains("does not match its manifest"),
                "{case}: {message}"
            );
            assert_original_tree_is_retained(repository);
        }
    }

    #[test]
    fn recovery_digest_never_walks_a_substituted_original_tree_path() {
        let fixture = evacuated_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");
        let original_tree = finish_witness(repository).join("original");
        let displaced = repository.join("displaced-original");
        let external = TempDir::new().unwrap();
        let socket = external.path().join("foreign-socket");
        let _listener = UnixListener::bind(&socket).unwrap();

        let error = pending_manifest_with_checkpoint(&grove_root, |checkpoint| {
            if checkpoint == "after-original-tree-open" {
                fs::rename(&original_tree, &displaced)?;
                symlink(external.path(), &original_tree)?;
            }
            Ok(())
        })
        .err()
        .expect("a substituted original-tree path must be refused");

        let message = format!("{error:#}");
        assert!(message.contains("finish recovery tree"), "{message}");
        assert!(
            !message.contains("unsupported task-tree entry"),
            "recovery traversed the substituted original-tree path: {message}"
        );
        assert!(socket.exists());
        assert!(displaced.join("02-finish-finish-k2.md").is_file());
    }

    #[test]
    fn preparation_refuses_task_root_replacement_without_touching_the_replacement() {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path();
        run_git(repository, &["init", "-q", "."]);
        run_git(repository, &["config", "user.name", "Grove Test"]);
        run_git(
            repository,
            &["config", "user.email", "grove-test@example.com"],
        );
        let grove_root = repository.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        fs::write(grove_root.join("FORMAT"), "session-kinds-v1\n").unwrap();
        fs::write(grove_root.join("01-DONE-impl-work-k1.md"), "# work-k1\n").unwrap();
        run_git(repository, &["add", "-A"]);
        run_git(repository, &["commit", "-q", "-m", "fixture"]);
        fs::write(grove_root.join("02-finish-finish-k2.md"), "# finish-k2\n").unwrap();

        let preflight = preflight_root(repository, &grove_root).unwrap();
        let preparing = begin_preparing_witness(
            &grove_root,
            &preflight.grove_root_directory,
            "finish-k2",
            "11111111111111111111111111111111",
        )
        .unwrap();
        let prepared =
            repo::prepare_finish(repository, "finish-k2", "11111111111111111111111111111111")
                .unwrap();
        fs::write(repository.join("late-staged"), "preserve\n").unwrap();
        run_git(repository, &["add", "late-staged"]);
        let original_root = repository.join("original-grove");
        fs::rename(&grove_root, &original_root).unwrap();
        fs::create_dir(&grove_root).unwrap();
        fs::write(grove_root.join("foreign"), "preserve\n").unwrap();

        let result = prepare_transaction(
            &grove_root,
            preflight,
            "finish-k2",
            "11111111111111111111111111111111".to_owned(),
            preparing,
            prepared,
        );

        let error = result.err().expect("replaced task root must be refused");
        let message = format!("{error:#}");
        assert!(message.contains("task root changed"), "{message}");
        for required in [
            "Recovery pending",
            "recorded directory device/inode",
            "observed directory device/inode",
            "restore the original task root",
        ] {
            assert!(message.contains(required), "{message}");
        }
        assert_eq!(fs::read(grove_root.join("foreign")).unwrap(), b"preserve\n");
        assert!(!grove_root.join("FINISHING-finish-k2").exists());
        assert!(!original_root.join("FINISHING-finish-k2").exists());
        assert_eq!(
            run_git(
                repository,
                &["diff", "--cached", "--name-only", "--", "late-staged"]
            ),
            "late-staged"
        );

        fs::remove_dir_all(&grove_root).unwrap();
        fs::rename(&original_root, &grove_root).unwrap();
        if let Err(error) =
            repo::prepare_finish(repository, "finish-k2", "11111111111111111111111111111111")
        {
            panic!(
                "restoring the original task root must make the same finish attempt retryable: {error:#}"
            );
        }
    }

    #[test]
    fn evacuation_refuses_task_root_replacement_without_moving_replacement_bytes() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");
        let original_root = repository.join("original-grove");
        fs::rename(&grove_root, &original_root).unwrap();
        fs::create_dir(&grove_root).unwrap();
        fs::write(grove_root.join("FORMAT"), "replacement format\n").unwrap();
        fs::write(grove_root.join("BRIEF.md"), "replacement brief\n").unwrap();
        fs::write(
            grove_root.join("01-DONE-impl-work-k1.md"),
            "replacement work\n",
        )
        .unwrap();
        fs::write(
            grove_root.join("02-finish-finish-k2.md"),
            "replacement finish\n",
        )
        .unwrap();
        symlink(
            original_root.join("FINISHING-finish-k2"),
            grove_root.join("FINISHING-finish-k2"),
        )
        .unwrap();

        let result = evacuate(transaction);

        let error = result.err().expect("replaced task root must be refused");
        let message = format!("{error:#}");
        assert!(message.contains("task root changed"), "{message}");
        for required in [
            "Recovery pending",
            "recorded directory device/inode",
            "observed directory device/inode",
            "restore the original task root",
        ] {
            assert!(message.contains(required), "{message}");
        }
        assert_eq!(
            fs::read(grove_root.join("FORMAT")).unwrap(),
            b"replacement format\n"
        );
        assert_eq!(
            fs::read(grove_root.join("02-finish-finish-k2.md")).unwrap(),
            b"replacement finish\n"
        );
        assert!(
            !original_root.join("FINISHING-finish-k2").exists(),
            "a refused evacuation must not strand its witness in the displaced task root"
        );
    }

    #[test]
    fn witness_replacement_during_materialization_does_not_touch_external_bytes() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");
        let witness = preparing_witness(repository);
        let displaced_witness = repository.join("displaced-finish-witness");
        let external = TempDir::new().unwrap();
        fs::write(external.path().join("preserve"), "external\n").unwrap();

        let result = evacuate_with_checkpoint(transaction, |checkpoint| {
            if checkpoint == "after-witness-directory" {
                fs::rename(&witness, &displaced_witness)?;
                symlink(external.path(), &witness)?;
            }
            Ok(())
        });

        let Err(error) = result else {
            panic!("a substituted finish witness must be refused");
        };
        assert!(format!("{error:#}").contains("finish witness"), "{error:#}");
        assert_eq!(
            fs::read_to_string(external.path().join("preserve")).unwrap(),
            "external\n"
        );
        assert!(
            !external.path().join("original").exists(),
            "witness substitution created recovery state outside the task root"
        );
        assert_eq!(
            fs::read_to_string(grove_root.join("FORMAT")).unwrap(),
            "session-kinds-v1\n"
        );
        assert!(displaced_witness.is_dir());
    }

    #[test]
    fn transaction_file_replacement_before_evacuation_preserves_external_bytes() {
        for (checkpoint_name, file_name, description) in [
            ("after-manifest", MANIFEST_FILE, "finish manifest"),
            ("after-ready", READY_FILE, "finish ready marker"),
        ] {
            let (fixture, transaction) = prepared_plain_git_transaction();
            let repository = fixture.path();
            let grove_root = repository.join(".grove");
            let witness = preparing_witness(repository);
            let transaction_file = witness.join(file_name);
            let displaced_file = repository.join(format!("displaced-{file_name}"));
            let external = TempDir::new().unwrap();
            let external_file = external.path().join("preserve");
            fs::write(&external_file, "external\n").unwrap();

            let result = evacuate_with_checkpoint(transaction, |checkpoint| {
                if checkpoint == checkpoint_name {
                    fs::rename(&transaction_file, &displaced_file)?;
                    symlink(&external_file, &transaction_file)?;
                }
                Ok(())
            });

            let Err(error) = result else {
                panic!("a substituted {description} must be refused");
            };
            assert!(format!("{error:#}").contains(description), "{error:#}");
            assert_eq!(fs::read_to_string(&external_file).unwrap(), "external\n");
            assert_eq!(
                fs::read_to_string(grove_root.join("FORMAT")).unwrap(),
                "session-kinds-v1\n"
            );
            assert!(displaced_file.is_file());
        }
    }

    #[test]
    fn recovery_tree_replacement_before_evacuation_does_not_move_external_bytes() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");
        let original_tree = preparing_witness(repository).join("original");
        let displaced_original_tree = repository.join("displaced-original-tree");
        let external = TempDir::new().unwrap();
        fs::write(external.path().join("preserve"), "external\n").unwrap();

        let result = evacuate_with_checkpoint(transaction, |checkpoint| {
            if checkpoint == "after-recovery-tree" {
                fs::rename(&original_tree, &displaced_original_tree)?;
                symlink(external.path(), &original_tree)?;
            }
            Ok(())
        });

        let Err(error) = result else {
            panic!("a substituted finish recovery tree must be refused");
        };
        assert!(format!("{error:#}").contains("recovery tree"), "{error:#}");
        assert_eq!(
            fs::read_to_string(external.path().join("preserve")).unwrap(),
            "external\n"
        );
        assert!(
            !external.path().join("FORMAT").exists(),
            "recovery-tree substitution evacuated task bytes outside the witness"
        );
        assert_eq!(
            fs::read_to_string(grove_root.join("FORMAT")).unwrap(),
            "session-kinds-v1\n"
        );
        assert!(displaced_original_tree.is_dir());
    }

    #[test]
    fn a_foreign_task_root_entry_stops_evacuation_before_the_first_move() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let grove_root = fixture.path().join(".grove");
        let foreign = grove_root.join("foreign");

        let result = evacuate_with_checkpoint(transaction, |checkpoint| {
            if checkpoint == "after-ready" {
                fs::write(&foreign, "preserve\n")?;
            }
            Ok(())
        });

        let Err(error) = result else {
            panic!("a foreign task-root entry must block evacuation");
        };
        assert!(
            format!("{error:#}").contains("foreign task-root entries"),
            "{error:#}"
        );
        assert_eq!(fs::read_to_string(&foreign).unwrap(), "preserve\n");
        for expected in ["FORMAT", "BRIEF.md", "01-DONE-impl-work-k1.md"] {
            assert!(
                grove_root.join(expected).exists(),
                "foreign-entry refusal moved {expected}"
            );
        }
    }

    #[test]
    fn changed_source_content_mode_or_link_target_stops_evacuation_before_the_first_move() {
        for case in ["content", "mode", "link target"] {
            let (fixture, transaction) = prepared_plain_git_transaction();
            let grove_root = fixture.path().join(".grove");

            let result = evacuate_with_checkpoint(transaction, |checkpoint| {
                if checkpoint == "after-ready" {
                    match case {
                        "content" => fs::write(grove_root.join("FORMAT"), "changed\n")?,
                        "mode" => {
                            let path = grove_root.join("FORMAT");
                            let mode = fs::metadata(&path)?.permissions().mode();
                            fs::set_permissions(path, fs::Permissions::from_mode(mode ^ 0o100))?;
                        }
                        "link target" => {
                            let path = grove_root.join("current-format");
                            fs::remove_file(&path)?;
                            symlink("BRIEF.md", path)?;
                        }
                        _ => unreachable!(),
                    }
                }
                Ok(())
            });

            let Err(error) = result else {
                panic!("changed source bytes must block evacuation");
            };
            assert!(
                format!("{error:#}").contains("does not match its manifest"),
                "{case}: {error:#}"
            );
            let witness = grove_root.join("FINISHING-finish-k2");
            assert!(witness.is_dir(), "{case}");
            assert!(
                fs::read_dir(witness.join("original"))
                    .unwrap()
                    .next()
                    .is_none(),
                "{case} moved a source entry"
            );
            for expected in ["FORMAT", "BRIEF.md", "01-DONE-impl-work-k1.md"] {
                assert!(
                    grove_root.join(expected).exists(),
                    "{case} moved {expected}"
                );
            }
        }
    }

    #[test]
    fn an_occupied_recovery_destination_is_not_replaced() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let grove_root = fixture.path().join(".grove");
        let destination = preparing_witness(fixture.path())
            .join("original")
            .join("01-DONE-impl-work-k1.md");
        let published_destination = grove_root
            .join("FINISHING-finish-k2/original")
            .join("01-DONE-impl-work-k1.md");

        let result = evacuate_with_checkpoint(transaction, |checkpoint| {
            if checkpoint == "after-ready" {
                fs::write(&destination, "collision\n")?;
            }
            Ok(())
        });

        let Err(error) = result else {
            panic!("an occupied recovery destination must block evacuation");
        };
        assert!(
            format!("{error:#}").contains("evacuating finish transaction entry"),
            "{error:#}"
        );
        assert_eq!(
            fs::read_to_string(&published_destination).unwrap(),
            "collision\n"
        );
        assert_eq!(
            fs::read_to_string(grove_root.join("01-DONE-impl-work-k1.md")).unwrap(),
            "# work-k1\n"
        );
    }

    #[test]
    fn witness_creation_refusal_aborts_repository_preparation_for_same_attempt_retry() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let witness = repository.join(".grove/FINISHING-finish-k2");
        fs::create_dir(&witness).unwrap();
        fs::write(witness.join("foreign"), "preserve\n").unwrap();

        let error = evacuate(transaction)
            .err()
            .expect("a foreign witness collision must be refused");

        assert!(
            format!("{error:#}").contains("publishing ready finish witness"),
            "{error:#}"
        );
        assert_eq!(
            fs::read_to_string(witness.join("foreign")).unwrap(),
            "preserve\n"
        );
        fs::remove_dir_all(&witness).unwrap();
        repo::prepare_finish(repository, "finish-k2", "11111111111111111111111111111111")
            .unwrap()
            .abort()
            .unwrap();
    }

    #[test]
    fn synchronous_witness_materialization_failure_removes_partial_state_and_aborts_preparation() {
        for failure_point in [
            "after-witness-directory",
            "after-recovery-tree",
            "after-manifest",
            "after-ready",
            "after-ready-witness",
        ] {
            let (fixture, transaction) = prepared_plain_git_transaction();
            let repository = fixture.path();
            let witness = repository.join(".grove/FINISHING-finish-k2");

            let error = evacuate_with_checkpoint(transaction, |checkpoint| {
                if checkpoint == failure_point {
                    anyhow::bail!("injected witness materialization failure at {checkpoint}");
                }
                Ok(())
            })
            .err()
            .expect("injected witness materialization failure must be reported");

            assert!(format!("{error:#}").contains(failure_point), "{error:#}");
            assert!(
                !witness.exists(),
                "{failure_point} left an incomplete witness"
            );
            assert!(
                !preparing_witness(repository).exists(),
                "{failure_point} left a preparing witness"
            );
            repo::prepare_finish(repository, "finish-k2", "11111111111111111111111111111111")
                .unwrap()
                .abort()
                .unwrap();
        }
    }

    #[test]
    fn restart_rolls_back_a_fully_evacuated_uncommitted_finish() {
        let fixture = evacuated_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");

        let recovered = recover_pending(repository, &grove_root).unwrap();

        assert_eq!(recovered, FinishRecovery::RolledBack);
        assert!(grove_root.join("02-finish-finish-k2.md").is_file());
        assert!(!grove_root.join("FINISHING-finish-k2").exists());
        assert_eq!(
            run_git(repository, &["log", "-1", "--format=%s"]),
            "fixture"
        );
    }

    #[test]
    fn recovery_refuses_a_symlinked_original_tree_without_moving_its_target() {
        let fixture = evacuated_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");
        let original_tree = grove_root.join("FINISHING-finish-k2/original");
        let external_tree = repository.join("external-original-tree");
        fs::rename(&original_tree, &external_tree).unwrap();
        symlink(&external_tree, &original_tree).unwrap();

        let error = recover_pending(repository, &grove_root).unwrap_err();

        assert!(
            format!("{error:#}").contains("finish recovery tree"),
            "{error:#}"
        );
        for expected in ["FORMAT", "BRIEF.md", "01-DONE-impl-work-k1.md"] {
            assert!(
                external_tree.join(expected).exists(),
                "recovery moved {expected} through a substituted original-tree path"
            );
            assert!(!grove_root.join(expected).exists());
        }
    }

    #[test]
    fn recovery_refuses_foreign_entries_beside_a_ready_witness() {
        let fixture = evacuated_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");
        let foreign = grove_root.join("foreign");
        fs::write(&foreign, "preserve\n").unwrap();

        let error = recover_pending(repository, &grove_root).unwrap_err();

        assert!(
            format!("{error:#}").contains("foreign task-root entries"),
            "{error:#}"
        );
        assert_eq!(fs::read_to_string(&foreign).unwrap(), "preserve\n");
        assert!(
            grove_root
                .join("FINISHING-finish-k2/original/FORMAT")
                .exists(),
            "foreign-entry refusal must happen before rollback"
        );
        assert!(!grove_root.join("FORMAT").exists());
    }

    #[test]
    fn lifecycle_recovers_a_finish_witness_before_ordinary_tree_classification() {
        let fixture = evacuated_plain_git_transaction();
        let repository = fixture.path();

        let transition = crate::tree_lifecycle::transition_to_current(repository).unwrap();

        assert_eq!(
            transition,
            crate::tree_lifecycle::CurrentTransition::AlreadyCurrent
        );
        assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());
    }

    #[test]
    fn divergent_repository_keeps_the_witness_with_both_recovery_paths() {
        let fixture = evacuated_plain_git_transaction();
        let repository = fixture.path();
        fs::write(repository.join("divergent"), "preserve\n").unwrap();
        run_git(repository, &["add", "divergent"]);
        run_git(repository, &["commit", "-q", "-m", "divergent"]);
        let witness = repository.join(".grove/FINISHING-finish-k2");

        let error = recover_pending(repository, &repository.join(".grove")).unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("Recovery pending"), "{message}");
        assert!(message.contains("recorded start"), "{message}");
        assert!(message.contains("exact teardown result"), "{message}");
        assert!(witness.is_dir());
    }

    #[test]
    fn quarantine_handoff_requires_one_filesystem() {
        ensure_same_device(
            7,
            7,
            Path::new("/work/.grove"),
            Path::new("/work/.git/grove"),
        )
        .unwrap();

        let error = ensure_same_device(
            7,
            11,
            Path::new("/work/.grove"),
            Path::new("/other/.git/grove"),
        )
        .unwrap_err();

        let message = error.to_string();
        assert!(message.contains("atomic quarantine"), "{message}");
        assert!(message.contains("/work/.grove"), "{message}");
        assert!(message.contains("/other/.git/grove"), "{message}");
    }
}
