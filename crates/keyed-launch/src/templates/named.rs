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
    pub(super) selection: Option<Selection>,
    commands: BTreeMap<String, Command>,
    bindings: BTreeMap<String, Target>,
    routes: BTreeMap<String, RoutePatch>,
    values: BTreeMap<String, Values>,
}

struct RoutePatch {
    declaration: Target,
    binding: Option<String>,
    parameters: BTreeMap<String, ParameterPatch>,
}

struct Values {
    target: Target,
    parameters: BTreeMap<String, ParameterPatch>,
}

struct ParameterPatch {
    value: Option<String>,
    span: SourceSpan,
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
            if kind == "select" {
                parse_selection(
                    path,
                    source,
                    child,
                    role,
                    &mut result.selection,
                    diagnostics,
                );
                continue;
            }
            if kind == "values" {
                parse_values(path, source, child, role, &mut result.values, diagnostics);
                continue;
            }
            if !matches!(kind, "command" | "bind" | "route") {
                diagnostics.push(at_node(loc, format!("unsupported config node `{kind}`; profile definitions are not yet supported")));
                continue;
            }
            let values: Option<Vec<_>> = child
                .entries()
                .iter()
                .map(|entry| entry.value().as_string())
                .collect();
            let Some(values) = values.filter(|v| v.len() == 2 || (kind == "route" && v.len() == 1))
            else {
                diagnostics.push(at_node(
                    loc,
                    format!("`{kind}` requires two string arguments (route permits just its key)"),
                ));
                continue;
            };
            if !plain(child) || (kind == "bind" && child.children().is_some()) {
                diagnostics.push(at_node(loc, format!("`{kind}` does not accept properties or types; commands accept declarations and routes accept parameter patches")));
                continue;
            }
            let name = values[0];
            if (kind == "route" && name.is_empty())
                || (kind != "route" && !valid_name(name))
                || (kind != "command" && values.get(1).is_some_and(|name| !valid_name(name)))
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
                value: values.get(1).unwrap_or(&name).to_string(),
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
                    result.routes.insert(
                        name.into(),
                        RoutePatch {
                            declaration: target,
                            binding: values.get(1).map(|name| (*name).to_owned()),
                            parameters: parse_patches(path, source, child, role, diagnostics),
                        },
                    );
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
            diagnostic.remedy = "Use command declarations, bind/route targets, values blocks and at most one select list inside config; select takes only valid profile-name strings. Keep definitions in primary policy; omit profile definitions.";
        }
    }
    result
}

fn parse_selection(
    path: &Path,
    source: &str,
    node: &KdlNode,
    role: DocumentRole,
    selection: &mut Option<Selection>,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let loc = location(source, node);
    let names: Option<Vec<_>> = node
        .entries()
        .iter()
        .map(|entry| entry.value().as_string())
        .collect();
    let Some(names) = names.filter(|names| {
        plain(node) && node.children().is_none() && names.iter().all(|name| valid_name(name))
    }) else {
        diagnostics.push(at_node(loc, "select requires zero or more valid profile-name strings, without properties, types or children".into()));
        return;
    };
    if let Some(previous) = selection
        .as_ref()
        .and_then(|selection| selection.origin.as_ref())
    {
        let mut earlier = source_location(source, previous.start);
        earlier.end = previous.end;
        duplicate("select", earlier, loc, diagnostics);
        return;
    }
    *selection = Some(Selection {
        profiles: names.into_iter().map(str::to_owned).collect(),
        origin: Some(SourceSpan {
            source: role.source(path),
            start: loc.start,
            end: loc.end,
        }),
    });
}

fn parse_values(
    path: &Path,
    source: &str,
    node: &KdlNode,
    role: DocumentRole,
    values: &mut BTreeMap<String, Values>,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let loc = location(source, node);
    let name = node
        .entries()
        .first()
        .and_then(|entry| entry.value().as_string());
    let Some(name) = name.filter(|name| valid_name(name)) else {
        diagnostics.push(at_node(loc, "values requires a valid command name".into()));
        return;
    };
    if !plain(node) || node.entries().len() != 1 || node.children().is_none() {
        diagnostics.push(at_node(
            loc,
            "values requires one command name and a child block, without properties or types"
                .into(),
        ));
        return;
    }
    let parameters = parse_patches(path, source, node, role, diagnostics);
    let target = Target {
        value: name.into(),
        span: SourceSpan {
            source: role.source(path),
            start: loc.start,
            end: loc.end,
        },
    };
    if let Some(previous) = values.insert(name.into(), Values { target, parameters }) {
        let mut earlier = source_location(source, previous.target.span.start);
        earlier.end = previous.target.span.end;
        duplicate(name, earlier, loc, diagnostics);
    }
}

fn parse_patches(
    path: &Path,
    source: &str,
    node: &KdlNode,
    role: DocumentRole,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> BTreeMap<String, ParameterPatch> {
    let mut parameters: BTreeMap<String, ParameterPatch> = BTreeMap::new();
    for child in node.children().into_iter().flat_map(KdlDocument::nodes) {
        let child_loc = location(source, child);
        let kind = child.name().value();
        let args: Option<Vec<_>> = child
            .entries()
            .iter()
            .map(|entry| entry.value().as_string())
            .collect();
        let Some(args) = args.filter(|args| {
            plain(child)
                && child.children().is_none()
                && ((kind == "param" && args.len() == 2) || (kind == "unset" && args.len() == 1))
                && valid_name(args[0])
        }) else {
            diagnostics.push(at_node(child_loc, "parameter patches require param with a name and value string, or unset with a name; no properties, types or children".into()));
            continue;
        };
        let patch = ParameterPatch {
            value: args.get(1).map(|value| (*value).to_owned()),
            span: SourceSpan {
                source: role.source(path),
                start: child_loc.start,
                end: child_loc.end,
            },
        };
        if let Some(previous) = parameters.insert(args[0].into(), patch) {
            let mut earlier = source_location(source, previous.span.start);
            earlier.end = previous.span.end;
            duplicate(args[0], earlier, child_loc, diagnostics);
        }
    }
    parameters
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
    Missing(Target),
    Literal(Template),
    Binding(Target),
}

impl Route {
    fn span(&self) -> &SourceSpan {
        match self {
            Self::Literal(template) => &template.span,
            Self::Binding(target) | Self::Missing(target) => &target.span,
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
    let mut value_targets = BTreeMap::new();
    let mut shared: BTreeMap<String, BTreeMap<String, Target>> = BTreeMap::new();
    let mut route_values: BTreeMap<String, BTreeMap<String, Target>> = BTreeMap::new();
    let mut literal_patches = BTreeMap::new();
    let mut diagnostics = Vec::new();
    let mut assignment_order = 0;
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
        for (command, values) in &document.named.values {
            value_targets.insert(command.clone(), values.target.clone());
            declarations.push((
                &values.target.span,
                None,
                AssignmentValue::Set(command.clone()),
            ));
            let effective = shared.entry(command.clone()).or_default();
            for (name, patch) in &values.parameters {
                let value = if let Some(value) = &patch.value {
                    effective.insert(
                        name.clone(),
                        Target {
                            value: value.clone(),
                            span: patch.span.clone(),
                        },
                    );
                    AssignmentValue::Set(value.clone())
                } else {
                    effective.remove(name);
                    AssignmentValue::Unset
                };
                declarations.push((
                    &patch.span,
                    Some(Setting::CommandParameter {
                        command: command.clone(),
                        parameter: name.clone(),
                    }),
                    value,
                ));
            }
        }
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
        for (key, patch) in &document.named.routes {
            let effective = route_values.entry(key.clone()).or_default();
            if let Some(binding) = &patch.binding {
                if matches!(routes.get(key), Some(Route::Literal(_))) {
                    effective.clear();
                    literal_patches.remove(key);
                }
                routes.insert(
                    key.clone(),
                    Route::Binding(Target {
                        value: binding.clone(),
                        span: patch.declaration.span.clone(),
                    }),
                );
                if primary {
                    admitted.insert(key.clone());
                }
                declarations.push((
                    &patch.declaration.span,
                    Some(Setting::RouteTarget { key: key.clone() }),
                    AssignmentValue::Set(binding.clone()),
                ));
            } else {
                routes
                    .entry(key.clone())
                    .or_insert_with(|| Route::Missing(patch.declaration.clone()));
                declarations.push((&patch.declaration.span, None, AssignmentValue::Unset));
                if primary {
                    let mut diagnostic = problem(
                        "missing_target",
                        &patch.declaration,
                        format!(
                            "key `{key}` has a personal parameter patch but no personal target"
                        ),
                    );
                    diagnostic.key = Some(key.clone());
                    diagnostic.remedy = "Add an explicit target for this key in personal policy, or remove the personal parameter patch; a local target cannot authorize it.".into();
                    diagnostics.push(diagnostic);
                }
                if matches!(routes.get(key), Some(Route::Literal(_))) {
                    literal_patches.insert(key.clone(), patch.declaration.clone());
                }
            }
            for (name, parameter) in &patch.parameters {
                let value = if let Some(value) = &parameter.value {
                    effective.insert(
                        name.clone(),
                        Target {
                            value: value.clone(),
                            span: parameter.span.clone(),
                        },
                    );
                    AssignmentValue::Set(value.clone())
                } else {
                    effective.remove(name);
                    AssignmentValue::Unset
                };
                declarations.push((
                    &parameter.span,
                    Some(Setting::RouteParameter {
                        key: key.clone(),
                        parameter: name.clone(),
                    }),
                    value,
                ));
            }
        }
        for (key, template) in &document.templates {
            route_values.remove(key);
            literal_patches.remove(key);
            for history in &view.histories {
                if matches!(&history.setting, Setting::RouteParameter { key: owner, .. } if owner == key)
                {
                    declarations.push((
                        &template.span,
                        Some(history.setting.clone()),
                        AssignmentValue::Reset,
                    ));
                }
            }
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
            let origin = if let Some(origin) = view.origins.iter().find(|o| &o.span == span) {
                origin.id
            } else {
                let id = view.origins.len();
                view.origins.push(Origin {
                    id,
                    span: span.clone(),
                    occurrence: None,
                });
                id
            };
            if let Some(setting) = setting {
                let assignment = Assignment {
                    order: assignment_order,
                    value,
                    origin,
                };
                assignment_order += 1;
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
    let mut invalid_values = std::collections::BTreeSet::new();
    for (name, target) in &value_targets {
        let Some(command) = definitions.get(name) else {
            let mut diagnostic = problem(
                "unknown_reference",
                target,
                format!("values refers to unknown command `{name}`"),
            );
            diagnostic.command = Some(name.clone());
            diagnostic.remedy =
                "Define this command in primary policy, or correct the values target.".into();
            diagnostics.push(diagnostic);
            continue;
        };
        for (parameter, value) in &shared[name] {
            let category = if !command.parameters.contains_key(parameter) {
                Some("unknown_parameter")
            } else if value.value.contains('\0') {
                Some("invalid_value")
            } else {
                None
            };
            if let Some(category) = category {
                let mut diagnostic = problem(
                    category,
                    value,
                    format!(
                        "command `{name}`, shared parameter `{parameter}`: {}",
                        if category == "unknown_parameter" {
                            "parameter is not declared"
                        } else {
                            "value contains NUL"
                        }
                    ),
                );
                diagnostic.command = Some(name.clone());
                diagnostic.parameter = Some(parameter.clone());
                diagnostic.related.push(command.template.span.clone());
                diagnostic.remedy = "Supply a NUL-free value for a declared parameter, or unset the shared override.".into();
                diagnostics.push(diagnostic);
                invalid_values.insert(name.clone());
            }
        }
    }
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
        let route_origin = origin_id(&view, route.span());
        if !admitted.contains(&key) {
            overlay_only.insert(key.clone(), route.span().clone());
            view.non_admitted_keys.push(NonAdmittedKey {
                key: key.clone(),
                origins: std::iter::once(route_origin).chain(view.histories.iter()
                    .filter(|h| matches!(&h.setting, Setting::RouteParameter { key: owner, .. } if owner == &key))
                    .flat_map(|h| h.assignments.iter().map(|a| a.origin))).collect(),
                reason: "Only the overlay declares this key; primary policy must authorize it.".into(),
            });
            continue;
        }
        let route_history = history_id(&view, &Setting::RouteTarget { key: key.clone() });
        let route_histories: Vec<_> = view.histories.iter()
            .filter(|h| matches!(&h.setting, Setting::RouteParameter { key: owner, .. } if owner == &key))
            .map(|h| h.id).collect();
        if let Some(patch) = literal_patches.get(&key) {
            let mut diagnostic = problem(
                "invalid_value",
                patch,
                format!("key `{key}` has a parameter patch but still uses a literal template"),
            );
            diagnostic.key = Some(key.clone());
            diagnostic.related.push(route.span().clone());
            diagnostic.remedy = "Set a binding target for this route, or replace the whole literal template without a parameter patch.".into();
            diagnostics.push(diagnostic);
            continue;
        }
        let mut parameters = Vec::new();
        let mut resolved_words = None;
        let (template, binding, command, origins, mut histories) = match route {
            Route::Missing(_) => continue,
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
                if invalid_values.contains(&binding.value) {
                    continue;
                }
                let overrides = route_values.get(&key);
                let before = diagnostics.len();
                for (name, value) in overrides.into_iter().flatten() {
                    let category = if !definition.parameters.contains_key(name) {
                        Some("unknown_parameter")
                    } else if value.value.contains('\0') {
                        Some("invalid_value")
                    } else {
                        None
                    };
                    if let Some(category) = category {
                        let mut diagnostic = problem(
                            category,
                            value,
                            format!(
                                "key `{key}`, command `{}`, route parameter `{name}`: {}",
                                binding.value,
                                if category == "unknown_parameter" {
                                    "parameter is not declared"
                                } else {
                                    "value contains NUL"
                                }
                            ),
                        );
                        diagnostic.key = Some(key.clone());
                        diagnostic.binding = Some(route.value.clone());
                        diagnostic.command = Some(binding.value.clone());
                        diagnostic.parameter = Some(name.clone());
                        diagnostic.related =
                            vec![route.span.clone(), definition.template.span.clone()];
                        diagnostic.remedy = "Supply a NUL-free value for a parameter in the final command, or unset the route override.".into();
                        diagnostics.push(diagnostic);
                    }
                }
                if diagnostics.len() != before {
                    continue;
                }
                let before = diagnostics.len();
                for (name, parameter) in &definition.parameters {
                    let assigned = overrides.and_then(|values| values.get(name)).or_else(|| {
                        shared
                            .get(&binding.value)
                            .and_then(|values| values.get(name))
                    });
                    let value = assigned
                        .map(|target| &target.value)
                        .or(parameter.default.as_ref());
                    let category = match value {
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
                        diagnostic.remedy = "Supply a NUL-free declaration default, shared values assignment or route override in primary or local configuration.".into();
                        diagnostics.push(diagnostic);
                        continue;
                    }
                    let mut origins = vec![origin_id(&view, &parameter.span)];
                    if let Some(assigned) = assigned {
                        origins.push(origin_id(&view, &assigned.span));
                    }
                    let settings = [
                        Setting::RouteParameter {
                            key: key.clone(),
                            parameter: name.clone(),
                        },
                        Setting::ParameterDefault {
                            command: binding.value.clone(),
                            parameter: name.clone(),
                        },
                        Setting::CommandParameter {
                            command: binding.value.clone(),
                            parameter: name.clone(),
                        },
                    ];
                    let histories = settings
                        .iter()
                        .filter_map(|setting| {
                            view.histories
                                .iter()
                                .find(|h| &h.setting == setting)
                                .map(|h| h.id)
                        })
                        .collect();
                    parameters.push(ParameterView {
                        name: name.clone(),
                        value: value.expect("checked value").clone(),
                        origins,
                        histories,
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
        for history in route_histories {
            if !histories.contains(&history) {
                histories.push(history);
            }
        }
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
        Setting::CommandParameter { command, parameter } => (3, command, parameter),
        Setting::RouteParameter { key, parameter } => (4, key, parameter),
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
