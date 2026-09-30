//! Later observations through the command seam: `record observe` validates and
//! appends a version-1 observation against a recorded run, and `record show`
//! exports it (`docs/specs/harness-selection-and-execution.md`, *Records and
//! later observations*).
//!
//! Runs are made the ordinary way, by `run` with a fake harness, and the store
//! is read back through `record show` and, to see what a refusal did or did not
//! change, directly with SQLite. Every refusal has a positive control, the same
//! import with its fault removed, that is seen to be recorded.

mod support;

use std::fs;
use std::path::Path;

use rusqlite::Connection;
use serde_json::{json, Value};
use support::{executable, run, Sandbox, ROUTED};

/// Routes `impl` to `./broken-harness`, whose `#!` interpreter does not exist,
/// so exec fails after the commit and a launch failure is appended.
const BROKEN: &str = r#"export const policy = {
  schemaVersion: 1,
  version: "observations-broken",
  catalog: [
    { id: "broken", provider: "origin-b", model: "model-b", effort: "low", program: "./broken-harness", args: [{ slot: "prompt" }] },
  ],
  routes: { impl: "broken" },
};
"#;

/// A successful run's ID, taken from the harness it reached.
fn launched(sandbox: &Sandbox, args: &[&str]) -> String {
    let result = sandbox.run(args);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let run_id = sandbox.harness_run_id();
    fs::remove_dir_all(&sandbox.record).unwrap();
    run_id
}

fn impl_run(sandbox: &Sandbox) -> String {
    launched(sandbox, &["--kind", "impl", "--prompt", "p"])
}

/// A version-1 observation of `run_id` with `measurements`.
fn observation(run_id: &str, id: &str, measurements: Value) -> Value {
    json!({
        "schemaVersion": 1,
        "observationId": id,
        "runId": run_id,
        "source": "a test observer",
        "observedAt": "2026-10-01T09:30:00Z",
        "evidence": "what the test saw",
        "measurements": measurements,
    })
}

/// Write `document` to `relative` under the sandbox's cwd and import it.
fn observe(sandbox: &Sandbox, run_id: &str, relative: &str, document: &Value) -> support::Run {
    sandbox.file(relative, &serde_json::to_string_pretty(document).unwrap());
    observe_file(sandbox, run_id, relative, &["--json"])
}

fn observe_file(sandbox: &Sandbox, run_id: &str, relative: &str, extra: &[&str]) -> support::Run {
    let mut command = sandbox.command();
    command
        .args(["record", "observe", "--run", run_id, "--file", relative])
        .args(extra);
    run(&mut command)
}

fn recorded(result: &support::Run) -> Value {
    result.report()["observation"].clone()
}

fn show(sandbox: &Sandbox, run_id: &str) -> Value {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id, "--json"]);
    run(&mut command).report()
}

fn show_text(sandbox: &Sandbox, run_id: &str) -> String {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id]);
    let result = run(&mut command);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    result.stdout
}

fn observation_count(store: &Path) -> i64 {
    Connection::open(store)
        .unwrap()
        .query_row("SELECT count(*) FROM observations", [], |row| row.get(0))
        .unwrap()
}

fn user_version(store: &Path) -> i64 {
    Connection::open(store)
        .unwrap()
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap()
}

/// Every supported measurement, observed.
fn every_measurement(run_id: &str) -> Value {
    json!({
        "executionConfirmation": { "state": "observed", "value": true },
        "exit": { "state": "observed", "value": { "code": 0 } },
        "duration": { "state": "observed", "value": 12.5, "unit": "s" },
        "inputUsage": { "state": "observed", "value": 1200, "unit": "tokens" },
        "outputUsage": { "state": "observed", "value": 300, "unit": "tokens" },
        "totalUsage": { "state": "observed", "value": 0.42, "unit": "USD" },
        "acceptance": { "state": "observed", "value": "accepted" },
        "missedDefects": { "state": "observed", "value": [{ "id": "F3", "summary": "unbounded read", "repairs": ["repair-f3"] }] },
        "falseFindings": { "state": "observed", "value": [] },
        "downstreamRepair": { "state": "observed", "value": [{ "id": "R1", "findings": ["F3"], "runId": run_id }] },
        "humanTime": { "state": "observed", "value": 20, "unit": "min" },
        "humanInterventions": { "state": "observed", "value": 2 },
        "evidenceLinks": { "state": "observed", "value": ["https://example.com/pr/12"] },
        "choiceProbability": { "state": "observed", "value": 0.6 },
        "successProbability": { "state": "observed", "value": { "probability": 0.8, "uncalibrated": true } },
    })
}

#[test]
fn an_observation_round_trips_through_observe_and_show() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let measurements = every_measurement(&run_id);
    let document = observation(&run_id, "o-1", measurements.clone());

    let receipt = recorded(&observe(&sandbox, &run_id, "o-1.json", &document));
    assert_eq!(receipt["observationId"], "o-1");
    assert_eq!(receipt["runId"], run_id.as_str());
    assert_eq!(receipt["status"], "recorded");
    assert_eq!(receipt["supersedes"], Value::Null);
    let recorded_at = receipt["recordedAt"].as_str().unwrap().to_owned();
    assert!(recorded_at.ends_with('Z'), "{recorded_at}");

    let export = show(&sandbox, &run_id);
    assert_eq!(export["evidence"], "execution_confirmed");
    assert_eq!(export["execution"], "confirmed");
    let mut expected = document.clone();
    expected["recordedAt"] = json!(recorded_at);
    expected["supersededBy"] = Value::Null;
    assert_eq!(export["observations"], json!([expected]));

    let summary = export["measurements"].as_object().unwrap();
    assert_eq!(summary.len(), measurements.as_object().unwrap().len());
    for (name, measurement) in measurements.as_object().unwrap() {
        let mut entry = measurement.clone();
        entry["observationId"] = json!("o-1");
        assert_eq!(
            summary[name],
            json!({ "state": "observed", "current": [entry] }),
            "{name}"
        );
    }

    let text = show_text(&sandbox, &run_id);
    for expected in [
        "execution confirmed",
        "measured",
        "duration 12.5 s (observation o-1)",
        "acceptance accepted (observation o-1)",
        "o-1 from a test observer at 2026-10-01T09:30:00Z",
    ] {
        assert!(text.contains(expected), "{expected}:\n{text}");
    }
    assert!(!text.contains("unobserved"), "{text}");

    let again = observe_file(&sandbox, &run_id, "o-1.json", &[]);
    assert_eq!(again.code, Some(0), "{}", again.stderr);
    assert!(
        again.stdout.contains("is already recorded") && again.stdout.contains("nothing changed"),
        "{}",
        again.stdout
    );
}

#[test]
fn unsupplied_measurements_stay_unobserved_and_an_attempt_stays_unconfirmed() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let before = show(&sandbox, &run_id);
    assert_eq!(before["observations"], json!([]));
    for (name, measurement) in before["measurements"].as_object().unwrap() {
        assert_eq!(
            measurement,
            &json!({ "state": "unobserved", "current": [] }),
            "{name}"
        );
    }
    assert!(show_text(&sandbox, &run_id).contains("unobserved every measurement"));

    let accepted = observation(
        &run_id,
        "o-accepted",
        json!({
            "acceptance": { "state": "observed", "value": "accepted" },
            "executionConfirmation": { "state": "unknown" },
            "totalUsage": { "state": "unobserved" },
        }),
    );
    recorded(&observe(&sandbox, &run_id, "accepted.json", &accepted));
    let export = show(&sandbox, &run_id);
    // Acceptance says nothing about execution, and neither does an unknown.
    assert_eq!(export["evidence"], "handoff_attempt");
    assert_eq!(export["execution"], "unknown");
    let exported = &export["observations"][0]["measurements"];
    assert_eq!(exported.as_object().unwrap().len(), 15);
    assert_eq!(exported["acceptance"]["value"], "accepted");
    assert_eq!(
        exported["executionConfirmation"],
        json!({ "state": "unknown" })
    );
    for absent in [
        "duration",
        "exit",
        "missedDefects",
        "humanTime",
        "totalUsage",
    ] {
        assert_eq!(
            exported[absent],
            json!({ "state": "unobserved" }),
            "{absent}"
        );
    }
    let summary = &export["measurements"];
    assert_eq!(summary["acceptance"]["state"], "observed");
    assert_eq!(summary["executionConfirmation"]["state"], "unknown");
    for absent in ["duration", "falseFindings", "totalUsage"] {
        assert_eq!(
            summary[absent],
            json!({ "state": "unobserved", "current": [] }),
            "{absent}"
        );
    }

    let confirmed = observation(
        &run_id,
        "o-confirmed",
        json!({ "executionConfirmation": { "state": "observed", "value": true } }),
    );
    recorded(&observe(&sandbox, &run_id, "confirmed.json", &confirmed));
    let export = show(&sandbox, &run_id);
    assert_eq!(export["evidence"], "execution_confirmed");
    assert_eq!(export["execution"], "confirmed");
    assert_eq!(
        export["measurements"]["executionConfirmation"]["current"]
            .as_array()
            .unwrap()
            .len(),
        2,
        "both current values are listed"
    );
}

#[test]
fn a_repeat_is_idempotent_and_a_conflicting_repeat_refuses() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let other_run = impl_run(&sandbox);
    let store = sandbox.default_store();
    let document = observation(
        &run_id,
        "o-1",
        json!({ "duration": { "state": "observed", "value": 3, "unit": "s" } }),
    );
    let first = recorded(&observe(&sandbox, &run_id, "o-1.json", &document));
    assert_eq!(first["status"], "recorded");

    // The same content, compactly and in another key order.
    let mut reordered = String::from("{");
    for (index, (key, value)) in document.as_object().unwrap().iter().rev().enumerate() {
        if index > 0 {
            reordered.push(',');
        }
        reordered.push_str(&format!("{}:{value}", json!(key)));
    }
    reordered.push('}');
    for file in ["o-1.json", "reordered.json"] {
        sandbox.file("reordered.json", &reordered);
        let repeat = recorded(&observe_file(&sandbox, &run_id, file, &["--json"]));
        assert_eq!(repeat["status"], "already_recorded", "{file}");
        assert_eq!(repeat["recordedAt"], first["recordedAt"], "{file}");
    }
    assert_eq!(observation_count(&store), 1);

    let mut changed = document.clone();
    changed["measurements"]["duration"]["value"] = json!(4);
    let refusal = observe(&sandbox, &run_id, "changed.json", &changed).refusal(3);
    assert_eq!(refusal["error"]["code"], "observation_conflict");
    assert_eq!(refusal["error"]["stage"], "record");
    assert_eq!(refusal["error"]["location"], "observation.observationId");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("with different content"),
        "{refusal}"
    );

    let mut elsewhere = document.clone();
    elsewhere["runId"] = json!(other_run);
    let refusal = observe(&sandbox, &other_run, "elsewhere.json", &elsewhere).refusal(3);
    assert_eq!(refusal["error"]["code"], "observation_conflict");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains(&format!("against run {run_id}")),
        "{refusal}"
    );
    assert_eq!(observation_count(&store), 1);
    assert_eq!(
        show(&sandbox, &run_id)["observations"][0]["measurements"]["duration"]["value"],
        3
    );

    // Positive control: a new ID with the changed content is recorded.
    changed["observationId"] = json!("o-2");
    recorded(&observe(&sandbox, &run_id, "o-2.json", &changed));
    assert_eq!(observation_count(&store), 2);
}

#[test]
fn a_correction_supersedes_its_observation_and_both_are_kept() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let other_run = impl_run(&sandbox);
    let original = observation(
        &run_id,
        "o-1",
        json!({
            "acceptance": { "state": "observed", "value": "accepted" },
            "duration": { "state": "observed", "value": 3, "unit": "s" },
        }),
    );
    recorded(&observe(&sandbox, &run_id, "o-1.json", &original));
    let mut correction = observation(
        &run_id,
        "o-2",
        json!({ "acceptance": { "state": "observed", "value": "rejected" } }),
    );
    correction["supersedes"] = json!("o-1");
    let receipt = recorded(&observe(&sandbox, &run_id, "o-2.json", &correction));
    assert_eq!(receipt["supersedes"], "o-1");

    let export = show(&sandbox, &run_id);
    let observations = export["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 2, "both are kept");
    assert_eq!(observations[0]["observationId"], "o-1");
    assert_eq!(observations[0]["supersededBy"], "o-2");
    assert_eq!(
        observations[0]["measurements"]["acceptance"]["value"],
        "accepted"
    );
    assert_eq!(observations[1]["supersedes"], "o-1");
    assert_eq!(observations[1]["supersededBy"], Value::Null);
    // The correction replaces the whole observation: what it omits is no
    // longer current.
    assert_eq!(
        export["measurements"]["acceptance"],
        json!({ "state": "observed", "current": [
            { "observationId": "o-2", "state": "observed", "value": "rejected" },
        ] })
    );
    assert_eq!(
        export["measurements"]["duration"],
        json!({ "state": "unobserved", "current": [] })
    );
    let text = show_text(&sandbox, &run_id);
    assert!(text.contains("superseded by o-2"), "{text}");
    assert!(text.contains("supersedes o-1"), "{text}");

    // Repeating the correction after it took effect is the same import.
    let repeat = recorded(&observe_file(&sandbox, &run_id, "o-2.json", &["--json"]));
    assert_eq!(repeat["status"], "already_recorded");

    let mut second = observation(&run_id, "o-3", json!({}));
    second["supersedes"] = json!("o-1");
    let refusal = observe(&sandbox, &run_id, "o-3.json", &second).refusal(3);
    assert_eq!(refusal["error"]["code"], "already_superseded");
    assert_eq!(refusal["error"]["location"], "observation.supersedes");

    second["supersedes"] = json!("o-missing");
    let refusal = observe(&sandbox, &run_id, "o-3.json", &second).refusal(3);
    assert_eq!(refusal["error"]["code"], "supersedes_unknown");

    let foreign = observation(&other_run, "o-other", json!({}));
    recorded(&observe(&sandbox, &other_run, "other.json", &foreign));
    second["supersedes"] = json!("o-other");
    let refusal = observe(&sandbox, &run_id, "o-3.json", &second).refusal(3);
    assert_eq!(refusal["error"]["code"], "supersedes_unknown");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains(&format!("belongs to run {other_run}")),
        "{refusal}"
    );

    // Positive control: correcting the current correction extends the chain.
    second["supersedes"] = json!("o-2");
    recorded(&observe(&sandbox, &run_id, "o-3.json", &second));
    let export = show(&sandbox, &run_id);
    assert_eq!(export["observations"][1]["supersededBy"], "o-3");
    assert_eq!(
        export["measurements"]["acceptance"],
        json!({ "state": "unobserved", "current": [] })
    );
}

#[test]
fn unit_and_state_errors_refuse_at_their_location_and_record_nothing() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let other_run = impl_run(&sandbox);
    let measured =
        |measurement: Value| observation(&run_id, "o-bad", json!({ "duration": measurement }));
    let mut version_2 = measured(json!({ "state": "unknown" }));
    version_2["schemaVersion"] = json!(2);
    let mut elsewhere = measured(json!({ "state": "unknown" }));
    elsewhere["runId"] = json!(other_run);
    let cases = [
        (
            measured(json!({ "state": "observed", "value": 3 })),
            "observation_invalid",
            "observation.measurements.duration.unit",
        ),
        (
            measured(json!({ "state": "observed", "value": 3, "unit": "fortnights" })),
            "observation_invalid",
            "observation.measurements.duration.unit",
        ),
        (
            observation(
                &run_id,
                "o-bad",
                json!({ "humanInterventions": { "state": "observed", "value": 2, "unit": "events" } }),
            ),
            "observation_invalid",
            "observation.measurements.humanInterventions.unit",
        ),
        (
            measured(json!({ "state": "unknown", "value": 0 })),
            "observation_invalid",
            "observation.measurements.duration.value",
        ),
        (
            measured(json!({ "state": "observed", "unit": "s" })),
            "observation_invalid",
            "observation.measurements.duration.value",
        ),
        (
            measured(json!({ "state": "absent" })),
            "observation_invalid",
            "observation.measurements.duration.state",
        ),
        (
            observation(
                &run_id,
                "o-bad",
                json!({ "successProbability": { "state": "observed", "value": { "probability": 0.9 } } }),
            ),
            "observation_invalid",
            "observation.measurements.successProbability.value",
        ),
        (
            version_2,
            "unsupported_version",
            "observation.schemaVersion",
        ),
        (elsewhere, "observation_invalid", "observation.runId"),
    ];
    for (document, code, location) in cases {
        let refusal = observe(&sandbox, &run_id, "bad.json", &document).refusal(3);
        assert_eq!(refusal["error"]["code"], code, "{document}");
        assert_eq!(refusal["error"]["stage"], "observation", "{document}");
        assert_eq!(refusal["error"]["location"], location, "{document}");
        assert_eq!(refusal["error"]["input"], "--file", "{document}");
    }
    assert_eq!(observation_count(&sandbox.default_store()), 0);

    // Positive control: the same observation with a unit is recorded.
    let fixed = measured(json!({ "state": "observed", "value": 3, "unit": "s" }));
    recorded(&observe(&sandbox, &run_id, "bad.json", &fixed));
    assert_eq!(observation_count(&sandbox.default_store()), 1);
}

#[test]
fn observations_survive_the_deletion_of_the_task_tree_that_named_the_run() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let task = sandbox.file(".grove/03-impl--parser-k7.md", "# parser-k7\n");
    let run_id = launched(
        &sandbox,
        &[
            "--kind",
            "impl",
            "--task-file",
            ".grove/03-impl--parser-k7.md",
            "--task-id",
            "parser-k7",
            "--prompt",
            "p",
        ],
    );
    fs::remove_dir_all(sandbox.cwd.join(".grove")).unwrap();
    assert!(!task.exists());

    let document = observation(
        &run_id,
        "after-teardown",
        json!({ "acceptance": { "state": "observed", "value": "accepted" } }),
    );
    recorded(&observe(&sandbox, &run_id, "after.json", &document));
    let export = show(&sandbox, &run_id);
    assert_eq!(export["launch"]["taskId"], "parser-k7");
    assert_eq!(export["launch"]["taskFile"], support::text(&task));
    assert_eq!(export["observations"][0]["observationId"], "after-teardown");
    assert_eq!(export["measurements"]["acceptance"]["state"], "observed");
}

#[test]
fn no_import_changes_the_launch_fields() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let store = sandbox.default_store();
    let launch_text = || -> String {
        Connection::open(&store)
            .unwrap()
            .query_row(
                "SELECT recorded_at || launch FROM runs WHERE run_id = ?1",
                [&run_id],
                |row| row.get(0),
            )
            .unwrap()
    };
    let before_text = launch_text();
    let before = show(&sandbox, &run_id);

    let mut claims = observation(&run_id, "o-launch", json!({}));
    claims["launch"] = json!({ "kind": "review", "candidate": { "provider": "origin-z" } });
    let refusal = observe(&sandbox, &run_id, "claims.json", &claims).refusal(3);
    assert_eq!(refusal["error"]["location"], "observation.launch");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("no observation can supply or change"),
        "{refusal}"
    );

    recorded(&observe(
        &sandbox,
        &run_id,
        "full.json",
        &observation(&run_id, "o-full", every_measurement(&run_id)),
    ));
    let mut correction = observation(&run_id, "o-fix", json!({}));
    correction["supersedes"] = json!("o-full");
    recorded(&observe(&sandbox, &run_id, "fix.json", &correction));

    assert_eq!(launch_text(), before_text);
    let after = show(&sandbox, &run_id);
    for field in ["runId", "recordedAt", "launch", "launchFailure"] {
        assert_eq!(after[field], before[field], "{field}");
    }

    let connection = Connection::open(&store).unwrap();
    for (statement, reason) in [
        ("UPDATE observations SET document = '{}'", "never changes"),
        ("DELETE FROM observations", "never removed"),
        ("UPDATE runs SET launch = '{}'", "never change"),
    ] {
        let error = connection.execute_batch(statement).unwrap_err();
        assert!(error.to_string().contains(reason), "{statement}: {error}");
    }
    drop(connection);
    assert_eq!(observation_count(&store), 2);
}

#[test]
fn an_unknown_run_or_a_missing_store_refuses_and_creates_nothing() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let unknown = "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e";
    let document = observation(unknown, "o-1", json!({}));
    let refusal = observe(&sandbox, unknown, "o-1.json", &document).refusal(3);
    assert_eq!(refusal["error"]["code"], "run_not_found");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("there is no record store"),
        "{refusal}"
    );
    assert!(!sandbox.home.join(".local").exists(), "a store was created");

    let refusal = observe_file(&sandbox, "not-a-run", "o-1.json", &["--json"]).refusal(2);
    assert_eq!(refusal["error"]["code"], "malformed_input");
    assert_eq!(refusal["error"]["input"], "--run");

    impl_run(&sandbox);
    let refusal = observe(&sandbox, unknown, "o-1.json", &document).refusal(3);
    assert_eq!(refusal["error"]["code"], "run_not_found");
    assert_eq!(observation_count(&sandbox.default_store()), 0);
}

#[test]
fn a_confirmation_contradicting_a_recorded_launch_failure_refuses() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(BROKEN);
    executable(
        &sandbox.cwd.join("broken-harness"),
        "#!/nonexistent/interpreter\n",
    );
    let result = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(result.code, Some(127), "{}", result.stderr);
    let notice: Value = serde_json::from_str(result.stderr.lines().next().unwrap()).unwrap();
    let run_id = notice["handoff"]["runId"].as_str().unwrap().to_owned();

    let confirms = observation(
        &run_id,
        "o-ran",
        json!({ "executionConfirmation": { "state": "observed", "value": true } }),
    );
    let refusal = observe(&sandbox, &run_id, "ran.json", &confirms).refusal(3);
    assert_eq!(refusal["error"]["code"], "observation_contradicts_record");
    assert_eq!(
        refusal["error"]["location"],
        "observation.measurements.executionConfirmation"
    );
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("exec_error"),
        "{refusal}"
    );

    // Positive control: an observation that does not claim execution is kept.
    let unsure = observation(
        &run_id,
        "o-unsure",
        json!({ "executionConfirmation": { "state": "unknown" } }),
    );
    recorded(&observe(&sandbox, &run_id, "unsure.json", &unsure));
    let export = show(&sandbox, &run_id);
    assert_eq!(export["evidence"], "launch_failure");
    assert_eq!(export["execution"], "not_executed");
    assert_eq!(export["observations"].as_array().unwrap().len(), 1);
}

/// Schema 1's tables, as `handoff-records-k24` created them, with one run.
const VERSION_1_STORE: &str = "
CREATE TABLE runs (run_id TEXT PRIMARY KEY NOT NULL, recorded_at TEXT NOT NULL, launch TEXT NOT NULL);
CREATE TABLE launch_failures (run_id TEXT PRIMARY KEY NOT NULL REFERENCES runs (run_id), recorded_at TEXT NOT NULL, detail TEXT NOT NULL);
CREATE TRIGGER runs_never_change BEFORE UPDATE ON runs BEGIN SELECT RAISE(ABORT, 'a committed run''s launch fields never change'); END;
CREATE TRIGGER runs_are_never_removed BEFORE DELETE ON runs BEGIN SELECT RAISE(ABORT, 'a committed run is never removed'); END;
CREATE TRIGGER launch_failures_never_change BEFORE UPDATE ON launch_failures BEGIN SELECT RAISE(ABORT, 'a recorded launch failure never changes'); END;
CREATE TRIGGER launch_failures_are_never_removed BEFORE DELETE ON launch_failures BEGIN SELECT RAISE(ABORT, 'a recorded launch failure is never removed'); END;
INSERT INTO runs VALUES ('5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34', '2026-09-30T08:15:42.117Z', '{\"schemaVersion\":1,\"kind\":\"impl\"}');
PRAGMA application_id = 1212437075;
PRAGMA user_version = 1;
";

#[test]
fn a_version_1_store_is_migrated_by_its_first_observation_and_read_as_it_is_otherwise() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let store = sandbox.default_store();
    fs::create_dir_all(store.parent().unwrap()).unwrap();
    Connection::open(&store)
        .unwrap()
        .execute_batch(VERSION_1_STORE)
        .unwrap();
    let old_run = "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34";

    let export = show(&sandbox, old_run);
    assert_eq!(export["launch"]["kind"], "impl");
    assert_eq!(export["observations"], json!([]));
    let new_run = impl_run(&sandbox);
    assert_eq!(user_version(&store), 1, "only record observe migrates");

    // A refused import rolls its migration back with it.
    let mut refused = observation(old_run, "o-1", json!({}));
    refused["supersedes"] = json!("o-missing");
    let refusal = observe(&sandbox, old_run, "o-1.json", &refused).refusal(3);
    assert_eq!(refusal["error"]["code"], "supersedes_unknown");
    assert_eq!(user_version(&store), 1);

    let document = observation(
        old_run,
        "o-1",
        json!({ "acceptance": { "state": "observed", "value": "accepted" } }),
    );
    recorded(&observe(&sandbox, old_run, "o-1.json", &document));
    assert_eq!(user_version(&store), 2);
    assert_eq!(observation_count(&store), 1);
    assert_eq!(
        show(&sandbox, old_run)["observations"][0]["observationId"],
        "o-1"
    );
    assert_eq!(show(&sandbox, &new_run)["observations"], json!([]));

    // A later version still refuses, and is left as it was.
    Connection::open(&store)
        .unwrap()
        .execute_batch("PRAGMA user_version = 3;")
        .unwrap();
    let before = fs::read(&store).unwrap();
    let refusal = observe(
        &sandbox,
        old_run,
        "o-2.json",
        &observation(old_run, "o-2", json!({})),
    )
    .refusal(4);
    assert_eq!(refusal["error"]["code"], "record_store_invalid");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("record schema version 3"),
        "{refusal}"
    );
    assert_eq!(fs::read(&store).unwrap(), before);
}

#[test]
fn the_observation_file_must_be_a_readable_bounded_json_file() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);

    let refusal = observe_file(&sandbox, &run_id, "missing.json", &["--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "observation_unreadable");
    assert_eq!(refusal["error"]["stage"], "observation");

    fs::create_dir(sandbox.cwd.join("a-directory")).unwrap();
    let refusal = observe_file(&sandbox, &run_id, "a-directory", &["--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "observation_unreadable");

    sandbox.file("not-json.json", "{ \"schemaVersion\": 1,");
    let refusal = observe_file(&sandbox, &run_id, "not-json.json", &["--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "observation_invalid");
    assert_eq!(refusal["error"]["location"], "observation");

    // One byte over the bound, in whitespace after a valid observation.
    let valid = serde_json::to_string(&observation(&run_id, "o-big", json!({}))).unwrap();
    let padding = 1_048_576 + 1 - valid.len();
    sandbox.file("big.json", &format!("{valid}{}", " ".repeat(padding)));
    let refusal = observe_file(&sandbox, &run_id, "big.json", &["--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "observation_too_large");
    assert_eq!(
        refusal["error"]["bound"],
        json!({ "name": "observation", "bytes": 1_048_576, "from": "fixed" })
    );

    // Positive control: at the bound exactly, it is recorded.
    sandbox.file("big.json", &format!("{valid}{}", " ".repeat(padding - 1)));
    recorded(&observe_file(&sandbox, &run_id, "big.json", &["--json"]));
    assert_eq!(observation_count(&sandbox.default_store()), 1);
}

#[test]
fn help_carries_an_observation_example() {
    let sandbox = Sandbox::new();
    let mut command = sandbox.command();
    command.args(["record", "observe", "--help"]);
    let help = run(&mut command);
    assert_eq!(help.code, Some(0), "{}", help.stderr);
    for expected in [
        "--run",
        "--file",
        "--state-dir",
        "\"schemaVersion\": 1",
        "\"observationId\"",
        "\"measurements\"",
        "\"state\": \"observed\"",
        "supersedes",
        "harness-dispatch record observe --run",
    ] {
        assert!(
            help.stdout.contains(expected),
            "{expected}:\n{}",
            help.stdout
        );
    }
    let mut command = sandbox.command();
    command.arg("--help");
    let top = run(&mut command);
    assert!(
        top.stdout.contains("harness-dispatch record observe --run"),
        "{}",
        top.stdout
    );

    // The example in help is itself a valid observation.
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let start = help.stdout.find("  {\n").expect("the example's first line");
    let end = help.stdout[start..]
        .find("\n  }\n")
        .expect("the example's last line")
        + start
        + 4;
    let example = help.stdout[start..end].replace("0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e", &run_id);
    let example: Value = serde_json::from_str(&example).expect("the example is JSON");
    recorded(&observe(&sandbox, &run_id, "example.json", &example));
}
