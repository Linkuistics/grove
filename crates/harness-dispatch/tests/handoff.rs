//! The handoff through the command seam: the linearization point after the
//! record commit, and the signal state the harness inherits
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and
//! authority*, *Records and later observations*).
//!
//! The harness's signal state is read by the C probe (`support::probe`),
//! started through the front by a caller whose ignored set and mask each case
//! sets exactly. Its positive control is the probe run through an intermediary
//! that alters the state, which it is seen to report.
//!
//! A signal between the commit and the final check is placed there by a
//! stall rather than by timing: the front's stderr is a pipe the test has
//! already filled, so the handoff notice it writes after the commit blocks
//! until the test drains it. The test waits until the store holds the run,
//! signals, and only then drains, so the signal always arrives after the
//! commit and before the check. The same stall unsignalled is the control.

mod support;

use std::fs::File;
use std::io::{Read as _, Write as _};
use std::os::fd::{AsRawFd as _, FromRawFd as _, OwnedFd};
use std::os::unix::process::ExitStatusExt as _;
use std::process::{Child, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use rusqlite::{Connection, OpenFlags};
use serde_json::Value;
use support::hold::{holding, interrupted, Hold, Interrupt, Place};
use support::probe::{self, State};
use support::{run, Sandbox, ROUTED};

const SIGNALS: [(i32, &str); 3] = [
    (libc::SIGINT, "SIGINT"),
    (libc::SIGTERM, "SIGTERM"),
    (libc::SIGHUP, "SIGHUP"),
];

/// Far past anything a case waits for, so a stuck front fails the test.
const WATCHDOG: Duration = Duration::from_secs(30);

#[test]
fn the_harness_inherits_the_callers_mask_and_ignored_signals() {
    let cases: [(&str, &[i32], &[i32]); 7] = [
        ("nothing ignored or blocked", &[], &[]),
        ("SIGPIPE ignored", &[libc::SIGPIPE], &[]),
        ("HUP ignored, as nohup leaves it", &[libc::SIGHUP], &[]),
        (
            "every handled signal ignored",
            &[libc::SIGINT, libc::SIGTERM, libc::SIGHUP],
            &[],
        ),
        (
            "a handled signal and another blocked",
            &[],
            &[libc::SIGTERM, libc::SIGUSR1],
        ),
        (
            "every handled signal blocked",
            &[],
            &[libc::SIGINT, libc::SIGTERM, libc::SIGHUP],
        ),
        (
            "ignored and blocked together",
            &[libc::SIGPIPE, libc::SIGHUP, libc::SIGQUIT],
            &[libc::SIGINT, libc::SIGUSR2],
        ),
    ];
    for (case, ignored, blocked) in cases {
        let sandbox = Sandbox::new();
        probe::install(&sandbox, "");
        let caller = State::caller(ignored, blocked);
        let mut command = sandbox.command();
        command.args(["run", "--kind", "impl", "--prompt", "p", "--json"]);
        caller.apply_to(&mut command);
        let result = run(&mut command);
        assert_eq!(result.code, Some(0), "{case}: {}", result.stderr);
        assert_eq!(State::observed(&sandbox), caller, "{case}");
    }
}

#[test]
fn a_state_altered_between_the_front_and_the_harness_is_seen() {
    // The positive control for the probe: the candidate is the probe behind
    // an intermediary that flips SIGPIPE and blocks SIGUSR2 before it execs
    // the probe proper. The harness then reports what it was handed, not the
    // caller's state, so a front that altered the state would be seen to.
    for ignored in [&[][..], &[libc::SIGPIPE][..]] {
        let sandbox = Sandbox::new();
        probe::install(&sandbox, r#""--alter", "#);
        let caller = State::caller(ignored, &[]);
        let mut command = sandbox.command();
        command.args(["run", "--kind", "impl", "--prompt", "p"]);
        caller.apply_to(&mut command);
        let result = run(&mut command);
        assert_eq!(result.code, Some(0), "{}", result.stderr);

        let mut altered = caller.clone();
        if !altered.ignored.remove(&libc::SIGPIPE) {
            altered.ignored.insert(libc::SIGPIPE);
        }
        altered.blocked.insert(libc::SIGUSR2);
        let observed = State::observed(&sandbox);
        assert_ne!(observed, caller, "the alteration was not seen");
        assert_eq!(observed, altered);
    }
}

#[test]
fn a_signal_the_caller_blocked_reaches_the_harness_pending_and_cancels_nothing() {
    // The caller blocks TERM, and TERM is sent while the policy holds. It
    // stays pending, selection goes on, and the harness starts with TERM
    // blocked and pending: the caller deferred it, and the harness receives
    // it deferred. The control is the same fixture and timing with TERM not
    // blocked, which cancels.
    for blocked in [true, false] {
        let sandbox = Sandbox::new();
        probe::install(&sandbox, "");
        let pid_file = sandbox.root.join("worker-pid");
        // The import-time hold, in front of the probe's policy rather than
        // the routed one it is written around.
        let policy = std::fs::read_to_string(sandbox.personal_path()).unwrap();
        let held = holding(&pid_file, Place::Import, Hold::Pending, Some(1500), "")
            .replace(ROUTED, &policy);
        sandbox.personal_policy(&held);

        let caller = State::caller(&[], if blocked { &[libc::SIGTERM] } else { &[] });
        let mut command = sandbox.command();
        command.args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--timeout-ms",
            "60000",
            "--json",
        ]);
        caller.apply_to(&mut command);
        let interrupt = Interrupt {
            when: pid_file.clone(),
            signal: libc::SIGTERM,
            group: false,
        };
        let timed = interrupted(&mut command, &pid_file, &interrupt, WATCHDOG);
        assert!(timed.after_signal.is_some(), "never signalled");
        if blocked {
            assert_eq!(timed.run.code, Some(0), "{}", timed.run.stderr);
            let observed = State::observed(&sandbox);
            assert_eq!(observed.blocked, caller.blocked);
            assert_eq!(observed.pending, [libc::SIGTERM].into());
        } else {
            timed.run.cancelled(libc::SIGTERM, "SIGTERM");
            assert!(!sandbox.harness_ran());
        }
    }
}

/// A pipe whose buffer is already full, for the front's stderr. Its write
/// end is blocking again by the time the front inherits it, so the front's
/// first write waits until the test reads.
struct Stall {
    read: File,
    write: Option<OwnedFd>,
    /// The bytes written to fill it, all newlines.
    filled: usize,
}

impl Stall {
    fn new() -> Stall {
        let mut fds = [0; 2];
        // SAFETY: pipe fills both descriptors, and each is owned once. Both
        // close on exec, so a front another test starts meanwhile holds no
        // end of this pipe; the front here gets its write end as stderr.
        let (read, write) = unsafe {
            assert_eq!(libc::pipe(fds.as_mut_ptr()), 0);
            for fd in fds {
                assert_eq!(libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC), 0);
            }
            (File::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1]))
        };
        let set_nonblocking = |on: bool| {
            // SAFETY: fcntl on a descriptor this function owns.
            unsafe {
                let flags = libc::fcntl(write.as_raw_fd(), libc::F_GETFL);
                let flags = if on {
                    flags | libc::O_NONBLOCK
                } else {
                    flags & !libc::O_NONBLOCK
                };
                assert_eq!(libc::fcntl(write.as_raw_fd(), libc::F_SETFL, flags), 0);
            }
        };
        set_nonblocking(true);
        let mut writer = File::from(write.try_clone().unwrap());
        let mut filled = 0;
        for chunk in [4096, 1] {
            let bytes = vec![b'\n'; chunk];
            loop {
                match writer.write(&bytes) {
                    Ok(written) => filled += written,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(error) => panic!("filling the pipe: {error}"),
                }
            }
        }
        drop(writer);
        set_nonblocking(false);
        Stall {
            read,
            write: Some(write),
            filled,
        }
    }

    fn stderr(&mut self) -> Stdio {
        Stdio::from(self.write.take().expect("one front per stall"))
    }

    /// Everything after the fill: read until every writer has closed.
    fn drain(mut self) -> String {
        let mut bytes = Vec::new();
        self.read.read_to_end(&mut bytes).unwrap();
        assert!(bytes.len() >= self.filled);
        String::from_utf8(bytes.split_off(self.filled)).unwrap()
    }
}

/// How many runs the store at `path` holds, or 0 while it cannot be read.
fn runs(path: &std::path::Path) -> i64 {
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .and_then(|store| store.query_row("SELECT count(*) FROM runs", [], |row| row.get(0)))
        .unwrap_or(0)
}

/// What a stalled run did.
struct Stalled {
    code: Option<i32>,
    signal: Option<i32>,
    /// The front's stderr, after the fill.
    stderr: String,
}

/// Run the front with its stderr stalled; once the store holds `runs_before
/// + 1` runs, send `signal` if there is one, then drain.
fn stalled(
    sandbox: &Sandbox,
    caller: Option<&State>,
    json: bool,
    signal: Option<i32>,
    runs_before: i64,
) -> Stalled {
    let mut stall = Stall::new();
    let mut command = sandbox.command();
    command.args(["run", "--kind", "impl", "--prompt", "p"]);
    if json {
        command.arg("--json");
    }
    if let Some(caller) = caller {
        caller.apply_to(&mut command);
    }
    command.stdout(Stdio::null()).stderr(stall.stderr());
    let mut front: Child = command.spawn().expect("the front starts");
    // The command owns this process's copy of the write end, and the drain
    // reads until every writer has closed.
    drop(command);

    let store = sandbox.default_store();
    let started = Instant::now();
    while runs(&store) <= runs_before {
        if started.elapsed() > WATCHDOG {
            let _ = front.kill();
            panic!("the front never committed its run");
        }
        if let Some(status) = front.try_wait().unwrap() {
            panic!("the front ended before committing: {status:?}");
        }
        thread::sleep(Duration::from_millis(5));
    }
    // Committed, and the notice cannot have been written: the pipe is full.
    assert!(
        front.try_wait().unwrap().is_none(),
        "the front did not stall"
    );
    if let Some(signal) = signal {
        let pid = libc::pid_t::try_from(front.id()).unwrap();
        // SAFETY: kill only sends a signal to this test's unreaped child.
        assert_eq!(unsafe { libc::kill(pid, signal) }, 0);
    }
    let stderr = stall.drain();
    let status = front.wait().unwrap();
    Stalled {
        code: status.code(),
        signal: status.signal(),
        stderr,
    }
}

/// The JSON lines of a stalled `--json` run's stderr.
fn documents(stderr: &str) -> Vec<Value> {
    stderr
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap_or_else(|error| panic!("{error}: {line}")))
        .collect()
}

fn show(sandbox: &Sandbox, run_id: &str, json: bool) -> support::Run {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id]);
    if json {
        command.arg("--json");
    }
    run(&mut command)
}

/// A first run, so that the store exists and a stalled run is its second.
fn first_run(sandbox: &Sandbox) {
    let result = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    std::fs::remove_dir_all(&sandbox.record).unwrap();
}

#[test]
fn a_signal_between_the_commit_and_exec_launches_nothing_and_marks_the_attempt_not_executed() {
    for (signal, name) in SIGNALS {
        let sandbox = Sandbox::new();
        sandbox.personal_policy(ROUTED);
        first_run(&sandbox);

        let stalled = stalled(&sandbox, None, true, Some(signal), 1);
        assert_eq!(
            (stalled.code, stalled.signal),
            (None, Some(signal)),
            "{name}: expected death by the signal\n{}",
            stalled.stderr
        );
        assert!(!sandbox.harness_ran(), "{name}: a harness was launched");
        let [notice, refusal] =
            documents(&stalled.stderr)
                .try_into()
                .unwrap_or_else(|lines: Vec<Value>| {
                    panic!("{name}: expected the notice and a refusal: {lines:?}")
                });
        let run_id = notice["handoff"]["runId"].as_str().unwrap();
        let error = &refusal["error"];
        assert_eq!(error["code"], "handoff_cancelled", "{refusal}");
        assert_eq!(error["stage"], "exec", "{refusal}");
        assert_eq!(error["signal"], name, "{refusal}");
        assert_eq!(error["exit"], 128 + signal, "{refusal}");
        assert_eq!(
            error["run"],
            serde_json::json!({ "id": run_id, "launchFailure": "recorded" })
        );
        for field in ["message", "remedy", "source"] {
            assert!(
                error[field].as_str().is_some_and(|text| !text.is_empty()),
                "{name}: the refusal lacks {field}: {refusal}"
            );
        }
        assert!(error["inspect"].is_object(), "{refusal}");

        let export = show(&sandbox, run_id, true).report();
        assert_eq!(export["evidence"], "launch_failure", "{name}");
        assert_eq!(export["execution"], "not_executed", "{name}");
        let failure = &export["launchFailure"];
        assert_eq!(failure["cause"], "cancelled");
        assert_eq!(failure["code"], "handoff_cancelled");
        assert_eq!(failure["stage"], "exec");
        assert_eq!(failure["signal"], name);
        assert_eq!(failure["exit"], 128 + signal);
        let text = show(&sandbox, run_id, false).stdout;
        assert!(
            text.contains("launch failure: the harness was not executed"),
            "{text}"
        );
    }
}

#[test]
fn the_same_stall_unsignalled_reaches_the_harness() {
    // The control for every stalled case: the same fixture, drained without a
    // signal, launches the harness, and its attempt stays unknown.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    first_run(&sandbox);

    let stalled = stalled(&sandbox, None, true, None, 1);
    assert_eq!(stalled.code, Some(0), "{}", stalled.stderr);
    assert!(sandbox.harness_ran());
    let [notice] = documents(&stalled.stderr).try_into().unwrap();
    let run_id = notice["handoff"]["runId"].as_str().unwrap();
    assert_eq!(sandbox.harness_run_id(), run_id);
    let export = show(&sandbox, run_id, true).report();
    assert_eq!(export["evidence"], "handoff_attempt");
    assert_eq!(export["execution"], "unknown");
}

#[test]
fn a_cancelled_handoff_whose_detail_cannot_be_appended_stays_unknown() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    first_run(&sandbox);
    // Fault injection: the store refuses exactly the append.
    Connection::open(sandbox.default_store())
        .unwrap()
        .execute_batch(
            "CREATE TRIGGER refuse_the_append BEFORE INSERT ON launch_failures \
             BEGIN SELECT RAISE(ABORT, 'simulated append failure'); END;",
        )
        .unwrap();

    let stalled = stalled(&sandbox, None, true, Some(libc::SIGTERM), 1);
    assert_eq!(
        (stalled.code, stalled.signal),
        (None, Some(libc::SIGTERM)),
        "{}",
        stalled.stderr
    );
    assert!(!sandbox.harness_ran());
    let [notice, refusal] = documents(&stalled.stderr).try_into().unwrap();
    let run_id = notice["handoff"]["runId"].as_str().unwrap();
    let note = &refusal["error"]["run"];
    assert_eq!(refusal["error"]["code"], "handoff_cancelled");
    assert_eq!(note["id"], run_id);
    assert_eq!(note["launchFailure"], "unrecorded");
    assert_eq!(note["evidence"], "handoff_attempt");
    assert_eq!(note["execution"], "unknown");
    assert!(note["recordError"]["message"]
        .as_str()
        .unwrap()
        .contains("simulated append failure"));

    let export = show(&sandbox, run_id, true).report();
    assert_eq!(export["evidence"], "handoff_attempt");
    assert_eq!(export["execution"], "unknown");
    assert_eq!(export["launchFailure"], Value::Null);
}

#[test]
fn a_signal_the_caller_ignored_does_not_cancel_the_handoff() {
    // nohup's HUP, sent between the commit and the final check: no handler
    // was installed for it, so it is discarded, and the harness runs with HUP
    // still ignored. The HUP case above, not ignored, is the control.
    let sandbox = Sandbox::new();
    probe::install(&sandbox, "");
    first_run(&sandbox);
    let caller = State::caller(&[libc::SIGHUP], &[]);

    let stalled = stalled(&sandbox, Some(&caller), true, Some(libc::SIGHUP), 1);
    assert_eq!(stalled.code, Some(0), "{}", stalled.stderr);
    assert_eq!(State::observed(&sandbox), caller);
}

#[test]
fn a_cancelled_handoff_in_text_mode_names_its_run_and_the_signal() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    first_run(&sandbox);

    let stalled = stalled(&sandbox, None, false, Some(libc::SIGINT), 1);
    assert_eq!(
        (stalled.code, stalled.signal),
        (None, Some(libc::SIGINT)),
        "{}",
        stalled.stderr
    );
    let stderr = &stalled.stderr;
    for expected in [
        "harness-dispatch: running candidate \"deep\"",
        "refused (handoff_cancelled, stage exec)",
        "the launch failure is recorded against it",
        "signal: SIGINT, re-raised once this is reported",
        "inspect: ",
    ] {
        assert!(stderr.contains(expected), "{expected:?} in {stderr}");
    }
    assert!(!sandbox.harness_ran());
}
