//! Structured refusals: every failure before handoff names a stable code, the
//! stage it happened in, the input or source involved, and a remedy
//! (`docs/specs/harness-selection-and-execution.md`, *Diagnostics and exits*).
//!
//! A refusal never launches anything and never substitutes another candidate.
//! Text mode renders it for a person on stderr; `--json` renders it as one JSON
//! object on stderr and prints nothing on stdout, so a parser never sees a
//! partial result. A refused `run` also names the equivalent `inspect`
//! invocation, so that an owner can reproduce an unattended selection without
//! reconstructing its inputs.

use std::fmt::Write as _;

use serde_json::{json, Map, Value};

use crate::inputs::Bound;

/// Exit results before exec.
pub const EXIT_MALFORMED: u8 = 2;
pub const EXIT_REFUSED: u8 = 3;
/// The required record could not be written, or a record could not be read.
pub const EXIT_RECORD: u8 = 4;
pub const EXIT_WORKER: u8 = 5;
pub const EXIT_TIMEOUT: u8 = 124;
pub const EXIT_UNEXECUTABLE: u8 = 126;
pub const EXIT_NOT_FOUND: u8 = 127;

/// Where in the invocation a refusal arose. The names are part of the JSON
/// contract, so they are stable strings rather than `Debug` output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// The command line itself.
    Cli,
    /// Choosing and opening the policy entry.
    Authority,
    /// Finding, starting and verifying the compiled worker, and its protocol.
    Worker,
    /// The worker's evaluation as a whole, from its start to its result. A
    /// timeout names this stage, because the front cannot see which part of
    /// the evaluation was running when the bound ran out.
    Evaluation,
    /// Importing the policy entry.
    Load,
    /// The policy's exported shape.
    Validation,
    /// Choosing a candidate from a valid policy.
    Selection,
    /// Filling the selected candidate's argument slots.
    Expansion,
    /// Finding the selected candidate's program.
    Resolution,
    /// Placing, opening, committing to or reading the run record store.
    Record,
    /// Replacing this process with the harness.
    Exec,
}

impl Stage {
    pub fn as_str(self) -> &'static str {
        match self {
            Stage::Cli => "cli",
            Stage::Authority => "authority",
            Stage::Worker => "worker",
            Stage::Evaluation => "evaluation",
            Stage::Load => "load",
            Stage::Validation => "validation",
            Stage::Selection => "selection",
            Stage::Expansion => "expansion",
            Stage::Resolution => "resolution",
            Stage::Record => "record",
            Stage::Exec => "exec",
        }
    }
}

/// A refusal, boxed so that every `Result` carrying one stays a pointer wide
/// on its success path. Its fields read through `Deref`.
#[derive(Debug)]
pub struct Refusal(Box<Details>);

#[derive(Debug)]
pub struct Details {
    pub code: &'static str,
    pub stage: Stage,
    pub message: String,
    pub remedy: String,
    pub exit: u8,
    /// The caller input involved, as the caller spelled it (`--kind impl`).
    pub input: Option<String>,
    /// The file the refusal is about, usually the resolved policy entry.
    pub source: Option<String>,
    /// Where inside `source` the problem is (`policy.catalog[1].provider`).
    pub location: Option<String>,
    /// A policy's own code for a refusal it returned, reported beside the
    /// stable `policy_refused` rather than in its place.
    pub policy_code: Option<String>,
    /// The bound whose exhaustion this refusal reports.
    pub bound: Option<Bound>,
    /// The committed run a failure after the handoff commit belongs to.
    pub run: Option<RunNote>,
}

/// A failure after the handoff commit: the run it belongs to, and whether its
/// launch-failure detail reached the store. If it did not, the run stays a
/// handoff attempt whose execution is unknown; it never becomes a success.
#[derive(Debug)]
pub struct RunNote {
    pub id: String,
    /// `None` once the detail is recorded; otherwise the refusal that stopped
    /// the append, by code and message.
    pub unrecorded: Option<(&'static str, String)>,
}

impl RunNote {
    fn to_json(&self) -> Value {
        match &self.unrecorded {
            None => json!({ "id": self.id, "launchFailure": "recorded" }),
            Some((code, message)) => json!({
                "id": self.id,
                "launchFailure": "unrecorded",
                "evidence": "handoff_attempt",
                "execution": "unknown",
                "recordError": { "code": code, "message": message },
            }),
        }
    }

    fn to_text(&self) -> String {
        match &self.unrecorded {
            None => format!("{}; the launch failure is recorded against it", self.id),
            Some((code, message)) => format!(
                "{}; recording the launch failure failed ({code}: {message}), so the run stays \
                 a handoff attempt whose execution is unknown",
                self.id
            ),
        }
    }
}

impl std::ops::Deref for Refusal {
    type Target = Details;

    fn deref(&self) -> &Details {
        &self.0
    }
}

impl Refusal {
    pub fn new(
        code: &'static str,
        stage: Stage,
        exit: u8,
        message: impl Into<String>,
        remedy: impl Into<String>,
    ) -> Self {
        Refusal(Box::new(Details {
            code,
            stage,
            message: message.into(),
            remedy: remedy.into(),
            exit,
            input: None,
            source: None,
            location: None,
            policy_code: None,
            bound: None,
            run: None,
        }))
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.0.input = Some(input.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.0.source = Some(source.into());
        self
    }

    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.0.location = Some(location.into());
        self
    }

    pub fn policy_code(mut self, code: impl Into<String>) -> Self {
        self.0.policy_code = Some(code.into());
        self
    }

    pub fn bound(mut self, bound: Bound) -> Self {
        self.0.bound = Some(bound);
        self
    }

    pub fn run(mut self, run: RunNote) -> Self {
        self.0.run = Some(run);
        self
    }
}

/// What the policy printed while the worker evaluated it. Captured from the
/// worker's own stdout and stderr, which carry no protocol, and reported
/// separately from any result.
#[derive(Clone, Debug, Default)]
pub struct Diagnostics {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Diagnostics {
    pub fn to_json(&self) -> Value {
        json!({
            "stdout": String::from_utf8_lossy(&self.stdout),
            "stderr": String::from_utf8_lossy(&self.stderr),
        })
    }

    /// Each captured line, prefixed with the stream it came from, for text mode.
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        for (stream, bytes) in [("stdout", &self.stdout), ("stderr", &self.stderr)] {
            let captured = String::from_utf8_lossy(bytes);
            if captured.is_empty() {
                continue;
            }
            for line in captured.strip_suffix('\n').unwrap_or(&captured).split('\n') {
                let _ = writeln!(text, "policy {stream}: {line}");
            }
        }
        text
    }
}

/// The `inspect` invocation equivalent to a refused `run`: the same selection
/// inputs without the prompt, and the directory they were given in, which
/// relative paths and the policy's view of the caller depend on.
#[derive(Debug)]
pub enum Invocation {
    Exact {
        /// `None` when the current directory could not be read.
        cwd: Option<String>,
        argv: Vec<String>,
    },
    /// A word or the directory is not UTF-8, so no JSON string or text
    /// command line can hold it exactly. A lossy copy would name different
    /// inputs, whose inspection could even succeed, so none is offered.
    Unavailable {
        /// The input, program or directory that cannot be written.
        what: String,
    },
}

impl Invocation {
    fn unavailable(what: &str) -> String {
        format!("{what} is not valid UTF-8, so no command line reproduces it exactly")
    }

    fn to_json(&self) -> Value {
        match self {
            Invocation::Exact { cwd, argv } => json!({ "cwd": cwd, "argv": argv }),
            Invocation::Unavailable { what } => json!({ "unavailable": Self::unavailable(what) }),
        }
    }

    /// One line a POSIX shell runs as the same invocation, in a subshell so
    /// that pasting it leaves the reader's own directory alone.
    fn to_text(&self) -> String {
        let (cwd, argv) = match self {
            Invocation::Exact { cwd, argv } => (cwd, argv),
            Invocation::Unavailable { what } => {
                return format!("unavailable: {}", Self::unavailable(what))
            }
        };
        let command: Vec<String> = argv.iter().map(|word| shell_word(word)).collect();
        let command = command.join(" ");
        match cwd {
            Some(cwd) => format!("(cd {} && {command})", shell_word(cwd)),
            None => command,
        }
    }
}

/// `word` as one POSIX shell word: unquoted when every character is plainly
/// literal, and otherwise single-quoted, with each `'` closed, escaped and
/// reopened.
fn shell_word(word: &str) -> String {
    let plain = |c: char| c.is_ascii_alphanumeric() || "_@%+=:,./-".contains(c);
    if !word.is_empty() && word.chars().all(plain) {
        return word.to_owned();
    }
    format!("'{}'", word.replace('\'', r"'\''"))
}

/// A refusal, with whatever the policy printed before it, if a worker ran, and
/// the equivalent `inspect` invocation, if `run` was refused.
#[derive(Debug)]
pub struct Failure {
    pub refusal: Refusal,
    pub diagnostics: Option<Diagnostics>,
    pub inspect: Option<Invocation>,
}

impl From<Refusal> for Failure {
    fn from(refusal: Refusal) -> Self {
        Failure {
            refusal,
            diagnostics: None,
            inspect: None,
        }
    }
}

impl Failure {
    pub fn with_diagnostics(refusal: Refusal, diagnostics: Diagnostics) -> Self {
        Failure {
            refusal,
            diagnostics: Some(diagnostics),
            inspect: None,
        }
    }

    pub fn with_inspect(mut self, inspect: Invocation) -> Self {
        self.inspect = Some(inspect);
        self
    }

    pub fn to_json(&self) -> Value {
        let refusal = &self.refusal;
        let mut error = Map::new();
        error.insert("code".into(), refusal.code.into());
        error.insert("stage".into(), refusal.stage.as_str().into());
        error.insert("message".into(), refusal.message.clone().into());
        for (name, value) in [
            ("input", &refusal.input),
            ("source", &refusal.source),
            ("location", &refusal.location),
            ("policyCode", &refusal.policy_code),
        ] {
            if let Some(value) = value {
                error.insert(name.into(), value.clone().into());
            }
        }
        if let Some(bound) = &refusal.bound {
            let mut named = bound.to_json();
            named["name"] = bound.name.into();
            error.insert("bound".into(), named);
        }
        if let Some(run) = &refusal.run {
            error.insert("run".into(), run.to_json());
        }
        error.insert("remedy".into(), refusal.remedy.clone().into());
        if let Some(inspect) = &self.inspect {
            error.insert("inspect".into(), inspect.to_json());
        }
        error.insert("exit".into(), refusal.exit.into());
        let mut document = Map::new();
        document.insert("schemaVersion".into(), 1.into());
        document.insert("error".into(), Value::Object(error));
        if let Some(diagnostics) = &self.diagnostics {
            document.insert("diagnostics".into(), diagnostics.to_json());
        }
        Value::Object(document)
    }

    pub fn to_text(&self) -> String {
        let refusal = &self.refusal;
        let mut text = String::new();
        if let Some(diagnostics) = &self.diagnostics {
            text.push_str(&diagnostics.to_text());
        }
        let _ = writeln!(
            text,
            "harness-dispatch: refused ({}, stage {}): {}",
            refusal.code,
            refusal.stage.as_str(),
            refusal.message
        );
        for (label, value) in [
            ("input", &refusal.input),
            ("source", &refusal.source),
            ("location", &refusal.location),
            ("policy code", &refusal.policy_code),
        ] {
            if let Some(value) = value {
                let _ = writeln!(text, "  {label}: {value}");
            }
        }
        if let Some(bound) = &refusal.bound {
            let _ = writeln!(text, "  bound: {} {}", bound.name, bound.to_text());
        }
        if let Some(run) = &refusal.run {
            let _ = writeln!(text, "  run: {}", run.to_text());
        }
        let _ = writeln!(text, "  remedy: {}", refusal.remedy);
        if let Some(inspect) = &self.inspect {
            let _ = writeln!(text, "  inspect: {}", inspect.to_text());
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shell_word_is_quoted_only_when_it_must_be() {
        assert_eq!(shell_word("--kind"), "--kind");
        assert_eq!(shell_word("./policies/p.ts"), "./policies/p.ts");
        assert_eq!(shell_word(""), "''");
        assert_eq!(shell_word("two words"), "'two words'");
        assert_eq!(shell_word("it's $HOME; `x`"), r"'it'\''s $HOME; `x`'");
        assert_eq!(shell_word("line\nbreak"), "'line\nbreak'");
    }

    #[test]
    fn an_unwritable_invocation_says_so_rather_than_approximating() {
        let invocation = Invocation::Unavailable {
            what: "--task-id".to_owned(),
        };
        let reason = "--task-id is not valid UTF-8, so no command line reproduces it exactly";
        assert_eq!(invocation.to_text(), format!("unavailable: {reason}"));
        assert_eq!(invocation.to_json(), json!({ "unavailable": reason }));
    }

    #[test]
    fn the_invocation_runs_in_its_directory_in_a_subshell() {
        let invocation = Invocation::Exact {
            cwd: Some("/work/my repo".to_owned()),
            argv: vec![
                "harness-dispatch".into(),
                "inspect".into(),
                "--kind".into(),
                "impl".into(),
            ],
        };
        assert_eq!(
            invocation.to_text(),
            "(cd '/work/my repo' && harness-dispatch inspect --kind impl)"
        );
        assert_eq!(
            invocation.to_json(),
            json!({ "cwd": "/work/my repo", "argv": ["harness-dispatch", "inspect", "--kind", "impl"] })
        );
    }
}
