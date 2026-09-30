//! Expand the selected candidate's arguments into argv
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and joint choice*).
//!
//! Each argument is a literal or a slot, and a slot fills one whole argument
//! with one value. There is no shell, no splitting, no interpolation inside a
//! literal and no second reading of the prompt. An optional input the caller
//! did not supply satisfies no slot: the invocation refuses, naming the flag
//! that would.

use serde_json::{json, Value};

use crate::inputs::Inputs;
use crate::policy::{Argument, Candidate, Slot};
use crate::refusal::{Refusal, Stage, EXIT_REFUSED};

/// One word of the expanded argv.
#[derive(Debug)]
pub enum Word {
    Text(String),
    /// Where the prompt goes, in an inspection that was given none.
    PromptPlaceholder,
}

impl Word {
    pub fn text(&self) -> Option<&str> {
        match self {
            Word::Text(text) => Some(text),
            Word::PromptPlaceholder => None,
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            Word::Text(text) => text.as_str().into(),
            Word::PromptPlaceholder => json!({ "placeholder": "prompt" }),
        }
    }

    /// Quoted and escaped, so that spaces, quotes and newlines stay visible;
    /// the placeholder is the one unquoted word.
    pub fn to_text(&self) -> String {
        match self {
            Word::Text(text) => format!("{text:?}"),
            Word::PromptPlaceholder => {
                "<prompt placeholder: no --prompt or --prompt-file given>".to_owned()
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
        });
    }
    Ok(words)
}
