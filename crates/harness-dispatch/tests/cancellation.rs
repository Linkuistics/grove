//! Cancelling a selection on INT, TERM or HUP, through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and
//! authority*, *Diagnostics and exits*).
//!
//! A policy holds evaluation at import, in its `loadContext` or in its
//! `select`, and the front is signalled once the hold has begun. It stops and
//! reaps the worker, records and launches nothing, prints one clean refusal,
//! and dies of the same signal. The fixtures are the deadline's holds, which
//! `support::hold` shares, run under a selection bound far longer than any
//! case takes, so no cancellation here is a timeout. The same fixtures,
//! uninterrupted or with the signal ignored at entry, reach the fake harness,
//! so no case can pass by never evaluating or never being signalled.

mod support;

use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::Value;
use support::hold::{exists, holding, interrupted, recorded_pid, Hold, Interrupt, Place, PLACES};
use support::{text, Sandbox};

const SIGNALS: [(i32, &str); 3] = [
    (libc::SIGINT, "SIGINT"),
    (libc::SIGTERM, "SIGTERM"),
    (libc::SIGHUP, "SIGHUP"),
];

/// The selection bound every case runs under: far past any case here.
const BOUND_MS: &str = "60000";

/// The watchdog: generous slack for a loaded machine, and still well inside
/// the selection bound, so a front that ignored its signal fails the test
/// rather than timing out into a pass.
const WATCHDOG: Duration = Duration::from_secs(30);

/// The longest a cancellation may take once the front is signalled: a poll of
/// the channel, the worker's cleanup grace, the drains' grace, and slack.
const PROMPTLY: Duration = Duration::from_secs(5);

/// A policy printed this before it held, and the refusal must keep it.
const PRINTED: &str = "console.log(\"before the hold\");";

/// Signal the front, or its whole group, once the policy holding at `place`
/// has begun, and check the cancellation: the refusal and the death by the
/// signal, no worker left, and no run recorded or launched.
fn assert_cancelled(
    command: &str,
    place: Place,
    hold: Hold,
    (signal, name): (i32, &str),
    group: bool,
) -> Value {
    let sandbox = Sandbox::new();
    let pid_file = sandbox.root.join("worker-pid");
    sandbox.personal_policy(&holding(&pid_file, place, hold, None, PRINTED));
    let context = format!("{command} {place:?} {hold:?} {name} (group {group})");

    let mut invocation = sandbox.command();
    invocation.args([
        command,
        "--kind",
        "impl",
        "--prompt",
        "the prompt",
        "--timeout-ms",
        BOUND_MS,
        "--json",
    ]);
    let interrupt = Interrupt {
        when: pid_file.clone(),
        signal,
        group,
    };
    let timed = interrupted(&mut invocation, &pid_file, &interrupt, WATCHDOG);

    let refusal = timed.run.cancelled(signal, name);
    assert_eq!(
        refusal["error"]["source"],
        text(&sandbox.personal_path()),
        "{context}"
    );
    assert!(
        refusal["error"]["message"].as_str().unwrap().contains(name),
        "{context}: {refusal}"
    );
    assert_eq!(
        refusal["diagnostics"]["stdout"], "before the hold\n",
        "{context}"
    );
    assert_eq!(
        refusal["error"].get("inspect").is_some(),
        command == "run",
        "{context}: a refused run names its inspect invocation"
    );
    let pid = recorded_pid(&pid_file).unwrap();
    assert!(!exists(pid), "{context}: worker {pid} survived the front");
    assert!(!sandbox.harness_ran(), "{context}: a harness was launched");
    assert!(
        !sandbox.default_store().exists(),
        "{context}: a run was recorded"
    );
    let after = timed.after_signal.expect("the front was signalled");
    assert!(after < PROMPTLY, "{context}: took {after:?} to cancel");
    refusal
}

#[test]
fn a_signal_while_the_policy_is_evaluated_cancels_and_launches_nothing() {
    for place in PLACES {
        for signal in SIGNALS {
            assert_cancelled("run", place, Hold::Pending, signal, false);
        }
    }
}

#[test]
fn a_spinning_policy_is_cancelled_and_inspect_is_cancelled_as_run_is() {
    // A synchronous spin runs no TERM listener, so the worker ends only by the
    // signal's default action or the kill after the grace.
    for (place, signal) in PLACES.into_iter().zip(SIGNALS) {
        for command in ["inspect", "run"] {
            assert_cancelled(command, place, Hold::Spin, signal, false);
        }
    }
}

#[test]
fn a_terminal_interrupt_to_the_whole_job_is_reported_as_the_cancellation() {
    // A terminal delivers its interrupt to the foreground group, which the
    // worker joins: the worker dies of it as the front catches it. Its death
    // ends the front's channel, and that is still reported as the
    // cancellation, never as a worker failure.
    for place in PLACES {
        for signal in SIGNALS {
            assert_cancelled("run", place, Hold::Pending, signal, true);
        }
    }
}

/// A prelude that starts `sleep` as an ordinary child of the worker, in its
/// process group, and records its PID in `file`.
fn with_a_child(file: &Path) -> String {
    format!(
        "import {{ spawn }} from \"node:child_process\";\n\
         const child = spawn(\"sleep\", [\"60\"], {{ stdio: \"ignore\" }});\n\
         writeFileSync({:?}, String(child.pid));",
        text(file)
    )
}

/// Wait up to `limit` for `pid` to be gone, since an orphan is reaped by
/// whatever adopts it rather than by this test.
fn gone_within(pid: libc::pid_t, limit: Duration) -> bool {
    let started = Instant::now();
    while exists(pid) {
        if started.elapsed() > limit {
            return false;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    true
}

#[test]
fn a_group_interrupt_reaches_the_policys_own_children_through_the_job() {
    // The worker and its ordinary children stay in the caller's job, so the
    // group's interrupt reaches a child the policy started. The control is the
    // same fixture with the front alone signalled: the front stops only its
    // worker, and the child outlives the selection, so the group is what
    // reached it. That is a control, not a promise: a policy reaps its own
    // children before it returns.
    //
    // The third run is the control for the fixture itself: the same policy,
    // its hold ended after a moment and nothing signalled, selects and
    // reaches the harness. So starting a child is not what keeps the
    // interrupted runs from launching.
    for interrupt in [Some(true), Some(false), None] {
        let sandbox = Sandbox::new();
        let pid_file = sandbox.root.join("worker-pid");
        let child_file = sandbox.root.join("child-pid");
        sandbox.personal_policy(&holding(
            &pid_file,
            Place::LoadContext,
            Hold::Pending,
            interrupt.map_or(Some(200), |_| None),
            &with_a_child(&child_file),
        ));
        let mut invocation = sandbox.command();
        invocation.args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--timeout-ms",
            BOUND_MS,
            "--json",
        ]);
        let timed = interrupted(
            &mut invocation,
            &pid_file,
            &Interrupt {
                // The control never sees its trigger, so it is never signalled.
                when: if interrupt.is_some() {
                    pid_file.clone()
                } else {
                    sandbox.root.join("never")
                },
                signal: libc::SIGINT,
                group: interrupt == Some(true),
            },
            WATCHDOG,
        );

        if interrupt.is_some() {
            timed.run.cancelled(libc::SIGINT, "SIGINT");
            assert!(!sandbox.harness_ran());
        }
        let child = recorded_pid(&child_file).expect("the policy started its child");
        if interrupt == Some(true) {
            assert!(
                gone_within(child, Duration::from_secs(5)),
                "the policy's child {child} survived the group's interrupt"
            );
            continue;
        }
        let survived = exists(child);
        // SAFETY: kill only sends a signal; the PID is the child the policy
        // recorded, and it is killed so that nothing outlives the test.
        unsafe { libc::kill(child, libc::SIGKILL) };
        if interrupt.is_some() {
            assert!(
                survived,
                "the control's child {child} was stopped without the group"
            );
        } else {
            assert_eq!(
                timed.run.code,
                Some(0),
                "uninterrupted\nstderr: {}",
                timed.run.stderr
            );
            assert!(timed.after_signal.is_none(), "the control was signalled");
            assert!(
                sandbox.harness_ran(),
                "uninterrupted: the harness never ran"
            );
        }
    }
}

#[test]
fn a_signal_ignored_at_entry_cannot_cancel_selection() {
    // Each signal is ignored when the front starts, as `nohup` ignores HUP,
    // and sent while the policy holds. It gets no handler, so the selection
    // ends as its hold does and reaches the harness. The control is the same
    // fixture and signal, not ignored, which the same timing cancels: the
    // signal does arrive during the hold.
    for (signal, name) in SIGNALS {
        for ignored in [true, false] {
            let sandbox = Sandbox::new();
            let pid_file = sandbox.root.join("worker-pid");
            sandbox.personal_policy(&holding(
                &pid_file,
                Place::Import,
                Hold::Pending,
                Some(1500),
                "",
            ));
            let mut invocation = sandbox.command();
            invocation.args([
                "run",
                "--kind",
                "impl",
                "--prompt",
                "the prompt",
                "--timeout-ms",
                BOUND_MS,
                "--json",
            ]);
            if ignored {
                // SAFETY: between fork and exec, signal(2) only sets this
                // child's disposition.
                unsafe {
                    invocation.pre_exec(move || {
                        libc::signal(signal, libc::SIG_IGN);
                        Ok(())
                    });
                }
            }
            let interrupt = Interrupt {
                when: pid_file.clone(),
                signal,
                group: false,
            };
            let timed = interrupted(&mut invocation, &pid_file, &interrupt, WATCHDOG);
            assert!(timed.after_signal.is_some(), "{name}: never signalled");
            if ignored {
                assert_eq!(
                    timed.run.code,
                    Some(0),
                    "{name} ignored at entry\nstderr: {}",
                    timed.run.stderr
                );
                assert!(sandbox.harness_ran(), "{name} ignored at entry");
                assert_eq!(sandbox.harness_args(), ["the prompt"]);
            } else {
                timed.run.cancelled(signal, name);
                assert!(!sandbox.harness_ran(), "{name}");
            }
        }
    }
}

#[test]
fn the_same_fixtures_uninterrupted_reach_the_harness() {
    // The positive control for every cancellation fixture: the same holds,
    // ended after a moment and never signalled, select and launch.
    for place in PLACES {
        for hold in [Hold::Pending, Hold::Spin] {
            let sandbox = Sandbox::new();
            let pid_file = sandbox.root.join("worker-pid");
            sandbox.personal_policy(&holding(&pid_file, place, hold, Some(200), PRINTED));
            let result = sandbox.run(&[
                "--kind",
                "impl",
                "--prompt",
                "the prompt",
                "--timeout-ms",
                BOUND_MS,
                "--json",
            ]);
            assert_eq!(
                result.code,
                Some(0),
                "{place:?} {hold:?}\nstderr: {}",
                result.stderr
            );
            assert!(recorded_pid(&pid_file).is_some(), "{place:?} {hold:?}");
            assert!(sandbox.harness_ran(), "{place:?} {hold:?}");
            assert_eq!(sandbox.harness_args(), ["the prompt"]);
        }
    }
}

#[test]
fn a_signal_after_the_result_while_the_worker_is_reaped_launches_nothing() {
    // The result arrives, and then the worker's exit handler marks that it
    // is exiting and spins, so the front is waiting out the cleanup grace
    // with the selection made. A signal then is seen once the worker is
    // reaped, and nothing is launched. The worker exits once it has sent its
    // selection, whether the policy held at import or in `select`. The control
    // is the same fixture unsignalled, which launches.
    for (place, signalled) in [
        (Place::Import, true),
        (Place::Select, true),
        (Place::Import, false),
    ] {
        let sandbox = Sandbox::new();
        let pid_file = sandbox.root.join("worker-pid");
        let exiting = sandbox.root.join("exiting");
        let prelude = format!(
            "process.on(\"exit\", () => {{ writeFileSync({:?}, \"exiting\"); while (true) {{}} }});",
            text(&exiting)
        );
        sandbox.personal_policy(&holding(&pid_file, place, Hold::Spin, Some(0), &prelude));
        let mut invocation = sandbox.command();
        invocation.args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "the prompt",
            "--timeout-ms",
            BOUND_MS,
            "--json",
        ]);
        let context = format!("{place:?} (signalled {signalled})");
        let interrupt = Interrupt {
            // The control never sees its trigger, so it is never signalled.
            when: if signalled {
                exiting.clone()
            } else {
                sandbox.root.join("never")
            },
            signal: libc::SIGTERM,
            group: false,
        };
        let timed = interrupted(&mut invocation, &pid_file, &interrupt, WATCHDOG);
        assert!(
            exiting.exists(),
            "{context}: the worker never began to exit"
        );
        if signalled {
            timed.run.cancelled(libc::SIGTERM, "SIGTERM");
            assert!(!sandbox.harness_ran(), "{context}");
            assert!(!sandbox.default_store().exists(), "{context}");
        } else {
            assert_eq!(timed.run.code, Some(0), "{context}: {}", timed.run.stderr);
            assert!(sandbox.harness_ran(), "{context}");
        }
        assert!(!exists(recorded_pid(&pid_file).unwrap()), "{context}");
    }
}

#[test]
fn a_cancellation_in_text_mode_keeps_the_policy_output_and_names_the_signal() {
    let sandbox = Sandbox::new();
    let pid_file = sandbox.root.join("worker-pid");
    sandbox.personal_policy(&holding(
        &pid_file,
        Place::Select,
        Hold::Pending,
        None,
        "console.log(\"loading the review table\");",
    ));
    let mut invocation = sandbox.command();
    invocation.args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        "the prompt",
        "--timeout-ms",
        BOUND_MS,
    ]);
    let interrupt = Interrupt {
        when: pid_file.clone(),
        signal: libc::SIGTERM,
        group: false,
    };
    let timed = interrupted(&mut invocation, &pid_file, &interrupt, WATCHDOG);

    assert_eq!(
        (timed.run.code, timed.run.signal),
        (None, Some(libc::SIGTERM)),
        "{}",
        timed.run.stderr
    );
    assert_eq!(timed.run.stdout, "");
    let stderr = &timed.run.stderr;
    for expected in [
        "policy stdout: loading the review table",
        "refused (selection_cancelled, stage evaluation)",
        "signal: SIGTERM, re-raised once this is reported",
        "inspect: ",
    ] {
        assert!(stderr.contains(expected), "{expected:?} in {stderr}");
    }
    assert!(!sandbox.harness_ran());
}
