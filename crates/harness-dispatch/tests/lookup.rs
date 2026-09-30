//! Run lookup through the command seam: a policy's `host.run` answered from the
//! record store, the answers the delivered context carries as `runs`, and the
//! creator provenance a run records
//! (`docs/specs/harness-selection-and-execution.md`, *Bounded context*,
//! *Identity and original creator*, *Records and later observations*).
//!
//! Every producer here is a real `run` of the front against the fake harness,
//! so each lookup reads a record that `run` committed. Each refusal has a
//! positive control, the same lookup against a readable store, that is seen to
//! select.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};

use rusqlite::Connection;
use serde_json::{json, Value};
use sha2::{Digest as _, Sha256};
use support::{executable, run, text, Sandbox, ROUTED};

/// A run ID no store in these tests holds.
const UNKNOWN: &str = "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e";

/// The producer's candidate as `ROUTED` configures it.
const DEEP: &str = r#"{ id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: [{ slot: "prompt" }] }"#;

/// A policy whose `loadContext` looks up the run its caller's reviewed
/// artifact names, with `call` (`host.run(run)`, or a variant of it), and whose
/// `select` picks `deep`, the catalog's first candidate, giving the delivered
/// `runs` as its reason, so a test sees exactly what selection saw.
fn lookup_policy(deep: &str, call: &str) -> String {
    format!(
        r#"export const policy = {{
  schemaVersion: 1,
  version: "lookup-1",
  catalog: [
    {deep},
    {{ id: "quick", provider: "origin-b", model: "model-small", effort: "low", program: "fake-harness", args: [{{ slot: "prompt" }}] }},
  ],
  loadContext(request, host) {{
    const context = request.context;
    const run = context?.reviewedArtifact?.creator?.run;
    if (run !== undefined) {{ {call}; }}
    return context;
  }},
  select(request, context) {{
    return {{ status: "selected", candidateId: "deep", reason: JSON.stringify(context.runs ?? null) }};
  }},
}};
"#
    )
}

fn looks_up() -> String {
    lookup_policy(DEEP, "host.run(run)")
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

/// Run the producer: `ROUTED`'s `deep` candidate for kind `impl`, as task
/// `producer-k1`.
fn producer(sandbox: &Sandbox) -> String {
    sandbox.personal_policy(ROUTED);
    launched(
        sandbox,
        &[
            "--kind",
            "impl",
            "--task-id",
            "producer-k1",
            "--prompt",
            "p",
        ],
    )
}

fn show(sandbox: &Sandbox, run_id: &str) -> support::Run {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id, "--json"]);
    run(&mut command)
}

fn show_text(sandbox: &Sandbox, run_id: &str) -> String {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id]);
    let result = run(&mut command);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    result.stdout
}

/// Write `review.json`, a caller context whose reviewed artifact names
/// `creator`.
fn review_context(sandbox: &Sandbox, creator: &Value) {
    let document = json!({
        "schemaVersion": 1,
        "reviewedArtifact": { "id": "producer-k1", "creator": creator },
    });
    sandbox.file("review.json", &document.to_string());
}

const REVIEW: [&str; 4] = ["--kind", "review", "--context", "review.json"];

fn inspect_review(sandbox: &Sandbox) -> support::Run {
    let mut args = REVIEW.to_vec();
    args.push("--json");
    sandbox.inspect(&args)
}

fn run_review(sandbox: &Sandbox) -> support::Run {
    let mut args = REVIEW.to_vec();
    args.extend(["--prompt", "p", "--json"]);
    sandbox.run(&args)
}

/// The answer `host.run` gives for `run_id`, built from `record show`'s export
/// of the same run: its immutable launch fields and any launch failure.
fn found(sandbox: &Sandbox, run_id: &str) -> Value {
    let export = show(sandbox, run_id).report();
    let launch = &export["launch"];
    let candidate = &launch["candidate"];
    json!({
        "runId": run_id,
        "status": "found",
        "recordedAt": export["recordedAt"],
        "kind": launch["kind"],
        "taskId": launch["taskId"],
        "candidate": {
            "id": candidate["id"],
            "provider": candidate["provider"],
            "model": candidate["model"],
            "effort": candidate["effort"],
        },
        "launchFailure": export["launchFailure"],
    })
}

/// The measured source a lookup's answer is: named by its run ID, sized and
/// digested over the answer's compact encoding with sorted keys.
fn run_source(answer: &Value) -> Value {
    let bytes = serde_json::to_vec(answer).unwrap();
    json!({
        "name": answer["runId"],
        "via": "run",
        "bytes": bytes.len(),
        "sha256": hex(&Sha256::digest(&bytes)),
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// What `select` saw as `runs`: the reason `lookup_policy` gives.
fn seen(report: &Value) -> Value {
    serde_json::from_str(report["selection"]["reason"].as_str().unwrap()).unwrap()
}

fn runs_in(store: &Path) -> i64 {
    Connection::open(store)
        .unwrap()
        .query_row("SELECT count(*) FROM runs", [], |row| row.get(0))
        .unwrap()
}

#[test]
fn host_run_returns_a_recorded_runs_immutable_launch_fields() {
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    let answer = found(&sandbox, &creator);
    assert_eq!(
        answer["candidate"],
        json!({ "id": "deep", "provider": "origin-a", "model": "model-large", "effort": "high" })
    );
    assert_eq!(answer["kind"], "impl");
    assert_eq!(answer["taskId"], "producer-k1");
    assert!(answer["recordedAt"].as_str().unwrap().ends_with('Z'));
    assert_eq!(answer["launchFailure"], Value::Null);

    review_context(&sandbox, &json!({ "run": creator }));
    sandbox.personal_policy(&looks_up());
    let store = sandbox.default_store();
    let before = fs::read(&store).unwrap();

    // Inspection: selection saw the answer, the delivered context carries it
    // after the caller's document as a measured source, and the creator
    // provenance names it.
    let report = inspect_review(&sandbox).report();
    assert_eq!(seen(&report), json!([answer]));
    let context = &report["context"];
    assert_eq!(context["value"]["runs"], json!([answer]));
    assert_eq!(context["sources"][1], run_source(&answer));
    assert_eq!(context["value"]["measured"], context["sources"]);
    let expected_creator = json!({
        "reference": { "run": creator },
        "evidence": "execution_recorded",
        "provider": "origin-a",
        "lookup": answer,
    });
    assert_eq!(report["creator"], expected_creator);
    // Inspection read the store and recorded nothing.
    assert_eq!(
        fs::read(&store).unwrap(),
        before,
        "inspection changed the store"
    );
    assert_eq!(runs_in(&store), 1);

    // The review's run records the same provenance, and the answer's digest
    // among its context sources.
    let result = run_review(&sandbox);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let review = sandbox.harness_run_id();
    let export = show(&sandbox, &review).report();
    assert_eq!(export["launch"]["creator"], expected_creator);
    assert_eq!(
        export["launch"]["context"]["sources"][1],
        run_source(&answer)
    );
    assert_eq!(
        export["launch"]["reviewedArtifact"]["creator"]["run"],
        creator
    );

    // Both text forms show the creator with its run's task identity.
    let mut args = REVIEW.to_vec();
    args.push("--prompt-file");
    sandbox.file("prompt.md", "p");
    args.push("prompt.md");
    let inspected = sandbox.inspect(&args);
    assert_eq!(inspected.code, Some(0), "{}", inspected.stderr);
    for text in [inspected.stdout, show_text(&sandbox, &review)] {
        let row = text
            .lines()
            .find(|line| line.trim_start().starts_with("creator "))
            .unwrap_or_else(|| panic!("no creator row:\n{text}"));
        for part in [
            creator.as_str(),
            "execution-recorded",
            "provider origin-a",
            "task producer-k1",
            "kind impl",
        ] {
            assert!(row.contains(part), "{part:?} is not in {row:?}");
        }
    }
}

#[test]
fn a_changed_current_catalog_does_not_alter_the_returned_snapshot() {
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    review_context(&sandbox, &json!({ "run": creator }));

    sandbox.personal_policy(&looks_up());
    let before = inspect_review(&sandbox).report();
    // The same ID, now another provider, model and effort: today's catalog
    // is not the record.
    sandbox.personal_policy(&lookup_policy(
        r#"{ id: "deep", provider: "origin-z", model: "model-new", effort: "max", program: "fake-harness", args: [{ slot: "prompt" }] }"#,
        "host.run(run)",
    ));
    let after = inspect_review(&sandbox).report();
    assert_eq!(seen(&after), seen(&before));
    assert_eq!(
        seen(&after)[0]["candidate"],
        json!({ "id": "deep", "provider": "origin-a", "model": "model-large", "effort": "high" })
    );
    assert_eq!(after["creator"]["provider"], "origin-a");
    // The control: the same report selected `deep` from today's catalog, so
    // the change did reach the policy.
    assert_eq!(before["selection"]["provider"], "origin-a");
    assert_eq!(after["selection"]["provider"], "origin-z");
    assert_eq!(after["selection"]["model"], "model-new");
}

#[test]
fn a_missing_run_is_reported_missing_whether_or_not_there_is_a_store() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&looks_up());
    review_context(&sandbox, &json!({ "run": UNKNOWN }));
    let missing = json!({ "runId": UNKNOWN, "status": "missing" });
    let store = sandbox.default_store();
    let expect_missing = |what: &str| {
        let report = inspect_review(&sandbox).report();
        assert_eq!(seen(&report), json!([missing]), "{what}");
        assert_eq!(
            report["context"]["sources"][1],
            run_source(&missing),
            "{what}"
        );
        assert_eq!(
            report["creator"],
            json!({
                "reference": { "run": UNKNOWN },
                "evidence": "execution_recorded",
                "provider": null,
                "lookup": missing,
            }),
            "{what}"
        );
    };

    // No store at all, and inspection does not make one.
    expect_missing("no store");
    assert!(!store.exists(), "inspection created a store");
    assert!(!store.parent().unwrap().exists());

    // An empty file: a store with nothing in it, left empty.
    support::write(&store, "");
    expect_missing("an empty store");
    assert_eq!(fs::read(&store).unwrap(), b"");

    // A store holding other runs.
    fs::remove_file(&store).unwrap();
    let other = producer(&sandbox);
    sandbox.personal_policy(&looks_up());
    expect_missing("a store without the run");

    // The control: the store's own run is found.
    review_context(&sandbox, &json!({ "run": other }));
    let report = inspect_review(&sandbox).report();
    assert_eq!(seen(&report), json!([found(&sandbox, &other)]));
}

#[test]
fn a_store_that_cannot_be_read_refuses_and_never_reads_as_missing() {
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    review_context(&sandbox, &json!({ "run": creator }));
    // The policy swallows any error from its lookup, and would select.
    sandbox.personal_policy(&lookup_policy(DEEP, "try { host.run(run); } catch {}"));
    let store = sandbox.default_store();
    let dir = store.parent().unwrap().to_owned();
    let good = fs::read(&store).unwrap();

    let refuses = |code: &str, detail: &str| {
        for (command, refusal) in [
            ("inspect", inspect_review(&sandbox).refusal(4)),
            ("run", run_review(&sandbox).refusal(4)),
        ] {
            let context = format!("{command}, {detail}: {refusal}");
            let error = &refusal["error"];
            assert_eq!(error["code"], code, "{context}");
            assert_eq!(error["stage"], "record", "{context}");
            assert_eq!(error["source"], text(&store), "{context}");
            let message = error["message"].as_str().unwrap();
            for part in [creator.as_str(), "host.run", detail] {
                assert!(message.contains(part), "{part:?}: {context}");
            }
            assert!(!sandbox.harness_ran(), "{context}");
        }
    };
    let unchanged = |before: &[u8], what: &str| {
        assert_eq!(
            fs::read(&store).unwrap(),
            before,
            "{what}: the store changed"
        );
    };
    let restore = || {
        let _ = fs::remove_file(&store);
        fs::write(&store, &good).unwrap();
        fs::set_permissions(&store, fs::Permissions::from_mode(0o600)).unwrap();
    };
    let selects = |what: &str| {
        let report = inspect_review(&sandbox).report();
        assert_eq!(seen(&report)[0]["status"], "found", "{what}");
    };
    selects("the control, before any fault");

    // A directory that cannot be searched: whether the store exists is
    // unknown, which is not the same as its absence.
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o000)).unwrap();
    refuses("record_store_unwritable", "unable to open");
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
    unchanged(&good, "an unsearchable directory");

    // A store file that cannot be opened.
    fs::set_permissions(&store, fs::Permissions::from_mode(0o000)).unwrap();
    refuses("record_store_unwritable", "unable to open");
    fs::set_permissions(&store, fs::Permissions::from_mode(0o600)).unwrap();
    unchanged(&good, "an unreadable file");

    let garbage = "this is not a database\n".repeat(200);
    support::write(&store, &garbage);
    refuses("record_store_invalid", "not a database");
    unchanged(garbage.as_bytes(), "a corrupt store");

    fs::remove_file(&store).unwrap();
    Connection::open(&store)
        .unwrap()
        .execute_batch("CREATE TABLE notes (body TEXT);")
        .unwrap();
    let foreign = fs::read(&store).unwrap();
    refuses(
        "record_store_invalid",
        "not a harness-dispatch record store",
    );
    unchanged(&foreign, "another application's store");

    restore();
    Connection::open(&store)
        .unwrap()
        .execute_batch("PRAGMA user_version = 3;")
        .unwrap();
    let newer = fs::read(&store).unwrap();
    refuses("record_store_invalid", "record schema version 3");
    unchanged(&newer, "a newer store");

    restore();
    selects("the control, after every fault");
    assert_eq!(runs_in(&store), 1, "a refused run was recorded");
}

#[test]
fn a_run_that_carries_a_launch_failure_returns_its_detail() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(
        r#"export const policy = {
  schemaVersion: 1,
  version: "broken-1",
  catalog: [
    { id: "broken", provider: "origin-b", model: "model-b", effort: "low", program: "./broken-harness", args: [{ slot: "prompt" }] },
  ],
  routes: { impl: "broken" },
};
"#,
    );
    executable(
        &sandbox.cwd.join("broken-harness"),
        "#!/nonexistent/interpreter\n",
    );
    let result = sandbox.run(&[
        "--kind",
        "impl",
        "--task-id",
        "producer-k1",
        "--prompt",
        "p",
        "--json",
    ]);
    assert_eq!(result.code, Some(127), "{}", result.stderr);
    let notice: Value = serde_json::from_str(result.stderr.lines().next().unwrap()).unwrap();
    let creator = notice["handoff"]["runId"].as_str().unwrap().to_owned();

    review_context(&sandbox, &json!({ "run": creator }));
    sandbox.personal_policy(&looks_up());
    let report = inspect_review(&sandbox).report();
    let answer = found(&sandbox, &creator);
    assert_eq!(seen(&report), json!([answer]));
    let failure = &answer["launchFailure"];
    assert_eq!(failure["cause"], "exec_error");
    assert_eq!(failure["code"], "exec_failed");
    assert_eq!(failure["errno"], libc::ENOENT);
    assert!(failure["recordedAt"].as_str().unwrap().ends_with('Z'));
    assert_eq!(answer["candidate"]["provider"], "origin-b");
    assert_eq!(report["creator"]["lookup"], answer);
}

#[test]
fn a_lookup_waits_for_a_locked_store_only_within_the_selection_bound() {
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    review_context(&sandbox, &json!({ "run": creator }));
    // The loader marks when it runs, so the wait is timed from the lookup
    // rather than from the front's start, which a loaded machine can delay.
    let marker = sandbox.root.join("looking-up");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\n{}",
        lookup_policy(
            DEEP,
            &format!("writeFileSync({:?}, \"\"); host.run(run)", text(&marker))
        )
    ));
    let holder = Connection::open(sandbox.default_store()).unwrap();
    holder.execute_batch("BEGIN EXCLUSIVE").unwrap();

    let waited = |extra: &[&str]| {
        let _ = fs::remove_file(&marker);
        let started = Instant::now();
        let mut command = sandbox.command();
        command
            .arg("inspect")
            .args(REVIEW)
            .args(extra)
            .arg("--json")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let child = command.spawn().unwrap();
        while !marker.exists() {
            assert!(
                started.elapsed() < Duration::from_secs(20),
                "the loader never ran"
            );
            thread::sleep(Duration::from_millis(5));
        }
        let looking = Instant::now();
        let result = support::Run::from(child.wait_with_output().unwrap());
        (result, looking.elapsed())
    };

    // With time to spare, the lookup waits the fixed 2 seconds, then refuses
    // on the lock.
    let (result, waited_for) = waited(&[]);
    let refusal = result.refusal(4);
    assert_eq!(refusal["error"]["code"], "record_store_locked", "{refusal}");
    assert!(
        waited_for >= Duration::from_millis(1950) && waited_for < Duration::from_millis(4500),
        "the lock wait was not the fixed 2 seconds: {waited_for:?}"
    );

    // With a one-second selection bound, the wait ends at the deadline, and
    // the selection times out rather than outliving its bound.
    let (result, waited_for) = waited(&["--timeout-ms", "1000"]);
    let refusal = result.refusal(124);
    assert_eq!(refusal["error"]["code"], "selection_timeout", "{refusal}");
    assert!(
        waited_for < Duration::from_millis(1900),
        "the lookup outlived the selection bound: {waited_for:?}"
    );

    // The control: released, the lock is no obstacle.
    holder.execute_batch("COMMIT").unwrap();
    let (result, _) = waited(&["--timeout-ms", "1000"]);
    assert_eq!(seen(&result.report())[0]["status"], "found");
}

/// Import `document` against `run_id` with `record observe`, as `--json`.
fn observe(sandbox: &Sandbox, run_id: &str, document: &Value) -> support::Run {
    sandbox.file("observation.json", &document.to_string());
    let mut command = sandbox.command();
    command.args([
        "record",
        "observe",
        "--run",
        run_id,
        "--file",
        "observation.json",
        "--json",
    ]);
    run(&mut command)
}

/// A version-1 observation of `run_id`, with one measurement.
fn observation(run_id: &str, id: &str) -> Value {
    json!({
        "schemaVersion": 1,
        "observationId": id,
        "runId": run_id,
        "source": "a test observer",
        "observedAt": "2026-10-01T09:30:00Z",
        "evidence": "what the test saw",
        "measurements": { "acceptance": { "state": "observed", "value": "accepted" } },
    })
}

#[test]
fn a_lookup_reads_the_runs_launch_fields_never_its_observation_history() {
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    let answer = found(&sandbox, &creator);
    review_context(&sandbox, &json!({ "run": creator }));
    sandbox.personal_policy(&looks_up());

    // A history, a correction and the observation it supersedes among it.
    for id in ["o-1", "o-2", "o-3"] {
        let mut document = observation(&creator, id);
        if id == "o-3" {
            document["supersedes"] = json!("o-2");
        }
        let result = observe(&sandbox, &creator, &document);
        assert_eq!(result.code, Some(0), "{}", result.stderr);
    }
    assert_eq!(seen(&inspect_review(&sandbox).report()), json!([answer]));

    // One more stored observation, which is not JSON at all.
    let store = sandbox.default_store();
    Connection::open(&store)
        .unwrap()
        .execute(
            "INSERT INTO observations (observation_id, run_id, recorded_at, document) \
             VALUES ('o-4', ?1, '2026-10-01T09:31:00.000Z', '{')",
            [&creator],
        )
        .unwrap();
    // The control: whatever reads the run's history finds it unreadable.
    let refusal = show(&sandbox, &creator).refusal(4);
    assert_eq!(
        refusal["error"]["code"], "record_store_invalid",
        "{refusal}"
    );
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("\"o-4\""),
        "{refusal}"
    );

    // The lookup never reads it: the answer is the launch fields, as before,
    // under inspection and in the run the review records.
    let before = fs::read(&store).unwrap();
    let report = inspect_review(&sandbox).report();
    assert_eq!(seen(&report), json!([answer]));
    assert_eq!(report["creator"]["lookup"], answer);
    assert_eq!(
        fs::read(&store).unwrap(),
        before,
        "inspection changed the store"
    );
    let result = run_review(&sandbox);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let review = sandbox.harness_run_id();
    assert_eq!(
        show(&sandbox, &review).report()["launch"]["creator"]["lookup"],
        answer
    );
}

#[test]
fn a_launch_record_this_release_cannot_read_refuses_every_read_of_the_run() {
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    let launch = show(&sandbox, &creator).report()["launch"].clone();
    // The policy swallows any error from its lookup, and would select.
    sandbox.personal_policy(&lookup_policy(DEEP, "try { host.run(run); } catch {}"));
    let store = sandbox.default_store();

    // Runs committed beside the producer by some other writer, each read by
    // `record show`, `record observe` and `host.run` alike.
    let mut later = launch.clone();
    later["schemaVersion"] = json!(2);
    let cases = [
        (
            "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d01",
            later.to_string(),
            None,
            "is version 2",
        ),
        (
            "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d02",
            "{".to_owned(),
            None,
            "is not valid JSON",
        ),
        (
            "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d03",
            "[1]".to_owned(),
            None,
            "is not a JSON object",
        ),
        (
            "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d04",
            launch.to_string(),
            Some(r#"{"message":"exec failed"}"#),
            "launch failure has no cause",
        ),
        // The control: the producer's own launch record, written again.
        (
            "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d05",
            launch.to_string(),
            None,
            "",
        ),
    ];
    {
        let connection = Connection::open(&store).unwrap();
        for (run_id, launch, failure, _) in &cases {
            connection
                .execute(
                    "INSERT INTO runs (run_id, recorded_at, launch) \
                     VALUES (?1, '2026-10-01T09:30:00.000Z', ?2)",
                    rusqlite::params![run_id, launch],
                )
                .unwrap();
            if let Some(detail) = failure {
                connection
                    .execute(
                        "INSERT INTO launch_failures (run_id, recorded_at, detail) \
                         VALUES (?1, '2026-10-01T09:30:01.000Z', ?2)",
                        rusqlite::params![run_id, detail],
                    )
                    .unwrap();
            }
        }
    }

    for (run_id, _, _, why) in cases {
        review_context(&sandbox, &json!({ "run": run_id }));
        let before = fs::read(&store).unwrap();
        let reads = [
            ("record show", show(&sandbox, run_id)),
            (
                "record observe",
                observe(&sandbox, run_id, &observation(run_id, "o-1")),
            ),
            ("host.run", inspect_review(&sandbox)),
        ];
        for (read, result) in reads {
            let context = format!("{read} of {run_id}");
            if why.is_empty() {
                assert_eq!(result.code, Some(0), "{context}: {}", result.stderr);
                continue;
            }
            let refusal = result.refusal(4);
            let error = &refusal["error"];
            assert_eq!(
                error["code"], "record_store_invalid",
                "{context}: {refusal}"
            );
            assert_eq!(error["stage"], "record", "{context}: {refusal}");
            assert_eq!(error["source"], text(&store), "{context}: {refusal}");
            let message = error["message"].as_str().unwrap();
            for part in [run_id, why] {
                assert!(message.contains(part), "{context}: {part:?}: {refusal}");
            }
            assert_eq!(
                fs::read(&store).unwrap(),
                before,
                "{context}: the store changed"
            );
        }
    }
    assert!(
        !sandbox.harness_ran(),
        "a refused lookup launched a harness"
    );
}

#[test]
fn each_lookup_is_a_measured_source_within_the_source_bound() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!(
        r#"export const policy = {{
  schemaVersion: 1,
  version: "lookups-1",
  catalog: [
    {DEEP},
  ],
  routes: {{ review: "deep" }},
  loadContext(request, host) {{
    for (let i = 0; i < request.context.facts.lookups; i++) {{
      try {{ host.run({UNKNOWN:?}); }} catch {{}}
    }}
    return request.context;
  }},
}};
"#
    ));
    let lookups = |count: u64| {
        sandbox.file(
            "lookups.json",
            &json!({ "schemaVersion": 1, "facts": { "lookups": count } }).to_string(),
        );
        sandbox.inspect(&["--kind", "review", "--context", "lookups.json", "--json"])
    };

    // The --context document and 255 lookups are 256 sources, the bound.
    let report = lookups(255).report();
    let sources = report["context"]["sources"].as_array().unwrap();
    assert_eq!(sources.len(), 256);
    assert!(sources[1..].iter().all(|source| source["via"] == "run"));
    assert_eq!(
        report["context"]["value"]["runs"].as_array().unwrap().len(),
        255
    );

    // One more is over it, although the loader caught the error.
    let refusal = lookups(256).refusal(3);
    assert_eq!(refusal["error"]["code"], "too_many_sources", "{refusal}");
    assert_eq!(refusal["error"]["stage"], "context", "{refusal}");
}

#[test]
fn run_lookup_is_the_loaders_alone_and_takes_only_a_run_id() {
    // `select` sees the delivered context, so its host looks nothing up, and a
    // lookup the loader leaves for later fails once its context is delivered.
    // A malformed ID is a TypeError before anything is asked of the store.
    // None of these is a bound: the policy catches each and selects.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!(
        r#"export const policy = {{
  schemaVersion: 1,
  version: "lookup-closed-1",
  catalog: [
    {DEEP},
  ],
  loadContext(request, host) {{
    const errors = [];
    for (const bad of [42, "", {upper:?}, "producer-k1", {UNKNOWN:?} + " "]) {{
      try {{ host.run(bad); }} catch (error) {{ errors.push(`${{error.name}}: ${{error.message}}`); }}
    }}
    globalThis.laterLookup = () => host.run({UNKNOWN:?});
    return {{ schemaVersion: 1, facts: {{ errors }} }};
  }},
  select(request, context, host) {{
    const errors = [...context.facts.errors];
    try {{ host.run({UNKNOWN:?}); }} catch (error) {{ errors.push(error.message); }}
    try {{ globalThis.laterLookup(); }} catch (error) {{ errors.push(error.message); }}
    return {{ status: "selected", candidateId: "deep", reason: JSON.stringify(errors) }};
  }},
}};
"#,
        upper = UNKNOWN.to_uppercase(),
    ));
    let report = sandbox.inspect(&["--kind", "review", "--json"]).report();
    let errors: Vec<String> =
        serde_json::from_str(report["selection"]["reason"].as_str().unwrap()).unwrap();
    assert_eq!(errors.len(), 7, "{errors:?}");
    for error in &errors[..5] {
        assert!(
            error.starts_with("TypeError: host.run takes a run ID"),
            "{error}"
        );
    }
    assert!(
        errors[5].contains("host.run is available in loadContext only"),
        "{}",
        errors[5]
    );
    assert!(
        errors[6].contains("after loadContext returned its context"),
        "{}",
        errors[6]
    );
    // Nothing was looked up, so nothing was measured or delivered.
    assert_eq!(report["context"]["sources"], json!([]));
    assert!(report["context"]["value"].get("runs").is_none());

    // Uncaught, a malformed ID fails the loader, naming the TypeError.
    sandbox.personal_policy(&format!(
        r#"export const policy = {{
  schemaVersion: 1,
  version: "lookup-malformed-1",
  catalog: [
    {DEEP},
  ],
  routes: {{ review: "deep" }},
  loadContext(request, host) {{ host.run("producer-k1"); return {{ schemaVersion: 1 }}; }},
}};
"#
    ));
    let refusal = sandbox.inspect(&["--kind", "review", "--json"]).refusal(3);
    assert_eq!(
        refusal["error"]["code"], "context_loader_failed",
        "{refusal}"
    );
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("TypeError: host.run takes a run ID"),
        "{refusal}"
    );
}

#[test]
fn the_run_record_names_the_creator_reference_its_evidence_and_its_lookup() {
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    // `ROUTED` has no loader: the context is the caller's, and nothing is
    // looked up.
    let recorded = |document: Value| {
        sandbox.file("review.json", &document.to_string());
        let run_id = launched(
            &sandbox,
            &[
                "--kind",
                "impl",
                "--context",
                "review.json",
                "--prompt",
                "p",
            ],
        );
        let export = show(&sandbox, &run_id).report();
        (
            export["launch"]["creator"].clone(),
            show_text(&sandbox, &run_id),
        )
    };
    let artifact = |creator: Value| json!({ "schemaVersion": 1, "reviewedArtifact": { "id": "producer-k1", "creator": creator } });

    let (declared, shown) = recorded(artifact(json!({ "declared": "origin-q" })));
    assert_eq!(
        declared,
        json!({
            "reference": { "declared": "origin-q" },
            "evidence": "declared",
            "provider": "origin-q",
            "lookup": null,
        })
    );
    assert!(
        shown.contains("declared origin-q (declared by the owner)"),
        "{shown}"
    );

    let (unresolved, shown) = recorded(artifact(json!({ "run": creator })));
    assert_eq!(
        unresolved,
        json!({
            "reference": { "run": creator },
            "evidence": "execution_recorded",
            "provider": null,
            "lookup": null,
        })
    );
    assert!(shown.contains("not looked up"), "{shown}");

    let (none, shown) =
        recorded(json!({ "schemaVersion": 1, "reviewedArtifact": { "id": "producer-k1" } }));
    assert_eq!(none, Value::Null);
    assert!(
        shown
            .lines()
            .any(|line| line.trim_start() == "creator    none"),
        "{shown}"
    );
}

#[test]
fn the_typed_lookup_fixture_selects_through_the_worker() {
    // `worker/typecheck/lookup-policy.ts` is type-checked against the shipped
    // declarations by `task dispatch:typecheck`; here the same file runs, so
    // the declared answer and the runtime's agree.
    let sandbox = Sandbox::new();
    let creator = producer(&sandbox);
    sandbox.personal_policy(include_str!("../worker/typecheck/lookup-policy.ts"));

    review_context(&sandbox, &json!({ "run": creator }));
    let report = inspect_review(&sandbox).report();
    assert_eq!(report["policy"]["version"], "typecheck-lookup-1");
    assert_eq!(report["selection"]["candidateId"], "quick");
    let reason = report["selection"]["reason"].as_str().unwrap();
    for part in [creator.as_str(), "producer-k1", "origin-a"] {
        assert!(reason.contains(part), "{part:?} is not in {reason:?}");
    }
    let stderr = report["diagnostics"]["stderr"].as_str().unwrap();
    assert!(
        stderr.contains(&format!("looked up {creator}: found")),
        "{stderr}"
    );

    review_context(&sandbox, &json!({ "run": UNKNOWN }));
    let refusal = inspect_review(&sandbox).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused", "{refusal}");
    assert_eq!(
        refusal["error"]["policyCode"], "creator_missing",
        "{refusal}"
    );
}
