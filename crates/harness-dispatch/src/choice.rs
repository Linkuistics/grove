//! The selection both commands share: read the caller's inputs, evaluate the
//! selected policy, route the kind, expand the candidate's argv and resolve its
//! program (`docs/specs/harness-selection-and-execution.md`, *Command
//! interface*).
//!
//! `inspect` reports the resulting choice and `run` execs it, so the two cannot
//! disagree about what a selection means. Every step refuses rather than
//! substitutes: nothing here ever picks a candidate the policy did not.

use std::time::Duration;

use serde_json::{Map, Value};

use crate::argv::{self, Word};
use crate::authority::{self, PolicyEntry};
use crate::cli::SelectionArgs;
use crate::inputs::{Inputs, PromptRequirement};
use crate::policy::{self, Candidate, Validator};
use crate::program::{self, Executable};
use crate::refusal::{Diagnostics, Failure, Refusal, Stage, EXIT_REFUSED};
use crate::worker::{self, Outcome, WorkerIdentity};

#[derive(Debug)]
pub struct Choice {
    pub inputs: Inputs,
    pub entry: PolicyEntry,
    pub version: String,
    pub candidate: Candidate,
    /// The candidate's place in the catalog, for refusal locations.
    pub index: usize,
    pub reason: String,
    pub argv: Vec<Word>,
    pub executable: Executable,
    pub elapsed: Duration,
    pub worker: WorkerIdentity,
    pub diagnostics: Diagnostics,
}

pub fn choose(args: &SelectionArgs, requirement: PromptRequirement) -> Result<Choice, Failure> {
    let inputs = Inputs::read(args, requirement)?;
    let home = std::env::var_os("HOME");
    let entry = authority::resolve(args.config.as_deref(), &inputs.cwd, home.as_deref())?;
    let source = entry.display();

    let worker_path = worker::locate()?;
    let evaluation = worker::evaluate(&worker_path, &entry.path, request(&inputs))?;
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
    let policy::Selection { index, reason } =
        policy::route(&policy, &inputs.kind, &source).map_err(refuse)?;
    let candidate = policy.catalog.swap_remove(index);
    let argv = argv::expand(&candidate, index, &inputs, &source).map_err(refuse)?;
    let path = std::env::var_os("PATH");
    let executable = program::resolve(&candidate, index, &source, &inputs.cwd, path.as_deref())
        .map_err(refuse)?;

    Ok(Choice {
        inputs,
        entry,
        version: policy.version,
        candidate,
        index,
        reason,
        argv,
        executable,
        elapsed: evaluation.elapsed,
        worker: evaluation.worker,
        diagnostics,
    })
}

/// The request the worker receives: the caller's data, and never the prompt,
/// which only ever fills the candidate's `prompt` argument. The worker hands
/// it to a `select` callback once computed selection lands; a `routes` policy
/// never sees it.
fn request(inputs: &Inputs) -> Value {
    let mut request = Map::new();
    request.insert("schemaVersion".into(), 1.into());
    request.insert("kind".into(), inputs.kind.clone().into());
    request.insert("cwd".into(), inputs.cwd.to_string_lossy().into());
    if let Some(task_file) = &inputs.task_file {
        request.insert("taskFile".into(), task_file.clone().into());
    }
    if let Some(task_id) = &inputs.task_id {
        request.insert("taskId".into(), task_id.clone().into());
    }
    Value::Object(request)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::inputs::{Prompt, PromptSource};

    #[test]
    fn the_request_carries_caller_data_and_never_the_prompt() {
        let inputs = Inputs {
            kind: "impl".to_owned(),
            cwd: PathBuf::from("/work"),
            task_file: Some("/work/task.md".to_owned()),
            task_id: Some("T-1".to_owned()),
            prompt: Some(Prompt {
                text: "the prompt text".to_owned(),
                source: PromptSource::File(PathBuf::from("/work/prompt.md")),
            }),
        };
        let request = request(&inputs);
        assert_eq!(
            request,
            serde_json::json!({
                "schemaVersion": 1, "kind": "impl", "cwd": "/work",
                "taskFile": "/work/task.md", "taskId": "T-1",
            })
        );
        let absent = Inputs {
            task_file: None,
            task_id: None,
            ..inputs
        };
        assert_eq!(
            super::request(&absent),
            serde_json::json!({ "schemaVersion": 1, "kind": "impl", "cwd": "/work" })
        );
    }
}
