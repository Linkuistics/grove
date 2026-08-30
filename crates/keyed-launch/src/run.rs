//! Spawning one child directly and supervising it until it ends.

use std::ffi::OsStr;
use std::io::Write as _;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::{Child, Command, ExitStatus};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};

use crate::channel::{Channel, Token};
use crate::error::LaunchError;

/// How long the supervisor waits between checks of the child's liveness and the
/// completion channel.
///
/// Not a knob. The interval only bounds how late an escalation starts, and the
/// escalation's own graces are measured in seconds — a caller tuning this would
/// be tuning latency it cannot observe.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

/// The two waits of the kill escalation.
///
/// **The escalation exists because an interactive child is never reaped on its
/// own.** A child that returns to a prompt after finishing its work has not
/// exited and will not: it sits waiting for input that is not coming. The token
/// is the only evidence it is done, and ending it is therefore the launcher's
/// job — which is a job only the launcher can do, since it is the child's own
/// parent process, outside whatever sandbox the child runs under. A child asked
/// to end itself may simply be denied (macOS Seatbelt refuses a same-sandbox
/// process signalling its own session), and denied silently.
///
/// `grace` runs from the token's appearance to SIGTERM, so a child that
/// signalled mid-operation gets to finish that operation and let its own call
/// return. `kill_grace` runs from SIGTERM to SIGKILL, for a child that installs
/// a handler and declines to die.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Escalation {
    pub grace: Duration,
    pub kill_grace: Duration,
}

/// Everything one launch is.
///
/// Every field is the caller's: this crate supplies no default program, no
/// default environment, and no default variable name. What it supplies is that
/// the `argv` is spawned **whole and directly** — no shell, no appended
/// argument, no reordering — and that the child's environment is the caller's
/// own minus `scrub` plus the one channel path.
pub struct Launch<'a> {
    /// The program and arguments, built only by
    /// [`Templates::expand`](crate::Templates::expand), so nothing reaches a
    /// spawn that a template did not author.
    pub argv: &'a crate::Argv,
    /// This launch's completion channel. Its path is published to the child;
    /// its appearance ends the launch.
    pub channel: &'a Channel,
    /// The environment variable the channel path is published under. The name
    /// is the caller's because the child is the caller's: only the two of them
    /// have agreed on it.
    pub channel_var: &'a str,
    /// Variable names removed from the child's inherited environment.
    ///
    /// **Scrubbing is the caller's obligation and this is where it is
    /// discharged.** An environment is inherited, not addressed: a launcher
    /// that merely declines to *set* its own control variables still hands the
    /// child whatever its own environment carried — including, for a nested
    /// launcher, a live channel path belonging to somebody else's launch, which
    /// is authority to end a session nobody meant to grant.
    pub scrub: &'a [&'a OsStr],
    /// The child's working directory. `None` inherits the launcher's, which is
    /// rarely what a launcher wants: it is wherever a human happened to be
    /// standing.
    pub cwd: Option<&'a Path>,
    pub escalation: Escalation,
}

/// How a launch ended.
///
/// `Signalled` and `Exited` both describe a child that is gone; they differ in
/// *who ended it*, which is what a caller needs to distinguish a launch that
/// completed its work from one that fell over. `token` is orthogonal to all
/// three: a child that signals and then exits before the grace elapses ends
/// `Exited` with a token, and is a perfectly ordinary completion.
#[derive(Debug)]
pub struct Ended {
    pub end: End,
    pub status: ExitStatus,
    pub elapsed: Duration,
    pub token: Option<Token>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    /// The child exited of its own accord — whether or not it left a token on
    /// the way out.
    Exited,
    /// The escalation ended the child: its token appeared, the grace elapsed
    /// with the child still running, and it was signalled.
    ///
    /// This is deliberately *narrower* than "a token appeared", which `token`
    /// already reports. A child that signals and then exits inside its own
    /// grace was never touched, and comes back `Exited` with a token.
    Signalled,
    /// The *launcher's* process was sent SIGTERM or SIGHUP **during this
    /// launch**. The child's process group was sent the same signal and reaped
    /// through the ordinary escalation, so it is never left orphaned onto the
    /// terminal. The channel cannot express this case — an interrupt normally
    /// leaves no token at all.
    ///
    /// **The signal is carried rather than merely noted** so a launcher can
    /// report it onward. A process that catches a termination signal, tidies up
    /// and then exits 0 tells its own parent it finished its work; the only way
    /// to say what actually happened is to die of the same signal, and that
    /// needs its number. [`reraise`] is that ending, and this field is its
    /// argument.
    ///
    /// A signal arriving *between* launches is not this: `run` discards it, and
    /// [`take_interrupt`] is where a looping launcher collects it.
    Interrupted { signal: i32 },
}

/// The supervisor's state machine: idle until the token appears, then timed
/// toward SIGTERM and finally SIGKILL.
enum Watch {
    Running,
    Signalled(Instant),
    Terminated(Instant),
}

/// The signal [`on_terminate`] last received, or `0`, read by [`run`]'s poll
/// loop.
///
/// Process-global because a signal disposition is process-global, and latched
/// because the launch on which the child finally exits still has to report it.
/// The *number* rather than a flag, because the ending is reported onward and a
/// launcher that re-raises SIGTERM for a SIGHUP has told its parent the wrong
/// thing.
///
/// **A latch that outlives its launch is a loaded gun**, and this one is scoped
/// to exactly one launch at both ends. [`run`] clears it immediately before
/// spawning, so a signal that arrived while no child existed can never be
/// spent on a fresh child that has not signalled and has done nothing wrong;
/// [`take_interrupt`] lets a launcher consume it between launches, which is
/// where such a signal actually belongs. Without the clear, a driver signalled
/// in the gap between two iterations starts the next session and SIGTERMs it on
/// its first poll.
static INTERRUPTED_BY: AtomicI32 = AtomicI32::new(0);

/// Which signal, if any, was sent to this process outside a launch — clearing
/// the latch.
///
/// For a launcher that runs launches in a loop: [`run`] reports a signal that
/// arrives *during* a launch as [`End::Interrupted`], but one arriving between
/// two launches has no launch to be reported against, and `run` deliberately
/// discards it rather than spending it on the next child. Call this at the top
/// of the loop to honour it instead.
///
/// It answers `None` before this process's first [`run`], because nothing has
/// installed a handler yet and the signal took its default disposition.
#[must_use]
pub fn take_interrupt() -> Option<i32> {
    match INTERRUPTED_BY.swap(0, Ordering::Relaxed) {
        0 => None,
        signal => Some(signal),
    }
}

/// Die of the signal that ended this launcher, so **its** parent sees the
/// conventional `128 + N` in the wait status.
///
/// A process that catches SIGTERM, cleans up and exits 0 has told a systemd
/// unit, a `timeout(1)` or a shell `wait` that it finished its work. An exit
/// *code* cannot express "was signalled" at all — only a wait status can, and
/// the only way to produce one is to actually die of the signal. So the
/// disposition this crate installed is put back to the default, the signal is
/// unblocked in case it is still masked from the handler that ran, and it is
/// raised.
///
/// **This crate owns the call because this crate installed the handler.** A
/// consumer undoing it would be reaching for a disposition it did not set and
/// cannot see, and would get it wrong for a signal `run` starts catching later.
/// What stays the consumer's is *whether* to re-raise, which is a statement
/// about that process's own exit status.
pub fn reraise(signal: i32) -> ! {
    // Buffered output is lost by a signal death the way it is lost by
    // `process::exit`, and the last diagnostic before a termination is the one
    // a reader most wants.
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();

    // SAFETY: restoring the default disposition of a signal this crate set a
    // handler for, unblocking that one signal, and raising it against this
    // process. `sigset_t` is initialised by `sigemptyset` before use.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        let mut unblock: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut unblock);
        libc::sigaddset(&mut unblock, signal);
        libc::sigprocmask(libc::SIG_UNBLOCK, &unblock, std::ptr::null_mut());
        libc::raise(signal);
    }

    // Unreachable for any signal whose default action terminates, which is
    // every signal a launcher is interrupted by. If a caller passes one that
    // does not — SIGCHLD, SIGURG — saying so in the exit code is still better
    // than falling through to whatever the caller does after an infallible
    // call it believed diverged.
    std::process::exit(128 + signal)
}

/// A single store is the *only* work done here, because it is the only work
/// that is async-signal-safe. Signalling and reaping the child happen one poll
/// tick later, on [`run`]'s ordinary stack.
extern "C" fn on_terminate(signal: libc::c_int) {
    INTERRUPTED_BY.store(signal, Ordering::Relaxed);
}

/// Catch SIGTERM and SIGHUP so a launcher can forward termination to its child
/// and reap it rather than orphan it.
///
/// Installed by [`run`] rather than exported, because [`End::Interrupted`] is a
/// promise this crate makes and a caller cannot be relied on to have enabled
/// it. Re-installing the same handler is idempotent, so calling it once per
/// launch costs nothing.
///
/// SIGINT is deliberately absent: Ctrl-C is delivered to the terminal's
/// foreground process group, which — once [`run`] has handed the terminal over
/// — is the child's and not the launcher's. What a launcher does about a SIGINT
/// it does receive is its policy, not this crate's.
fn install_termination_handler() {
    // Through the function *pointer* rather than casting the function item
    // straight to an integer, which rustc warns about: a function item is
    // zero-sized and the cast reads as a value conversion rather than the
    // address-taking it is.
    let handler = on_terminate as extern "C" fn(libc::c_int) as usize;
    // SAFETY: `signal(2)` with a handler that performs one relaxed atomic store.
    unsafe {
        libc::signal(libc::SIGTERM, handler as libc::sighandler_t);
        libc::signal(libc::SIGHUP, handler as libc::sighandler_t);
    }
}

/// The signals the child is handed back at their **default** disposition.
///
/// **Only an *ignored* disposition survives `execve`.** POSIX resets a caught
/// handler to the default across an exec and leaves an ignore in place, and
/// `std::process::Command` restores exactly one thing on top of that — SIGPIPE,
/// which Rust ignores process-wide at start-up. So this list is not about
/// handlers, which take care of themselves. It is about a launcher that
/// *ignores* one of these for its own reasons and would otherwise hand the
/// ignore to its child, to that child's children, and to every wrapper in
/// between: a login shell or an `ssh` hop that inherits an ignored SIGINT keeps
/// ignoring it *and propagates it onward*, so an interactive session under one
/// cannot be interrupted at all and nothing in it can tell why.
///
/// A launcher that wanted a child to inherit an ignore has to say so some other
/// way. That is the right default for this crate, whose child owns a terminal:
/// a terminal-generated signal the human types must reach the process the
/// human is looking at.
const DEFAULT_DISPOSITION_IN_CHILD: [libc::c_int; 7] = [
    libc::SIGINT,
    libc::SIGQUIT,
    libc::SIGTERM,
    libc::SIGHUP,
    libc::SIGTSTP,
    libc::SIGTTIN,
    libc::SIGTTOU,
];

/// The launcher's controlling terminal, open for as long as a launch needs to
/// hand it back and forth.
struct Terminal(OwnedFd);

impl Terminal {
    /// `/dev/tty` rather than stdin, and the difference is the gate.
    ///
    /// `/dev/tty` *is* the controlling terminal by definition, so a launcher
    /// whose stdin was redirected still hands over the right device — and a
    /// launcher that has no controlling terminal at all (a test runner, a CI
    /// job, a daemon) simply fails to open it and gets no job control. There is
    /// no flag to set and nothing for a caller to configure wrongly.
    fn open() -> Option<Self> {
        // SAFETY: `open(2)` against a constant NUL-terminated path. The
        // returned descriptor is owned from here on and closed by `OwnedFd`.
        let fd = unsafe {
            libc::open(
                c"/dev/tty".as_ptr(),
                libc::O_RDWR | libc::O_NOCTTY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return None;
        }
        // SAFETY: a fresh descriptor this process just opened and has not
        // handed to anything else.
        Some(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }

    fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    /// Which process group currently owns the terminal, or `-1`.
    fn foreground(&self) -> libc::pid_t {
        // SAFETY: `tcgetpgrp(3)` on a descriptor this struct owns.
        unsafe { libc::tcgetpgrp(self.fd()) }
    }

    /// Make `pgid` the terminal's foreground process group.
    ///
    /// **`tcsetpgrp` from a group that is not already the foreground one raises
    /// SIGTTOU at the caller**, whose default action stops it — so a launcher
    /// reclaiming the terminal from its child would stop itself in the act of
    /// taking it back. Ignoring SIGTTOU across the call and restoring the
    /// previous disposition afterwards is the standard job-control dance, and
    /// it is why this is a method rather than a bare call at three sites.
    fn hand_to(&self, pgid: libc::pid_t) {
        // SAFETY: `signal(2)` and `tcsetpgrp(3)` on a descriptor this struct
        // owns; the previous disposition is restored before returning.
        unsafe {
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::tcsetpgrp(self.fd(), pgid);
            libc::signal(libc::SIGTTOU, previous);
        }
    }
}

/// This process's own process group.
fn own_group() -> libc::pid_t {
    // SAFETY: `getpgrp(2)` takes no argument and cannot fail.
    unsafe { libc::getpgrp() }
}

/// Spawn `launch`'s argv directly and supervise the child until it ends.
///
/// The child's environment is the launcher's, minus [`Launch::scrub`], plus the
/// channel path under [`Launch::channel_var`]. Nothing else is added: no
/// argument, no flag, no variable. A child that needs one says so in its own
/// template, where whoever wrote the configuration can see it.
///
/// **The child is a job, not just a process.** It is put in a process group of
/// its own and — when this launcher owns a controlling terminal and is the
/// foreground group of it — handed that terminal, exactly as a shell does for a
/// foreground job. Two things follow, and both are the point. A terminal signal
/// the human types reaches *the child's* group rather than the launcher's, so
/// the launcher survives a Ctrl-C it never has to catch; and the escalation can
/// signal the whole group, so a grandchild the child spawned is reaped with it
/// rather than left running and attached to the terminal. The child's group is
/// *not* a new session: a session leader has no controlling terminal, which is
/// what would put an interactive child in a background group and stop it with
/// SIGTTIN on its first read.
///
/// The child's signal dispositions are the defaults, whatever the launcher's
/// are — see [`DEFAULT_DISPOSITION_IN_CHILD`].
///
/// Supervision polls three things, and they are the only three ways a launch
/// ends: the child exits, the token appears, or the launcher itself is
/// signalled. **A child that finishes its work and never signals reaches none
/// of them** — an interactive one returns to its prompt instead of exiting, so
/// the launch *stalls* rather than ending. That is a real failure mode with no
/// cheap fix here: nothing this crate can observe distinguishes a child that
/// forgot to signal from one still working, so a second completion observable
/// would only trade a stall for a wrong kill. It is the caller's to close, at
/// the layer that instructs the child.
pub fn run(launch: Launch<'_>) -> Result<Ended, LaunchError> {
    install_termination_handler();

    let mut command = Command::new(launch.argv.program());
    command.args(launch.argv.args());
    if let Some(cwd) = launch.cwd {
        command.current_dir(cwd);
    }
    // Scrub first, grant second, and the order is load-bearing rather than
    // stylistic: a caller whose scrub list *contains* its own `channel_var` is
    // the expected shape, not a mistake — the list names the launch-control
    // variables a nested launcher must not inherit, and the channel variable is
    // the first of them. Granting before scrubbing would remove the path this
    // launch just published and leave the child unable to signal, which reads
    // as a session that hung. `tests/launch.rs` pins it.
    for name in launch.scrub {
        command.env_remove(name);
    }
    command.env(launch.channel_var, launch.channel.path());

    let terminal = Terminal::open();
    // Hand the terminal over from *inside* the child as well as from the parent
    // below, because either one alone leaves a window: the parent can reach
    // `tcsetpgrp` before the child's `setpgid` has created the group, and the
    // child can reach its first read before the parent has handed anything
    // over. Only when this launcher is the terminal's current owner — handing
    // over a terminal owned by somebody else's job is theft, not job control.
    let handover_fd = terminal
        .as_ref()
        .filter(|terminal| terminal.foreground() == own_group())
        .map(Terminal::fd);

    // The group, through `std`'s own checked path rather than a `setpgid` of our
    // own: it runs it before the `pre_exec` callbacks below and reports a
    // failure as a failed spawn, which a raw call in the closure could only do
    // by hand.
    command.process_group(0);

    // SAFETY: the closure runs between `fork` and `exec`, so it may call only
    // async-signal-safe functions. `signal`, `getpid` and `tcsetpgrp` (an
    // `ioctl`) are all on POSIX's list; nothing here allocates, locks, or
    // touches Rust runtime state. `std` itself resets only SIGPIPE across a
    // spawn and inherits the signal mask, so everything below is work nothing
    // else is doing.
    unsafe {
        command.pre_exec(move || {
            // Until the `tcsetpgrp` below returns, this process is a background
            // group touching the terminal, which is precisely what SIGTTOU is
            // raised for. The loop that follows puts the disposition back.
            libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            if let Some(fd) = handover_fd {
                libc::tcsetpgrp(fd, libc::getpid());
            }
            for signal in DEFAULT_DISPOSITION_IN_CHILD {
                libc::signal(signal, libc::SIG_DFL);
            }
            Ok(())
        });
    }

    // Clear the latch *before* the spawn, never after: see `INTERRUPTED_BY`. A
    // signal that arrived while no child existed is the launcher's to handle
    // through `take_interrupt`, and is not evidence about the child below.
    INTERRUPTED_BY.store(0, Ordering::Relaxed);

    let child = command.spawn().map_err(|error| {
        LaunchError::new(format!(
            "cannot spawn {:?}: {error}; check that the program exists and is executable",
            launch.argv.program()
        ))
    })?;

    // The parent's half of the same `setpgid`. Whichever side runs first wins
    // and the other fails harmlessly — EACCES once the child has exec'd, ESRCH
    // once it has exited — and doing it on both sides is what closes the window
    // in which the parent could signal a group that does not exist yet.
    let pgid = child.id() as libc::pid_t;
    // SAFETY: `setpgid(2)` naming this process's own child.
    unsafe { libc::setpgid(pgid, pgid) };

    supervise(child, launch.channel, launch.escalation, terminal, pgid)
}

fn supervise(
    child: Child,
    channel: &Channel,
    escalation: Escalation,
    terminal: Option<Terminal>,
    pgid: libc::pid_t,
) -> Result<Ended, LaunchError> {
    let outcome = watch(child, channel, escalation, terminal.as_ref(), pgid);
    // Take the terminal back, and only from the job this launch owned. A
    // launcher that returned while the terminal still belonged to a dead group
    // would leave its own next write — `stty`, a diagnostic — to a terminal it
    // is a background job on, which is a SIGTTOU stop rather than an error
    // anybody could read.
    if let Some(terminal) = &terminal {
        if terminal.foreground() == pgid {
            terminal.hand_to(own_group());
        }
    }
    outcome
}

fn watch(
    mut child: Child,
    channel: &Channel,
    escalation: Escalation,
    terminal: Option<&Terminal>,
    pgid: libc::pid_t,
) -> Result<Ended, LaunchError> {
    let started = Instant::now();
    let mut watch = Watch::Running;
    let mut interrupted: Option<i32> = None;
    let mut signalled = false;

    let ended = |status: ExitStatus, interrupted: Option<i32>, signalled: bool| Ended {
        end: match (interrupted, signalled) {
            (Some(signal), _) => End::Interrupted { signal },
            (None, true) => End::Signalled,
            (None, false) => End::Exited,
        },
        status,
        elapsed: started.elapsed(),
        // Read after the child is gone, so a child still mid-write cannot be
        // observed half-signalled.
        token: channel.read(),
    };

    loop {
        // Re-checked every tick rather than only at the spawn, so a launcher
        // started in the background and later brought forward (`grove &`, then
        // `fg`) hands the terminal on to the job that is actually running under
        // it. The guard is the same one the spawn used: hand over only what
        // this launcher currently owns.
        if let Some(terminal) = terminal {
            if terminal.foreground() == own_group() {
                terminal.hand_to(pgid);
            }
        }

        let waited = match child.try_wait() {
            Ok(waited) => waited,
            Err(error) => {
                // The child's state is now unknown, and returning here would
                // leave an interactive one holding the terminal with nothing
                // left to reap it. Ending it is the last thing this launch can
                // still do correctly, so it does that before reporting.
                kill(pgid, libc::SIGKILL);
                let reaped = child.wait().is_ok();
                return Err(LaunchError::new(format!(
                    "cannot wait on the launched child: {error}; it has been sent SIGKILL and {}",
                    if reaped {
                        "reaped"
                    } else {
                        "could not be reaped — check for an orphaned process"
                    }
                )));
            }
        };
        if let Some(status) = waited {
            // A child ended by the escalation exits non-zero, or by signal.
            // That is the normal completion path, not a failure: the token,
            // never the exit status, says what the launch meant.
            return Ok(ended(status, interrupted, signalled));
        }

        // A signalled launcher forwards **the signal it was sent** and hands
        // over to the same escalation the token path uses, so a child that
        // ignores it is still SIGKILL'd rather than left on the terminal.
        // Forwarding a fixed SIGTERM instead would tell a child that its
        // terminal had not gone away when it had.
        if interrupted.is_none() {
            if let Some(signal) = take_interrupt() {
                interrupted = Some(signal);
                kill(pgid, signal);
                // Start the kill grace only if nothing is already counting one
                // down. Overwriting a running `Terminated` deadline would
                // *extend* the child's life by a full `kill_grace` — so a
                // supervisor trying to hurry a stuck teardown along would be
                // told to wait longer, and each further signal would re-arm it
                // again.
                if !matches!(watch, Watch::Terminated(_)) {
                    watch = Watch::Terminated(Instant::now());
                }
            }
        }

        watch = match watch {
            Watch::Running if channel.path().exists() => Watch::Signalled(Instant::now()),
            Watch::Signalled(at) if at.elapsed() >= escalation.grace => {
                // `signalled` is latched *here*, where the escalation actually
                // runs, and not where the token appeared. `End` would otherwise
                // be telling the caller only what `token` already tells it,
                // while claiming something stronger: that this launch had to be
                // ended. A child that signals and then exits inside its own
                // grace was never touched, and says so.
                signalled = true;
                kill(pgid, libc::SIGTERM);
                Watch::Terminated(Instant::now())
            }
            Watch::Terminated(at) if at.elapsed() >= escalation.kill_grace => {
                kill(pgid, libc::SIGKILL);
                let status = child.wait().map_err(|error| {
                    LaunchError::new(format!("cannot reap the killed child: {error}"))
                })?;
                return Ok(ended(status, interrupted, signalled));
            }
            other => other,
        };

        std::thread::sleep(POLL_INTERVAL);
    }
}

/// Signal the job this process launched — **the whole process group, then the
/// child itself**.
///
/// A grandchild the child spawned — a tool subprocess, a language server, an
/// agent's own in-flight command — is a member of that group and is reaped with
/// its parent rather than surviving it. That matters beyond tidiness: such a
/// grandchild can hold a lock its launcher's caller is about to wait on, and
/// then the escalation's SIGKILL buys a stall rather than a teardown.
///
/// `pgid` is the child's pid, made a group leader by the `setpgid` on both
/// sides of the fork in [`run`]. A group with that id can only have been
/// created by that process, so `-pgid` cannot name an unrelated job even in the
/// impossible case where both `setpgid` calls failed; the direct `kill` behind
/// it covers that case.
///
/// A failure is ignored on purpose — ESRCH means the process exited between the
/// poll and the signal, which the next `try_wait` reports anyway. This is the
/// shell's `kill … 2>/dev/null`, written down.
fn kill(pgid: libc::pid_t, signal: libc::c_int) {
    // SAFETY: `kill(2)` on the process group of, and then the pid of, a child
    // of this process.
    unsafe {
        libc::kill(-pgid, signal);
        libc::kill(pgid, signal);
    }
}
