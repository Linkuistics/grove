pub mod cli;
mod ledger;
mod parser;
mod validator;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use validator::validate;

#[derive(Clone, Debug, Default)]
pub struct BookSnapshot {
    pub book_files: BTreeMap<String, Vec<u8>>,
    pub source_files: BTreeMap<String, Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub scope: Scope,
    pub check: Check,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Scope {
    Through(String),
    Final,
}

impl Serialize for Scope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(None)?;
        match self {
            Self::Through(slice) => {
                map.serialize_entry("kind", "through")?;
                map.serialize_entry("slice", slice)?;
            }
            Self::Final => map.serialize_entry("kind", "final")?,
        }
        map.end()
    }
}

pub const SOURCE_PATHS: &[&str] = &[
    "crates/ordinal-fs-tree/Cargo.toml",
    "crates/ordinal-fs-tree/bin/syllabus.rs",
    "crates/ordinal-fs-tree/src/lib.rs",
    "crates/ordinal-fs-tree/src/conformance.rs",
    "crates/ordinal-fs-tree/src/error.rs",
    "crates/ordinal-fs-tree/src/name.rs",
    "crates/ordinal-fs-tree/src/ops.rs",
    "crates/ordinal-fs-tree/src/plan.rs",
    "crates/ordinal-fs-tree/src/reference.rs",
    "crates/ordinal-fs-tree/src/report.rs",
    "crates/ordinal-fs-tree/src/snapshot.rs",
    "crates/ordinal-fs-tree/src/fs/mod.rs",
    "crates/ordinal-fs-tree/src/fs/read.rs",
    "crates/ordinal-fs-tree/src/fs/apply.rs",
    "crates/ordinal-fs-tree/src/fs/lock.rs",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Check {
    Fragments,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ValidationReport {
    pub schema: u8,
    pub status: ReportStatus,
    pub valid: bool,
    pub scope: Scope,
    pub coverage: Coverage,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReportStatus {
    Valid,
    Findings,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Coverage {
    pub files: usize,
    pub resolved_lines: usize,
    pub deferred_lines: usize,
    #[serde(rename = "final")]
    pub final_: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: String,
    pub phase: String,
    pub message: String,
    pub primary: Location,
    pub fragment_id: Option<String>,
    pub root_id: Option<String>,
    pub remedy: Option<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Location {
    pub path: String,
    pub byte: usize,
    pub line: usize,
    pub column: usize,
}

impl Diagnostic {
    pub(crate) fn new(
        code: &str,
        phase: &str,
        message: impl Into<String>,
        primary: Location,
        fragment_id: Option<&str>,
        root_id: Option<&str>,
    ) -> Self {
        Self {
            code: code.into(),
            phase: phase.into(),
            message: message.into(),
            primary,
            fragment_id: fragment_id.map(str::to_owned),
            root_id: root_id.map(str::to_owned),
            remedy: None,
        }
    }
}
