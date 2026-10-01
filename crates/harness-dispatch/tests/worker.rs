//! Finding and verifying the worker, through the command seam.
//!
//! The worker is found only at `../libexec/harness-dispatch/` from the real
//! front executable. These tests run copies of the front in private prefixes,
//! so the layout under test is the one each test builds, and decoy workers on
//! PATH, in the cwd and in the environment would announce themselves if used.

mod support;

use std::fs;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::json;
use support::{executable, run, text, Run, Sandbox, FRONT, ROUTED};

const LAYOUT: &str = "libexec/harness-dispatch/harness-dispatch-policy";

/// A copy of the front at `<prefix>/bin/harness-dispatch`, with no worker.
fn copied_front(sandbox: &Sandbox) -> (PathBuf, PathBuf) {
    let prefix = sandbox.root.join("prefix");
    let front = prefix.join("bin/harness-dispatch");
    fs::create_dir_all(front.parent().unwrap()).unwrap();
    fs::copy(FRONT, &front).unwrap();
    (front, prefix)
}

/// A shell script that records it ran; it speaks no protocol.
fn decoy(sentinel: &Path) -> String {
    format!("#!/bin/sh\necho ran >> '{}'\n", text(sentinel))
}

/// A fake worker that sends `hello` as its first frame and exits, so a front
/// that accepts it finds the channel closed at once rather than waiting out
/// the selection deadline.
fn fake_worker(sandbox: &Sandbox, path: &Path, hello: &serde_json::Value) {
    executable(
        path,
        &format!(
            "#!/bin/sh\nexec /bin/cat '{}' >&3\n",
            text(&framed(sandbox, hello))
        ),
    );
}

/// A file holding `hello` as one frame: its length, then its JSON.
fn framed(sandbox: &Sandbox, hello: &serde_json::Value) -> PathBuf {
    let body = serde_json::to_vec(hello).unwrap();
    let mut frame = u32::try_from(body.len()).unwrap().to_be_bytes().to_vec();
    frame.extend(body);
    let frame_file = sandbox.root.join(format!(
        "frame-{}",
        fs::read_dir(&sandbox.root).unwrap().count()
    ));
    fs::write(&frame_file, frame).unwrap();
    frame_file
}

/// A fake worker that sends `hello` as its first frame and records in
/// `received` the frame it is then sent, byte for byte, or nothing when the
/// front closes the channel without sending one. `received` appears only once
/// the recording is whole.
///
/// The front kills a worker it refuses. So the recording is a child of the
/// worker's, started before the hello is sent, which that kill does not
/// reach: a frame the front sent before it refused is still read.
fn recording_worker(sandbox: &Sandbox, path: &Path, hello: &serde_json::Value, received: &Path) {
    executable(
        path,
        &format!(
            "#!/bin/sh\n\
             (\n  \
               set -- $(dd bs=1 count=4 <&3 2>/dev/null | od -An -tu1)\n  \
               if [ $# -eq 4 ]; then\n    \
                 dd bs=1 count=$(( ($1 << 24) + ($2 << 16) + ($3 << 8) + $4 )) <&3 2>/dev/null > '{received}.part'\n  \
               else\n    \
                 : > '{received}.part'\n  \
               fi\n  \
               mv '{received}.part' '{received}'\n\
             ) > /dev/null 2>&1 &\n\
             /bin/cat '{hello}' >&3\n\
             wait\n",
            hello = text(&framed(sandbox, hello)),
            received = text(received),
        ),
    );
}

/// What a recording worker was sent, once its recording is whole.
fn recorded(received: &Path) -> String {
    let started = Instant::now();
    while !received.exists() {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the fake worker recorded nothing at {}: it never ran, or its channel never closed",
            received.display()
        );
        thread::sleep(Duration::from_millis(5));
    }
    fs::read_to_string(received).unwrap()
}

/// Run the front as the leader of a new process group, and say whether any
/// member of that group outlived it. The front forks its worker into its own
/// group, so the answer covers the worker from the fork on, whether or not it
/// ever ran a line of its own.
fn run_leading_a_group(command: &mut Command) -> (Run, bool) {
    // A PGID of 0 makes the child's PID its group's ID:
    // https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.process_group
    let child = command
        .process_group(0)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the front executable runs");
    let group = libc::pid_t::try_from(child.id()).unwrap();
    let run = Run::from(child.wait_with_output().expect("the front exits"));
    // SAFETY: signal 0 sent to the negated group ID only checks whether the
    // group has a member. The front is reaped, so a member is a process it
    // left behind.
    let result = unsafe { libc::kill(-group, 0) };
    let outlived =
        result == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH);
    (run, outlived)
}

#[test]
fn a_symlink_to_the_front_still_finds_its_worker() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let link = sandbox.root.join("links/harness-dispatch");
    fs::create_dir_all(link.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(FRONT, &link).unwrap();

    let mut command = sandbox.command_for(&link);
    command.args(["inspect", "--kind", "impl", "--json"]);
    let report = run(&mut command).report();

    let real_bin = fs::canonicalize(FRONT).unwrap();
    let expected = real_bin.parent().unwrap().parent().unwrap().join(LAYOUT);
    assert_eq!(report["worker"]["path"], text(&expected));
    assert_eq!(report["selection"]["candidateId"], "deep");
}

#[test]
fn a_front_without_its_worker_refuses_and_no_ambient_decoy_substitutes() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let (front, prefix) = copied_front(&sandbox);
    let sentinel = sandbox.root.join("decoy-ran");
    let decoy_bin = sandbox.root.join("decoy-bin");
    for place in [
        decoy_bin.join("harness-dispatch-policy"),
        sandbox.cwd.join("harness-dispatch-policy"),
        sandbox.cwd.join(LAYOUT),
        sandbox.root.join("env-decoy"),
    ] {
        executable(&place, &decoy(&sentinel));
    }

    let mut command = sandbox.command_for(&front);
    command
        .env("PATH", format!("{}:/usr/bin:/bin", text(&decoy_bin)))
        .env("HARNESS_DISPATCH_WORKER", sandbox.root.join("env-decoy"))
        .env(
            "HARNESS_DISPATCH_POLICY_WORKER",
            sandbox.root.join("env-decoy"),
        )
        .args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut command).refusal(5);

    assert_eq!(refusal["error"]["code"], "worker_missing");
    assert_eq!(refusal["error"]["stage"], "worker");
    assert_eq!(refusal["error"]["source"], text(&prefix.join(LAYOUT)));
    assert!(refusal["error"]["remedy"]
        .as_str()
        .unwrap()
        .contains("task dispatch:worker"));
    assert!(!sentinel.exists(), "an ambient decoy was run as the worker");

    // The firing configuration: the same decoy at the layout path does run.
    executable(&prefix.join(LAYOUT), &decoy(&sentinel));
    let refusal = run(&mut command).refusal(5);
    assert!(
        sentinel.exists(),
        "the decoy at the layout path did not run"
    );
    assert_eq!(refusal["error"]["code"], "worker_failed", "{refusal}");
}

#[test]
fn the_front_starts_its_worker_in_a_private_empty_directory_and_removes_it() {
    // The real worker leaves its start directory before any policy runs, so a
    // stand-in at the layout path says where the front started it.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let (front, prefix) = copied_front(&sandbox);
    let seen = sandbox.root.join("start-directory");
    executable(
        &prefix.join(LAYOUT),
        &format!(
            "#!/bin/sh\n{{ pwd -P; ls -ld . | cut -c1-10; ls -A; }} > '{}'\n",
            text(&seen)
        ),
    );

    let mut command = sandbox.command_for(&front);
    command.args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut command).refusal(5);
    assert_eq!(refusal["error"]["code"], "worker_failed", "{refusal}");

    let seen = fs::read_to_string(&seen).expect("the stand-in worker ran");
    let lines: Vec<&str> = seen.lines().collect();
    // Its path and its mode, and no line for an entry: it is empty.
    let [started, mode] = lines[..] else {
        panic!("the start directory is not empty: {seen}");
    };
    let started = Path::new(started);
    assert!(
        started.starts_with(fs::canonicalize(&sandbox.tmp).unwrap()),
        "{seen}"
    );
    assert_ne!(started, fs::canonicalize(&sandbox.cwd).unwrap());
    assert_eq!(mode, "drwx------", "{seen}");
    assert!(
        !started.exists(),
        "the private directory outlived the worker"
    );
}

#[test]
fn an_argv0_naming_another_prefix_never_relocates_the_worker() {
    // argv[0] is whatever the caller passes to exec; the front's own path
    // comes from the operating system (`std::env::current_exe`).
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let decoy_prefix = sandbox.root.join("decoy-prefix");
    let sentinel = sandbox.root.join("decoy-ran");
    executable(&decoy_prefix.join(LAYOUT), &decoy(&sentinel));
    let spoofed = decoy_prefix.join("bin/harness-dispatch");

    let mut command = sandbox.command();
    command
        .arg0(&spoofed)
        .args(["inspect", "--kind", "impl", "--json"]);
    let report = run(&mut command).report();
    let real_bin = fs::canonicalize(FRONT).unwrap();
    let expected = real_bin.parent().unwrap().parent().unwrap().join(LAYOUT);
    assert_eq!(report["worker"]["path"], text(&expected));
    assert!(!sentinel.exists(), "the decoy named by argv[0] was run");

    // The firing configuration: a front really at that path runs the decoy.
    fs::create_dir_all(spoofed.parent().unwrap()).unwrap();
    fs::copy(FRONT, &spoofed).unwrap();
    let mut command = sandbox.command_for(&spoofed);
    command.args(["inspect", "--kind", "impl", "--json"]);
    run(&mut command).refusal(5);
    assert!(sentinel.exists(), "the decoy prefix's worker never ran");
}

#[test]
fn a_worker_that_never_identifies_itself_is_stopped_at_the_deadline() {
    // The bound counts from the worker's start, not from handing it the
    // entry. This worker never says hello; if the deadline did not hold, its
    // sleep would end first and the front would refuse with exit 5 instead.
    // So exit 124 is the evidence here. How promptly the bound acts is
    // asserted in `deadline.rs`, with the built front: a fresh copy like this
    // one pays a first-exec cost, which under parallel load has taken seconds.
    //
    // The freshly written worker pays that cost inside the bound, so under
    // load it is often stopped before its first line runs. Nothing it could
    // record is therefore evidence; the front's process group is.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let (front, prefix) = copied_front(&sandbox);
    executable(&prefix.join(LAYOUT), "#!/bin/sh\nexec /bin/sleep 30\n");

    let mut command = sandbox.command_for(&front);
    command.args([
        "inspect",
        "--kind",
        "impl",
        "--timeout-ms",
        "1000",
        "--json",
    ]);
    let (run, outlived) = run_leading_a_group(&mut command);
    let refusal = run.refusal(124);

    assert_eq!(refusal["error"]["code"], "selection_timeout", "{refusal}");
    assert_eq!(refusal["error"]["stage"], "evaluation");
    assert_eq!(refusal["error"]["source"], text(&prefix.join(LAYOUT)));
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("was not ready for the policy"),
        "{refusal}"
    );
    assert!(!outlived, "the worker survived the front");
}

#[test]
fn a_worker_from_another_build_refuses_before_it_is_given_a_policy() {
    // Each worker here records the frame it is sent after its hello. A
    // worker whose hello is refused is sent none: the front closes the
    // channel on it, and the policy's entry and the request never leave the
    // front.
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(ROUTED);
    let (front, prefix) = copied_front(&sandbox);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let build_id = report["worker"]["buildId"].as_str().unwrap().to_owned();
    let version = env!("CARGO_PKG_VERSION");
    let received = sandbox.root.join("received");

    for (name, hello) in [
        (
            "another build",
            json!({"type": "hello", "protocol": 1, "packageVersion": version, "buildId": "0".repeat(64), "bunVersion": "1.4.2"}),
        ),
        (
            "another version",
            json!({"type": "hello", "protocol": 1, "packageVersion": "0.0.1", "buildId": build_id, "bunVersion": "1.4.2"}),
        ),
        (
            "another protocol",
            json!({"type": "hello", "protocol": 2, "packageVersion": version, "buildId": build_id, "bunVersion": "1.4.2"}),
        ),
    ] {
        recording_worker(&sandbox, &prefix.join(LAYOUT), &hello, &received);
        let mut command = sandbox.command_for(&front);
        command.args(["inspect", "--kind", "impl", "--json"]);
        let refusal = run(&mut command).refusal(5);
        assert_eq!(
            refusal["error"]["code"], "worker_identity_mismatch",
            "{name}: {refusal}"
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains("not this front's pair"),
            "{name}"
        );
        assert_eq!(
            recorded(&received),
            "",
            "{name}: the front sent a frame to a worker it refused"
        );
        fs::remove_file(&received).unwrap();
    }

    // The positive control: a hello carrying the real identity is accepted,
    // and the same recording then holds the policy's entry. This fake fails
    // for want of a result, not for its identity.
    recording_worker(
        &sandbox,
        &prefix.join(LAYOUT),
        &json!({"type": "hello", "protocol": 1, "packageVersion": version, "buildId": build_id, "bunVersion": "1.4.2"}),
        &received,
    );
    let mut command = sandbox.command_for(&front);
    command.args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut command).refusal(5);
    assert_eq!(refusal["error"]["code"], "worker_failed", "{refusal}");
    let message: serde_json::Value = serde_json::from_str(&recorded(&received)).unwrap();
    assert_eq!(message["type"], "evaluate");
    assert_eq!(message["entry"], text(&entry));
}

#[test]
fn a_worker_that_breaks_the_protocol_refuses_with_exit_5() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let (front, prefix) = copied_front(&sandbox);

    executable(
        &prefix.join(LAYOUT),
        "#!/bin/sh\nprintf 'not a frame at all' >&3\n",
    );
    let mut command = sandbox.command_for(&front);
    command.args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut command).refusal(5);
    assert_eq!(refusal["error"]["code"], "protocol_error", "{refusal}");

    fake_worker(&sandbox, &prefix.join(LAYOUT), &json!({"type": "greeting"}));
    let refusal = run(&mut command).refusal(5);
    assert_eq!(refusal["error"]["code"], "protocol_error", "{refusal}");

    executable(
        &prefix.join(LAYOUT),
        "#!/bin/sh\necho 'worker crashed' >&2\nexit 9\n",
    );
    let refusal = run(&mut command).refusal(5);
    assert_eq!(refusal["error"]["code"], "worker_failed", "{refusal}");
    assert!(
        refusal["diagnostics"]["stderr"]
            .as_str()
            .unwrap()
            .contains("worker crashed"),
        "{refusal}"
    );
}

#[test]
fn the_request_carries_the_task_inputs_explicit_choice_and_limits_and_never_the_prompt() {
    // A fake worker with the real identity records the evaluate frame it is
    // sent, byte for byte, and exits; the front then refuses for want of a
    // result. The frame is everything the worker ever learns of the caller:
    // the select frame that may follow it names no input at all.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let (front, prefix) = copied_front(&sandbox);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let hello = json!({
        "type": "hello", "protocol": 1, "packageVersion": env!("CARGO_PKG_VERSION"),
        "buildId": report["worker"]["buildId"], "bunVersion": "1.4.2",
    });
    let request_file = sandbox.root.join("request.json");
    recording_worker(&sandbox, &prefix.join(LAYOUT), &hello, &request_file);
    let prompt_file = sandbox.file("mandate.md", "file-prompt-token\n");

    for prompt in [
        ["--prompt", "argument-prompt-token"],
        ["--prompt-file", "mandate.md"],
    ] {
        let _ = fs::remove_file(&request_file);
        let mut command = sandbox.command_for(&front);
        command
            .args(["inspect", "--kind", "impl", "--json"])
            .args([
                "--task-file",
                "tasks/t.md",
                "--task-id",
                "T-7",
                "--choice",
                "deep",
            ])
            .args(prompt);
        let refusal = run(&mut command).refusal(5);
        assert_eq!(refusal["error"]["code"], "worker_failed", "{refusal}");

        let raw = recorded(&request_file);
        let message: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(message["type"], "evaluate");
        assert_eq!(
            message["request"],
            json!({
                "schemaVersion": 1,
                "kind": "impl",
                "cwd": text(&sandbox.cwd),
                "taskFile": text(&sandbox.cwd.join("tasks/t.md")),
                "taskId": "T-7",
                "explicitChoice": "deep",
                "limits": {
                    "selectionMs": 30_000, "contextBytes": 262_144, "sourceBytes": 65_536,
                    "sources": 256, "messageBytes": 1_048_576, "diagnosticsBytes": 262_144,
                },
            })
        );
        // The worker's own copies of the bounds, and no measured source
        // without a --context document.
        assert_eq!(
            message["bounds"],
            json!({
                "contextBytes": 262_144, "sourceBytes": 65_536, "sources": 256,
                "messageBytes": 1_048_576,
            })
        );
        assert_eq!(message["measured"], json!([]));
        for leak in ["prompt-token", "mandate.md", text(&prompt_file).as_str()] {
            assert!(!raw.contains(leak), "{leak} reached the worker: {raw}");
        }
    }
}
