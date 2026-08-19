use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use kdl::{KdlDocument, KdlNode};

const CONFIG_PATH: &str = ".config/grove/config.kdl";
/// The configuration delta's fixed name, searched at the two roots
/// [`DeltaRoots`] carries (`docs/adr/untracked-configuration-delta.md`).
pub const DELTA_FILE_NAME: &str = ".grove.kdl";
// This is the expand-side kind set. Once filename kinds replace the legacy
// leaf grammar, derive it from `leaf::Kind::ALL` so adding a kind invalidates
// every old complete configuration by construction.
const REQUIRED_KINDS: [&str; 19] = [
    "requirements",
    "review-requirements",
    "integrate-review-requirements",
    "design",
    "review-design",
    "integrate-review-design",
    "planning",
    "review-planning",
    "integrate-review-planning",
    "prototype",
    "review-prototype",
    "integrate-review-prototype",
    "impl",
    "review-impl",
    "integrate-review-impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
];

pub struct ExpansionContext<'a> {
    pub prompt: &'a str,
    pub session_name: &'a str,
    pub worktree: &'a Path,
    pub repository: &'a Path,
}

/// The two roots the [configuration delta](`DELTA_FILE_NAME`) is searched at,
/// **in that order** — the same two `${worktree}` and `${repo}` expand to.
///
/// They are *taken*, never re-derived here. A second notion of "the repository
/// root" computed inside this module is exactly the drift that would let the
/// search order disagree with what `${repo}` expands to in the very template it
/// selected; `crate::repo::main_repo_of` is the one derivation, and its result
/// arrives through this struct. Naming both fields also makes a caller-side swap
/// of two same-typed paths impossible.
pub struct DeltaRoots<'a> {
    pub worktree: &'a Path,
    pub repository: &'a Path,
}

pub struct SessionConfig {
    templates: HashMap<String, Template>,
}

/// One kind's compiled template together with **the file it was read from**.
///
/// Carried per kind rather than once per config, because after a delta resolves
/// there is no single answer: one kind's launch may come from the delta while
/// its neighbour's comes from the personal file. Every diagnostic that names a
/// file — the aggregate validation report, and the spawn failure — has to name
/// the one that actually supplied the failing kind, or it points a reader at a
/// file that never contained the template.
struct Template {
    words: Vec<TemplateWord>,
    source: PathBuf,
}

#[derive(Debug)]
enum TemplateWord {
    Literal(String),
    Prompt,
    SessionName,
    Worktree,
    Repository,
}

/// Which document is being validated, and so which rules bind it.
///
/// The delta is a **partial**: it may declare any subset of the nineteen, and
/// every kind it omits falls through untouched. Completeness stays the personal
/// file's rule alone (`docs/adr/complete-session-configuration.md`) — that is
/// what keeps a newly added session kind failing visibly in every stale personal
/// config instead of being silently supplied by a source that never mentions it.
/// Everything else — unknown kinds, duplicates, node shape, every template rule
/// — binds both.
#[derive(Clone, Copy, PartialEq, Eq)]
enum DocumentRole {
    Personal,
    Delta,
}

impl DocumentRole {
    fn noun(self) -> &'static str {
        match self {
            Self::Personal => "Grove configuration",
            Self::Delta => "Grove configuration delta",
        }
    }
}

#[derive(Clone, Copy)]
struct SourceLocation {
    line: usize,
    column: usize,
}

struct ValidationDiagnostic {
    location: Option<SourceLocation>,
    message: String,
}

struct NodeValidation {
    kind: String,
    location: SourceLocation,
    template: Option<Vec<TemplateWord>>,
    diagnostics: Vec<ValidationDiagnostic>,
}

impl SessionConfig {
    pub fn path(home: &Path) -> PathBuf {
        home.join(CONFIG_PATH)
    }

    /// Where the delta is looked for, in search order: the worktree root, then
    /// the main repository root. The two coincide in a single-worktree
    /// repository, which is harmless — the first candidate found wins outright.
    pub fn delta_candidates(roots: &DeltaRoots<'_>) -> [PathBuf; 2] {
        [
            roots.worktree.join(DELTA_FILE_NAME),
            roots.repository.join(DELTA_FILE_NAME),
        ]
    }

    /// The personal file, then at most one delta laid over it per kind.
    ///
    /// All-or-nothing in both halves: the personal file is read and fully
    /// validated whatever a delta says, and an unreadable, unparseable, invalid
    /// or **tracked** delta fails the load rather than falling back to the very
    /// policy its owner was moving work away from.
    pub fn load(home: &Path, roots: &DeltaRoots<'_>) -> Result<Self> {
        let path = Self::path(home);
        let source = read_source(&path)?;
        let mut templates = parse_and_validate(&path, &source, DocumentRole::Personal)?;

        if let Some(delta_path) = find_delta(roots)? {
            let delta_source = read_delta_source(&delta_path)?;
            let delta = parse_and_validate(&delta_path, &delta_source, DocumentRole::Delta)?;
            // Each declared kind wins outright: one whole template replaces one
            // whole template, so no rule has to decide which *words* of a launch
            // come from where.
            templates.extend(delta);
        }

        Ok(SessionConfig { templates })
    }

    /// The file the resolved template for `kind` was read from — the personal
    /// file, or the delta that overrode it.
    pub fn source(&self, kind: &str) -> Option<&Path> {
        self.templates
            .get(kind)
            .map(|template| template.source.as_path())
    }

    pub fn expand(&self, kind: &str, context: &ExpansionContext<'_>) -> Result<Vec<OsString>> {
        let template = self.templates.get(kind).ok_or_else(|| {
            anyhow!("Grove configuration has no target for session kind {kind:?}")
        })?;
        let mut argv = Vec::new();

        for word in &template.words {
            match word {
                TemplateWord::Literal(value) => argv.push(OsString::from(value)),
                TemplateWord::Prompt => argv.push(OsString::from(context.prompt)),
                TemplateWord::SessionName => argv.push(OsString::from(context.session_name)),
                TemplateWord::Worktree => argv.push(context.worktree.as_os_str().to_owned()),
                TemplateWord::Repository => argv.push(context.repository.as_os_str().to_owned()),
            }
        }

        Ok(argv)
    }
}

/// The first of the two searched paths that **holds anything at all**; the other
/// is not read, and the two are never merged with each other.
///
/// `symlink_metadata` rather than `is_file`, so a broken symlink or a directory
/// at the searched path is a candidate that then fails closed on read, not an
/// absence that silently resolves to the personal file.
///
/// **Only `NotFound` is absence.** Any other error means this candidate's state
/// could not be established, and the two things a caller would otherwise do with
/// it are both wrong: at the worktree root it would move on and read the
/// repository root, inverting the search precedence requirement 6 fixes, and at
/// the repository root it would fall through to the very personal file the delta
/// exists to move work away from. An unresolvable candidate is therefore the
/// same refusal an unreadable delta already is, reported against the path whose
/// state is unknown.
fn find_delta(roots: &DeltaRoots<'_>) -> Result<Option<PathBuf>> {
    for candidate in SessionConfig::delta_candidates(roots) {
        match fs::symlink_metadata(&candidate) {
            Ok(_) => return Ok(Some(candidate)),
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "failed to determine whether a Grove configuration delta is present at {}",
                        candidate.display()
                    )
                })
            }
        }
    }
    Ok(None)
}

fn read_source(path: &Path) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(source) => Ok(source),
        Err(error) if error.kind() == ErrorKind::NotFound => Err(anyhow!(
            "Grove configuration is missing at {}; required session kinds: {}",
            path.display(),
            REQUIRED_KINDS.join(", ")
        )),
        Err(error) => Err(error)
            .with_context(|| format!("failed to read Grove configuration at {}", path.display())),
    }
}

/// Read the selected delta, refusing a **tracked** one first.
///
/// Trackedness is validated on the delta the search already selected, never used
/// to select it: a tracked file at the worktree root is a refusal, not a reason
/// to read the repository root. The probe runs only because a candidate file
/// exists, so a checkout with no delta pays nothing for it, and a probe that
/// cannot be completed fails closed like any other unresolved validation.
fn read_delta_source(path: &Path) -> Result<String> {
    let tracked = crate::repo::path_is_tracked(path).with_context(|| {
        format!(
            "checking whether the Grove configuration delta at {} is tracked",
            path.display()
        )
    })?;
    if tracked {
        bail!(
            "refusing the Grove configuration delta at {path}: it is tracked in version control, \
             and a tracked delta lets a repository choose what Grove executes in every checkout \
             of it.\n  Untrack it (`git rm --cached {name}`, or drop it from the jj working-copy \
             commit) and add `/{name}` to `.gitignore`.",
            path = path.display(),
            name = DELTA_FILE_NAME
        );
    }
    fs::read_to_string(path).with_context(|| {
        format!(
            "failed to read the Grove configuration delta at {}",
            path.display()
        )
    })
}

fn parse_and_validate(
    path: &Path,
    source: &str,
    role: DocumentRole,
) -> Result<HashMap<String, Template>> {
    let document: KdlDocument = source.parse().map_err(|error: kdl::KdlError| {
        let location = source_location(source, error.span.offset());
        anyhow!(
            "{}:{}:{}: KDL syntax error: {}",
            path.display(),
            location.line,
            location.column,
            error
        )
    })?;

    validate_document(path, source, &document, role)
}

fn validate_document(
    path: &Path,
    source: &str,
    document: &KdlDocument,
    role: DocumentRole,
) -> Result<HashMap<String, Template>> {
    let mut validations = Vec::new();
    let mut occurrences: HashMap<String, Vec<SourceLocation>> = HashMap::new();

    for node in document.nodes() {
        let validation = validate_node(source, node);
        occurrences
            .entry(validation.kind.clone())
            .or_default()
            .push(validation.location);
        validations.push(validation);
    }

    let mut diagnostics = Vec::new();
    if role == DocumentRole::Personal {
        let missing = REQUIRED_KINDS
            .iter()
            .filter(|kind| !occurrences.contains_key(**kind))
            .copied()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            diagnostics.push(ValidationDiagnostic {
                location: None,
                message: format!("missing session kinds: {}", missing.join(", ")),
            });
        }
    }

    let mut duplicate_kinds = REQUIRED_KINDS
        .iter()
        .filter(|kind| occurrences.get(**kind).is_some_and(|items| items.len() > 1))
        .map(|kind| (*kind).to_owned())
        .collect::<Vec<_>>();
    for validation in &validations {
        if occurrences
            .get(&validation.kind)
            .is_some_and(|items| items.len() > 1)
            && !duplicate_kinds.contains(&validation.kind)
        {
            duplicate_kinds.push(validation.kind.clone());
        }
    }

    for kind in duplicate_kinds {
        let Some(locations) = occurrences.get(&kind) else {
            continue;
        };
        let declarations = locations
            .iter()
            .map(|location| format_location(path, *location))
            .collect::<Vec<_>>()
            .join(", ");
        diagnostics.push(ValidationDiagnostic {
            location: locations.first().copied(),
            message: format!("duplicate session kind `{kind}`; declarations at {declarations}"),
        });
    }

    let mut templates = HashMap::new();
    for validation in validations {
        if !REQUIRED_KINDS.contains(&validation.kind.as_str()) {
            diagnostics.push(ValidationDiagnostic {
                location: Some(validation.location),
                message: format!("unknown session kind `{}`", validation.kind),
            });
        }
        diagnostics.extend(validation.diagnostics);
        if let Some(words) = validation.template {
            templates.insert(
                validation.kind,
                Template {
                    words,
                    source: path.to_path_buf(),
                },
            );
        }
    }

    if !diagnostics.is_empty() {
        return Err(anyhow!(render_diagnostics(path, role, diagnostics)));
    }

    Ok(templates)
}

fn validate_node(source: &str, node: &KdlNode) -> NodeValidation {
    let kind = node.name().value().to_owned();
    let location = source_location(source, node.span().offset());
    let mut diagnostics = Vec::new();
    let has_property = node.entries().iter().any(|entry| entry.name().is_some());

    if has_property || node.children().is_some() {
        diagnostics.push(at_node(
            location,
            "properties and child blocks are not allowed".to_owned(),
        ));
    }
    if node.ty().is_some() || node.entries().iter().any(|entry| entry.ty().is_some()) {
        diagnostics.push(at_node(
            location,
            "type annotations are not allowed".to_owned(),
        ));
    }

    let positional = node
        .entries()
        .iter()
        .filter(|entry| entry.name().is_none())
        .collect::<Vec<_>>();
    if positional.len() != 1 {
        diagnostics.push(at_node(
            location,
            "session kind must have exactly one positional argument".to_owned(),
        ));
    }

    let template = if positional.len() == 1 {
        match positional[0].value().as_string() {
            Some(template) => validate_template(&kind, location, template, &mut diagnostics),
            None => {
                diagnostics.push(at_node(
                    location,
                    "session kind's sole argument must be a string".to_owned(),
                ));
                None
            }
        }
    } else {
        None
    };

    NodeValidation {
        kind,
        location,
        template,
        diagnostics,
    }
}

fn validate_template(
    kind: &str,
    location: SourceLocation,
    template: &str,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> Option<Vec<TemplateWord>> {
    if contains_shell_comment_start(template) {
        diagnostics.push(at_template(
            location,
            kind,
            "`#` starts a comment in a command template; quote it to pass it literally".to_owned(),
        ));
        return None;
    }

    let words = match shell_words::split(template) {
        Ok(words) => words,
        Err(_) => {
            diagnostics.push(at_template(
                location,
                kind,
                "command template has unmatched quotes".to_owned(),
            ));
            return None;
        }
    };

    if words.is_empty() {
        diagnostics.push(at_template(
            location,
            kind,
            "word zero must be a literal non-empty executable".to_owned(),
        ));
    }

    let mut compiled = Vec::with_capacity(words.len());
    let mut counts: HashMap<&'static str, usize> = HashMap::new();
    for (index, word) in words.into_iter().enumerate() {
        let parsed = parse_template_word(kind, &word, location, diagnostics);
        if index == 0 && !matches!(parsed, TemplateWord::Literal(ref value) if !value.is_empty()) {
            diagnostics.push(at_template(
                location,
                kind,
                "word zero must be a literal executable".to_owned(),
            ));
        }
        if let Some(name) = parsed.substitution_name() {
            *counts.entry(name).or_default() += 1;
        }
        compiled.push(parsed);
    }

    if counts.get("${prompt}").copied().unwrap_or_default() != 1 {
        diagnostics.push(at_template(
            location,
            kind,
            "command template must contain `${prompt}` exactly once".to_owned(),
        ));
    }
    for name in ["${session_name}", "${worktree}", "${repo}"] {
        if counts.get(name).copied().unwrap_or_default() > 1 {
            diagnostics.push(at_template(
                location,
                kind,
                format!("`{name}` may appear at most once"),
            ));
        }
    }

    Some(compiled)
}

#[derive(Clone, Copy)]
enum ShellWordScanState {
    Delimiter,
    DelimiterBackslash,
    Unquoted,
    UnquotedBackslash,
    SingleQuoted,
    DoubleQuoted,
    DoubleQuotedBackslash,
}

fn contains_shell_comment_start(template: &str) -> bool {
    use ShellWordScanState::*;

    let mut state = Delimiter;
    for character in template.chars() {
        state = match (state, character) {
            (Delimiter, '#') => return true,
            (Delimiter, '\'') => SingleQuoted,
            (Delimiter, '"') => DoubleQuoted,
            (Delimiter, '\\') => DelimiterBackslash,
            (Delimiter, '\t' | ' ' | '\n') => Delimiter,
            (Delimiter, _) => Unquoted,
            (DelimiterBackslash, '\n') => Delimiter,
            (DelimiterBackslash, _) => Unquoted,
            (Unquoted, '\'') => SingleQuoted,
            (Unquoted, '"') => DoubleQuoted,
            (Unquoted, '\\') => UnquotedBackslash,
            (Unquoted, '\t' | ' ' | '\n') => Delimiter,
            (Unquoted, _) | (UnquotedBackslash, _) => Unquoted,
            (SingleQuoted, '\'') => Unquoted,
            (SingleQuoted, _) => SingleQuoted,
            (DoubleQuoted, '"') => Unquoted,
            (DoubleQuoted, '\\') => DoubleQuotedBackslash,
            (DoubleQuoted, _) | (DoubleQuotedBackslash, _) => DoubleQuoted,
        };
    }
    false
}

fn parse_template_word(
    kind: &str,
    word: &str,
    location: SourceLocation,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> TemplateWord {
    match word {
        "${prompt}" => TemplateWord::Prompt,
        "${session_name}" => TemplateWord::SessionName,
        "${worktree}" => TemplateWord::Worktree,
        "${repo}" => TemplateWord::Repository,
        _ if is_whole_substitution(word) => {
            diagnostics.push(at_template(
                location,
                kind,
                format!("unknown substitution `{word}`"),
            ));
            TemplateWord::Literal(word.to_owned())
        }
        _ if word.contains("${") => {
            diagnostics.push(at_template(
                location,
                kind,
                format!("substitutions must occupy a complete shell word, got `{word}`"),
            ));
            TemplateWord::Literal(word.to_owned())
        }
        _ => TemplateWord::Literal(word.to_owned()),
    }
}

impl TemplateWord {
    fn substitution_name(&self) -> Option<&'static str> {
        match self {
            TemplateWord::Literal(_) => None,
            TemplateWord::Prompt => Some("${prompt}"),
            TemplateWord::SessionName => Some("${session_name}"),
            TemplateWord::Worktree => Some("${worktree}"),
            TemplateWord::Repository => Some("${repo}"),
        }
    }
}

fn is_whole_substitution(word: &str) -> bool {
    word.starts_with("${") && word.ends_with('}') && word.matches('}').count() == 1
}

fn at_node(location: SourceLocation, message: String) -> ValidationDiagnostic {
    ValidationDiagnostic {
        location: Some(location),
        message,
    }
}

fn at_template(location: SourceLocation, kind: &str, message: String) -> ValidationDiagnostic {
    at_node(location, format!("session kind `{kind}`: {message}"))
}

fn render_diagnostics(
    path: &Path,
    role: DocumentRole,
    diagnostics: Vec<ValidationDiagnostic>,
) -> String {
    let mut rendered = format!("invalid {} at {}:", role.noun(), path.display());
    for diagnostic in diagnostics {
        rendered.push_str("\n  - ");
        if let Some(location) = diagnostic.location {
            let _ = write!(rendered, "{}: ", format_location(path, location));
        }
        rendered.push_str(&diagnostic.message);
    }
    rendered
}

fn format_location(path: &Path, location: SourceLocation) -> String {
    format!("{}:{}:{}", path.display(), location.line, location.column)
}

fn source_location(source: &str, offset: usize) -> SourceLocation {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    let column = source[line_start..offset].chars().count() + 1;
    SourceLocation { line, column }
}
