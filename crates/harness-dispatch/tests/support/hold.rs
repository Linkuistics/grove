//! Policies that hold evaluation, and the watchdog that runs the front against
//! them: the deadline's fixtures, and the cancellation's, which interrupt the
//! same holds.
//!
//! Each hold records the worker's PID before it starts, so a test can show
//! that evaluation began and that the process it began in is gone. Every
//! invocation runs under a watchdog. A front that ignored its deadline or a
//! signal would otherwise hang the test, and leave a spinning worker behind it.

use std::fs;
use std::io::Read as _;
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::{text, Run, ROUTED};

/// How a policy holds evaluation.
#[derive(Clone, Copy, Debug)]
pub enum Hold {
    /// A synchronous loop, which no timer inside the worker can interrupt.
    Spin,
    /// An awaited promise that a live interval keeps pending, so the worker's
    /// event loop stays busy and never reports the await as stuck.
    Pending,
}

/// Where a policy holds evaluation.
#[derive(Clone, Copy, Debug)]
pub enum Place {
    /// While its module loads, before the routed policy is exported.
    Import,
    /// Inside `select`, which the front calls only once it has accepted the
    /// policy the worker loaded.
    Select,
    /// Inside `loadContext`, which the front calls only once it has accepted
    /// the policy, and before `select`.
    LoadContext,
}

pub const PLACES: [Place; 3] = [Place::Import, Place::LoadContext, Place::Select];

/// A policy whose `select` runs `$HOLD`, then selects the fake harness.
const SELECTING: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "seam-1",
  async select(request) {
    $HOLD
    return { status: "selected", program: "fake-harness", args: [request.prompt], provider: "origin-a", model: "model-large", effort: "high", reason: "the hold ended" };
  },
};
"#;

/// A policy whose `loadContext` runs `$HOLD`, then returns a context that its
/// `select` selects with.
const LOADING: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "seam-1",
  async loadContext() {
    $HOLD
    return { schemaVersion: 1, summary: "the hold ended" };
  },
  select(request, context) {
    return { status: "selected", program: "fake-harness", args: [request.prompt], provider: "origin-a", model: "model-large", effort: "high", reason: context.summary };
  },
};
"#;

/// A policy that holds at `place`: the routed policy preceded by an
/// import-time hold, or a computed one holding in `select` or `loadContext`.
/// Either way, the worker's PID is recorded just before the hold starts.
/// `ends_after` ends the hold after that many milliseconds; without it, the
/// hold never ends. `prelude` runs first, at import.
pub fn holding(
    pid_file: &Path,
    place: Place,
    hold: Hold,
    ends_after: Option<u64>,
    prelude: &str,
) -> String {
    let until = ends_after.map_or("Infinity".to_owned(), |ms| ms.to_string());
    let hold = match hold {
        Hold::Spin => format!("const until = Date.now() + {until};\nwhile (Date.now() < until) {{}}\n"),
        Hold::Pending => format!(
            "await new Promise((resolve) => {{\n  \
               const work = setInterval(() => {{}}, 20);\n  \
               const ms = {until};\n  \
               if (ms !== Infinity) setTimeout(() => {{ clearInterval(work); resolve(undefined); }}, ms);\n\
             }});\n"
        ),
    };
    let hold = format!(
        "writeFileSync({:?}, String(process.pid));\n{hold}",
        text(pid_file)
    );
    let policy = match place {
        Place::Import => format!("{hold}{ROUTED}"),
        Place::Select => SELECTING.replace("$HOLD", &hold),
        Place::LoadContext => LOADING.replace("$HOLD", &hold),
    };
    format!("import {{ writeFileSync }} from \"node:fs\";\n{prelude}\n{policy}")
}

/// A signal to send once `when` holds something: to the front alone, or, as a
/// terminal delivers one, to the process group the front leads, which the
/// worker joins.
pub struct Interrupt {
    pub when: PathBuf,
    pub signal: libc::c_int,
    pub group: bool,
}

/// What a guarded invocation did, and how long it took.
pub struct Timed {
    pub run: Run,
    pub elapsed: Duration,
    /// From sending the interrupt to the front's exit.
    pub after_signal: Option<Duration>,
}

/// Run the front to completion, failing the test rather than hanging if it
/// has not exited within `limit`. On that failure the front and the worker
/// whose PID it recorded are both killed, so a broken bound leaves nothing
/// spinning.
pub fn guarded(command: &mut Command, pid_file: &Path, limit: Duration) -> Timed {
    watched(command, pid_file, None, limit)
}

/// The same, sending `interrupt` once its file holds something. A group
/// interrupt makes the front lead a process group of its own first.
pub fn interrupted(
    command: &mut Command,
    pid_file: &Path,
    interrupt: &Interrupt,
    limit: Duration,
) -> Timed {
    if interrupt.group {
        command.process_group(0);
    }
    watched(command, pid_file, Some(interrupt), limit)
}

fn watched(
    command: &mut Command,
    pid_file: &Path,
    interrupt: Option<&Interrupt>,
    limit: Duration,
) -> Timed {
    let started = Instant::now();
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the front executable starts");
    let front = libc::pid_t::try_from(child.id()).unwrap();
    let mut signalled_at = None;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if let Some(interrupt) = interrupt.filter(|_| signalled_at.is_none()) {
            if fs::read(&interrupt.when).is_ok_and(|held| !held.is_empty()) {
                let target = if interrupt.group { -front } else { front };
                // SAFETY: kill only sends a signal; the front is this test's
                // unreaped child, so its PID and group name nothing else.
                assert_eq!(unsafe { libc::kill(target, interrupt.signal) }, 0);
                signalled_at = Some(Instant::now());
            }
        }
        if started.elapsed() > limit {
            let _ = child.kill();
            let _ = child.wait();
            if let Some(pid) = recorded_pid(pid_file) {
                // SAFETY: kill only sends a signal.
                unsafe { libc::kill(pid, libc::SIGKILL) };
            }
            panic!("the front was still running after {limit:?}");
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let elapsed = started.elapsed();
    let mut stdout = String::new();
    let mut stderr = String::new();
    child
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut stdout)
        .unwrap();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();
    Timed {
        run: Run {
            code: status.code(),
            signal: status.signal(),
            stdout,
            stderr,
        },
        elapsed,
        after_signal: signalled_at.map(|at| at.elapsed()),
    }
}

pub fn recorded_pid(pid_file: &Path) -> Option<libc::pid_t> {
    fs::read_to_string(pid_file).ok()?.trim().parse().ok()
}

/// Whether `pid` names any process at all, a zombie included. The front
/// reaps its worker before it exits, so right after the front's exit the PID
/// is free.
pub fn exists(pid: libc::pid_t) -> bool {
    // SAFETY: signal 0 only checks that the process exists.
    let result = unsafe { libc::kill(pid, 0) };
    result == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}
