use crate::{driver_lease, finish_cleanup, repo};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

const WITNESS_PREFIX: &[u8] = b"FINISHING-";
const MANIFEST_FILE: &str = "MANIFEST.json";
const ORIGINAL_TREE_DIRECTORY: &str = "original";
const READY_FILE: &str = "READY";

pub(crate) fn finish(worktree: &Path, grove_root: &Path, finish_handle: &str) -> Result<()> {
    let preflight = preflight_root(worktree, grove_root)?;
    let attempt_identity = finish_attempt_identity()?;
    let prepared_commit = repo::prepare_finish(worktree, finish_handle, &attempt_identity)?;
    let transaction = prepare_transaction(
        grove_root,
        preflight,
        finish_handle,
        attempt_identity,
        prepared_commit,
    )?;
    let EvacuatedTransaction {
        transaction,
        prepared_commit,
    } = evacuate(transaction)?;
    finish_test_checkpoint("after-evacuation")?;
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
            match rollback(&transaction, proof) {
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
    if std::env::var("GROVE_TEST_FINISH_FAIL_AT").as_deref() == Ok(name) {
        bail!("injected finish interruption at {name}");
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
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
    grove_root_directory: File,
    prepared_commit: repo::PreparedFinish,
}

struct EvacuatedTransaction {
    transaction: FinishTransaction,
    prepared_commit: repo::PreparedFinish,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FinishRecovery {
    None,
    RolledBack,
    Committed,
}

pub(crate) fn cleanup_owner(grove_root: &Path) -> Result<Option<finish_cleanup::CleanupOwner>> {
    let Some((_, manifest)) = pending_manifest(grove_root)? else {
        return Ok(None);
    };
    finish_cleanup::CleanupOwner::new(manifest.finish_handle, manifest.attempt_identity).map(Some)
}

pub(crate) fn recover_pending(worktree: &Path, grove_root: &Path) -> Result<FinishRecovery> {
    let Some((witness_path, manifest)) = pending_manifest(grove_root)? else {
        return Ok(FinishRecovery::None);
    };
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
            rollback(&transaction, proof)?;
            Ok(FinishRecovery::RolledBack)
        }
        repo::FinishRecoveryOutcome::RecoveryPending(error) => Err(error.context(format!(
            "Recovery pending at {}; preserve divergent work, restore the recorded start or the exact teardown result, then retry",
            witness_path.display()
        ))),
    }
}

fn pending_manifest(grove_root: &Path) -> Result<Option<(PathBuf, FinishManifest)>> {
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
    let mut witnesses = fs::read_dir(grove_root)
        .with_context(|| {
            format!(
                "scanning task root for a pending finish transaction: {}",
                grove_root.display()
            )
        })?
        .filter_map(|entry| match entry {
            Ok(entry) if entry.file_name().as_bytes().starts_with(WITNESS_PREFIX) => {
                Some(Ok(entry.path()))
            }
            Ok(_) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<std::io::Result<Vec<_>>>()?;
    if witnesses.is_empty() {
        return Ok(None);
    }
    witnesses.sort();
    if witnesses.len() != 1 {
        bail!(
            "Recovery pending: multiple finish transaction witnesses exist: {}",
            witnesses
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let witness_path = witnesses.pop().expect("one finish witness");
    if !fs::symlink_metadata(&witness_path)?.file_type().is_dir() {
        bail!(
            "Recovery pending: finish witness is not a real directory at {}",
            witness_path.display()
        );
    }
    let ready_path = witness_path.join(READY_FILE);
    let ready = read_regular_file_no_follow(&ready_path).with_context(|| {
        format!(
            "Recovery pending: validating finish cleanup ownership ready marker at {}",
            ready_path.display()
        )
    })?;
    if ready != b"ready\n" {
        bail!(
            "Recovery pending: finish witness has an invalid ready marker at {}",
            witness_path.display()
        );
    }
    let manifest_path = witness_path.join(MANIFEST_FILE);
    let manifest_body = read_regular_file_no_follow(&manifest_path).with_context(|| {
        format!(
            "Recovery pending: validating finish cleanup ownership manifest at {}",
            manifest_path.display()
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
    let expected_witness = format!("FINISHING-{}", manifest.finish_handle);
    if witness_path.file_name().and_then(|name| name.to_str()) != Some(&expected_witness) {
        bail!(
            "Recovery pending: finish manifest handle {} does not match witness {}",
            manifest.finish_handle,
            witness_path.display()
        );
    }
    let original_tree = witness_path.join(ORIGINAL_TREE_DIRECTORY);
    let observed_entries = digest_root_entries(&original_tree).with_context(|| {
        format!(
            "Recovery pending: validating evacuated finish tree at {}",
            original_tree.display()
        )
    })?;
    if observed_entries != manifest.entries {
        bail!(
            "Recovery pending: evacuated finish tree does not match its manifest at {}",
            witness_path.display()
        );
    }
    Ok(Some((witness_path, manifest)))
}

fn read_regular_file_no_follow(path: &Path) -> Result<Vec<u8>> {
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)
        .with_context(|| format!("opening {} without following symlinks", path.display()))?;
    if !file
        .metadata()
        .with_context(|| format!("reading identity for {}", path.display()))?
        .file_type()
        .is_file()
    {
        bail!(
            "finish transaction object is not a regular file: {}",
            path.display()
        );
    }
    let mut body = Vec::new();
    file.read_to_end(&mut body)
        .with_context(|| format!("reading {}", path.display()))?;
    Ok(body)
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

    for entry in fs::read_dir(grove_root)
        .with_context(|| format!("reading finish transaction input {}", grove_root.display()))?
    {
        let entry = entry?;
        if entry.file_name().as_bytes().starts_with(WITNESS_PREFIX) {
            bail!(
                "reserved finish transaction path already exists: {}",
                entry.path().display()
            );
        }
    }
    Ok(FinishPreflight {
        grove_root_directory: grove_directory,
        entry_digests: digest_root_entries(grove_root)?,
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
    prepared_commit: repo::PreparedFinish,
) -> Result<PreparedTransaction> {
    if let Err(error) = revalidate_task_root(grove_root, &preflight.grove_root_directory) {
        return Err(abort_prepared_finish(prepared_commit, error));
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
            grove_root_directory: preflight.grove_root_directory,
            prepared_commit,
        }),
        Err(error) => Err(abort_prepared_finish(prepared_commit, error)),
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
struct DirectoryIdentity {
    device: u64,
    inode: u64,
}

fn materialize_witness_with_checkpoint(
    transaction: &FinishTransaction,
    mut checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<()> {
    let manifest_body = serde_json::to_vec_pretty(&transaction.manifest)
        .context("serializing finish transaction manifest")?;
    fs::create_dir(&transaction.witness_path).with_context(|| {
        format!(
            "creating finish transaction witness {}",
            transaction.witness_path.display()
        )
    })?;
    let witness_metadata = fs::symlink_metadata(&transaction.witness_path).with_context(|| {
        format!(
            "recording finish transaction witness identity {}",
            transaction.witness_path.display()
        )
    })?;
    if !witness_metadata.file_type().is_dir() {
        bail!(
            "Recovery pending: newly created finish witness is no longer a real directory at {}; preserve it for exact ownership recovery",
            transaction.witness_path.display()
        );
    }
    let identity = DirectoryIdentity {
        device: witness_metadata.dev(),
        inode: witness_metadata.ino(),
    };
    let result = (|| {
        checkpoint("after-witness-directory")?;
        fs::create_dir(&transaction.original_tree).with_context(|| {
            format!(
                "creating finish transaction recovery tree {}",
                transaction.original_tree.display()
            )
        })?;
        checkpoint("after-recovery-tree")?;
        fs::write(transaction.witness_path.join(MANIFEST_FILE), manifest_body).with_context(
            || {
                format!(
                    "writing finish transaction manifest {}",
                    transaction.witness_path.join(MANIFEST_FILE).display()
                )
            },
        )?;
        checkpoint("after-manifest")?;
        fs::write(transaction.witness_path.join(READY_FILE), b"ready\n").with_context(|| {
            format!(
                "marking finish transaction ready at {}",
                transaction.witness_path.display()
            )
        })?;
        checkpoint("after-ready")
    })();
    match result {
        Ok(()) => Ok(()),
        Err(error) => match remove_incomplete_witness(transaction, identity) {
            Ok(()) => Err(error),
            Err(cleanup_error) => Err(error.context(format!(
                "Recovery pending: removing the incomplete finish witness {} failed: {cleanup_error:#}; preserve it for exact ownership recovery",
                transaction.witness_path.display()
            ))),
        },
    }
}

fn remove_incomplete_witness(
    transaction: &FinishTransaction,
    expected: DirectoryIdentity,
) -> Result<()> {
    revalidate_witness_identity(&transaction.witness_path, expected)?;
    let mut has_original_tree = false;
    let mut has_manifest = false;
    let mut has_ready = false;
    for entry in fs::read_dir(&transaction.witness_path).with_context(|| {
        format!(
            "reading incomplete finish witness {}",
            transaction.witness_path.display()
        )
    })? {
        let entry = entry?;
        let name = entry.file_name();
        let metadata = fs::symlink_metadata(entry.path())?;
        match name.as_bytes() {
            name if name == ORIGINAL_TREE_DIRECTORY.as_bytes() => {
                if !metadata.file_type().is_dir() || fs::read_dir(entry.path())?.next().is_some() {
                    bail!(
                        "incomplete finish recovery tree is not an empty real directory: {}",
                        entry.path().display()
                    );
                }
                has_original_tree = true;
            }
            name if name == MANIFEST_FILE.as_bytes() => {
                if !metadata.file_type().is_file() {
                    bail!(
                        "incomplete finish manifest is not a regular file: {}",
                        entry.path().display()
                    );
                }
                has_manifest = true;
            }
            name if name == READY_FILE.as_bytes() => {
                if !metadata.file_type().is_file() {
                    bail!(
                        "incomplete finish ready marker is not a regular file: {}",
                        entry.path().display()
                    );
                }
                has_ready = true;
            }
            _ => bail!(
                "incomplete finish witness contains an unexpected entry: {}",
                entry.path().display()
            ),
        }
    }
    revalidate_witness_identity(&transaction.witness_path, expected)?;
    if has_ready {
        fs::remove_file(transaction.witness_path.join(READY_FILE))?;
    }
    if has_manifest {
        fs::remove_file(transaction.witness_path.join(MANIFEST_FILE))?;
    }
    if has_original_tree {
        fs::remove_dir(&transaction.original_tree)?;
    }
    revalidate_witness_identity(&transaction.witness_path, expected)?;
    fs::remove_dir(&transaction.witness_path).with_context(|| {
        format!(
            "removing incomplete finish witness {}",
            transaction.witness_path.display()
        )
    })
}

fn revalidate_witness_identity(path: &Path, expected: DirectoryIdentity) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("revalidating incomplete finish witness {}", path.display()))?;
    if !metadata.file_type().is_dir()
        || metadata.dev() != expected.device
        || metadata.ino() != expected.inode
    {
        bail!(
            "incomplete finish witness identity changed at {}; expected directory device/inode {}/{}, observed device/inode {}/{}",
            path.display(),
            expected.device,
            expected.inode,
            metadata.dev(),
            metadata.ino()
        );
    }
    Ok(())
}

fn entry_path(root: &Path, path: &[u8]) -> PathBuf {
    root.join(OsString::from_vec(path.to_vec()))
}

fn evacuate(prepared: PreparedTransaction) -> Result<EvacuatedTransaction> {
    evacuate_with_checkpoint(prepared, |_| Ok(()))
}

fn evacuate_with_checkpoint(
    prepared: PreparedTransaction,
    checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<EvacuatedTransaction> {
    let PreparedTransaction {
        transaction,
        grove_root_directory,
        prepared_commit,
    } = prepared;
    if let Err(error) = revalidate_task_root(&transaction.grove_root, &grove_root_directory) {
        return Err(abort_prepared_finish(prepared_commit, error));
    }
    if let Err(error) = materialize_witness_with_checkpoint(&transaction, checkpoint) {
        return Err(abort_prepared_finish(prepared_commit, error));
    }
    for entry in &transaction.manifest.entries {
        let source = entry_path(&transaction.grove_root, &entry.path);
        let destination = entry_path(&transaction.original_tree, &entry.path);
        fs::rename(&source, &destination).with_context(|| {
            format!(
                "evacuating finish transaction entry {} to {}",
                source.display(),
                destination.display()
            )
        })?;
    }
    Ok(EvacuatedTransaction {
        transaction,
        prepared_commit,
    })
}

fn rollback(transaction: &FinishTransaction, proof: repo::FinishStartProof) -> Result<()> {
    proof.revalidate_before_rollback()?;
    for expected in &transaction.manifest.entries {
        let source = entry_path(&transaction.original_tree, &expected.path);
        let destination = entry_path(&transaction.grove_root, &expected.path);
        fs::rename(&source, &destination).with_context(|| {
            format!(
                "restoring finish transaction entry {} to {}",
                source.display(),
                destination.display()
            )
        })?;
        let actual = digest_entry(&transaction.grove_root, &destination)?;
        if actual != *expected {
            bail!(
                "restored finish transaction entry does not match its manifest: {}",
                destination.display()
            );
        }
    }
    proof.revalidate_after_rollback()?;
    fs::remove_dir_all(&transaction.witness_path).with_context(|| {
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

fn digest_root_entries(grove_root: &Path) -> Result<Vec<RootEntryDigest>> {
    let mut entries = sorted_entries(grove_root)?;
    let mut digests = Vec::with_capacity(entries.len());
    for entry in entries.drain(..) {
        digests.push(digest_entry(grove_root, &entry.path())?);
    }
    Ok(digests)
}

fn digest_entry(grove_root: &Path, path: &Path) -> Result<RootEntryDigest> {
    let relative = path.strip_prefix(grove_root).with_context(|| {
        format!(
            "computing finish transaction path relative to {}: {}",
            grove_root.display(),
            path.display()
        )
    })?;
    let path_bytes = relative.as_os_str().as_bytes().to_vec();
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("reading finish transaction entry {}", path.display()))?;
    let file_type = metadata.file_type();
    let (entry_type, type_bytes, mode) = if file_type.is_dir() {
        (
            EntryType::Directory,
            b"directory".as_slice(),
            metadata.mode() & 0o7777,
        )
    } else if file_type.is_file() {
        (
            EntryType::File,
            b"file".as_slice(),
            metadata.mode() & 0o7777,
        )
    } else if file_type.is_symlink() {
        (EntryType::Symlink, b"symlink".as_slice(), 0)
    } else {
        bail!(
            "unsupported task-tree entry in finish transaction: {}",
            path.display()
        );
    };

    let mut hasher = Sha256::new();
    update_field(&mut hasher, &path_bytes);
    update_field(&mut hasher, type_bytes);
    hasher.update(u64::from(mode).to_le_bytes());
    match entry_type {
        EntryType::Directory => {
            let entries = sorted_entries(path)?;
            hasher.update((entries.len() as u64).to_le_bytes());
            for child in entries {
                let child = digest_entry(grove_root, &child.path())?;
                update_field(&mut hasher, &child.digest);
            }
        }
        EntryType::File => {
            let body = fs::read(path)
                .with_context(|| format!("reading finish transaction file {}", path.display()))?;
            update_field(&mut hasher, &body);
        }
        EntryType::Symlink => {
            let target = fs::read_link(path).with_context(|| {
                format!("reading finish transaction symlink {}", path.display())
            })?;
            update_field(&mut hasher, target.as_os_str().as_bytes());
        }
    }

    Ok(RootEntryDigest {
        path: path_bytes,
        entry_type,
        mode,
        digest: hasher.finalize().into(),
    })
}

fn sorted_entries(directory: &Path) -> Result<Vec<fs::DirEntry>> {
    let mut entries = fs::read_dir(directory)
        .with_context(|| format!("reading finish transaction input {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by(|left, right| {
        left.file_name()
            .as_bytes()
            .cmp(right.file_name().as_bytes())
    });
    Ok(entries)
}

fn update_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

#[cfg(test)]
mod tests {
    use super::{
        digest_root_entries, ensure_same_device, evacuate, evacuate_with_checkpoint,
        preflight_root, prepare_transaction, recover_pending, FinishRecovery, PreparedTransaction,
    };
    use crate::repo;
    use std::fs;
    use std::os::unix::fs::{symlink, PermissionsExt};
    use std::path::Path;
    use std::process::Command;
    use tempfile::TempDir;

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
        run_git(repository, &["add", "-A"]);
        run_git(repository, &["commit", "-q", "-m", "fixture"]);
        fs::write(grove_root.join("02-finish-finish-k2.md"), "# finish-k2\n").unwrap();

        let preflight = preflight_root(repository, &grove_root).unwrap();
        let prepared =
            repo::prepare_finish(repository, "finish-k2", "11111111111111111111111111111111")
                .unwrap();
        let transaction = prepare_transaction(
            &grove_root,
            preflight,
            "finish-k2",
            "11111111111111111111111111111111".to_owned(),
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
            format!("{error:#}").contains("creating finish transaction witness"),
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
