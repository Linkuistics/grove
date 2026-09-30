//! The run record through the command seam: the handoff commit that precedes
//! every exec, the identity exported to the harness, the failures that launch
//! nothing, the exec failure appended to its attempt, and `record show`
//! (`docs/specs/harness-selection-and-execution.md`, *Records and later
//! observations*).
//!
//! The store is read back two ways: through `record show`, the public export,
//! and directly with SQLite, to see what a test's fault did or did not change.
//! Every failure case has a positive control, the same invocation with the
//! fault removed, that is seen to reach the harness.

mod support;

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};

use rusqlite::Connection;
use serde_json::Value;
use sha2::{Digest as _, Sha256};
use support::{executable, run, text, Sandbox, ROUTED};

/// Routes `impl` to the fake harness, passing the run ID in its arguments too.
const WITH_RUN_ID: &str = r#"export const policy = {
  schemaVersion: 1,
  version: "records-1",
  catalog: [
    { id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: ["--run", { slot: "runId" }, { slot: "prompt" }] },
  ],
  routes: { impl: "deep" },
};
"#;

/// Routes `impl` to `./broken-harness`, whose `#!` interpreter does not exist,
/// so exec fails with `ENOENT` after the commit.
const BROKEN: &str = r#"export const policy = {
  schemaVersion: 1,
  version: "records-broken",
  catalog: [
    { id: "broken", provider: "origin-b", model: "model-b", effort: "low", program: "./broken-harness", args: [{ slot: "prompt" }] },
  ],
  routes: { impl: "broken" },
};
"#;

fn is_run_id(id: &str) -> bool {
    id.len() == 36
        && id.bytes().enumerate().all(|(at, byte)| match at {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte),
        })
}

fn runs(store: &Path) -> i64 {
    let connection = Connection::open(store).unwrap();
    connection
        .query_row("SELECT count(*) FROM runs", [], |row| row.get(0))
        .unwrap()
}

fn show(sandbox: &Sandbox, run_id: &str, extra: &[&str]) -> support::Run {
    let mut command = sandbox.command();
    command
        .args(["record", "show", "--run", run_id])
        .args(extra);
    run(&mut command)
}

/// A successful run's ID, taken from the harness it reached.
fn launched(sandbox: &Sandbox, args: &[&str]) -> String {
    let result = sandbox.run(args);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert!(sandbox.harness_ran());
    let run_id = sandbox.harness_run_id();
    fs::remove_dir_all(&sandbox.record).unwrap();
    run_id
}

#[test]
fn a_run_commits_its_handoff_record_before_the_harness_starts() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let result = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(result.code, Some(0), "{}", result.stderr);

    let run_id = sandbox.harness_run_id();
    assert!(is_run_id(&run_id), "{run_id:?}");
    let state_dir = sandbox.home.join(".local/state/harness-dispatch");
    assert_eq!(sandbox.harness_state_dir(), text(&state_dir));

    // The harness found its own run already committed.
    let at_start = Connection::open(sandbox.record.join("store-at-start")).unwrap();
    let recorded: String = at_start
        .query_row(
            "SELECT run_id FROM runs WHERE run_id = ?1",
            [&run_id],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(recorded, run_id);

    let notice: Value = serde_json::from_str(result.stderr.trim_end()).unwrap();
    assert_eq!(notice["handoff"]["runId"], run_id.as_str());
    assert_eq!(notice["handoff"]["stateDir"], text(&state_dir));
    assert!(notice["handoff"]["recordedAt"]
        .as_str()
        .is_some_and(|at| at.ends_with('Z')));

    // Created on first use, privately; the harness inherited no descriptor.
    let mode = |path: &Path| fs::metadata(path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode(&state_dir), 0o700);
    assert_eq!(mode(&sandbox.default_store()), 0o600);
    assert_eq!(sandbox.harness_fds(), Vec::<u32>::new());
}

#[test]
fn the_harness_receives_its_run_identity_in_place_of_inherited_values() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(WITH_RUN_ID);
    let mut command = sandbox.command();
    command
        .args(["run", "--kind", "impl", "--prompt", "p"])
        .env("HARNESS_DISPATCH_RUN_ID", "stale-run")
        .env("HARNESS_DISPATCH_STATE_DIR", "/stale/state");
    let result = run(&mut command);
    assert_eq!(result.code, Some(0), "{}", result.stderr);

    let run_id = sandbox.harness_run_id();
    assert!(is_run_id(&run_id), "{run_id:?}");
    assert_eq!(
        sandbox.harness_state_dir(),
        text(&sandbox.home.join(".local/state/harness-dispatch"))
    );
    assert_eq!(sandbox.harness_args(), ["--run", run_id.as_str(), "p"]);
    assert!(
        result.stderr.contains(&format!("as run {run_id}")),
        "{}",
        result.stderr
    );
}

#[test]
fn every_run_is_a_new_run_with_its_own_identity() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let args = ["--kind", "impl", "--task-id", "T-1", "--prompt", "p"];
    let first = launched(&sandbox, &args);
    let second = launched(&sandbox, &args);
    assert_ne!(first, second);
    assert_eq!(runs(&sandbox.default_store()), 2);
}

#[test]
fn a_state_dir_replaces_the_default_and_resolves_against_the_cwd() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = launched(
        &sandbox,
        &["--kind", "impl", "--state-dir", "records", "--prompt", "p"],
    );
    let state_dir = sandbox.cwd.join("records");
    assert!(state_dir.join("records.sqlite3").is_file());
    assert!(
        !sandbox.home.join(".local").exists(),
        "the default directory was created beside an explicit one"
    );

    let result = sandbox.run(&["--kind", "impl", "--state-dir", "records", "--prompt", "p"]);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert_eq!(sandbox.harness_state_dir(), text(&state_dir));
    let export = show(&sandbox, &run_id, &["--state-dir", "records", "--json"]).report();
    assert_eq!(export["runId"], run_id.as_str());
    let missing = show(&sandbox, &run_id, &["--json"]).refusal(3);
    assert_eq!(missing["error"]["code"], "run_not_found");
}

#[test]
fn inspect_proposes_a_marked_run_id_and_writes_nothing() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(WITH_RUN_ID);
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt", "p", "--json"])
        .report();
    let proposed = report["proposedRunId"].as_str().unwrap();
    assert!(is_run_id(proposed), "{report}");
    assert_eq!(
        report["argv"],
        serde_json::json!(["fake-harness", "--run", { "proposedRunId": proposed }, "p"])
    );
    assert_eq!(
        report["stateDir"],
        serde_json::json!({
            "path": text(&sandbox.home.join(".local/state/harness-dispatch")),
            "from": "default",
        })
    );
    let text_report = sandbox.inspect(&["--kind", "impl", "--prompt", "p"]);
    assert!(
        text_report.stdout.contains("(proposed only")
            && text_report.stdout.contains("<proposed run ID "),
        "{}",
        text_report.stdout
    );
    assert!(
        !sandbox.home.join(".local").exists(),
        "inspection created the record directory"
    );

    // With a store in place, inspection leaves it byte for byte as it was.
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    let before = fs::read(sandbox.default_store()).unwrap();
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt", "p", "--json"])
        .report();
    assert_eq!(fs::read(sandbox.default_store()).unwrap(), before);
    let refusal = show(
        &sandbox,
        report["proposedRunId"].as_str().unwrap(),
        &["--json"],
    )
    .refusal(3);
    assert_eq!(refusal["error"]["code"], "run_not_found");
}

#[test]
fn record_show_exports_the_launch_fields_with_the_attempt_unknown_and_every_measurement_unobserved()
{
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(ROUTED);
    let mut command = sandbox.command();
    command
        .args([
            "run",
            "--kind",
            "impl",
            "--task-id",
            "T-7",
            "--task-file",
            "tasks/t7.md",
            "--timeout-ms",
            "20000",
            "--prompt",
            "Implement the parser",
        ])
        .env("DISTINCTIVE_SECRET", "never-recorded-5d0c9a");
    let result = run(&mut command);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let run_id = sandbox.harness_run_id();

    let export = show(&sandbox, &run_id, &["--json"]).report();
    assert_eq!(export["schemaVersion"], 1);
    assert_eq!(export["runId"], run_id.as_str());
    assert!(export["recordedAt"].as_str().unwrap().ends_with('Z'));
    // The harness exited 0, and that proves nothing: the run is an attempt.
    assert_eq!(export["evidence"], "handoff_attempt");
    assert_eq!(export["execution"], "unknown");
    assert_eq!(export["launchFailure"], Value::Null);
    assert_eq!(export["observations"], serde_json::json!([]));
    let measurements = export["measurements"].as_object().unwrap();
    assert!(measurements.len() >= 10, "{export}");
    for (name, measurement) in measurements {
        assert_eq!(
            measurement,
            &serde_json::json!({ "state": "unobserved", "current": [] }),
            "{name}"
        );
    }

    let launch = &export["launch"];
    let digest: String = Sha256::digest(fs::read(&entry).unwrap())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    assert_eq!(launch["schemaVersion"], 1);
    assert_eq!(launch["kind"], "impl");
    assert_eq!(launch["taskId"], "T-7");
    assert_eq!(launch["taskFile"], text(&sandbox.cwd.join("tasks/t7.md")));
    assert_eq!(launch["cwd"], text(&sandbox.cwd));
    assert_eq!(
        launch["policy"],
        serde_json::json!({
            "path": text(&entry), "authority": "personal", "sha256": digest, "version": "seam-1",
        })
    );
    assert_eq!(
        launch["candidate"],
        serde_json::json!({
            "id": "deep", "provider": "origin-a", "model": "model-large", "effort": "high",
            "program": "fake-harness", "args": [{ "slot": "prompt" }],
        })
    );
    assert_eq!(
        launch["selection"],
        serde_json::json!({
            "form": "routes", "selectedBy": "route", "explicitChoice": null,
            "reason": "routes[\"impl\"] names candidate \"deep\"",
        })
    );
    assert_eq!(
        launch["executable"],
        serde_json::json!({
            "program": "fake-harness", "resolvedBy": "PATH",
            "path": text(&sandbox.bin.join("fake-harness")),
        })
    );
    assert_eq!(
        launch["argv"],
        serde_json::json!(["fake-harness", "Implement the parser"])
    );
    assert_eq!(
        launch["bounds"],
        serde_json::json!({
            "selection": { "ms": 20000, "from": "--timeout-ms" },
            "context": { "bytes": 262_144, "from": "default" },
            "source": { "bytes": 65_536, "from": "default" },
            "sources": { "sources": 256, "from": "fixed" },
            "message": { "bytes": 1_048_576, "from": "fixed" },
            "diagnostics": { "bytes": 262_144, "from": "fixed" },
        })
    );
    assert!(launch["timing"]["selectionMs"].is_u64());
    assert_eq!(
        launch["worker"]["packageVersion"],
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(launch["worker"]["bunVersion"], "1.4.2");
    // A run given no context records none, no reviewed artifact and no
    // creator, and a policy that imports no adapter records none.
    for absent in ["reviewedArtifact", "context", "creator", "adapter"] {
        assert_eq!(launch[absent], Value::Null, "{absent}");
    }

    // No raw environment value reaches the store.
    let store = fs::read(sandbox.default_store()).unwrap();
    let contains = |needle: &[u8]| store.windows(needle.len()).any(|window| window == needle);
    assert!(contains(b"Implement the parser"), "the store is readable");
    assert!(!contains(b"never-recorded-5d0c9a"));

    let text_export = show(&sandbox, &run_id, &[]);
    assert_eq!(text_export.code, Some(0), "{}", text_export.stderr);
    for expected in [
        run_id.as_str(),
        "handoff attempt",
        "unobserved every measurement",
        "origin-a",
        "\"Implement the parser\"",
    ] {
        assert!(
            text_export.stdout.contains(expected),
            "{expected}:\n{}",
            text_export.stdout
        );
    }
}

#[test]
fn record_show_refuses_a_malformed_id_an_unknown_run_and_a_missing_store() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let unknown = "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e";
    let refusal = show(&sandbox, unknown, &["--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "run_not_found");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("there is no record store"),
        "{refusal}"
    );
    assert!(
        !sandbox.home.join(".local").exists(),
        "record show created a store"
    );

    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    let refusal = show(&sandbox, unknown, &["--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "run_not_found");
    assert_eq!(refusal["error"]["stage"], "record");

    for malformed in ["ABC", &unknown.to_uppercase()] {
        let refusal = show(&sandbox, malformed, &["--json"]).refusal(2);
        assert_eq!(refusal["error"]["code"], "malformed_input", "{malformed}");
        assert_eq!(refusal["error"]["input"], "--run");
    }
}

#[test]
fn a_refusal_before_the_commit_creates_no_run() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(
        r#"export const policy = {
  schemaVersion: 1,
  version: "v",
  catalog: [
    { id: "deep", provider: "o", model: "m", effort: "e", program: "fake-harness", args: [{ slot: "prompt" }] },
    { id: "gone", provider: "o", model: "m", effort: "e", program: "no-such-harness", args: [{ slot: "prompt" }] },
  ],
  routes: { impl: "deep", missing: "gone" },
};
"#,
    );
    let unrouted = sandbox.run(&["--kind", "design", "--prompt", "p", "--json"]);
    assert_eq!(unrouted.refusal(3)["error"]["code"], "incomplete_mapping");
    let absent = sandbox.run(&["--kind", "missing", "--prompt", "p", "--json"]);
    assert_eq!(absent.refusal(127)["error"]["code"], "program_not_found");
    assert!(
        !sandbox.home.join(".local").exists(),
        "a refused run created the record directory"
    );

    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    sandbox
        .run(&["--kind", "design", "--prompt", "p", "--json"])
        .refusal(3);
    sandbox
        .run(&["--kind", "missing", "--prompt", "p", "--json"])
        .refusal(127);
    assert_eq!(runs(&sandbox.default_store()), 1);
    assert!(!sandbox.harness_ran());
}

#[test]
fn an_unwritable_record_directory_launches_nothing() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let locked = sandbox.cwd.join("locked");
    fs::create_dir(&locked).unwrap();
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o500)).unwrap();
    let args = [
        "--kind",
        "impl",
        "--state-dir",
        "locked",
        "--prompt",
        "p",
        "--json",
    ];

    let refusal = sandbox.run(&args).refusal(4);
    assert_eq!(refusal["error"]["code"], "record_store_unwritable");
    assert_eq!(refusal["error"]["stage"], "record");
    assert!(!sandbox.harness_ran());

    // A state directory that is a file is no better.
    sandbox.file("a-file", "not a directory\n");
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--state-dir",
            "a-file",
            "--prompt",
            "p",
            "--json",
        ])
        .refusal(4);
    assert_eq!(refusal["error"]["code"], "record_store_unwritable");
    assert!(!sandbox.harness_ran());

    // Positive control: the same invocation, once the directory is writable.
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o700)).unwrap();
    let result = sandbox.run(&args);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert!(sandbox.harness_ran());
}

#[test]
fn the_directories_a_first_run_creates_are_synced_before_it_launches() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    // Writable and searchable, so the record directories can be created in
    // it, but unreadable, so it cannot be opened to sync its new entry.
    let parent = sandbox.cwd.join("parent");
    fs::create_dir(&parent).unwrap();
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o300)).unwrap();
    let run = |state_dir: &str| {
        sandbox.run(&[
            "--kind",
            "impl",
            "--state-dir",
            state_dir,
            "--prompt",
            "p",
            "--json",
        ])
    };

    let refusal = run("parent/first/records").refusal(4);
    let error = &refusal["error"];
    assert_eq!(error["code"], "record_store_unwritable");
    assert_eq!(error["stage"], "record");
    let message = error["message"].as_str().unwrap();
    assert!(
        message.contains(&format!("{} cannot be synced", text(&parent))),
        "{refusal}"
    );
    assert!(!sandbox.harness_ran());

    // Positive control: a new hierarchy under the same parent, once the parent
    // can be opened, is created, synced and committed before the launch.
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o700)).unwrap();
    let result = run("parent/second/records");
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert!(sandbox.harness_ran());
    assert_eq!(runs(&parent.join("second/records/records.sqlite3")), 1);
}

#[test]
fn a_lock_held_past_the_wait_launches_nothing_and_spends_no_selection_time() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);

    // The policy marks when it runs, so the wait can be timed from the end of
    // selection rather than from the front's start, which a loaded machine
    // can delay by seconds.
    let marker = sandbox.root.join("selected");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"\");\n{ROUTED}",
        text(&marker)
    ));
    let holder = Connection::open(sandbox.default_store()).unwrap();
    holder.execute_batch("BEGIN EXCLUSIVE").unwrap();
    // A one-second selection bound: were the lock wait spent from it, this
    // would time out (exit 124) rather than refuse on the lock.
    let started = Instant::now();
    let mut command = sandbox.command();
    command
        .args([
            "run",
            "--kind",
            "impl",
            "--timeout-ms",
            "1000",
            "--prompt",
            "p",
            "--json",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let child = command.spawn().unwrap();
    while !marker.exists() {
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "the policy never ran"
        );
        thread::sleep(Duration::from_millis(5));
    }
    let selected = Instant::now();
    let result = support::Run::from(child.wait_with_output().unwrap());
    let (total, waited) = (started.elapsed(), selected.elapsed());
    let refusal = result.refusal(4);
    assert_eq!(refusal["error"]["code"], "record_store_locked");
    assert!(
        total >= Duration::from_secs(2) && waited >= Duration::from_millis(1950),
        "the front gave up after {waited:?} of waiting, within its 2-second lock wait"
    );
    assert!(
        waited < Duration::from_millis(4500),
        "the lock wait was not the fixed 2 seconds: {waited:?} after selection"
    );
    assert!(!sandbox.harness_ran());
    assert_eq!(runs_while_locked(&holder), 1);

    holder.execute_batch("COMMIT").unwrap();
    let result = sandbox.run(&["--kind", "impl", "--timeout-ms", "1000", "--prompt", "p"]);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert!(sandbox.harness_ran());
}

fn runs_while_locked(holder: &Connection) -> i64 {
    holder
        .query_row("SELECT count(*) FROM runs", [], |row| row.get(0))
        .unwrap()
}

#[test]
fn no_lock_is_held_while_the_policy_evaluates() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);

    let marker = sandbox.root.join("evaluating");
    sandbox.personal_policy(&format!(
        r#"import {{ writeFileSync }} from "node:fs";
writeFileSync({marker:?}, "evaluating");
await new Promise((resolve) => setTimeout(resolve, 2500));
{ROUTED}"#,
        marker = text(&marker)
    ));
    let mut command = sandbox.command();
    command
        .args(["run", "--kind", "impl", "--prompt", "p"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let child = command.spawn().unwrap();

    let deadline = Instant::now() + Duration::from_secs(20);
    while !marker.exists() {
        assert!(Instant::now() < deadline, "the policy never started");
        thread::sleep(Duration::from_millis(20));
    }
    // With no wait at all, an exclusive lock is free while the policy runs.
    let probe = Connection::open(sandbox.default_store()).unwrap();
    probe.busy_timeout(Duration::ZERO).unwrap();
    probe
        .execute_batch("BEGIN EXCLUSIVE; COMMIT;")
        .expect("the store was locked during evaluation");
    drop(probe);

    let output = child.wait_with_output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(sandbox.harness_ran());
}

#[test]
fn a_store_that_cannot_grow_launches_nothing() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let limited = |sandbox: &Sandbox| {
        let mut command = sandbox.command();
        command.args(["run", "--kind", "impl", "--prompt", "p", "--json"]);
        // SAFETY: signal and setrlimit are async-signal-safe. Writing past
        // 512 bytes of any file then fails with EFBIG, as a full disk fails a
        // write with ENOSPC, instead of raising SIGXFSZ.
        unsafe {
            command.pre_exec(|| {
                libc::signal(libc::SIGXFSZ, libc::SIG_IGN);
                let limit = libc::rlimit {
                    rlim_cur: 512,
                    rlim_max: 512,
                };
                if libc::setrlimit(libc::RLIMIT_FSIZE, &limit) != 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        run(&mut command)
    };

    // On first use, the store cannot be initialized.
    let refusal = limited(&sandbox).refusal(4);
    assert_eq!(refusal["error"]["stage"], "record");
    assert!(!sandbox.harness_ran());
    // A later commit cannot journal its change either.
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    let refusal = limited(&sandbox).refusal(4);
    assert_eq!(refusal["error"]["stage"], "record");
    let code = refusal["error"]["code"].as_str().unwrap();
    assert!(
        ["record_commit_failed", "record_store_full"].contains(&code),
        "{refusal}"
    );
    assert!(!sandbox.harness_ran());
    assert_eq!(runs(&sandbox.default_store()), 1);

    // Positive control: the same invocation without the limit.
    let result = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert_eq!(runs(&sandbox.default_store()), 2);
}

#[test]
fn a_store_that_is_corrupt_foreign_or_newer_refuses_and_is_left_as_it_was() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let store = sandbox.default_store();
    let unknown = "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e";
    let refuses = |expected: &str| {
        let before = fs::read(&store).unwrap();
        let refusal = sandbox
            .run(&["--kind", "impl", "--prompt", "p", "--json"])
            .refusal(4);
        assert_eq!(
            refusal["error"]["code"], "record_store_invalid",
            "{refusal}"
        );
        let message = refusal["error"]["message"].as_str().unwrap().to_owned();
        assert!(message.contains(expected), "{message}");
        let shown = show(&sandbox, unknown, &["--json"]).refusal(4);
        assert_eq!(shown["error"]["code"], "record_store_invalid", "{shown}");
        assert!(!sandbox.harness_ran());
        assert_eq!(fs::read(&store).unwrap(), before, "the store was changed");
    };

    support::write(&store, &"this is not a database\n".repeat(200));
    refuses("not a database");

    fs::remove_file(&store).unwrap();
    Connection::open(&store)
        .unwrap()
        .execute_batch("CREATE TABLE notes (body TEXT); INSERT INTO notes VALUES ('mine');")
        .unwrap();
    refuses("not a harness-dispatch record store");

    fs::remove_file(&store).unwrap();
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    Connection::open(&store)
        .unwrap()
        .execute_batch("PRAGMA user_version = 3;")
        .unwrap();
    refuses("record schema version 3");

    // Positive control: an empty file is a store with nothing in it yet.
    fs::remove_file(&store).unwrap();
    fs::write(&store, b"").unwrap();
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    assert_eq!(runs(&store), 1);
}

#[test]
fn an_exec_failure_is_appended_to_its_attempt() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(BROKEN);
    executable(
        &sandbox.cwd.join("broken-harness"),
        "#!/nonexistent/interpreter\n",
    );
    let result = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(result.code, Some(127), "{}", result.stderr);
    let mut lines = result.stderr.lines();
    let notice: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    let error: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    let run_id = notice["handoff"]["runId"].as_str().unwrap();
    assert_eq!(error["error"]["code"], "exec_failed");
    assert_eq!(
        error["error"]["run"],
        serde_json::json!({ "id": run_id, "launchFailure": "recorded" })
    );

    let export = show(&sandbox, run_id, &["--json"]).report();
    assert_eq!(export["evidence"], "launch_failure");
    assert_eq!(export["execution"], "not_executed");
    let failure = &export["launchFailure"];
    assert_eq!(failure["cause"], "exec_error");
    assert_eq!(failure["code"], "exec_failed");
    assert_eq!(failure["errno"], libc::ENOENT);
    assert_eq!(failure["exit"], 127);
    assert!(failure["recordedAt"].as_str().unwrap().ends_with('Z'));
    // The attempt's own launch fields are as committed.
    assert_eq!(export["launch"]["candidate"]["id"], "broken");
    assert_eq!(
        export["launch"]["argv"],
        serde_json::json!(["./broken-harness", "p"])
    );
    let text_export = show(&sandbox, run_id, &[]);
    assert!(
        text_export.stdout.contains("launch failure"),
        "{}",
        text_export.stdout
    );
}

#[test]
fn a_failed_append_leaves_the_attempt_unknown() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    // Fault injection: the store refuses exactly the append.
    Connection::open(sandbox.default_store())
        .unwrap()
        .execute_batch(
            "CREATE TRIGGER refuse_the_append BEFORE INSERT ON launch_failures \
             BEGIN SELECT RAISE(ABORT, 'simulated append failure'); END;",
        )
        .unwrap();
    sandbox.personal_policy(BROKEN);
    executable(
        &sandbox.cwd.join("broken-harness"),
        "#!/nonexistent/interpreter\n",
    );

    let result = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(result.code, Some(127), "the exec failure's exit stands");
    let mut lines = result.stderr.lines();
    let notice: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    let error: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    let run_id = notice["handoff"]["runId"].as_str().unwrap();
    let note = &error["error"]["run"];
    assert_eq!(note["id"], run_id);
    assert_eq!(note["launchFailure"], "unrecorded");
    assert_eq!(note["evidence"], "handoff_attempt");
    assert_eq!(note["execution"], "unknown");
    assert!(note["recordError"]["message"]
        .as_str()
        .unwrap()
        .contains("simulated append failure"));

    let export = show(&sandbox, run_id, &["--json"]).report();
    assert_eq!(export["evidence"], "handoff_attempt");
    assert_eq!(export["execution"], "unknown");
    assert_eq!(export["launchFailure"], Value::Null);

    let text_run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert!(
        text_run
            .stderr
            .contains("stays a handoff attempt whose execution is unknown"),
        "{}",
        text_run.stderr
    );
}

#[test]
fn committed_launch_fields_never_change() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(BROKEN);
    executable(
        &sandbox.cwd.join("broken-harness"),
        "#!/nonexistent/interpreter\n",
    );
    let result = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(result.code, Some(127), "{}", result.stderr);
    let notice: Value = serde_json::from_str(result.stderr.lines().next().unwrap()).unwrap();
    let run_id = notice["handoff"]["runId"].as_str().unwrap();
    let before = show(&sandbox, run_id, &["--json"]).report();

    let store = Connection::open(sandbox.default_store()).unwrap();
    for (statement, reason) in [
        ("UPDATE runs SET launch = '{}'", "never change"),
        ("DELETE FROM runs", "never removed"),
        ("UPDATE launch_failures SET detail = '{}'", "never changes"),
        ("DELETE FROM launch_failures", "never removed"),
    ] {
        let error = store.execute_batch(statement).unwrap_err();
        assert!(error.to_string().contains(reason), "{statement}: {error}");
    }
    drop(store);
    assert_eq!(show(&sandbox, run_id, &["--json"]).report(), before);
}

#[test]
fn without_a_home_the_default_state_directory_refuses_before_any_policy_runs() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.file(
        "policy.ts",
        &format!(
            "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"ran\");\n{ROUTED}",
            text(&sentinel)
        ),
    );
    for home in [None, Some("relative-home")] {
        for command in ["inspect", "run"] {
            let mut invocation = sandbox.command();
            match home {
                None => invocation.env_remove("HOME"),
                Some(home) => invocation.env("HOME", home),
            };
            invocation.args([
                command,
                "--kind",
                "impl",
                "--config",
                "policy.ts",
                "--prompt",
                "p",
                "--json",
            ]);
            let refusal = run(&mut invocation).refusal(4);
            assert_eq!(refusal["error"]["code"], "home_unset", "{command} {home:?}");
            assert_eq!(refusal["error"]["stage"], "record");
            assert!(!sentinel.exists(), "the policy ran before the refusal");
        }
    }
    // Positive control: an explicit state directory needs no HOME.
    let mut invocation = sandbox.command();
    invocation.env_remove("HOME").args([
        "run",
        "--kind",
        "impl",
        "--config",
        "policy.ts",
        "--state-dir",
        "records",
        "--prompt",
        "p",
    ]);
    let result = run(&mut invocation);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert!(sentinel.exists() && sandbox.harness_ran());
}
