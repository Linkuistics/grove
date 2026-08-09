use crate::{driver_lease, repo};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
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
    let prepared_commit = repo::prepare_finish(worktree)?;
    let transaction = prepare_transaction(
        grove_root,
        finish_handle,
        attempt_identity,
        preflight.entry_digests.clone(),
        &preflight.quarantine_directory,
        prepared_commit.start_anchor(),
        prepared_commit.deletion_fingerprint(),
    )?;
    evacuate(&transaction)?;
    match prepared_commit.commit(finish_handle, &transaction.manifest.attempt_identity) {
        repo::FinishCommitOutcome::Committed(proof) => {
            quarantine_and_dispose(grove_root, &transaction.quarantine_path, &proof)
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
    _grove_root: File,
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

struct PreparedTransaction {
    grove_root: PathBuf,
    witness_path: PathBuf,
    original_tree: PathBuf,
    quarantine_path: PathBuf,
    manifest: FinishManifest,
}

fn preflight_root(worktree: &Path, grove_root: &Path) -> Result<FinishPreflight> {
    let grove_directory = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(grove_root)
        .with_context(|| {
            format!(
                "opening task root without following symlinks: {}",
                grove_root.display()
            )
        })?;
    let descriptor_metadata = grove_directory.metadata().with_context(|| {
        format!(
            "reading opened task-root identity: {}",
            grove_root.display()
        )
    })?;
    let path_metadata = fs::symlink_metadata(grove_root)
        .with_context(|| format!("revalidating task-root identity: {}", grove_root.display()))?;
    if descriptor_metadata.dev() != path_metadata.dev()
        || descriptor_metadata.ino() != path_metadata.ino()
    {
        bail!(
            "task root changed while finish preflight opened it: {}",
            grove_root.display()
        );
    }

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
        _grove_root: grove_directory,
        entry_digests: digest_root_entries(grove_root)?,
        quarantine_directory: control.control_dir().to_path_buf(),
    })
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
    finish_handle: &str,
    attempt_identity: String,
    entries: Vec<RootEntryDigest>,
    quarantine_directory: &Path,
    repository_anchor: repo::FinishStartAnchor,
    deletion_fingerprint: [u8; 32],
) -> Result<PreparedTransaction> {
    fs::create_dir_all(quarantine_directory).with_context(|| {
        format!(
            "creating workspace-control quarantine directory {}",
            quarantine_directory.display()
        )
    })?;
    let quarantine_path =
        quarantine_directory.join(format!("FINISHED-{finish_handle}-{attempt_identity}"));
    if fs::symlink_metadata(&quarantine_path).is_ok() {
        bail!(
            "finish cleanup quarantine already exists: {}",
            quarantine_path.display()
        );
    }

    let witness_path = grove_root.join(format!("FINISHING-{finish_handle}"));
    fs::create_dir(&witness_path).with_context(|| {
        format!(
            "creating finish transaction witness {}",
            witness_path.display()
        )
    })?;
    let original_tree = witness_path.join(ORIGINAL_TREE_DIRECTORY);
    fs::create_dir(&original_tree).with_context(|| {
        format!(
            "creating finish transaction recovery tree {}",
            original_tree.display()
        )
    })?;
    let manifest = FinishManifest {
        version: 1,
        finish_handle: finish_handle.to_owned(),
        attempt_identity,
        repository_anchor,
        deletion_fingerprint,
        entries,
    };
    let manifest_body =
        serde_json::to_vec_pretty(&manifest).context("serializing finish transaction manifest")?;
    fs::write(witness_path.join(MANIFEST_FILE), manifest_body).with_context(|| {
        format!(
            "writing finish transaction manifest {}",
            witness_path.join(MANIFEST_FILE).display()
        )
    })?;
    fs::write(witness_path.join(READY_FILE), b"ready\n").with_context(|| {
        format!(
            "marking finish transaction ready at {}",
            witness_path.display()
        )
    })?;
    Ok(PreparedTransaction {
        grove_root: grove_root.to_path_buf(),
        witness_path,
        original_tree,
        quarantine_path,
        manifest,
    })
}

fn entry_path(root: &Path, path: &[u8]) -> PathBuf {
    root.join(OsString::from_vec(path.to_vec()))
}

fn evacuate(transaction: &PreparedTransaction) -> Result<()> {
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
    Ok(())
}

fn rollback(transaction: &PreparedTransaction, proof: repo::FinishStartProof) -> Result<()> {
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
    proof: &repo::FinishProof,
) -> Result<()> {
    proof.revalidate()?;
    fs::rename(grove_root, quarantine_path).with_context(|| {
        format!(
            "atomically quarantining completed task root {} at {}",
            grove_root.display(),
            quarantine_path.display()
        )
    })?;
    if let Err(error) = proof.revalidate() {
        fs::rename(quarantine_path, grove_root).with_context(|| {
            format!(
                "restoring finish quarantine {} after repository proof changed",
                quarantine_path.display()
            )
        })?;
        return Err(error.context("Recovery pending: Git finish proof changed after quarantine"));
    }
    dispose_quarantine_with(quarantine_path, |path| fs::remove_dir_all(path))
}

fn dispose_quarantine_with(
    quarantine_path: &Path,
    remove: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<()> {
    if let Err(error) = remove(quarantine_path) {
        eprintln!(
            "warning: completed Grove cleanup remains at {}: {error}",
            quarantine_path.display()
        );
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
    use super::{digest_root_entries, dispose_quarantine_with, ensure_same_device};
    use std::fs;
    use std::os::unix::fs::{symlink, PermissionsExt};
    use std::path::Path;
    use tempfile::TempDir;

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

    #[test]
    fn cleanup_failure_leaves_a_complete_quarantine_for_later_reaping() {
        let fixture = TempDir::new().unwrap();
        let quarantine = fixture.path().join("FINISHED-finish-k2-attempt");
        fs::create_dir(&quarantine).unwrap();
        fs::write(quarantine.join("manifest"), "keep\n").unwrap();

        dispose_quarantine_with(&quarantine, |_| {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "simulated cleanup refusal",
            ))
        })
        .unwrap();

        assert!(quarantine.is_dir());
        assert_eq!(
            fs::read_to_string(quarantine.join("manifest")).unwrap(),
            "keep\n"
        );
    }

    #[test]
    fn quarantine_disposal_unlinks_symlinks_without_following_targets() {
        let fixture = TempDir::new().unwrap();
        let target = fixture.path().join("target");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("preserved"), "keep\n").unwrap();
        let quarantine = fixture.path().join("FINISHED-finish-k2-attempt");
        fs::create_dir(&quarantine).unwrap();
        symlink(&target, quarantine.join("external-link")).unwrap();

        dispose_quarantine_with(&quarantine, |path| fs::remove_dir_all(path)).unwrap();

        assert!(!quarantine.exists());
        assert_eq!(
            fs::read_to_string(target.join("preserved")).unwrap(),
            "keep\n"
        );
    }
}
