//! `inspect`: evaluate the selected policy and report its choice, launching
//! nothing (`docs/specs/harness-selection-and-execution.md`, *Command interface*).
//!
//! Inspection is a proposal. It is not a launch reservation, and evaluating
//! trusted TypeScript is not promised to be free of side effects. It makes the
//! same choice `run` would, including argv expansion and program resolution,
//! and reports it in both forms: human text, and one version-1 JSON object on
//! stdout. Without a prompt, the prompt's argument is a marked placeholder.
//!
//! It records nothing. It reads the record store only to answer the policy's
//! run lookups, as `run` does, so that both make the same choice; it never
//! creates a store or writes a run. It reports where `run` would record, and a
//! proposed run ID, visibly marked, which no run holds and no later `run`
//! reuses.

use std::fmt::Write as _;
use std::path::Path;
use std::time::Duration;

use serde_json::{json, Map, Value};

use crate::argv::RunSlot;
use crate::authority::{Authority, PERSONAL_DEFAULT};
use crate::choice::{self, Choice};
use crate::cli::InspectArgs;
use crate::context::Delivered;
use crate::inputs::{PromptRequirement, PromptSource};
use crate::policy::SelectedBy;
use crate::record;
use crate::refusal::{Diagnostics, Failure};
use crate::run_id::RunId;
use crate::worker::WorkerIdentity;

pub struct Report(Choice);

pub fn inspect(args: &InspectArgs) -> Result<Report, Failure> {
    let proposed = RunSlot::Proposed(RunId::allocate()?);
    choice::choose(&args.selection, PromptRequirement::Optional, proposed)?
        .settle()
        .map(Report)
}

impl Report {
    pub fn diagnostics(&self) -> &Diagnostics {
        &self.0.diagnostics
    }

    pub fn to_json(&self) -> Value {
        let choice = &self.0;
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
        let prompt = match &choice.inputs.prompt {
            None => json!({ "supplied": false }),
            Some(prompt) => {
                let mut report = Map::new();
                report.insert("supplied".into(), true.into());
                match &prompt.source {
                    PromptSource::Argument => {
                        report.insert("from".into(), "--prompt".into());
                    }
                    PromptSource::File(path) => {
                        report.insert("from".into(), "--prompt-file".into());
                        report.insert("path".into(), path.to_string_lossy().into());
                    }
                }
                report.insert("bytes".into(), prompt.text.len().into());
                Value::Object(report)
            }
        };
        let candidate = &choice.candidate;
        let context = choice.context.as_ref();
        json!({
            "schemaVersion": 1,
            "evidence": "proposal",
            "proposedRunId": choice.run.id().as_str(),
            "stateDir": choice.state_dir.to_json(),
            "kind": choice.inputs.kind,
            "taskFile": choice.inputs.task_file,
            "taskId": choice.inputs.task_id,
            "prompt": prompt,
            "reviewedArtifact": context.and_then(|context| context.reviewed_artifact()),
            "creator": context.and_then(Delivered::creator),
            "context": context.map(|context| context.to_json(true)),
            "policy": policy,
            "selection": {
                "form": choice.selected_by.form(),
                "selectedBy": choice.selected_by.as_str(),
                "explicitChoice": choice.inputs.choice,
                "candidateId": candidate.id,
                "provider": candidate.provider,
                "model": candidate.model,
                "effort": candidate.effort,
                "reason": choice.reason,
            },
            "executable": choice.executable.to_json(),
            "argv": choice.argv.iter().map(crate::argv::Word::to_json).collect::<Vec<_>>(),
            "bounds": choice.inputs.limits.to_json(),
            "timing": { "selectionMs": millis(choice.elapsed) },
            "worker": worker_json(&choice.worker),
            "diagnostics": choice.diagnostics.to_json(),
        })
    }

    pub fn to_text(&self) -> String {
        let choice = &self.0;
        let authority = match &choice.entry.authority {
            Authority::Personal => format!("personal (the default ~/{PERSONAL_DEFAULT})"),
            Authority::Explicit { argument } => format!(
                "explicit (--config {}, resolved against the current directory)",
                Path::new(argument).display()
            ),
        };
        let inputs = &choice.inputs;
        let prompt = match &inputs.prompt {
            None => "not supplied; argv shows a placeholder where it goes".to_owned(),
            Some(prompt) => match &prompt.source {
                PromptSource::Argument => format!("{} bytes from --prompt", prompt.text.len()),
                PromptSource::File(path) => format!(
                    "{} bytes from --prompt-file {}",
                    prompt.text.len(),
                    shown(&path.to_string_lossy())
                ),
            },
        };
        let candidate = &choice.candidate;
        let worker = &choice.worker;
        let rows = [
            ("policy", choice.entry.display()),
            ("authority", authority),
            ("version", choice.version.clone()),
            ("sha256", choice.entry.sha256.clone()),
            ("kind", shown(&inputs.kind)),
            (
                "task file",
                inputs.task_file.as_deref().map_or("none".to_owned(), shown),
            ),
            (
                "task id",
                inputs.task_id.as_deref().map_or("none".to_owned(), shown),
            ),
            ("prompt", prompt),
            (
                "reviewed",
                choice
                    .context
                    .as_ref()
                    .and_then(|context| context.reviewed_artifact())
                    .map_or("none".to_owned(), |artifact| shown(&artifact.to_string())),
            ),
            (
                "creator",
                record::creator_text(
                    &choice
                        .context
                        .as_ref()
                        .and_then(Delivered::creator)
                        .unwrap_or_default(),
                ),
            ),
            (
                "choice",
                match (inputs.choice.as_deref(), choice.selected_by) {
                    (None, SelectedBy::Select) => "none; the policy's select chooses".to_owned(),
                    (None, _) => "none; the routes select by kind".to_owned(),
                    (Some(id), SelectedBy::Select) => {
                        format!("--choice {}, which the policy's select accepted", shown(id))
                    }
                    (Some(id), _) => {
                        format!("--choice {}, which selected the candidate", shown(id))
                    }
                },
            ),
            ("candidate", candidate.id.clone()),
            ("provider", candidate.provider.clone()),
            ("model", candidate.model.clone()),
            ("effort", candidate.effort.clone()),
            (
                "selected",
                match choice.selected_by {
                    SelectedBy::Route => "by the routes table (static)",
                    SelectedBy::ExplicitChoice => {
                        "by the explicit choice (static; the routes were not consulted)"
                    }
                    SelectedBy::Select => "by the policy's select (computed)",
                }
                .to_owned(),
            ),
            ("reason", shown(&choice.reason)),
            ("executable", choice.executable.to_text()),
            (
                "timing",
                format!("selection took {} ms", millis(choice.elapsed)),
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
            (
                "run id",
                format!(
                    "{} (proposed only: run allocates and records its own)",
                    choice.run.id()
                ),
            ),
            ("records", choice.state_dir.to_text()),
        ];
        let mut text =
            String::from("Proposal only: nothing was launched and no run was recorded.\n");
        for (label, value) in rows {
            let _ = writeln!(text, "  {label:<10} {value}");
        }
        let context = match &choice.context {
            None => vec!["none; select receives no context".to_owned()],
            Some(context) => {
                let mut lines = context.to_text();
                lines.push("the delivered value itself is in --json".to_owned());
                lines
            }
        };
        for (label, lines) in [("context", context), ("bounds", inputs.limits.to_text())] {
            for (index, line) in lines.iter().enumerate() {
                let label = if index == 0 { label } else { "" };
                let _ = writeln!(text, "  {label:<10} {line}");
            }
        }
        for (index, word) in choice.argv.iter().enumerate() {
            let label = if index == 0 { "argv" } else { "" };
            let _ = writeln!(text, "  {label:<10} [{index}] {}", word.to_text());
        }
        text
    }
}

/// A caller value as it is, unless a control character would break the row,
/// in which case it is shown escaped and quoted.
fn shown(value: &str) -> String {
    if value.chars().any(char::is_control) {
        format!("{value:?}")
    } else {
        value.to_owned()
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
