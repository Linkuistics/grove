use crate::repo;
use anyhow::{bail, Context, Result};
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const LEASE_FILE_NAME: &str = "driver.lease";
const EPOCH_FILE_NAME: &str = "session.epoch";
const IDENTITY_RETRY_LIMIT: usize = 8;
const EPOCH_HANDOFF_TIMEOUT: Duration = Duration::from_secs(30);
const EPOCH_WAIT_INTERVAL: Duration = Duration::from_millis(10);
const SIGNAL_FILE_PREFIX: &str = "signal-";
pub(crate) const RANDOM_DRAW_RETRY_LIMIT: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FileIdentity {
    device: u64,
    inode: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ProcessRecord {
    worktree_identity: FileIdentity,
    worktree_root: PathBuf,
    nonce: String,
}

#[derive(Debug, PartialEq, Eq)]
struct EpochRecord {
    process: ProcessRecord,
    signal_path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LockMode {
    Shared,
    Exclusive,
}

impl LockMode {
    fn operation(self) -> libc::c_int {
        match self {
            Self::Shared => libc::LOCK_SH,
            Self::Exclusive => libc::LOCK_EX,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::Exclusive => "exclusive",
        }
    }
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
    lease_file: File,
    lease_identity: FileIdentity,
    nonce: String,
}

#[derive(Debug)]
pub(crate) struct SignalChannel {
    path: PathBuf,
}

#[derive(Debug)]
pub(crate) struct SessionEpochGuard {
    _epoch_file: File,
    signal_path: PathBuf,
}

impl SessionEpochGuard {
    pub(crate) fn require_signal_path(&self, signal_path: Option<&Path>) -> Result<()> {
        if signal_path != Some(self.signal_path.as_path()) {
            bail!(
                "completion signal path does not match the admitted session epoch: expected {}, got {}",
                self.signal_path.display(),
                signal_path
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "<none>".to_string())
            );
        }
        Ok(())
    }
}

impl SignalChannel {
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

impl DriverLease {
    pub fn acquire(path: &Path) -> Result<Self> {
        Self::acquire_with(path, || {})
    }

    fn acquire_with(path: &Path, before_initial_epoch_handoff: impl FnOnce()) -> Result<Self> {
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
        let (lease_file, lease_identity) = acquire_lease_file(&lease_path, &worktree_root)?;
        let nonce = hex_nonce(random_nonce()?)?;

        let mut lease = Self {
            worktree_root,
            control_dir,
            _worktree_directory: worktree_directory,
            worktree_identity,
            lease_path,
            lease_file,
            lease_identity,
            nonce,
        };
        lease.revalidate()?;
        before_initial_epoch_handoff();
        lease.initialize_epoch_record()?;
        if let Err(error) = lease.cleanup_abandoned_signal_channels() {
            eprintln!(
                "grove: warning: could not clean every signal channel abandoned by a previous driver; continuing because fresh channel allocation does not depend on an empty control directory: {error:#}"
            );
        }
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
        remove_file_if_present(&channel.path)
            .with_context(|| format!("removing signal channel {}", channel.path.display()))
    }

    pub(crate) fn activate_session_epoch(&self, signal_path: &Path) -> Result<()> {
        self.write_epoch_record(Some(signal_path), "pre-spawn activation")
    }

    pub(crate) fn invalidate_session_epoch(&self) -> Result<()> {
        self.write_epoch_record(None, "post-reap invalidation")
    }

    /// Remove channels abandoned by a previous driver only after this lease
    /// owns the workspace. This stays separate from acquisition so the epoch
    /// layer can sequence it after installing the replacement driver's
    /// inactive record.
    pub(crate) fn cleanup_abandoned_signal_channels(&self) -> Result<()> {
        cleanup_abandoned_signal_channels_with(
            &self.control_dir,
            |control_dir| {
                let entries = fs::read_dir(control_dir).with_context(|| {
                    format!(
                        "listing abandoned signal channels in {}",
                        control_dir.display()
                    )
                })?;
                Ok(entries
                    .map(|entry| {
                        entry
                            .map(|entry| entry.path())
                            .context("reading a Grove control-directory entry")
                    })
                    .collect())
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

    fn write_epoch_record(&self, signal_path: Option<&Path>, operation: &str) -> Result<()> {
        let path = self.control_dir.join(EPOCH_FILE_NAME);
        let mut file = acquire_epoch_file(&path, LockMode::Exclusive, operation)?;
        write_epoch_contents(
            &mut file,
            self.worktree_identity,
            &self.worktree_root,
            &self.nonce,
            signal_path,
        )
    }

    fn initialize_epoch_record(&mut self) -> Result<()> {
        let path = self.control_dir.join(EPOCH_FILE_NAME);
        let mut epoch_file = acquire_epoch_file(&path, LockMode::Exclusive, "driver acquisition")?;
        // Keep the predecessor's lease bytes intact until exclusive epoch
        // handoff succeeds. An operation that already holds shared admission
        // may still probe those bytes while this replacement waits; publishing
        // the new nonce early would reject an operation the old epoch admitted.
        write_epoch_contents(
            &mut epoch_file,
            self.worktree_identity,
            &self.worktree_root,
            &self.nonce,
            None,
        )?;
        write_record(
            &mut self.lease_file,
            self.worktree_identity,
            &self.worktree_root,
            &self.nonce,
        )
    }
}

fn write_epoch_contents(
    file: &mut File,
    worktree_identity: FileIdentity,
    worktree_root: &Path,
    nonce: &str,
    signal_path: Option<&Path>,
) -> Result<()> {
    file.set_len(0)
        .context("truncating previous session epoch record")?;
    file.seek(SeekFrom::Start(0))
        .context("rewinding session epoch record")?;
    writeln!(
        file,
        "state={}",
        if signal_path.is_some() {
            "active"
        } else {
            "inactive"
        }
    )?;
    writeln!(file, "worktree-device={}", worktree_identity.device)?;
    writeln!(file, "worktree-inode={}", worktree_identity.inode)?;
    writeln!(file, "worktree-path-hex={}", encode_path(worktree_root)?)?;
    writeln!(file, "nonce={nonce}")?;
    if let Some(signal_path) = signal_path {
        writeln!(file, "signal-path-hex={}", encode_path(signal_path)?)?;
    }
    file.flush().context("flushing session epoch record")
}

fn allocate_signal_channel_with(
    control_dir: &Path,
    mut draw_nonce: impl FnMut() -> Result<[u8; 16]>,
    mut path_exists: impl FnMut(&Path) -> Result<bool>,
) -> Result<SignalChannel> {
    for _ in 0..RANDOM_DRAW_RETRY_LIMIT {
        let path = control_dir.join(format!("{SIGNAL_FILE_PREFIX}{}", hex_nonce(draw_nonce()?)?));
        if !path_exists(&path)? {
            return Ok(SignalChannel { path });
        }
    }
    bail!(
        "could not allocate a fresh signal path after {} occupied random draws in {}",
        RANDOM_DRAW_RETRY_LIMIT,
        control_dir.display()
    )
}

fn cleanup_abandoned_signal_channels_with(
    control_dir: &Path,
    mut list_paths: impl FnMut(&Path) -> Result<Vec<Result<PathBuf>>>,
    mut remove_file: impl FnMut(&Path) -> Result<()>,
) -> Result<()> {
    let mut failures = Vec::new();
    for entry in list_paths(control_dir)? {
        let path = match entry {
            Ok(path) => path,
            Err(error) => {
                failures.push(format!("{error:#}"));
                continue;
            }
        };
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if is_signal_channel_name(name) {
            if let Err(error) = remove_file(&path)
                .with_context(|| format!("removing abandoned signal channel {}", path.display()))
            {
                failures.push(format!("{error:#}"));
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        bail!(
            "encountered {} abandoned signal cleanup failure(s):\n- {}",
            failures.len(),
            failures.join("\n- ")
        )
    }
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

fn acquire_epoch_file(path: &Path, mode: LockMode, operation: &str) -> Result<File> {
    acquire_epoch_file_with(
        path,
        mode,
        operation,
        EPOCH_HANDOFF_TIMEOUT,
        Instant::now,
        || std::thread::sleep(EPOCH_WAIT_INTERVAL),
        |_, _| Ok(()),
        |_, _| Ok(()),
        || {
            eprintln!("{}", epoch_contention_diagnostic(mode, operation));
        },
    )
}

fn epoch_contention_diagnostic(mode: LockMode, operation: &str) -> String {
    format!(
        "waiting for {} session epoch lock for {operation}",
        mode.label()
    )
}

#[allow(clippy::too_many_arguments)]
fn acquire_epoch_file_with(
    path: &Path,
    mode: LockMode,
    operation: &str,
    timeout: Duration,
    mut now: impl FnMut() -> Instant,
    mut wait: impl FnMut(),
    mut after_open: impl FnMut(usize, &Path) -> Result<()>,
    mut after_lock: impl FnMut(usize, &Path) -> Result<()>,
    mut report_contention: impl FnMut(),
) -> Result<File> {
    let deadline = now()
        .checked_add(timeout)
        .context("computing session epoch handoff deadline")?;
    let mut reported_contention = false;

    for attempt in 1..=IDENTITY_RETRY_LIMIT {
        let mut options = OpenOptions::new();
        options.read(true);
        if mode == LockMode::Exclusive {
            options.write(true).create(true).mode(0o600);
        }
        let file = options
            .open(path)
            .with_context(|| format!("opening session epoch file {}", path.display()))?;
        ensure_close_on_exec(file.as_raw_fd())?;
        after_open(attempt, path).context("running session epoch post-open barrier")?;

        loop {
            let result = unsafe { libc::flock(file.as_raw_fd(), mode.operation() | libc::LOCK_NB) };
            if result == 0 {
                break;
            }
            let error = std::io::Error::last_os_error();
            if !matches!(
                error.raw_os_error(),
                Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
            ) {
                return Err(error).with_context(|| {
                    format!("locking {} session epoch for {operation}", mode.label())
                });
            }
            if !reported_contention {
                report_contention();
                reported_contention = true;
            }
            if now() >= deadline {
                bail!(
                    "timed out after {}s waiting for {} session epoch lock for {operation}",
                    timeout.as_secs(),
                    mode.label()
                );
            }
            wait();
        }

        after_lock(attempt, path).context("running session epoch post-lock barrier")?;
        let descriptor_identity = FileIdentity::from_metadata(
            &file
                .metadata()
                .context("reading locked session epoch identity")?,
        );
        let path_identity = FileIdentity::from_metadata(
            &fs::metadata(path)
                .with_context(|| format!("reading session epoch path {}", path.display()))?,
        );
        if descriptor_identity == path_identity {
            return Ok(file);
        }
        if attempt == IDENTITY_RETRY_LIMIT {
            bail!(
                "session epoch path {} was replaced during acquisition {} times",
                path.display(),
                IDENTITY_RETRY_LIMIT
            );
        }
    }
    bail!("session epoch acquisition exhausted its bounded retries")
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

pub(crate) fn fresh_nonce() -> Result<String> {
    hex_nonce(random_nonce()?)
}

fn encode_path(path: &Path) -> Result<String> {
    let mut rendered = String::with_capacity(path.as_os_str().as_bytes().len() * 2);
    for byte in path.as_os_str().as_bytes() {
        write!(&mut rendered, "{byte:02x}").context("encoding working-tree path")?;
    }
    Ok(rendered)
}

fn decode_path(value: &str) -> Result<PathBuf> {
    if value.len() % 2 != 0 {
        bail!("working-tree path hex has odd length");
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for offset in (0..value.len()).step_by(2) {
        let byte = u8::from_str_radix(&value[offset..offset + 2], 16)
            .context("decoding working-tree path hex")?;
        bytes.push(byte);
    }
    Ok(PathBuf::from(std::ffi::OsString::from_vec(bytes)))
}

fn record_field<'a>(record: &'a str, name: &str) -> Result<&'a str> {
    let prefix = format!("{name}=");
    let mut values = record.lines().filter_map(|line| line.strip_prefix(&prefix));
    let value = values
        .next()
        .with_context(|| format!("missing {name} field"))?;
    if values.next().is_some() {
        bail!("duplicate {name} field");
    }
    Ok(value)
}

fn parse_process_record(record: &str) -> Result<ProcessRecord> {
    let device = record_field(record, "worktree-device")?
        .parse::<u64>()
        .context("parsing working-tree device")?;
    let inode = record_field(record, "worktree-inode")?
        .parse::<u64>()
        .context("parsing working-tree inode")?;
    let worktree_root = decode_path(record_field(record, "worktree-path-hex")?)?;
    let nonce = record_field(record, "nonce")?;
    if nonce.len() != 32
        || !nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        bail!("driver nonce is not 128-bit lowercase hex");
    }
    Ok(ProcessRecord {
        worktree_identity: FileIdentity { device, inode },
        worktree_root,
        nonce: nonce.to_string(),
    })
}

fn read_record(file: &mut File, label: &str) -> Result<String> {
    file.seek(SeekFrom::Start(0))
        .with_context(|| format!("rewinding {label}"))?;
    let mut record = String::new();
    file.read_to_string(&mut record)
        .with_context(|| format!("reading {label}"))?;
    Ok(record)
}

fn read_epoch_record(file: &mut File) -> Result<EpochRecord> {
    let record = read_record(file, "session epoch record")?;
    let process = parse_process_record(&record)?;
    let signal_path = match record_field(&record, "state")? {
        "inactive" => {
            if record
                .lines()
                .any(|line| line.starts_with("signal-path-hex="))
            {
                bail!("inactive session epoch unexpectedly carries a signal path");
            }
            None
        }
        "active" => Some(decode_path(record_field(&record, "signal-path-hex")?)?),
        state => bail!("unknown session epoch state {state:?}"),
    };
    Ok(EpochRecord {
        process,
        signal_path,
    })
}

fn probe_live_lease(control_dir: &Path, epoch: &EpochRecord, operation: &str) -> Result<()> {
    probe_live_lease_with_post_unlock_hook(control_dir, epoch, operation, |_| Ok(()))
}

fn probe_live_lease_with_post_unlock_hook(
    control_dir: &Path,
    epoch: &EpochRecord,
    operation: &str,
    mut after_successful_probe: impl FnMut(&Path) -> Result<()>,
) -> Result<()> {
    let lease_path = control_dir.join(LEASE_FILE_NAME);
    for attempt in 1..=IDENTITY_RETRY_LIMIT {
        let mut lease_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&lease_path)
            .with_context(|| format!("opening driver lease file {}", lease_path.display()))?;
        ensure_close_on_exec(lease_file.as_raw_fd())?;
        let lease_record =
            parse_process_record(&read_record(&mut lease_file, "driver lease record")?)?;
        let probe = unsafe { libc::flock(lease_file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
        let probe_error = (probe != 0).then(std::io::Error::last_os_error);
        if probe == 0 {
            let unlock = unsafe { libc::flock(lease_file.as_raw_fd(), libc::LOCK_UN) };
            if unlock != 0 {
                return Err(std::io::Error::last_os_error())
                    .context("releasing successful driver lease liveness probe");
            }
            after_successful_probe(&lease_path)
                .context("running driver lease post-unlock check")?;
        }
        let descriptor_identity = FileIdentity::from_metadata(
            &lease_file
                .metadata()
                .context("reading probed driver lease identity")?,
        );
        let path_identity = FileIdentity::from_metadata(
            &fs::metadata(&lease_path)
                .with_context(|| format!("reading driver lease path {}", lease_path.display()))?,
        );
        if descriptor_identity != path_identity {
            if attempt == IDENTITY_RETRY_LIMIT {
                bail!(
                    "driver lease path {} was replaced during liveness probe {} times",
                    lease_path.display(),
                    IDENTITY_RETRY_LIMIT
                );
            }
            continue;
        }
        if lease_record != epoch.process {
            bail!("driver lease record does not match the admitted session epoch");
        }
        if probe == 0 {
            bail!("driver lease is unlocked");
        }
        let error = probe_error.expect("failed flock records an OS error");
        if matches!(
            error.raw_os_error(),
            Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
        ) {
            return Ok(());
        }
        return Err(error).with_context(|| format!("probing driver lease for {operation}"));
    }
    bail!("driver lease liveness probe exhausted its bounded retries")
}

/// Admit one agent-side operation when it carries a live loop-control context.
///
/// The returned guard owns the shared epoch lock and must remain alive through
/// the operation's separately acquired Tree access guard. With no ambient
/// signal path this is a manual command and no driver epoch is required.
pub(crate) fn admit_ambient_session(
    path: &Path,
    operation: &str,
) -> Result<Option<SessionEpochGuard>> {
    let Some(signal_path) = std::env::var_os("GROVE_SIGNAL_FILE")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
    else {
        return Ok(None);
    };
    let current_control = repo::workspace_control(path)?;
    let control_dir = signal_path.parent().with_context(|| {
        format!(
            "stale Grove session for {operation}: signal path has no control-directory parent: {}",
            signal_path.display()
        )
    })?;
    let epoch_path = control_dir.join(EPOCH_FILE_NAME);
    let mut epoch_file = acquire_epoch_file(&epoch_path, LockMode::Shared, operation)
        .with_context(|| format!("stale Grove session for {operation}"))?;
    let epoch = read_epoch_record(&mut epoch_file)
        .with_context(|| format!("stale Grove session for {operation}"))?;

    if epoch.process.worktree_root != current_control.worktree_root() {
        bail!(
            "wrong working tree for {operation}: session belongs to {}, command resolved {}",
            epoch.process.worktree_root.display(),
            current_control.worktree_root().display()
        );
    }
    let current_identity = FileIdentity::from_metadata(
        &fs::metadata(current_control.worktree_root()).with_context(|| {
            format!(
                "reading current working-tree identity {}",
                current_control.worktree_root().display()
            )
        })?,
    );
    if epoch.process.worktree_identity != current_identity {
        bail!("stale Grove session for {operation}: working-tree identity changed");
    }
    let Some(epoch_signal_path) = epoch.signal_path.as_deref() else {
        bail!("stale Grove session for {operation}: session epoch is inactive");
    };
    if epoch_signal_path != signal_path {
        bail!(
            "stale Grove session for {operation}: loop-control path does not match the active epoch"
        );
    }
    probe_live_lease(control_dir, &epoch, operation)
        .with_context(|| format!("stale Grove session for {operation}"))?;
    Ok(Some(SessionEpochGuard {
        _epoch_file: epoch_file,
        signal_path,
    }))
}

fn write_record(
    file: &mut File,
    worktree_identity: FileIdentity,
    worktree_root: &Path,
    nonce: &str,
) -> Result<()> {
    file.set_len(0)
        .context("truncating previous driver lease record")?;
    file.seek(SeekFrom::Start(0))
        .context("rewinding driver lease record")?;
    write!(
        file,
        "worktree-device={}\nworktree-inode={}\nworktree-path-hex={}\nnonce={}\n",
        worktree_identity.device,
        worktree_identity.inode,
        encode_path(worktree_root)?,
        nonce
    )
    .context("writing driver lease record")?;
    file.flush().context("flushing driver lease record")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::ffi::OsString;
    use std::process::Command;
    use std::sync::{mpsc, Mutex, MutexGuard};
    use std::thread;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    static ENV_LOCK: Mutex<()> = Mutex::new(());
    const FORK_SENSITIVE_TEST: &str = "GROVE_DRIVER_LEASE_FORK_SENSITIVE_TEST";

    struct SignalEnvironment {
        previous: Option<OsString>,
    }

    impl SignalEnvironment {
        fn set(path: Option<&Path>) -> (MutexGuard<'static, ()>, Self) {
            let lock = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
            let previous = std::env::var_os("GROVE_SIGNAL_FILE");
            match path {
                Some(path) => std::env::set_var("GROVE_SIGNAL_FILE", path),
                None => std::env::remove_var("GROVE_SIGNAL_FILE"),
            }
            (lock, Self { previous })
        }
    }

    impl Drop for SignalEnvironment {
        fn drop(&mut self) {
            match self.previous.take() {
                Some(previous) => std::env::set_var("GROVE_SIGNAL_FILE", previous),
                None => std::env::remove_var("GROVE_SIGNAL_FILE"),
            }
        }
    }

    fn fork_sensitive_driver_lease_test_body_runs_here() -> bool {
        let current_thread = thread::current();
        let test_name = current_thread
            .name()
            .expect("the Rust test harness names every test thread");
        let arguments: Vec<_> = std::env::args_os().collect();
        let is_isolated_child = std::env::var_os(FORK_SENSITIVE_TEST)
            == Some(OsString::from(test_name))
            && arguments
                .windows(2)
                .any(|pair| pair[0] == "--exact" && pair[1] == test_name);
        if is_isolated_child {
            return true;
        }

        // flock locks survive fork until the child execs. A parallel unit test
        // that launches a subprocess can therefore extend this test's lease
        // briefly after its owner drops, even though every descriptor is
        // close-on-exec. Re-run only the fork-sensitive assertion in a child
        // test process with no parallel siblings; the production ordering and
        // lock assertions remain unchanged inside that process.
        let output = Command::new(std::env::current_exe().expect("locating the unit-test binary"))
            .args(["--exact", test_name, "--nocapture"])
            .env(FORK_SENSITIVE_TEST, test_name)
            .output()
            .expect("launching the isolated driver-lease test");
        assert!(
            output.status.success(),
            "isolated driver-lease test {test_name} failed with {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        false
    }

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
            ("driver lease", lease.lease_file.as_raw_fd()),
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
    fn signal_allocation_refuses_after_eight_occupied_draws() {
        let control_dir = PathBuf::from("/workspace-control");
        let occupied_nonce = [0x11; 16];
        let mut draws = 0;

        let error = allocate_signal_channel_with(
            &control_dir,
            || {
                draws += 1;
                Ok(occupied_nonce)
            },
            |_| Ok(true),
        )
        .unwrap_err();

        assert_eq!(draws, 8, "the allocator must stop at its retry bound");
        assert_eq!(
            error.to_string(),
            "could not allocate a fresh signal path after 8 occupied random draws in /workspace-control"
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
                    Ok(valid.clone()),
                    Ok(uppercase.clone()),
                    Ok(malformed.clone()),
                    Ok(lease.clone()),
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

    #[test]
    fn abandoned_cleanup_attempts_every_channel_when_one_removal_fails() {
        let control_dir = PathBuf::from("/workspace-control");
        let blocked = control_dir.join("signal-11111111111111111111111111111111");
        let removable = control_dir.join("signal-22222222222222222222222222222222");
        let mut attempted = Vec::new();

        let error = cleanup_abandoned_signal_channels_with(
            &control_dir,
            |_| Ok(vec![Ok(blocked.clone()), Ok(removable.clone())]),
            |path| {
                attempted.push(path.to_path_buf());
                if path == blocked {
                    anyhow::bail!("simulated refusal")
                }
                Ok(())
            },
        )
        .unwrap_err();

        assert_eq!(attempted, vec![blocked.clone(), removable]);
        assert!(
            format!("{error:#}").contains(&blocked.display().to_string()),
            "the aggregate warning must identify the channel that could not be removed: {error:#}"
        );
    }

    #[test]
    fn abandoned_cleanup_keeps_recoverable_entries_after_one_enumeration_error() {
        let control_dir = PathBuf::from("/workspace-control");
        let first = control_dir.join("signal-11111111111111111111111111111111");
        let later = control_dir.join("signal-22222222222222222222222222222222");
        let mut removed = Vec::new();

        let error = cleanup_abandoned_signal_channels_with(
            &control_dir,
            |_| {
                Ok(vec![
                    Ok(first.clone()),
                    Err(anyhow::anyhow!("simulated unreadable directory entry")),
                    Ok(later.clone()),
                ])
            },
            |path| {
                removed.push(path.to_path_buf());
                Ok(())
            },
        )
        .unwrap_err();

        assert_eq!(removed, vec![first, later]);
        assert!(
            format!("{error:#}").contains("simulated unreadable directory entry"),
            "the aggregate warning must preserve enumeration failures: {error:#}"
        );
    }

    #[test]
    fn activation_and_invalidation_replace_one_stable_epoch_record() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&root).unwrap();
        let epoch_path = root.join(".git/grove").join(EPOCH_FILE_NAME);
        let epoch_identity = FileIdentity::from_metadata(&fs::metadata(&epoch_path).unwrap());
        let signal_path = root.join(".git/grove/signal-11111111111111111111111111111111");

        lease.activate_session_epoch(&signal_path).unwrap();

        let active = fs::read_to_string(&epoch_path).unwrap();
        assert!(active.starts_with("state=active\n"), "{active:?}");
        assert!(
            active.contains(&format!(
                "signal-path-hex={}\n",
                encode_path(&signal_path).unwrap()
            )),
            "{active:?}"
        );
        assert_eq!(
            FileIdentity::from_metadata(&fs::metadata(&epoch_path).unwrap()),
            epoch_identity,
            "activation must rewrite the stable epoch file rather than replace it"
        );

        lease.invalidate_session_epoch().unwrap();

        let inactive = fs::read_to_string(&epoch_path).unwrap();
        assert!(inactive.starts_with("state=inactive\n"), "{inactive:?}");
        assert!(!inactive.contains("signal-path-hex="), "{inactive:?}");
        assert_eq!(
            FileIdentity::from_metadata(&fs::metadata(&epoch_path).unwrap()),
            epoch_identity,
            "invalidation must keep the same stable epoch file"
        );
    }

    #[test]
    fn epoch_acquisition_retries_open_lock_path_replacement_in_event_order() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join(EPOCH_FILE_NAME);
        fs::write(&path, "old epoch\n").unwrap();
        let events = RefCell::new(Vec::new());
        let start = Instant::now();

        let guard = acquire_epoch_file_with(
            &path,
            LockMode::Exclusive,
            "test replacement race",
            Duration::from_secs(30),
            || start,
            || {},
            |attempt, path| {
                events.borrow_mut().push(format!("open-{attempt}"));
                if attempt == 1 {
                    fs::rename(path, path.with_extension("old"))?;
                    fs::write(path, "replacement epoch\n")?;
                }
                Ok(())
            },
            |attempt, _| {
                events.borrow_mut().push(format!("lock-{attempt}"));
                Ok(())
            },
            || events.borrow_mut().push("contended".to_string()),
        )
        .unwrap();

        assert_eq!(
            events.into_inner(),
            ["open-1", "lock-1", "open-2", "lock-2"]
        );
        assert_eq!(
            FileIdentity::from_metadata(&guard.metadata().unwrap()),
            FileIdentity::from_metadata(&fs::metadata(&path).unwrap())
        );
    }

    #[test]
    fn an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join(EPOCH_FILE_NAME);
        fs::write(&path, "active epoch\n").unwrap();
        let owner = File::open(&path).unwrap();
        assert_eq!(unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_EX) }, 0);
        let elapsed = Cell::new(Duration::ZERO);
        let contention_reports = Cell::new(0);
        let start = Instant::now();

        let error = acquire_epoch_file_with(
            &path,
            LockMode::Exclusive,
            "post-reap session epoch invalidation",
            Duration::from_secs(30),
            || start + elapsed.get(),
            || elapsed.set(elapsed.get() + Duration::from_secs(10)),
            |_, _| Ok(()),
            |_, _| Ok(()),
            || contention_reports.set(contention_reports.get() + 1),
        )
        .unwrap_err();

        assert_eq!(contention_reports.get(), 1);
        assert_eq!(elapsed.get(), Duration::from_secs(30));
        assert!(
            error.to_string().contains(
                "timed out after 30s waiting for exclusive session epoch lock for post-reap session epoch invalidation"
            ),
            "unexpected error: {error:#}"
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "active epoch\n",
            "a timed-out post-reap acquisition rewrote the epoch"
        );
    }

    #[test]
    fn the_epoch_contention_diagnostic_names_the_lock_mode_and_operation() {
        let diagnostic = epoch_contention_diagnostic(LockMode::Exclusive, "post-reap invalidation");

        assert!(diagnostic.contains("exclusive"), "{diagnostic}");
        assert!(
            diagnostic.contains("post-reap invalidation"),
            "{diagnostic}"
        );
    }

    #[test]
    fn manual_agent_operations_need_no_driver_epoch() {
        let (_lock, _environment) = SignalEnvironment::set(None);

        let admission =
            admit_ambient_session(Path::new("/not-a-working-tree"), "manual pick").unwrap();

        assert!(admission.is_none());
    }

    #[test]
    fn an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&root).unwrap();
        let signal_path = root.join(".git/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        let (_lock, _environment) = SignalEnvironment::set(Some(&signal_path));

        let admission = admit_ambient_session(&root, "test pick")
            .unwrap()
            .expect("ambient loop context must return a held admission guard");

        let epoch_path = root.join(".git/grove").join(EPOCH_FILE_NAME);
        let probe = File::open(&epoch_path).unwrap();
        assert_ne!(
            unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0,
            "exclusive invalidation overlapped the admitted ambient operation"
        );

        drop(probe);
        drop(lease);
        let (started_tx, started_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let replacement_root = root.clone();
        let replacement = thread::spawn(move || {
            let _ = started_tx.send(());
            let result = DriverLease::acquire(&replacement_root);
            if result_tx.send(result).is_err() {
                panic!("replacement result receiver disappeared");
            }
        });
        started_rx.recv().unwrap();
        assert!(
            matches!(
                result_rx.recv_timeout(Duration::from_millis(50)),
                Err(mpsc::RecvTimeoutError::Timeout)
            ),
            "replacement invalidated the epoch before the admitted operation returned"
        );

        drop(admission);
        let replacement_lease = result_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("replacement did not acquire after the ambient guard dropped")
            .unwrap();
        replacement.join().unwrap();

        let error = admit_ambient_session(&root, "test pick").unwrap_err();
        assert!(
            format!("{error:#}").contains("session epoch is inactive"),
            "a new call from the old session was not refused: {error:#}"
        );
        drop(replacement_lease);
    }

    #[test]
    fn replacement_keeps_the_old_lease_record_until_it_owns_epoch_handoff() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let old_lease = DriverLease::acquire(&root).unwrap();
        let signal_path = root.join(".git/grove/signal-11111111111111111111111111111111");
        old_lease.activate_session_epoch(&signal_path).unwrap();
        let lease_path = root.join(".git/grove").join(LEASE_FILE_NAME);
        let old_record = fs::read_to_string(&lease_path).unwrap();
        let epoch_guard = File::open(root.join(".git/grove").join(EPOCH_FILE_NAME)).unwrap();
        assert_eq!(
            unsafe { libc::flock(epoch_guard.as_raw_fd(), libc::LOCK_SH) },
            0
        );
        drop(old_lease);

        let (reached_tx, reached_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let replacement_root = root.clone();
        let replacement = thread::spawn(move || {
            DriverLease::acquire_with(&replacement_root, || {
                reached_tx.send(()).unwrap();
                release_rx.recv().unwrap();
            })
        });
        reached_rx.recv().unwrap();

        assert_eq!(
            fs::read_to_string(&lease_path).unwrap(),
            old_record,
            "replacement published its nonce before acquiring exclusive epoch handoff"
        );
        release_tx.send(()).unwrap();
        drop(epoch_guard);
        let replacement_lease = replacement.join().unwrap().unwrap();
        assert_ne!(fs::read_to_string(&lease_path).unwrap(), old_record);
        drop(replacement_lease);
    }

    #[test]
    fn ambient_context_from_another_worktree_names_both_roots() {
        let tmp = TempDir::new().unwrap();
        let owner_root = tmp.path().join("owner");
        let foreign_root = tmp.path().join("foreign");
        fs::create_dir_all(owner_root.join(".git")).unwrap();
        fs::create_dir_all(foreign_root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&owner_root).unwrap();
        let signal_path = owner_root.join(".git/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        let (_lock, _environment) = SignalEnvironment::set(Some(&signal_path));

        let error = admit_ambient_session(&foreign_root, "test pick").unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("wrong working tree"), "{message}");
        assert!(
            message.contains(owner_root.canonicalize().unwrap().to_str().unwrap()),
            "{message}"
        );
        assert!(
            message.contains(foreign_root.canonicalize().unwrap().to_str().unwrap()),
            "{message}"
        );
    }

    #[test]
    fn an_inactive_epoch_is_reported_without_claiming_a_session_is_active() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let _lease = DriverLease::acquire(&root).unwrap();
        let stale_signal = root.join(".git/grove/signal-11111111111111111111111111111111");
        let (_lock, _environment) = SignalEnvironment::set(Some(&stale_signal));

        let error = admit_ambient_session(&root, "test pick").unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("session epoch is inactive"), "{message}");
        assert!(!message.contains("active epoch"), "{message}");
    }

    #[test]
    fn a_rotated_epoch_refuses_the_old_signal_path() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&root).unwrap();
        let old_signal = root.join(".git/grove/signal-11111111111111111111111111111111");
        let new_signal = root.join(".git/grove/signal-22222222222222222222222222222222");
        lease.activate_session_epoch(&old_signal).unwrap();
        let (_lock, _environment) = SignalEnvironment::set(Some(&old_signal));

        drop(
            admit_ambient_session(&root, "test pick")
                .unwrap()
                .expect("the old signal must be admitted while its epoch is live"),
        );
        lease.activate_session_epoch(&new_signal).unwrap();

        let error = admit_ambient_session(&root, "test pick").unwrap_err();
        assert!(
            format!("{error:#}").contains("loop-control path does not match the active epoch"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn an_epoch_signal_path_round_trips_record_separator_bytes() {
        let tmp = TempDir::new().unwrap();
        let root = tmp
            .path()
            .join(OsString::from_vec(b"worktree-\n-name".to_vec()));
        fs::create_dir_all(root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&root).unwrap();
        let signal_path = root.join(".git/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        let (_lock, _environment) = SignalEnvironment::set(Some(&signal_path));

        drop(
            admit_ambient_session(&root, "test pick")
                .unwrap()
                .expect("the exact signal path must survive epoch serialization"),
        );
        let record = fs::read_to_string(root.join(".git/grove/session.epoch")).unwrap();
        assert!(record.contains("signal-path-hex="), "{record:?}");
    }

    #[test]
    fn a_successful_liveness_probe_releases_the_lease_before_validation() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&root).unwrap();
        let signal_path = root.join(".git/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        let control_dir = root.join(".git/grove");
        let mut epoch_file = File::open(control_dir.join(EPOCH_FILE_NAME)).unwrap();
        let epoch = read_epoch_record(&mut epoch_file).unwrap();
        drop(lease);
        let mut observed_unlocked_probe = false;

        let error = probe_live_lease_with_post_unlock_hook(
            &control_dir,
            &epoch,
            "test pick",
            |lease_path| {
                let contender = File::open(lease_path)?;
                let result =
                    unsafe { libc::flock(contender.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
                if result != 0 {
                    return Err(std::io::Error::last_os_error())
                        .context("successful liveness probe still held the driver lease");
                }
                observed_unlocked_probe = true;
                Ok(())
            },
        )
        .unwrap_err();

        assert!(observed_unlocked_probe);
        assert!(
            format!("{error:#}").contains("driver lease is unlocked"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn an_active_epoch_without_a_live_lease_is_stale() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&root).unwrap();
        let signal_path = root.join(".git/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        let (_lock, _environment) = SignalEnvironment::set(Some(&signal_path));
        drop(lease);

        let error = admit_ambient_session(&root, "test pick").unwrap_err();
        assert!(
            format!("{error:#}").contains("driver lease is unlocked"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn a_malformed_epoch_is_stale() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".git")).unwrap();
        let lease = DriverLease::acquire(&root).unwrap();
        let signal_path = root.join(".git/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        fs::write(root.join(".git/grove/session.epoch"), "state=active\n").unwrap();
        let (_lock, _environment) = SignalEnvironment::set(Some(&signal_path));

        let error = admit_ambient_session(&root, "test pick").unwrap_err();
        assert!(
            format!("{error:#}").contains("stale Grove session"),
            "unexpected error: {error:#}"
        );
    }
}
