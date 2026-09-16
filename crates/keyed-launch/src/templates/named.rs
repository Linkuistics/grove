//! Named base commands: capture structure first, then resolve effective targets.
use super::{
    at_node, contains_shell_comment_start, source_location, Assignment, AssignmentHistory,
    AssignmentValue, BTreeMap, Captured, CommandView, ConfigError, Diagnostic, DocumentRole,
    Inspection, KdlDocument, KdlNode, NonAdmittedKey, Origin, Path, Selection, Setting, SlotSpec,
    SourceLocation, SourceRole, SourceSpan, Template, ValidationDiagnostic, Word, WordView,
};
use crate::ParameterView;

#[derive(Default)]
pub(super) struct Declarations {
    commands: BTreeMap<String, Command>,
    bindings: BTreeMap<String, Target>,
    routes: BTreeMap<String, Target>,
}

#[derive(Clone)]
struct Target {
    value: String,
    span: SourceSpan,
}

struct Command {
    template: Target,
    parameters: BTreeMap<String, Parameter>,
}

struct Parameter {
    default: Option<String>,
    span: SourceSpan,
}

pub(super) fn is_wrapper(node: &KdlNode) -> bool {
    node.name().value() == "config" && (node.children().is_some() || node.entries().is_empty())
}

fn location(source: &str, node: &KdlNode) -> SourceLocation {
    // KDL spans address captured UTF-8 bytes, excluding surrounding trivia:
    // https://docs.rs/kdl/4.7.1/kdl/struct.KdlNode.html#method.span
    let mut location = source_location(source, node.span().offset());
    location.end += node.span().len();
    location
}

fn valid_name(name: &str) -> bool {
    name.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !name.ends_with('-')
        && !name.contains("--")
}

fn plain(node: &KdlNode) -> bool {
    node.ty().is_none()
        && node
            .entries()
            .iter()
            .all(|entry| entry.name().is_none() && entry.ty().is_none())
}

pub(super) fn parse(
    path: &Path,
    source: &str,
    document: &KdlDocument,
    role: DocumentRole,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> Declarations {
    let first_diagnostic = diagnostics.len();
    let mut result = Declarations::default();
    let mut wrapper = None;
    // Flat and named routes share a namespace, independent of textual order.
    let mut routes: BTreeMap<String, SourceLocation> = document
        .nodes()
        .iter()
        .filter(|node| !is_wrapper(node))
        .map(|node| (node.name().value().to_owned(), location(source, node)))
        .collect();
    for node in document.nodes().iter().filter(|node| is_wrapper(node)) {
        let loc = location(source, node);
        if let Some(previous) = wrapper {
            duplicate("config wrapper", previous, loc, diagnostics);
        }
        wrapper = Some(loc);
        if !plain(node) || !node.entries().is_empty() || node.children().is_none() {
            diagnostics.push(at_node(
                loc,
                "config requires zero arguments and a child block, without properties or types"
                    .into(),
            ));
            continue;
        }
        let Some(children) = node.children() else {
            continue;
        };
        for child in children.nodes() {
            let loc = location(source, child);
            let kind = child.name().value();
            if !matches!(kind, "command" | "bind" | "route") {
                diagnostics.push(at_node(loc, format!("unsupported config node `{kind}`; parameter patches, profiles and selections are not yet supported")));
                continue;
            }
            let values: Option<Vec<_>> = child
                .entries()
                .iter()
                .map(|entry| entry.value().as_string())
                .collect();
            let Some(values) = values.filter(|v| v.len() == 2) else {
                diagnostics.push(at_node(loc, format!("`{kind}` requires exactly two string arguments; parameter-only routes are not yet supported")));
                continue;
            };
            if !plain(child)
                || (kind == "route" && child.children().is_some_and(|c| !c.nodes().is_empty()))
                || (kind == "bind" && child.children().is_some())
            {
                diagnostics.push(at_node(loc, format!("`{kind}` does not accept properties or types; only commands accept param declaration children; parameter patches are not yet supported")));
                continue;
            }
            let name = values[0];
            if (kind == "route" && name.is_empty())
                || (kind != "route" && !valid_name(name))
                || (kind != "command" && !valid_name(values[1]))
            {
                diagnostics.push(at_node(loc, format!("invalid `{kind}` name; use lowercase letters, digits and single interior dashes; route keys must be nonempty")));
                continue;
            }
            if kind == "command" && role == DocumentRole::Overlay {
                diagnostics.push(at_node(
                    loc,
                    "command definitions belong in primary configuration, not an overlay".into(),
                ));
                continue;
            }
            let target = Target {
                value: values[1].into(),
                span: SourceSpan {
                    source: role.source(path),
                    start: loc.start,
                    end: loc.end,
                },
            };
            if kind == "command" {
                let parameters = parse_parameters(path, source, child, role, diagnostics);
                if let Some(previous) = result.commands.insert(
                    name.into(),
                    Command {
                        template: target,
                        parameters,
                    },
                ) {
                    let mut earlier = source_location(source, previous.template.span.start);
                    earlier.end = previous.template.span.end;
                    duplicate(name, earlier, loc, diagnostics);
                }
                continue;
            }
            let table = match kind {
                "bind" => &mut result.bindings,
                _ => {
                    if let Some(previous) = routes.insert(name.into(), loc) {
                        duplicate(name, previous, loc, diagnostics);
                    }
                    result.routes.insert(name.into(), target);
                    continue;
                }
            };
            if let Some(previous) = table.insert(name.into(), target) {
                let mut earlier = source_location(source, previous.span.start);
                earlier.end = previous.span.end;
                duplicate(name, earlier, loc, diagnostics);
            }
        }
    }
    for diagnostic in &mut diagnostics[first_diagnostic..] {
        if diagnostic.category == "shape" {
            diagnostic.remedy = "Use config { command \"name\" \"template\" { param \"name\" \"default\"; }; bind \"binding\" \"name\"; route \"key\" \"binding\"; }; keep definitions in primary policy and omit parameter patches/profiles/selections.";
        }
    }
    result
}

fn parse_parameters(
    path: &Path,
    source: &str,
    command: &KdlNode,
    role: DocumentRole,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> BTreeMap<String, Parameter> {
    let mut parameters: BTreeMap<String, Parameter> = BTreeMap::new();
    for node in command.children().into_iter().flat_map(KdlDocument::nodes) {
        let loc = location(source, node);
        let values: Option<Vec<_>> = node
            .entries()
            .iter()
            .map(|entry| entry.value().as_string())
            .collect();
        let Some(values) = values.filter(|v| (1..=2).contains(&v.len())) else {
            diagnostics.push(at_node(
                loc,
                "param requires a name and optional default string".into(),
            ));
            continue;
        };
        if node.name().value() != "param"
            || !plain(node)
            || node.children().is_some()
            || !valid_name(values[0])
        {
            diagnostics.push(at_node(loc, "command children must be param declarations with a valid lowercase name, no properties, types or children".into()));
            continue;
        }
        let parameter = Parameter {
            default: values.get(1).map(|value| (*value).to_owned()),
            span: SourceSpan {
                source: role.source(path),
                start: loc.start,
                end: loc.end,
            },
        };
        if let Some(previous) = parameters.insert(values[0].into(), parameter) {
            let mut earlier = source_location(source, previous.span.start);
            earlier.end = previous.span.end;
            duplicate(values[0], earlier, loc, diagnostics);
        }
    }
    parameters
}

fn duplicate(
    name: &str,
    first: SourceLocation,
    second: SourceLocation,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let mut diagnostic = at_node(first, format!("duplicate declaration `{name}`"));
    diagnostic.category = "duplicate";
    diagnostic.related.push(second);
    diagnostic.remedy = "Keep one declaration per name in this namespace and document.";
    diagnostics.push(diagnostic);
}

fn problem(category: &str, target: &Target, message: String) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        category,
        format!(
            "{} (bytes {}..{}): {message}",
            target.span.source.path.display(),
            target.span.start,
            target.span.end
        ),
        "Correct the named command or its effective binding/route target in the reported source.",
    );
    diagnostic.primary = Some(target.span.clone());
    diagnostic.source = Some(target.span.source.clone());
    diagnostic
}

/// Shell-word splitting precedes named dollar scanning. Escaping a dollar must
/// not turn escaped runtime-looking text into a slot on a second pass.
fn compile(command: &Command, slots: &[SlotSpec]) -> Result<Vec<NamedWord>, Box<Diagnostic>> {
    let invalid =
        |message: String| Box::new(problem("invalid_template", &command.template, message));
    let text = &command.template.value;
    if text.contains('\0') {
        return Err(invalid("command template contains NUL".into()));
    }
    if contains_shell_comment_start(text) {
        return Err(invalid("quote literal comment-starting `#`".into()));
    }
    let words = shell_words::split(text)
        .map_err(|_| invalid("command template has unmatched quotes".into()))?;
    if words.is_empty() {
        return Err(invalid(
            "word zero must be a literal non-empty executable".into(),
        ));
    }
    let mut result = Vec::new();
    let mut counts = vec![0; slots.len()];
    for (index, word) in words.iter().enumerate() {
        let mut rest = word.as_str();
        let mut literal = String::new();
        let mut fragments = Vec::new();
        let mut slot_word = None;
        while !rest.is_empty() {
            if let Some(next) = rest.strip_prefix("$$") {
                literal.push('$');
                rest = next;
            } else if let Some(next) = rest.strip_prefix("${") {
                let Some(end) = next.find('}') else {
                    return Err(invalid("unterminated substitution".into()));
                };
                let name = &next[..end];
                if name.contains("${") {
                    return Err(invalid("unterminated substitution".into()));
                }
                if index == 0 {
                    return Err(invalid(
                        "word zero must be a literal non-empty executable".into(),
                    ));
                }
                if let Some(parameter) = name.strip_prefix("param.") {
                    if !command.parameters.contains_key(parameter) {
                        let mut diagnostic = problem(
                            "unknown_parameter",
                            &command.template,
                            format!("undeclared parameter `{parameter}`"),
                        );
                        diagnostic.parameter = Some(parameter.into());
                        diagnostic.remedy = "Declare the referenced parameter in this command, or correct the reference.".into();
                        return Err(Box::new(diagnostic));
                    }
                    fragments.push(Fragment::Literal(std::mem::take(&mut literal)));
                    fragments.push(Fragment::Parameter(parameter.into()));
                    rest = &next[end + 1..];
                    continue;
                }
                let Some(slot) = slots.iter().position(|slot| slot.name == name) else {
                    return Err(invalid(format!("unknown substitution `${{{name}}}`")));
                };
                if rest.len() != word.len() || end + 3 != word.len() || index == 0 {
                    return Err(invalid("runtime substitutions must occupy a whole argument; word zero must be literal".into()));
                }
                counts[slot] += 1;
                slot_word = Some(Word::Slot(name.into()));
                rest = &next[end + 1..];
            } else {
                let Some(character) = rest.chars().next() else {
                    break;
                };
                literal.push(character);
                rest = &rest[character.len_utf8()..];
            }
        }
        if index == 0 && literal.is_empty() {
            return Err(invalid(
                "word zero must be a literal non-empty executable".into(),
            ));
        }
        result.push(if let Some(slot) = slot_word {
            NamedWord::Runtime(slot)
        } else {
            fragments.push(Fragment::Literal(literal));
            NamedWord::Fragments(fragments)
        });
    }
    for (slot, count) in slots.iter().zip(counts) {
        if !slot.requirement.admits(count) {
            return Err(invalid(slot.requirement.violation(&slot.name)));
        }
    }
    Ok(result)
}

enum Fragment {
    Literal(String),
    Parameter(String),
}

enum NamedWord {
    Runtime(Word),
    Fragments(Vec<Fragment>),
}

impl NamedWord {
    fn instantiate(&self, parameters: &[ParameterView], template_origin: usize) -> WordView {
        let mut origins = vec![template_origin];
        let word = match self {
            Self::Runtime(word) => word.clone(),
            Self::Fragments(fragments) => {
                let mut text = String::new();
                for fragment in fragments {
                    match fragment {
                        Fragment::Literal(literal) => text.push_str(literal),
                        Fragment::Parameter(name) => {
                            // Schema compilation and route completeness precede instantiation.
                            let parameter = parameters
                                .iter()
                                .find(|p| &p.name == name)
                                .expect("validated parameter");
                            text.push_str(&parameter.value);
                            for origin in &parameter.origins {
                                if !origins.contains(origin) {
                                    origins.push(*origin);
                                }
                            }
                        }
                    }
                }
                Word::Literal(text)
            }
        };
        WordView { word, origins }
    }
}

#[derive(Clone)]
enum Route {
    Literal(Template),
    Binding(Target),
}

impl Route {
    fn span(&self) -> &SourceSpan {
        match self {
            Self::Literal(template) => &template.span,
            Self::Binding(target) => &target.span,
        }
    }
}

type Resolved = (
    BTreeMap<String, Template>,
    BTreeMap<String, SourceSpan>,
    Inspection,
);

pub(super) fn resolve(captured: &Captured, selection: &Selection) -> Result<Resolved, ConfigError> {
    let mut view = Inspection {
        sources: Vec::new(),
        selection: selection.clone(),
        profile_occurrences: Vec::new(),
        commands: Vec::new(),
        non_admitted_keys: Vec::new(),
        origins: Vec::new(),
        histories: Vec::new(),
    };
    let mut bindings = BTreeMap::new();
    let mut routes = BTreeMap::new();
    let mut admitted = std::collections::BTreeSet::new();
    for document in std::iter::once(&captured.primary).chain(captured.overlay.iter()) {
        let primary = std::ptr::eq(document, &captured.primary);
        view.sources.push(super::Source {
            role: if primary {
                SourceRole::Primary
            } else {
                SourceRole::Overlay
            },
            path: document.path.clone(),
        });
        let mut declarations = Vec::new();
        for (name, command) in &document.named.commands {
            declarations.push((
                &command.template.span,
                None,
                AssignmentValue::Set(name.clone()),
            ));
            for (parameter, declaration) in &command.parameters {
                declarations.push((
                    &declaration.span,
                    declaration
                        .default
                        .as_ref()
                        .map(|_| Setting::ParameterDefault {
                            command: name.clone(),
                            parameter: parameter.clone(),
                        }),
                    AssignmentValue::Set(declaration.default.clone().unwrap_or_default()),
                ));
            }
        }
        for (binding, target) in &document.named.bindings {
            bindings.insert(binding.clone(), target.clone());
            declarations.push((
                &target.span,
                Some(Setting::BindingTarget {
                    binding: binding.clone(),
                }),
                AssignmentValue::Set(target.value.clone()),
            ));
        }
        for (key, target) in &document.named.routes {
            routes.insert(key.clone(), Route::Binding(target.clone()));
            if primary {
                admitted.insert(key.clone());
            }
            declarations.push((
                &target.span,
                Some(Setting::RouteTarget { key: key.clone() }),
                AssignmentValue::Set(target.value.clone()),
            ));
        }
        for (key, template) in &document.templates {
            routes.insert(key.clone(), Route::Literal(template.clone()));
            if primary {
                admitted.insert(key.clone());
            }
            declarations.push((
                &template.span,
                Some(Setting::RouteTarget { key: key.clone() }),
                AssignmentValue::LiteralTemplate(template.text.clone()),
            ));
        }
        declarations.sort_by_key(|(span, _, _)| span.start);
        for (span, setting, value) in declarations {
            let origin = view.origins.len();
            view.origins.push(Origin {
                id: origin,
                span: span.clone(),
                occurrence: None,
            });
            if let Some(setting) = setting {
                let assignment = Assignment {
                    order: origin,
                    value,
                    origin,
                };
                if let Some(history) = view.histories.iter_mut().find(|h| h.setting == setting) {
                    history.assignments.push(assignment);
                } else {
                    view.histories.push(AssignmentHistory {
                        id: 0,
                        setting,
                        assignments: vec![assignment],
                    });
                }
            }
        }
    }
    view.histories
        .sort_by(|a, b| setting_key(&a.setting).cmp(&setting_key(&b.setting)));
    for (id, history) in view.histories.iter_mut().enumerate() {
        history.id = id;
    }
    let definitions = &captured.primary.named.commands;
    let mut compiled = BTreeMap::new();
    let mut diagnostics = Vec::new();
    for (binding, target) in &bindings {
        let Some(command) = definitions.get(&target.value) else {
            let mut diagnostic = problem(
                "unknown_reference",
                target,
                format!(
                    "binding `{binding}` refers to unknown command `{}`",
                    target.value
                ),
            );
            diagnostic.binding = Some(binding.clone());
            diagnostic.command = Some(target.value.clone());
            diagnostics.push(diagnostic);
            continue;
        };
        if compiled.contains_key(&target.value) {
            continue;
        }
        match compile(command, &captured.slots) {
            Ok(words) => {
                compiled.insert(target.value.clone(), Some(words));
            }
            Err(mut diagnostic) => {
                diagnostic.command = Some(target.value.clone());
                diagnostic.binding = Some(binding.clone());
                diagnostic.related.push(target.span.clone());
                diagnostics.push(*diagnostic);
                compiled.insert(target.value.clone(), None);
            }
        }
    }
    let mut templates = BTreeMap::new();
    let mut overlay_only = BTreeMap::new();
    for (key, route) in routes {
        let route_history = history_id(&view, &Setting::RouteTarget { key: key.clone() });
        let route_origin = origin_id(&view, route.span());
        if !admitted.contains(&key) {
            overlay_only.insert(key.clone(), route.span().clone());
            view.non_admitted_keys.push(NonAdmittedKey {
                key,
                origins: view.histories[route_history]
                    .assignments
                    .iter()
                    .map(|a| a.origin)
                    .collect(),
                reason: "Only the overlay declares this key; primary policy must authorize it."
                    .into(),
            });
            continue;
        }
        let mut parameters = Vec::new();
        let mut resolved_words = None;
        let (template, binding, command, origins, histories) = match route {
            Route::Literal(template) => (
                template,
                None,
                None,
                vec![route_origin],
                vec![route_history],
            ),
            Route::Binding(route) => {
                let Some(binding) = bindings.get(&route.value) else {
                    let mut diagnostic = problem(
                        "unknown_reference",
                        &route,
                        format!("key `{key}` refers to unknown binding `{}`", route.value),
                    );
                    diagnostic.key = Some(key);
                    diagnostic.binding = Some(route.value);
                    diagnostics.push(diagnostic);
                    continue;
                };
                let Some(Some(words)) = compiled.get(&binding.value) else {
                    continue;
                };
                let definition = &definitions[&binding.value];
                let before = diagnostics.len();
                for (name, parameter) in &definition.parameters {
                    let category = match &parameter.default {
                        None => Some("missing_parameter"),
                        Some(value) if value.contains('\0') => Some("invalid_value"),
                        Some(_) => None,
                    };
                    if let Some(category) = category {
                        let target = Target {
                            value: String::new(),
                            span: parameter.span.clone(),
                        };
                        let mut diagnostic = problem(
                            category,
                            &target,
                            format!(
                                "key `{key}`, command `{}`, parameter `{name}`: {}",
                                binding.value,
                                if category == "missing_parameter" {
                                    "a value is required"
                                } else {
                                    "value contains NUL"
                                }
                            ),
                        );
                        diagnostic.related = vec![route.span.clone(), binding.span.clone()];
                        diagnostic.key = Some(key.clone());
                        diagnostic.binding = Some(route.value.clone());
                        diagnostic.command = Some(binding.value.clone());
                        diagnostic.parameter = Some(name.clone());
                        diagnostic.remedy = "Supply a NUL-free default string in the command's parameter declaration; parameter override patches are not yet supported.".into();
                        diagnostics.push(diagnostic);
                        continue;
                    }
                    parameters.push(ParameterView {
                        name: name.clone(),
                        value: parameter.default.clone().expect("checked default"),
                        origins: vec![origin_id(&view, &parameter.span)],
                        histories: vec![history_id(
                            &view,
                            &Setting::ParameterDefault {
                                command: binding.value.clone(),
                                parameter: name.clone(),
                            },
                        )],
                    });
                }
                if diagnostics.len() != before {
                    continue;
                }
                let words: Vec<_> = words
                    .iter()
                    .map(|word| {
                        word.instantiate(&parameters, origin_id(&view, &definition.template.span))
                    })
                    .collect();
                let origins = vec![
                    route_origin,
                    origin_id(&view, &binding.span),
                    origin_id(&view, &definition.template.span),
                ];
                let mut histories = vec![
                    route_history,
                    history_id(
                        &view,
                        &Setting::BindingTarget {
                            binding: route.value.clone(),
                        },
                    ),
                ];
                histories.extend(parameters.iter().flat_map(|p| p.histories.iter().copied()));
                let template = Template {
                    span: definition.template.span.clone(),
                    text: definition.template.value.clone(),
                    words: words.iter().map(|word| word.word.clone()).collect(),
                    source: definition.template.span.source.path.clone(),
                };
                resolved_words = Some(words);
                (
                    template,
                    Some(route.value),
                    Some(binding.value.clone()),
                    origins,
                    histories,
                )
            }
        };
        let template_origin = origin_id(&view, &template.span);
        view.commands.push(CommandView {
            key: key.clone(),
            binding,
            command,
            parameters,
            words: resolved_words.unwrap_or_else(|| {
                template
                    .words
                    .iter()
                    .map(|word| WordView {
                        word: word.clone(),
                        origins: vec![template_origin],
                    })
                    .collect()
            }),
            origins,
            histories,
        });
        templates.insert(key, template);
    }
    diagnostics.sort_by(|a, b| diagnostic_order(a).cmp(&diagnostic_order(b)));
    if !diagnostics.is_empty() {
        return Err(ConfigError::from_diagnostics(diagnostics));
    }
    Ok((templates, overlay_only, view))
}

fn setting_key(setting: &Setting) -> (u8, &str, &str) {
    match setting {
        Setting::RouteTarget { key } => (0, key, ""),
        Setting::BindingTarget { binding } => (1, binding, ""),
        Setting::ParameterDefault { command, parameter } => (2, command, parameter),
        _ => unreachable!("parameter patches are not yet supported"),
    }
}

fn diagnostic_order(diagnostic: &Diagnostic) -> (u8, usize, &Option<String>) {
    let role = diagnostic
        .source
        .as_ref()
        .map_or(0, |s| u8::from(s.role == SourceRole::Overlay));
    (
        role,
        diagnostic.primary.as_ref().map_or(0, |s| s.start),
        &diagnostic.key,
    )
}

fn origin_id(view: &Inspection, span: &SourceSpan) -> usize {
    view.origins
        .iter()
        .position(|origin| &origin.span == span)
        .expect("captured declaration has an origin")
}

fn history_id(view: &Inspection, setting: &Setting) -> usize {
    view.histories
        .iter()
        .position(|history| &history.setting == setting)
        .expect("captured target has a history")
}
