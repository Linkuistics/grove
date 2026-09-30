//! The signal state this process inherited, recorded before the Rust runtime
//! changes it, and reinstated for the harness
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and authority*).
//!
//! The harness is to receive the signal mask, and every disposition that
//! survives exec, exactly as this process inherited them. Two facts of Rust's
//! standard library stand in the way, and both hold in 1.85.0, the workspace's
//! minimum, and in 1.98.1:
//!
//! - Before `main`, `lang_start` runs `sys::init`, whose `reset_sigpipe` sets
//!   SIGPIPE to ignored unless the unstable `-Zon-broken-pipe` is used
//!   (`library/std/src/sys/pal/unix/mod.rs`). Nothing in `main` can see the
//!   caller's SIGPIPE.
//! - `Command::exec` runs `do_exec` in this process. It keeps the calling
//!   thread's mask, sets SIGPIPE to default, and only then runs the `pre_exec`
//!   closures before `execvp` (1.85.0:
//!   `library/std/src/sys/pal/unix/process/process_unix.rs`; 1.98.1:
//!   `library/std/src/sys/process/unix/unix.rs`). Nothing before those
//!   closures can restore SIGPIPE.
//!
//! So [`record`] runs from the executable's initializer section, which the
//! loader runs before the C `main` that calls `lang_start`, and [`reinstate`]
//! is the handoff's pre-exec hook. Exec keeps an ignored disposition and resets
//! every other one to default, so the ignored set and the mask are the whole
//! of what survives it. Every signal is reinstated, not only SIGPIPE and the
//! three that cancel selection, so the harness receives its caller's state
//! whatever else in this process changed a disposition.

use std::io;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::refusal::{Refusal, Stage, EXIT_WORKER};

/// Every signal number either platform uses: 1 to 31 on macOS, to 64 on
/// Linux. The kernel refuses a query of a number it does not have, and that
/// signal is then left alone.
const SIGNALS: std::ops::RangeInclusive<libc::c_int> = 1..=64;

/// Whether the initializer ran, and what it found, one bit per signal (bit
/// `N - 1` for signal `N`): which signals could be queried, which were
/// ignored, and which were blocked. Written once before `main`, while this
/// process has one thread, and only read afterwards.
static RECORDED: AtomicBool = AtomicBool::new(false);
static KNOWN: AtomicU64 = AtomicU64::new(0);
static IGNORED: AtomicU64 = AtomicU64::new(0);
static BLOCKED: AtomicU64 = AtomicU64::new(0);

fn bit(signal: libc::c_int) -> u64 {
    1 << (signal - 1)
}

/// The initializer. The ELF loader runs `.init_array` and dyld runs a Mach-O
/// executable's `__mod_init_func` before `main`: the section names std itself
/// uses for its glibc `ARGV_INIT_ARRAY`, and the `ctor` crate for Apple
/// targets (`__DATA,__mod_init_func,mod_init_funcs`).
#[used]
#[cfg_attr(
    target_vendor = "apple",
    unsafe(link_section = "__DATA,__mod_init_func,mod_init_funcs")
)]
#[cfg_attr(not(target_vendor = "apple"), unsafe(link_section = ".init_array"))]
static RECORD: extern "C" fn() = record;

/// Record the mask and the ignored set. It runs before the Rust runtime is
/// initialized, so it uses nothing but system calls and atomics.
extern "C" fn record() {
    // SAFETY: the set and each action are written by the calls that take
    // them, from zeroed values; a null new action or set only queries.
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        if libc::pthread_sigmask(libc::SIG_BLOCK, ptr::null(), &mut mask) != 0 {
            return;
        }
        let (mut known, mut ignored, mut blocked) = (0, 0, 0);
        for signal in SIGNALS {
            let mut action: libc::sigaction = std::mem::zeroed();
            if libc::sigaction(signal, ptr::null(), &mut action) == 0 {
                known |= bit(signal);
                if action.sa_sigaction == libc::SIG_IGN {
                    ignored |= bit(signal);
                }
            }
            if libc::sigismember(&mask, signal) == 1 {
                blocked |= bit(signal);
            }
        }
        KNOWN.store(known, Ordering::Relaxed);
        IGNORED.store(ignored, Ordering::Relaxed);
        BLOCKED.store(blocked, Ordering::Relaxed);
        RECORDED.store(true, Ordering::Release);
    }
}

/// Refuse a handoff that could not be transparent: the initializer did not
/// run, so this build cannot know its caller's SIGPIPE.
pub fn recorded() -> Result<(), Refusal> {
    if RECORDED.load(Ordering::Acquire) {
        return Ok(());
    }
    Err(Refusal::new(
        "signal_state_unavailable",
        Stage::Exec,
        EXIT_WORKER,
        "this build of harness-dispatch did not record its caller's signal state before the Rust \
         runtime changed it, so it cannot hand that state to a harness; nothing was evaluated",
        "reinstall harness-dispatch from a release archive or the Homebrew formula; a build for \
         this platform must run its initializer before main",
    )
    .source(
        std::env::current_exe()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "harness-dispatch".to_owned()),
    ))
}

/// Whether the caller had `signal` blocked.
pub fn blocked_at_entry(signal: libc::c_int) -> bool {
    BLOCKED.load(Ordering::Relaxed) & bit(signal) != 0
}

/// The entry mask, rebuilt from its record.
fn entry_mask() -> libc::sigset_t {
    let blocked = BLOCKED.load(Ordering::Relaxed);
    // SAFETY: sigemptyset initializes the set before any member is added.
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut mask);
        for signal in SIGNALS.filter(|&signal| blocked & bit(signal) != 0) {
            libc::sigaddset(&mut mask, signal);
        }
        mask
    }
}

/// Set this thread's mask back to the caller's. A signal that was blocked here
/// and not by the caller, and is pending, is delivered as this returns.
pub fn restore_mask() -> io::Result<()> {
    let mask = entry_mask();
    // SAFETY: the set is initialized; pthread_sigmask returns its error.
    match unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &mask, ptr::null_mut()) } {
        0 => Ok(()),
        error => Err(io::Error::from_raw_os_error(error)),
    }
}

/// The handoff's pre-exec hook: set every recorded signal's disposition to the
/// caller's, ignored or default, then the caller's mask. The mask comes last,
/// so a handled signal pending from the final check onwards is delivered only
/// once its entry disposition is back. Nothing here allocates on success.
pub fn reinstate() -> io::Result<()> {
    let (known, ignored) = (
        KNOWN.load(Ordering::Relaxed),
        IGNORED.load(Ordering::Relaxed),
    );
    let settable = SIGNALS
        .filter(|&signal| known & bit(signal) != 0)
        .filter(|&signal| signal != libc::SIGKILL && signal != libc::SIGSTOP);
    for signal in settable {
        // SAFETY: the action's mask is emptied before use, and its handler is
        // SIG_IGN or SIG_DFL.
        let set = unsafe {
            let mut action: libc::sigaction = std::mem::zeroed();
            action.sa_sigaction = if ignored & bit(signal) != 0 {
                libc::SIG_IGN
            } else {
                libc::SIG_DFL
            };
            libc::sigemptyset(&mut action.sa_mask);
            libc::sigaction(signal, &action, ptr::null_mut())
        };
        if set == -1 {
            let error = io::Error::last_os_error();
            return Err(io::Error::new(
                error.kind(),
                format!("cannot reinstate the caller's disposition of signal {signal}: {error}"),
            ));
        }
    }
    restore_mask().map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot reinstate the caller's signal mask: {error}"),
        )
    })
}
