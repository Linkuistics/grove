//! Supervision through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Supervision*, *The exit
//! signal*, *Cancellation while the harness runs*, *The run ending*).
//!
//! `run` spawns the harness as its own child and supervises it to its end. The
//! fakes here are shell scripts the policy selects by kind, so each case names
//! the harness it needs. A fake reaches the front by `$FRONT`, which every
//! command in this file is given, to send the exit signal or to start a nested
//! run.
//!
//! Where a controlling terminal matters, the front runs under a `sh` session
//! leader on a fresh pseudo-terminal (`support::pty`). A plain leader keeps the
//! front in its own process group, so what the leader measures once the front
//! returns is what the front restored. A leader with `set -m` runs the front as
//! a job of its own instead, the way an interactive shell does, so that a typed
//! Ctrl-C reaches only that job and the leader survives to report it.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::process::ExitStatusExt as _;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use rusqlite::Connection;
use serde_json::{json, Value};
use support::pty::in_pty;
use support::{executable, run, selecting, text, Sandbox, FRONT};

/// A policy that runs, for each kind, the program of the same name with no
/// arguments, so a case selects its fake by the kind it passes.
fn by_kind() -> String {
    selecting(
        "",
        r#"    return { status: "selected", program: request.kind, args: [], provider: "origin-a", model: "m", effort: "e", reason: "the kind names the fake" };"#,
    )
}

/// A sandbox with the by-kind policy installed.
fn sandbox() -> Sandbox {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&by_kind());
    sandbox
}

/// Put a fake harness named `name` on the sandbox's PATH.
fn fake(sandbox: &Sandbox, name: &str, body: &str) {
    executable(&sandbox.bin.join(name), &format!("#!/bin/sh\n{body}"));
}

/// The front, run for `kind`, with `$FRONT` and `$ROOT` (the sandbox root)
/// in its environment for the fake.
fn front(sandbox: &Sandbox, kind: &str, extra: &[&str]) -> std::process::Command {
    let mut command = sandbox.command();
    command
        .args(["run", "--kind", kind, "--prompt", "p", "--json"])
        .args(extra)
        .env("FRONT", FRONT)
        .env("ROOT", &sandbox.root);
    command
}

/// The JSON lines a `run --json` wrote: the handoff notice, then the end
/// notice, which this answers. The harness shares the front's stderr, so a
/// line of its own, such as its shell reporting a command the escalation
/// killed, is passed over.
fn end_notice(stderr: &str) -> Value {
    let notices: Vec<Value> = stderr
        .lines()
        .filter(|line| line.starts_with('{'))
        .map(|line| serde_json::from_str(line).unwrap_or_else(|error| panic!("{error}: {line}")))
        .collect();
    let [handoff, end]: [Value; 2] = notices
        .try_into()
        .unwrap_or_else(|_| panic!("expected a handoff and an end notice: {stderr}"));
    assert_eq!(end["end"]["runId"], handoff["handoff"]["runId"], "{stderr}");
    end["end"].clone()
}

/// A `stty -g` reading, less PENDIN in its local modes. PENDIN is the
/// driver's own transient state, not a mode anyone sets: BSD sets it when a
/// terminal goes from raw back to canonical with input queued, which is what a
/// restore does, and clears it on the next read.
fn modes(path: &Path) -> String {
    read(path)
        .split(':')
        .map(|field| match field.strip_prefix("lflag=") {
            Some(hex) => {
                #[allow(clippy::useless_conversion)]
                let pendin = u64::from(libc::PENDIN);
                let lflag = u64::from_str_radix(hex, 16).unwrap() & !pendin;
                format!("lflag={lflag:x}")
            }
            None => field.to_owned(),
        })
        .collect::<Vec<_>>()
        .join(":")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()))
        .trim()
        .to_owned()
}

fn wait_for(path: &Path, within: Duration) {
    let until = Instant::now() + within;
    while !path.exists() {
        assert!(Instant::now() < until, "{} never appeared", path.display());
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Whether `pid` names no process within two seconds. A member the front
/// killed is reaped by `init` once the front has gone, so it may linger
/// briefly as a zombie; only ESRCH answers that it is gone.
fn gone(pid: &str) -> bool {
    let pid: libc::pid_t = pid.parse().unwrap();
    let until = Instant::now() + Duration::from_secs(2);
    loop {
        // SAFETY: signal 0 only probes.
        if unsafe { libc::kill(pid, 0) } == -1
            && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
        {
            return true;
        }
        if Instant::now() >= until {
            return false;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// A TERM-, HUP- and INT-ignoring descendant in the fake's group, its PID in
/// `$ROOT/descendant`: the survivor the end of every run has to kill.
const DESCENDANT: &str =
    "sh -c 'trap \"\" TERM HUP INT; while : ; do sleep 0.05 ; done' &\necho $! > \"$ROOT/descendant\"\n";

// ---------------------------------------------------------------------------
// The job and its terminal

/// The leader that runs the front for `kind` in its own group and measures
/// the terminal before and after: its foreground group and its modes.
const MEASURING_LEADER: &str = r#"
ps -o tpgid= -p $$ > "$ROOT/before-foreground"
stty -g > "$ROOT/before-modes"
"$FRONT" run --kind "$KIND" --prompt p --json 2> "$ROOT/stderr"
echo $? > "$ROOT/status"
ps -o tpgid= -p $$ > "$ROOT/after-foreground"
stty -g > "$ROOT/after-modes"
"#;

fn leader(sandbox: &Sandbox, script: &str, kind: &str) -> std::process::Command {
    let mut command = sandbox.command_for(Path::new("/bin/sh"));
    command
        .args(["-c", script])
        .env("FRONT", FRONT)
        .env("ROOT", &sandbox.root)
        .env("KIND", kind);
    command
}

/// The harness leads a job of its own and holds the terminal. A raw-mode
/// harness that sends the exit signal and then declines to die is sent
/// SIGTERM after the grace, which it traps, and SIGKILL after the kill-grace,
/// which it cannot. The front then takes the terminal back and restores the
/// modes it saved, and its own exit is 0, since the exit signal's escalation
/// ended the run. A TERM-ignoring descendant is gone with it.
///
/// The harness's own `stty -g` is the positive control: the terminal really
/// was raw while it held it, so equal modes afterwards are a restore and not a
/// terminal nothing touched.
#[test]
fn a_raw_mode_harness_ended_by_the_escalation_leaves_the_terminal_restored() {
    let sandbox = sandbox();
    fake(
        &sandbox,
        "raw",
        &format!(
            "echo $$ > \"$ROOT/harness\"\nps -o pgid=,tpgid= -p $$ > \"$ROOT/job\"\n\
             stty raw -echo\nstty -g > \"$ROOT/raw-modes\"\n{DESCENDANT}\
             trap 'echo term >> \"$ROOT/terms\"' TERM\n\"$FRONT\" exit\n\
             while : ; do sleep 0.05 ; done\n"
        ),
    );
    let (status, output) = in_pty(
        leader(&sandbox, MEASURING_LEADER, "raw"),
        Duration::from_secs(30),
        |_| {},
    );
    assert!(status.success(), "{status:?}\n{output}");
    let stderr = read(&sandbox.root.join("stderr"));
    assert_eq!(read(&sandbox.root.join("status")), "0", "{stderr}");

    let harness = read(&sandbox.root.join("harness"));
    let job: Vec<String> = read(&sandbox.root.join("job"))
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        job,
        [harness.clone(), harness.clone()],
        "the harness leads its own group, which holds the terminal"
    );
    assert_ne!(
        modes(&sandbox.root.join("raw-modes")),
        modes(&sandbox.root.join("before-modes")),
        "control: the harness never set the terminal raw"
    );
    assert_eq!(
        modes(&sandbox.root.join("after-modes")),
        modes(&sandbox.root.join("before-modes")),
        "the front did not restore the modes it saved"
    );
    assert_eq!(
        read(&sandbox.root.join("after-foreground")),
        read(&sandbox.root.join("before-foreground")),
        "the front did not take the terminal back"
    );

    let end = end_notice(&stderr);
    assert_eq!(end["ending"], "exit_signal", "{end}");
    assert_eq!(
        end["signal"], "SIGKILL",
        "the kill-grace's SIGKILL ended it: {end}"
    );
    assert_eq!(
        read(&sandbox.root.join("terms")),
        "term",
        "one SIGTERM, trapped"
    );
    let lasted = end["durationMs"].as_u64().unwrap();
    assert!(
        (6_500..20_000).contains(&lasted),
        "grace 2 s then kill-grace 5 s, from the signal: {lasted} ms"
    );
    assert!(
        gone(&read(&sandbox.root.join("descendant"))),
        "a TERM-ignoring member of the harness's group outlived the run"
    );
}

/// A typed Ctrl-C reaches the harness's group and not the front. The harness
/// dies of it, so the run ends `harness_exit` and the front dies of the
/// harness's SIGINT. Had the front received it as well, the run would have
/// been `cancelled`, which is what
/// `a_signal_after_the_linearization_point_cancels_the_run` shows an INT sent
/// to the front does.
#[test]
fn a_typed_ctrl_c_reaches_the_harness_alone() {
    let sandbox = sandbox();
    fake(
        &sandbox,
        "waiting",
        ": > \"$ROOT/ready\"\nwhile : ; do sleep 0.05 ; done\n",
    );
    let ready = sandbox.root.join("ready");
    let (status, output) = in_pty(
        leader(&sandbox, MEASURING_LEADER, "waiting"),
        Duration::from_secs(30),
        |keyboard| {
            wait_for(&ready, Duration::from_secs(20));
            keyboard.type_bytes(b"\x03");
        },
    );
    assert!(status.success(), "{status:?}\n{output}");
    let stderr = read(&sandbox.root.join("stderr"));
    let end = end_notice(&stderr);
    assert_eq!(end["ending"], "harness_exit", "{end}");
    assert_eq!(end["signal"], "SIGINT", "{end}");
    assert_eq!(
        read(&sandbox.root.join("status")),
        (128 + libc::SIGINT).to_string(),
        "the front dies of the harness's signal: {stderr}"
    );
    assert_eq!(
        read(&sandbox.root.join("after-foreground")),
        read(&sandbox.root.join("before-foreground")),
        "the terminal came back after the harness's death by signal"
    );
}

/// During selection, a typed Ctrl-C reaches the front and its worker, which
/// are the foreground job then, and cancels the selection: nothing launches.
/// The leader runs the front as a job of its own, as an interactive shell
/// does, and survives to report it, so the interrupt reached that job alone.
#[test]
fn a_typed_ctrl_c_during_selection_reaches_the_front_and_its_worker_alone() {
    let sandbox = Sandbox::new();
    let ready = sandbox.root.join("selecting");
    sandbox.personal_policy(&selecting(
        "",
        &format!(
            "    require(\"node:fs\").writeFileSync({:?}, \"\");\n    while (true) {{}}",
            text(&ready)
        ),
    ));
    let script = format!("set -m\n{MEASURING_LEADER}");
    let (status, output) = in_pty(
        leader(&sandbox, &script, "impl"),
        Duration::from_secs(30),
        |keyboard| {
            wait_for(&ready, Duration::from_secs(20));
            keyboard.type_bytes(b"\x03");
        },
    );
    assert!(status.success(), "{status:?}\n{output}");
    assert_eq!(
        read(&sandbox.root.join("status")),
        (128 + libc::SIGINT).to_string(),
        "{output}"
    );
    let stderr = read(&sandbox.root.join("stderr"));
    let refusal: Value = serde_json::from_str(&stderr).unwrap_or_else(|_| panic!("{stderr}"));
    assert_eq!(refusal["error"]["code"], "selection_cancelled", "{refusal}");
    assert_eq!(refusal["error"]["signal"], "SIGINT", "{refusal}");
    assert!(!sandbox.harness_ran());
}

// ---------------------------------------------------------------------------
// The exit signal

/// A harness that sends the exit signal and exits within the grace is never
/// signalled, and its own exit decides the front's: 0 for 0, its own code
/// otherwise. The run ends `exit_signal` either way.
#[test]
fn a_harness_that_exits_within_the_grace_is_never_signalled() {
    let sandbox = sandbox();
    for code in [0, 4] {
        let _ = fs::remove_file(sandbox.root.join("terms"));
        fake(
            &sandbox,
            "prompt",
            &format!(
                "trap 'echo term >> \"$ROOT/terms\"' TERM\n\"$FRONT\" exit\nsleep 0.5\nexit {code}\n"
            ),
        );
        let ran = run(&mut front(&sandbox, "prompt", &[]));
        assert_eq!(ran.code, Some(code), "{}", ran.stderr);
        let end = end_notice(&ran.stderr);
        assert_eq!(end["ending"], "exit_signal", "{end}");
        assert_eq!(end["exitCode"], code, "{end}");
        assert!(
            !sandbox.root.join("terms").exists(),
            "a harness that exited within the grace was signalled"
        );
    }
}

/// Every run gets a fresh exit channel, published to its harness and removed
/// after the run. With `--exit-dir` it is allocated there, and the directory
/// is left in place; without, in a private owner-only directory under TMPDIR
/// that goes with the run. A channel an earlier run left ends nothing: the
/// second harness recreates the first run's channel, and then outlasts the
/// grace and exits with its own code.
#[test]
fn every_run_gets_a_fresh_exit_channel_where_the_caller_names() {
    let sandbox = sandbox();
    let exit_dir = sandbox.root.join("control");
    fs::create_dir(&exit_dir).unwrap();
    fake(
        &sandbox,
        "channel",
        "printf '%s' \"$HARNESS_DISPATCH_EXIT_FILE\" > \"$ROOT/channel\"\n\
         ls -ld \"$(dirname \"$HARNESS_DISPATCH_EXIT_FILE\")\" > \"$ROOT/directory\"\n\
         if [ -n \"$OLD_CHANNEL\" ]; then : > \"$OLD_CHANNEL\"; sleep 3; exit 7; fi\n",
    );

    let first = run(&mut front(
        &sandbox,
        "channel",
        &["--exit-dir", "../control"],
    ));
    assert_eq!(first.code, Some(0), "{}", first.stderr);
    let first_channel = PathBuf::from(read(&sandbox.root.join("channel")));
    assert_eq!(first_channel.parent(), Some(exit_dir.as_path()));
    assert!(!first_channel.exists(), "the channel outlived its run");
    assert!(exit_dir.is_dir(), "the caller's directory was removed");

    let mut second = front(&sandbox, "channel", &["--exit-dir", "../control"]);
    second.env("OLD_CHANNEL", &first_channel);
    let second = run(&mut second);
    let second_channel = PathBuf::from(read(&sandbox.root.join("channel")));
    assert_ne!(second_channel, first_channel, "a channel was reused");
    assert_eq!(
        second.code,
        Some(7),
        "the earlier run's channel ended this one: {}",
        second.stderr
    );
    assert_eq!(end_notice(&second.stderr)["ending"], "harness_exit");
    let _ = fs::remove_file(&first_channel);

    let private = run(&mut front(&sandbox, "channel", &[]));
    assert_eq!(private.code, Some(0), "{}", private.stderr);
    let channel = PathBuf::from(read(&sandbox.root.join("channel")));
    let directory = channel.parent().unwrap();
    assert_eq!(
        directory.parent(),
        Some(sandbox.tmp.as_path()),
        "under TMPDIR"
    );
    assert!(
        read(&sandbox.root.join("directory")).starts_with("drwx------"),
        "owner-only: {}",
        read(&sandbox.root.join("directory"))
    );
    assert!(
        !directory.exists(),
        "the private directory outlived its run"
    );
}

/// `exit` under no supervised run signals nothing, says so, and exits 0.
/// Under one whose channel directory has gone, it names the path and fails.
#[test]
fn exit_outside_a_supervised_run_is_a_no_op_and_a_lost_channel_fails() {
    let sandbox = Sandbox::new();
    let mut command = sandbox.command();
    command.arg("exit");
    let outside = run(&mut command);
    assert_eq!(outside.code, Some(0), "{}", outside.stderr);
    assert!(
        outside
            .stderr
            .contains("not running under a supervised run"),
        "{}",
        outside.stderr
    );

    let lost = sandbox.root.join("gone/signal");
    let mut command = sandbox.command();
    command.arg("exit").env("HARNESS_DISPATCH_EXIT_FILE", &lost);
    let failed = run(&mut command);
    assert_eq!(failed.code, Some(1), "{}", failed.stderr);
    assert!(failed.stderr.contains(&text(&lost)), "{}", failed.stderr);

    let present = sandbox.root.join("signal");
    let mut command = sandbox.command();
    command
        .arg("exit")
        .env("HARNESS_DISPATCH_EXIT_FILE", &present);
    assert_eq!(run(&mut command).code, Some(0));
    assert!(present.exists(), "control: the channel was created");
    let mut command = sandbox.command();
    command
        .arg("exit")
        .env("HARNESS_DISPATCH_EXIT_FILE", &present);
    assert_eq!(
        run(&mut command).code,
        Some(0),
        "one already there is success"
    );
}

/// A nested run publishes its own channel to its own harness, so that
/// harness's exit signal ends the nested run alone. The outer harness goes on
/// after it and exits with its own code, and the outer run ends
/// `harness_exit`.
#[test]
fn a_nested_runs_exit_signal_ends_only_the_nested_run() {
    let sandbox = sandbox();
    fake(
        &sandbox,
        "outer",
        "printf '%s' \"$HARNESS_DISPATCH_EXIT_FILE\" > \"$ROOT/outer-channel\"\n\
         \"$FRONT\" run --kind inner --prompt p 2> \"$ROOT/inner-stderr\"\n\
         echo $? > \"$ROOT/inner-status\"\nexit 5\n",
    );
    fake(
        &sandbox,
        "inner",
        "printf '%s' \"$HARNESS_DISPATCH_EXIT_FILE\" > \"$ROOT/inner-channel\"\n\
         \"$FRONT\" exit\nwhile : ; do sleep 0.05 ; done\n",
    );
    let outer = run(&mut front(&sandbox, "outer", &[]));
    assert_eq!(
        read(&sandbox.root.join("inner-status")),
        "0",
        "{}",
        read(&sandbox.root.join("inner-stderr"))
    );
    assert_ne!(
        read(&sandbox.root.join("inner-channel")),
        read(&sandbox.root.join("outer-channel"))
    );
    assert_eq!(outer.code, Some(5), "{}", outer.stderr);
    assert_eq!(
        end_notice(&outer.stderr)["ending"],
        "harness_exit",
        "the nested harness's exit signal ended the outer run"
    );
}

// ---------------------------------------------------------------------------
// Cancellation

/// A handled signal sent to the front while its harness runs cancels the run:
/// the harness's group is sent the same signal and reaped, the run ends
/// `cancelled`, and the front dies of that signal. A member of the group that
/// ignores the signal is gone too.
#[test]
fn a_signal_after_the_linearization_point_cancels_the_run() {
    for (signal, name) in [
        (libc::SIGTERM, "SIGTERM"),
        (libc::SIGHUP, "SIGHUP"),
        (libc::SIGINT, "SIGINT"),
    ] {
        let sandbox = sandbox();
        fake(
            &sandbox,
            "busy",
            &format!(
                "echo $$ > \"$ROOT/harness\"\n{DESCENDANT}: > \"$ROOT/ready\"\n\
                 while : ; do sleep 0.05 ; done\n"
            ),
        );
        let mut command = front(&sandbox, "busy", &[]);
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        wait_for(&sandbox.root.join("ready"), Duration::from_secs(20));
        // SAFETY: the front this test started, still running its harness.
        unsafe { libc::kill(child.id() as libc::pid_t, signal) };
        let output = child.wait_with_output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.signal(), Some(signal), "{name}: {stderr}");
        let end = end_notice(&stderr);
        assert_eq!(end["ending"], "cancelled", "{name}: {end}");
        assert!(gone(&read(&sandbox.root.join("harness"))), "{name}");
        assert!(gone(&read(&sandbox.root.join("descendant"))), "{name}");
    }
}

/// A missing exit directory refuses before selection: with no policy
/// installed, the refusal is about the directory, not the policy. The control
/// is the same run with the directory present, which reaches the policy and
/// refuses that it is missing.
#[test]
fn a_missing_exit_directory_refuses_before_selection() {
    let sandbox = Sandbox::new();
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--exit-dir",
            "absent",
            "--json",
        ])
        .refusal(2);
    assert_eq!(refusal["error"]["code"], "exit_dir_unusable", "{refusal}");
    assert_eq!(refusal["error"]["input"], "--exit-dir", "{refusal}");

    let file = sandbox.file("not-a-directory", "");
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--exit-dir",
            &text(&file),
            "--json",
        ])
        .refusal(2);
    assert_eq!(refusal["error"]["code"], "exit_dir_unusable", "{refusal}");

    fs::create_dir(sandbox.cwd.join("present")).unwrap();
    fs::set_permissions(
        sandbox.cwd.join("present"),
        fs::Permissions::from_mode(0o700),
    )
    .unwrap();
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--exit-dir",
            "present",
            "--json",
        ])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_missing", "{refusal}");
}

// ---------------------------------------------------------------------------
// The end observation and the ending file

/// `record show --run R --json` against the sandbox's own store.
fn show(sandbox: &Sandbox, run_id: &str) -> Value {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id, "--json"]);
    run(&mut command).report()
}

/// The ID of the run an end notice names.
fn run_of(end: &Value) -> String {
    end["runId"].as_str().unwrap().to_owned()
}

fn user_version(store: &Path) -> i64 {
    Connection::open(store)
        .unwrap()
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap()
}

/// The ending file's one JSON document.
fn ending_document(path: &Path) -> Value {
    let text =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    assert_eq!(text.lines().count(), 1, "{text}");
    serde_json::from_str(&text).unwrap()
}

/// Each of the three endings is reported in the ending file with the harness's
/// exit and duration, and gives the exit status its row states: the harness's
/// own code for `harness_exit`, 0 for an `exit_signal` the escalation ended,
/// and death by the cancelling signal for `cancelled`. The same document is
/// the run's end observation in the store, so `record show` carries it and
/// reads the run as execution confirmed.
#[test]
fn each_ending_is_reported_in_the_ending_file_and_recorded() {
    let sandbox = sandbox();
    let file = sandbox.root.join("ending.json");

    // harness_exit: the harness's own code.
    fake(&sandbox, "quits", "sleep 0.3\nexit 3\n");
    let ran = run(&mut front(
        &sandbox,
        "quits",
        &["--ending-file", &text(&file)],
    ));
    assert_eq!(ran.code, Some(3), "{}", ran.stderr);
    let end = end_notice(&ran.stderr);
    assert_eq!(end["ending"], "harness_exit", "{end}");
    assert_eq!(end["recorded"], true, "{end}");
    let document = ending_document(&file);
    let measured = &document["measurements"];
    assert_eq!(document["source"], "harness-dispatch");
    assert_eq!(document["runId"], end["runId"]);
    assert_eq!(measured["executionConfirmation"]["value"], true);
    assert_eq!(measured["ending"]["value"], "harness_exit");
    assert_eq!(measured["exit"]["value"], json!({ "code": 3 }));
    assert_eq!(measured["duration"]["unit"], "ms");
    let duration = measured["duration"]["value"].as_f64().unwrap();
    assert!((300.0..20_000.0).contains(&duration), "{duration}");
    assert_eq!(
        fs::metadata(&file).unwrap().permissions().mode() & 0o777,
        0o600,
        "owner-only"
    );

    // The store holds the very document the file does.
    let export = show(&sandbox, &run_of(&end));
    assert_eq!(export["evidence"], "execution_confirmed", "{export}");
    assert_eq!(export["execution"], "confirmed", "{export}");
    let stored = &export["observations"][0];
    assert_eq!(stored["observationId"], document["observationId"]);
    assert_eq!(stored["source"], "harness-dispatch");
    assert_eq!(stored["measurements"]["ending"]["value"], "harness_exit");
    assert_eq!(
        stored["measurements"]["exit"]["value"],
        json!({ "code": 3 })
    );
    assert_eq!(
        stored["measurements"]["duration"]["value"],
        measured["duration"]["value"]
    );
    assert_eq!(export["measurements"]["ending"]["state"], "observed");

    // exit_signal: the escalation ends a harness that will not exit.
    let file = sandbox.root.join("exit-signal.json");
    fake(
        &sandbox,
        "lingers",
        "trap '' TERM\n\"$FRONT\" exit\nwhile : ; do sleep 0.05 ; done\n",
    );
    let ran = run(&mut front(
        &sandbox,
        "lingers",
        &["--ending-file", &text(&file)],
    ));
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let measured = ending_document(&file)["measurements"].clone();
    assert_eq!(measured["ending"]["value"], "exit_signal");
    assert_eq!(measured["exit"]["value"], json!({ "signal": "SIGKILL" }));

    // exit_signal again, from a harness that exits 0 on its own after it.
    let file = sandbox.root.join("exit-signal-zero.json");
    fake(&sandbox, "signals", "\"$FRONT\" exit\nexit 0\n");
    let ran = run(&mut front(
        &sandbox,
        "signals",
        &["--ending-file", &text(&file)],
    ));
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let measured = ending_document(&file)["measurements"].clone();
    assert_eq!(measured["ending"]["value"], "exit_signal");
    assert_eq!(measured["exit"]["value"], json!({ "code": 0 }));

    // cancelled: a handled signal while the harness runs.
    let file = sandbox.root.join("cancelled.json");
    fake(
        &sandbox,
        "busy",
        ": > \"$ROOT/ready\"\nwhile : ; do sleep 0.05 ; done\n",
    );
    let mut command = front(&sandbox, "busy", &["--ending-file", &text(&file)]);
    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for(&sandbox.root.join("ready"), Duration::from_secs(20));
    // SAFETY: the front this test started, still running its harness.
    unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM) };
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.signal(), Some(libc::SIGTERM));
    let measured = ending_document(&file)["measurements"].clone();
    assert_eq!(measured["ending"]["value"], "cancelled");
    assert_eq!(measured["exit"]["value"], json!({ "signal": "SIGTERM" }));
    assert!(measured["duration"]["value"].as_f64().unwrap() > 0.0);
}

/// The store lock places a signal after supervision and before recording.
/// Dropping the handlers there loses the end of an already reaped harness.
#[test]
fn a_signal_while_recording_preserves_the_reaped_harnesses_ending() {
    for cancelled in [false, true] {
        let sandbox = sandbox();
        let file = sandbox.root.join("ending.json");
        fake(
            &sandbox,
            "quits-after-lock",
            ": > \"$ROOT/ready\"\nwhile test ! -e \"$ROOT/go\"; do sleep 0.01; done\nexit 3\n",
        );
        let child = front(
            &sandbox,
            "quits-after-lock",
            &["--ending-file", &text(&file)],
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
        wait_for(&sandbox.root.join("ready"), Duration::from_secs(20));
        let connection = Connection::open(sandbox.default_store()).unwrap();
        connection.execute_batch("BEGIN EXCLUSIVE").unwrap();
        if cancelled {
            // SAFETY: the live front this test started; its harness is waiting.
            unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGHUP) };
        } else {
            fs::write(sandbox.root.join("go"), "").unwrap();
        }
        // Publication proves the harness is reaped and its group gone. The
        // append cannot finish while this exclusive lock remains held.
        wait_for(&file, Duration::from_secs(10));
        // SAFETY: the front is blocked on the store, with its harness reaped.
        unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM) };
        connection.execute_batch("COMMIT").unwrap();
        let output = child.wait_with_output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            output.status.signal(),
            Some(if cancelled {
                libc::SIGHUP
            } else {
                libc::SIGTERM
            }),
            "{stderr}"
        );
        let end = end_notice(&stderr);
        assert_eq!(
            end["ending"],
            if cancelled {
                "cancelled"
            } else {
                "harness_exit"
            },
            "{end}"
        );
        assert_eq!(end["recorded"], true, "{end}");
        assert_eq!(
            ending_document(&file)["measurements"]["exit"]["value"],
            if cancelled {
                json!({"signal": "SIGHUP"})
            } else {
                json!({"code": 3})
            }
        );
        assert_eq!(
            show(&sandbox, &run_of(&end))["evidence"],
            "execution_confirmed"
        );
    }
}

/// The caller can read the ending while an unrelated writer holds the store.
#[test]
fn the_ending_file_is_published_before_waiting_for_the_store() {
    let sandbox = sandbox();
    let file = sandbox.root.join("ending.json");
    fake(
        &sandbox,
        "quits-after-lock",
        ": > \"$ROOT/ready\"\nwhile test ! -e \"$ROOT/go\"; do sleep 0.01; done\nexit 3\n",
    );
    let child = front(
        &sandbox,
        "quits-after-lock",
        &["--ending-file", &text(&file)],
    )
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();
    wait_for(&sandbox.root.join("ready"), Duration::from_secs(20));
    let connection = Connection::open(sandbox.default_store()).unwrap();
    connection.execute_batch("BEGIN EXCLUSIVE").unwrap();
    fs::write(sandbox.root.join("go"), "").unwrap();
    wait_for(&file, Duration::from_secs(10));
    let published_while_locked = file.exists();
    connection.execute_batch("COMMIT").unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        end_notice(&stderr)["recorded"],
        true,
        "a lock timeout preceded publication: {stderr}"
    );
    assert!(
        published_while_locked,
        "the ending file waited on the store"
    );
    assert_eq!(
        ending_document(&file)["measurements"]["ending"]["value"],
        "harness_exit"
    );
}

/// An ending file that exists refuses before selection, with no policy needed
/// to see it, and so does one in a directory that does not. The control is the
/// same run with a usable path, which reaches the policy and refuses that it
/// is missing. Nothing replaces the file that was there.
#[test]
fn an_unusable_ending_file_refuses_before_selection() {
    let sandbox = Sandbox::new();
    let existing = sandbox.file("existing.json", "keep me");
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--ending-file",
            &text(&existing),
            "--json",
        ])
        .refusal(2);
    assert_eq!(
        refusal["error"]["code"], "ending_file_unusable",
        "{refusal}"
    );
    assert_eq!(refusal["error"]["input"], "--ending-file", "{refusal}");
    assert_eq!(fs::read_to_string(&existing).unwrap(), "keep me");

    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--ending-file",
            "absent/ending.json",
            "--json",
        ])
        .refusal(2);
    assert_eq!(
        refusal["error"]["code"], "ending_file_unusable",
        "{refusal}"
    );

    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--ending-file",
            "ending.json",
            "--json",
        ])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_missing", "{refusal}");
}

/// A run records its end observation in a version-1 store with no import
/// between, and the end observation migrates it. A failed append leaves the
/// store at version 1, is reported on stderr, and changes neither the ending
/// nor the exit, nor the ending file.
#[test]
fn the_end_observation_migrates_a_version_1_store_and_a_failed_append_does_not() {
    const VERSION_1_STORE: &str = "
CREATE TABLE runs (run_id TEXT PRIMARY KEY NOT NULL, recorded_at TEXT NOT NULL, launch TEXT NOT NULL);
CREATE TABLE launch_failures (run_id TEXT PRIMARY KEY NOT NULL REFERENCES runs (run_id), recorded_at TEXT NOT NULL, detail TEXT NOT NULL);
CREATE TRIGGER runs_never_change BEFORE UPDATE ON runs BEGIN SELECT RAISE(ABORT, 'a committed run''s launch fields never change'); END;
CREATE TRIGGER runs_are_never_removed BEFORE DELETE ON runs BEGIN SELECT RAISE(ABORT, 'a committed run is never removed'); END;
CREATE TRIGGER launch_failures_never_change BEFORE UPDATE ON launch_failures BEGIN SELECT RAISE(ABORT, 'a recorded launch failure never changes'); END;
CREATE TRIGGER launch_failures_are_never_removed BEFORE DELETE ON launch_failures BEGIN SELECT RAISE(ABORT, 'a recorded launch failure is never removed'); END;
PRAGMA application_id = 1212437075;
PRAGMA user_version = 1;
";
    let sandbox = sandbox();
    let store = sandbox.default_store();
    let directory = store.parent().unwrap().to_owned();
    fs::create_dir_all(&directory).unwrap();
    Connection::open(&store)
        .unwrap()
        .execute_batch(VERSION_1_STORE)
        .unwrap();
    assert_eq!(user_version(&store), 1);

    // The harness takes away the store directory's write permission, so its
    // journal cannot be created and the append fails after the commit.
    let file = sandbox.root.join("failed.json");
    fake(
        &sandbox,
        "locks",
        &format!("chmod 500 '{}'\nexit 5\n", directory.display()),
    );
    let failed = run(&mut front(
        &sandbox,
        "locks",
        &["--ending-file", &text(&file)],
    ));
    fs::set_permissions(&directory, fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(failed.code, Some(5), "{}", failed.stderr);
    let end = end_notice(&failed.stderr);
    assert_eq!(end["ending"], "harness_exit", "{end}");
    assert_eq!(end["exitCode"], 5, "{end}");
    assert_eq!(end["recorded"], false, "{end}");
    assert!(
        failed
            .stderr
            .contains("its end observation was not recorded"),
        "{}",
        failed.stderr
    );
    assert_eq!(
        user_version(&store),
        1,
        "a failed append migrated the store"
    );
    assert!(file.exists(), "the ending file waited on the store");
    assert_eq!(
        show(&sandbox, &run_of(&end))["evidence"],
        "handoff_attempt",
        "an end observation that was not recorded still confirmed the run"
    );

    // The same run, with the store writable, records its end and migrates.
    fake(&sandbox, "quits", "exit 0\n");
    let ran = run(&mut front(&sandbox, "quits", &[]));
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let end = end_notice(&ran.stderr);
    assert_eq!(end["recorded"], true, "{end}");
    assert_eq!(user_version(&store), 2);
    let export = show(&sandbox, &run_of(&end));
    assert_eq!(export["evidence"], "execution_confirmed", "{export}");
}

/// A dispatch killed while its harness runs leaves the attempt as it stood:
/// execution unknown, no end observation and no ending file. Outside imports
/// still append to and correct a supervised run, its end observation included.
#[test]
fn a_killed_dispatch_records_no_end_and_imports_still_append_and_correct() {
    let sandbox = sandbox();
    let file = sandbox.root.join("killed.json");
    fake(
        &sandbox,
        "busy",
        "echo $$ > \"$ROOT/harness\"\n: > \"$ROOT/ready\"\nwhile : ; do sleep 0.05 ; done\n",
    );
    let mut command = front(&sandbox, "busy", &["--ending-file", &text(&file)]);
    // No pipes: the orphaned harness would hold their write ends open, and
    // there would be nothing to read them to the end.
    let mut child = command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    wait_for(&sandbox.root.join("ready"), Duration::from_secs(20));
    child.kill().unwrap();
    let status = child.wait().unwrap();
    assert_eq!(status.signal(), Some(libc::SIGKILL));
    // The orphaned harness is nothing's to escalate; end it here.
    let harness: libc::pid_t = read(&sandbox.root.join("harness")).parse().unwrap();
    // SAFETY: the test's own orphaned fake.
    unsafe { libc::kill(harness, libc::SIGKILL) };
    assert!(!file.exists(), "a killed dispatch wrote an ending file");
    let store = sandbox.default_store();
    let observations: i64 = Connection::open(&store)
        .unwrap()
        .query_row("SELECT count(*) FROM observations", [], |row| row.get(0))
        .unwrap();
    assert_eq!(observations, 0, "a killed dispatch recorded an end");
    let run_id: String = Connection::open(&store)
        .unwrap()
        .query_row("SELECT run_id FROM runs", [], |row| row.get(0))
        .unwrap();
    let export = show(&sandbox, &run_id);
    assert_eq!(export["evidence"], "handoff_attempt", "{export}");
    assert_eq!(export["execution"], "unknown", "{export}");

    // A run that ended, then outside observations: one appended, one
    // correcting dispatch's own end observation.
    fake(&sandbox, "quits", "exit 2\n");
    let ran = run(&mut front(&sandbox, "quits", &[]));
    assert_eq!(ran.code, Some(2), "{}", ran.stderr);
    let run_id = run_of(&end_notice(&ran.stderr));
    let end_id = format!("harness-dispatch-end-{run_id}");
    let import = |name: &str, document: Value| {
        let path = sandbox.file(name, &serde_json::to_string(&document).unwrap());
        let mut command = sandbox.command();
        command.args([
            "record",
            "observe",
            "--run",
            &run_id,
            "--file",
            &text(&path),
            "--json",
        ]);
        run(&mut command)
    };
    let observation = |id: &str, measurements: Value| {
        json!({
            "schemaVersion": 1,
            "observationId": id,
            "runId": run_id,
            "source": "an outside observer",
            "observedAt": "2026-10-01T09:30:00Z",
            "evidence": "what the observer saw",
            "measurements": measurements,
        })
    };
    let appended = import(
        "accepted.json",
        observation(
            "accepted",
            json!({ "acceptance": { "state": "observed", "value": "accepted" } }),
        ),
    );
    assert_eq!(appended.code, Some(0), "{}", appended.stderr);
    let mut correction = observation(
        "corrected-end",
        json!({ "exit": { "state": "observed", "value": { "code": 0 } } }),
    );
    correction["supersedes"] = json!(end_id);
    let corrected = import("corrected.json", correction);
    assert_eq!(corrected.code, Some(0), "{}", corrected.stderr);
    let export = show(&sandbox, &run_id);
    let listed: Vec<&Value> = export["observations"].as_array().unwrap().iter().collect();
    assert_eq!(listed.len(), 3, "{export}");
    assert_eq!(listed[0]["observationId"], json!(end_id));
    assert_eq!(listed[0]["supersededBy"], "corrected-end", "{export}");
    assert_eq!(export["measurements"]["acceptance"]["state"], "observed");
    assert_eq!(
        export["measurements"]["exit"]["current"],
        json!([{
            "observationId": "corrected-end",
            "state": "observed",
            "value": { "code": 0 },
        }])
    );
}

/// A run that started no harness has no ending: a refusal before the handoff
/// and a harness that could not be started write no ending file, record no
/// end observation, and exit as the diagnostics say.
#[test]
fn a_run_that_started_no_harness_has_no_ending() {
    let sandbox = sandbox();
    let file = sandbox.root.join("never.json");
    let ran = run(&mut front(
        &sandbox,
        "no-such-harness",
        &["--ending-file", &text(&file)],
    ));
    assert_eq!(ran.code, Some(127), "{}", ran.stderr);
    assert!(!file.exists(), "a refused run wrote an ending file");
    assert!(
        !sandbox.default_store().exists(),
        "a refused run recorded something"
    );

    // Committed, then not started: its `#!` interpreter does not exist.
    executable(&sandbox.bin.join("broken"), "#!/nonexistent/interpreter\n");
    let ran = run(&mut front(
        &sandbox,
        "broken",
        &["--ending-file", &text(&file)],
    ));
    assert_eq!(ran.code, Some(127), "{}", ran.stderr);
    assert!(!file.exists(), "an unstarted harness wrote an ending file");
    let observations: i64 = Connection::open(sandbox.default_store())
        .unwrap()
        .query_row("SELECT count(*) FROM observations", [], |row| row.get(0))
        .unwrap();
    assert_eq!(observations, 0, "{}", ran.stderr);
}
