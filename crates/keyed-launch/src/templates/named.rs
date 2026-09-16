//! Named base commands: capture structure first, then resolve effective targets.
use super::{
    at_node, contains_shell_comment_start, source_location, Assignment, AssignmentHistory,
    AssignmentValue, BTreeMap, Captured, CommandView, ConfigError, Diagnostic, DocumentRole,
    Inspection, KdlDocument, KdlNode, NonAdmittedKey, Origin, Path, Selection, Setting, SlotSpec,
    SourceLocation, SourceRole, SourceSpan, Template, ValidationDiagnostic, Word, WordView,
};

#[derive(Default)]
pub(super) struct Declarations {
    commands: BTreeMap<String, Target>,
    bindings: BTreeMap<String, Target>,
    routes: BTreeMap<String, Target>,
}

#[derive(Clone)]
struct Target {
    value: String,
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
                || child.children().is_some_and(|c| !c.nodes().is_empty())
                || (kind == "bind" && child.children().is_some())
            {
                diagnostics.push(at_node(loc, format!("`{kind}` does not accept properties, types or child nodes; parameters are not yet supported")));
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
            let table = match kind {
                "command" => &mut result.commands,
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
            diagnostic.remedy = "Use config { command \"name\" \"template\"; bind \"binding\" \"name\"; route \"key\" \"binding\"; }; keep definitions in primary policy and omit parameters/profiles/selections.";
        }
    }
    result
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
fn compile(target: &Target, slots: &[SlotSpec]) -> Result<Vec<Word>, String> {
    let text = &target.value;
    if text.contains('\0') {
        return Err("command template contains NUL".into());
    }
    if contains_shell_comment_start(text) {
        return Err("quote literal comment-starting `#`".into());
    }
    let words = shell_words::split(text).map_err(|_| "command template has unmatched quotes")?;
    if words.is_empty() {
        return Err("word zero must be a literal non-empty executable".into());
    }
    let mut result = Vec::new();
    let mut counts = vec![0; slots.len()];
    for (index, word) in words.iter().enumerate() {
        let mut rest = word.as_str();
        let mut literal = String::new();
        let mut slot_word = None;
        while !rest.is_empty() {
            if let Some(next) = rest.strip_prefix("$$") {
                literal.push('$');
                rest = next;
            } else if let Some(next) = rest.strip_prefix("${") {
                let Some(end) = next.find('}') else {
                    return Err("unterminated substitution".into());
                };
                let name = &next[..end];
                if name.starts_with("param.") {
                    return Err("parameter substitution is not yet supported".into());
                }
                let Some(slot) = slots.iter().position(|slot| slot.name == name) else {
                    return Err(format!("unknown substitution `${{{name}}}`"));
                };
                if rest.len() != word.len() || end + 3 != word.len() || index == 0 {
                    return Err("runtime substitutions must occupy a whole argument; word zero must be literal".into());
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
            return Err("word zero must be a literal non-empty executable".into());
        }
        result.push(slot_word.unwrap_or(Word::Literal(literal)));
    }
    for (slot, count) in slots.iter().zip(counts) {
        if !slot.requirement.admits(count) {
            return Err(slot.requirement.violation(&slot.name));
        }
    }
    Ok(result)
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
            declarations.push((&command.span, None, AssignmentValue::Set(name.clone())));
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
            Err(message) => {
                let mut diagnostic = problem(
                    "invalid_template",
                    command,
                    format!("command `{}`: {message}", target.value),
                );
                diagnostic.command = Some(target.value.clone());
                diagnostic.related.push(target.span.clone());
                diagnostics.push(diagnostic);
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
                let origins = vec![
                    route_origin,
                    origin_id(&view, &binding.span),
                    origin_id(&view, &definition.span),
                ];
                let histories = vec![
                    route_history,
                    history_id(
                        &view,
                        &Setting::BindingTarget {
                            binding: route.value.clone(),
                        },
                    ),
                ];
                let template = Template {
                    span: definition.span.clone(),
                    text: definition.value.clone(),
                    words: words.clone(),
                    source: definition.span.source.path.clone(),
                };
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
            parameters: Vec::new(),
            words: template
                .words
                .iter()
                .map(|word| WordView {
                    word: word.clone(),
                    origins: vec![template_origin],
                })
                .collect(),
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

fn setting_key(setting: &Setting) -> (u8, &str) {
    match setting {
        Setting::RouteTarget { key } => (0, key),
        Setting::BindingTarget { binding } => (1, binding),
        _ => unreachable!("only target histories are created here"),
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
