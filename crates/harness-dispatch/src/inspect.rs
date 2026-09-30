//! `inspect`: evaluate the selected policy and report its choice, launching
//! nothing (`docs/specs/harness-selection-and-execution.md`, *Command interface*).
//!
//! Inspection is a proposal. It is not a launch reservation, and evaluating
//! trusted TypeScript is not promised to be free of side effects. The report
//! states the facts in both forms: human text, and one version-1 JSON object on
//! stdout. Argument expansion arrives with `run`; until then the report shows
//! no argv at all rather than a placeholder that would imply expansion ran.

use std::fmt::Write as _;
use std::path::Path;
use std::time::Duration;

use serde_json::{json, Map, Value};

use crate::authority::{self, Authority, PolicyEntry, PERSONAL_DEFAULT};
use crate::cli::InspectArgs;
use crate::policy::{self, Candidate, Validator};
use crate::refusal::{Diagnostics, Failure, Refusal, Stage, EXIT_REFUSED};
use crate::worker::{self, Outcome, WorkerIdentity};

pub struct Report {
    kind: String,
    entry: PolicyEntry,
    version: String,
    candidate: Candidate,
    reason: String,
    elapsed: Duration,
    worker: WorkerIdentity,
    diagnostics: Diagnostics,
}

pub fn inspect(args: &InspectArgs) -> Result<Report, Failure> {
    args.refuse_unsupported()?;
    let cwd = std::env::current_dir().map_err(|error| {
        Refusal::new(
            "cwd_unavailable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the current directory cannot be read: {error}"),
            "run harness-dispatch from an existing, readable directory",
        )
    })?;
    let home = std::env::var_os("HOME");
    let entry = authority::resolve(args.config.as_deref(), &cwd, home.as_deref())?;
    let source = entry.display();

    let worker_path = worker::locate()?;
    let request = json!({
        "schemaVersion": 1,
        "kind": args.kind,
        "cwd": cwd.to_string_lossy(),
    });
    let evaluation = worker::evaluate(&worker_path, &entry.path, request)?;
    let diagnostics = evaluation.diagnostics;
    let refuse = |refusal: Refusal| Failure::with_diagnostics(refusal, diagnostics.clone());

    let snapshot = match evaluation.outcome {
        Outcome::Policy(snapshot) => snapshot,
        Outcome::LoadFailed { name, message } => {
            return Err(refuse(
                Refusal::new(
                    "policy_import_failed",
                    Stage::Load,
                    EXIT_REFUSED,
                    format!("the policy entry {source} failed to load: {name}: {message}"),
                    format!(
                        "fix {source} or the module it imports; relative imports resolve from \
                         the importing file, bare ones through node_modules beside it, and \
                         nothing is installed automatically"
                    ),
                )
                .source(source.clone()),
            ));
        }
        Outcome::Invalid { location, message } => {
            return Err(refuse(
                Refusal::new(
                    "policy_invalid",
                    Stage::Validation,
                    EXIT_REFUSED,
                    message,
                    format!(
                        "export a plain object as `export const policy = {{ ... }}` from {source}"
                    ),
                )
                .source(source.clone())
                .location(location),
            ));
        }
    };
    let mut policy = Validator::new(&source)
        .validate(&snapshot)
        .map_err(refuse)?;
    let selection = policy::route(&policy, &args.kind, &source).map_err(refuse)?;
    let reason = selection.reason;
    let index = policy
        .catalog
        .iter()
        .position(|candidate| candidate.id == selection.candidate.id)
        .expect("the selection came from this catalog");
    let candidate = policy.catalog.swap_remove(index);

    Ok(Report {
        kind: args.kind.clone(),
        entry,
        version: policy.version,
        candidate,
        reason,
        elapsed: evaluation.elapsed,
        worker: evaluation.worker,
        diagnostics,
    })
}

impl Report {
    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }

    pub fn to_json(&self) -> Value {
        let mut policy = Map::new();
        policy.insert("path".into(), self.entry.display().into());
        match &self.entry.authority {
            Authority::Personal => {
                policy.insert("authority".into(), "personal".into());
            }
            Authority::Explicit { argument } => {
                policy.insert("authority".into(), "explicit".into());
                policy.insert("argument".into(), argument.to_string_lossy().into());
            }
        }
        policy.insert("version".into(), self.version.clone().into());
        let candidate = &self.candidate;
        json!({
            "schemaVersion": 1,
            "evidence": "proposal",
            "kind": self.kind,
            "policy": policy,
            "selection": {
                "form": "routes",
                "selectedBy": "route",
                "candidateId": candidate.id,
                "provider": candidate.provider,
                "model": candidate.model,
                "effort": candidate.effort,
                "reason": self.reason,
            },
            "timing": { "selectionMs": millis(self.elapsed) },
            "worker": worker_json(&self.worker),
            "diagnostics": self.diagnostics.to_json(),
        })
    }

    pub fn to_text(&self) -> String {
        let authority = match &self.entry.authority {
            Authority::Personal => format!("personal (the default ~/{PERSONAL_DEFAULT})"),
            Authority::Explicit { argument } => format!(
                "explicit (--config {}, resolved against the current directory)",
                Path::new(argument).display()
            ),
        };
        let candidate = &self.candidate;
        let worker = &self.worker;
        let rows = [
            ("policy", self.entry.display()),
            ("authority", authority),
            ("version", self.version.clone()),
            ("kind", self.kind.clone()),
            ("candidate", candidate.id.clone()),
            ("provider", candidate.provider.clone()),
            ("model", candidate.model.clone()),
            ("effort", candidate.effort.clone()),
            ("reason", self.reason.clone()),
            (
                "timing",
                format!("selection took {} ms", millis(self.elapsed)),
            ),
            (
                "worker",
                format!(
                    "{} (package {}, build {}, Bun {})",
                    worker.path.display(),
                    worker.package_version,
                    &worker.build_id[..worker.build_id.len().min(12)],
                    worker.bun_version
                ),
            ),
        ];
        let mut text =
            String::from("Proposal only: nothing was launched and no run was recorded.\n");
        for (label, value) in rows {
            let _ = writeln!(text, "  {label:<10} {value}");
        }
        text
    }
}

fn worker_json(worker: &WorkerIdentity) -> Value {
    json!({
        "path": worker.path.to_string_lossy(),
        "packageVersion": worker.package_version,
        "buildId": worker.build_id,
        "bunVersion": worker.bun_version,
    })
}

fn millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}
