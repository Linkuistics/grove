use crate::{driver_lease, finish_cleanup, repo};
use anyhow::{bail, Context, Result};
use finish_cleanup::unix::{
    create_directory_at, create_new_file_at, directory_names, open_directory_at, open_file_at,
    rename_at_noreplace, unlink_at,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FinishRecovery {
    None,
    RolledBack,
    Committed,
}

pub(crate) fn cleanup_owner(grove_root: &Path) -> Result<Option<finish_cleanup::CleanupOwner>> {
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
    let Some(pending) = pending_manifest(grove_root)? else {
        return Ok(FinishRecovery::None);
    };
    let witness_path = pending.witness_path;
    let manifest = pending.manifest;
    let directories = pending.directories;
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
            rollback(&transaction, &directories, proof)?;
            Ok(FinishRecovery::RolledBack)
        }
        repo::FinishRecoveryOutcome::RecoveryPending(error) => Err(error.context(format!(
            "Recovery pending at {}; preserve divergent work, restore the recorded start or the exact teardown result, then retry",
            witness_path.display()
        ))),
    }
}

fn pending_manifest(grove_root: &Path) -> Result<Option<PendingFinish>> {
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
    if root_entries != [witness_name.clone()] {
        bail!(
            "Recovery pending: finish task root contains foreign task-root entries beside the ready witness; observed {root_entries:?}; no repository classification was attempted"
        );
    }
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
    grove_root_directory: File,
    mut checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<TransactionDirectories> {
    let manifest_body = serde_json::to_vec_pretty(&transaction.manifest)
        .context("serializing finish transaction manifest")?;
    let witness_name = transaction
        .witness_path
        .file_name()
        .context("finish transaction witness has no file name")?
        .to_os_string();
    create_directory_at(&grove_root_directory, &witness_name).with_context(|| {
        format!(
            "creating finish transaction witness {}",
            transaction.witness_path.display()
        )
    })?;
    let witness_directory = open_directory_at(&grove_root_directory, &witness_name)
        .with_context(|| format!("opening finish witness {witness_name:?}"))?;
    let mut original_tree_directory = None;
    let mut manifest_file = None;
    let mut ready_file = None;
    let result = (|| {
        checkpoint("after-witness-directory")?;
        revalidate_open_directory_at(
            &grove_root_directory,
            &witness_name,
            &witness_directory,
            "finish witness",
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
        checkpoint("after-ready")
    })();
    match result {
        Ok(()) => Ok(TransactionDirectories {
            grove_root: grove_root_directory,
            witness: witness_directory,
            original_tree: original_tree_directory.expect("recovery tree was opened"),
            manifest: manifest_file.expect("manifest was created"),
            ready: ready_file.expect("ready marker was created"),
            witness_name,
        }),
        Err(error) => match remove_incomplete_witness(
            transaction,
            &grove_root_directory,
            &witness_directory,
            &witness_name,
            original_tree_directory.as_ref(),
            manifest_file.as_ref(),
            ready_file.as_ref(),
        ) {
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
    evacuate_with_checkpoint(prepared, |_| Ok(()))
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
    Ok(())
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
    let directories =
        match materialize_witness_with_checkpoint(&transaction, grove_root_directory, checkpoint) {
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
        let actual = digest_entry(&transaction.grove_root, &destination)?;
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
        MANIFEST_FILE, READY_FILE,
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
    fn witness_replacement_during_materialization_does_not_touch_external_bytes() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let repository = fixture.path();
        let grove_root = repository.join(".grove");
        let witness = grove_root.join("FINISHING-finish-k2");
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
            let witness = grove_root.join("FINISHING-finish-k2");
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
        let original_tree = grove_root.join("FINISHING-finish-k2").join("original");
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
    fn an_occupied_recovery_destination_is_not_replaced() {
        let (fixture, transaction) = prepared_plain_git_transaction();
        let grove_root = fixture.path().join(".grove");
        let destination = grove_root
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
        assert_eq!(fs::read_to_string(&destination).unwrap(), "collision\n");
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
