//! What a run records, and `record show`
//! (`docs/specs/harness-selection-and-execution.md`, *Records and later
//! observations*).
//!
//! A run's launch fields are one JSON document, committed before exec and never
//! changed. Every field a later increment supplies is present already, as
//! `null`, so that increment fills it for new runs without rewriting a
//! committed one. Raw environment values are not stored. The document
//! describes the configured launched choice; it is not evidence that the
//! harness ran, or of which backend model it reached.
//!
//! The evidence a run carries is derived, never stored as a state to advance:
//! a handoff attempt whose execution is unknown, or, once a detail is appended,
//! an observable launch failure. There is no success.

use std::fmt::Write as _;
use std::time::Duration;

use serde_json::{json, Map, Value};

use crate::authority::Authority;
use crate::choice::Choice;
use crate::cli::ShowArgs;
use crate::program::ResolvedBy;
use crate::refusal::{Refusal, Stage, EXIT_MALFORMED, EXIT_REFUSED};
use crate::run_id::RunId;
use crate::store::{self, Lookup, StateDir, StoredRun};

/// The launch document's own version, separate from the store's schema.
pub const LAUNCH_VERSION: u64 = 1;

/// The outcome measurements the spec names. Nothing in this release observes
/// any of them, so each is exported as unobserved. `run-observations-k25`
/// owns the observation envelope and may refine these names before the first
/// release.
const OUTCOMES: [&str; 11] = [
    "executionConfirmation",
    "exit",
    "duration",
    "inputUsage",
    "outputUsage",
    "totalUsage",
    "acceptance",
    "missedDefects",
    "falseFindings",
    "downstreamRepair",
    "humanWork",
];

/// The launch fields `run` commits for `choice`.
pub fn launch(choice: &Choice) -> Value {
    let mut policy = Map::new();
    policy.insert("path".into(), choice.entry.display().into());
    match &choice.entry.authority {
        Authority::Personal => {
            policy.insert("authority".into(), "personal".into());
        }
        Authority::Explicit { argument } => {
            policy.insert("authority".into(), "explicit".into());
            policy.insert("argument".into(), argument.to_string_lossy().into());
        }
    }
    policy.insert("sha256".into(), choice.entry.sha256.clone().into());
    policy.insert("version".into(), choice.version.clone().into());
    let executable = &choice.executable;
    let resolved_by = match executable.resolved_by {
        ResolvedBy::Absolute => "absolute",
        ResolvedBy::Cwd => "cwd",
        // The matching PATH entry is a piece of the caller's PATH value, so it
        // is not stored; the resolved path is.
        ResolvedBy::Path { .. } => "PATH",
    };
    let worker = &choice.worker;
    let inputs = &choice.inputs;
    json!({
        "schemaVersion": LAUNCH_VERSION,
        "kind": inputs.kind,
        "taskId": inputs.task_id,
        "taskFile": inputs.task_file,
        "reviewedArtifact": null,
        "cwd": inputs.cwd.to_string_lossy(),
        "policy": policy,
        "selection": {
            "form": choice.selected_by.form(),
            "selectedBy": choice.selected_by.as_str(),
            "explicitChoice": inputs.choice,
            "reason": choice.reason,
        },
        "candidate": choice.candidate.to_json(),
        "executable": {
            "program": executable.program,
            "resolvedBy": resolved_by,
            "path": executable.path.to_string_lossy(),
        },
        "argv": choice.argv.iter().map(crate::argv::Word::to_json).collect::<Vec<_>>(),
        "context": null,
        "creator": null,
        "worker": {
            "path": worker.path.to_string_lossy(),
            "packageVersion": worker.package_version,
            "buildId": worker.build_id,
            "bunVersion": worker.bun_version,
        },
        "adapter": null,
        "bounds": { "selection": inputs.selection.to_json() },
        "timing": { "selectionMs": millis(choice.elapsed) },
    })
}

/// A recorded run, as `record show` exports it.
pub struct Export {
    run_id: RunId,
    stored: StoredRun,
}

pub fn show(args: &ShowArgs) -> Result<Export, Refusal> {
    let given = args.run.to_str().ok_or_else(|| {
        Refusal::new(
            "malformed_input",
            Stage::Cli,
            EXIT_MALFORMED,
            "--run is not valid UTF-8",
            "pass a run ID exactly as harness-dispatch reported it",
        )
        .input("--run")
    })?;
    let run_id = RunId::parse(given, "--run")?;
    let cwd = std::env::current_dir().map_err(|error| {
        Refusal::new(
            "cwd_unavailable",
            Stage::Record,
            EXIT_REFUSED,
            format!("the current directory cannot be read: {error}"),
            "run harness-dispatch from an existing, readable directory",
        )
        .input("cwd")
    })?;
    let home = std::env::var_os("HOME");
    let dir = StateDir::resolve(args.state_dir.as_deref(), &cwd, home.as_deref())?;
    match store::load(&dir, &run_id)? {
        Lookup::Found(stored) => Ok(Export { run_id, stored }),
        Lookup::Missing { store_exists } => {
            let file = dir.store();
            let message = if store_exists {
                format!("the record store {} holds no run {run_id}", file.display())
            } else {
                format!(
                    "there is no record store at {}, so it holds no run {run_id}",
                    file.display()
                )
            };
            Err(Refusal::new(
                "run_not_found",
                Stage::Record,
                EXIT_REFUSED,
                message,
                format!(
                    "check the run ID, and pass the --state-dir the run was recorded under; this \
                     looked in {}",
                    dir.to_text()
                ),
            )
            .input(format!("--run {run_id}"))
            .source(file.to_string_lossy()))
        }
    }
}

impl Export {
    fn evidence(&self) -> (&'static str, &'static str) {
        match self.stored.launch_failure {
            None => ("handoff_attempt", "unknown"),
            Some(_) => ("launch_failure", "not_executed"),
        }
    }

    pub fn to_json(&self) -> Value {
        let (evidence, execution) = self.evidence();
        let launch_failure = match &self.stored.launch_failure {
            None => Value::Null,
            Some((recorded_at, detail)) => {
                let mut failure = Map::new();
                failure.insert("recordedAt".into(), recorded_at.clone().into());
                if let Some(detail) = detail.as_object() {
                    failure.extend(detail.clone());
                }
                Value::Object(failure)
            }
        };
        let outcomes: Map<String, Value> = OUTCOMES
            .iter()
            .map(|name| ((*name).to_owned(), json!({ "state": "unobserved" })))
            .collect();
        json!({
            "schemaVersion": 1,
            "runId": self.run_id.as_str(),
            "recordedAt": self.stored.recorded_at,
            "evidence": evidence,
            "execution": execution,
            "launch": self.stored.launch,
            "launchFailure": launch_failure,
            "observations": [],
            "outcomes": outcomes,
        })
    }

    pub fn to_text(&self) -> String {
        let (evidence, _) = self.evidence();
        let launch = &self.stored.launch;
        let field = |value: &Value| match value {
            Value::Null => "none".to_owned(),
            Value::String(text) => shown(text),
            other => other.to_string(),
        };
        let mut text = format!(
            "Run {} (launch record version {}), recorded {}\n",
            self.run_id,
            field(&launch["schemaVersion"]),
            self.stored.recorded_at
        );
        let evidence = match evidence {
            "handoff_attempt" => "handoff attempt: recorded before exec; whether the harness ran \
                                  is unknown"
                .to_owned(),
            _ => "launch failure: the harness was not executed".to_owned(),
        };
        let candidate = &launch["candidate"];
        let policy = &launch["policy"];
        let rows = [
            ("evidence", evidence),
            (
                "outcomes",
                "every outcome unobserved (exit, duration, usage, acceptance, findings, repair, \
                 human work)"
                    .to_owned(),
            ),
            ("kind", field(&launch["kind"])),
            ("task id", field(&launch["taskId"])),
            ("task file", field(&launch["taskFile"])),
            ("cwd", field(&launch["cwd"])),
            (
                "policy",
                format!(
                    "{} ({}, version {}, sha256 {})",
                    field(&policy["path"]),
                    field(&policy["authority"]),
                    field(&policy["version"]),
                    field(&policy["sha256"])
                ),
            ),
            ("choice", field(&launch["selection"]["explicitChoice"])),
            ("selected", field(&launch["selection"]["selectedBy"])),
            ("candidate", field(&candidate["id"])),
            ("provider", field(&candidate["provider"])),
            ("model", field(&candidate["model"])),
            ("effort", field(&candidate["effort"])),
            ("reason", field(&launch["selection"]["reason"])),
            ("executable", field(&launch["executable"]["path"])),
            (
                "worker",
                format!(
                    "package {}, build {}, Bun {}",
                    field(&launch["worker"]["packageVersion"]),
                    field(&launch["worker"]["buildId"]),
                    field(&launch["worker"]["bunVersion"])
                ),
            ),
            (
                "timing",
                format!(
                    "selection took {} ms",
                    field(&launch["timing"]["selectionMs"])
                ),
            ),
        ];
        for (label, value) in rows {
            let _ = writeln!(text, "  {label:<10} {value}");
        }
        if let Some(argv) = launch["argv"].as_array() {
            for (index, word) in argv.iter().enumerate() {
                let label = if index == 0 { "argv" } else { "" };
                let word = match word {
                    Value::String(word) => format!("{word:?}"),
                    other => other.to_string(),
                };
                let _ = writeln!(text, "  {label:<10} [{index}] {word}");
            }
        }
        if let Some((recorded_at, detail)) = &self.stored.launch_failure {
            let _ = writeln!(
                text,
                "  {:<10} {} (recorded {recorded_at})",
                "failure",
                field(&detail["message"])
            );
        }
        text
    }
}

/// A recorded value as it is, unless a control character would break the row.
fn shown(value: &str) -> String {
    if value.chars().any(char::is_control) {
        format!("{value:?}")
    } else {
        value.to_owned()
    }
}

fn millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}
