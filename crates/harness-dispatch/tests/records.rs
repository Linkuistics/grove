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
//!
//! The launch document is version 1 under both policy contracts. The last
//! tests read a store that harness-dispatch 21.13.0 wrote under the catalog
//! contract (`tests/fixtures/catalog-contract`), beside runs this release
//! records.

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
use support::{executable, run, selecting, text, Sandbox, ROUTED};

/// Runs the fake harness in the repository the caller's `repo` parameter
/// names, with the prompt last, and says so in its reason.
const PARAMETERISED: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "records-1",
  select: (request) => ({
    status: "selected",
    program: "fake-harness",
    args: ["-C", request.params.repo, request.prompt],
    provider: "origin-a",
    model: "model-large",
    effort: "high",
    reason: `kind ${request.kind} in ${request.params.repo}`,
  }),
};
"#;

/// Runs `./broken-harness`, whose `#!` interpreter does not exist, so exec
/// fails with `ENOENT` after the commit.
const BROKEN: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "records-broken",
  select: (request) => ({
    status: "selected",
    program: "./broken-harness",
    args: [request.prompt],
    provider: "origin-b",
    model: "model-b",
    effort: "low",
    reason: "the broken harness",
  }),
};
"#;

/// The store harness-dispatch 21.13.0 wrote under the catalog contract, and
/// its two runs (`tests/fixtures/catalog-contract/README.md`).
const CATALOG_STORE: &[u8] = include_bytes!("fixtures/catalog-contract/records.sqlite3");
/// Kind `build`, task `parser-k12`: the routes table named candidate `builder`.
const ROUTED_RUN: &str = "45308255-7446-42b2-bbd7-e5f7861e46ef";
/// Kind `audit`, task `parser-k13`: `--choice auditor` named the candidate.
const CHOSEN_RUN: &str = "b2aec552-b5eb-4de0-b5e9-5db0d70a6d55";

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

/// A run's launch document as the store holds it, read with SQLite.
fn stored_launch(store: &Path, run_id: &str) -> Value {
    let launch: String = Connection::open(store)
        .unwrap()
        .query_row(
            "SELECT launch FROM runs WHERE run_id = ?1",
            [run_id],
            |row| row.get(0),
        )
        .unwrap();
    serde_json::from_str(&launch).unwrap()
}

/// A copy of the catalog-contract store as the sandbox's default store.
fn catalog_store(sandbox: &Sandbox) {
    let store = sandbox.default_store();
    fs::create_dir_all(store.parent().unwrap()).unwrap();
    fs::write(store, CATALOG_STORE).unwrap();
}

/// The rows of `record show`'s text export that begin with `label`.
fn rows<'a>(export: &'a str, label: &str) -> Vec<&'a str> {
    let label = format!("  {label:<10} ");
    export
        .lines()
        .filter_map(|line| line.strip_prefix(label.as_str()))
        .collect()
}

/// Whether any string in `value` is a run ID.
fn names_a_run(value: &Value) -> bool {
    match value {
        Value::String(text) => is_run_id(text),
        Value::Array(items) => items.iter().any(names_a_run),
        Value::Object(fields) => fields.values().any(names_a_run),
        _ => false,
    }
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
    let mut command = sandbox.command();
    command.args(["run", "--kind", "impl", "--prompt", "p", "--json"]);
    // So that the descriptors the harness holds, read below, are the front's
    // doing and not another test's.
    support::caller_leaves_open(&mut command, None);
    let result = run(&mut command);
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

    let notice = result.handoff();
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
    sandbox.personal_policy(ROUTED);
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
    // The environment is the only place the harness learns its run from: the
    // policy is given no run identity to put in an argument.
    assert_eq!(sandbox.harness_args(), ["p"]);
    assert_eq!(runs(&sandbox.default_store()), 1);
    assert_eq!(
        show(&sandbox, &run_id, &["--json"]).report()["runId"],
        run_id.as_str()
    );
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
fn inspect_reports_no_run_id_and_writes_nothing() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt", "p", "--json"])
        .report();
    assert_eq!(report["evidence"], "proposal");
    assert!(!names_a_run(&report), "{report}");
    assert_eq!(
        report["stateDir"],
        serde_json::json!({
            "path": text(&sandbox.home.join(".local/state/harness-dispatch")),
            "from": "default",
        })
    );
    let text_report = sandbox.inspect(&["--kind", "impl", "--prompt", "p"]);
    assert!(
        text_report
            .stdout
            .starts_with("Proposal only: nothing was launched and no run was recorded.\n"),
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
    assert!(!names_a_run(&report), "{report}");
    assert_eq!(fs::read(sandbox.default_store()).unwrap(), before);
    assert_eq!(runs(&sandbox.default_store()), 1);
}

#[test]
fn record_show_exports_the_launch_fields_with_dispatchs_end_and_every_other_measurement_unobserved()
{
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(PARAMETERISED);
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
            "--param",
            "repo=/work/parser",
            "--param",
            "session_name=parser: a b",
            "--timeout-ms",
            "20000",
            "--prompt",
            "Implement the parser",
        ])
        .env("DISTINCTIVE_SECRET", "never-recorded-5d0c9a");
    let result = run(&mut command);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let run_id = sandbox.harness_run_id();
    assert_eq!(
        sandbox.harness_args(),
        ["-C", "/work/parser", "Implement the parser"]
    );

    let export = show(&sandbox, &run_id, &["--json"]).report();
    assert_eq!(export["schemaVersion"], 1);
    assert_eq!(export["runId"], run_id.as_str());
    assert!(export["recordedAt"].as_str().unwrap().ends_with('Z'));
    // The harness was supervised to its end, and dispatch's own observation of
    // that end is the only evidence: it confirms execution and measures how
    // the run ended, and nothing else was observed.
    assert_eq!(export["evidence"], "execution_confirmed");
    assert_eq!(export["execution"], "confirmed");
    assert_eq!(export["launchFailure"], Value::Null);
    let observations = export["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 1, "{export}");
    assert_eq!(observations[0]["source"], "harness-dispatch");
    assert_eq!(
        observations[0]["observationId"],
        format!("harness-dispatch-end-{run_id}")
    );
    let measurements = export["measurements"].as_object().unwrap();
    assert!(measurements.len() >= 10, "{export}");
    let ends = ["executionConfirmation", "ending", "exit", "duration"];
    for (name, measurement) in measurements {
        if ends.contains(&name.as_str()) {
            assert_eq!(measurement["state"], "observed", "{name}");
            assert_eq!(
                measurement["current"][0]["observationId"], observations[0]["observationId"],
                "{name}"
            );
        } else {
            assert_eq!(
                measurement,
                &serde_json::json!({ "state": "unobserved", "current": [] }),
                "{name}"
            );
        }
    }

    // The whole launch document: every field is present, `null` where the run
    // has no value for it. Only the selection's duration and the worker's
    // build are this invocation's own.
    let launch = &export["launch"];
    assert_eq!(launch, &stored_launch(&sandbox.default_store(), &run_id));
    let digest: String = Sha256::digest(fs::read(&entry).unwrap())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    assert!(launch["timing"]["selectionMs"].is_u64());
    let worker = &launch["worker"];
    assert_eq!(worker["packageVersion"], env!("CARGO_PKG_VERSION"));
    assert_eq!(worker["bunVersion"], "1.4.2");
    assert!(worker["path"]
        .as_str()
        .is_some_and(|path| path.ends_with("/harness-dispatch-policy")));
    assert!(worker["buildId"].as_str().is_some_and(|id| id.len() == 64));
    assert_eq!(
        launch,
        &serde_json::json!({
            "schemaVersion": 1,
            "kind": "impl",
            "taskId": "T-7",
            "taskFile": text(&sandbox.cwd.join("tasks/t7.md")),
            "params": { "repo": "/work/parser", "session_name": "parser: a b" },
            // A run given no context records none, no reviewed artifact and
            // no creator, and a policy that imports no adapter records none.
            "reviewedArtifact": null,
            "context": null,
            "creator": null,
            "adapter": null,
            "cwd": text(&sandbox.cwd),
            "policy": {
                "path": text(&entry), "authority": "personal", "sha256": digest,
                "version": "records-1",
            },
            // What only a run recorded under the catalog contract has a value
            // for: a selection form, an explicit choice and a candidate ID.
            "selection": {
                "form": null, "selectedBy": null, "explicitChoice": null,
                "reason": "kind impl in /work/parser",
            },
            // The command as `select` returned it, under its labels.
            "candidate": {
                "id": null, "provider": "origin-a", "model": "model-large", "effort": "high",
                "program": "fake-harness",
                "args": ["-C", "/work/parser", "Implement the parser"],
            },
            "executable": {
                "program": "fake-harness", "resolvedBy": "PATH",
                "path": text(&sandbox.bin.join("fake-harness")),
            },
            "argv": ["fake-harness", "-C", "/work/parser", "Implement the parser"],
            "bounds": {
                "selection": { "ms": 20000, "from": "--timeout-ms" },
                "context": { "bytes": 262_144, "from": "default" },
                "source": { "bytes": 65_536, "from": "default" },
                "sources": { "sources": 256, "from": "fixed" },
                "message": { "bytes": 1_048_576, "from": "fixed" },
                "diagnostics": { "bytes": 262_144, "from": "fixed" },
            },
            "timing": launch["timing"],
            "worker": worker,
        })
    );

    // No raw environment value reaches the store.
    let store = fs::read(sandbox.default_store()).unwrap();
    let contains = |needle: &[u8]| store.windows(needle.len()).any(|window| window == needle);
    assert!(contains(b"Implement the parser"), "the store is readable");
    assert!(!contains(b"never-recorded-5d0c9a"));

    let text_export = show(&sandbox, &run_id, &[]);
    assert_eq!(text_export.code, Some(0), "{}", text_export.stderr);
    for expected in [
        run_id.as_str(),
        "execution confirmed",
        "ending harness_exit (observation harness-dispatch-end-",
        "\"Implement the parser\"",
    ] {
        assert!(
            text_export.stdout.contains(expected),
            "{expected}:\n{}",
            text_export.stdout
        );
    }
    let row = |label: &str| rows(&text_export.stdout, label);
    // A value holding a space is quoted, so two parameters cannot run together.
    assert_eq!(
        row("params"),
        [r#"repo=/work/parser "session_name=parser: a b""#]
    );
    assert_eq!(row("provider"), ["origin-a"]);
    assert_eq!(row("model"), ["model-large"]);
    assert_eq!(row("effort"), ["high"]);
    assert_eq!(row("reason"), ["kind impl in /work/parser"]);
    // The rows for what a catalog-contract run recorded are left out.
    for absent in ["choice", "selected", "candidate"] {
        assert_eq!(row(absent), Vec::<&str>::new(), "{absent}");
    }

    // A run given no parameter records an empty set, not an absent one.
    fs::remove_dir_all(&sandbox.record).unwrap();
    sandbox.personal_policy(ROUTED);
    let bare = launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);
    let export = show(&sandbox, &bare, &["--json"]).report();
    assert_eq!(export["launch"]["params"], serde_json::json!({}));
    assert_eq!(export["launch"]["taskId"], Value::Null);
    assert_eq!(export["launch"]["taskFile"], Value::Null);
    let text_export = show(&sandbox, &bare, &[]);
    assert_eq!(rows(&text_export.stdout, "params"), ["none"]);
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
    sandbox.personal_policy(&selecting(
        "",
        r#"    const program = { impl: "fake-harness", missing: "no-such-harness" }[request.kind];
    if (program === undefined) {
      return { status: "refused", code: "unrouted", message: "no command for this kind", remedy: "add one" };
    }
    return { status: "selected", program, args: [request.prompt], provider: "o", model: "m", effort: "e", reason: "r" };"#,
    ));
    let unrouted = sandbox
        .run(&["--kind", "design", "--prompt", "p", "--json"])
        .refusal(3);
    assert_eq!(unrouted["error"]["code"], "policy_refused");
    assert_eq!(unrouted["error"]["policyCode"], "unrouted");
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
fn a_lock_released_within_the_wait_is_waited_out_and_the_run_launches() {
    // The other side of the lock wait: the same invocation, with the store
    // locked when selection ends and released half a second later. The front
    // is still running while the lock is held, so it did not give up at once,
    // and it then commits its run and launches. The half second is from the
    // policy's own mark, as above. It leaves a second and a half of the
    // 2-second wait for a loaded machine. On one so loaded that the front
    // reaches the store only after the release, the front never waited, and
    // this passes as an unlocked run does.
    const HELD: Duration = Duration::from_millis(500);
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    launched(&sandbox, &["--kind", "impl", "--prompt", "p"]);

    let marker = sandbox.root.join("selected");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"\");\n{ROUTED}",
        text(&marker)
    ));
    let holder = Connection::open(sandbox.default_store()).unwrap();
    holder.execute_batch("BEGIN EXCLUSIVE").unwrap();
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
    let mut child = command.spawn().unwrap();
    while !marker.exists() {
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "the policy never ran"
        );
        thread::sleep(Duration::from_millis(5));
    }
    thread::sleep(HELD);
    let ended_early = child.try_wait().unwrap();
    assert_eq!(runs_while_locked(&holder), 1);
    holder.execute_batch("COMMIT").unwrap();
    let result = support::Run::from(child.wait_with_output().unwrap());
    assert!(
        ended_early.is_none(),
        "the front ended while the store was locked ({ended_early:?}), without waiting\nstderr: {}",
        result.stderr
    );
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    assert!(sandbox.harness_ran());
    assert_eq!(runs(&sandbox.default_store()), 2);
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
    assert_eq!(export["launch"]["candidate"]["program"], "./broken-harness");
    assert_eq!(export["launch"]["candidate"]["provider"], "origin-b");
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

#[test]
fn a_run_recorded_under_the_catalog_contract_is_exported_as_it_was_stored() {
    let sandbox = Sandbox::new();
    catalog_store(&sandbox);
    let store = sandbox.default_store();
    let before = fs::read(&store).unwrap();

    let routed = show(&sandbox, ROUTED_RUN, &["--json"]).report();
    assert_eq!(routed["schemaVersion"], 1);
    assert_eq!(routed["runId"], ROUTED_RUN);
    assert_eq!(routed["recordedAt"], "2026-10-02T04:11:46.561Z");
    assert_eq!(routed["evidence"], "handoff_attempt");
    assert_eq!(routed["execution"], "unknown");
    assert_eq!(routed["launchFailure"], Value::Null);
    assert_eq!(routed["observations"], serde_json::json!([]));
    for (name, measurement) in routed["measurements"].as_object().unwrap() {
        assert_eq!(
            measurement,
            &serde_json::json!({ "state": "unobserved", "current": [] }),
            "{name}"
        );
    }
    // The launch document is the stored one, with what only that contract
    // recorded: the candidate's ID, the slots among its arguments, the
    // selection form and the explicit choice.
    let launch = &routed["launch"];
    assert_eq!(launch, &stored_launch(&store, ROUTED_RUN));
    assert_eq!(launch["schemaVersion"], 1);
    assert_eq!(launch["kind"], "build");
    assert_eq!(launch["taskId"], "parser-k12");
    assert_eq!(
        launch["candidate"],
        serde_json::json!({
            "id": "builder", "provider": "your-provider", "model": "model-a", "effort": "high",
            "program": "fake-harness",
            "args": ["--model", { "slot": "model" }, "--effort", { "slot": "effort" }, { "slot": "prompt" }],
        })
    );
    assert_eq!(
        launch["selection"],
        serde_json::json!({
            "form": "routes", "selectedBy": "route", "explicitChoice": null,
            "reason": "routes[\"build\"] names candidate \"builder\"",
        })
    );
    assert_eq!(
        launch["argv"],
        serde_json::json!([
            "fake-harness",
            "--model",
            "model-a",
            "--effort",
            "high",
            "Build the parser"
        ])
    );
    // That release recorded no parameters, and none is supplied for it.
    assert!(launch.get("params").is_none(), "{launch}");

    let chosen = show(&sandbox, CHOSEN_RUN, &["--json"]).report();
    assert_eq!(chosen["evidence"], "handoff_attempt");
    let launch = &chosen["launch"];
    assert_eq!(launch, &stored_launch(&store, CHOSEN_RUN));
    assert_eq!(launch["kind"], "audit");
    assert_eq!(launch["taskId"], "parser-k13");
    assert_eq!(
        launch["candidate"],
        serde_json::json!({
            "id": "auditor", "provider": "your-other-provider", "model": "model-b",
            "effort": "medium", "program": "fake-harness",
            "args": [{ "slot": "taskId" }, { "slot": "prompt" }],
        })
    );
    assert_eq!(launch["selection"]["form"], "routes");
    assert_eq!(launch["selection"]["selectedBy"], "explicit_choice");
    assert_eq!(launch["selection"]["explicitChoice"], "auditor");
    assert_eq!(
        launch["argv"],
        serde_json::json!(["fake-harness", "parser-k13", "Audit the parser"])
    );

    // The text export has a row for each value such a run recorded.
    let text_export = show(&sandbox, ROUTED_RUN, &[]);
    assert_eq!(text_export.code, Some(0), "{}", text_export.stderr);
    let row = |label: &str| rows(&text_export.stdout, label);
    assert!(
        text_export.stdout.contains("handoff attempt"),
        "{}",
        text_export.stdout
    );
    assert_eq!(row("kind"), ["build"]);
    assert_eq!(row("task id"), ["parser-k12"]);
    assert_eq!(row("params"), ["none"]);
    assert_eq!(row("selected"), ["route"]);
    assert_eq!(row("candidate"), ["builder"]);
    assert_eq!(row("choice"), Vec::<&str>::new(), "no choice was given");
    assert_eq!(row("provider"), ["your-provider"]);
    assert_eq!(row("model"), ["model-a"]);
    assert_eq!(row("effort"), ["high"]);
    assert_eq!(
        row("reason"),
        ["routes[\"build\"] names candidate \"builder\""]
    );
    assert!(
        text_export.stdout.contains("[5] \"Build the parser\""),
        "{}",
        text_export.stdout
    );
    let text_export = show(&sandbox, CHOSEN_RUN, &[]);
    assert_eq!(text_export.code, Some(0), "{}", text_export.stderr);
    let row = |label: &str| rows(&text_export.stdout, label);
    assert_eq!(row("choice"), ["auditor"]);
    assert_eq!(row("selected"), ["explicit_choice"]);
    assert_eq!(row("candidate"), ["auditor"]);
    assert_eq!(row("provider"), ["your-other-provider"]);

    // Reading changed nothing.
    assert_eq!(fs::read(&store).unwrap(), before, "the store was changed");
}

#[test]
fn a_run_commits_beside_the_runs_of_the_catalog_contract_and_both_export() {
    let sandbox = Sandbox::new();
    catalog_store(&sandbox);
    let store = sandbox.default_store();
    assert_eq!(runs(&store), 2);
    let routed = show(&sandbox, ROUTED_RUN, &["--json"]).report();
    let chosen = show(&sandbox, CHOSEN_RUN, &["--json"]).report();

    sandbox.personal_policy(ROUTED);
    let run_id = launched(
        &sandbox,
        &["--kind", "impl", "--task-id", "parser-k14", "--prompt", "p"],
    );
    assert_eq!(runs(&store), 3);

    // The run this release recorded has no value for what the others carry.
    let export = show(&sandbox, &run_id, &["--json"]).report();
    assert_eq!(export["evidence"], "execution_confirmed");
    let launch = &export["launch"];
    assert_eq!(launch["schemaVersion"], routed["launch"]["schemaVersion"]);
    assert_eq!(launch["taskId"], "parser-k14");
    assert_eq!(launch["candidate"]["id"], Value::Null);
    assert_eq!(launch["candidate"]["provider"], "origin-a");
    assert_eq!(launch["candidate"]["args"], serde_json::json!(["p"]));
    assert_eq!(
        launch["selection"],
        serde_json::json!({
            "form": null, "selectedBy": null, "explicitChoice": null,
            "reason": "impl runs the deep harness",
        })
    );
    assert_eq!(launch["params"], serde_json::json!({}));

    // The commit left the earlier runs exactly as they were.
    assert_eq!(show(&sandbox, ROUTED_RUN, &["--json"]).report(), routed);
    assert_eq!(show(&sandbox, CHOSEN_RUN, &["--json"]).report(), chosen);
}
