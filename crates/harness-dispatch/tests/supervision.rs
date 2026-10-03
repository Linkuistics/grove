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

use serde_json::Value;
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
