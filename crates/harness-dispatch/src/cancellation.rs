//! Cancelling a selection on INT, TERM or HUP
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and authority*,
//! *Diagnostics and exits*).
//!
//! While the policy is evaluated, each of the three signals the caller did not
//! ignore at entry is caught, and the handler does nothing but note the first
//! one received. The selection looks at that note wherever it waits or ends a
//! step: every channel read and write between polls, once the worker has
//! answered, once it is reaped, once the chosen candidate's argv is expanded,
//! and once its program is resolved. A noted signal stops and reaps the worker
//! as the deadline does, launches nothing, and is reported as a refusal; then
//! the process ends by re-raising it under its restored entry disposition, so
//! the caller's wait status says what happened.
//!
//! A signal the caller ignored at entry gets no handler: it stays ignored, and
//! cannot cancel selection. The signal mask is never changed for evaluation, so
//! one the caller blocked stays pending, and cannot cancel selection either.
//! Outside evaluation every signal takes its entry course, which before a
//! launch launches nothing either.
//!
//! Handlers restart interrupted calls, so no wait elsewhere in the front sees
//! `EINTR`; the channel's poll bounds how long a noted signal goes unseen. Only
//! the thread that evaluates takes the signals: the threads that drain the
//! worker's output are started with them blocked. A terminal's interrupt
//! reaches the worker and the front together, and the worker's death ends the
//! front's channel. The handler runs before that thread next returns from a
//! system call, and the note is looked at again once the worker is reaped, so
//! the interrupt is reported as the cancellation, never as a worker failure.

use std::io::{self, Write as _};
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::refusal::{Refusal, Stage};

/// The handled signals, by number and name.
const HANDLED: [(libc::c_int, &str); 3] = [
    (libc::SIGINT, "SIGINT"),
    (libc::SIGTERM, "SIGTERM"),
    (libc::SIGHUP, "SIGHUP"),
];

/// The first handled signal received, or 0. Process-global, because a
/// disposition is.
static RECEIVED: AtomicI32 = AtomicI32::new(0);

/// One of the handled signals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Signal {
    number: libc::c_int,
    name: &'static str,
}

impl Signal {
    pub fn name(self) -> &'static str {
        self.name
    }

    /// The status a shell reports for a process this signal ended: `128 + N`.
    pub fn exit(self) -> u8 {
        u8::try_from(128 + self.number).unwrap_or(u8::MAX)
    }
}

/// The handled signal received since the handlers were installed, if any.
pub fn received() -> Option<Signal> {
    let number = RECEIVED.load(Ordering::SeqCst);
    HANDLED
        .iter()
        .find(|&&(handled, _)| handled == number)
        .map(|&(number, name)| Signal { number, name })
}

/// Only an atomic store is async-signal-safe, so that is all the handler
/// does. The first signal is kept: it is the one that cancelled.
extern "C" fn on_signal(signal: libc::c_int) {
    let _ = RECEIVED.compare_exchange(0, signal, Ordering::SeqCst, Ordering::SeqCst);
}

/// The handlers installed for one evaluation. Dropping them restores each
/// signal's entry disposition.
pub struct Handlers {
    entry: Vec<(libc::c_int, libc::sigaction)>,
}

impl Handlers {
    /// Catch each handled signal that is not ignored, keeping its entry
    /// disposition to restore.
    pub fn install() -> io::Result<Handlers> {
        let mut handlers = Handlers { entry: Vec::new() };
        for (signal, _) in HANDLED {
            // SAFETY: a zeroed sigaction is a valid value, and with a null new
            // action sigaction only writes the current one into it.
            let mut entry: libc::sigaction = unsafe { std::mem::zeroed() };
            if unsafe { libc::sigaction(signal, ptr::null(), &mut entry) } == -1 {
                return Err(io::Error::last_os_error());
            }
            if entry.sa_sigaction == libc::SIG_IGN {
                continue;
            }
            // SAFETY: the action's mask is emptied before use, and the handler
            // performs one atomic operation.
            let installed = unsafe {
                let mut action: libc::sigaction = std::mem::zeroed();
                action.sa_sigaction = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
                action.sa_flags = libc::SA_RESTART;
                libc::sigemptyset(&mut action.sa_mask);
                libc::sigaction(signal, &action, ptr::null_mut())
            };
            if installed == -1 {
                return Err(io::Error::last_os_error());
            }
            handlers.entry.push((signal, entry));
        }
        Ok(handlers)
    }
}

impl Drop for Handlers {
    fn drop(&mut self) {
        for (signal, entry) in &self.entry {
            // SAFETY: reinstating a disposition sigaction itself reported.
            unsafe { libc::sigaction(*signal, entry, ptr::null_mut()) };
        }
    }
}

/// Refuse with the cancellation if a handled signal has been received.
pub fn check(source: &str) -> Result<(), Refusal> {
    match received() {
        Some(signal) => Err(refusal(signal, source)),
        None => Ok(()),
    }
}

/// The refusal a cancelled selection reports before it re-raises `signal`.
pub fn refusal(signal: Signal, source: &str) -> Refusal {
    let name = signal.name();
    Refusal::new(
        "selection_cancelled",
        Stage::Evaluation,
        signal.exit(),
        format!(
            "selection with the policy {source} was cancelled by {name}: its worker was stopped \
             and reaped, and nothing was recorded or launched"
        ),
        format!(
            "nothing needs fixing if the signal was meant; otherwise run the same command again. \
             harness-dispatch ends by re-raising {name}, so its caller sees that signal"
        ),
    )
    .source(source)
    .signal(signal)
}

/// End this process with `signal`, under the disposition it had at entry,
/// which the evaluation's handlers restored when they were dropped.
pub fn reraise(signal: Signal) -> ! {
    // A signal death loses buffered output, and the refusal is the last thing
    // the caller reads.
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    // SAFETY: raise only sends a signal to this thread.
    unsafe { libc::raise(signal.number) };
    // Every handled signal terminates by default, a caught one could not have
    // been inherited, and one the caller blocked was never received, so this
    // is reached only if something else in the process has since changed that
    // disposition or blocked the signal.
    std::process::exit(i32::from(signal.exit()))
}

/// Run `start` with the handled signals blocked in this thread, so that a
/// thread it starts inherits them blocked and never takes one.
pub fn shielded<T>(start: impl FnOnce() -> T) -> T {
    // SAFETY: the sets are initialised by sigemptyset and pthread_sigmask
    // before use, and the calling thread's own mask is put back as it was.
    unsafe {
        let mut blocked: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut blocked);
        for (signal, _) in HANDLED {
            libc::sigaddset(&mut blocked, signal);
        }
        let mut previous: libc::sigset_t = std::mem::zeroed();
        libc::pthread_sigmask(libc::SIG_BLOCK, &blocked, &mut previous);
        let started = start();
        libc::pthread_sigmask(libc::SIG_SETMASK, &previous, ptr::null_mut());
        started
    }
}
