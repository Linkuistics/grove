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
//! a handoff attempt whose execution is unknown; an observable launch failure,
//! once a detail is appended; or, once a current observation confirms it, an
//! attempt whose execution an observer confirmed. There is no success: how the
//! work went is what observations measure, and the export never infers it.

use std::ffi::OsStr;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{json, Map, Value};

use crate::authority::Authority;
use crate::choice::Choice;
use crate::cli::ShowArgs;
use crate::observation;
use crate::program::ResolvedBy;
use crate::refusal::{Refusal, Stage, EXIT_MALFORMED, EXIT_REFUSED};
use crate::run_id::RunId;
use crate::store::{self, Lookup, StateDir, StoredObservation, StoredRun};

/// The launch document's own version, separate from the store's schema.
pub const LAUNCH_VERSION: u64 = 1;

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
    let context = choice.context.as_ref();
    json!({
        "schemaVersion": LAUNCH_VERSION,
        "kind": inputs.kind,
        "taskId": inputs.task_id,
        "taskFile": inputs.task_file,
        "reviewedArtifact": context.and_then(|context| context.reviewed_artifact()),
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
        // Digests and sizes only: the delivered value can hold whole sources.
        "context": context.map(|context| context.to_json(false)),
        "creator": null,
        "worker": {
            "path": worker.path.to_string_lossy(),
            "packageVersion": worker.package_version,
            "buildId": worker.build_id,
            "bunVersion": worker.bun_version,
        },
        "adapter": null,
        "bounds": inputs.limits.to_json(),
        "timing": { "selectionMs": millis(choice.elapsed) },
    })
}

/// A recorded run, as `record show` exports it.
pub struct Export {
    run_id: RunId,
    stored: StoredRun,
}

/// A record command's run and where its records live.
pub struct Located {
    pub run_id: RunId,
    pub cwd: PathBuf,
    pub dir: StateDir,
}

/// Parse `--run` and place the state directory, as `record show` and
/// `record observe` both do.
pub fn locate(run: &OsStr, state_dir: Option<&Path>) -> Result<Located, Refusal> {
    let given = run.to_str().ok_or_else(|| {
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
    let dir = StateDir::resolve(state_dir, &cwd, home.as_deref())?;
    Ok(Located { run_id, cwd, dir })
}

pub fn show(args: &ShowArgs) -> Result<Export, Refusal> {
    let Located { run_id, dir, .. } = locate(&args.run, args.state_dir.as_deref())?;
    match store::load(&dir, &run_id)? {
        Lookup::Found(stored) => Ok(Export { run_id, stored }),
        Lookup::Missing { store_exists } => Err(run_not_found(&dir, &run_id, store_exists)),
    }
}

/// The refusal for a run the store in `dir` does not hold.
pub fn run_not_found(dir: &StateDir, run_id: &RunId, store_exists: bool) -> Refusal {
    let file = dir.store();
    let message = if store_exists {
        format!("the record store {} holds no run {run_id}", file.display())
    } else {
        format!(
            "there is no record store at {}, so it holds no run {run_id}",
            file.display()
        )
    };
    Refusal::new(
        "run_not_found",
        Stage::Record,
        EXIT_REFUSED,
        message,
        format!(
            "check the run ID, and pass the --state-dir the run was recorded under; this looked \
             in {}",
            dir.to_text()
        ),
    )
    .input(format!("--run {run_id}"))
    .source(file.to_string_lossy())
}

impl Export {
    /// The observations no other one corrects, in recorded order, by ID.
    fn current(&self) -> impl Iterator<Item = (&str, &Value)> + Clone {
        self.stored
            .observations
            .iter()
            .filter(|observation| observation.superseded_by.is_none())
            .map(|observation| {
                let id = observation.document["observationId"]
                    .as_str()
                    .unwrap_or_default();
                (id, &observation.document)
            })
    }

    /// The run's evidence class and what it says of execution. harness-dispatch's
    /// own launch-failure detail comes first; `record observe` refuses a
    /// confirmation for such a run, so the two meet only if an observation was
    /// imported before the detail was appended.
    fn evidence(&self) -> (&'static str, &'static str) {
        if self.stored.launch_failure.is_some() {
            ("launch_failure", "not_executed")
        } else if self
            .current()
            .any(|(_, document)| observation::confirms_execution(document))
        {
            ("execution_confirmed", "confirmed")
        } else {
            ("handoff_attempt", "unknown")
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
        let observations: Vec<Value> = self.stored.observations.iter().map(exported).collect();
        json!({
            "schemaVersion": 1,
            "runId": self.run_id.as_str(),
            "recordedAt": self.stored.recorded_at,
            "evidence": evidence,
            "execution": execution,
            "launch": self.stored.launch,
            "launchFailure": launch_failure,
            "observations": observations,
            "measurements": observation::summary(self.current()),
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
            "execution_confirmed" => "execution confirmed: recorded before exec, and a current \
                                      observation says the harness ran"
                .to_owned(),
            _ => "launch failure: the harness was not executed".to_owned(),
        };
        let _ = writeln!(text, "  {:<10} {evidence}", "evidence");
        text.push_str(&measurements_text(&observation::summary(self.current())));
        let candidate = &launch["candidate"];
        let policy = &launch["policy"];
        let rows = [
            ("kind", field(&launch["kind"])),
            ("task id", field(&launch["taskId"])),
            ("task file", field(&launch["taskFile"])),
            ("reviewed", field(&launch["reviewedArtifact"])),
            (
                "context",
                match &launch["context"] {
                    Value::Null => "none".to_owned(),
                    context => format!(
                        "{} bytes encoded from {} sources ({} bytes), sha256 {}",
                        field(&context["encodedBytes"]),
                        context["sources"].as_array().map_or(0, Vec::len),
                        field(&context["sourceBytes"]),
                        field(&context["sha256"])
                    ),
                },
            ),
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
        for (index, stored) in self.stored.observations.iter().enumerate() {
            let label = if index == 0 { "observed" } else { "" };
            let document = &stored.document;
            let mut line = format!(
                "{} from {} at {} (recorded {})",
                field(&document["observationId"]),
                field(&document["source"]),
                field(&document["observedAt"]),
                stored.recorded_at
            );
            if let Some(supersedes) = document["supersedes"].as_str() {
                let _ = write!(line, ", supersedes {}", shown(supersedes));
            }
            if let Some(by) = &stored.superseded_by {
                let _ = write!(line, ", superseded by {}", shown(by));
            }
            let _ = writeln!(text, "  {label:<10} {line}");
        }
        text
    }
}

/// One stored observation as the export shows it: the envelope as imported,
/// with every supported measurement, when it was recorded, and what corrects it.
fn exported(stored: &StoredObservation) -> Value {
    let mut document = stored.document.as_object().cloned().unwrap_or_default();
    document.insert(
        "measurements".into(),
        Value::Object(observation::expanded(&stored.document)),
    );
    document.insert("recordedAt".into(), stored.recorded_at.clone().into());
    document.insert("supersededBy".into(), stored.superseded_by.clone().into());
    Value::Object(document)
}

/// The run's measurements for text output: one line per current value, then
/// the fields nothing current supplies.
fn measurements_text(summary: &Map<String, Value>) -> String {
    let mut text = String::new();
    let mut unobserved = Vec::new();
    let mut first = true;
    for (name, measurement) in summary {
        let current = measurement["current"]
            .as_array()
            .map_or(&[][..], Vec::as_slice);
        if current.is_empty() {
            unobserved.push(name.as_str());
            continue;
        }
        for entry in current {
            let label = if first { "measured" } else { "" };
            first = false;
            let value = match (&entry["value"], &entry["unit"]) {
                (Value::Null, _) => entry["state"].as_str().unwrap_or_default().to_owned(),
                (Value::String(value), _) => shown(value),
                (value, Value::String(unit)) => format!("{value} {}", shown(unit)),
                (value, _) => value.to_string(),
            };
            let _ = writeln!(
                text,
                "  {label:<10} {name} {value} (observation {})",
                shown(entry["observationId"].as_str().unwrap_or_default())
            );
        }
    }
    let unobserved = if unobserved.len() == summary.len() {
        "every measurement".to_owned()
    } else {
        unobserved.join(", ")
    };
    if !unobserved.is_empty() {
        let _ = writeln!(text, "  {:<10} {unobserved}", "unobserved");
    }
    text
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
