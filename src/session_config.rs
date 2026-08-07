use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use kdl::{KdlDocument, KdlNode, KdlValue};

const CONFIG_PATH: &str = ".config/grove/config.kdl";
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
    pub herdr_settings: Option<[&'a str; 2]>,
}

pub struct SessionConfig {
    templates: HashMap<String, Vec<TemplateWord>>,
}

#[derive(Debug)]
enum TemplateWord {
    Literal(String),
    Prompt,
    SessionName,
    Worktree,
    Repository,
    HerdrSettings,
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
    pub fn load(home: &Path) -> Result<Self> {
        let path = home.join(CONFIG_PATH);
        let source = read_source(&path)?;
        let document: KdlDocument = source.parse().map_err(|error: kdl::KdlError| {
            let location = source_location(&source, error.span.offset());
            anyhow!(
                "{}:{}:{}: KDL syntax error: {}",
                path.display(),
                location.line,
                location.column,
                error
            )
        })?;

        validate_document(&path, &source, &document)
    }

    pub fn expand(&self, kind: &str, context: &ExpansionContext<'_>) -> Result<Vec<OsString>> {
        let template = self.templates.get(kind).ok_or_else(|| {
            anyhow!("Grove configuration has no target for session kind {kind:?}")
        })?;
        let mut argv = Vec::new();

        for word in template {
            match word {
                TemplateWord::Literal(value) => argv.push(OsString::from(value)),
                TemplateWord::Prompt => argv.push(OsString::from(context.prompt)),
                TemplateWord::SessionName => argv.push(OsString::from(context.session_name)),
                TemplateWord::Worktree => argv.push(context.worktree.as_os_str().to_owned()),
                TemplateWord::Repository => argv.push(context.repository.as_os_str().to_owned()),
                TemplateWord::HerdrSettings => {
                    if let Some(settings) = context.herdr_settings {
                        argv.extend(settings.into_iter().map(OsString::from));
                    }
                }
            }
        }

        Ok(argv)
    }
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

fn validate_document(path: &Path, source: &str, document: &KdlDocument) -> Result<SessionConfig> {
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
        if let Some(template) = validation.template {
            templates.insert(validation.kind, template);
        }
    }

    if !diagnostics.is_empty() {
        return Err(anyhow!(render_diagnostics(path, diagnostics)));
    }

    Ok(SessionConfig { templates })
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
        match positional[0].value() {
            KdlValue::String(template) => validate_template(location, template, &mut diagnostics),
            _ => {
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
    location: SourceLocation,
    template: &str,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> Option<Vec<TemplateWord>> {
    let words = match shell_words::split(template) {
        Ok(words) => words,
        Err(_) => {
            diagnostics.push(at_node(
                location,
                "command template has unmatched quotes".to_owned(),
            ));
            return None;
        }
    };

    if words.is_empty() || words[0].is_empty() {
        diagnostics.push(at_node(
            location,
            "word zero must be a literal non-empty executable".to_owned(),
        ));
    }

    let mut compiled = Vec::with_capacity(words.len());
    let mut counts: HashMap<&'static str, usize> = HashMap::new();
    for (index, word) in words.into_iter().enumerate() {
        let parsed = parse_template_word(&word, location, diagnostics);
        if index == 0 && !matches!(parsed, TemplateWord::Literal(ref value) if !value.is_empty()) {
            diagnostics.push(at_node(
                location,
                "word zero must be a literal executable".to_owned(),
            ));
        }
        if let Some(name) = parsed.substitution_name() {
            *counts.entry(name).or_default() += 1;
        }
        compiled.push(parsed);
    }

    if counts.get("${prompt}").copied().unwrap_or_default() != 1 {
        diagnostics.push(at_node(
            location,
            "command template must contain `${prompt}` exactly once".to_owned(),
        ));
    }
    for name in [
        "${session_name}",
        "${worktree}",
        "${repo}",
        "${herdr_settings}",
    ] {
        if counts.get(name).copied().unwrap_or_default() > 1 {
            diagnostics.push(at_node(
                location,
                format!("`{name}` may appear at most once"),
            ));
        }
    }

    Some(compiled)
}

fn parse_template_word(
    word: &str,
    location: SourceLocation,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> TemplateWord {
    match word {
        "${prompt}" => TemplateWord::Prompt,
        "${session_name}" => TemplateWord::SessionName,
        "${worktree}" => TemplateWord::Worktree,
        "${repo}" => TemplateWord::Repository,
        "${herdr_settings}" => TemplateWord::HerdrSettings,
        _ if is_whole_substitution(word) => {
            diagnostics.push(at_node(location, format!("unknown substitution `{word}`")));
            TemplateWord::Literal(word.to_owned())
        }
        _ if word.contains("${") => {
            diagnostics.push(at_node(
                location,
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
            TemplateWord::HerdrSettings => Some("${herdr_settings}"),
        }
    }
}

fn is_whole_substitution(word: &str) -> bool {
    word.starts_with("${") && word.ends_with('}')
}

fn at_node(location: SourceLocation, message: String) -> ValidationDiagnostic {
    ValidationDiagnostic {
        location: Some(location),
        message,
    }
}

fn render_diagnostics(path: &Path, diagnostics: Vec<ValidationDiagnostic>) -> String {
    let mut rendered = format!("invalid Grove configuration at {}:", path.display());
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
