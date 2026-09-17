# What a template must be
<!-- book-page id="template-law" slice="words-not-shell" order="4" -->
[Previous: Two documents and explicit targets](03-two-documents.md) | [Contents](README.md) | [Next: From a template to an argv](05-to-an-argv.md)

<a id="words-not-shell"></a>
## Words, not shell

Chapter 3 captures modular declarations and resolves their targets. This chapter
explains how an effective command definition becomes parsed argument fragments,
why comment starts are refused before splitting, and how structural findings
become source-attributed diagnostics.

A command template uses POSIX quoting to delimit words. No shell evaluates it:
no environment variable expands, no glob matches, and no command substitution,
redirection or pipeline runs. Runtime slots come from the consumer's vocabulary
and must occupy whole arguments. Author-declared parameters may fill fragments
inside arguments, but their values are inserted after splitting and never parsed
again. Both kinds of substitution preserve argument boundaries.

<a id="the-rules"></a>
## Every rule, and the line that binds it

The modular parser checks declaration shape and duplicate names during capture.
The compiler checks definitions reached by effective bindings during resolution.
An inactive definition can therefore remain unfinished while active commands
must compile before a Templates snapshot exists.

| Rule | Enforced at | Public-seam evidence |
|---|---|---|
| Every top-level declaration is a modular wrapper | `named::parse` | `both_loaders_reject_flat_and_mixed_input_in_either_source` |
| Wrapper and declaration shapes and namespaces are valid | `named::parse`, `parse_scope` and their helpers | `structural_reports_aggregate_in_source_order_with_real_utf8_ranges` |
| A template contains no NUL or unquoted comment start | `named::compile` | `named_commands_preserve_native_runtime_values_and_refuse_nul`, `an_unquoted_hash_is_refused_rather_than_truncating_the_argv` |
| Quotes close and word zero is literal and nonempty | `named::compile` | `unmatched_quotes_are_refused`, `word_zero_must_be_a_literal_executable` |
| A runtime slot is declared and occupies a whole argument | `named::compile` | `template_failures_are_aggregated_with_source_locations` |
| Slot counts satisfy the consumer's requirements | `named::compile` | `template_failures_are_aggregated_with_source_locations` |
| Parameter references name declarations in this command | `named::compile` | `active_parameter_errors_are_classified_without_cascading_missing_values` |

The diagnostic contract is the same at both stages: name the failure, identify
its source, and give a remedy. Structural failure prevents semantic validation
from manufacturing cascades from malformed declarations.

<a id="the-rules-on-one-line"></a>
## A command reaching the compiler

A base-only configuration can define, bind and route one command:

```kdl
config {
    command "assistant" "claude --model opus ${prompt}"
    bind "lead" "assistant"
    route "impl" "lead"
}
```

For a vocabulary requiring `prompt` exactly once, the active definition compiles
to three literal words and one runtime slot. Adding `#` before `${prompt}` would
make the shell-word splitter discard that slot. The compiler instead refuses
the comment start at the command definition's span. Quoting it as `'#tag'`, or
placing it inside `mid#word`, passes a literal hash to the child.

<a id="the-scan"></a>
## Why the scan cannot be a `contains`

`contains_shell_comment_start` receives template text before splitting and
answers only whether a comment would truncate it. Its states distinguish word
boundaries, quoted regions and escapes. A hash at `Delimiter` is a comment;
in the other states it is literal. A delimiter escape followed by a newline
returns to `Delimiter`, matching a continued line rather than inventing a word.
A plain substring check would also reject quoted hashes, including the remedy
that the diagnostic recommends.

<!-- fragment «word-scanning» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="555-604" parent="source-templates" -->
````rust

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

````
<!-- /fragment -->

The scanner does not validate quote closure; the subsequent shell-word split
owns that error. The public hash tests exercise both refusal and literal forms.
The same scanner serves modular compilation after removal of the flat grammar.

<a id="what-a-refusal-owes"></a>
## Name what is wrong, name where, name what fixes it

`at_node` creates an internal shape finding with a modular-form remedy. The
modular parser replaces that remedy for errors inside a wrapper and attaches
related spans for duplicates. `render_diagnostics` orders findings by byte
position and key, then attaches the explicit source role and path. The result
is a vector of public Diagnostic records for Catalog to aggregate across files.

`source_location` keeps byte offsets for structured spans and counts Unicode
characters for human columns. `format_location` adds the path without reopening
it. Every location continues to describe the immutable captured bytes, even if
the file is later edited or removed.

<!-- fragment «diagnostics» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="605-675" parent="source-templates" -->
````rust
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
````
<!-- /fragment -->

<a id="named-compile"></a>
## Named dollar scanning

`compile` receives one command definition and the captured vocabulary. It first
refuses NUL and comment starts, then splits once. A left-to-right dollar scan
turns each word into literal/parameter fragments or a whole runtime slot.
`$$` contributes one literal dollar without rescanning it, so `$${prompt}`
remains text and cannot satisfy a required slot count. Unknown and unterminated
substitutions fail; substitutions in word zero fail before an executable can
become dynamic. The final count check applies each consumer requirement.

`NamedWord::instantiate` later fills parameter fragments from resolved values
while preserving each word's contributing parameter names for inspection.
Runtime slots remain symbolic until expansion receives native operating-system
strings. A parameter containing spaces or dollar syntax is text at this point,
so it cannot add arguments or introduce another substitution.

<!-- fragment «named-compile» owner="words-not-shell" source="crates/keyed-launch/src/templates/named.rs" lines="551-688" parent="source-named" -->
````rust
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

````
<!-- /fragment -->

[Previous: Two documents and explicit targets](03-two-documents.md) | [Contents](README.md) | [Next: From a template to an argv](05-to-an-argv.md)
