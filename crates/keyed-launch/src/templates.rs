use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use kdl::{KdlDocument, KdlNode};

use crate::argv::{Argv, Slot};
use crate::error::ConfigError;
use crate::vocabulary::{Requirement, Vocabulary};

/// A loaded configuration: every key the primary document declares, mapped to
/// one complete command template read whole out of one file.
pub struct Templates {
    primary: PathBuf,
    overlay: Option<PathBuf>,
    slots: Vec<SlotSpec>,
    templates: BTreeMap<String, Template>,
    /// Keys the overlay declares and the primary does not. Kept rather than
    /// discarded so the refusal can say *why* a key that is plainly written down
    /// somewhere still does not resolve — the difference between a typo and a
    /// misunderstanding of what an overlay may do.
    overlay_only: BTreeSet<String>,
}

struct SlotSpec {
    name: String,
    requirement: Requirement,
}

/// One key's compiled template together with **the file it was read from**.
///
/// Carried per key rather than once per configuration, because after an overlay
/// resolves there is no single answer: one key's launch may come from the
/// overlay while its neighbour's comes from the primary file. Every diagnostic
/// that names a file has to name the one that actually supplied the failing key,
/// or it points a reader at a file that never held the template.
struct Template {
    words: Vec<Word>,
    source: PathBuf,
}

/// A compiled template word: a literal, or the slot it stands for by index into
/// [`Templates::slots`].
enum Word {
    Literal(String),
    Slot(usize),
}

/// Which document is being validated, and so which file a diagnostic names.
///
/// The rules do **not** differ by role: syntax, duplicates, node shape and every
/// template rule bind both documents identically, and the one asymmetry between
/// them — that an overlay overrides and never supplies — is a resolution rule
/// rather than a validation one, applied after both documents have passed.
#[derive(Clone, Copy, PartialEq, Eq)]
enum DocumentRole {
    Primary,
    Overlay,
}

impl DocumentRole {
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
}

struct ValidationDiagnostic {
    location: Option<SourceLocation>,
    message: String,
}

struct NodeValidation {
    key: String,
    location: SourceLocation,
    template: Option<Vec<Word>>,
    diagnostics: Vec<ValidationDiagnostic>,
}

impl Templates {
    /// Read and fully validate both documents, then resolve one template per key.
    ///
    /// A key resolves from the primary file or the overlay, never from both, and
    /// only if the **primary** declares it: the overlay overrides and never
    /// supplies. Both documents are validated whole against `vocabulary` — a
    /// malformed template for a key this run will never reach still fails here,
    /// before anything is spawned.
    ///
    /// All-or-nothing in both halves: an unreadable, unparseable or invalid
    /// overlay fails the load rather than falling back to the very policy its
    /// owner was moving work away from.
    pub fn load(
        primary: &Path,
        overlay: Option<&Path>,
        vocabulary: Vocabulary<'_>,
    ) -> Result<Self, ConfigError> {
        let slots = compile_vocabulary(&vocabulary)?;

        let primary_source = read_primary(primary)?;
        let mut templates =
            parse_and_validate(primary, &primary_source, DocumentRole::Primary, &slots)?;

        let mut overlay_only = BTreeSet::new();
        if let Some(overlay_path) = overlay {
            let overlay_source = read_overlay(overlay_path)?;
            let declared =
                parse_and_validate(overlay_path, &overlay_source, DocumentRole::Overlay, &slots)?;
            // Each key the primary already declares wins outright: one whole
            // template replaces one whole template, so no rule has to decide
            // which *words* of a launch come from where. A key the primary does
            // not declare is set aside rather than admitted, which is the
            // per-key form of what the old all-keys completeness rule bought.
            for (key, template) in declared {
                match templates.entry(key) {
                    Entry::Occupied(mut occupied) => {
                        occupied.insert(template);
                    }
                    Entry::Vacant(vacant) => {
                        overlay_only.insert(vacant.into_key());
                    }
                }
            }
        }

        Ok(Templates {
            primary: primary.to_path_buf(),
            overlay: overlay.map(Path::to_path_buf),
            slots,
            templates,
            overlay_only,
        })
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
        Err(ConfigError::new(self.unresolved(key)))
    }

    /// Expand this key's template into an argv.
    ///
    /// The values must fill the slots the vocabulary declared: one value per
    /// declared slot, no duplicates, no name the vocabulary does not hold. That
    /// is expansion's whole obligation — every other template rule was checked
    /// at load — and it is stated over the vocabulary rather than over this
    /// template's own words so a consumer cannot have a call that works for one
    /// key and fails for its neighbour purely because the two templates mention
    /// different optional slots.
    pub fn expand(&self, key: &str, values: &[Slot<'_>]) -> Result<Argv, ConfigError> {
        self.require(key)?;
        let template = &self.templates[key];
        let offered = self.match_values(values)?;

        let mut words = Vec::with_capacity(template.words.len());
        for word in &template.words {
            words.push(match word {
                Word::Literal(value) => OsString::from(value),
                Word::Slot(index) => offered[*index].to_owned(),
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
                return Err(ConfigError::new(format!(
                    "no slot named `{}` is declared; declared slots: {}",
                    value.name,
                    self.declared_slots()
                )));
            };
            if offered[index].is_some() {
                return Err(ConfigError::new(format!(
                    "slot `{}` was offered more than one value",
                    value.name
                )));
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
            return Err(ConfigError::new(format!(
                "no value offered for declared slot{}: {}",
                if missing.len() == 1 { "" } else { "s" },
                missing.join(", ")
            )));
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
        if self.overlay_only.contains(key) {
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
        if slots.iter().any(|slot| slot.name == rule.name) {
            return Err(ConfigError::new(format!(
                "the slot vocabulary declares `{}` more than once",
                rule.name
            )));
        }
        slots.push(SlotSpec {
            name: rule.name.to_owned(),
            requirement: rule.requirement,
        });
    }
    Ok(slots)
}

fn read_primary(path: &Path) -> Result<String, ConfigError> {
    match fs::read_to_string(path) {
        Ok(source) => Ok(source),
        Err(error) if error.kind() == ErrorKind::NotFound => Err(ConfigError::new(format!(
            "configuration is missing at {}",
            path.display()
        ))),
        Err(error) => Err(ConfigError::new(format!(
            "failed to read the configuration at {}: {error}",
            path.display()
        ))),
    }
}

fn read_overlay(path: &Path) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|error| {
        ConfigError::new(format!(
            "failed to read the configuration overlay at {}: {error}",
            path.display()
        ))
    })
}

fn parse_and_validate(
    path: &Path,
    source: &str,
    role: DocumentRole,
    slots: &[SlotSpec],
) -> Result<BTreeMap<String, Template>, ConfigError> {
    let document: KdlDocument = source.parse().map_err(|error: kdl::KdlError| {
        let location = source_location(source, error.span.offset());
        ConfigError::new(format!(
            "{}:{}:{}: KDL syntax error: {}",
            path.display(),
            location.line,
            location.column,
            error
        ))
    })?;

    validate_document(path, source, &document, role, slots)
}

fn validate_document(
    path: &Path,
    source: &str,
    document: &KdlDocument,
    role: DocumentRole,
    slots: &[SlotSpec],
) -> Result<BTreeMap<String, Template>, ConfigError> {
    let mut validations = Vec::new();
    let mut occurrences: HashMap<String, Vec<SourceLocation>> = HashMap::new();

    for node in document.nodes() {
        let validation = validate_node(source, node, slots);
        occurrences
            .entry(validation.key.clone())
            .or_default()
            .push(validation.location);
        validations.push(validation);
    }

    let mut diagnostics = Vec::new();

    // Duplicates, in declaration order of their first appearance, each naming
    // every one of its own locations.
    let mut duplicates: Vec<String> = Vec::new();
    for validation in &validations {
        if occurrences
            .get(&validation.key)
            .is_some_and(|items| items.len() > 1)
            && !duplicates.contains(&validation.key)
        {
            duplicates.push(validation.key.clone());
        }
    }
    for key in duplicates {
        let Some(locations) = occurrences.get(&key) else {
            continue;
        };
        let declarations = locations
            .iter()
            .map(|location| format_location(path, *location))
            .collect::<Vec<_>>()
            .join(", ");
        diagnostics.push(ValidationDiagnostic {
            location: locations.first().copied(),
            message: format!("duplicate key `{key}`; declarations at {declarations}"),
        });
    }

    let mut templates = BTreeMap::new();
    for validation in validations {
        diagnostics.extend(validation.diagnostics);
        if let Some(words) = validation.template {
            templates.insert(
                validation.key,
                Template {
                    words,
                    source: path.to_path_buf(),
                },
            );
        }
    }

    if !diagnostics.is_empty() {
        return Err(ConfigError::new(render_diagnostics(
            path,
            role,
            diagnostics,
        )));
    }

    Ok(templates)
}

fn validate_node(source: &str, node: &KdlNode, slots: &[SlotSpec]) -> NodeValidation {
    let key = node.name().value().to_owned();
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
            "a key must have exactly one positional argument".to_owned(),
        ));
    }

    let template = if positional.len() == 1 {
        match positional[0].value().as_string() {
            Some(template) => validate_template(&key, location, template, slots, &mut diagnostics),
            None => {
                diagnostics.push(at_node(
                    location,
                    "a key's sole argument must be a string".to_owned(),
                ));
                None
            }
        }
    } else {
        None
    };

    NodeValidation {
        key,
        location,
        template,
        diagnostics,
    }
}

fn validate_template(
    key: &str,
    location: SourceLocation,
    template: &str,
    slots: &[SlotSpec],
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> Option<Vec<Word>> {
    if contains_shell_comment_start(template) {
        diagnostics.push(at_template(
            location,
            key,
            "`#` starts a comment in a command template; quote it to pass it literally".to_owned(),
        ));
        return None;
    }

    let words = match shell_words::split(template) {
        Ok(words) => words,
        Err(_) => {
            diagnostics.push(at_template(
                location,
                key,
                "command template has unmatched quotes".to_owned(),
            ));
            return None;
        }
    };

    if words.is_empty() {
        diagnostics.push(at_template(
            location,
            key,
            "word zero must be a literal non-empty executable".to_owned(),
        ));
    }

    let mut compiled = Vec::with_capacity(words.len());
    let mut counts = vec![0usize; slots.len()];
    for (index, word) in words.into_iter().enumerate() {
        let parsed = parse_template_word(key, &word, location, slots, diagnostics);
        if index == 0 && !matches!(parsed, Word::Literal(ref value) if !value.is_empty()) {
            diagnostics.push(at_template(
                location,
                key,
                "word zero must be a literal executable".to_owned(),
            ));
        }
        if let Word::Slot(slot) = parsed {
            counts[slot] += 1;
        }
        compiled.push(parsed);
    }

    for (slot, count) in slots.iter().zip(counts) {
        if !slot.requirement.admits(count) {
            diagnostics.push(at_template(
                location,
                key,
                slot.requirement.violation(&slot.name),
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

fn parse_template_word(
    key: &str,
    word: &str,
    location: SourceLocation,
    slots: &[SlotSpec],
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> Word {
    if let Some(name) = whole_substitution(word) {
        if let Some(index) = slots.iter().position(|slot| slot.name == name) {
            return Word::Slot(index);
        }
        diagnostics.push(at_template(
            location,
            key,
            format!("unknown substitution `{word}`"),
        ));
        return Word::Literal(word.to_owned());
    }
    if word.contains("${") {
        diagnostics.push(at_template(
            location,
            key,
            format!("substitutions must occupy a complete shell word, got `{word}`"),
        ));
    }
    Word::Literal(word.to_owned())
}

/// The slot name in `${name}`, when the word is *nothing but* that substitution.
fn whole_substitution(word: &str) -> Option<&str> {
    let inner = word.strip_prefix("${")?.strip_suffix('}')?;
    (!inner.contains('}')).then_some(inner)
}

fn at_node(location: SourceLocation, message: String) -> ValidationDiagnostic {
    ValidationDiagnostic {
        location: Some(location),
        message,
    }
}

fn at_template(location: SourceLocation, key: &str, message: String) -> ValidationDiagnostic {
    at_node(location, format!("key `{key}`: {message}"))
}

/// Aggregate, not first-error: one report lists every duplicate with all of its
/// locations, every malformed node, and every invalid template with its key and
/// location, so one edit fixes the file rather than uncovering the next problem.
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

/// The keys the primary document declares, in name order. The conformance kit's
/// one window into a loaded configuration — enough to say *this checked
/// nothing*, and nothing more.
impl Templates {
    #[must_use]
    pub fn keys(&self) -> Vec<&str> {
        self.templates.keys().map(String::as_str).collect()
    }
}
