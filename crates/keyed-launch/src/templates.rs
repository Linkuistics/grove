use std::collections::{BTreeMap, HashMap};
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use kdl::{KdlDocument, KdlNode};

use crate::argv::{Argv, Slot};
use crate::error::{ConfigError, Diagnostic};
use crate::inspection::{
    Assignment, AssignmentHistory, AssignmentValue, CommandView, CompiledWord, Inspection,
    NonAdmittedKey, Origin, Setting, WordView,
};
use crate::vocabulary::{Requirement, Vocabulary};

mod named;

/// Which explicit input supplied a declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceRole {
    Primary,
    Overlay,
}

/// An explicit source path and its role, independent of filesystem availability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub role: SourceRole,
    pub path: PathBuf,
}

/// Zero-based UTF-8 byte range in a captured source; end is exclusive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    pub source: Source,
    pub start: usize,
    pub end: usize,
}

/// Explicit profile selection, applied left to right with each include occurrence.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    pub profiles: Vec<String>,
    pub origin: Option<SourceSpan>,
}

/// Validated input documents and vocabulary, captured once at load.
/// Resolution never opens their paths again. Inactive profiles are structurally
/// validated; selected occurrences compose before the local overlay.
pub struct Catalog {
    captured: Arc<Captured>,
}

struct Captured {
    primary: CapturedDocument,
    overlay: Option<CapturedDocument>,
    slots: Vec<SlotSpec>,
}

/// Keep original bytes and parsed declarations, including overridden and
/// overlay-only entries, for subsequent provenance without rereading a file.
struct CapturedDocument {
    path: PathBuf,
    _source: String,
    _document: KdlDocument,
    named: named::Declarations,
}

/// A resolved configuration: primary-authorized keys mapped to compiled commands.
pub struct Templates {
    // Preserve both documents independently of the Catalog's lifetime.
    _captured: Arc<Captured>,
    primary: PathBuf,
    overlay: Option<PathBuf>,
    slots: Vec<SlotSpec>,
    templates: BTreeMap<String, Template>,
    /// Keys the overlay declares and the primary does not. Kept rather than
    /// discarded so the refusal can say *why* a key that is plainly written down
    /// somewhere still does not resolve — the difference between a typo and a
    /// misunderstanding of what an overlay may do.
    overlay_only: BTreeMap<String, SourceSpan>,
    inspection: Inspection,
}

#[derive(Clone)]
struct SlotSpec {
    name: String,
    requirement: Requirement,
}

/// One key's compiled words and the command definition's source path.
/// Runtime expansion errors name that definition; inspection retains the
/// separate route, binding and parameter origins that contributed to the words.
#[derive(Clone)]
struct Template {
    words: Vec<Word>,
    source: PathBuf,
}

/// Validation and expansion use the same literal/slot representation as inspection.
type Word = CompiledWord;

/// Which document is being validated, and so which file a diagnostic names.
///
/// Both sources receive structural checks. Named definitions are
/// primary-only; primary key authority is enforced later during resolution.
#[derive(Clone, Copy, PartialEq, Eq)]
enum DocumentRole {
    Primary,
    Overlay,
}

impl DocumentRole {
    fn source(self, path: &Path) -> Source {
        Source {
            role: match self {
                Self::Primary => SourceRole::Primary,
                Self::Overlay => SourceRole::Overlay,
            },
            path: path.to_owned(),
        }
    }

    fn noun(self) -> &'static str {
        match self {
            Self::Primary => "configuration",
            Self::Overlay => "configuration overlay",
        }
    }
}

#[derive(Clone, Copy)]
struct SourceLocation {
    line: usize,
    column: usize,
    start: usize,
    end: usize,
}

struct ValidationDiagnostic {
    category: &'static str,
    key: Option<String>,
    related: Vec<SourceLocation>,
    remedy: &'static str,
    location: Option<SourceLocation>,
    message: String,
}

impl Catalog {
    /// Capture both documents with structural validation.
    /// Named command templates validate only when an effective binding uses them,
    /// after resolution folds local target replacements. An invalid input fails
    /// closed rather than falling back to another policy.
    pub fn load(
        primary: &Path,
        overlay: Option<&Path>,
        vocabulary: Vocabulary<'_>,
    ) -> Result<Self, ConfigError> {
        let slots = compile_vocabulary(&vocabulary)?;

        let primary_result = read_primary(primary)
            .and_then(|text| parse_and_validate(primary, text, DocumentRole::Primary));
        let overlay_result = overlay
            .map(|path| {
                read_overlay(path)
                    .and_then(|text| parse_and_validate(path, text, DocumentRole::Overlay))
            })
            .transpose();
        let mut diagnostics = Vec::new();
        let primary = primary_result
            .map_err(|error| diagnostics.extend(error.into_diagnostics()))
            .ok();
        let overlay = overlay_result
            .map_err(|error| diagnostics.extend(error.into_diagnostics()))
            .ok();
        if !diagnostics.is_empty() {
            return Err(ConfigError::from_diagnostics(diagnostics));
        }
        let primary = primary.expect("successful primary capture");
        let overlay = overlay.expect("successful overlay capture");
        Ok(Self {
            captured: Arc::new(Captured {
                primary,
                overlay,
                slots,
            }),
        })
    }

    /// Captured primary declaration; absence differs from an explicit empty list.
    #[must_use]
    pub fn primary_selection(&self) -> Option<&Selection> {
        self.captured.primary.named.selection.as_ref()
    }

    /// Captured overlay declaration; choosing between sources belongs to the caller.
    #[must_use]
    pub fn overlay_selection(&self) -> Option<&Selection> {
        self.captured
            .overlay
            .as_ref()
            .and_then(|document| document.named.selection.as_ref())
    }

    /// Fold base, selected profile occurrences and local patches with primary authority.
    /// Validate effective references and return an independently owned snapshot.
    pub fn resolve(&self, selection: &Selection) -> Result<Templates, ConfigError> {
        let (templates, overlay_only, inspection) = named::resolve(&self.captured, selection)?;

        Ok(Templates {
            inspection,
            _captured: Arc::clone(&self.captured),
            primary: self.captured.primary.path.clone(),
            overlay: self
                .captured
                .overlay
                .as_ref()
                .map(|source| source.path.clone()),
            slots: self.captured.slots.clone(),
            templates,
            overlay_only,
        })
    }

    pub(crate) fn slot_names(&self) -> impl Iterator<Item = &str> {
        self.captured.slots.iter().map(|slot| slot.name.as_str())
    }
}

impl Templates {
    /// Explain this captured resolution without reading sources or launching.
    #[must_use]
    pub fn inspect(&self) -> &Inspection {
        &self.inspection
    }

    /// Load a Catalog and resolve an empty explicit selection through the same
    /// validation path. Source discovery and selection policy belong to the caller.
    pub fn load(
        primary: &Path,
        overlay: Option<&Path>,
        vocabulary: Vocabulary<'_>,
    ) -> Result<Self, ConfigError> {
        Catalog::load(primary, overlay, vocabulary)?.resolve(&Selection::default())
    }

    /// The file this key's template was actually read from — the primary file,
    /// or the overlay that overrode it. `None` when the primary does not declare
    /// it, whatever the overlay says.
    #[must_use]
    pub fn source(&self, key: &str) -> Option<&Path> {
        self.templates
            .get(key)
            .map(|template| template.source.as_path())
    }

    /// Does this key resolve to exactly one complete template?
    ///
    /// The obligation a consumer discharges *before* it commits to a key —
    /// before it writes down work of that kind, or launches it — stated once,
    /// here, so the refusal's wording has one owner. [`Self::expand`] asks the
    /// same question on its own way in.
    pub fn require(&self, key: &str) -> Result<(), ConfigError> {
        if self.templates.contains_key(key) {
            return Ok(());
        }
        let mut diagnostic = Diagnostic::new(
            "unconfigured_key",
            self.unresolved(key),
            &format!("Declare `{key}` in {}.", self.primary.display()),
        );
        diagnostic.source = Some(Source {
            role: SourceRole::Primary,
            path: self.primary.clone(),
        });
        diagnostic.key = Some(key.to_owned());
        if let Some(span) = self.overlay_only.get(key) {
            diagnostic.related.push(span.clone());
        }
        Err(ConfigError::from_diagnostics(vec![diagnostic]))
    }

    /// Expand this key's template into an argv.
    ///
    /// The values must fill the slots the vocabulary declared: one value per
    /// declared slot, no duplicates, no name the vocabulary does not hold, and
    /// no NUL in any offered value, even for an unused optional slot. Every
    /// other template rule was checked
    /// at load — and it is stated over the vocabulary rather than over this
    /// template's own words so a consumer cannot have a call that works for one
    /// key and fails for its neighbour purely because the two templates mention
    /// different optional slots.
    pub fn expand(&self, key: &str, values: &[Slot<'_>]) -> Result<Argv, ConfigError> {
        self.require(key)?;
        let template = &self.templates[key];
        let offered = self.match_values(values).map_err(|error| {
            let role = if self.overlay.as_ref() == Some(&template.source) {
                SourceRole::Overlay
            } else {
                SourceRole::Primary
            };
            error.contextualize(
                Some(Source {
                    role,
                    path: template.source.clone(),
                }),
                Some(key),
            )
        })?;

        let offered: HashMap<_, _> = self
            .slots
            .iter()
            .map(|slot| slot.name.as_str())
            .zip(offered)
            .collect();
        let mut words = Vec::with_capacity(template.words.len());
        for word in &template.words {
            words.push(match word {
                Word::Literal(value) => OsString::from(value),
                Word::Slot(name) => offered[name.as_str()].to_owned(),
            });
        }

        // Word zero is a literal non-empty executable, checked at load for every
        // template in both documents, so the split below cannot fail on a
        // template this type holds.
        let mut words = words.into_iter();
        let program = words
            .next()
            .expect("a validated template has at least one word");
        Ok(Argv::new(program, words.collect()))
    }

    /// Line up the offered values with the declared slots, by name.
    fn match_values<'v>(
        &self,
        values: &[Slot<'v>],
    ) -> Result<Vec<&'v std::ffi::OsStr>, ConfigError> {
        let mut offered: Vec<Option<&std::ffi::OsStr>> = vec![None; self.slots.len()];
        for value in values {
            let Some(index) = self.slots.iter().position(|slot| slot.name == value.name) else {
                return Err(ConfigError::new(
                    "invalid_value",
                    format!(
                        "no slot named `{}` is declared; declared slots: {}",
                        value.name,
                        self.declared_slots()
                    ),
                    "Supply exactly one value for each declared runtime slot and no other names.",
                ));
            };
            if offered[index].is_some() {
                return Err(ConfigError::new(
                    "invalid_value",
                    format!("slot `{}` was offered more than one value", value.name),
                    "Supply exactly one value for each declared runtime slot and no other names.",
                ));
            }
            // OsStr's encoding preserves ASCII, so NUL can be checked without
            // converting native strings to Unicode (stable since Rust 1.74):
            // https://doc.rust-lang.org/1.85.0/std/ffi/struct.OsStr.html#method.as_encoded_bytes
            if value.value.as_encoded_bytes().contains(&0) {
                return Err(ConfigError::new(
                    "invalid_value",
                    format!("runtime slot `{}` contains NUL", value.name),
                    "Remove NUL from the runtime slot value before expansion.",
                ));
            }
            offered[index] = Some(value.value);
        }

        let missing = self
            .slots
            .iter()
            .zip(&offered)
            .filter(|(_, value)| value.is_none())
            .map(|(slot, _)| slot.name.as_str())
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            return Err(ConfigError::new(
                "invalid_value",
                format!(
                    "no value offered for declared slot{}: {}",
                    if missing.len() == 1 { "" } else { "s" },
                    missing.join(", ")
                ),
                "Supply exactly one value for each declared runtime slot and no other names.",
            ));
        }

        Ok(offered
            .into_iter()
            .map(|value| value.expect("checked"))
            .collect())
    }

    fn declared_slots(&self) -> String {
        self.slots
            .iter()
            .map(|slot| slot.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The refusal for a key that does not resolve — naming the key and the
    /// primary file that must declare it, and saying so differently when the
    /// overlay does declare it, because that reader has written the key down and
    /// needs to know it is in the wrong file rather than misspelled.
    fn unresolved(&self, key: &str) -> String {
        if self.overlay_only.contains_key(key) {
            let overlay = self.overlay.as_deref().map_or_else(
                || "the overlay".to_owned(),
                |path| path.display().to_string(),
            );
            return format!(
                "key `{key}` does not resolve: it is declared only in the configuration overlay \
                 at {overlay}, and an overlay overrides a key the primary declares but never \
                 supplies one of its own.\n  Declare `{key}` in {primary}.",
                primary = self.primary.display()
            );
        }
        format!(
            "key `{key}` does not resolve: no template for it.\n  Declare `{key}` in {primary}.",
            primary = self.primary.display()
        )
    }
}

/// Turn the borrowed vocabulary into the owned slot table `Templates` keeps, and
/// refuse a duplicate name.
///
/// A duplicated slot would be counted twice against its own cardinality rule and
/// would take whichever value arrived first at expansion — a consumer bug that
/// looks like a template bug for as long as it goes unnamed.
fn compile_vocabulary(vocabulary: &Vocabulary<'_>) -> Result<Vec<SlotSpec>, ConfigError> {
    let mut slots: Vec<SlotSpec> = Vec::with_capacity(vocabulary.slots.len());
    for rule in vocabulary.slots {
        if rule.name.starts_with("param.") {
            return Err(ConfigError::new(
                "invalid_value",
                format!(
                    "slot `{}` uses the reserved `param.` prefix; choose a runtime slot name \
                 outside the configuration parameter namespace",
                    rule.name
                ),
                "Use unique runtime slot names outside the reserved `param.` namespace.",
            ));
        }
        if slots.iter().any(|slot| slot.name == rule.name) {
            return Err(ConfigError::new(
                "invalid_value",
                format!(
                    "the slot vocabulary declares `{}` more than once",
                    rule.name
                ),
                "Use unique runtime slot names outside the reserved `param.` namespace.",
            ));
        }
        slots.push(SlotSpec {
            name: rule.name.to_owned(),
            requirement: rule.requirement,
        });
    }
    Ok(slots)
}

fn read_primary(path: &Path) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|error| {
        let message = if error.kind() == ErrorKind::NotFound {
            format!("configuration is missing at {}", path.display())
        } else {
            format!(
                "failed to read the configuration at {}: {error}",
                path.display()
            )
        };
        ConfigError::new(
            "source_read",
            message,
            "Create a readable UTF-8 configuration at this path.",
        )
        .contextualize(
            Some(Source {
                role: SourceRole::Primary,
                path: path.to_owned(),
            }),
            None,
        )
    })
}

fn read_overlay(path: &Path) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|error| {
        ConfigError::new(
            "source_read",
            format!(
                "failed to read the configuration overlay at {}: {error}",
                path.display()
            ),
            "Make the selected overlay readable as UTF-8, or stop selecting it.",
        )
        .contextualize(
            Some(Source {
                role: SourceRole::Overlay,
                path: path.to_owned(),
            }),
            None,
        )
    })
}

fn parse_and_validate(
    path: &Path,
    source: String,
    role: DocumentRole,
) -> Result<CapturedDocument, ConfigError> {
    let document: KdlDocument = source.parse().map_err(|error: kdl::KdlError| {
        let location = source_location(&source, error.span.offset());
        let mut diagnostic = Diagnostic::new(
            "kdl_syntax",
            format!(
                "{}:{}:{}: KDL syntax error: {}",
                path.display(),
                location.line,
                location.column,
                error
            ),
            "Correct the KDL syntax at the reported location.",
        );
        let source = role.source(path);
        diagnostic.primary = Some(SourceSpan {
            source: source.clone(),
            start: error.span.offset(),
            end: error.span.offset() + error.span.len(),
        });
        diagnostic.source = Some(source);
        ConfigError::from_diagnostics(vec![diagnostic])
    })?;

    let mut diagnostics = Vec::new();
    let named = named::parse(path, &source, &document, role, &mut diagnostics);
    if !diagnostics.is_empty() {
        return Err(ConfigError::from_diagnostics(render_diagnostics(
            path,
            role,
            diagnostics,
        )));
    }

    Ok(CapturedDocument {
        path: path.to_path_buf(),
        _source: source,
        _document: document,
        named,
    })
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

/// Does a `#` start a comment anywhere in this template?
///
/// `shell_words::split` treats `#` at a word boundary as a comment and silently
/// drops the rest of the line, so a template that meant to pass a `#` literally
/// would lose every word after it with nothing said. This scan is the same state
/// machine the splitter walks, run only to answer that one question.
fn contains_shell_comment_start(template: &str) -> bool {
    use ShellWordScanState::{
        Delimiter, DelimiterBackslash, DoubleQuoted, DoubleQuotedBackslash, SingleQuoted, Unquoted,
        UnquotedBackslash,
    };

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

fn at_node(location: SourceLocation, message: String) -> ValidationDiagnostic {
    ValidationDiagnostic {
        category: "shape", key: None, related: Vec::new(),
        remedy: "Use config { ... } with command definitions, bind targets and route declarations; local overlays may select or override personal bindings and routes.",
        location: Some(location),
        message,
    }
}

/// Convert validator findings into ordered records. Catalog decides whether
/// structure permits semantic reports after both document results are available.
fn render_diagnostics(
    path: &Path,
    role: DocumentRole,
    mut diagnostics: Vec<ValidationDiagnostic>,
) -> Vec<Diagnostic> {
    diagnostics.sort_by(|a, b| {
        a.location
            .map(|l| l.start)
            .cmp(&b.location.map(|l| l.start))
            .then(a.key.cmp(&b.key))
    });
    let source = role.source(path);
    let span = |location: SourceLocation| SourceSpan {
        source: source.clone(),
        start: location.start,
        end: location.end,
    };
    diagnostics
        .into_iter()
        .map(|item| {
            let location = item
                .location
                .map(|l| format!("{}: ", format_location(path, l)))
                .unwrap_or_default();
            let mut diagnostic = Diagnostic::new(
                item.category,
                format!(
                    "invalid {} at {}:\n  - {location}{}",
                    role.noun(),
                    path.display(),
                    item.message
                ),
                item.remedy,
            );
            diagnostic.source = Some(source.clone());
            diagnostic.primary = item.location.map(span);
            diagnostic.related = item.related.into_iter().map(span).collect();
            diagnostic.key = item.key;
            diagnostic
        })
        .collect()
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
    SourceLocation {
        line,
        column,
        start: offset,
        end: offset,
    }
}

/// The keys the primary document declares, in name order. The conformance kit's
/// one window into a loaded configuration — enough to say *this checked
/// nothing*, and nothing more.
impl Templates {
    #[must_use]
    pub fn keys(&self) -> Vec<&str> {
        self.templates.keys().map(String::as_str).collect()
    }
}
