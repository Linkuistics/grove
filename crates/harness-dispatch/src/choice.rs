//! The selection both commands share: read the caller's inputs, evaluate the
//! selected policy, assemble and measure its context, select by the caller's
//! explicit choice or the kind's route, or by the policy's own `select`, then
//! expand the candidate's argv and resolve its program
//! (`docs/specs/harness-selection-and-execution.md`, *Command interface*,
//! *Policy and joint choice*, *Bounded context*).
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
use crate::context::{self, Delivered, SourceBreach};
use crate::inputs::{Inputs, PromptRequirement};
use crate::limits::{Limits, Origin};
use crate::policy::{self, Candidate, Form, Policy, SelectedBy, Selection, Validator};
use crate::program::{self, Executable};
use crate::refusal::{Diagnostics, Failure, Refusal, Stage, EXIT_REFUSED};
use crate::store::StateDir;
use crate::worker::{self, Assembled, Breach, Halt, Loaded, Outcome, WorkerIdentity};

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
    /// The context selection saw, when there was one: a loader's result, or
    /// the caller's document, measured.
    pub context: Option<Delivered>,
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
        &inputs.limits,
        inputs.context.as_ref().map(|caller| &caller.measured),
        |outcome, loaded| judge(outcome, loaded, &inputs, &source),
    )?;
    let diagnostics = evaluation.diagnostics;
    let refuse = |refusal: Refusal| Failure::with_diagnostics(refusal, diagnostics.clone());

    let Judged {
        mut policy,
        selection: Selection { index, reason, by },
        context,
    } = evaluation.decided;
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
        context,
        elapsed: evaluation.elapsed,
        worker: evaluation.worker,
        diagnostics,
    })
}

/// What the judge decided: the valid policy, its selection, and the context
/// it was made with.
struct Judged {
    policy: Policy,
    selection: Selection,
    context: Option<Delivered>,
}

/// Decide on what the worker reported. Validate the policy, then run the
/// checks that need no more of its code: an explicit choice the catalog lacks,
/// and for a routes policy its whole selection. Only then ask the worker for
/// the context, when the policy has a loader or the caller gave one, and
/// validate and measure it. A `select` policy is then asked to select, with
/// that context, and what it produced is judged. Nothing is asked of the
/// worker once a refusal is known.
fn judge(
    outcome: Outcome,
    mut loaded: Loaded<'_>,
    inputs: &Inputs,
    source: &str,
) -> Result<Judged, Halt> {
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
        Outcome::Breach(breach) => {
            return Err(
                policy::message_too_large(Stage::Load, &breach, source, &inputs.limits).into(),
            );
        }
    };
    let policy = Validator::new(source).validate(&snapshot)?;
    let choice = inputs.choice.as_deref();
    let routed = match &policy.form {
        Form::Routes(routes) => Some(policy::by_routes(
            &policy,
            routes,
            &inputs.kind,
            choice,
            source,
        )?),
        Form::Select => {
            if let Some(choice) = choice {
                policy::configured(&policy, choice, source)?;
            }
            None
        }
    };
    let context = if policy.loader || inputs.context.is_some() {
        let assembled = loaded.context()?;
        Some(deliver(assembled, &policy, inputs, source)?)
    } else {
        None
    };
    let selection = match routed {
        Some(selection) => selection,
        None => {
            let produced = loaded.select()?;
            policy::computed(
                &policy,
                produced,
                &inputs.kind,
                choice,
                source,
                &inputs.limits,
            )?
        }
    };
    Ok(Judged {
        policy,
        selection,
        context,
    })
}

/// Judge what the worker assembled as the context: validate and measure it,
/// or refuse for the reason it could not be assembled. Without a loader it is
/// the caller's own document, and a refusal about it names that document.
fn deliver(
    assembled: Assembled,
    policy: &Policy,
    inputs: &Inputs,
    source: &str,
) -> Result<Delivered, Refusal> {
    let limits = &inputs.limits;
    let refuse = |code: &'static str, message: String, remedy: String| {
        Refusal::new(code, Stage::Context, EXIT_REFUSED, message, remedy)
            .source(source)
            .location("policy.loadContext")
    };
    match assembled {
        Assembled::Context { context, measured } => {
            let about = match (&inputs.context, policy.loader) {
                (Some(caller), false) => caller.measured.name.as_str(),
                _ => source,
            };
            context::deliver(context, measured, policy.loader, limits, about)
        }
        Assembled::Threw { name, message } => Err(refuse(
            "context_loader_failed",
            format!("loadContext in {source} threw, or its promise rejected: {name}: {message}"),
            format!(
                "fix loadContext in {source}, or supply what it requires; a loader that fails \
                 fails the whole selection, and nothing is selected without its context"
            ),
        )),
        Assembled::SourceUnreadable {
            source: path,
            message,
        } => Err(Refusal::new(
            "context_source_unreadable",
            Stage::Context,
            EXIT_REFUSED,
            format!("loadContext in {source} failed because it could not read {path}: {message}"),
            format!(
                "supply {path}, or change loadContext in {source} so that it does not require \
                 it; a context the policy requires is never assumed"
            ),
        )
        .source(path)
        .location("policy.loadContext")),
        Assembled::Unsettled => Err(refuse(
            "context_loader_unsettled",
            format!(
                "loadContext in {source} returned a promise that never settled: it was still \
                 pending when nothing was left running that could settle it"
            ),
            "make every path through loadContext resolve or reject its promise; an await on \
             something that will never complete leaves it pending"
                .to_owned(),
        )),
        Assembled::Unserializable { name, message } => Err(refuse(
            "context_invalid",
            format!("the context loadContext returned cannot be serialized: {name}: {message}"),
            format!(
                "return plain JSON data from loadContext in {source}: objects, arrays, strings, \
                 finite numbers, booleans and null, with no cycles"
            ),
        )
        .location("context")),
        Assembled::Breach(breach) => Err(context_breach(&breach, policy, inputs, source)),
        Assembled::Unsupported { operation } => Err(policy::unsupported_operation(
            Stage::Context,
            &operation,
            "loadContext",
            source,
        )),
    }
}

/// A bound the worker saw exceeded while it assembled the context, refused by
/// name with the front's own limits.
fn context_breach(breach: &Breach, policy: &Policy, inputs: &Inputs, source: &str) -> Refusal {
    let limits: &Limits = &inputs.limits;
    let actual = breach.actual.unwrap_or(0);
    match breach.bound.as_str() {
        "source" => {
            let budget = limits.context.value;
            let (limit, from, requested) = match breach.max_bytes {
                Some(max) if max > budget => (budget, limits.context.origin, Some(max)),
                Some(max) => (max, Origin::Set("maxBytes"), None),
                None => (limits.source.value, limits.source.origin, None),
            };
            context::source_too_large(
                limits,
                &SourceBreach {
                    name: breach.source.clone().unwrap_or_else(|| source.to_owned()),
                    limit,
                    from,
                    actual: breach.actual,
                    requested,
                },
                source,
            )
        }
        "sources" => context::too_many_sources(limits, actual, source),
        "message" => policy::message_too_large(Stage::Context, breach, source, limits),
        _ => {
            let about = match (&inputs.context, policy.loader) {
                (Some(caller), false) => caller.measured.name.as_str(),
                _ => source,
            };
            context::context_too_large(limits, actual, about, policy.loader)
        }
    }
}

/// The request the worker receives: the caller's data, the explicit choice
/// and the caller's context included, the effective bounds, and never the
/// prompt, which only ever fills the candidate's `prompt` argument. The
/// policy's `loadContext` and `select` receive it; a `routes` table never
/// sees it.
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
    if let Some(context) = &inputs.context {
        request.insert("context".into(), context.value.clone());
    }
    if let Some(choice) = &inputs.choice {
        request.insert("explicitChoice".into(), choice.clone().into());
    }
    request.insert("limits".into(), inputs.limits.to_request());
    Value::Object(request)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::context::{CallerContext, Measured};
    use crate::inputs::{Prompt, PromptSource};

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
            context: Some(CallerContext {
                value: serde_json::json!({ "schemaVersion": 1, "facts": {} }),
                measured: Measured {
                    name: "/work/context.json".to_owned(),
                    via: "--context".to_owned(),
                    bytes: 32,
                    sha256: "0".repeat(64),
                },
            }),
            limits: Limits::read(None, None).unwrap(),
        };
        let request = request(&inputs);
        assert_eq!(
            request,
            serde_json::json!({
                "schemaVersion": 1, "kind": "impl", "cwd": "/work",
                "taskFile": "/work/task.md", "taskId": "T-1", "explicitChoice": "deep",
                "context": { "schemaVersion": 1, "facts": {} },
                "limits": {
                    "selectionMs": 30_000, "contextBytes": 262_144, "sourceBytes": 65_536,
                    "sources": 256, "messageBytes": 1_048_576, "diagnosticsBytes": 262_144,
                },
            })
        );
        let absent = Inputs {
            task_file: None,
            task_id: None,
            choice: None,
            context: None,
            ..inputs
        };
        let request = super::request(&absent);
        assert_eq!(
            request.as_object().unwrap().keys().collect::<Vec<_>>(),
            ["cwd", "kind", "limits", "schemaVersion"]
        );
    }
}
