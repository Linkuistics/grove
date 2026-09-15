//! **One live driver per working tree** — the lease that makes it true, and the
//! session-epoch handoff that decides which `grove-llm` calls it admits.
//!
//! It arrived here with the rest of the driver at `loop-crate-driver-k22`. The
//! one thing that is not a move is its derivation of a control directory: it
//! takes a resolved [`Workspace`] and asks the seam for grove's namespace inside
//! it (`docs/adr/one-live-driver-per-working-tree.md`), rather than resolving a
//! path itself. The caller has already resolved the workspace — `run` takes one
//! — so a second resolution here could only disagree with the first.

pub(crate) mod observation;
mod witnesses;

use anyhow::{bail, Context, Result};
use jj_workspace::Workspace;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Grove's control namespace inside the workspace's administration area.
///
/// The seam owns *where* an untracked coordination directory may live and
/// guarantees it is not shared; grove owns *whose* it is, and this is the whole
/// of that ownership. One constant rather than a literal per call site, because
/// two spellings of the namespace are two control directories and the second
/// driver would not see the first one's lease.
const CONTROL_NAMESPACE: &str = "grove";

const LEASE_FILE_NAME: &str = "driver.lease";
const EPOCH_FILE_NAME: &str = "session.epoch";
const IDENTITY_RETRY_LIMIT: usize = 8;
const EPOCH_HANDOFF_TIMEOUT: Duration = Duration::from_secs(30);
const EPOCH_WAIT_INTERVAL: Duration = Duration::from_millis(10);

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
    // Released explicitly before driver ownership, including on unwind.
    launch: Option<witnesses::LaunchWitnesses>,
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
pub struct SessionEpochGuard {
    _epoch_file: File,
    signal_path: PathBuf,
}

impl SessionEpochGuard {
    /// Refuse an operation whose completion channel is not the one this epoch
    /// admitted.
    ///
    /// # Errors
    ///
    /// Any other path, including none.
    pub fn require_signal_path(&self, signal_path: Option<&Path>) -> Result<(), crate::Error> {
        Ok(self.require_signal_path_inner(signal_path)?)
    }

    fn require_signal_path_inner(&self, signal_path: Option<&Path>) -> Result<()> {
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

impl DriverLease {
    /// Take exclusive driver ownership of `workspace`'s working tree.
    ///
    /// # Errors
    ///
    /// A workspace whose control directory cannot be created, a working tree
    /// root that cannot be opened, or a predecessor whose session epoch does not
    /// hand over.
    pub fn acquire(workspace: &Workspace) -> Result<Self, crate::Error> {
        Ok(Self::acquire_with(workspace, || {})?)
    }

    fn acquire_with(
        workspace: &Workspace,
        before_initial_epoch_handoff: impl FnOnce(),
    ) -> Result<Self> {
        // The control directory comes from the resolution the caller already
        // performed: the seam refuses a working tree that is not jj-enabled
        // before it hands one out, and it hands back grove's own namespace
        // inside it, created if absent.
        let control_dir = workspace.control_dir(CONTROL_NAMESPACE)?;
        let worktree_root = workspace.root().to_path_buf();
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

        let lease_path = control_dir.join(LEASE_FILE_NAME);
        let (lease_file, lease_identity) = acquire_lease_file(&lease_path, &worktree_root)?;
        let nonce = hex_nonce(random_nonce()?)?;

        let mut lease = Self {
            launch: None,
            worktree_root,
            control_dir,
            _worktree_directory: worktree_directory,
            worktree_identity,
            lease_path,
            lease_file,
            lease_identity,
            nonce,
        };
        lease.revalidate_inner()?;
        before_initial_epoch_handoff();
        lease.initialize_epoch_record()?;
        lease.clean_witnesses();
        // Only after this lease owns the workspace, and after the replacement
        // driver's inactive epoch record is installed: cleaning first would
        // remove a live predecessor's channel. The grammar of an abandoned
        // channel is the runner's, so the runner recognises them
        // (`keyed_launch::Channel::discard_abandoned`); the lease supplies only
        // the directory.
        if let Err(error) = keyed_launch::Channel::discard_abandoned(&lease.control_dir) {
            eprintln!(
                "grove: warning: could not clean every signal channel abandoned by a previous driver; continuing because fresh channel allocation does not depend on an empty control directory: {error}"
            );
        }
        Ok(lease)
    }

    /// The working tree this lease owns, in the spelling the workspace resolved
    /// to.
    #[must_use]
    pub fn worktree_root(&self) -> &Path {
        &self.worktree_root
    }

    /// Grove's control directory inside the workspace's administration area —
    /// where the runner allocates each launch's completion channel.
    ///
    /// Handed out rather than used here: the channel's name grammar, allocation
    /// and cleanup are `keyed-launch`'s, and the lease's contribution is the one
    /// thing that is genuinely grove's — *which* directory is ours.
    pub(crate) fn control_dir(&self) -> &Path {
        &self.control_dir
    }

    #[cfg(test)]
    pub(crate) fn activate_session_epoch(&self, signal_path: &Path) -> Result<()> {
        self.write_epoch_record(Some(signal_path), "pre-spawn activation")
    }

    pub(crate) fn invalidate_session_epoch(&self) -> Result<()> {
        self.write_epoch_record(None, "post-reap invalidation")?;
        self.clean_witnesses();
        Ok(())
    }

    fn clean_witnesses(&self) {
        let retained = self.launch.as_ref().and_then(|launch| launch.path());
        if let Err(error) = witnesses::discard_abandoned(&self.control_dir, retained) {
            eprintln!("grove: warning: could not clean abandoned witnesses; continuing: {error:#}");
        }
    }

    /// Transfer the selected pin before publication. The second check occurs
    /// after the potentially blocking epoch acquisition, with that guard held.
    pub(crate) fn prepare_launch(
        &mut self,
        root: crate::TreeLifetime,
        selected: &crate::Selection,
        signal_path: &Path,
    ) -> Result<()> {
        self.prepare_launch_with(root, selected, signal_path, |path| {
            acquire_epoch_file(path, LockMode::Exclusive, "pre-spawn activation")
        })
    }

    fn prepare_launch_with(
        &mut self,
        root: crate::TreeLifetime,
        selected: &crate::Selection,
        signal_path: &Path,
        acquire: impl FnOnce(&Path) -> Result<File>,
    ) -> Result<()> {
        self.prepare_launch_using(root, signal_path, acquire, |launch, epoch| {
            launch.publish(epoch, selected)
        })
    }

    fn prepare_launch_using(
        &mut self,
        root: crate::TreeLifetime,
        signal_path: &Path,
        acquire: impl FnOnce(&Path) -> Result<File>,
        publish: impl FnOnce(&witnesses::LaunchWitnesses, &mut File) -> Result<()>,
    ) -> Result<()> {
        anyhow::ensure!(
            self.launch.is_none(),
            "previous launch has no confirmed reap"
        );
        anyhow::ensure!(
            root.at(&self.worktree_root)?,
            "task tree changed before foreground launch"
        );
        self.launch = Some(witnesses::LaunchWitnesses::new(root));
        let result = (|| {
            let mut epoch = acquire(&self.control_dir.join(EPOCH_FILE_NAME))?;
            anyhow::ensure!(
                self.launch
                    .as_ref()
                    .context("selected root missing during preparation")?
                    .root
                    .at(&self.worktree_root)?,
                "task tree changed before foreground launch"
            );
            // Drain predecessor readers and invalidate before either exclusive
            // witness can become visible under an old record.
            write_epoch_contents(
                &mut epoch,
                self.worktree_identity,
                &self.worktree_root,
                &self.nonce,
                None,
            )?;
            self.clean_witnesses();
            if let Some(launch) = self.launch.as_mut() {
                if let Err(error) = launch.prepare(&self.control_dir) {
                    eprintln!(
                        "grove: warning: launch observation unavailable; continuing: {error:#}"
                    );
                }
            }
            write_epoch_contents(
                &mut epoch,
                self.worktree_identity,
                &self.worktree_root,
                &self.nonce,
                Some(signal_path),
            )?;
            if let Some(launch) = &self.launch {
                if let Err(error) = publish(launch, &mut epoch) {
                    eprintln!("grove: warning: launch observation publication failed; continuing: {error:#}");
                }
            }
            Ok(())
        })();
        if result.is_err() {
            self.launch.take();
        }
        result
    }

    /// Only Started/Reaped evidence changes the lifetime. An unsuccessful wait
    /// leaves the pin here after the helper returns; epoch invalidation alone
    /// never releases it. The runner invokes Reaped before terminal recovery.
    pub(crate) fn supervise_launch<T>(
        &mut self,
        run: impl FnOnce(&mut dyn FnMut(keyed_launch::LaunchEvent)) -> T,
    ) -> T {
        let mut started = false;
        let result = run(&mut |event| match event {
            keyed_launch::LaunchEvent::Started => {
                started = true;
                if let Some(launch) = self.launch.as_mut() {
                    if let Err(error) = launch.started() {
                        eprintln!("grove: warning: launch observation publication failed; continuing: {error:#}");
                    }
                }
            }
            keyed_launch::LaunchEvent::Reaped => {
                self.launch.take();
            }
        });
        if !started {
            self.launch.take();
        }
        result
    }

    /// Confirm that the paths still name the descriptors this process owns.
    ///
    /// # Errors
    ///
    /// A working tree root or a lease file that was replaced while this process
    /// held ownership, or either one gone unreadable.
    pub fn revalidate(&self) -> Result<(), crate::Error> {
        Ok(self.revalidate_inner()?)
    }

    fn revalidate_inner(&self) -> Result<()> {
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

impl Drop for DriverLease {
    fn drop(&mut self) {
        self.launch.take();
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
            // Same contract as the lease open in `acquire_lease_file_with_hook`:
            // never truncate at open, because this runs before the lock is held;
            // `write_epoch_contents` truncates under the lock instead.
            // `suspicious_open_options` does not fire here only because clippy
            // cannot follow a builder split across statements — stated
            // explicitly so a future refactor into a chained call stays clean.
            options.write(true).create(true).truncate(false).mode(0o600);
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
        // `truncate(false)` is load-bearing, not decoration. This open happens
        // *before* `lock_exclusively_nonblocking` below, so it runs while the
        // incumbent lease holder may still own the file. Truncating here would
        // destroy a live holder's record before we know whether we can even take
        // the lock — and on the path where the lock attempt then fails, we would
        // have wrecked the record of a lease we do not hold. The reader at
        // `probe_live_lease_with_post_unlock_hook` parses that record, so an
        // emptied file also reads as a corrupt lease rather than an absent one.
        //
        // Truncation is deliberately deferred to `write_record`, which does its
        // own `set_len(0)` + rewind *after* the lock is held. Stating the
        // behaviour explicitly is what clears `suspicious_open_options`; the
        // semantics are unchanged, since `create` alone never truncated.
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
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

fn encode_path(path: &Path) -> Result<String> {
    let mut rendered = String::with_capacity(path.as_os_str().as_bytes().len() * 2);
    for byte in path.as_os_str().as_bytes() {
        write!(&mut rendered, "{byte:02x}").context("encoding working-tree path")?;
    }
    Ok(rendered)
}

fn decode_path(value: &str) -> Result<PathBuf> {
    anyhow::ensure!(value.is_ascii(), "path hex contains non-ASCII bytes");
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
    file.seek(SeekFrom::Start(0))
        .context("rewinding session epoch record")?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .context("reading session epoch record")?;
    // Unknown extension bytes cannot veto admission. Every mandatory value is
    // ASCII state, decimal or hex; replacement characters there still fail its
    // existing validation, including encoded paths and the nonce.
    parse_epoch_record(&String::from_utf8_lossy(&bytes))
}

fn parse_epoch_record(record: &str) -> Result<EpochRecord> {
    let process = parse_process_record(record)?;
    let signal_path = match record_field(record, "state")? {
        "inactive" => {
            if record
                .lines()
                .any(|line| line.starts_with("signal-path-hex="))
            {
                bail!("inactive session epoch unexpectedly carries a signal path");
            }
            None
        }
        "active" => Some(decode_path(record_field(record, "signal-path-hex")?)?),
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
///
/// # Errors
///
/// A stale session: an epoch that is inactive, one belonging to another working
/// tree, one whose channel is not the ambient one, or one whose driver is no
/// longer alive.
pub fn admit_ambient_session(
    path: &Path,
    operation: &str,
) -> Result<Option<SessionEpochGuard>, crate::Error> {
    Ok(admit_session(path, operation, ambient_signal_path())?)
}

/// The loop-control context this process was launched into, if any.
///
/// Reading the environment is the *whole* of this function, and the only place
/// in the admission path that touches it — [`admit_session`] takes the resolved
/// path as an argument instead. That split is what lets admission be tested
/// without a unit test writing a process-global that production code in a
/// parallel sibling test is reading at the same moment.
fn ambient_signal_path() -> Option<PathBuf> {
    signal_path_from(std::env::var_os("GROVE_SIGNAL_FILE"))
}

/// Classify a loop-control value as ambient context or none. Empty is *none*
/// rather than a degenerate path: `.cargo/config.toml` force-clears the variable
/// to the empty string rather than unsetting it (`tests/env_hygiene.rs` owns
/// that claim), so empty is the value every cargo-launched `grove-llm` sees.
fn signal_path_from(value: Option<OsString>) -> Option<PathBuf> {
    value.filter(|value| !value.is_empty()).map(PathBuf::from)
}

/// Admission proper, with the ambient context already resolved.
fn admit_session(
    path: &Path,
    operation: &str,
    signal_path: Option<PathBuf>,
) -> Result<Option<SessionEpochGuard>> {
    let Some(signal_path) = signal_path else {
        return Ok(None);
    };
    let current_workspace = Workspace::resolve(path)?;
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

    if epoch.process.worktree_root != current_workspace.root() {
        bail!(
            "wrong working tree for {operation}: session belongs to {}, command resolved {}",
            epoch.process.worktree_root.display(),
            current_workspace.root().display()
        );
    }
    let current_identity = FileIdentity::from_metadata(
        &fs::metadata(current_workspace.root()).with_context(|| {
            format!(
                "reading current working-tree identity {}",
                current_workspace.root().display()
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
    pub(super) fn witness_selection() -> crate::Selection {
        crate::Selection {
            path: PathBuf::from(".grove/01-impl--work-k1.md"),
            handle: crate::Handle::parse("work-k1").unwrap(),
            kind: crate::Kind::new("impl").unwrap(),
        }
    }

    #[test]
    fn witnessed_epoch_binds_open_objects_and_admission_ignores_extensions() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let tree = root.directory().metadata().unwrap();
        let signal = lease.control_dir.join("signal-test\nnonce=injected");
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        let record = fs::read_to_string(&epoch_path).unwrap();
        let path = lease.launch.as_ref().unwrap().path().unwrap();
        let private = fs::metadata(path).unwrap();
        for (field, expected) in [
            ("observation-tree-device", tree.dev()),
            ("observation-tree-inode", tree.ino()),
            ("observation-witness-device", private.dev()),
            ("observation-witness-inode", private.ino()),
        ] {
            assert_eq!(
                record_field(&record, field)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                expected
            );
        }
        let basename =
            decode_path(record_field(&record, "observation-witness-name-hex").unwrap()).unwrap();
        assert_eq!(basename.components().count(), 1);
        assert_eq!(lease.control_dir.join(basename), path);
        let mandatory = parse_epoch_record(&record).unwrap();
        assert_eq!(mandatory.process.nonce, lease.nonce);
        assert_eq!(mandatory.signal_path.as_deref(), Some(signal.as_path()));
        lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Started));
        assert!(matches!(
            crate::try_observe(temp.path(), &[None]).activity,
            crate::ActivityObservation::Unavailable(_)
        ));
        let prefix = record.split("observation-version=").next().unwrap();
        for extension in ["", "observation-version=999\n", "observation-version=1\nobservation-key=bad\nobservation-key=2\nobservation-handle-hex=zz\n"] {
            fs::write(&epoch_path, format!("{prefix}{extension}")).unwrap();
            assert!(admit_session(temp.path(), "test", ambient(&signal)).is_ok(), "{extension}");
            assert!(admit_session(temp.path(), "test", ambient(&signal.with_extension("stale"))).is_err());
        }
        let mut bytes = prefix.as_bytes().to_vec();
        bytes.extend_from_slice(b"observation-kind-hex=\xff\n");
        fs::write(&epoch_path, bytes).unwrap();
        assert!(admit_session(temp.path(), "test", ambient(&signal)).is_ok());
        for field in ["worktree-path-hex", "signal-path-hex"] {
            let valid = record_field(prefix, field).unwrap();
            let corrupt =
                prefix.replace(&format!("{field}={valid}"), &format!("{field}=\u{fffd}0"));
            fs::write(&epoch_path, corrupt.as_bytes()).unwrap();
            assert!(admit_session(temp.path(), "test", ambient(&signal)).is_err());
            let mut corrupt_bytes = prefix
                .replace(&format!("{field}={valid}"), &format!("{field}=X0"))
                .into_bytes();
            let offset = corrupt_bytes
                .windows(field.len() + 2)
                .position(|bytes| bytes == format!("{field}=X").as_bytes())
                .unwrap();
            corrupt_bytes[offset + field.len() + 1] = 0xff;
            fs::write(&epoch_path, corrupt_bytes).unwrap();
            assert!(admit_session(temp.path(), "test", ambient(&signal)).is_err());
        }
    }

    #[test]
    fn witnessed_epoch_partial_publication_preserves_mandatory_activation() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let signal = lease.control_dir.join("signal-test");
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch_using(
                root,
                &signal,
                |path| acquire_epoch_file(path, LockMode::Exclusive, "test"),
                |launch, epoch| {
                    // Exhaust capacity partway through the real extension serializer.
                    struct Limited<'a>(&'a mut File, usize);
                    impl Write for Limited<'_> {
                        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                            if self.1 == 0 {
                                return Err(std::io::Error::other("injected full device"));
                            }
                            let n = self.0.write(&bytes[..bytes.len().min(self.1)])?;
                            self.1 -= n;
                            Ok(n)
                        }
                        fn flush(&mut self) -> std::io::Result<()> {
                            self.0.flush()
                        }
                    }
                    launch.publish(&mut Limited(epoch, 30), &witness_selection())
                },
            )
            .unwrap();
        assert!(admit_session(temp.path(), "test", ambient(&signal)).is_ok());
        let record = fs::read_to_string(lease.control_dir.join(EPOCH_FILE_NAME)).unwrap();
        assert!(
            record.ends_with("observation-version=1\nobservat"),
            "{record}"
        );
        let result = lease.supervise_launch(|event| {
            event(keyed_launch::LaunchEvent::Started);
            event(keyed_launch::LaunchEvent::Reaped);
            42
        });
        assert_eq!(result, 42);
        assert!(lease.launch.is_none());
    }

    #[test]
    fn paired_witness_failure_preserves_real_launch_and_admission() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
        lease
            .prepare_launch(root, &witness_selection(), channel.path())
            .unwrap();
        assert!(lease.launch.as_ref().unwrap().path().is_none());
        assert!(admit_session(temp.path(), "test", ambient(channel.path())).is_ok());
        assert!(matches!(
            crate::try_observe(temp.path(), &[None]).activity,
            crate::ActivityObservation::Unavailable(_)
        ));
        let config = temp.path().join("launch.kdl");
        fs::write(&config, "test \"/bin/sh -c 'echo launched > proof'\"\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        lease
            .supervise_launch(|observer| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(temp.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    observer,
                )
            })
            .unwrap();
        assert_eq!(fs::read(temp.path().join("proof")).unwrap(), b"launched\n");
        assert!(lease.launch.is_none());
        lease.invalidate_session_epoch().unwrap();
        channel.discard().unwrap();
    }

    #[test]
    fn paired_witness_lease_drop_releases_both_before_replacement_cleanup() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for unwind in [false, true] {
            let temp = TempDir::new().unwrap();
            fs::create_dir(temp.path().join(".jj")).unwrap();
            fs::create_dir(temp.path().join(".grove")).unwrap();
            let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let signal = lease.control_dir.join("signal-test");
            lease
                .prepare_launch(root, &witness_selection(), &signal)
                .unwrap();
            let path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
            let directory = File::open(temp.path().join(".grove")).unwrap();
            let private = File::open(&path).unwrap();
            lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Started));
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                let _lease = lease;
                assert!(!unwind, "injected lease unwind");
            }));
            assert_eq!(result.is_err(), unwind);
            for file in [&directory, &private] {
                assert_eq!(
                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                    0
                );
            }
            assert!(path.exists());
            let replacement = DriverLease::acquire_with(&workspace_at(temp.path()), || {
                assert!(
                    path.exists(),
                    "replacement cannot clean before epoch handoff"
                );
            })
            .unwrap();
            assert!(!path.exists());
            assert!(
                fs::read_to_string(replacement.control_dir.join(EPOCH_FILE_NAME))
                    .unwrap()
                    .starts_with("state=inactive\n")
            );
        }
    }

    #[test]
    fn paired_witnesses_are_owned_until_reap() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let signal = lease.control_dir.join("signal-test");
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        assert_ne!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0,
            "prepared directory must be exclusively witnessed"
        );
        let path = fs::read_dir(&lease.control_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("witness-")
            })
            .expect("a fresh private witness");
        let private = File::open(&path).unwrap();
        assert_ne!(
            unsafe { libc::flock(private.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert!(fs::read(&path).unwrap().is_empty());
        assert!(matches!(
            crate::try_observe(temp.path(), &[None]).activity,
            crate::ActivityObservation::Unavailable(_)
        ));
        // The directory witness must not take the containing-directory tree lock.
        assert!(matches!(
            crate::try_read(temp.path()).unwrap(),
            crate::TryReading::Ready(_)
        ));
        lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Started));
        lease.invalidate_session_epoch().unwrap();
        assert!(
            path.exists(),
            "unconfirmed reap retains witness even across invalidation"
        );
        assert_ne!(
            unsafe { libc::flock(private.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Reaped));
        assert_eq!(
            unsafe { libc::flock(private.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert!(path.exists(), "reap must not clean before invalidation");
        lease.invalidate_session_epoch().unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn lease_root_owner_real_spawn_and_preparation_failure_release() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for program in ["/bin/sh", "/no-such-grove-test-program"] {
            let temp = TempDir::new().unwrap();
            fs::create_dir(temp.path().join(".jj")).unwrap();
            fs::create_dir(temp.path().join(".grove")).unwrap();
            let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
            lease
                .prepare_launch(root, &witness_selection(), channel.path())
                .unwrap();
            let directory = File::open(temp.path().join(".grove")).unwrap();
            let private_path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
            let private = File::open(&private_path).unwrap();
            let config = temp.path().join("launch.kdl");
            fs::write(&config, format!("test \"{program} -c true\"\n")).unwrap();
            let templates = keyed_launch::Templates::load(
                &config,
                None,
                keyed_launch::Vocabulary { slots: &[] },
            )
            .unwrap();
            let argv = templates.expand("test", &[]).unwrap();
            let result = lease.supervise_launch(|observer| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(temp.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    &mut |event| {
                        observer(event);
                        assert_eq!(fs::read(&private_path).unwrap(), b"started\n");
                        if event == keyed_launch::LaunchEvent::Started {
                            keyed_launch::signal(channel.path(), "relaunch").unwrap();
                            assert!(channel.path().exists());
                        }
                        for file in [&directory, &private] {
                            let result = unsafe {
                                libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB)
                            };
                            if event == keyed_launch::LaunchEvent::Started {
                                assert_ne!(result, 0);
                            } else {
                                assert_eq!(result, 0, "Reaped releases before returning to runner");
                                assert_eq!(
                                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) },
                                    0
                                );
                            }
                        }
                    },
                )
            });
            assert_eq!(result.is_ok(), program == "/bin/sh", "{result:?}");
            assert!(lease.launch.is_none(), "{program}");
            for file in [&directory, &private] {
                assert_eq!(
                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                    0
                );
                assert_eq!(unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) }, 0);
            }
            assert_eq!(
                fs::read(&private_path).unwrap(),
                if program == "/bin/sh" {
                    b"started\n".as_slice()
                } else {
                    b""
                }
            );

            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let result =
                lease.prepare_launch_with(root, &witness_selection(), channel.path(), |_| {
                    Err(anyhow::anyhow!("injected epoch acquisition failure"))
                });
            assert!(result.is_err());
            assert!(lease.launch.is_none());
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let result =
                lease.prepare_launch_with(root, &witness_selection(), channel.path(), |path| {
                    // A read-only descriptor makes the mandatory epoch write fail.
                    Ok(File::open(path)?)
                });
            assert!(result.is_err());
            assert!(lease.launch.is_none());
            channel.discard().unwrap();
        }
    }

    #[test]
    fn lease_root_owner_unwind_retains_until_lease_drop() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let workspace = workspace_at(temp.path());
        let mut lease = DriverLease::acquire(&workspace).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let signal = lease.control_dir.join("signal-test");
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lease.supervise_launch(|observer| {
                observer(keyed_launch::LaunchEvent::Started);
                panic!("injected runner unwind");
            });
        }));
        assert!(unwind.is_err());
        assert!(lease.launch.is_some());
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lease.supervise_launch(|observer| {
                observer(keyed_launch::LaunchEvent::Reaped);
                panic!("failure after confirmed reap, before helper return");
            });
        }));
        assert!(unwind.is_err());
        assert!(lease.launch.is_none());
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let original = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        fs::rename(temp.path().join(".grove"), temp.path().join("old")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let replacement_pin = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        assert!(lease
            .prepare_launch(replacement_pin, &witness_selection(), &signal)
            .is_err());
        assert!(
            lease.launch.as_ref().unwrap().root.same(&original).unwrap(),
            "a second attempt replaced the unreaped pin"
        );
        assert!(DriverLease::acquire(&workspace).is_err());
        drop(lease);
        assert!(DriverLease::acquire(&workspace).is_ok());
    }

    #[test]
    fn lease_root_owner_rechecks_after_epoch_wait() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let signal = lease.control_dir.join("signal-test");
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        let reader = acquire_epoch_file(&epoch_path, LockMode::Shared, "test reader").unwrap();
        let (waiting_tx, waiting_rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            let result = lease.prepare_launch_with(root, &witness_selection(), &signal, |path| {
                acquire_epoch_file_with(
                    path,
                    LockMode::Exclusive,
                    "test preparation",
                    Duration::from_secs(5),
                    Instant::now,
                    thread::yield_now,
                    |_, _| Ok(()),
                    |_, _| Ok(()),
                    || waiting_tx.send(()).unwrap(),
                )
            });
            (lease, result)
        });
        waiting_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0,
            "preparation waiting on an old epoch must not acquire the directory witness"
        );
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_UN) },
            0
        );
        assert!(fs::read_dir(temp.path().join(".jj/grove"))
            .unwrap()
            .all(|entry| !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("witness-")));
        fs::rename(temp.path().join(".grove"), temp.path().join("old")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        drop(reader);
        let (lease, result) = worker.join().unwrap();
        assert!(format!("{:#}", result.unwrap_err()).contains("task tree changed"));
        assert!(lease.launch.is_none());
        assert!(fs::read_to_string(epoch_path)
            .unwrap()
            .starts_with("state=inactive\n"));
    }

    #[test]
    fn lease_root_owner_retains_unconfirmed_reap_and_releases_other_returns() {
        for events in [
            vec![],
            vec![keyed_launch::LaunchEvent::Started],
            vec![
                keyed_launch::LaunchEvent::Started,
                keyed_launch::LaunchEvent::Reaped,
            ],
        ] {
            let temp = TempDir::new().unwrap();
            fs::create_dir(temp.path().join(".jj")).unwrap();
            fs::create_dir(temp.path().join(".grove")).unwrap();
            let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let signal = lease.control_dir.join("signal-test");
            lease
                .prepare_launch(root, &witness_selection(), &signal)
                .unwrap();
            let result: Result<()> = lease.supervise_launch(|observer| {
                for event in &events {
                    observer(*event);
                }
                Err(anyhow::anyhow!("injected supervisor failure"))
            });
            assert!(result.is_err());
            let unconfirmed = events == [keyed_launch::LaunchEvent::Started];
            assert_eq!(lease.launch.is_some(), unconfirmed, "{events:?}");
            // Neither helper return nor epoch invalidation is evidence of reap.
            lease.invalidate_session_epoch().unwrap();
            assert_eq!(lease.launch.is_some(), unconfirmed);
        }
    }

    /// The fixtures below build a `.jj` marker directly and then acquire against
    /// it. Resolving here rather than inside `acquire` is the shape of the
    /// change this leaf made: the lease takes a workspace it did not resolve.
    fn workspace_at(path: &Path) -> Workspace {
        Workspace::resolve(path).expect("a fixture worktree carries a `.jj` marker")
    }

    use super::*;
    use std::cell::{Cell, RefCell};
    use std::process::Command;
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    const FORK_SENSITIVE_TEST: &str = "GROVE_DRIVER_LEASE_FORK_SENSITIVE_TEST";

    /// The ambient context an admission test would once have installed by
    /// writing `GROVE_SIGNAL_FILE`. Nothing here mutates the environment: these
    /// tests exercise [`admit_session`], whose ambient path is an argument, and
    /// the reading half above is covered separately — purely by
    /// [`signal_path_from`], and end to end by `tests/driver_lease.rs`, which
    /// sets the real variable on a real `grove-llm` subprocess.
    fn ambient(path: &Path) -> Option<PathBuf> {
        Some(path.to_path_buf())
    }

    pub(super) fn fork_sensitive_driver_lease_test_body_runs_here() -> bool {
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
        let control = root.join(".jj/grove");
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
        let control = root.join(".jj/grove");
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
        fs::create_dir_all(root.join(".jj")).unwrap();

        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();

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
    fn activation_and_invalidation_replace_one_stable_epoch_record() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let epoch_path = root.join(".jj/grove").join(EPOCH_FILE_NAME);
        let epoch_identity = FileIdentity::from_metadata(&fs::metadata(&epoch_path).unwrap());
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");

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
        let admission = admit_session(Path::new("/not-a-working-tree"), "manual pick", None)
            .expect("no ambient context is a manual command, not a failure");

        assert!(admission.is_none());
    }

    /// The reading half, pinned without touching the environment. The empty case
    /// is not a curiosity: `.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE`
    /// to the empty string, so treating empty as a *path* would make every
    /// cargo-launched `grove-llm` stale-fail before reaching its test seam.
    #[test]
    fn only_a_nonempty_loop_control_value_is_ambient_context() {
        assert_eq!(signal_path_from(None), None, "unset is no ambient context");
        assert_eq!(
            signal_path_from(Some(OsString::new())),
            None,
            "the cargo-cleared empty value is no ambient context either"
        );
        assert_eq!(
            signal_path_from(Some(OsString::from("/w/.jj/grove/signal-0"))),
            Some(PathBuf::from("/w/.jj/grove/signal-0"))
        );
    }

    #[test]
    fn an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();

        let admission = admit_session(&root, "test pick", ambient(&signal_path))
            .unwrap()
            .expect("ambient loop context must return a held admission guard");

        let epoch_path = root.join(".jj/grove").join(EPOCH_FILE_NAME);
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
            let result = DriverLease::acquire(&workspace_at(&replacement_root));
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

        let error = admit_session(&root, "test pick", ambient(&signal_path)).unwrap_err();
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
        fs::create_dir_all(root.join(".jj")).unwrap();
        let old_lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        old_lease.activate_session_epoch(&signal_path).unwrap();
        let lease_path = root.join(".jj/grove").join(LEASE_FILE_NAME);
        let old_record = fs::read_to_string(&lease_path).unwrap();
        let epoch_guard = File::open(root.join(".jj/grove").join(EPOCH_FILE_NAME)).unwrap();
        assert_eq!(
            unsafe { libc::flock(epoch_guard.as_raw_fd(), libc::LOCK_SH) },
            0
        );
        drop(old_lease);

        let (reached_tx, reached_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let replacement_root = root.clone();
        let replacement = thread::spawn(move || {
            DriverLease::acquire_with(&workspace_at(&replacement_root), || {
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
        fs::create_dir_all(owner_root.join(".jj")).unwrap();
        fs::create_dir_all(foreign_root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&owner_root)).unwrap();
        let signal_path = owner_root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();

        let error = admit_session(&foreign_root, "test pick", ambient(&signal_path)).unwrap_err();

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
        fs::create_dir_all(root.join(".jj")).unwrap();
        let _lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let stale_signal = root.join(".jj/grove/signal-11111111111111111111111111111111");

        let error = admit_session(&root, "test pick", ambient(&stale_signal)).unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("session epoch is inactive"), "{message}");
        assert!(!message.contains("active epoch"), "{message}");
    }

    #[test]
    fn a_rotated_epoch_refuses_the_old_signal_path() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let old_signal = root.join(".jj/grove/signal-11111111111111111111111111111111");
        let new_signal = root.join(".jj/grove/signal-22222222222222222222222222222222");
        lease.activate_session_epoch(&old_signal).unwrap();

        drop(
            admit_session(&root, "test pick", ambient(&old_signal))
                .unwrap()
                .expect("the old signal must be admitted while its epoch is live"),
        );
        lease.activate_session_epoch(&new_signal).unwrap();

        let error = admit_session(&root, "test pick", ambient(&old_signal)).unwrap_err();
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
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();

        drop(
            admit_session(&root, "test pick", ambient(&signal_path))
                .unwrap()
                .expect("the exact signal path must survive epoch serialization"),
        );
        let record = fs::read_to_string(root.join(".jj/grove/session.epoch")).unwrap();
        assert!(record.contains("signal-path-hex="), "{record:?}");
    }

    #[test]
    fn a_successful_liveness_probe_releases_the_lease_before_validation() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        let control_dir = root.join(".jj/grove");
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
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        drop(lease);

        let error = admit_session(&root, "test pick", ambient(&signal_path)).unwrap_err();
        assert!(
            format!("{error:#}").contains("driver lease is unlocked"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn a_malformed_epoch_is_stale() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        fs::write(root.join(".jj/grove/session.epoch"), "state=active\n").unwrap();

        let error = admit_session(&root, "test pick", ambient(&signal_path)).unwrap_err();
        assert!(
            format!("{error:#}").contains("stale Grove session"),
            "unexpected error: {error:#}"
        );
    }
}
