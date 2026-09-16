use std::fmt;

use crate::templates::{Source, SourceSpan};

/// One selected/include occurrence, including an unresolved selection in an error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Occurrence {
    pub id: usize,
    pub profile: String,
    pub parent: Option<usize>,
    pub selection_index: usize,
    pub via: Option<SourceSpan>,
}

/// A stable machine-readable refusal with the locations and names available at
/// the failing operation. Byte ranges refer to the captured UTF-8 source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub category: String,
    pub message: String,
    pub source: Option<Source>,
    pub primary: Option<SourceSpan>,
    pub related: Vec<SourceSpan>,
    pub occurrence_chain: Vec<Occurrence>,
    pub key: Option<String>,
    pub binding: Option<String>,
    pub command: Option<String>,
    pub parameter: Option<String>,
    pub remedy: String,
}

impl Diagnostic {
    pub(crate) fn new(category: &str, message: impl Into<String>, remedy: &str) -> Self {
        Self {
            category: category.to_owned(),
            message: message.into(),
            source: None,
            primary: None,
            related: Vec::new(),
            occurrence_chain: Vec::new(),
            key: None,
            binding: None,
            command: None,
            parameter: None,
            remedy: remedy.to_owned(),
        }
    }
}

/// Reading, validation, resolution or expansion failed. Display is for humans;
/// [`Self::diagnostics`] exposes stable categories without parsing that prose.
/// The error remains opaque and implements `std::error::Error` without imposing
/// an error-handling dependency on its consumers.
pub struct ConfigError {
    message: String,
    diagnostics: Vec<Diagnostic>,
}

impl ConfigError {
    pub(crate) fn new(category: &str, message: impl Into<String>, remedy: &str) -> Self {
        Self::from_diagnostics(vec![Diagnostic::new(category, message, remedy)])
    }

    pub(crate) fn from_diagnostics(diagnostics: Vec<Diagnostic>) -> Self {
        let message = diagnostics
            .iter()
            .map(|d| format!("{}\n  {}", d.message, d.remedy))
            .collect::<Vec<_>>()
            .join("\n");
        Self {
            message,
            diagnostics,
        }
    }

    pub(crate) fn contextualize(mut self, source: Option<Source>, key: Option<&str>) -> Self {
        for diagnostic in &mut self.diagnostics {
            diagnostic.source.clone_from(&source);
            diagnostic.key = key.map(str::to_owned);
        }
        self
    }

    pub(crate) fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Independent refusals in source-role, byte-position and key order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a `{:?}` of this error is read by a human in
/// a panic or an `anyhow` chain, and a record dump would obscure the human report.
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ConfigError {}

/// Everything that can go wrong allocating a channel, spawning a child, or
/// supervising one.
///
/// **Opaque for the same reason [`ConfigError`] is**, and deliberately a
/// *separate* type rather than a shared one: a caller that only loads and
/// expands a configuration never handles a spawn failure, and one that only
/// launches never handles a KDL parse error. Two types keep the two halves of
/// this crate usable apart — which is the whole claim `Templates` and `run`
/// make by not referring to each other.
///
/// Its obligation is the same: name what is wrong, name where, and name what
/// would fix it.
pub struct LaunchError {
    message: String,
}

impl LaunchError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump — see [`ConfigError`]'s `Debug` for why.
impl fmt::Debug for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LaunchError {}
