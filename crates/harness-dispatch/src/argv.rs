//! Expand the selected candidate's arguments into argv
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and joint choice*).
//!
//! Each argument is a literal or a slot, and a slot fills one whole argument
//! with one value. There is no shell, no splitting, no interpolation inside a
//! literal and no second reading of the prompt. An optional input the caller
//! did not supply satisfies no slot: the invocation refuses, naming the flag
//! that would.
//!
//! The `runId` slot is the run's own identity. Under `run` it is the ID the
//! handoff record commits; under `inspect` it is a proposed ID, marked as one,
//! because inspection records nothing and a later `run` allocates afresh.

use serde_json::{json, Value};

use crate::inputs::Inputs;
use crate::policy::{Argument, Candidate, Slot};
use crate::refusal::{Refusal, Stage, EXIT_REFUSED};
use crate::run_id::RunId;

/// One word of the expanded argv.
#[derive(Debug)]
pub enum Word {
    Text(String),
    /// Where the prompt goes, in an inspection that was given none.
    PromptPlaceholder,
    /// The `runId` slot in an inspection: an ID no run holds.
    ProposedRunId(String),
}

/// The run identity a `runId` slot expands to.
#[derive(Clone, Debug)]
pub enum RunSlot {
    /// Allocated for this `run`, and committed before exec.
    Allocated(RunId),
    /// Shown by `inspect`, never recorded and never reused.
    Proposed(RunId),
}

impl RunSlot {
    pub fn id(&self) -> &RunId {
        match self {
            RunSlot::Allocated(id) | RunSlot::Proposed(id) => id,
        }
    }
}

impl Word {
    /// The word as `run` passes it. Only a literal or an allocated value has
    /// one; `run` never produces the marked kinds.
    pub fn text(&self) -> Option<&str> {
        match self {
            Word::Text(text) => Some(text),
            Word::PromptPlaceholder | Word::ProposedRunId(_) => None,
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            Word::Text(text) => text.as_str().into(),
            Word::PromptPlaceholder => json!({ "placeholder": "prompt" }),
            Word::ProposedRunId(id) => json!({ "proposedRunId": id }),
        }
    }

    /// Quoted and escaped, so that spaces, quotes and newlines stay visible;
    /// the marked words are the unquoted ones.
    pub fn to_text(&self) -> String {
        match self {
            Word::Text(text) => format!("{text:?}"),
            Word::PromptPlaceholder => {
                "<prompt placeholder: no --prompt or --prompt-file given>".to_owned()
            }
            Word::ProposedRunId(id) => {
                format!("<proposed run ID {id}: run allocates and records its own>")
            }
        }
    }
}

/// argv for `candidate`, the catalog entry at `index` of the policy at
/// `source`. Its first word is the program exactly as configured, as a shell
/// passes a command as typed.
pub fn expand(
    candidate: &Candidate,
    index: usize,
    inputs: &Inputs,
    run: &RunSlot,
    source: &str,
) -> Result<Vec<Word>, Refusal> {
    let mut words = vec![Word::Text(candidate.program.clone())];
    for (position, argument) in candidate.args.iter().enumerate() {
        let slot = match argument {
            Argument::Literal(literal) => {
                words.push(Word::Text(literal.clone()));
                continue;
            }
            Argument::Slot(slot) => *slot,
        };
        let missing = |flag: &str, name: &str| {
            Refusal::new(
                "missing_input",
                Stage::Expansion,
                EXIT_REFUSED,
                format!(
                    "candidate {:?} passes the `{name}` slot, and no {flag} was given to fill it",
                    candidate.id
                ),
                format!(
                    "pass {flag}, or remove the `{name}` slot from candidate {:?} in {source}; \
                     an absent input never fills a slot",
                    candidate.id
                ),
            )
            .input(flag)
            .source(source)
            .location(format!("policy.catalog[{index}].args[{position}]"))
        };
        words.push(match slot {
            Slot::Prompt => inputs
                .prompt
                .as_ref()
                .map_or(Word::PromptPlaceholder, |prompt| {
                    Word::Text(prompt.text.clone())
                }),
            Slot::Kind => Word::Text(inputs.kind.clone()),
            Slot::TaskFile => Word::Text(
                inputs
                    .task_file
                    .clone()
                    .ok_or_else(|| missing("--task-file", "taskFile"))?,
            ),
            Slot::TaskId => Word::Text(
                inputs
                    .task_id
                    .clone()
                    .ok_or_else(|| missing("--task-id", "taskId"))?,
            ),
            Slot::Model => Word::Text(candidate.model.clone()),
            Slot::Effort => Word::Text(candidate.effort.clone()),
            Slot::RunId => match run {
                RunSlot::Allocated(id) => Word::Text(id.to_string()),
                RunSlot::Proposed(id) => Word::ProposedRunId(id.to_string()),
            },
        });
    }
    Ok(words)
}
