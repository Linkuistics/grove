pub mod cli;
pub mod corpus;
mod ledger;
pub mod manifest;
mod markdown;
mod parser;
mod validator;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub use corpus::{derive as derive_corpus, CorpusError};
pub use manifest::{
    Class, Corpus, EarlyUse, Exception, Guide, Manifest, ManifestError, OutboundDocument,
    OwnershipBlock, Page, Pattern, Role, ScopedSlice, SourceRoot,
};
pub use markdown::{scan_markdown_links, MarkdownLink};
pub use validator::validate;

#[derive(Clone, Debug, Default)]
pub struct BookSnapshot {
    /// The book's own structure, loaded from its `walkthrough.toml`. The core
    /// performs no discovery of its own: everything it knows about which pages
    /// exist, what they are called and which slice owns which, it reads here.
    pub manifest: Manifest,
    /// The repository-relative paths the book's corpus rule reaches, derived
    /// from the real directory by [`corpus::derive`]. The core requires this to
    /// equal the set of declared `[[root]]` paths and reports `F006` in either
    /// direction; it is the one thing in the snapshot that is *not* the book's
    /// own account of itself.
    pub derived_corpus: BTreeSet<String>,
    pub book_files: BTreeMap<String, Vec<u8>>,
    pub source_files: BTreeMap<String, Vec<u8>>,
    /// The bytes of the documents outside the book its manifest declares — the
    /// `[guide]` path and each `[[glossary]]` path. A book is self-contained
    /// for its claims and still cites two documents it does not own, and the
    /// anchors it reserves from them cannot be checked against bytes nobody
    /// read.
    pub outbound_files: BTreeMap<String, Vec<u8>>,
    pub book_entries: BTreeSet<String>,
    pub non_regular_book_entries: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub scope: Scope,
    pub check: Check,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Scope {
    Through(ScopedSlice),
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
                map.serialize_entry("slice", slice.as_str())?;
            }
            Self::Final => map.serialize_entry("kind", "final")?,
        }
        map.end()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Check {
    Fragments,
    Markdown,
    All,
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
    pub source: Option<SourceLocation>,
    pub related: Vec<RelatedLocation>,
    pub remedy: Option<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct Location {
    pub path: String,
    pub byte: usize,
    pub line: usize,
    pub column: usize,
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("path", &self.path)?;
        map.serialize_entry("byte", &self.byte)?;
        if self.line == 0 {
            map.serialize_entry("line", &Option::<usize>::None)?;
        } else {
            map.serialize_entry("line", &self.line)?;
        }
        if self.column == 0 {
            map.serialize_entry("column", &Option::<usize>::None)?;
        } else {
            map.serialize_entry("column", &self.column)?;
        }
        map.end()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SourceLocation {
    pub path: String,
    pub byte: usize,
    pub line: usize,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RelatedLocation {
    pub path: String,
    pub byte: usize,
    pub line: usize,
    pub column: usize,
    pub label: String,
}

impl RelatedLocation {
    pub(crate) fn from_location(location: &Location, label: impl Into<String>) -> Self {
        Self {
            path: location.path.clone(),
            byte: location.byte,
            line: location.line,
            column: location.column,
            label: label.into(),
        }
    }
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
            source: None,
            related: Vec::new(),
            remedy: default_remedy(code).map(str::to_owned),
        }
    }

    pub(crate) fn with_source(mut self, source: SourceLocation) -> Self {
        self.source = Some(source);
        self
    }

    pub(crate) fn with_related(mut self, related: Vec<RelatedLocation>) -> Self {
        self.related = related;
        self
    }
}

fn default_remedy(code: &str) -> Option<&'static str> {
    Some(match code {
        "P001" => "replace the line with the exact reserved directive form",
        "P002" => "move or close the construct so the directive is in a valid context",
        "P003" => "encode the page as UTF-8 with LF line endings and one final LF",
        "F001" => "give every fragment and source root a unique ID",
        "F002" => "define exactly one target with this ID or correct the insert",
        "F003" => "make the defer match a named later ownership block",
        "F004" => "remove one insertion edge from the reported cycle",
        "F005" => "attach the fragment once beneath its declared source root",
        "F006" => "restore the fixed source-root inventory and authoritative path",
        "F007" => "restore a gapless, ordered, single-owner source partition",
        "F008" => "restore the literal bytes from the declared source range",
        "F009" => "make the ledger row and directive describe the same value",
        "F010" => "move the fragment definition to its owner's assigned numbered page",
        "M101" => "restore the canonical page inventory, identity, and required structure",
        "M102" => "restore the required heading and explicit-anchor structure",
        "M103" => "restore canonical contents and previous/contents/next navigation",
        "M104" => "move production source into a declared four-backtick fragment",
        "M105" => {
            "add a prose paragraph before the literal fragment with no intervening nonblank block"
        }
        "M201" => "correct the link destination or restore its local target",
        "U001" => "run `book-check --help` for accepted arguments",
        "U002" => "restore read access to the required repository input",
        "I001" => "retry has no defined remedy; report the stable internal category",
        _ => return None,
    })
}
