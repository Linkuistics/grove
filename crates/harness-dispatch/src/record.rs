//! What a run records, and `record show`
//! (`docs/specs/harness-selection-and-execution.md`, *Records and later
//! observations*).
//!
//! A run's launch fields are one JSON document, committed before exec and never
//! changed. Every field is present in it, `null` where the run has no value for
//! it, so a release that records something new fills it for new runs without
//! rewriting a committed one. Raw environment values are not stored. The document
//! describes the configured launched choice; it is not evidence that the
//! harness ran, or of which backend model it reached.
//!
//! The evidence a run carries is derived, never stored as a state to advance:
//! a handoff attempt whose execution is unknown; an observable launch failure,
//! once a detail is appended; or, once a current observation confirms it, an
//! attempt whose execution an observer confirmed. There is no success: how the
//! work went is what observations measure, and the export never infers it.
//!
//! A policy's `host.run` reads a recorded run too, as a projection of its
//! launch fields ([`lookup`]): what the run was launched as, never its argv,
//! which holds the prompt, and never its program or arguments, which no
//! context carries.

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
use crate::store::{self, Lookup, Observations, StateDir, StoredObservation, StoredRun};

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
        "creator": context.and_then(crate::context::Delivered::creator),
        "worker": {
            "path": worker.path.to_string_lossy(),
            "packageVersion": worker.package_version,
            "buildId": worker.build_id,
            "bunVersion": worker.bun_version,
        },
        "adapter": choice.adapter.as_ref().map(crate::worker::Adapter::to_json),
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
    let attempt = "read the run record";
    match store::load(&dir, &run_id, store::LOCK_WAIT, attempt, Observations::Read)? {
        Lookup::Found(stored) => Ok(Export { run_id, stored }),
        Lookup::Missing { store_exists } => Err(run_not_found(&dir, &run_id, store_exists)),
    }
}

/// Why a stored run's launch document, or its launch-failure detail, is not
/// one this release reads, beginning with what it is about. Every read of a
/// run checks this ([`store::load`], and `record observe` before it appends),
/// so `record show`, `record observe` and `host.run` never disagree about
/// which runs they can read. A run lookup, which types the kind, task
/// identity and candidate into its answer, also requires those fields.
pub fn readable(launch: &Value, failure: Option<&Value>) -> Result<(), String> {
    if !launch.is_object() {
        return Err("launch record is not a JSON object".to_owned());
    }
    if launch["schemaVersion"] != LAUNCH_VERSION {
        return Err(format!(
            "launch record is version {}, and this release of harness-dispatch reads version \
             {LAUNCH_VERSION}",
            launch["schemaVersion"]
        ));
    }
    if failure.is_some_and(|detail| !detail["cause"].is_string()) {
        return Err("launch failure has no cause".to_owned());
    }
    Ok(())
}

/// Answer a policy's `host.run(runId)` from the store in `dir`, waiting at
/// most `wait` for a writer's lock: the run's immutable launch fields and any
/// launch failure, or that the store does not hold it. A missing or empty
/// store holds no run. A store that cannot be read refuses, and so does a
/// launch document this release cannot read, rather than answer with less.
/// The run's observations are never read, so the lookup's work does not grow
/// with its history.
pub fn lookup(dir: &StateDir, run_id: &RunId, wait: Duration) -> Result<Value, Refusal> {
    let attempt = format!("look up run {run_id} for the policy's host.run");
    let stored = match store::load(dir, run_id, wait, &attempt, Observations::Skip)? {
        Lookup::Missing { .. } => {
            return Ok(json!({ "runId": run_id.as_str(), "status": "missing" }));
        }
        Lookup::Found(stored) => stored,
    };
    let launch = &stored.launch;
    let unreadable = |why: &str| {
        store::unreadable_record(dir, &attempt, format!("run {run_id}'s launch record {why}"))
    };
    let text = |value: &Value, field: &str| {
        value
            .as_str()
            .map(|_| value.clone())
            .ok_or_else(|| unreadable(&format!("has no string {field}")))
    };
    let candidate = &launch["candidate"];
    let task_id = match &launch["taskId"] {
        Value::Null => Value::Null,
        other => text(other, "taskId")?,
    };
    let failure = launch_failure(stored.launch_failure.as_ref());
    Ok(json!({
        "runId": run_id.as_str(),
        "status": "found",
        "recordedAt": stored.recorded_at,
        "kind": text(&launch["kind"], "kind")?,
        "taskId": task_id,
        "candidate": {
            "id": text(&candidate["id"], "candidate.id")?,
            "provider": text(&candidate["provider"], "candidate.provider")?,
            "model": text(&candidate["model"], "candidate.model")?,
            "effort": text(&candidate["effort"], "candidate.effort")?,
        },
        "launchFailure": failure,
    }))
}

/// A run's launch-failure detail as it is exported and looked up: when it was
/// appended, beside the detail's own fields; `null` when there is none.
fn launch_failure(failure: Option<&(String, Value)>) -> Value {
    let Some((recorded_at, detail)) = failure else {
        return Value::Null;
    };
    let mut exported = Map::new();
    exported.insert("recordedAt".into(), recorded_at.clone().into());
    if let Some(detail) = detail.as_object() {
        exported.extend(detail.clone());
    }
    Value::Object(exported)
}

/// The creator provenance a run or a proposal records, for a person: the
/// reference, its evidence class, and what a lookup found, with the run's
/// task identity beside its recorded choice. `creator` is the JSON that
/// [`crate::context::Delivered::creator`] makes, or `null`.
pub fn creator_text(creator: &Value) -> String {
    if creator.is_null() {
        return "none".to_owned();
    }
    let reference = &creator["reference"];
    if let Some(declared) = reference["declared"].as_str() {
        return format!("declared {} (declared by the owner)", shown(declared));
    }
    let run = reference["run"].as_str().unwrap_or_default();
    let lookup = &creator["lookup"];
    let found = match lookup["status"].as_str() {
        None => "not looked up by loadContext".to_owned(),
        Some("missing") => "missing from the record store".to_owned(),
        Some(_) => {
            let candidate = &lookup["candidate"];
            let field = |value: &Value| value.as_str().map_or_else(|| "none".to_owned(), shown);
            let mut found = format!(
                "provider {}, model {}, effort {}; task {}, kind {}, recorded {}",
                field(&candidate["provider"]),
                field(&candidate["model"]),
                field(&candidate["effort"]),
                field(&lookup["taskId"]),
                field(&lookup["kind"]),
                field(&lookup["recordedAt"]),
            );
            if let Some(cause) = lookup["launchFailure"]["cause"].as_str() {
                let _ = write!(found, "; launch failure: {}", shown(cause));
            }
            found
        }
    };
    format!("run {run} (execution-recorded): {found}")
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
            .map(|observation| (observation.observation_id.as_str(), &observation.document))
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
        let launch_failure = launch_failure(self.stored.launch_failure.as_ref());
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
        let mut rows = vec![
            ("kind", field(&launch["kind"])),
            ("task id", field(&launch["taskId"])),
            ("task file", field(&launch["taskFile"])),
            ("reviewed", field(&launch["reviewedArtifact"])),
            ("creator", creator_text(&launch["creator"])),
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
                "adapter",
                match &launch["adapter"] {
                    Value::Null => "none".to_owned(),
                    adapter => format!(
                        "{} {}",
                        field(&adapter["specifier"]),
                        field(&adapter["version"])
                    ),
                },
            ),
            (
                "timing",
                format!(
                    "selection took {} ms",
                    field(&launch["timing"]["selectionMs"])
                ),
            ),
        ];
        // Each measured source under the context's own row, the task file a
        // review's creator came from included, with its digest.
        let sources = launch["context"]["sources"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default();
        let at = rows
            .iter()
            .position(|(label, _)| *label == "context")
            .map_or(rows.len(), |at| at + 1);
        for (index, source) in sources.iter().enumerate().rev() {
            let line = format!(
                "source [{index}] {} ({}, {} bytes, sha256 {})",
                field(&source["name"]),
                field(&source["via"]),
                field(&source["bytes"]),
                field(&source["sha256"])
            );
            rows.insert(at, ("", line));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A store holding one run whose launch document is `launch`.
    fn committed(launch: &Value) -> (tempfile::TempDir, StateDir, RunId) {
        let dir = tempfile::tempdir().unwrap();
        let state = StateDir {
            path: dir.path().join("state"),
            flag: Some("--state-dir"),
        };
        let run_id = RunId::allocate().unwrap();
        store::commit(&state, &run_id, launch).unwrap();
        (dir, state, run_id)
    }

    fn launch() -> Value {
        json!({
            "schemaVersion": 1, "kind": "impl", "taskId": null,
            "candidate": {
                "id": "c", "provider": "origin-a", "model": "m", "effort": "e",
                "program": "harness", "args": [{ "slot": "prompt" }],
            },
            "argv": ["harness", "the prompt"],
        })
    }

    #[test]
    fn a_later_launch_version_or_a_missing_field_is_refused_never_answered_with_less() {
        let (_dir, state, run) = committed(&launch());
        let answer = lookup(&state, &run, store::LOCK_WAIT).unwrap();
        assert_eq!(
            answer["candidate"],
            json!({ "id": "c", "provider": "origin-a", "model": "m", "effort": "e" })
        );
        assert_eq!(answer["taskId"], Value::Null);
        assert!(
            answer.get("argv").is_none(),
            "the prompt is never looked up"
        );

        let mut later = launch();
        later["schemaVersion"] = 2.into();
        let mut no_provider = launch();
        no_provider["candidate"]
            .as_object_mut()
            .unwrap()
            .remove("provider");
        let mut numeric_kind = launch();
        numeric_kind["kind"] = 3.into();
        let mut numeric_task = launch();
        numeric_task["taskId"] = 3.into();
        for (launch, why) in [
            (later, "is version 2"),
            (no_provider, "has no string candidate.provider"),
            (numeric_kind, "has no string kind"),
            (numeric_task, "has no string taskId"),
        ] {
            let (_dir, state, run) = committed(&launch);
            let refusal = lookup(&state, &run, store::LOCK_WAIT).unwrap_err();
            assert_eq!(refusal.code, "record_store_invalid", "{why}");
            assert_eq!(refusal.exit, crate::refusal::EXIT_RECORD, "{why}");
            assert!(refusal.message.contains(why), "{why}: {}", refusal.message);
            assert!(refusal.message.contains("host.run"), "{}", refusal.message);
        }
    }
}
