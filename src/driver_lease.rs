use crate::repo;
use anyhow::{bail, Context, Result};
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

const LEASE_FILE_NAME: &str = "driver.lease";
const IDENTITY_RETRY_LIMIT: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FileIdentity {
    device: u64,
    inode: u64,
}

impl FileIdentity {
    fn from_metadata(metadata: &fs::Metadata) -> Self {
        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
        }
    }
}

/// Process-scoped ownership of one exact Grove working tree.
///
/// Dropping this guard closes both descriptors. The kernel then releases the
/// advisory lease; bytes left in the lease file do not carry ownership.
#[derive(Debug)]
pub struct DriverLease {
    worktree_root: PathBuf,
    worktree_directory: File,
    worktree_identity: FileIdentity,
    lease_path: PathBuf,
    lease_file: File,
    lease_identity: FileIdentity,
}

impl DriverLease {
    pub fn acquire(path: &Path) -> Result<Self> {
        let control = repo::workspace_control(path)?;
        fs::create_dir_all(control.control_dir()).with_context(|| {
            format!(
                "creating Grove control directory {}",
                control.control_dir().display()
            )
        })?;

        let worktree_root = control.worktree_root().to_path_buf();
        let worktree_directory = File::open(&worktree_root).with_context(|| {
            format!(
                "opening working tree root {} for driver ownership",
                worktree_root.display()
            )
        })?;
        ensure_close_on_exec(worktree_directory.as_raw_fd())?;
        let worktree_identity = FileIdentity::from_metadata(
            &worktree_directory
                .metadata()
                .context("reading pinned working-tree identity")?,
        );

        let lease_path = control.control_dir().join(LEASE_FILE_NAME);
        let (mut lease_file, lease_identity) = acquire_lease_file(&lease_path, &worktree_root)?;
        let nonce = process_nonce()?;
        write_record(&mut lease_file, worktree_identity, &hex_nonce(nonce)?)?;

        let lease = Self {
            worktree_root,
            worktree_directory,
            worktree_identity,
            lease_path,
            lease_file,
            lease_identity,
        };
        lease.revalidate()?;
        Ok(lease)
    }

    pub fn worktree_root(&self) -> &Path {
        &self.worktree_root
    }

    /// Confirm that the paths still name the descriptors this process owns.
    pub fn revalidate(&self) -> Result<()> {
        let pinned_root = FileIdentity::from_metadata(
            &self
                .worktree_directory
                .metadata()
                .context("reading pinned working-tree identity")?,
        );
        let current_root =
            FileIdentity::from_metadata(&fs::metadata(&self.worktree_root).with_context(|| {
                format!(
                    "reading working tree root {} during driver lease revalidation",
                    self.worktree_root.display()
                )
            })?);
        if pinned_root != self.worktree_identity || current_root != self.worktree_identity {
            bail!(
                "working tree root was replaced while Grove held its driver lease: {}",
                self.worktree_root.display()
            );
        }

        let pinned_lease = FileIdentity::from_metadata(
            &self
                .lease_file
                .metadata()
                .context("reading held driver lease identity")?,
        );
        let current_lease =
            FileIdentity::from_metadata(&fs::metadata(&self.lease_path).with_context(|| {
                format!(
                    "reading driver lease path {} during revalidation",
                    self.lease_path.display()
                )
            })?);
        if pinned_lease != self.lease_identity || current_lease != self.lease_identity {
            bail!(
                "driver lease path was replaced while Grove held ownership: {}",
                self.lease_path.display()
            );
        }
        Ok(())
    }
}

fn acquire_lease_file(path: &Path, worktree_root: &Path) -> Result<(File, FileIdentity)> {
    acquire_lease_file_with_hook(path, worktree_root, |_, _| Ok(()))
}

fn acquire_lease_file_with_hook(
    path: &Path,
    worktree_root: &Path,
    mut after_lock: impl FnMut(usize, &Path) -> Result<()>,
) -> Result<(File, FileIdentity)> {
    for attempt in 1..=IDENTITY_RETRY_LIMIT {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .open(path)
            .with_context(|| format!("opening driver lease file {}", path.display()))?;
        ensure_close_on_exec(file.as_raw_fd())?;
        lock_exclusively_nonblocking(&file, worktree_root)?;
        after_lock(attempt, path).context("running driver lease post-lock check")?;

        let descriptor_identity = FileIdentity::from_metadata(
            &file
                .metadata()
                .context("reading locked driver lease identity")?,
        );
        let path_identity = FileIdentity::from_metadata(
            &fs::metadata(path)
                .with_context(|| format!("reading driver lease path {}", path.display()))?,
        );
        if descriptor_identity == path_identity {
            return Ok((file, descriptor_identity));
        }
        if attempt == IDENTITY_RETRY_LIMIT {
            bail!(
                "driver lease path {} was replaced during acquisition {} times",
                path.display(),
                IDENTITY_RETRY_LIMIT
            );
        }
    }
    bail!("driver lease acquisition exhausted its bounded retries")
}

fn lock_exclusively_nonblocking(file: &File, worktree_root: &Path) -> Result<()> {
    let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if result == 0 {
        return Ok(());
    }
    let error = std::io::Error::last_os_error();
    if matches!(
        error.raw_os_error(),
        Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
    ) {
        bail!(
            "another Grove driver already owns {}; the existing Grove driver must stop before this one can start",
            worktree_root.display()
        );
    }
    Err(error).with_context(|| format!("locking driver lease for {}", worktree_root.display()))
}

fn ensure_close_on_exec(descriptor: RawFd) -> Result<()> {
    let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFD) };
    if flags == -1 {
        return Err(std::io::Error::last_os_error()).context("reading descriptor flags");
    }
    if flags & libc::FD_CLOEXEC == 0 {
        let result = unsafe { libc::fcntl(descriptor, libc::F_SETFD, flags | libc::FD_CLOEXEC) };
        if result == -1 {
            return Err(std::io::Error::last_os_error())
                .context("marking driver descriptor close-on-exec");
        }
    }
    Ok(())
}

fn process_nonce() -> Result<[u8; 16]> {
    let mut source = File::open("/dev/urandom").context("opening OS randomness source")?;
    let mut nonce = [0_u8; 16];
    source
        .read_exact(&mut nonce)
        .context("reading 128-bit driver nonce from OS randomness source")?;
    Ok(nonce)
}

fn hex_nonce(nonce: [u8; 16]) -> Result<String> {
    let mut rendered = String::with_capacity(32);
    for byte in nonce {
        write!(&mut rendered, "{byte:02x}").context("rendering 128-bit driver nonce")?;
    }
    Ok(rendered)
}

fn write_record(file: &mut File, worktree_identity: FileIdentity, nonce: &str) -> Result<()> {
    file.set_len(0)
        .context("truncating previous driver lease record")?;
    file.seek(SeekFrom::Start(0))
        .context("rewinding driver lease record")?;
    write!(
        file,
        "worktree-device={}\nworktree-inode={}\nnonce={}\n",
        worktree_identity.device, worktree_identity.inode, nonce
    )
    .context("writing driver lease record")?;
    file.flush().context("flushing driver lease record")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn replace_locked_path(attempt: usize, path: &Path) -> Result<()> {
        fs::rename(path, path.with_extension(format!("attempt-{attempt}")))?;
        fs::write(path, format!("replacement {attempt}"))?;
        Ok(())
    }

    #[test]
    fn lease_path_replacement_retries_until_the_locked_descriptor_is_current() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        let control = root.join(".git/grove");
        fs::create_dir_all(&control).unwrap();
        let path = control.join(LEASE_FILE_NAME);
        let mut observed_attempts = 0;

        let (_file, identity) = acquire_lease_file_with_hook(&path, &root, |attempt, path| {
            observed_attempts = attempt;
            if attempt < 3 {
                replace_locked_path(attempt, path)?;
            }
            Ok(())
        })
        .unwrap();

        assert_eq!(observed_attempts, 3);
        assert_eq!(
            identity,
            FileIdentity::from_metadata(&fs::metadata(&path).unwrap())
        );
    }

    #[test]
    fn lease_path_replacement_fails_closed_after_eight_attempts() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        let control = root.join(".git/grove");
        fs::create_dir_all(&control).unwrap();
        let path = control.join(LEASE_FILE_NAME);
        let mut observed_attempts = 0;

        let error = acquire_lease_file_with_hook(&path, &root, |attempt, path| {
            observed_attempts = attempt;
            replace_locked_path(attempt, path)
        })
        .unwrap_err();

        assert_eq!(observed_attempts, IDENTITY_RETRY_LIMIT);
        assert!(
            error
                .to_string()
                .contains("was replaced during acquisition 8 times"),
            "unexpected error: {error:#}"
        );
    }
}
