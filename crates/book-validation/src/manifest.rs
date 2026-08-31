//! The book manifest — `walkthrough.toml`, loaded as data.
//!
//! Everything this module exposes was a compiled-in constant naming one book
//! until `validator-structure-k21`: the page inventory, the page-to-slice
//! mapping, the canonical slice order and the accepted `--through` tokens. The
//! validator now learns all of it from the book directory named by `--book`,
//! which is what lets a second book be checked by the same binary.
//!
//! What is *not* book-specific stays here as spec vocabulary: the three page
//! roles, `README.md` as the contents file, and `source-index` /
//! `concept-index` as the two lookup identities. Those are fixed by
//! `docs/specs/walkthrough-books.md` for every book, so a book cannot rename
//! them and the manifest is not asked to restate them.

use std::collections::BTreeSet;

use serde::Deserialize;

/// The contents page's file name and identifier, fixed for every book by the
/// specification's *What a book is* and its canonical `book-page` identity
/// line. A book that renamed either would be demanding an identity line no
/// other book can carry.
const CONTENTS_FILE: &str = "README.md";
const CONTENTS_ID: &str = "contents";
/// The two lookup page identities, fixed for every book by the specification's
/// canonical `book-page` identity lines.
const LOOKUP_IDS: [&str; 2] = ["concept-index", "source-index"];
/// The lookup page whose directives carry the source roots.
const SOURCE_INDEX_ID: &str = "source-index";
/// The manifest's own file name inside the book directory.
pub(crate) const MANIFEST_FILE: &str = "walkthrough.toml";

/// A schema-invalid manifest, reported by the CLI as `U002` with this reason.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManifestError {
    reason: String,
}

impl ManifestError {
    fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    Contents,
    Chapter,
    Lookup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    file: String,
    id: String,
    title: String,
    role: Role,
    slice: Option<String>,
    /// Position among chapters, from 1; `None` for the contents and lookup
    /// pages. It is derived from array position rather than declared, so there
    /// is no second place for it to disagree.
    order: Option<usize>,
}

impl Page {
    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn slice(&self) -> Option<&str> {
        self.slice.as_deref()
    }

    /// The exact `book-page` identity line this page must declare.
    pub fn identity(&self) -> String {
        match (self.role, &self.slice, self.order) {
            (Role::Chapter, Some(slice), Some(order)) => format!(
                "<!-- book-page id=\"{}\" slice=\"{slice}\" order=\"{order}\" -->",
                self.id
            ),
            (Role::Contents, _, _) => {
                format!("<!-- book-page id=\"{}\" role=\"contents\" -->", self.id)
            }
            _ => format!("<!-- book-page id=\"{}\" role=\"lookup\" -->", self.id),
        }
    }
}

/// A `--through` value that resolved against one book's scoped-slice domain.
///
/// The core cannot represent an unknown scoped value or a final-only slice:
/// the only constructor is [`Manifest::resolve_scoped`], and it carries the
/// resolved chapter index the prefix is computed from rather than free text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedSlice {
    chapter_index: usize,
    token: String,
}

impl ScopedSlice {
    pub fn as_str(&self) -> &str {
        &self.token
    }

    /// Position among the book's chapters, from zero.
    pub fn chapter_index(&self) -> usize {
        self.chapter_index
    }
}

/// One book's structure, loaded from its `walkthrough.toml`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Manifest {
    root: String,
    id: String,
    pages: Vec<Page>,
    scoped: Vec<String>,
    root_paths: Vec<String>,
}

impl Manifest {
    /// Parse and validate `text` as the manifest of the book rooted at
    /// `book_root` (a normalized repository-relative directory path).
    pub fn load(book_root: &str, text: &str) -> Result<Self, ManifestError> {
        // The same encoding rule every book page is held to, and for the same
        // reason: a CRLF or unterminated manifest is drift that would otherwise
        // reach the reader as a confusing structural finding somewhere else.
        if text.contains('\r') {
            return Err(ManifestError::new(
                "the manifest must use LF line endings and contain no carriage return",
            ));
        }
        if !text.ends_with('\n') {
            return Err(ManifestError::new("the manifest must end in one LF"));
        }
        let raw: RawManifest =
            toml::from_str(text).map_err(|error| ManifestError::new(flatten(&error)))?;
        raw.into_manifest(book_root)
    }

    /// The repository-relative book directory, without a trailing separator.
    pub fn book_root(&self) -> &str {
        &self.root
    }

    pub fn book_id(&self) -> &str {
        &self.id
    }

    /// The repository-relative path of `file` inside the book directory.
    pub fn path(&self, file: &str) -> String {
        format!("{}/{file}", self.root)
    }

    pub fn manifest_path(&self) -> String {
        self.path(MANIFEST_FILE)
    }

    /// The repository-relative paths of the book's declared source roots, in
    /// declaration order. The CLI loads exactly these as the snapshot's source
    /// bytes: a book names its own sources, and nothing else is read.
    pub fn root_paths(&self) -> &[String] {
        &self.root_paths
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn chapters(&self) -> impl Iterator<Item = &Page> + Clone {
        self.pages.iter().filter(|page| page.role == Role::Chapter)
    }

    pub fn chapter(&self, index: usize) -> Option<&Page> {
        self.chapters().nth(index)
    }

    pub fn chapter_count(&self) -> usize {
        self.chapters().count()
    }

    /// The page a slice owns, if the slice names a chapter.
    pub fn page_of_slice(&self, slice: &str) -> Option<&Page> {
        self.chapters().find(|page| page.slice() == Some(slice))
    }

    /// Position of `slice` in the canonical chapter order.
    pub fn slice_order(&self, slice: &str) -> Option<usize> {
        self.chapters().position(|page| page.slice() == Some(slice))
    }

    pub fn contents_page(&self) -> Option<&Page> {
        self.pages.iter().find(|page| page.role == Role::Contents)
    }

    pub fn lookup_pages(&self) -> impl Iterator<Item = &Page> + Clone {
        self.pages.iter().filter(|page| page.role == Role::Lookup)
    }

    /// The lookup page whose directives carry this book's source roots.
    pub fn source_index_path(&self) -> String {
        self.path(
            self.lookup_pages()
                .find(|page| page.id == SOURCE_INDEX_ID)
                .map_or(SOURCE_INDEX_ID, Page::file),
        )
    }

    /// The identifier of the lookup page carrying this book's source roots.
    pub fn source_index_id(&self) -> &str {
        SOURCE_INDEX_ID
    }

    /// The accepted `--through` values, in canonical order.
    pub fn scoped_slices(&self) -> &[String] {
        &self.scoped
    }

    /// Resolve a caller's `--through` text against this book's scoped domain.
    pub fn resolve_scoped(&self, token: &str) -> Option<ScopedSlice> {
        self.scoped
            .iter()
            .find(|candidate| *candidate == token)
            .and_then(|token| {
                Some(ScopedSlice {
                    chapter_index: self.slice_order(token)?,
                    token: token.clone(),
                })
            })
    }
}

/// One line of prose from a TOML parse error.
///
/// The parser renders a multi-line snippet with a gutter and a caret ruler,
/// which reads well in a terminal and badly inside a diagnostic message that is
/// itself compared byte for byte by the contract tests. The source excerpt is
/// dropped and the sentences kept.
fn flatten(error: &toml::de::Error) -> String {
    let text = error.to_string();
    let kept: Vec<&str> = text
        .lines()
        .map(str::trim_end)
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with('|')
                && !trimmed.starts_with('^')
                && !trimmed
                    .split_once(" | ")
                    .is_some_and(|(gutter, _)| gutter.bytes().all(|byte| byte.is_ascii_digit()))
        })
        .collect();
    if kept.is_empty() {
        "unparseable manifest".to_owned()
    } else {
        kept.join("; ")
    }
}

// ---------------------------------------------------------------------------
// The TOML shape.
//
// Unknown keys are rejected rather than ignored: a mistyped field that is
// silently dropped is an obligation that silently disappears.
//
// What is checked here and what is not, exactly.
//
// Checked: every group's key set and value types; that no field a group
// requires is empty; the fragment-ID grammar of `[[root]] id` and `[[block]]
// id`; the uniqueness rules; and that every `[[block]]` names a declared root
// and a declared chapter's slice. Those are the identities and references the
// page-structure code and the derived scoped domain reason about.
//
// NOT checked, and belonging to `validator-fragments-k22` with `ROOTS`,
// `BLOCKS` and `EARLY_USES`: the corpus rule itself — the base patterns
// anchored to `[book].subject`, the two accepted pattern forms, the closed
// exception classes, the `tests.rs` form of an `inline-test-module` exclusion,
// the `N-M` range grammar, and the block partition. A manifest breaking one of
// those loads clean today. That is a stated gap, not an oversight: those rules
// are the corpus rule, and the leaf that implements them is the leaf that
// starts reading the data they govern.
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    schema: i64,
    book: RawBook,
    corpus: RawCorpus,
    #[serde(default)]
    page: Vec<RawPage>,
    #[serde(default)]
    root: Vec<RawRoot>,
    #[serde(default)]
    block: Vec<RawBlock>,
    #[serde(default, rename = "early-use")]
    early_use: Vec<RawEarlyUse>,
    guide: RawGuide,
    #[serde(default)]
    glossary: Vec<RawGlossary>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawBook {
    id: String,
    title: String,
    subject: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCorpus {
    include: Vec<String>,
    #[serde(default)]
    add: Vec<RawException>,
    #[serde(default)]
    exclude: Vec<RawException>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawException {
    path: String,
    class: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPage {
    file: String,
    id: String,
    title: String,
    role: Role,
    #[serde(default)]
    slice: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRoot {
    id: String,
    path: String,
    // Read by `validator-fragments-k22`, which replaces `ROOTS` with these
    // rows; this leaf validates only that the field is present and typed.
    #[expect(dead_code, reason = "corpus data belongs to validator-fragments-k22")]
    lines: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawBlock {
    id: String,
    root: String,
    owner: String,
    // As `RawRoot::lines`: the partition this expresses is checked by
    // `validator-fragments-k22`, which owns `BLOCKS`.
    #[expect(dead_code, reason = "corpus data belongs to validator-fragments-k22")]
    lines: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEarlyUse {
    symbols: String,
    #[serde(rename = "first-use")]
    first_use: String,
    owner: String,
    statement: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawGuide {
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    anchors: Option<Vec<String>>,
    #[serde(default)]
    omitted: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawGlossary {
    path: String,
    anchors: Vec<String>,
}

impl RawManifest {
    fn into_manifest(self, book_root: &str) -> Result<Manifest, ManifestError> {
        if self.schema != 1 {
            return Err(ManifestError::new(format!(
                "manifest schema {} is unsupported; this validator reads schema 1",
                self.schema
            )));
        }
        let directory = book_root.rsplit('/').next().unwrap_or(book_root);
        if !valid_id(&self.book.id) {
            return Err(ManifestError::new(format!(
                "`[book] id` `{}` is not a valid book id",
                self.book.id
            )));
        }
        if self.book.id != directory {
            return Err(ManifestError::new(format!(
                "`[book] id` `{}` does not equal the book directory name `{directory}`",
                self.book.id
            )));
        }
        if self.book.title.is_empty() || self.book.subject.is_empty() {
            return Err(ManifestError::new(
                "`[book] title` and `[book] subject` must be non-empty",
            ));
        }
        check_guide(&self.guide)?;
        check_corpus(&self.corpus)?;
        check_early_uses(&self.early_use)?;
        check_glossary(&self.glossary)?;
        let pages = build_pages(self.page)?;
        for root in &self.root {
            if !valid_id(&root.id) {
                return Err(ManifestError::new(format!(
                    "`[[root]] id` `{}` is not a valid fragment id",
                    root.id
                )));
            }
        }
        for block in &self.block {
            if !valid_id(&block.id) {
                return Err(ManifestError::new(format!(
                    "`[[block]] id` `{}` is not a valid fragment id",
                    block.id
                )));
            }
        }
        let roots = collect_unique(
            self.root.iter().map(|root| root.id.as_str()),
            "`[[root]]` id",
        )?;
        collect_unique(
            self.root.iter().map(|root| root.path.as_str()),
            "`[[root]]` path",
        )?;
        collect_unique(
            self.block.iter().map(|block| block.id.as_str()),
            "`[[block]]` id",
        )?;
        let slices: Vec<&str> = pages
            .iter()
            .filter_map(|page| page.slice.as_deref())
            .collect();
        for block in &self.block {
            if !roots.contains(block.root.as_str()) {
                return Err(ManifestError::new(format!(
                    "`[[block]]` `{}` names undeclared root `{}`",
                    block.id, block.root
                )));
            }
            if !slices.contains(&block.owner.as_str()) {
                return Err(ManifestError::new(format!(
                    "`[[block]]` `{}` names owner `{}`, which is no chapter's slice",
                    block.id, block.owner
                )));
            }
        }
        // The scoped-slice domain is derived, never declared: a slice is a
        // `--through` value exactly when some block names it as owner, so a
        // slice that owns no source has no prefix to prove and is final-only.
        let scoped: Vec<String> = slices
            .iter()
            .filter(|slice| self.block.iter().any(|block| block.owner == **slice))
            .map(|slice| (*slice).to_owned())
            .collect();
        Ok(Manifest {
            root: book_root.trim_end_matches('/').to_owned(),
            id: self.book.id,
            pages,
            scoped,
            root_paths: self.root.iter().map(|root| root.path.clone()).collect(),
        })
    }
}

fn build_pages(raw: Vec<RawPage>) -> Result<Vec<Page>, ManifestError> {
    if raw.is_empty() {
        return Err(ManifestError::new("a manifest declares at least one page"));
    }
    collect_unique(raw.iter().map(|page| page.file.as_str()), "`[[page]]` file")?;
    collect_unique(raw.iter().map(|page| page.id.as_str()), "`[[page]]` id")?;
    collect_unique(
        raw.iter().filter_map(|page| page.slice.as_deref()),
        "`[[page]]` slice",
    )?;
    let mut pages = Vec::with_capacity(raw.len());
    let mut chapters = 0;
    let mut lookups = Vec::new();
    for (index, page) in raw.into_iter().enumerate() {
        if page.title.is_empty() || !valid_id(&page.id) {
            return Err(ManifestError::new(format!(
                "`[[page]]` `{}` needs a title and a valid id",
                page.file
            )));
        }
        if !plain_markdown_file(&page.file) {
            return Err(ManifestError::new(format!(
                "`[[page]] file` `{}` must be a plain `.md` file name inside the book directory",
                page.file
            )));
        }
        match page.role {
            Role::Contents => {
                if index != 0 || page.file != CONTENTS_FILE || page.id != CONTENTS_ID {
                    return Err(ManifestError::new(format!(
                        "the first `[[page]]` is `{CONTENTS_FILE}` with `id = \"{CONTENTS_ID}\"` and `role = \"contents\"`"
                    )));
                }
            }
            Role::Chapter => {
                if !lookups.is_empty() {
                    return Err(ManifestError::new(
                        "every chapter `[[page]]` precedes the lookup pages",
                    ));
                }
                chapters += 1;
            }
            Role::Lookup => lookups.push(page.id.clone()),
        }
        if index == 0 && page.role != Role::Contents {
            return Err(ManifestError::new(format!(
                "the first `[[page]]` is `{CONTENTS_FILE}` with `id = \"{CONTENTS_ID}\"` and `role = \"contents\"`"
            )));
        }
        let order = match (page.role, &page.slice) {
            (Role::Chapter, Some(slice)) => {
                if !valid_id(slice) {
                    return Err(ManifestError::new(format!(
                        "`[[page]]` `{}` declares invalid slice `{slice}`",
                        page.file
                    )));
                }
                let Some((number, stem)) = numbered_stem(&page.file) else {
                    return Err(ManifestError::new(format!(
                        "chapter `{}` must be named `NN-<stem>.md`",
                        page.file
                    )));
                };
                if stem != page.id {
                    return Err(ManifestError::new(format!(
                        "chapter `{}` must have id `{stem}`, its file name stem without the numeric prefix",
                        page.file
                    )));
                }
                // Chapters are numbered from `01` and the numbers are the
                // canonical order, so a file name that disagrees with its own
                // position is two orderings in one manifest — exactly what
                // deriving `order` from array position exists to prevent.
                if number != chapters {
                    return Err(ManifestError::new(format!(
                        "chapter `{}` is chapter {chapters}; its file name must be numbered `{chapters:02}`",
                        page.file
                    )));
                }
                Some(chapters)
            }
            (Role::Chapter, None) => {
                return Err(ManifestError::new(format!(
                    "chapter `{}` must declare a `slice`",
                    page.file
                )))
            }
            (_, Some(_)) => {
                return Err(ManifestError::new(format!(
                    "only a chapter may declare a `slice`; `{}` does",
                    page.file
                )))
            }
            (_, None) => None,
        };
        pages.push(Page {
            file: page.file,
            id: page.id,
            title: page.title,
            role: page.role,
            slice: page.slice,
            order,
        });
    }
    lookups.sort();
    if lookups != LOOKUP_IDS {
        return Err(ManifestError::new(format!(
            "a manifest declares exactly the lookup pages `{}` and `{}`",
            LOOKUP_IDS[0], LOOKUP_IDS[1]
        )));
    }
    if chapters == 0 {
        return Err(ManifestError::new(
            "a manifest declares at least one chapter `[[page]]`",
        ));
    }
    Ok(pages)
}

/// `NN-<stem>.md` -> `(NN, <stem>)`.
fn numbered_stem(file: &str) -> Option<(usize, &str)> {
    let stem = file.strip_suffix(".md")?;
    let (prefix, stem) = stem.split_once('-')?;
    Some((prefix.parse().ok()?, stem))
}

/// A page file is a plain `.md` name in the book directory: no separator, no
/// parent component, no dot-relative prefix. Anything else would join into a
/// path the inventory can never match, and would be diagnosed as a missing page
/// rather than as the malformed manifest it is.
fn plain_markdown_file(file: &str) -> bool {
    file.len() > 3
        && file.ends_with(".md")
        && !file.starts_with('.')
        && !file.contains('/')
        && !file.contains('\\')
}

fn check_guide(guide: &RawGuide) -> Result<(), ManifestError> {
    match (&guide.path, &guide.anchors, &guide.omitted) {
        (Some(path), Some(anchors), None) if !path.is_empty() && !anchors.is_empty() => Ok(()),
        (None, None, Some(reason)) if !reason.is_empty() => Ok(()),
        _ => Err(ManifestError::new(
            "`[guide]` is either `path` with a non-empty `anchors` array or `omitted` with a non-empty reason",
        )),
    }
}

fn check_glossary(entries: &[RawGlossary]) -> Result<(), ManifestError> {
    for entry in entries {
        if entry.path.is_empty() || entry.anchors.is_empty() {
            return Err(ManifestError::new(
                "every `[[glossary]]` entry carries a `path` and a non-empty `anchors` array",
            ));
        }
    }
    Ok(())
}

fn check_corpus(corpus: &RawCorpus) -> Result<(), ManifestError> {
    if corpus.include.is_empty() {
        return Err(ManifestError::new(
            "`[corpus] include` is a non-empty array of patterns",
        ));
    }
    for exception in corpus.add.iter().chain(&corpus.exclude) {
        if exception.path.is_empty() || exception.class.is_empty() || exception.reason.is_empty() {
            return Err(ManifestError::new(
                "every corpus exception carries a non-empty `path`, `class` and `reason`",
            ));
        }
    }
    Ok(())
}

fn check_early_uses(entries: &[RawEarlyUse]) -> Result<(), ManifestError> {
    for entry in entries {
        if entry.symbols.is_empty()
            || entry.first_use.is_empty()
            || entry.owner.is_empty()
            || entry.statement.is_empty()
        {
            return Err(ManifestError::new(
                "every `[[early-use]]` entry carries `symbols`, `first-use`, `owner` and `statement`",
            ));
        }
    }
    Ok(())
}

fn collect_unique<'a>(
    values: impl Iterator<Item = &'a str>,
    subject: &str,
) -> Result<BTreeSet<&'a str>, ManifestError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(ManifestError::new(format!(
                "{subject} `{value}` is declared twice"
            )));
        }
    }
    Ok(seen)
}

fn valid_id(id: &str) -> bool {
    let Some(first) = id.as_bytes().first() else {
        return false;
    };
    first.is_ascii_lowercase()
        && !id.contains("--")
        && !id.ends_with('-')
        && id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}
