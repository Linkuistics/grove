//! Starting `ravel` as an ordinary child, as a job-control shell starts a job.
//!
//! One function, [`launch`], serves both callers. The lifecycle uses the
//! foreground [`Shape`]. `grove run` uses the detached one. The caller contract
//! states the procedures: `docs/specs/harness-selection-and-execution.md`, in
//! the section *Jobs and authority*.
//!
//! Grove keeps no runner of its own. Ravel supervises its managed process and
//! ends its group. So this module sends no KILL and probes no process group.
//! A successful Ravel status with an exact result is the only evidence of
//! cleanup. Ravel states its side of the arrangement in chapter 9 of its book:
//! <https://github.com/Linkuistics/Ravel/blob/v1.1.0/docs/book/src/09-jobs-signals-and-the-terminal.md>

use std::fmt;
use std::fs::File;
use std::io::Write as _;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::session_command::Argv;

/// How long the function waits between two checks of `ravel`.
///
/// This is not a knob. The interval only bounds how late a caught signal
/// reaches `ravel`, and Ravel budgets its own cancellation in seconds.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

/// The loop control variables. Every spawn removes them.
///
/// An environment is inherited, so a child that Grove merely declines to give
/// these still receives whatever Grove's own environment carried. That
/// includes a live launch directory of another session, which is authority to
/// end that session. `GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE` are
/// retired completion authority, kept for installed v22 sessions.
pub const CONTROL_ENV: [&str; 10] = [
    "GROVE_SIGNAL_FILE",
    "GROVE_RUN_SIGNAL_FILE",
    "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE",
    "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR",
    "RAVEL_EXIT_FILE",
    "GROVE_PROVIDER_EVIDENCE",
    "GROVE_HARNESS_PID",
    "GROVE_CLAUDE_PID",
];

/// Remove every [`CONTROL_ENV`] name from `command`'s environment.
///
/// Each spawn of Grove calls this: the job, the version probe and the terminal
/// reset. A rule with one argued exception is a rule that a later spawn breaks.
pub fn scrub(command: &mut Command) {
    for name in CONTROL_ENV {
        command.env_remove(name);
    }
}

/// Which kind of job `ravel` is. The shape fixes the terminal, the streams and
/// the one variable that Grove grants.
pub enum Shape<'a> {
    /// `ravel` leads a new process group in Grove's session. When Grove holds
    /// the foreground of its terminal, the group of `ravel` takes it. Standard
    /// input and standard error are inherited. The job receives
    /// `GROVE_LAUNCH_DIR` with `launch_dir` after the scrub.
    Foreground { launch_dir: &'a Path },
    /// `ravel` leads a new POSIX session with no terminal. Standard input is
    /// end of file, and standard error goes to `transcript`. Every descriptor
    /// above standard error closes at the exec. The job receives no variable
    /// that Grove grants.
    Detached { transcript: File },
}

/// Evidence about one job, independent of any result that `ravel` returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchEvent {
    /// The spawn succeeded. `ravel` may already have exited.
    Started,
    /// A wait confirmed the reap. It comes before the terminal recovery.
    Reaped,
}

/// How a job ended.
#[derive(Debug)]
pub struct JobEnd {
    /// The wait status of `ravel`. Only a successful status counts as evidence
    /// of cleanup.
    pub status: ExitStatus,
    /// The first signal that Grove caught during the job, or the signal that
    /// the latch still held at the reap. A later signal belongs to the caller.
    pub interrupted: Option<i32>,
    /// The time from the spawn to the reap.
    pub elapsed: Duration,
}

/// A job that failed to start or to be waited on.
///
/// **The error carries the interruption.** A signal that Grove caught is the
/// driver's, even when the spawn that follows it fails. Dropping it would turn
/// a Ctrl-C into an ordinary error, and the driver would carry on.
pub struct JobError {
    message: String,
    interrupted: Option<i32>,
}

impl JobError {
    /// The signal that Grove caught, if one reached the process before or
    /// during the job.
    #[must_use]
    pub fn interrupted(&self) -> Option<i32> {
        self.interrupted
    }
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a human reads this in an `anyhow` chain.
impl fmt::Debug for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for JobError {}

/// An error before the latch is taken into it.
struct Failure {
    message: String,
    /// A signal that Grove already sent to `ravel` before the failure.
    sent: Option<i32>,
}

impl Failure {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            sent: None,
        }
    }
}

/// Start `ravel` with `command` in `cwd` and wait until it exits.
///
/// Standard output goes to `stdout`, a regular file that the caller holds. Its
/// position and contents stay the caller's. The function never truncates it,
/// rewinds it or reads it.
///
/// `observer` receives [`LaunchEvent::Started`] once after a successful spawn
/// and [`LaunchEvent::Reaped`] once after a successful wait. A failed spawn
/// sends neither notice. A failed wait sends no `Reaped`. The observer must
/// return promptly and must not panic.
///
/// **The environment is Grove's minus [`CONTROL_ENV`].** Only [`Shape::Foreground`]
/// then adds `GROVE_LAUNCH_DIR`.
///
/// **The latch catches INT, TERM and HUP from the first call on.** A signal
/// that Grove ignores stays ignored. The function never clears the latch before
/// the spawn. So a signal that arrives after the last check of the caller and
/// before the spawn goes to the new `ravel` at the first poll. At each poll the
/// function passes the foreground on, sends each latched signal to `ravel`
/// alone, and checks for an exit without blocking. It never signals a process
/// group. It waits until `ravel` exits, however long that takes.
///
/// # Errors
/// A command that cannot be spawned, a file that is not regular, or a wait that
/// fails. Each error carries the interruption: see [`JobError`].
pub fn launch(
    command: &Argv,
    cwd: &Path,
    stdout: File,
    observer: &mut dyn FnMut(LaunchEvent),
    shape: Shape<'_>,
) -> Result<JobEnd, JobError> {
    let sigchld_ignored = *ENTRY.get_or_init(enter);
    start_and_watch(command, cwd, stdout, observer, shape, sigchld_ignored).map_err(|failure| {
        JobError {
            message: failure.message,
            interrupted: failure.sent.or_else(take_interrupt),
        }
    })
}

fn start_and_watch(
    command: &Argv,
    cwd: &Path,
    stdout: File,
    observer: &mut dyn FnMut(LaunchEvent),
    shape: Shape<'_>,
    sigchld_ignored: bool,
) -> Result<JobEnd, Failure> {
    ensure_regular(&stdout, "stdout")?;
    let detached = matches!(shape, Shape::Detached { .. });
    let mut job = Command::new(command.program());
    job.args(command.args()).current_dir(cwd).stdout(stdout);
    // Scrub first and grant second. The order is load-bearing: the scrub list
    // names `GROVE_LAUNCH_DIR`, and a grant before it would remove the path
    // that this job is meant to receive.
    scrub(&mut job);
    let mut lease = match shape {
        Shape::Foreground { launch_dir } => {
            job.env("GROVE_LAUNCH_DIR", launch_dir);
            Some(Lease {
                terminal: Terminal::open(),
                child: 0,
                held: false,
                saved: None,
            })
        }
        Shape::Detached { transcript } => {
            ensure_regular(&transcript, "transcript")?;
            job.stdin(Stdio::null()).stderr(transcript);
            None
        }
    };
    // Hand the terminal over from inside the child as well as from the parent
    // below. The handover of the parent waits on `spawn`, and nothing orders the
    // first read of the child after it. A read from a background group is what
    // SIGTTIN stops. Holding the foreground now is also when the modes are
    // saved, before a child can set the terminal raw.
    let handover = lease.as_mut().and_then(Lease::hold_before_spawn);
    let descriptor_limit = if detached { descriptor_limit()? } else { 3 };
    if !detached {
        // Through `std`, which runs it before the `pre_exec` callback and
        // reports a failure as a failed spawn.
        job.process_group(0);
    }
    // SAFETY: the closure runs between `fork` and `exec`, so it may call only
    // async-signal-safe functions. `fcntl`, `setsid`, `signal`, `getpid` and
    // `tcsetpgrp` (an `ioctl`) are all on the list of POSIX. The closure
    // allocates nothing, takes no lock and touches no state of the Rust runtime.
    unsafe {
        job.pre_exec(move || {
            if detached {
                // Close on exec, not here: the error pipe of `std` must survive
                // until the exec succeeds, and it is close-on-exec too.
                for fd in 3..descriptor_limit {
                    if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1 {
                        let error = std::io::Error::last_os_error();
                        if error.raw_os_error() != Some(libc::EBADF) {
                            return Err(error);
                        }
                    }
                }
                // `setsid` creates the session and the group. It must come
                // before the process becomes a group leader.
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            // Until the `tcsetpgrp` below returns, this process is a background
            // group that touches the terminal. That is what SIGTTOU is raised
            // for. The loop that follows puts the default back.
            libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            if let Some(fd) = handover {
                libc::tcsetpgrp(fd, libc::getpid());
            }
            for signal in DEFAULT_DISPOSITION_IN_CHILD {
                libc::signal(signal, libc::SIG_DFL);
            }
            // Grove repaired an inherited ignored SIGCHLD for itself alone. The
            // child gets the disposition that it would have inherited.
            if sigchld_ignored {
                libc::signal(libc::SIGCHLD, libc::SIG_IGN);
            }
            Ok(())
        });
    }
    let mut child = job.spawn().map_err(|error| {
        Failure::new(format!(
            "cannot spawn {:?}: {error}; check that the program exists and is executable",
            command.program()
        ))
    })?;
    observer(LaunchEvent::Started);
    let pid = child_pid(&child);
    if !detached {
        // The parent's half of the same `setpgid`. It is insurance and not a
        // race: `spawn` returns only after the exec, so the call is measured to
        // fail with EACCES. That ordering is undocumented, so the call stays.
        // SAFETY: `setpgid(2)` naming a child of this process.
        unsafe { libc::setpgid(pid, pid) };
    }
    if let Some(lease) = lease.as_mut() {
        lease.child = pid;
    }
    watch(&mut child, lease.as_mut(), observer)
}

fn child_pid(child: &Child) -> libc::pid_t {
    // The cast cannot truncate: `std` took this value from a `pid_t`.
    child.id() as libc::pid_t
}

/// Watch `ravel` to its exit.
fn watch(
    child: &mut Child,
    mut lease: Option<&mut Lease>,
    observer: &mut dyn FnMut(LaunchEvent),
) -> Result<JobEnd, Failure> {
    let pid = child_pid(child);
    let started = Instant::now();
    let mut interrupted = None;
    loop {
        if let Some(lease) = lease.as_deref_mut() {
            lease.lend();
        }
        if let Some(signal) = take_interrupt() {
            interrupted.get_or_insert(signal);
            // SAFETY: `kill(2)` on a child of this process that is not reaped.
            // An unreaped child keeps its ID, so the signal cannot reach
            // another process. A failure is ignored: ESRCH means that the exit
            // came between the poll and the signal, and the check below sees it.
            unsafe { libc::kill(pid, signal) };
        }
        // `try_wait` reports an exit and never a stop. A stopped child is not
        // an exit.
        match child.try_wait() {
            Ok(Some(status)) => {
                let elapsed = started.elapsed();
                // Freeze the job at the reap, before the observer or the
                // terminal recovery can delay it. Later signals are the caller's.
                let interrupted = interrupted.or_else(take_interrupt);
                observer(LaunchEvent::Reaped);
                if let Some(lease) = lease {
                    lease.reclaim(Some(status));
                }
                return Ok(JobEnd {
                    status,
                    interrupted,
                    elapsed,
                });
            }
            Ok(None) => std::thread::sleep(POLL_INTERVAL),
            Err(error) => {
                // The status is unknown. Grove sends no KILL: Ravel alone gives
                // the cleanup assurance. The terminal comes back by the narrow
                // rule, and the error names the process.
                if let Some(lease) = lease {
                    lease.reclaim(None);
                }
                return Err(Failure {
                    message: format!(
                        "cannot wait on ravel (process {pid}): {error}; it was not killed, \
                         so check for a running process"
                    ),
                    sent: interrupted,
                });
            }
        }
    }
}

fn ensure_regular(file: &File, name: &str) -> Result<(), Failure> {
    let metadata = file
        .metadata()
        .map_err(|error| Failure::new(format!("cannot inspect the job {name}: {error}")))?;
    if metadata.is_file() {
        Ok(())
    } else {
        Err(Failure::new(format!(
            "the job {name} must be a regular file"
        )))
    }
}

/// Include existing descriptors even if the caller lowered its soft limit after
/// opening them. The range then covers the descriptors that `std` opens while it
/// prepares the child. A detached job must not start while another thread
/// changes the descriptor limit.
fn descriptor_limit() -> Result<RawFd, Failure> {
    // SAFETY: `getrlimit` writes the initialised structure only.
    let mut limit: libc::rlimit = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } != 0 {
        return Err(Failure::new(format!(
            "cannot inspect the descriptor limit: {}",
            std::io::Error::last_os_error()
        )));
    }
    let mut maximum = i32::try_from(limit.rlim_cur).map_err(|_| {
        Failure::new("cannot isolate an unbounded descriptor table; set a finite open-file limit")
    })?;
    let directory = if cfg!(target_os = "linux") {
        "/proc/self/fd"
    } else {
        "/dev/fd"
    };
    for entry in std::fs::read_dir(directory)
        .map_err(|error| Failure::new(format!("cannot inspect inherited descriptors: {error}")))?
    {
        let entry = entry.map_err(|error| Failure::new(error.to_string()))?;
        if let Some(fd) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<i32>().ok())
        {
            maximum = maximum.max(fd.saturating_add(1));
        }
    }
    Ok(maximum)
}

/// The signal that the handler last received, or `0`.
///
/// The state is process-global because a signal disposition is process-global.
/// The latch holds the number and not a flag, because the ending is reported
/// onward. A driver that re-raises TERM for a HUP has told its parent the wrong
/// thing.
///
/// **The function never clears the latch before the spawn.** A signal that
/// arrived while no child existed belongs to the driver, and [`take_interrupt`]
/// is where the driver collects it. A signal that arrives after the last check of
/// the driver and before the spawn goes to the new `ravel` at the first poll.
static INTERRUPTED_BY: AtomicI32 = AtomicI32::new(0);

/// Whether Grove inherited SIGCHLD ignored, read at the first call.
///
/// The first call also installs the handlers. The state is read once because
/// the repair below changes the disposition, and every later read would see the
/// repair.
static ENTRY: OnceLock<bool> = OnceLock::new();

/// Which signal, if any, reached this process since the last check. The call
/// clears the latch.
///
/// A looping driver calls this at the top of each iteration and after each reap.
/// It answers `None` before the first call of [`launch`], because no handler
/// exists yet and the signal took its default action.
#[must_use]
pub fn take_interrupt() -> Option<i32> {
    match INTERRUPTED_BY.swap(0, Ordering::Relaxed) {
        0 => None,
        signal => Some(signal),
    }
}

/// A single store is the only work here that is async-signal-safe. The signal
/// reaches `ravel` one poll later, on the ordinary stack of [`launch`].
extern "C" fn on_signal(signal: libc::c_int) {
    INTERRUPTED_BY.store(signal, Ordering::Relaxed);
}

/// Catch INT, TERM and HUP, then repair an inherited ignored SIGCHLD. Answer
/// whether SIGCHLD was ignored.
///
/// **A disposition that Grove ignores stays ignored.** Ignoring is a statement
/// that Grove made. The driver ignores SIGINT so that a typed Ctrl-C never ends
/// it. A handler over that ignore would turn the signal into a cancellation.
///
/// **An ignored SIGCHLD makes the kernel reap children unwatched.** Then
/// nothing is left for the wait to observe. Grove restores the default for
/// itself, with no flags and never a handler: a handler would turn the exit of
/// each child into EINTR on whatever Grove is doing.
fn enter() -> bool {
    // Through the function pointer, as rustc requires for the cast.
    let handler = on_signal as extern "C" fn(libc::c_int) as usize;
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        if disposition(signal) != libc::SIG_IGN {
            // SAFETY: `signal(2)` with a handler that performs one relaxed
            // atomic store.
            unsafe { libc::signal(signal, handler as libc::sighandler_t) };
        }
    }
    let ignored = disposition(libc::SIGCHLD) == libc::SIG_IGN;
    if ignored {
        // SAFETY: `sigaction(2)` installing the default with an empty mask and
        // no flags, which clears SA_NOCLDWAIT with it.
        unsafe {
            let mut default: libc::sigaction = std::mem::zeroed();
            default.sa_sigaction = libc::SIG_DFL;
            libc::sigemptyset(&mut default.sa_mask);
            libc::sigaction(libc::SIGCHLD, &default, std::ptr::null_mut());
        }
    }
    ignored
}

/// The current disposition of `signal`, read without changing it.
fn disposition(signal: libc::c_int) -> libc::sighandler_t {
    // SAFETY: `sigaction(2)` with a null new action only reads the current one
    // into an initialised structure.
    unsafe {
        let mut current: libc::sigaction = std::mem::zeroed();
        libc::sigaction(signal, std::ptr::null(), &mut current);
        current.sa_sigaction
    }
}

/// Die of the signal that ended this process, so that its parent sees `128 + N`
/// in the wait status.
///
/// A process that catches TERM, tidies up and exits 0 tells its parent that it
/// finished its work. An exit code cannot say that a signal ended the process.
/// Only a wait status can, and the only way to produce one is to die of the
/// signal. So the default disposition returns, the signal is unblocked in case
/// the handler left it masked, and the signal is raised.
///
/// **This module owns the call because this module installed the handler.** What
/// stays with the caller is whether to re-raise at all. That is a statement about
/// the exit status of the caller.
pub fn reraise(signal: i32) -> ! {
    // A death by signal loses buffered output as `process::exit` does, and the
    // last diagnostic before a termination is the one that a reader wants most.
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();

    // SAFETY: restoring the default disposition of a signal that this module set
    // a handler for, unblocking that one signal and raising it against this
    // process. The set is initialised by `sigemptyset` before use.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        let mut unblock: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut unblock);
        libc::sigaddset(&mut unblock, signal);
        libc::sigprocmask(libc::SIG_UNBLOCK, &unblock, std::ptr::null_mut());
        libc::raise(signal);
    }

    // Unreachable for a signal whose default action terminates, which is every
    // signal that interrupts a driver. A caller that passes SIGCHLD or SIGURG
    // gets the conventional code and not a fall-through to code that expected
    // this call to diverge.
    std::process::exit(128 + signal)
}

/// The signals that the child gets at their **default** disposition.
///
/// **Only an ignored disposition survives `execve`.** POSIX resets a caught
/// handler across an exec and leaves an ignore in place. The driver ignores
/// SIGINT for its own reasons, and without this list `ravel` would inherit the
/// ignore. Ravel passes its entry signal state to the harness, so the ignore
/// would reach the harness and every login shell or `ssh` hop in between. An
/// interactive session under them could not be interrupted, and nothing in it
/// could say why.
const DEFAULT_DISPOSITION_IN_CHILD: [libc::c_int; 7] = [
    libc::SIGINT,
    libc::SIGQUIT,
    libc::SIGTERM,
    libc::SIGHUP,
    libc::SIGTSTP,
    libc::SIGTTIN,
    libc::SIGTTOU,
];

/// The controlling terminal of Grove, open for as long as a job needs to hand it
/// over and take it back.
struct Terminal(OwnedFd);

impl Terminal {
    /// `/dev/tty` and not standard input, and the difference is the gate.
    ///
    /// `/dev/tty` is the controlling terminal by definition. A process whose
    /// standard input was redirected still hands over the right device. A
    /// process with no controlling terminal, such as a test runner, a CI job or a
    /// daemon, cannot open it and gets no job control. There is no flag to set
    /// and nothing for a caller to configure wrongly.
    fn open() -> Option<Self> {
        // SAFETY: `open(2)` on a constant NUL-terminated path. The descriptor is
        // owned from here on and closed by `OwnedFd`.
        let fd = unsafe {
            libc::open(
                c"/dev/tty".as_ptr(),
                libc::O_RDWR | libc::O_NOCTTY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return None;
        }
        // SAFETY: a fresh descriptor that this process just opened and has not
        // handed to anything else.
        Some(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }

    fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    /// The process group that owns the terminal now, or `-1`.
    fn foreground(&self) -> libc::pid_t {
        // SAFETY: `tcgetpgrp(3)` on a descriptor that this structure owns.
        unsafe { libc::tcgetpgrp(self.fd()) }
    }

    /// Make `pgid` the foreground group of the terminal.
    ///
    /// **`tcsetpgrp` from a group that is not the foreground group raises
    /// SIGTTOU at the caller**, and the default action stops it. A process that
    /// reclaims the terminal from its child would stop itself in the act. Ignore
    /// SIGTTOU across the call and restore the previous disposition after it.
    /// That is the standard dance of job control, and it is why this is a method
    /// and not a bare call at three sites.
    fn hand_to(&self, pgid: libc::pid_t) {
        // SAFETY: `signal(2)` and `tcsetpgrp(3)` on a descriptor that this
        // structure owns. The previous disposition returns before this function
        // does.
        unsafe {
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::tcsetpgrp(self.fd(), pgid);
            libc::signal(libc::SIGTTOU, previous);
        }
    }

    /// The attributes of the terminal now, or `None` when they cannot be read.
    fn attributes(&self) -> Option<libc::termios> {
        // SAFETY: `tcgetattr(3)` fills an initialised structure from a
        // descriptor that this structure owns.
        unsafe {
            let mut attributes: libc::termios = std::mem::zeroed();
            (libc::tcgetattr(self.fd(), &mut attributes) == 0).then_some(attributes)
        }
    }

    /// Put back attributes that were saved earlier: cooked mode and echo, for a
    /// child that set the terminal raw and ended before it could undo that.
    ///
    /// SIGTTOU is ignored across the call, for the reason in [`Terminal::hand_to`].
    fn restore(&self, attributes: &libc::termios) {
        // SAFETY: `signal(2)` and `tcsetattr(3)` on a descriptor that this
        // structure owns, with attributes that `tcgetattr` produced. The previous
        // disposition returns before this function does.
        unsafe {
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::tcsetattr(self.fd(), libc::TCSANOW, attributes);
            libc::signal(libc::SIGTTOU, previous);
        }
    }
}

/// The process group of this process.
fn own_group() -> libc::pid_t {
    // SAFETY: `getpgrp(2)` takes no argument and cannot fail.
    unsafe { libc::getpgrp() }
}

/// The share of the terminal that one foreground job holds: when it was held,
/// what it looked like then, and how it returns.
///
/// **Having held the foreground is the gate for everything on the way back.**
/// A process in the foreground lends the terminal to its child and owes the
/// human a terminal back. A process that never held it lent nothing. It takes
/// nothing and restores nothing, because anything else would steal the terminal
/// from the job that holds it.
struct Lease {
    terminal: Option<Terminal>,
    child: libc::pid_t,
    /// Whether the job has held the foreground at any moment so far.
    held: bool,
    /// The attributes at the moment the job first held the foreground, when they
    /// could be read.
    saved: Option<libc::termios>,
}

impl Lease {
    /// Before the spawn: whether Grove holds the foreground now. If it does,
    /// answer the descriptor that the child hands itself the terminal through.
    fn hold_before_spawn(&mut self) -> Option<RawFd> {
        let terminal = self.terminal.as_ref()?;
        if terminal.foreground() != own_group() {
            return None;
        }
        self.held = true;
        self.saved = terminal.attributes();
        Some(terminal.fd())
    }

    /// Called at each poll. The check repeats each time, so a driver that started
    /// in the background and later came forward (`grove &`, then `fg`) hands the
    /// terminal to the job that runs under it. It saves the modes only then.
    fn lend(&mut self) {
        let Some(terminal) = &self.terminal else {
            return;
        };
        if terminal.foreground() != own_group() {
            return;
        }
        if !self.held {
            self.held = true;
            self.saved = terminal.attributes();
        }
        terminal.hand_to(self.child);
    }

    /// Take the terminal back after the reap, and restore it.
    ///
    /// **Three rules decide from whom.**
    ///
    /// - After an ordinary exit, take it only from the group of `ravel`. That
    ///   group is gone by now, but the terminal still names it until another
    ///   group receives it. A group that took the terminal on purpose, such as a
    ///   job-control shell nested in this session, keeps it.
    /// - After a death by signal, take it from any group except this one and
    ///   the session leader's. A supervisor that died can leave the group of its
    ///   own child holding the terminal, an orphan in the foreground that reads
    ///   the input of the human. Only this rule takes the terminal back.
    /// - After a failed wait the status is unknown. Take it only from the group
    ///   of `ravel`.
    ///
    /// The attributes return whenever this group ends in the foreground, so a
    /// raw-mode harness that was killed still leaves cooked mode behind.
    fn reclaim(&self, status: Option<ExitStatus>) {
        use std::os::unix::process::ExitStatusExt as _;
        let Some(terminal) = self.terminal.as_ref().filter(|_| self.held) else {
            return;
        };
        let holder = terminal.foreground();
        let own = own_group();
        let take = if status.and_then(|status| status.signal()).is_some() {
            // SAFETY: `getsid(2)` on this process, which cannot fail.
            let leader = unsafe { libc::getsid(0) };
            holder > 0 && holder != own && holder != leader
        } else {
            holder == self.child
        };
        if take {
            terminal.hand_to(own);
        }
        if let Some(saved) = self.saved.as_ref().filter(|_| terminal.foreground() == own) {
            terminal.restore(saved);
        }
    }
}
