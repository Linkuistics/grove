//! Owned explanation records. These are output, never authority to construct an Argv.
use crate::{Occurrence, Selection, Source, SourceSpan};

/// One declaration's location. IDs are response-local indices into `origins`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Origin {
    pub id: usize,
    pub span: SourceSpan,
    pub occurrence: Option<usize>,
}

/// The scope and name of an assigned setting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Setting {
    BindingTarget { binding: String },
    RouteTarget { key: String },
    ParameterDefault { command: String, parameter: String },
    CommandParameter { command: String, parameter: String },
    RouteParameter { key: String, parameter: String },
}

/// Literal templates remain distinct from binding names, even for identical text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssignmentValue {
    Set(String),
    LiteralTemplate(String),
    Unset,
    Reset,
}

/// An application in total fold order, referencing an origin ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    pub order: usize,
    pub value: AssignmentValue,
    pub origin: usize,
}

/// All assignments to a setting, including overwritten values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignmentHistory {
    pub id: usize,
    pub setting: Setting,
    pub assignments: Vec<Assignment>,
}

/// A compiled word, shared by template validation, inspection and expansion.
/// A runtime slot stays symbolic until expansion supplies its native value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompiledWord {
    Literal(String),
    Slot(String),
}

/// One word and every declaration contributing to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WordView {
    pub word: CompiledWord,
    pub origins: Vec<usize>,
}

/// A resolved parameter and its contributing origin and history IDs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterView {
    pub name: String,
    pub value: String,
    pub origins: Vec<usize>,
    pub histories: Vec<usize>,
}

/// One admitted command, executable first. Flat commands have no binding,
/// named command or parameters; their target history retains replaced templates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandView {
    pub key: String,
    pub binding: Option<String>,
    pub command: Option<String>,
    pub parameters: Vec<ParameterView>,
    pub words: Vec<WordView>,
    pub origins: Vec<usize>,
    pub histories: Vec<usize>,
}

/// A declared key that primary policy did not authorize.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonAdmittedKey {
    pub key: String,
    pub origins: Vec<usize>,
    pub reason: String,
}

/// A captured resolution's explanation, independent of later source changes.
/// Sources follow primary then overlay order. Origins and assignments follow
/// base, included/selected patches and overlay, in source order within each patch.
/// Commands, non-admitted keys and flat target histories follow key order.
/// Origin/history IDs index their respective vectors; spans address the original
/// UTF-8 source bytes. The view is never accepted as input to expansion or launch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inspection {
    pub sources: Vec<Source>,
    pub selection: Selection,
    pub profile_occurrences: Vec<Occurrence>,
    pub commands: Vec<CommandView>,
    pub non_admitted_keys: Vec<NonAdmittedKey>,
    pub origins: Vec<Origin>,
    pub histories: Vec<AssignmentHistory>,
}
