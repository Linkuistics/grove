//! Finding and verifying the worker, through the command seam.
//!
//! The worker is found only at `../libexec/harness-dispatch/` from the real
//! front executable. These tests run copies of the front in private prefixes,
//! so the layout under test is the one each test builds, and decoy workers on
//! PATH, in the cwd and in the environment would announce themselves if used.

mod support;

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::json;
use support::{executable, run, text, Sandbox, FRONT, ROUTED};

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

/// A fake worker that sends `hello` as its first frame and exits. Nothing ends
/// a worker that waits forever until the selection deadline arrives, so this
/// one never waits: a front that accepts it then finds the channel closed.
fn fake_worker(sandbox: &Sandbox, path: &Path, hello: &serde_json::Value) {
    let body = serde_json::to_vec(hello).unwrap();
    let mut frame = u32::try_from(body.len()).unwrap().to_be_bytes().to_vec();
    frame.extend(body);
    let frame_file = sandbox.root.join(format!(
        "frame-{}",
        fs::read_dir(&sandbox.root).unwrap().count()
    ));
    fs::write(&frame_file, frame).unwrap();
    executable(
        path,
        &format!("#!/bin/sh\nexec /bin/cat '{}' >&3\n", text(&frame_file)),
    );
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
fn a_worker_from_another_build_refuses_before_it_is_given_a_policy() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"fired\");\n{ROUTED}",
        text(&sentinel)
    ));
    let (front, prefix) = copied_front(&sandbox);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let build_id = report["worker"]["buildId"].as_str().unwrap().to_owned();
    let version = env!("CARGO_PKG_VERSION");
    fs::remove_file(&sentinel).unwrap();

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
        fake_worker(&sandbox, &prefix.join(LAYOUT), &hello);
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
    }

    // The positive control: a hello carrying the real identity is accepted,
    // and this fake then fails for want of a result, not for its identity.
    fake_worker(
        &sandbox,
        &prefix.join(LAYOUT),
        &json!({"type": "hello", "protocol": 1, "packageVersion": version, "buildId": build_id, "bunVersion": "1.4.2"}),
    );
    let mut command = sandbox.command_for(&front);
    command.args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut command).refusal(5);
    assert_eq!(refusal["error"]["code"], "worker_failed", "{refusal}");
    assert!(
        !sentinel.exists(),
        "a fake worker cannot have evaluated the policy"
    );
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
fn the_request_carries_the_task_inputs_and_never_the_prompt() {
    // A fake worker with the real identity records the evaluate frame it is
    // sent, byte for byte, and exits; the front then refuses for want of a
    // result. The frame is everything the worker ever learns from the front.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let (front, prefix) = copied_front(&sandbox);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let hello = json!({
        "type": "hello", "protocol": 1, "packageVersion": env!("CARGO_PKG_VERSION"),
        "buildId": report["worker"]["buildId"], "bunVersion": "1.4.2",
    });
    let body = serde_json::to_vec(&hello).unwrap();
    let mut frame = u32::try_from(body.len()).unwrap().to_be_bytes().to_vec();
    frame.extend(body);
    let hello_file = sandbox.root.join("hello-frame");
    fs::write(&hello_file, frame).unwrap();
    let request_file = sandbox.root.join("request.json");
    executable(
        &prefix.join(LAYOUT),
        &format!(
            "#!/bin/sh\n/bin/cat '{hello}' >&3\n\
             set -- $(dd bs=1 count=4 <&3 2>/dev/null | od -An -tu1)\n\
             dd bs=1 count=$(( ($1 << 24) + ($2 << 16) + ($3 << 8) + $4 )) <&3 2>/dev/null > '{request}'\n",
            hello = text(&hello_file),
            request = text(&request_file),
        ),
    );
    let prompt_file = sandbox.file("mandate.md", "file-prompt-token\n");

    for prompt in [
        ["--prompt", "argument-prompt-token"],
        ["--prompt-file", "mandate.md"],
    ] {
        let _ = fs::remove_file(&request_file);
        let mut command = sandbox.command_for(&front);
        command
            .args(["inspect", "--kind", "impl", "--json"])
            .args(["--task-file", "tasks/t.md", "--task-id", "T-7"])
            .args(prompt);
        let refusal = run(&mut command).refusal(5);
        assert_eq!(refusal["error"]["code"], "worker_failed", "{refusal}");

        let raw = fs::read_to_string(&request_file).expect("the fake worker saw a request");
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
            })
        );
        for leak in ["prompt-token", "mandate.md", text(&prompt_file).as_str()] {
            assert!(!raw.contains(leak), "{leak} reached the worker: {raw}");
        }
    }
}
