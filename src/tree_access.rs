use crate::tree_id::{parse, Entry};
use anyhow::{bail, Context, Result};
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

const PROMOTING_PREFIX: &str = "PROMOTING-";

pub struct TreeReadGuard {
    _root_directory: File,
    root: PathBuf,
}

pub struct TreeWriteGuard {
    _root_directory: File,
    root: PathBuf,
}

impl TreeReadGuard {
    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl TreeWriteGuard {
    pub fn root(&self) -> &Path {
        &self.root
    }
}

pub fn read(grove_root: &Path) -> Result<TreeReadGuard> {
    let (root, root_directory) = acquire(grove_root, libc::LOCK_SH)?;
    refuse_pending(&root)?;
    Ok(TreeReadGuard {
        _root_directory: root_directory,
        root,
    })
}

pub fn write(grove_root: &Path) -> Result<TreeWriteGuard> {
    let (root, root_directory) = acquire(grove_root, libc::LOCK_EX)?;
    refuse_pending(&root)?;
    Ok(TreeWriteGuard {
        _root_directory: root_directory,
        root,
    })
}

/// Promotion takes the same exclusive lock but must inspect and recover the
/// witness every other operation refuses.
pub(crate) fn write_for_promotion(grove_root: &Path) -> Result<TreeWriteGuard> {
    let (root, root_directory) = acquire(grove_root, libc::LOCK_EX)?;
    Ok(TreeWriteGuard {
        _root_directory: root_directory,
        root,
    })
}

fn acquire(grove_root: &Path, operation: libc::c_int) -> Result<(PathBuf, File)> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    // Keep the caller's spelling for returned paths. On macOS `/var` and
    // `/private/var` name the same inode; canonicalising here would make adding
    // locking observably rewrite every `pick` path even though the descriptor
    // lock itself already follows the filesystem identity.
    let root = grove_root.to_path_buf();
    let root_directory = File::open(&root)
        .with_context(|| format!("opening grove root {} for tree access", root.display()))?;
    let descriptor = root_directory.as_raw_fd();
    let first = unsafe { libc::flock(descriptor, operation | libc::LOCK_NB) };
    if first != 0 {
        let error = std::io::Error::last_os_error();
        if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
        {
            eprintln!("waiting for active Grove tree operation");
            let waited = unsafe { libc::flock(descriptor, operation) };
            if waited != 0 {
                return Err(std::io::Error::last_os_error())
                    .context("waiting for the active Grove tree operation");
            }
        } else {
            return Err(error).context("locking the Grove task tree");
        }
    }
    Ok((root, root_directory))
}

fn refuse_pending(grove_root: &Path) -> Result<()> {
    if let Some(path) = find_pending(grove_root)? {
        bail!(
            "pending Grove promotion transaction: {}. Recover it with: \
             grove-llm leaf-promote-chain {}",
            path.display(),
            path.display()
        );
    }
    Ok(())
}

pub(crate) fn find_pending(grove_root: &Path) -> Result<Option<PathBuf>> {
    find_pending_in(grove_root)
}

fn find_pending_in(directory: &Path) -> Result<Option<PathBuf>> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .with_context(|| format!("reading task-tree directory {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        let path = entry.path();
        if name.starts_with(PROMOTING_PREFIX) {
            return Ok(Some(path));
        }
        if matches!(parse(name), Some(Entry::Node { .. })) {
            if let Some(found) = find_pending_in(&path)? {
                return Ok(Some(found));
            }
        }
    }
    Ok(None)
}

pub(crate) fn promoting_prefix() -> &'static str {
    PROMOTING_PREFIX
}
