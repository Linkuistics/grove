//! The selection both commands share: read the caller's inputs, evaluate the
//! selected policy, assemble and measure its context, call its `select`, then
//! validate the command it returned and resolve that command's program
//! (`docs/specs/harness-selection-and-execution.md`, *Command interface*,
//! *Policy and the selected command*, *Bounded context*).
//!
//! `inspect` reports the resulting choice and `run` execs it, so the two cannot
//! disagree about what a selection means. Every step refuses rather than
//! substitutes: nothing here ever runs a command the policy did not return.
//!
//! The record's state directory is placed here too, before the worker starts,
//! so a HOME that cannot place it refuses in `inspect` as it would in `run`,
//! and before anything is evaluated. During selection the store is only ever
//! read, to answer a policy's run lookups, so both commands give the same
//! answers; only `run` writes to it, once this selection has finished.
//!
//! INT, TERM and HUP cancel selection from just before the worker starts
//! (`cancellation`). A signal received from then until the program is
//! resolved decides the outcome, whatever else selection came to. The choice
//! is returned with the handlers still installed: `inspect` ends cancellation
//! at once, and `run` at its linearization point, after the record commit.

use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::{Map, Value};

use crate::authority::{self, PolicyEntry};
use crate::cancellation::{self, Handlers};
use crate::cli::SelectionArgs;
use crate::context::{self, Delivered, SourceBreach};
use crate::inputs::{Inputs, PromptRequirement, PROMPT_NOT_SUPPLIED};
use crate::limits::{Limits, Origin};
use crate::policy::{self, Command, Policy, Validator, SCHEMA_VERSION};
use crate::program::{self, Executable};
use crate::record;
use crate::refusal::{Diagnostics, Failure, Refusal, Stage, EXIT_REFUSED, EXIT_WORKER};
use crate::run_id::RunId;
use crate::store::{self, StateDir};
use crate::worker::{self, Adapter, Assembled, Breach, Halt, Loaded, Outcome, WorkerIdentity};

#[derive(Debug)]
pub struct Choice {
    pub inputs: Inputs,
    pub entry: PolicyEntry,
    pub state_dir: StateDir,
    pub version: String,
    /// The command `select` returned, exactly.
    pub command: Command,
    /// The file its program resolved to.
    pub executable: Executable,
    /// The context selection saw, when there was one: a loader's result, or
    /// the caller's document, measured.
    pub context: Option<Delivered>,
    pub elapsed: Duration,
    pub worker: WorkerIdentity,
    /// The Grove adapter the policy imported, as the worker reported it last.
    pub adapter: Option<Adapter>,
    pub diagnostics: Diagnostics,
}

pub fn choose(args: &SelectionArgs, requirement: PromptRequirement) -> Result<Selected, Failure> {
    let inputs = Inputs::read(args, requirement)?;
    let home = std::env::var_os("HOME");
    let entry = authority::resolve(args.config.as_deref(), &inputs.cwd, home.as_deref())?;
    let state_dir = StateDir::resolve(args.state_dir.as_deref(), &inputs.cwd, home.as_deref())?;
    let source = entry.display();
    let worker_path = worker::locate()?;

    let handlers = Handlers::install().map_err(|error| {
        Refusal::new(
            "cancellation_unavailable",
            Stage::Evaluation,
            EXIT_WORKER,
            format!("cannot catch INT, TERM and HUP, which cancel selection: {error}"),
            "run harness-dispatch where it can install signal handlers; nothing was evaluated",
        )
        .source(&source)
    })?;
    match evaluate_and_resolve(inputs, entry, state_dir, &worker_path) {
        // The program is resolved, and the handlers stay: the caller decides
        // when a signal stops cancelling.
        Ok(choice) => match cancellation::received() {
            None => Ok(Selected {
                choice,
                handlers,
                source,
            }),
            Some(signal) => Err(Failure::with_diagnostics(
                cancellation::refusal(signal, &source),
                choice.diagnostics,
            )),
        },
        Err(failure) => {
            drop(handlers);
            Err(overruled(failure, &source))
        }
    }
}

/// A choice made while the evaluation's handlers are still installed: a
/// handled signal from here on is noted, and nothing yet acts on it.
pub struct Selected {
    pub choice: Choice,
    pub handlers: Handlers,
    /// The policy entry, as a cancellation names it.
    pub source: String,
}

impl Selected {
    /// End cancellation for a command that launches nothing. The entry
    /// dispositions come back before the last look, which leaves no gap: a
    /// signal from then on takes its entry course, and one noted before is
    /// seen by the look.
    pub fn settle(self) -> Result<Choice, Failure> {
        let Selected {
            choice,
            handlers,
            source,
        } = self;
        drop(handlers);
        match cancellation::received() {
            None => Ok(choice),
            Some(signal) => Err(Failure::with_diagnostics(
                cancellation::refusal(signal, &source),
                choice.diagnostics,
            )),
        }
    }
}

/// A handled signal received before anything is recorded decides the outcome:
/// a refusal reached meanwhile is reported as the cancellation, with whatever
/// the policy printed.
pub fn overruled(failure: Failure, source: &str) -> Failure {
    match cancellation::received() {
        None => failure,
        Some(signal) => Failure {
            refusal: cancellation::refusal(signal, source),
            ..failure
        },
    }
}

fn evaluate_and_resolve(
    inputs: Inputs,
    entry: PolicyEntry,
    state_dir: StateDir,
    worker_path: &Path,
) -> Result<Choice, Failure> {
    let source = entry.display();
    let evaluation = worker::evaluate(
        worker_path,
        &entry.path,
        request(&inputs),
        &inputs.limits,
        inputs.context.as_ref().map(|caller| &caller.measured),
        inputs.grants.worker_environment(),
        |outcome, loaded| judge(outcome, loaded, &inputs, &source, &state_dir),
    )?;
    let diagnostics = evaluation.diagnostics;
    let refuse = |refusal: Refusal| Failure::with_diagnostics(refusal, diagnostics.clone());

    let Judged {
        policy,
        command,
        context,
        adapter,
    } = evaluation.decided;
    // The command is validated.
    cancellation::check(&source).map_err(refuse)?;
    let path = std::env::var_os("PATH");
    let executable = program::resolve(&command.program, &source, &inputs.cwd, path.as_deref())
        .map_err(refuse)?;

    Ok(Choice {
        inputs,
        entry,
        state_dir,
        version: policy.version,
        command,
        executable,
        context,
        elapsed: evaluation.elapsed,
        worker: evaluation.worker,
        adapter,
        diagnostics,
    })
}

/// What the judge decided: the valid policy, the command it selected, the
/// context that was made with, and the adapter the policy had imported by the
/// end.
struct Judged {
    policy: Policy,
    command: Command,
    context: Option<Delivered>,
    adapter: Option<Adapter>,
}

/// Decide on what the worker reported. Validate the policy, and only then ask
/// the worker for the context, when the policy has a loader or the caller gave
/// one, answering its run lookups from the store in `state_dir`, and validate
/// and measure it. The policy is then asked to select, with that context, and
/// what it produced is judged. Nothing is asked of the worker once a refusal
/// is known.
fn judge(
    outcome: Outcome,
    mut loaded: Loaded<'_>,
    inputs: &Inputs,
    source: &str,
    state_dir: &StateDir,
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
    let context = if policy.loader || inputs.context.is_some() {
        // A lookup waits for a writer's lock at most the fixed lock wait, and
        // never past the selection's deadline: its wait is part of the
        // selection bound, unlike the commit's, which follows it.
        let mut lookup = |run_id: &RunId, deadline: Instant| {
            let left = deadline.saturating_duration_since(Instant::now());
            record::lookup(state_dir, run_id, left.min(store::LOCK_WAIT))
        };
        let assembled = loaded.context(&mut lookup)?;
        Some(deliver(assembled, &policy, inputs, source)?)
    } else {
        None
    };
    let command = policy::selected(loaded.select()?, &inputs.kind, source, &inputs.limits)?;
    Ok(Judged {
        policy,
        command,
        context,
        adapter: loaded.adapter().cloned(),
    })
}

/// Judge what the worker assembled as the context: validate and measure it,
/// or refuse for the reason it could not be assembled, or with the refusal the
/// loader returned in its place. Without a loader it is the caller's own
/// document, and a refusal about it names that document.
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
        Assembled::Context {
            context,
            measured,
            runs,
        } => {
            // A version-1 context has no `status`, so a loader's result with
            // one is its refusal, or neither.
            if let Some(fields) = context
                .as_object()
                .filter(|fields| policy.loader && fields.contains_key("status"))
            {
                return Err(policy::loader_refused(fields, &inputs.kind, source));
            }
            let about = match (&inputs.context, policy.loader) {
                (Some(caller), false) => caller.measured.name.as_str(),
                _ => source,
            };
            context::deliver(context, measured, runs, policy.loader, limits, about)
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

/// The request the policy's `loadContext` and `select` receive: the caller's
/// data, the prompt byte for byte, every parameter by name and the caller's
/// context included, and the effective bounds. An inspection given no prompt
/// sends the marker in its place. It carries no run identity.
fn request(inputs: &Inputs) -> Value {
    let mut request = Map::new();
    request.insert("schemaVersion".into(), SCHEMA_VERSION.into());
    request.insert("kind".into(), inputs.kind.clone().into());
    let prompt = inputs
        .prompt
        .as_ref()
        .map_or(PROMPT_NOT_SUPPLIED, |prompt| prompt.text.as_str());
    request.insert("prompt".into(), prompt.into());
    request.insert("cwd".into(), inputs.cwd.to_string_lossy().into());
    let params: Map<String, Value> = inputs
        .params
        .iter()
        .map(|(name, value)| (name.clone(), value.clone().into()))
        .collect();
    request.insert("params".into(), params.into());
    if let Some(task_file) = &inputs.task_file {
        request.insert("taskFile".into(), task_file.clone().into());
    }
    if let Some(task_id) = &inputs.task_id {
        request.insert("taskId".into(), task_id.clone().into());
    }
    if let Some(context) = &inputs.context {
        request.insert("context".into(), context.value.clone());
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
    fn the_request_carries_the_prompt_and_every_parameter_beside_the_caller_data() {
        let inputs = Inputs {
            kind: "impl".to_owned(),
            cwd: PathBuf::from("/work"),
            task_file: Some("/work/task.md".to_owned()),
            task_id: Some("T-1".to_owned()),
            params: [("repo", "/work"), ("session_name", "a b\n")]
                .map(|(name, value)| (name.to_owned(), value.to_owned()))
                .into(),
            prompt: Some(Prompt {
                text: "the prompt text\n".to_owned(),
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
            grants: crate::environment::Grants::default(),
        };
        let request = request(&inputs);
        assert_eq!(
            request,
            serde_json::json!({
                "schemaVersion": 2, "kind": "impl", "prompt": "the prompt text\n", "cwd": "/work",
                "params": { "repo": "/work", "session_name": "a b\n" },
                "taskFile": "/work/task.md", "taskId": "T-1",
                "context": { "schemaVersion": 1, "facts": {} },
                "limits": {
                    "selectionMs": 30_000, "contextBytes": 262_144, "sourceBytes": 65_536,
                    "sources": 256, "messageBytes": 1_048_576, "diagnosticsBytes": 262_144,
                },
            })
        );
        // Absent inputs stay absent, the parameters are an empty object, and
        // an inspection given no prompt sends the marker.
        let absent = Inputs {
            task_file: None,
            task_id: None,
            params: Default::default(),
            prompt: None,
            context: None,
            ..inputs
        };
        let request = super::request(&absent);
        assert_eq!(
            request.as_object().unwrap().keys().collect::<Vec<_>>(),
            ["cwd", "kind", "limits", "params", "prompt", "schemaVersion"]
        );
        assert_eq!(request["params"], serde_json::json!({}));
        assert_eq!(request["prompt"], PROMPT_NOT_SUPPLIED);
    }
}
