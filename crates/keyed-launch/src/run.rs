//! Spawning one child directly and supervising it until it ends.

use std::ffi::OsStr;
use std::path::Path;
use std::process::{Child, Command, ExitStatus};
use std::sync::atomic::{AtomicBool, Ordering};
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
    /// launch**. The child was sent the same signal and reaped through the
    /// ordinary escalation, so it is never left orphaned onto the terminal. The
    /// channel cannot express this case — an interrupt normally leaves no token
    /// at all.
    ///
    /// A signal arriving *between* launches is not this: `run` discards it, and
    /// [`take_interrupt`] is where a looping launcher collects it.
    Interrupted,
}

/// The supervisor's state machine: idle until the token appears, then timed
/// toward SIGTERM and finally SIGKILL.
enum Watch {
    Running,
    Signalled(Instant),
    Terminated(Instant),
}

/// Set by [`on_terminate`], read by [`run`]'s poll loop.
///
/// Process-global because a signal disposition is process-global, and latched
/// because the launch on which the child finally exits still has to report it.
///
/// **A latch that outlives its launch is a loaded gun**, and this one is scoped
/// to exactly one launch at both ends. [`run`] clears it immediately before
/// spawning, so a signal that arrived while no child existed can never be
/// spent on a fresh child that has not signalled and has done nothing wrong;
/// [`take_interrupt`] lets a launcher consume it between launches, which is
/// where such a signal actually belongs. Without the clear, a driver signalled
/// in the gap between two iterations starts the next session and SIGTERMs it on
/// its first poll.
static TERMINATED: AtomicBool = AtomicBool::new(false);

/// Whether this process was sent SIGTERM or SIGHUP outside a launch, clearing
/// the flag.
///
/// For a launcher that runs launches in a loop: [`run`] reports a signal that
/// arrives *during* a launch as [`End::Interrupted`], but one arriving between
/// two launches has no launch to be reported against, and `run` deliberately
/// discards it rather than spending it on the next child. Call this at the top
/// of the loop to honour it instead.
///
/// It answers `false` before this process's first [`run`], because nothing has
/// installed a handler yet and the signal took its default disposition.
#[must_use]
pub fn take_interrupt() -> bool {
    TERMINATED.swap(false, Ordering::Relaxed)
}

/// A `bool` store is the *only* work done here, because it is the only work
/// that is async-signal-safe. Signalling and reaping the child happen one poll
/// tick later, on [`run`]'s ordinary stack.
extern "C" fn on_terminate(_signal: libc::c_int) {
    TERMINATED.store(true, Ordering::Relaxed);
}

/// Catch SIGTERM and SIGHUP so a launcher can forward termination to its child
/// and reap it rather than orphan it.
///
/// Installed by [`run`] rather than exported, because [`End::Interrupted`] is a
/// promise this crate makes and a caller cannot be relied on to have enabled
/// it. Re-installing the same handler is idempotent, so calling it once per
/// launch costs nothing.
///
/// SIGINT is deliberately absent: Ctrl-C is delivered to the whole foreground
/// process group and belongs to the child, which owns the terminal. What a
/// launcher does about its own SIGINT is its policy, not this crate's.
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

/// Spawn `launch`'s argv directly and supervise the child until it ends.
///
/// The child's environment is the launcher's, minus [`Launch::scrub`], plus the
/// channel path under [`Launch::channel_var`]. Nothing else is added: no
/// argument, no flag, no variable. A child that needs one says so in its own
/// template, where whoever wrote the configuration can see it.
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

    // Clear the latch *before* the spawn, never after: see `TERMINATED`. A
    // signal that arrived while no child existed is the launcher's to handle
    // through `take_interrupt`, and is not evidence about the child below.
    TERMINATED.store(false, Ordering::Relaxed);

    let child = command.spawn().map_err(|error| {
        LaunchError::new(format!(
            "cannot spawn {:?}: {error}; check that the program exists and is executable",
            launch.argv.program()
        ))
    })?;

    supervise(child, launch.channel, launch.escalation)
}

fn supervise(
    mut child: Child,
    channel: &Channel,
    escalation: Escalation,
) -> Result<Ended, LaunchError> {
    let started = Instant::now();
    let mut watch = Watch::Running;
    let mut interrupted = false;
    let mut signalled = false;

    let ended = |status: ExitStatus, interrupted: bool, signalled: bool| Ended {
        end: match (interrupted, signalled) {
            (true, _) => End::Interrupted,
            (false, true) => End::Signalled,
            (false, false) => End::Exited,
        },
        status,
        elapsed: started.elapsed(),
        // Read after the child is gone, so a child still mid-write cannot be
        // observed half-signalled.
        token: channel.read(),
    };

    loop {
        let waited = match child.try_wait() {
            Ok(waited) => waited,
            Err(error) => {
                // The child's state is now unknown, and returning here would
                // leave an interactive one holding the terminal with nothing
                // left to reap it. Ending it is the last thing this launch can
                // still do correctly, so it does that before reporting.
                kill(&child, libc::SIGKILL);
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

        // A signalled launcher forwards the signal and hands over to the same
        // escalation the token path uses, so a child that ignores SIGTERM is
        // still SIGKILL'd rather than left on the terminal.
        if !interrupted && TERMINATED.swap(false, Ordering::Relaxed) {
            interrupted = true;
            kill(&child, libc::SIGTERM);
            // Start the kill grace only if nothing is already counting one
            // down. Overwriting a running `Terminated` deadline would *extend*
            // the child's life by a full `kill_grace` — so a supervisor trying
            // to hurry a stuck teardown along would be told to wait longer, and
            // each further signal would re-arm it again.
            if !matches!(watch, Watch::Terminated(_)) {
                watch = Watch::Terminated(Instant::now());
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
                kill(&child, libc::SIGTERM);
                Watch::Terminated(Instant::now())
            }
            Watch::Terminated(at) if at.elapsed() >= escalation.kill_grace => {
                kill(&child, libc::SIGKILL);
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

/// Signal a child this process itself spawned.
///
/// A failure is ignored on purpose — ESRCH means the child exited between the
/// poll and the signal, which the next `try_wait` reports anyway. This is the
/// shell's `kill … 2>/dev/null`, written down.
///
/// **The direct child, and not its process group.** A grandchild the child
/// spawned — a tool subprocess, a language server, an editor — survives this and
/// is not reaped here. Signalling the group would need the spawn to lead its own
/// session, and a child that leads its own session is no longer the terminal's
/// foreground process group: an interactive one would then take SIGTTIN on its
/// first read from the terminal and stop. Reaping descendants is worth having
/// and is not free, so it is a decision of its own rather than a line added
/// here.
fn kill(child: &Child, signal: libc::c_int) {
    // SAFETY: `kill(2)` on the pid of a child of this process.
    unsafe { libc::kill(child.id() as i32, signal) };
}
