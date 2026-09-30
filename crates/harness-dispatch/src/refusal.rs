//! Structured refusals: every failure before handoff names a stable code, the
//! stage it happened in, the input or source involved, and a remedy
//! (`docs/specs/harness-selection-and-execution.md`, *Diagnostics and exits*).
//!
//! A refusal never launches anything and never substitutes another candidate.
//! Text mode renders it for a person on stderr; `--json` renders it as one JSON
//! object on stderr and prints nothing on stdout, so a parser never sees a
//! partial result.

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

/// A refusal, with whatever the policy printed before it, if a worker ran.
#[derive(Debug)]
pub struct Failure {
    pub refusal: Refusal,
    pub diagnostics: Option<Diagnostics>,
}

impl From<Refusal> for Failure {
    fn from(refusal: Refusal) -> Self {
        Failure {
            refusal,
            diagnostics: None,
        }
    }
}

impl Failure {
    pub fn with_diagnostics(refusal: Refusal, diagnostics: Diagnostics) -> Self {
        Failure {
            refusal,
            diagnostics: Some(diagnostics),
        }
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
        text
    }
}
