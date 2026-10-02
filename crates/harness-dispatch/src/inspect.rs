//! `inspect`: evaluate the selected policy and report its choice, launching
//! nothing (`docs/specs/harness-selection-and-execution.md`, *Command interface*).
//!
//! Inspection is a proposal. It is not a launch reservation, and evaluating
//! trusted TypeScript is not promised to be free of side effects. It makes the
//! same choice `run` would, program resolution included, and reports it in
//! both forms: human text, and one version-2 JSON object on stdout. The
//! selected command is its `command`: the program and arguments `select`
//! returned and the file the program resolved to, which a caller that must
//! launch the command itself takes from there. Without a prompt, `select`
//! receives a fixed marker in its place, and the report says so.
//!
//! It records nothing. It reads the record store only to answer the policy's
//! run lookups, as `run` does, so that both make the same choice; it never
//! creates a store or writes a run. It reports where `run` would record, and
//! no run ID.

use std::fmt::Write as _;
use std::path::Path;
use std::time::Duration;

use serde_json::{json, Map, Value};

use crate::authority::{Authority, PERSONAL_DEFAULT};
use crate::choice::{self, Choice};
use crate::cli::InspectArgs;
use crate::context::Delivered;
use crate::inputs::{PromptRequirement, PromptSource, PROMPT_NOT_SUPPLIED};
use crate::policy::SCHEMA_VERSION;
use crate::record;
use crate::refusal::{Diagnostics, Failure};
use crate::worker::WorkerIdentity;

pub struct Report(Choice);

pub fn inspect(args: &InspectArgs) -> Result<Report, Failure> {
    choice::choose(&args.selection, PromptRequirement::Optional)?
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
            None => json!({ "supplied": false, "marker": PROMPT_NOT_SUPPLIED }),
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
        let command = &choice.command;
        let context = choice.context.as_ref();
        json!({
            "schemaVersion": SCHEMA_VERSION,
            "evidence": "proposal",
            "stateDir": choice.state_dir.to_json(),
            "kind": choice.inputs.kind,
            "taskFile": choice.inputs.task_file,
            "taskId": choice.inputs.task_id,
            "params": choice.inputs.params,
            "prompt": prompt,
            "reviewedArtifact": context.and_then(|context| context.reviewed_artifact()),
            "creator": context.and_then(Delivered::creator),
            "context": context.map(|context| context.to_json(true)),
            "policy": policy,
            "policyEnv": choice.inputs.grants.to_json(),
            "selection": {
                "provider": command.provider,
                "model": command.model,
                "effort": command.effort,
                "reason": command.reason,
            },
            // What a caller that launches the command itself executes:
            // `executable` with `args`, resolving nothing again.
            "command": {
                "program": command.program,
                "args": command.args,
                "executable": choice.executable.path,
            },
            "bounds": choice.inputs.limits.to_json(),
            "timing": { "selectionMs": millis(choice.elapsed) },
            "worker": worker_json(&choice.worker),
            "adapter": choice.adapter.as_ref().map(crate::worker::Adapter::to_json),
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
            None => format!("not supplied; select received the marker {PROMPT_NOT_SUPPLIED}"),
            Some(prompt) => match &prompt.source {
                PromptSource::Argument => format!("{} bytes from --prompt", prompt.text.len()),
                PromptSource::File(path) => format!(
                    "{} bytes from --prompt-file {}",
                    prompt.text.len(),
                    shown(&path.to_string_lossy())
                ),
            },
        };
        let command = &choice.command;
        let worker = &choice.worker;
        let rows = [
            ("policy", choice.entry.display()),
            ("authority", authority),
            ("version", choice.version.clone()),
            ("sha256", choice.entry.sha256.clone()),
            ("policy env", inputs.grants.to_text()),
            ("kind", shown(&inputs.kind)),
            (
                "task file",
                inputs.task_file.as_deref().map_or("none".to_owned(), shown),
            ),
            (
                "task id",
                inputs.task_id.as_deref().map_or("none".to_owned(), shown),
            ),
            (
                "params",
                record::params_text(
                    inputs
                        .params
                        .iter()
                        .map(|(name, value)| (name.as_str(), value.as_str())),
                ),
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
            ("provider", shown(&command.provider)),
            ("model", shown(&command.model)),
            ("effort", shown(&command.effort)),
            ("reason", shown(&command.reason)),
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
                "adapter",
                choice.adapter.as_ref().map_or_else(
                    || "none; the policy imported no adapter".to_owned(),
                    |adapter| format!("{}, which the policy imported", adapter.to_text()),
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
        // Quoted and escaped, so that spaces, quotes and newlines stay
        // visible. The first word is the program as `select` returned it.
        let argv = std::iter::once(&command.program).chain(&command.args);
        for (index, word) in argv.enumerate() {
            let label = if index == 0 { "argv" } else { "" };
            let _ = writeln!(text, "  {label:<10} [{index}] {word:?}");
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
