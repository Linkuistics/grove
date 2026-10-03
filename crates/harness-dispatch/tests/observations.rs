//! Later observations through the command seam: `record observe` validates and
//! appends a version-1 observation against a recorded run, and `record show`
//! exports it (`docs/specs/harness-selection-and-execution.md`, *Records and
//! later observations*).
//!
//! Runs are made the ordinary way, by `run` with a fake harness, and the store
//! is read back through `record show` and, to see what a refusal did or did not
//! change, directly with SQLite. Every refusal has a positive control, the same
//! import with its fault removed, that is seen to be recorded. One test
//! observes a run that harness-dispatch 21.13.0 recorded under the catalog
//! contract (`tests/fixtures/catalog-contract`).

mod support;

use std::fs;
use std::path::Path;

use rusqlite::Connection;
use serde_json::{json, Value};
use support::{executable, run, Sandbox, ROUTED};

/// Runs `./broken-harness`, whose `#!` interpreter does not exist, so exec
/// fails after the commit and a launch failure is appended.
const BROKEN: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "observations-broken",
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
const ROUTED_RUN: &str = "45308255-7446-42b2-bbd7-e5f7861e46ef";
const CHOSEN_RUN: &str = "b2aec552-b5eb-4de0-b5e9-5db0d70a6d55";

/// A successful run's ID, taken from the harness it reached. The run has none
/// of dispatch's own observation of its end: these cases are about what an
/// outside observer imports, and count and order what they import, so they
/// start from a run no observation has reached. Dispatch's own end observation
/// has its cases in `tests/supervision.rs`, and `without_dispatchs_end` says
/// how it is taken away.
fn launched(sandbox: &Sandbox, args: &[&str]) -> String {
    let result = sandbox.run(args);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let run_id = sandbox.harness_run_id();
    fs::remove_dir_all(&sandbox.record).unwrap();
    without_dispatchs_end(&sandbox.default_store());
    run_id
}

/// Take dispatch's end observations out of the store at `store`, by the one
/// route the store's own triggers leave: dropping the trigger that keeps an
/// observation for good, and putting it back. Only a test makes such a store.
fn without_dispatchs_end(store: &Path) {
    let connection = Connection::open(store).unwrap();
    connection
        .execute_batch(
            "DROP TRIGGER observations_are_never_removed;
             DELETE FROM observations WHERE json_extract(document, '$.source') = 'harness-dispatch';
             CREATE TRIGGER observations_are_never_removed BEFORE DELETE ON observations
             BEGIN SELECT RAISE(ABORT, 'a recorded observation is never removed'); END;",
        )
        .unwrap();
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
        "ending": { "state": "observed", "value": "harness_exit" },
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

/// The same, with each measurement whose value has a second form in that
/// form: an exit by signal, findings listed where there were none and none
/// where they were listed, and a calibrated success estimate.
fn every_measurement_in_its_other_form(run_id: &str) -> Value {
    let mut measurements = every_measurement(run_id);
    measurements["exit"] = json!({ "state": "observed", "value": { "signal": "SIGKILL" } });
    measurements["missedDefects"] = json!({ "state": "observed", "value": [] });
    measurements["falseFindings"] = json!({ "state": "observed", "value": [
        { "id": "F1", "summary": "not a defect", "repairs": ["repair-f1"] },
        { "id": "F2" },
    ] });
    measurements["successProbability"] = json!({
        "state": "observed",
        "value": { "probability": 0.7, "calibration": "pilot-2026-10" },
    });
    measurements
}

#[test]
fn an_observation_round_trips_through_observe_and_show() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    // Each form on a run of its own, so that each export holds one
    // observation and each measurement one current value.
    for (id, measurements, worded) in [
        (
            "o-1",
            every_measurement as fn(&str) -> Value,
            [
                r#"exit {"code":0} (observation o-1)"#,
                "falseFindings [] (observation o-1)",
                r#"successProbability {"probability":0.8,"uncalibrated":true} (observation o-1)"#,
            ],
        ),
        (
            "o-2",
            every_measurement_in_its_other_form,
            [
                r#"exit {"signal":"SIGKILL"} (observation o-2)"#,
                r#"falseFindings [{"id":"F1","repairs":["repair-f1"],"summary":"not a defect"},{"id":"F2"}] (observation o-2)"#,
                r#"successProbability {"calibration":"pilot-2026-10","probability":0.7} (observation o-2)"#,
            ],
        ),
    ] {
        let run_id = impl_run(&sandbox);
        let measurements = measurements(&run_id);
        let document = observation(&run_id, id, measurements.clone());
        let file = format!("{id}.json");

        let receipt = recorded(&observe(&sandbox, &run_id, &file, &document));
        assert_eq!(receipt["observationId"], id);
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
        assert_eq!(export["observations"], json!([expected]), "{id}");

        let summary = export["measurements"].as_object().unwrap();
        assert_eq!(summary.len(), measurements.as_object().unwrap().len());
        for (name, measurement) in measurements.as_object().unwrap() {
            let mut entry = measurement.clone();
            entry["observationId"] = json!(id);
            assert_eq!(
                summary[name],
                json!({ "state": "observed", "current": [entry] }),
                "{id} {name}"
            );
        }

        let text = show_text(&sandbox, &run_id);
        for expected in [
            "execution confirmed".to_owned(),
            "measured".to_owned(),
            format!("duration 12.5 s (observation {id})"),
            format!("acceptance accepted (observation {id})"),
            format!("{id} from a test observer at 2026-10-01T09:30:00Z"),
        ]
        .iter()
        .map(String::as_str)
        .chain(worded)
        {
            assert!(text.contains(expected), "{expected}:\n{text}");
        }
        assert!(!text.contains("unobserved"), "{text}");

        let again = observe_file(&sandbox, &run_id, &file, &[]);
        assert_eq!(again.code, Some(0), "{}", again.stderr);
        assert!(
            again.stdout.contains("is already recorded")
                && again.stdout.contains("nothing changed"),
            "{}",
            again.stdout
        );
    }
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
    assert_eq!(exported.as_object().unwrap().len(), 16);
    assert_eq!(exported["acceptance"]["value"], "accepted");
    assert_eq!(
        exported["executionConfirmation"],
        json!({ "state": "unknown" })
    );
    let summary = &export["measurements"];
    assert_eq!(summary["acceptance"]["state"], "observed");
    assert_eq!(summary["executionConfirmation"]["state"], "unknown");
    // Every other measurement, by name: the one the document gave as
    // unobserved, and each one it left out. The names are the fixture's own
    // list of every measurement, not the export's.
    let every = every_measurement(&run_id);
    let absent: Vec<&str> = every
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .filter(|name| !["acceptance", "executionConfirmation"].contains(name))
        .collect();
    assert_eq!(absent.len(), 14, "{absent:?}");
    for absent in absent {
        assert_eq!(
            exported[absent],
            json!({ "state": "unobserved" }),
            "{absent}"
        );
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

#[test]
fn a_stored_observation_this_release_cannot_read_refuses_the_export() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run_id = impl_run(&sandbox);
    let other_run = impl_run(&sandbox);
    recorded(&observe(
        &sandbox,
        &run_id,
        "o-1.json",
        &observation(&run_id, "o-1", json!({})),
    ));
    let store = sandbox.default_store();
    let good = fs::read(&store).unwrap();
    let restore = || fs::write(&store, &good).unwrap();

    let stored = |id: &str, measurements: Value| {
        serde_json::to_string(&observation(&run_id, id, measurements)).unwrap()
    };
    let confirmed = json!({ "executionConfirmation": { "state": "observed", "value": true } });
    let mut later = observation(&run_id, "o-2", json!({}));
    later["schemaVersion"] = json!(2);
    let mut corrects = observation(&run_id, "o-2", json!({}));
    corrects["supersedes"] = json!("o-1");
    // (row supersedes, stored document, what the refusal says); the row's ID
    // is always o-2, of `run_id`.
    let cases = [
        (None, "{".to_owned(), "is not valid JSON"),
        (None, "{}".to_owned(), "`schemaVersion` is missing"),
        (
            None,
            stored(
                "o-2",
                json!({ "executionConfirmation": { "state": "observed", "value": false } }),
            ),
            "an execution confirmation's value is true",
        ),
        (None, later.to_string(), "schemaVersion 2 is not supported"),
        (None, stored("o-9", json!({})), "names observation \"o-9\""),
        (
            None,
            serde_json::to_string(&observation(&other_run, "o-2", json!({}))).unwrap(),
            "is about run",
        ),
        (None, corrects.to_string(), "supersedes"),
        // The controls: the same row, written with a document the import
        // would have written, is exported.
        (None, stored("o-2", confirmed.clone()), ""),
        (Some("o-1"), corrects.to_string(), ""),
    ];
    for (supersedes, document, why) in cases {
        restore();
        Connection::open(&store)
            .unwrap()
            .execute(
                "INSERT INTO observations (observation_id, run_id, recorded_at, supersedes, \
                 document) VALUES ('o-2', ?1, '2026-10-01T09:31:00.000Z', ?2, ?3)",
                rusqlite::params![run_id, supersedes, document],
            )
            .unwrap();
        let mut command = sandbox.command();
        command.args(["record", "show", "--run", &run_id, "--json"]);
        let result = run(&mut command);
        if why.is_empty() {
            let export = result.report();
            assert_eq!(
                export["observations"][1]["observationId"], "o-2",
                "{document}"
            );
            continue;
        }
        let refusal = result.refusal(4);
        let error = &refusal["error"];
        assert_eq!(
            error["code"], "record_store_invalid",
            "{document}: {refusal}"
        );
        assert_eq!(error["stage"], "record", "{document}: {refusal}");
        assert_eq!(
            error["source"],
            support::text(&store),
            "{document}: {refusal}"
        );
        let message = error["message"].as_str().unwrap();
        for part in [run_id.as_str(), "\"o-2\"", why] {
            assert!(message.contains(part), "{document}: {part:?}: {refusal}");
        }
    }
    // The last control confirmed execution through a well-formed document; a
    // value of false, above, never did.
    restore();
    Connection::open(&store)
        .unwrap()
        .execute(
            "INSERT INTO observations (observation_id, run_id, recorded_at, document) \
             VALUES ('o-2', ?1, '2026-10-01T09:31:00.000Z', ?2)",
            rusqlite::params![run_id, stored("o-2", confirmed)],
        )
        .unwrap();
    assert_eq!(show(&sandbox, &run_id)["evidence"], "execution_confirmed");
}

#[test]
fn a_run_recorded_under_the_catalog_contract_is_observed_and_its_neighbour_left_alone() {
    let sandbox = Sandbox::new();
    let store = sandbox.default_store();
    fs::create_dir_all(store.parent().unwrap()).unwrap();
    fs::write(&store, CATALOG_STORE).unwrap();
    let version = user_version(&store);
    let before = show(&sandbox, ROUTED_RUN);
    let neighbour = show(&sandbox, CHOSEN_RUN);
    assert_eq!(before["evidence"], "handoff_attempt");
    assert_eq!(before["launch"]["candidate"]["id"], "builder");
    assert_eq!(observation_count(&store), 0);

    let confirmed = observation(
        ROUTED_RUN,
        "o-ran",
        json!({
            "executionConfirmation": { "state": "observed", "value": true },
            "acceptance": { "state": "observed", "value": "accepted" },
        }),
    );
    let receipt = recorded(&observe(&sandbox, ROUTED_RUN, "ran.json", &confirmed));
    assert_eq!(receipt["observationId"], "o-ran");
    assert_eq!(observation_count(&store), 1);
    assert_eq!(
        user_version(&store),
        version,
        "the store needed no migration"
    );

    // The observation is exported with the run, whose execution it confirms,
    // and the launch document is the one that release committed.
    let after = show(&sandbox, ROUTED_RUN);
    assert_eq!(after["evidence"], "execution_confirmed");
    assert_eq!(after["execution"], "confirmed");
    assert_eq!(after["observations"].as_array().unwrap().len(), 1);
    assert_eq!(after["observations"][0]["observationId"], "o-ran");
    assert_eq!(after["observations"][0]["runId"], ROUTED_RUN);
    assert_eq!(
        after["measurements"]["acceptance"],
        json!({
            "state": "observed",
            "current": [{ "observationId": "o-ran", "state": "observed", "value": "accepted" }],
        })
    );
    for field in ["runId", "recordedAt", "launch", "launchFailure"] {
        assert_eq!(after[field], before[field], "{field}");
    }
    let text = show_text(&sandbox, ROUTED_RUN);
    assert!(text.contains("execution confirmed"), "{text}");
    assert!(text.contains("o-ran from a test observer"), "{text}");

    // A correction of it is kept beside it, as for any run.
    let mut correction = observation(
        ROUTED_RUN,
        "o-fix",
        json!({ "executionConfirmation": { "state": "unknown" } }),
    );
    correction["supersedes"] = json!("o-ran");
    recorded(&observe(&sandbox, ROUTED_RUN, "fix.json", &correction));
    let corrected = show(&sandbox, ROUTED_RUN);
    assert_eq!(corrected["evidence"], "handoff_attempt");
    assert_eq!(corrected["observations"].as_array().unwrap().len(), 2);
    assert_eq!(corrected["observations"][0]["supersededBy"], "o-fix");

    // An observation naming the other run is not this run's, and that run is
    // exactly as it was.
    let misdirected = observation(CHOSEN_RUN, "o-other", json!({}));
    let refusal = observe(&sandbox, ROUTED_RUN, "other.json", &misdirected).refusal(3);
    assert_eq!(refusal["error"]["location"], "observation.runId");
    assert_eq!(show(&sandbox, CHOSEN_RUN), neighbour);
    assert_eq!(observation_count(&store), 2);
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
    assert_eq!(user_version(&store), 1, "reading migrates nothing");

    // A refused import rolls its migration back with it.
    let mut refused = observation(old_run, "o-1", json!({}));
    refused["supersedes"] = json!("o-missing");
    let refusal = observe(&sandbox, old_run, "o-1.json", &refused).refusal(3);
    assert_eq!(refusal["error"]["code"], "supersedes_unknown");
    assert_eq!(user_version(&store), 1);

    // So does one refused because the run's launch record is of a later
    // version, which the import reads inside that transaction.
    let later_run = "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e35";
    Connection::open(&store)
        .unwrap()
        .execute(
            "INSERT INTO runs VALUES (?1, '2026-09-30T08:15:43.000Z', '{\"schemaVersion\":2}')",
            [later_run],
        )
        .unwrap();
    let later = observation(later_run, "o-later", json!({}));
    let refusal = observe(&sandbox, later_run, "o-later.json", &later).refusal(4);
    assert_eq!(
        refusal["error"]["code"], "record_store_invalid",
        "{refusal}"
    );
    assert_eq!(user_version(&store), 1);

    // A run's own end observation is an observation too: the first one
    // appended migrates the store, with no import between. The run is the
    // ordinary kind, so dispatch's end observation stays on it.
    let result = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let new_run = sandbox.harness_run_id();
    assert_eq!(user_version(&store), 2, "dispatch's end migrated the store");
    assert_eq!(observation_count(&store), 1);
    let own = show(&sandbox, &new_run);
    assert_eq!(own["evidence"], "execution_confirmed");
    assert_eq!(own["observations"][0]["source"], "harness-dispatch");

    let document = observation(
        old_run,
        "o-1",
        json!({ "acceptance": { "state": "observed", "value": "accepted" } }),
    );
    recorded(&observe(&sandbox, old_run, "o-1.json", &document));
    assert_eq!(user_version(&store), 2);
    assert_eq!(observation_count(&store), 2);
    assert_eq!(
        show(&sandbox, old_run)["observations"][0]["observationId"],
        "o-1"
    );
    assert_eq!(
        show(&sandbox, &new_run),
        own,
        "an import touched another run"
    );

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
