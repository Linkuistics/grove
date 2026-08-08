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
const SIGNAL_FILE_PREFIX: &str = "signal-";
const SIGNAL_DRAW_RETRY_LIMIT: usize = 8;

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
    control_dir: PathBuf,
    _worktree_directory: File,
    worktree_identity: FileIdentity,
    lease_path: PathBuf,
    _lease_file: File,
    lease_identity: FileIdentity,
}

#[derive(Debug)]
pub(crate) struct SignalChannel {
    path: PathBuf,
}

impl SignalChannel {
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
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
        let control_dir = control.control_dir().to_path_buf();
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
        let nonce = random_nonce()?;
        write_record(&mut lease_file, worktree_identity, &hex_nonce(nonce)?)?;

        let lease = Self {
            worktree_root,
            control_dir,
            _worktree_directory: worktree_directory,
            worktree_identity,
            lease_path,
            _lease_file: lease_file,
            lease_identity,
        };
        lease.revalidate()?;
        Ok(lease)
    }

    pub fn worktree_root(&self) -> &Path {
        &self.worktree_root
    }

    pub(crate) fn allocate_signal_channel(&self) -> Result<SignalChannel> {
        allocate_signal_channel_with(&self.control_dir, random_nonce, |path| {
            match fs::symlink_metadata(path) {
                Ok(_) => Ok(true),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
                Err(error) => {
                    Err(error).with_context(|| format!("checking signal path {}", path.display()))
                }
            }
        })
    }

    pub(crate) fn remove_signal_channel(&self, channel: SignalChannel) -> Result<()> {
        if channel.path.parent() != Some(self.control_dir.as_path()) {
            bail!(
                "refusing to remove signal channel outside this driver lease's control directory: {}",
                channel.path.display()
            );
        }
        remove_file_if_present(&channel.path)
            .with_context(|| format!("removing signal channel {}", channel.path.display()))
    }

    /// Remove channels abandoned by a previous driver only after this lease
    /// owns the workspace. This stays separate from acquisition so the epoch
    /// layer can sequence it after installing the replacement driver's
    /// inactive record.
    pub(crate) fn cleanup_abandoned_signal_channels(&self) -> Result<()> {
        cleanup_abandoned_signal_channels_with(
            &self.control_dir,
            |control_dir| {
                fs::read_dir(control_dir)
                    .with_context(|| {
                        format!(
                            "listing abandoned signal channels in {}",
                            control_dir.display()
                        )
                    })?
                    .map(|entry| {
                        entry
                            .map(|entry| entry.path())
                            .context("reading a Grove control-directory entry")
                    })
                    .collect()
            },
            remove_file_if_present,
        )
    }

    /// Confirm that the paths still name the descriptors this process owns.
    pub fn revalidate(&self) -> Result<()> {
        let current_root =
            FileIdentity::from_metadata(&fs::metadata(&self.worktree_root).with_context(|| {
                format!(
                    "reading working tree root {} during driver lease revalidation",
                    self.worktree_root.display()
                )
            })?);
        if current_root != self.worktree_identity {
            bail!(
                "working tree root was replaced while Grove held its driver lease: {}",
                self.worktree_root.display()
            );
        }

        let current_lease =
            FileIdentity::from_metadata(&fs::metadata(&self.lease_path).with_context(|| {
                format!(
                    "reading driver lease path {} during revalidation",
                    self.lease_path.display()
                )
            })?);
        if current_lease != self.lease_identity {
            bail!(
                "driver lease path was replaced while Grove held ownership: {}",
                self.lease_path.display()
            );
        }
        Ok(())
    }
}

fn allocate_signal_channel_with(
    control_dir: &Path,
    mut draw_nonce: impl FnMut() -> Result<[u8; 16]>,
    mut path_exists: impl FnMut(&Path) -> Result<bool>,
) -> Result<SignalChannel> {
    for _ in 0..SIGNAL_DRAW_RETRY_LIMIT {
        let path = control_dir.join(format!("{SIGNAL_FILE_PREFIX}{}", hex_nonce(draw_nonce()?)?));
        if !path_exists(&path)? {
            return Ok(SignalChannel { path });
        }
    }
    bail!(
        "could not allocate a fresh signal path after {} occupied random draws in {}",
        SIGNAL_DRAW_RETRY_LIMIT,
        control_dir.display()
    )
}

fn cleanup_abandoned_signal_channels_with(
    control_dir: &Path,
    mut list_paths: impl FnMut(&Path) -> Result<Vec<PathBuf>>,
    mut remove_file: impl FnMut(&Path) -> Result<()>,
) -> Result<()> {
    for path in list_paths(control_dir)? {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if is_signal_channel_name(name) {
            remove_file(&path)
                .with_context(|| format!("removing abandoned signal channel {}", path.display()))?;
        }
    }
    Ok(())
}

fn is_signal_channel_name(name: &str) -> bool {
    let Some(suffix) = name.strip_prefix(SIGNAL_FILE_PREFIX) else {
        return false;
    };
    suffix.len() == 32
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn remove_file_if_present(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("removing {}", path.display())),
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

fn random_nonce() -> Result<[u8; 16]> {
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
    use std::collections::VecDeque;
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

    #[test]
    fn acquired_driver_descriptors_are_close_on_exec() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();

        let lease = DriverLease::acquire(&root).unwrap();

        for (label, descriptor) in [
            ("working-tree root", lease._worktree_directory.as_raw_fd()),
            ("driver lease", lease._lease_file.as_raw_fd()),
        ] {
            let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFD) };
            assert_ne!(flags, -1, "reading {label} descriptor flags failed");
            assert_ne!(
                flags & libc::FD_CLOEXEC,
                0,
                "{label} descriptor can leak across exec"
            );
        }
    }

    #[test]
    fn an_occupied_signal_draw_is_retried_without_touching_the_old_channel() {
        let tmp = TempDir::new().unwrap();
        let control_dir = tmp.path().join("control");
        fs::create_dir_all(&control_dir).unwrap();
        let occupied_nonce = [0x11; 16];
        let fresh_nonce = [0x22; 16];
        let occupied_path = control_dir.join(format!(
            "{SIGNAL_FILE_PREFIX}{}",
            hex_nonce(occupied_nonce).unwrap()
        ));
        fs::write(&occupied_path, "old completion\n").unwrap();
        let mut draws = VecDeque::from([occupied_nonce, fresh_nonce]);

        let channel = allocate_signal_channel_with(
            &control_dir,
            || Ok(draws.pop_front().unwrap()),
            |path| Ok(fs::symlink_metadata(path).is_ok()),
        )
        .unwrap();

        assert_eq!(
            channel.path(),
            control_dir
                .join(format!(
                    "{SIGNAL_FILE_PREFIX}{}",
                    hex_nonce(fresh_nonce).unwrap()
                ))
                .as_path()
        );
        assert_eq!(
            fs::read_to_string(occupied_path).unwrap(),
            "old completion\n"
        );
        assert!(
            draws.is_empty(),
            "the allocator must consume exactly two draws"
        );
    }

    #[test]
    fn abandoned_cleanup_uses_the_injected_filesystem_and_exact_channel_grammar() {
        let control_dir = PathBuf::from("/workspace-control");
        let valid = control_dir.join("signal-0123456789abcdef0123456789abcdef");
        let uppercase = control_dir.join("signal-0123456789ABCDEF0123456789ABCDEF");
        let malformed = control_dir.join("signal-not-a-channel");
        let lease = control_dir.join(LEASE_FILE_NAME);
        let mut listed_directory = None;
        let mut removed = Vec::new();

        cleanup_abandoned_signal_channels_with(
            &control_dir,
            |directory| {
                listed_directory = Some(directory.to_path_buf());
                Ok(vec![
                    valid.clone(),
                    uppercase.clone(),
                    malformed.clone(),
                    lease.clone(),
                ])
            },
            |path| {
                removed.push(path.to_path_buf());
                Ok(())
            },
        )
        .unwrap();

        assert_eq!(listed_directory, Some(control_dir));
        assert_eq!(removed, vec![valid]);
    }
}
