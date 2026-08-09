use crate::repo;
use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

const WITNESS_PREFIX: &[u8] = b"FINISHING-";

pub(crate) fn finish(worktree: &Path, grove_root: &Path, finish_handle: &str) -> Result<()> {
    let _preflight = preflight_root(worktree, grove_root)?;
    repo::validate_finish_commit(worktree)?;
    fs::remove_dir_all(grove_root)
        .with_context(|| format!("deleting completed grove {}", grove_root.display()))?;
    repo::commit_finish(worktree, finish_handle)
}

#[derive(Debug, PartialEq, Eq)]
struct RootEntryDigest {
    path: Vec<u8>,
    entry_type: EntryType,
    mode: u32,
    digest: [u8; 32],
}

#[derive(Debug, PartialEq, Eq)]
enum EntryType {
    Directory,
    File,
    Symlink,
}

struct FinishPreflight {
    _grove_root: File,
    _entry_digests: Vec<RootEntryDigest>,
    _quarantine_directory: PathBuf,
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
        _entry_digests: digest_root_entries(grove_root)?,
        _quarantine_directory: control.control_dir().to_path_buf(),
    })
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
    use super::{digest_root_entries, ensure_same_device};
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
}
