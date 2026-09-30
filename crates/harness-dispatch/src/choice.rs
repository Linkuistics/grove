//! The selection both commands share: read the caller's inputs, evaluate the
//! selected policy, select by the caller's explicit choice or the kind's route,
//! or by the policy's own `select`, then expand the candidate's argv and
//! resolve its program (`docs/specs/harness-selection-and-execution.md`,
//! *Command interface*, *Policy and joint choice*).
//!
//! `inspect` reports the resulting choice and `run` execs it, so the two cannot
//! disagree about what a selection means. Every step refuses rather than
//! substitutes: nothing here ever picks a candidate the policy did not.
//!
//! The record's state directory is placed here too, before the worker starts,
//! so a HOME that cannot place it refuses in `inspect` as it would in `run`,
//! and before anything is evaluated. The store itself is `run`'s alone, and is
//! opened only after this selection has finished.

use std::time::Duration;

use serde_json::{Map, Value};

use crate::argv::{self, RunSlot, Word};
use crate::authority::{self, PolicyEntry};
use crate::cli::SelectionArgs;
use crate::inputs::{Inputs, PromptRequirement};
use crate::policy::{self, Candidate, Form, Policy, SelectedBy, Selection, Validator};
use crate::program::{self, Executable};
use crate::refusal::{Diagnostics, Failure, Refusal, Stage, EXIT_REFUSED};
use crate::store::StateDir;
use crate::worker::{self, Halt, Loaded, Outcome, WorkerIdentity};

#[derive(Debug)]
pub struct Choice {
    pub inputs: Inputs,
    pub entry: PolicyEntry,
    pub state_dir: StateDir,
    /// The identity a `runId` slot expanded to.
    pub run: RunSlot,
    pub version: String,
    pub candidate: Candidate,
    /// The candidate's place in the catalog, for refusal locations.
    pub index: usize,
    pub reason: String,
    /// The routes table, the explicit choice, or the policy's `select`.
    pub selected_by: SelectedBy,
    pub argv: Vec<Word>,
    pub executable: Executable,
    pub elapsed: Duration,
    pub worker: WorkerIdentity,
    pub diagnostics: Diagnostics,
}

pub fn choose(
    args: &SelectionArgs,
    requirement: PromptRequirement,
    run: RunSlot,
) -> Result<Choice, Failure> {
    let inputs = Inputs::read(args, requirement)?;
    let home = std::env::var_os("HOME");
    let entry = authority::resolve(args.config.as_deref(), &inputs.cwd, home.as_deref())?;
    let state_dir = StateDir::resolve(args.state_dir.as_deref(), &inputs.cwd, home.as_deref())?;
    let source = entry.display();

    let worker_path = worker::locate()?;
    let evaluation = worker::evaluate(
        &worker_path,
        &entry.path,
        request(&inputs),
        inputs.selection,
        |outcome, loaded| judge(outcome, loaded, &inputs, &source),
    )?;
    let diagnostics = evaluation.diagnostics;
    let refuse = |refusal: Refusal| Failure::with_diagnostics(refusal, diagnostics.clone());

    let (mut policy, Selection { index, reason, by }) = evaluation.decided;
    let candidate = policy.catalog.swap_remove(index);
    let argv = argv::expand(&candidate, index, &inputs, &run, &source).map_err(refuse)?;
    let path = std::env::var_os("PATH");
    let executable = program::resolve(&candidate, index, &source, &inputs.cwd, path.as_deref())
        .map_err(refuse)?;

    Ok(Choice {
        inputs,
        entry,
        state_dir,
        run,
        version: policy.version,
        candidate,
        index,
        reason,
        selected_by: by,
        argv,
        executable,
        elapsed: evaluation.elapsed,
        worker: evaluation.worker,
        diagnostics,
    })
}

/// Decide on what the worker reported: validate the policy, then select
/// through its routes, or, for `select`, check any explicit choice against the
/// catalog before asking the worker to run it, and judge what it produced.
/// Nothing is asked of the worker once a refusal is known.
fn judge(
    outcome: Outcome,
    loaded: Loaded<'_>,
    inputs: &Inputs,
    source: &str,
) -> Result<(Policy, Selection), Halt> {
    let import_failed = |detail: String| {
        Refusal::new(
            "policy_import_failed",
            Stage::Load,
            EXIT_REFUSED,
            format!("the policy entry {source} failed to load: {detail}"),
            format!(
                "fix {source} or the module it imports; relative imports resolve from the \
                 importing file, bare ones through node_modules beside it, and nothing is \
                 installed automatically"
            ),
        )
        .source(source)
    };
    let snapshot = match outcome {
        Outcome::Policy(snapshot) => snapshot,
        Outcome::LoadFailed { name, message } => {
            return Err(import_failed(format!("{name}: {message}")).into());
        }
        Outcome::LoadUnsettled => {
            return Err(import_failed(
                "an await in it, or in a module it imports, never settled, and nothing was left \
                 running that could settle it"
                    .to_owned(),
            )
            .into());
        }
        Outcome::Invalid { location, message } => {
            return Err(Refusal::new(
                "policy_invalid",
                Stage::Validation,
                EXIT_REFUSED,
                message,
                format!("export a plain object as `export const policy = {{ ... }}` from {source}"),
            )
            .source(source)
            .location(location)
            .into());
        }
    };
    let policy = Validator::new(source).validate(&snapshot)?;
    let choice = inputs.choice.as_deref();
    let selection = match &policy.form {
        Form::Routes(routes) => policy::by_routes(&policy, routes, &inputs.kind, choice, source)?,
        Form::Select => {
            if let Some(choice) = choice {
                policy::configured(&policy, choice, source)?;
            }
            let produced = loaded.select()?;
            policy::computed(&policy, produced, &inputs.kind, choice, source)?
        }
    };
    Ok((policy, selection))
}

/// The request the worker receives: the caller's data, the explicit choice
/// included, the effective bounds, and never the prompt, which only ever fills
/// the candidate's `prompt` argument. A `select` callback receives it; a
/// `routes` policy never sees it.
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
    if let Some(choice) = &inputs.choice {
        request.insert("explicitChoice".into(), choice.clone().into());
    }
    request.insert(
        "limits".into(),
        serde_json::json!({ "selectionMs": inputs.selection.value }),
    );
    Value::Object(request)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::inputs::{Bound, Prompt, PromptSource};

    #[test]
    fn the_request_carries_caller_data_and_never_the_prompt() {
        let inputs = Inputs {
            kind: "impl".to_owned(),
            cwd: PathBuf::from("/work"),
            task_file: Some("/work/task.md".to_owned()),
            task_id: Some("T-1".to_owned()),
            choice: Some("deep".to_owned()),
            prompt: Some(Prompt {
                text: "the prompt text".to_owned(),
                source: PromptSource::File(PathBuf::from("/work/prompt.md")),
            }),
            selection: Bound {
                name: "selection",
                unit: "ms",
                value: 30_000,
                flag: None,
            },
        };
        let request = request(&inputs);
        assert_eq!(
            request,
            serde_json::json!({
                "schemaVersion": 1, "kind": "impl", "cwd": "/work",
                "taskFile": "/work/task.md", "taskId": "T-1", "explicitChoice": "deep",
                "limits": { "selectionMs": 30_000 },
            })
        );
        let absent = Inputs {
            task_file: None,
            task_id: None,
            choice: None,
            ..inputs
        };
        assert_eq!(
            super::request(&absent),
            serde_json::json!({
                "schemaVersion": 1, "kind": "impl", "cwd": "/work",
                "limits": { "selectionMs": 30_000 },
            })
        );
    }
}
