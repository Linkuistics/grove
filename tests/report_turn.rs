// The `grove-llm report-turn` verb — the intra-session half of the status
// surface (herdr-turn-boundary-hooks).
//
// Driven as a **subprocess**, deliberately, because that is exactly how it
// runs in production: a claude hook spawns `grove-llm report-turn <boundary>`
// with the session's environment. Spawning it proves the whole chain a unit
// test cannot — that the verb exists under that name, parses that argument,
// reads `GROVE_SIGNAL_FILE` and `HERDR_*` out of the inherited environment, and
// puts the right bytes on the socket. It also keeps the tests free of the
// process-global env lock every in-process env test in this suite needs: each
// child's environment is built per spawn.
//
// The policy table itself is pinned in `src/herdr.rs`; these are about the
// wiring around it.

mod support;

use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

const GROVE_LLM: &str = env!("CARGO_BIN_EXE_grove-llm");

/// How a turn hook is invoked: the verb, its boundary argument, and the two
/// environment facts it reads. `signal_file` is `None` for "not running under
/// the loop driver at all"; `Some(path)` for a signal file that may or may not
/// exist on disk — the distinction the whole discriminator turns on.
fn run_hook(boundary: &str, socket: Option<&Path>, signal_file: Option<&Path>) -> Output {
    let mut cmd = Command::new(GROVE_LLM);
    cmd.arg("report-turn").arg(boundary);
    // This suite is itself usually run from inside a herdr pane, so the pane
    // environment is scrubbed and then rebuilt per case; inheriting it would
    // let a passing test report over the developer's own pane.
    cmd.env_remove("HERDR_ENV")
        .env_remove("HERDR_SOCKET_PATH")
        .env_remove("HERDR_PANE_ID")
        .env_remove("GROVE_SIGNAL_FILE");
    if let Some(socket) = socket {
        cmd.env("HERDR_ENV", "1")
            .env("HERDR_SOCKET_PATH", socket)
            .env("HERDR_PANE_ID", "wQ:p1");
    }
    if let Some(signal_file) = signal_file {
        cmd.env("GROVE_SIGNAL_FILE", signal_file);
    }
    cmd.output().expect("running the report-turn verb")
}

/// Assert the verb was invisible to the session that ran it. Both halves are
/// load-bearing, not hygiene: a non-zero exit makes claude print a
/// `hook error` notice into the transcript on every single turn, and stdout
/// from a `UserPromptSubmit` hook is **injected into the conversation as
/// context** — so a stray byte here would be read by the model as an
/// instruction.
fn assert_silent(out: &Output) {
    assert!(
        out.status.success(),
        "a turn hook must never fail the turn (status: {}, stderr: {})",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.stdout.is_empty(),
        "a turn hook must print nothing on stdout — UserPromptSubmit stdout is \
         injected into the conversation (stdout: {:?})",
        String::from_utf8_lossy(&out.stdout)
    );
}

// The headline case: the agent stopped mid-task without signalling, which in a
// grove session means it asked a question and is waiting. Nothing on disk
// records that, and the driver — the harness's *parent* — cannot see it, which
// is the entire reason this verb exists.
#[test]
fn a_turn_that_ended_without_a_completion_signal_reports_blocked() {
    let tmp = TempDir::new().unwrap();
    let sock = tmp.path().join("herdr.sock");
    let herdr = support::fake_herdr(&sock);
    // The path the driver would have exported — with no file at it, because
    // this session never ran `grove-llm complete`.
    let signal = tmp.path().join("loop.signal");

    let out = run_hook("end", Some(&sock), Some(&signal));

    assert_silent(&out);
    assert_eq!(
        support::reported(&herdr.lock().unwrap()),
        vec![("pane.report_agent".into(), "blocked".into())],
        "an unsignalled turn end is a model that stopped and needs a human"
    );
}

// The other half of the discriminator, and the one that stops the fix being
// worse than the bug: *every* completed grove task also ends a turn. Reading
// those as `blocked` would flap the pane at the end of every task, which is
// indistinguishable from the stall the `blocked` report exists to surface.
#[test]
fn a_turn_that_ended_after_completing_a_task_reports_working_not_blocked() {
    let tmp = TempDir::new().unwrap();
    let sock = tmp.path().join("herdr.sock");
    let herdr = support::fake_herdr(&sock);
    let signal = tmp.path().join("loop.signal");
    std::fs::write(&signal, "relaunch\n").unwrap();

    let out = run_hook("end", Some(&sock), Some(&signal));

    assert_silent(&out);
    assert_eq!(
        support::reported(&herdr.lock().unwrap()),
        vec![("pane.report_agent".into(), "working".into())],
        "the task signalled: the driver is about to relaunch, so the pane is busy"
    );
}

// A finished grove is the driver's line to deliver — it reports `idle` and then
// releases. Saying `working` here would flap a finished grove back to busy for
// the couple of seconds until the completion kill lands.
#[test]
fn a_turn_that_ended_the_whole_grove_reports_nothing_and_leaves_it_to_the_driver() {
    let tmp = TempDir::new().unwrap();
    let sock = tmp.path().join("herdr.sock");
    let herdr = support::fake_herdr(&sock);
    let signal = tmp.path().join("loop.signal");
    std::fs::write(&signal, "done\n").unwrap();

    let out = run_hook("end", Some(&sock), Some(&signal));

    assert_silent(&out);
    assert!(
        herdr.lock().unwrap().is_empty(),
        "the finish cycle's idle-then-release belongs to the driver alone \
         (reported: {:?})",
        herdr.lock().unwrap()
    );
}

// Without this, a pane would stay `blocked` for the rest of any session in
// which the agent asked anything: the human answers, the agent works on, and
// nothing ever takes the report back down.
#[test]
fn a_turn_that_started_reports_working_whatever_the_pane_last_said() {
    let tmp = TempDir::new().unwrap();
    let sock = tmp.path().join("herdr.sock");
    let herdr = support::fake_herdr(&sock);
    let signal = tmp.path().join("loop.signal");

    let out = run_hook("start", Some(&sock), Some(&signal));

    assert_silent(&out);
    assert_eq!(
        support::reported(&herdr.lock().unwrap()),
        vec![("pane.report_agent".into(), "working".into())],
        "a submitted prompt means the human is done and the agent is running"
    );
}

// herdr-optional-ui's load-bearing negative, at the one site that runs most
// often. The hooks are only injected under herdr, so this is belt and braces —
// but the verb is on `PATH` and a human may well run it out of curiosity.
#[test]
fn outside_a_herdr_pane_the_verb_is_a_silent_no_op() {
    let tmp = TempDir::new().unwrap();
    let signal = tmp.path().join("loop.signal");

    let out = run_hook("end", None, Some(&signal));

    assert_silent(&out);
    assert!(
        out.stderr.is_empty(),
        "no herdr means nothing to say — not even a diagnostic (stderr: {:?})",
        String::from_utf8_lossy(&out.stderr)
    );
}

// No `GROVE_SIGNAL_FILE` means this session is not a loop task at all — a
// `grove retire` launch, or a human running claude by hand with the hooks
// somehow present. Reporting `blocked` there would pin a pane grove knows
// nothing about, and herdr never expires an authority.
#[test]
fn a_session_outside_the_loop_reports_nothing_at_all() {
    let tmp = TempDir::new().unwrap();
    let sock = tmp.path().join("herdr.sock");
    let herdr = support::fake_herdr(&sock);

    let out = run_hook("end", Some(&sock), None);

    assert_silent(&out);
    assert!(
        herdr.lock().unwrap().is_empty(),
        "no signal file *path* is not the same as an absent signal file: grove \
         has no opinion about a pane it is not driving (reported: {:?})",
        herdr.lock().unwrap()
    );
}

// A refused socket is the everyday case for anyone running stock herdr, or
// whose herdr died mid-session. It must cost the turn nothing.
#[test]
fn a_socket_that_is_not_there_costs_the_turn_nothing() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("not-a-socket");
    let signal = tmp.path().join("loop.signal");

    let started = std::time::Instant::now();
    let out = run_hook("end", Some(&missing), Some(&signal));

    assert_silent(&out);
    assert!(
        started.elapsed() < std::time::Duration::from_secs(2),
        "a missing socket must fail fast, not spend the report budget on every \
         turn (took {:?})",
        started.elapsed()
    );
}
