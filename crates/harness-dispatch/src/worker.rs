//! Find, start, verify and converse with the compiled policy worker
//! (`docs/specs/harness-selection-and-execution.md`, *Policy authority and
//! runtime discovery*; `docs/adr/policy-evaluation-precedes-process-replacement.md`).
//!
//! The worker is found only relative to the real front executable. It starts in
//! a private empty directory with null stdin, a fresh environment, captured
//! stdout and stderr, and one socket at descriptor 3 as its private channel. Its
//! first frame states its protocol and build identity, and the front checks both
//! before the worker learns which entry to evaluate: a worker from another build
//! is never handed a policy.
//!
//! The whole selection is bounded from the worker's start to its result, by the
//! front's own clock (*Bounded context*, *Execution and authority*). A policy
//! spinning at import, or awaiting work that never settles, cannot end itself,
//! so the deadline relies on nothing the worker does: every channel read and
//! write waits only for the time left, and at expiry the front stops and reaps
//! the worker and refuses with exit 124.

use std::ffi::OsString;
use std::fs;
use std::io::{self, ErrorKind, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::frame::{read_frame, write_frame, FrameError};
use crate::inputs::{Bound, SELECTION_MAX_MS};
use crate::refusal::{Diagnostics, Failure, Refusal, Stage, EXIT_TIMEOUT, EXIT_WORKER};

pub const PROTOCOL: u64 = 1;
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
/// The digest of the worker source this front was built beside (see build.rs).
pub const BUILD_ID: &str = env!("HARNESS_DISPATCH_WORKER_BUILD_ID");

/// The installed layout, relative to the directory holding the real front
/// executable. `release-layout-k17` must ship archives in the same shape.
const WORKER_FROM_BIN: &str = "../libexec/harness-dispatch/harness-dispatch-policy";

/// The descriptor the worker's channel occupies, fixed on both sides.
const CHANNEL_FD: RawFd = 3;

/// How long to keep collecting diagnostics after the worker is reaped. A
/// descendant the policy left holding its stdout could otherwise keep a drain
/// open forever; detached policy children are outside the contract, so their
/// late output is not waited for. Both streams share the one grace.
const DRAIN_GRACE: Duration = Duration::from_secs(1);

/// How long a worker has to exit, after TERM at the deadline or by itself
/// after its result, before it is killed.
const CLEANUP_GRACE: Duration = Duration::from_secs(1);

const REBUILD: &str =
    "reinstall harness-dispatch so that its front and worker come from one build; \
     in a source checkout, run `task dispatch:worker`";

#[derive(Debug)]
pub struct WorkerIdentity {
    pub path: PathBuf,
    pub package_version: String,
    pub build_id: String,
    pub bun_version: String,
}

/// What the worker reported about the selected entry.
#[derive(Debug)]
pub enum Outcome {
    /// The serializable snapshot of the entry's `policy` export.
    Policy(Value),
    /// The entry, or something it imports, failed to load.
    LoadFailed { name: String, message: String },
    /// The export is not a policy-shaped object at all.
    Invalid { location: String, message: String },
}

#[derive(Debug)]
pub struct Evaluation {
    pub worker: WorkerIdentity,
    pub outcome: Outcome,
    pub diagnostics: Diagnostics,
    /// From starting the worker to receiving its result.
    pub elapsed: Duration,
}

/// The worker beside the real front executable, following any symlinks to it.
/// No PATH, cwd or environment input takes part.
pub fn locate() -> Result<PathBuf, Refusal> {
    // Without the front's own path the worker's has no directory either, so
    // the source is the layout the worker would be found at.
    let unlocatable = |detail: String, source: String| {
        Refusal::new(
            "worker_missing",
            Stage::Worker,
            EXIT_WORKER,
            format!("cannot locate the policy worker: {detail}"),
            REBUILD,
        )
        .source(source)
    };
    let exe = std::env::current_exe()
        .and_then(fs::canonicalize)
        .map_err(|error| {
            unlocatable(
                format!("the front executable's path is unknown ({error})"),
                format!("<the front executable's directory>/{WORKER_FROM_BIN}"),
            )
        })?;
    let bin = exe.parent().ok_or_else(|| {
        unlocatable(
            format!("{} has no parent directory", exe.display()),
            exe.to_string_lossy().into_owned(),
        )
    })?;
    let worker = normalize(&bin.join(WORKER_FROM_BIN));
    match fs::metadata(&worker) {
        Ok(metadata) if metadata.is_file() => Ok(worker),
        Ok(_) => Err(Refusal::new(
            "worker_missing",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker {} is not a regular file",
                worker.display()
            ),
            REBUILD,
        )
        .source(worker.to_string_lossy())),
        Err(error) => Err(Refusal::new(
            "worker_missing",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker is not installed at {} ({error})",
                worker.display()
            ),
            REBUILD,
        )
        .source(worker.to_string_lossy())),
    }
}

/// Drop `..` lexically. The input is already canonical up to the layout suffix,
/// so this only makes the reported path readable.
fn normalize(path: &Path) -> PathBuf {
    let mut normal = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                normal.pop();
            }
            other => normal.push(other),
        }
    }
    normal
}

/// Start the worker, verify it, and have it evaluate `entry`, all within
/// `bound`, which counts from the worker's start.
pub fn evaluate(
    worker: &Path,
    entry: &Path,
    request: Value,
    bound: Bound,
) -> Result<Evaluation, Failure> {
    let failed = |message: String| {
        Refusal::new(
            "worker_failed",
            Stage::Worker,
            EXIT_WORKER,
            message,
            REBUILD,
        )
        .source(worker.to_string_lossy())
    };

    let private_dir = tempfile::Builder::new()
        .prefix("harness-dispatch-worker.")
        .tempdir()
        .map_err(|error| {
            failed(format!(
                "cannot create the worker's private directory: {error}"
            ))
        })?;
    let (front_end, pair_end) = UnixStream::pair()
        .map_err(|error| failed(format!("cannot create the protocol channel: {error}")))?;
    let worker_end = above_stdio(&pair_end)
        .map_err(|error| failed(format!("cannot prepare the protocol channel: {error}")))?;
    // Only the duplicate crosses into the worker. Any other copy of its end
    // held here would keep the channel open after the worker dies, and the
    // read below would wait for an EOF that never comes.
    drop(pair_end);

    let mut command = Command::new(worker);
    command
        .current_dir(private_dir.path())
        .env_clear()
        .envs(worker_environment())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let channel = worker_end.as_raw_fd();
    let descriptor_limit = descriptor_limit();
    // SAFETY: the closure runs between fork and exec, so it calls only
    // async-signal-safe functions (`dup2`, `fcntl`) on descriptors computed
    // before the fork, and touches no Rust runtime state or allocator.
    unsafe {
        command.pre_exec(move || {
            // `dup2` onto itself would leave close-on-exec set, and
            // `above_stdio` guarantees the source is at least 3.
            if channel == CHANNEL_FD {
                let flags = libc::fcntl(channel, libc::F_GETFD);
                if flags == -1
                    || libc::fcntl(channel, libc::F_SETFD, flags & !libc::FD_CLOEXEC) == -1
                {
                    return Err(std::io::Error::last_os_error());
                }
            } else if libc::dup2(channel, CHANNEL_FD) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            // Everything above the channel closes at exec, so the worker holds
            // descriptors 0-3 and nothing a caller happened to leave open.
            for fd in CHANNEL_FD + 1..descriptor_limit {
                if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1
                    && std::io::Error::last_os_error().raw_os_error() != Some(libc::EBADF)
                {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Ok(())
        });
    }

    let started = Instant::now();
    let deadline = started + bound.duration();
    let mut child = command.spawn().map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            Refusal::new(
                "worker_missing",
                Stage::Worker,
                EXIT_WORKER,
                format!(
                    "the policy worker {} cannot be started: {error}",
                    worker.display()
                ),
                REBUILD,
            )
            .source(worker.to_string_lossy())
        } else {
            failed(format!(
                "the policy worker {} cannot be started: {error}",
                worker.display()
            ))
        }
    })?;
    // The front keeps only its own end, so the worker's exit reads as EOF.
    drop(worker_end);
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());

    let channel = Bounded {
        channel: &front_end,
        deadline,
    };
    let result = converse(channel, worker, entry, request, started);
    drop(front_end);
    let status = match &result {
        // Its work is done, and it exits by itself unless the policy keeps it.
        Ok(_) => stop(&mut child, None),
        // TERM first, so that a policy awaiting work can clean up.
        Err(Conversation::Expired { .. }) => stop(&mut child, Some(libc::SIGTERM)),
        Err(_) => {
            let _ = child.kill();
            child.wait()
        }
    };
    let drained_by = Instant::now() + DRAIN_GRACE;
    let diagnostics = Diagnostics {
        stdout: stdout.collect(drained_by),
        stderr: stderr.collect(drained_by),
    };
    drop(private_dir);

    match result {
        Ok((worker, outcome, elapsed)) => Ok(Evaluation {
            worker,
            outcome,
            diagnostics,
            elapsed,
        }),
        Err(Conversation::Refused(refusal)) => Err(Failure::with_diagnostics(refusal, diagnostics)),
        Err(Conversation::Expired { handed_over }) => Err(Failure::with_diagnostics(
            expired(worker, entry, bound, handed_over),
            diagnostics,
        )),
        Err(Conversation::Ended(error)) => {
            let status = match status {
                Ok(status) => status.to_string(),
                Err(error) => format!("unknown status ({error})"),
            };
            Err(Failure::with_diagnostics(
                failed(format!(
                    "the policy worker ended without a result ({error}; it exited with {status}); \
                     a policy that calls process.exit ends it the same way"
                )),
                diagnostics,
            ))
        }
    }
}

enum Conversation {
    Refused(Refusal),
    /// The channel failed or closed: the worker died, or exited on its own.
    Ended(FrameError),
    /// The deadline passed first, after the entry was handed over or before.
    Expired {
        handed_over: bool,
    },
}

impl Conversation {
    fn failed(error: FrameError, handed_over: bool) -> Self {
        match error {
            FrameError::Io(error) if error.kind() == ErrorKind::TimedOut => {
                Conversation::Expired { handed_over }
            }
            other => Conversation::Ended(other),
        }
    }
}

fn converse(
    mut channel: Bounded<'_>,
    worker: &Path,
    entry: &Path,
    request: Value,
    started: Instant,
) -> Result<(WorkerIdentity, Outcome, Duration), Conversation> {
    let protocol_error = |message: String| {
        Conversation::Refused(
            Refusal::new(
                "protocol_error",
                Stage::Worker,
                EXIT_WORKER,
                message,
                REBUILD,
            )
            .source(worker.to_string_lossy()),
        )
    };
    let receive = |channel: &mut Bounded<'_>, handed_over| match read_frame(channel) {
        Err(error @ (FrameError::TooLarge(_) | FrameError::NotJson(_))) => Err(protocol_error(
            format!("the policy worker sent a malformed frame: {error}"),
        )),
        other => other.map_err(|error| Conversation::failed(error, handed_over)),
    };

    let hello = receive(&mut channel, false)?;
    let identity = verify_hello(&hello, worker).map_err(Conversation::Refused)?;

    write_frame(
        &mut channel,
        &json!({
            "type": "evaluate",
            "protocol": PROTOCOL,
            "entry": entry.to_string_lossy(),
            "request": request,
        }),
    )
    .map_err(|error| Conversation::failed(error, false))?;

    let result = receive(&mut channel, true)?;
    let elapsed = started.elapsed();
    let text = |field: &str| result.get(field).and_then(Value::as_str).map(str::to_owned);
    let outcome = match (text("type").as_deref(), text("stage").as_deref()) {
        (Some("policy"), _) => result.get("policy").cloned().map(Outcome::Policy),
        (Some("failure"), Some("load")) => text("message").map(|message| Outcome::LoadFailed {
            name: text("name").unwrap_or_else(|| "Error".to_owned()),
            message,
        }),
        (Some("failure"), Some("validation")) => text("message").map(|message| Outcome::Invalid {
            location: text("location").unwrap_or_else(|| "policy".to_owned()),
            message,
        }),
        _ => None,
    };
    let outcome =
        outcome.ok_or_else(|| protocol_error(format!("unexpected result frame: {result}")))?;
    Ok((identity, outcome, elapsed))
}

/// The refusal when the deadline passes: after the entry was handed over, the
/// policy held evaluation; before, the worker never became ready for it.
fn expired(worker: &Path, entry: &Path, bound: Bound, handed_over: bool) -> Refusal {
    let limit = bound.to_text();
    let raise = format!("or allow more time with --timeout-ms, up to {SELECTION_MAX_MS}");
    let refusal = if handed_over {
        Refusal::new(
            "selection_timeout",
            Stage::Evaluation,
            EXIT_TIMEOUT,
            format!(
                "the policy entry {} did not return a result within the selection bound of \
                 {limit}, so its worker was stopped and nothing was launched",
                entry.display()
            ),
            format!(
                "make the policy return sooner: a loop, or an await on work that never \
                 settles, holds its evaluation; {raise}"
            ),
        )
        .source(entry.to_string_lossy())
    } else {
        Refusal::new(
            "selection_timeout",
            Stage::Evaluation,
            EXIT_TIMEOUT,
            format!(
                "the policy worker {} was not ready for the policy within the selection bound \
                 of {limit}, so it was stopped and nothing was launched",
                worker.display()
            ),
            format!("{REBUILD}; on a heavily loaded machine, {raise}"),
        )
        .source(worker.to_string_lossy())
    };
    let refusal = refusal.bound(bound);
    match bound.flag {
        Some(flag) => refusal.input(flag),
        None => refusal,
    }
}

/// Reap the worker. It has `CLEANUP_GRACE` to exit, after `signal` when one is
/// given, and is then killed. Only the worker is signalled: it shares the
/// caller's process group, so a group signal would reach this process and its
/// caller too.
fn stop(child: &mut Child, signal: Option<libc::c_int>) -> io::Result<ExitStatus> {
    if let (Some(signal), Ok(pid)) = (signal, libc::pid_t::try_from(child.id())) {
        // SAFETY: kill only sends a signal. The worker is an unreaped child of
        // this process, so its PID cannot yet name any other process.
        unsafe { libc::kill(pid, signal) };
    }
    let grace = Instant::now() + CLEANUP_GRACE;
    let mut poll = Duration::from_millis(1);
    while Instant::now() < grace {
        // `try_wait` reaps only an exited worker, so the PID stays this
        // worker's until the kill below. If it cannot tell, the kill decides.
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) => {}
            Err(_) => break,
        }
        thread::sleep(poll);
        poll = (poll * 2).min(Duration::from_millis(16));
    }
    let _ = child.kill();
    child.wait()
}

/// The protocol channel with the deadline applied: each read or write waits
/// only for the time left, and once none is left it fails with `TimedOut`.
struct Bounded<'a> {
    channel: &'a UnixStream,
    deadline: Instant,
}

impl Bounded<'_> {
    /// The time left, which is never zero: std refuses a zero socket timeout,
    /// and rounds a sub-microsecond one up to a microsecond.
    fn left(&self) -> io::Result<Duration> {
        let left = self.deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            Err(ErrorKind::TimedOut.into())
        } else {
            Ok(left)
        }
    }
}

/// An expired socket timeout reads as `WouldBlock` (`EAGAIN`) on Unix; std
/// documents either kind, and both mean the deadline passed.
fn timed_out(error: io::Error) -> io::Error {
    match error.kind() {
        ErrorKind::WouldBlock | ErrorKind::TimedOut => ErrorKind::TimedOut.into(),
        _ => error,
    }
}

impl Read for Bounded<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.channel.set_read_timeout(Some(self.left()?))?;
        let mut channel = self.channel;
        channel.read(buffer).map_err(timed_out)
    }
}

impl Write for Bounded<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.channel.set_write_timeout(Some(self.left()?))?;
        let mut channel = self.channel;
        channel.write(buffer).map_err(timed_out)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn verify_hello(hello: &Value, worker: &Path) -> Result<WorkerIdentity, Refusal> {
    let text = |field: &str| hello.get(field).and_then(Value::as_str).unwrap_or("absent");
    if hello.get("type").and_then(Value::as_str) != Some("hello") {
        return Err(Refusal::new(
            "protocol_error",
            Stage::Worker,
            EXIT_WORKER,
            format!("the policy worker's first frame is not a hello: {hello}"),
            REBUILD,
        )
        .source(worker.to_string_lossy()));
    }
    let protocol = hello.get("protocol").and_then(Value::as_u64);
    if protocol != Some(PROTOCOL)
        || text("packageVersion") != PACKAGE_VERSION
        || text("buildId") != BUILD_ID
    {
        let protocol = protocol.map_or_else(|| "absent".to_owned(), |p| p.to_string());
        return Err(Refusal::new(
            "worker_identity_mismatch",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker {} is not this front's pair: it reports protocol {protocol}, \
                 package {}, build {}; this front needs protocol {PROTOCOL}, package \
                 {PACKAGE_VERSION}, build {BUILD_ID}",
                worker.display(),
                text("packageVersion"),
                text("buildId"),
            ),
            REBUILD,
        )
        .source(worker.to_string_lossy()));
    }
    Ok(WorkerIdentity {
        path: worker.to_owned(),
        package_version: PACKAGE_VERSION.to_owned(),
        build_id: BUILD_ID.to_owned(),
        bun_version: text("bunVersion").to_owned(),
    })
}

/// HOME, a PATH snapshot, TMPDIR, LANG and LC_*, from the caller; nothing else.
/// Later increments add the owner's exact `--policy-env` grants.
fn worker_environment() -> Vec<(OsString, OsString)> {
    std::env::vars_os()
        .filter(|(name, _)| {
            let name = name.as_bytes();
            matches!(name, b"HOME" | b"PATH" | b"TMPDIR" | b"LANG") || name.starts_with(b"LC_")
        })
        .collect()
}

/// A close-on-exec duplicate of `stream` numbered 3 or higher. A caller that
/// started this process with a standard descriptor closed would otherwise let
/// the socket land on 0-2, where the child's stdio setup overwrites it.
fn above_stdio(stream: &UnixStream) -> std::io::Result<OwnedFd> {
    // SAFETY: F_DUPFD_CLOEXEC returns a new descriptor this function owns.
    let fd = unsafe { libc::fcntl(stream.as_raw_fd(), libc::F_DUPFD_CLOEXEC, CHANNEL_FD) };
    if fd == -1 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: `fd` is a fresh descriptor owned by nothing else.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

/// The soft descriptor limit, bounded so an unlimited one cannot make the
/// close-on-exec sweep effectively endless.
fn descriptor_limit() -> RawFd {
    // SAFETY: getrlimit writes only the rlimit structure it is given.
    let mut limit: libc::rlimit = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } == -1 {
        return 1024;
    }
    RawFd::try_from(limit.rlim_cur.min(65_536)).unwrap_or(1024)
}

/// A diagnostic stream being read to its end on another thread.
struct Drain {
    captured: Arc<Mutex<Vec<u8>>>,
    done: mpsc::Receiver<()>,
}

fn drain(stream: Option<impl Read + Send + 'static>) -> Drain {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let (sender, done) = mpsc::channel();
    if let Some(mut stream) = stream {
        let captured = Arc::clone(&captured);
        thread::spawn(move || {
            let mut chunk = [0; 8192];
            loop {
                match stream.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(read) => captured
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .extend_from_slice(&chunk[..read]),
                    Err(error) if error.kind() == ErrorKind::Interrupted => {}
                    Err(_) => break,
                }
            }
            let _ = sender.send(());
        });
    }
    Drain { captured, done }
}

impl Drain {
    /// Everything read so far, once the stream ends or `by` passes.
    fn collect(self, by: Instant) -> Vec<u8> {
        let _ = self
            .done
            .recv_timeout(by.saturating_duration_since(Instant::now()));
        let captured = self
            .captured
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        captured.clone()
    }
}
