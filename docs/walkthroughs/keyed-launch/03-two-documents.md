# Two documents and explicit targets
<!-- book-page id="two-documents" slice="never-assembled" order="3" -->
[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)

<a id="never-assembled"></a>
## Capture once, resolve without reading

`Catalog::load` reads the explicit primary and optional overlay, validates document structure and retains original bytes, parsed KDL, eager flat
commands and named declarations.
`Catalog::resolve` folds binding and route targets, applies primary authority,
and compiles effective named commands from those captured declarations. `Templates::load` is the convenience composition
of those operations with an empty selection. None of these operations decides
which files a consumer should supply.

The running example resolves each key to one whole template. Capture preserves
both original declarations when an overlay replaces a command, and Templates
owns its snapshot independently. Changing or removing a file cannot change
expansion, inspection or diagnostic locations. Inspection explains flat commands and named reference chains. Parameter
composition and profiles remain pending.

<a id="one-entry-point"></a>
## One validation path

Both public loading routes reach Catalog's validator. The constructor checks
the vocabulary, then captures both explicit document results even when the
primary fails. It reports read, syntax, shape and duplicate failures before
template-semantic failures. With valid structure, unused flat templates still
undergo eager checking. Errors are ordered by primary/overlay, byte position
and key; a bad document never causes fallback to another configuration.

<!-- fragment «templates-load» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="165-278" parent="source-templates" -->
<!-- insert «templates-load-three-promises» -->
<!-- insert «templates-load-primary» -->
<!-- insert «templates-load-overlay» -->
<!-- insert «templates-load-value» -->
<!-- /fragment -->

The opening fragment states the eager flat validation and fail-closed contract.
Its output is a Catalog rather than an already merged map.

<!-- fragment «templates-load-three-promises» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="165-174" parent="templates-load" -->
````rust
impl Catalog {
    /// Capture both documents with structural and eager legacy validation.
    /// Named command templates validate only when an effective binding uses them,
    /// after resolution folds local target replacements. An invalid input fails
    /// closed rather than falling back to another policy.
    pub fn load(
        primary: &Path,
        overlay: Option<&Path>,
        vocabulary: Vocabulary<'_>,
    ) -> Result<Self, ConfigError> {
````
<!-- /fragment -->

The next fragment captures the documents and implements selection accessors and
the start of resolution. Flat documents have no selection declaration, so both
accessors return `None`. Any nonempty explicit selection names an unknown
profile and fails. Every selected entry has a diagnostic occurrence with its
zero-based selection index; external selections have no invented span. An empty
selection resolves the captured base declarations.
The consumer chooses that list, even when the consumer is the convenience loader.

<!-- fragment «templates-load-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="175-239" parent="templates-load" -->
````rust
        let slots = compile_vocabulary(&vocabulary)?;

        let primary_result = read_primary(primary)
            .and_then(|text| parse_and_validate(primary, text, DocumentRole::Primary, &slots));
        let overlay_result = overlay
            .map(|path| {
                read_overlay(path)
                    .and_then(|text| parse_and_validate(path, text, DocumentRole::Overlay, &slots))
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
            // Structure must pass before semantic reports are meaningful.
            if diagnostics.iter().any(|d| d.category != "invalid_template") {
                diagnostics.retain(|d| d.category != "invalid_template");
            }
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

    /// Base-only documents contain no profile selection declaration.
    #[must_use]
    pub fn primary_selection(&self) -> Option<&Selection> {
        None
    }

    /// Base-only overlays contain no profile selection declaration.
    #[must_use]
    pub fn overlay_selection(&self) -> Option<&Selection> {
        None
    }

    /// Fold captured base targets with primary authority and local replacement.
    /// Validate effective references and return an independently owned snapshot.
    pub fn resolve(&self, selection: &Selection) -> Result<Templates, ConfigError> {
        if !selection.profiles.is_empty() {
            let diagnostics = selection.profiles.iter().enumerate().map(|(index, profile)| {
                let mut diagnostic = Diagnostic::new("unknown_profile", format!(
                    "unknown profile `{profile}` at selection index {index}; base configuration declares no profiles."
                ), "Resolve with an empty selection; base files declare no profiles.");
                diagnostic.primary.clone_from(&selection.origin);
                diagnostic.source = selection.origin.as_ref().map(|span| span.source.clone());
                diagnostic.occurrence_chain.push(Occurrence {
                    id: index, profile: profile.clone(), parent: None,
                    selection_index: index, via: selection.origin.clone(),
                });
                diagnostic
            }).collect();
            return Err(ConfigError::from_diagnostics(diagnostics));
        }
````
<!-- /fragment -->

<a id="overrides-never-supplies"></a>
## An overlay overrides and never supplies

Resolution delegates to the private named resolver, which handles both literal
and binding targets. Primary declarations authorize keys before local replacements;
a local-only route is retained for an explanatory refusal. The declaration fold
and reference validation are explained [below](#named-fold).

<!-- fragment «templates-load-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="240-240" parent="templates-load" -->
````rust
        let (templates, overlay_only, inspection) = named::resolve(&self.captured, selection)?;
````
<!-- /fragment -->

The resolved value owns the winner map and slot table and shares captured inputs
through `Arc`. `slot_names` lends the captured vocabulary to conformance.
Templates' convenience constructor delegates directly to the same Catalog path,
so it cannot drift into a second reader or different vocabulary rules.

The resolver records original declarations before projecting the admitted commands.
Origins follow primary then overlay source order, while histories retain replaced
targets. Empty documents still appear in `sources`. The returned Templates owns
that explanation alongside compiled words, without reopening any source path.

<!-- fragment «templates-load-value» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="241-278" parent="templates-load" -->
````rust

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

````
<!-- /fragment -->

<a id="both-documents"></a>
## Two files, one captured result

For a primary declaring `impl` and `review-impl`, and an overlay replacing only
`impl`, resolution keeps the primary review command and the overlay impl command.
`source("impl")` names the overlay; `source("review-impl")` names the primary.
`validate_node` and `validate_template` are the per-node and per-template rule
checks `validate_document` drives over both documents; each returns diagnostics
with locations rather than stopping at the first. Chapter 4 explains those rules.

`crates/keyed-launch/tests/catalog.rs` uses the unrelated key `opaque` and runtime
slot `payload` to test the same boundary. It edits the primary and removes the
overlay before resolution and conformance, then removes the primary and drops
Catalog before expansion. Exact expected words prove both snapshot independence
and equality with the convenience loader; native non-UTF-8 slot bytes survive.

<a id="the-second-ending"></a>
## A local declaration cannot admit a key

A valid overlay-only command is retained but does not appear in `keys()` and has
no `source()`. `require` and `expand` refuse it, naming the primary file where the
key must be declared. Validation and admission are distinct: an invalid
overlay-only command still prevents loading, because flat template checks are
eager in both documents. A valid one lets other admitted keys resolve normally.

<a id="the-slot-table-first"></a>
## The slot table first, and the duplicate it refuses

The rest of the chapter is the five functions `load` reaches, in the order it
reaches them. They are free functions rather than methods because none of them
needs a `Templates` — they run before one exists — and all five are private:
`Catalog::load` is their entry point; Catalog resolution then produces
Templates without invoking them again. The block is read in seven fragments.

<!-- fragment «reading-and-whole-document-validation» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="461-669" parent="source-templates" -->
<!-- insert «compile-vocabulary» -->
<!-- insert «read-primary» -->
<!-- insert «read-overlay» -->
<!-- insert «parse-and-validate» -->
<!-- insert «validate-document-nodes» -->
<!-- insert «validate-document-duplicates» -->
<!-- insert «validate-document-report» -->
<!-- /fragment -->

`compile_vocabulary` is the first, and it validates names before copying them: it
copies the consumer's borrowed `SlotRule`s into the owned `SlotSpec` table that
`Templates` will keep, and it refuses duplicate names and names in the reserved `param.` namespace. Its comment is the only
statement anywhere in the crate of why the duplicate is a refusal rather than a
tolerated redundancy, and the reason is that the failure it would otherwise cause
is silent and lands on the wrong file.

<!-- fragment «compile-vocabulary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="461-498" parent="reading-and-whole-document-validation" -->
````rust

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
````
<!-- /fragment -->

The check is a linear scan over the slots accumulated so far, which is quadratic
in the vocabulary's size and is the right shape for a table with four entries in
grove's case and no plausible consumer with hundreds. It fires before the
filesystem is touched, and its message names the offending slot:
``the slot vocabulary declares `prompt` more than once``. Note what it is *not* —
there is no location, because a vocabulary is a `const` array in the consumer's
own source rather than a document this crate read, and the crate has no offset to
report.

The comment's second paragraph is the argument this section owes, and it is the
kind the source states only here. A duplicated slot would be counted twice
against its own cardinality rule: `validate_template` walks the slot table and
asks each entry's `Requirement` whether the occurrence count admits it, so a
`prompt` declared twice would be asked twice about the same one occurrence, and a
template containing `${prompt}` once would be refused by the second copy under
`ExactlyOnce` or accepted twice under `AtMostOnce`. At expansion the same
duplication makes `match_values` — chapter 5's — resolve the name to the *first*
matching index, so a value offered for `prompt` fills one of the two positions
and the other reports itself unfilled. Both symptoms name a template, or a value,
and neither names the vocabulary. That is the phrase the comment ends on: *a
consumer bug that looks like a template bug for as long as it goes unnamed*.
`a_duplicated_slot_name_is_refused_at_load` is what holds it, and it is the only
test in the file that builds its own vocabulary rather than using the shared one.

<a id="optional-at-the-filesystem"></a>
## What *optional* means at the filesystem

The two reading functions are thirteen lines and eight, and neither carries a
comment. They are worth reading side by side: the difference between them is the
whole of what *the overlay is optional* means once a path has been handed in, and
the source nowhere says so.

<!-- fragment «read-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="499-523" parent="reading-and-whole-document-validation" -->
````rust

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
````
<!-- /fragment -->

`read_primary` distinguishes one `io::ErrorKind` and folds every other into a
second message. `NotFound` gets ``configuration is missing at <path>``, which
tells an operator that the file they have not written yet is the thing to write;
`a_missing_primary_names_its_path` requires both the sentence and the path. Every
other kind — a permission denial, a directory where a file was expected, an I/O
error — gets ``failed to read the configuration at <path>: <error>``, with the
underlying error interpolated rather than classified. That is the whole taxonomy:
one case named, everything else reported verbatim.

The overlay's reader has no such case.

<!-- fragment «read-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="524-543" parent="reading-and-whole-document-validation" -->
````rust

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
````
<!-- /fragment -->

Every failure is the same failure, including `NotFound`. The asymmetry is the
answer to a question the signature raises and does not settle: `load` takes
`Option<&Path>`, so *optional* has already been decided one level up. A consumer
that has no overlay passes `None` and `read_overlay` is never called. A consumer
that passes `Some(path)` has asserted that a file is there, and a missing file at
an asserted path is a broken assertion rather than an absence.
`an_unreadable_overlay_fails_closed` pins exactly that: it passes a path inside a
temporary directory that was never written and requires
``failed to read the configuration overlay at`` in the error. The optionality
lives in the `Option`, and nowhere else — which is what lets the fallback
question be answered once, in `load`'s comment, rather than per error kind here.

<a id="parsed-then-validated"></a>
## Parsed, then validated

`parse_and_validate` is the seam between the two document formats the crate
depends on and the rules it adds on top of them. It is nineteen lines, and eight
of them are the message it builds when the parse fails.

<!-- fragment «parse-and-validate» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="544-582" parent="reading-and-whole-document-validation" -->
````rust

fn parse_and_validate(
    path: &Path,
    source: String,
    role: DocumentRole,
    slots: &[SlotSpec],
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

    let (templates, named) = validate_document(path, &source, &document, role, slots)?;
    Ok(CapturedDocument {
        path: path.to_path_buf(),
        _source: source,
        _document: document,
        templates,
        named,
    })
}
````
<!-- /fragment -->

The parse is `source.parse::<KdlDocument>()`, and the crate adds one thing to
`kdl`'s own error: a position a human can act on. `kdl` reports a byte offset in
`error.span`; `source_location` converts it into a one-based line and column, and
the message is assembled as `path:line:column: KDL syntax error: <error>`, with
the crate's location first and the parser's own text last.
`a_kdl_syntax_error_names_its_source_location` requires both the phrase and a
`:1:` in the result.

Three functions chapter 4 owns are used here and in the next section, and the
minimum to carry until then is small. `source_location` turns a byte offset into
a one-based line and column by counting newlines before the offset and characters
since the last one; `format_location` renders one as `path:line:column`; and
`render_diagnostics` assembles a whole refusal from a document's path, its role
and its list of diagnostics. Chapter 4 explains all three together, as the
machinery behind *name what is wrong, name where, name what fixes it*.

On success, `CapturedDocument` retains the source String, parsed KDL document,
and validated map together. The parsed spans therefore still refer to the exact
bytes that were read, even after the path changes.

A syntax error stops validation of that document because there is no parsed
node list to walk. Its parser byte range becomes a `kdl_syntax` diagnostic.
Catalog still collects errors from the other explicit document. DocumentRole
attaches source identity to syntax and node reports. It also restricts named
command definitions to primary policy.

<a id="one-refusal"></a>
## Every diagnostic in one refusal

`validate_document` is the last function in the block and the one that decides
what a refusal looks like. It runs in three passes over one document, and the
three fragments below are those passes. The first walks flat nodes, recording
validation and each key’s declaration locations. Wrapper nodes are captured by
`named::parse`, whose duplicate checks share the flat route namespace.

<!-- fragment «validate-document-nodes» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="583-605" parent="reading-and-whole-document-validation" -->
````rust

fn validate_document(
    path: &Path,
    source: &str,
    document: &KdlDocument,
    role: DocumentRole,
    slots: &[SlotSpec],
) -> Result<(BTreeMap<String, Template>, named::Declarations), ConfigError> {
    let mut validations = Vec::new();
    let mut occurrences: HashMap<String, Vec<SourceLocation>> = HashMap::new();

    for node in document.nodes() {
        if named::is_wrapper(node) {
            continue;
        }
        let validation = validate_node(source, node, slots);
        occurrences
            .entry(validation.key.clone())
            .or_default()
            .push(validation.location);
        validations.push(validation);
    }

````
<!-- /fragment -->

Nothing is rejected in this pass. `validate_node` is called for every node in the
document, and the `NodeValidation` it returns is pushed whether or not the node
was valid — which is the design chapter 2 read from the type's side, where its
`key` and `location` fields are filled unconditionally and its `template` field
is `Some` only when the node compiled. The `occurrences` map is a
`HashMap<String, Vec<SourceLocation>>` rather than a count, because the finding
this pass makes possible has to name every declaration, not say how many there
were.

The second pass is the duplicate check, and it is the one finding
`validate_document` produces on its own rather than collecting from a node.

<!-- fragment «validate-document-duplicates» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="606-638" parent="reading-and-whole-document-validation" -->
````rust
    let mut diagnostics = Vec::new();
    let named = named::parse(path, source, document, role, &mut diagnostics);

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
            category: "duplicate",
            key: Some(key.clone()),
            related: locations.iter().skip(1).copied().collect(),
            remedy: "Keep one declaration per key in each document.",
            location: locations.first().copied(),
            message: format!("duplicate key `{key}`; declarations at {declarations}"),
        });
    }
````
<!-- /fragment -->

The two loops are two different jobs. The first builds `duplicates` by walking
`validations` in document order and taking each key whose occurrence list has
more than one entry, guarded by `!duplicates.contains` so a key declared three
times is reported once; the comment above it fixes the resulting order as
*declaration order of their first appearance*, which is a stable, file-shaped
order rather than the `HashMap`'s. The second loop turns each into a diagnostic
whose message joins every location for that key with `format_location`.
`a_duplicate_key_reports_every_declaration_location` is the adjudication: it
loads a document declaring `one` twice and requires both `:1:1,` and `:2:1` in
the single message — the trailing comma in the first assertion is what pins that
the locations are joined rather than only the first being reported.

The `location` field of that diagnostic is `locations.first().copied()`, and it
is the sole reason `ValidationDiagnostic::location` is an `Option` at all.
Chapter 2 named this from the type's side: the surrounding code has already
established that the vector is non-empty, the compiler cannot see it, and the
`None` is unreachable. This is the line that makes it so.

The third pass drains everything into one result. It is where a document either
becomes a map of templates or becomes a single refusal.

<!-- fragment «validate-document-report» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="639-669" parent="reading-and-whole-document-validation" -->
````rust

    let mut templates = BTreeMap::new();
    for validation in validations {
        diagnostics.extend(validation.diagnostics);
        if let Some(words) = validation.template {
            templates.insert(
                validation.key,
                Template {
                    text: validation.text,
                    span: SourceSpan {
                        source: role.source(path),
                        start: validation.location.start,
                        end: validation.location.end,
                    },
                    words,
                    source: path.to_path_buf(),
                },
            );
        }
    }

    if !diagnostics.is_empty() {
        return Err(ConfigError::from_diagnostics(render_diagnostics(
            path,
            role,
            diagnostics,
        )));
    }

    Ok((templates, named))
}
````
<!-- /fragment -->

Duplicate and per-node findings enter one vector. `render_diagnostics` sorts
them by source position and key, so duplicates do not jump ahead of an earlier
shape failure. Templates are collected into a BTreeMap for deterministic keys.
A duplicate may replace a temporary map entry, but that map cannot escape:
any diagnostic makes the whole document fail.

A document with diagnostics returns an error rather than a partial command
map. Catalog combines that error with the other document result, reporting
structure first and semantics only after structure passes. The template
aggregation test supplies structurally valid declarations with missing, doubled,
unknown and embedded substitutions, then checks that all those independent
failures appear together with locations. The structured diagnostics suite covers
the separate shape phase and cross-file ordering.

NodeValidation retains the key and location even when its node is malformed,
so duplicate checking can still identify every declaration. The resulting load
is all-or-nothing: no invalid template or partial source set reaches expansion.

That is the whole of what a successful load has established. Both documents
parsed, every node in both satisfied every rule the vocabulary makes checkable,
no key was declared twice in either, and each key that resolves resolves to one
complete template together with the file it was read from. What none of it
established is what any of those words mean. Chapter 4 reads the rules
themselves — the node shape, the words, the `#` that would silently truncate a
line, and the diagnostics all three of this chapter's aggregating passes carry.


<a id="named-capture"></a>
## Named declarations before resolution

The private named module captures command definitions, binding targets and route targets with their original spans. `is_wrapper` distinguishes a child-bearing `config` from a flat key named config. `parse` checks both namespaces and document roles without compiling dormant templates. Flat keys enter the route duplicate table before wrapper traversal, so textual order cannot hide a duplicate. Empty command and route blocks are accepted; parameter-bearing blocks and all profile syntax fail explicitly at this increment.

<!-- fragment «named-capture» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="1-169" parent="source-named" -->
````rust
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

````
<!-- /fragment -->

<a id="named-diagnostics"></a>
## Named semantic refusals

`problem` attaches a target’s source and byte range to a semantic diagnostic. Reference resolution adds the affected names. This gives a failed binding a useful file location even though it was resolved after capture, and no file needs to be reopened.

<!-- fragment «named-diagnostics» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="170-184" parent="source-named" -->
````rust
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
````
<!-- /fragment -->

<a id="named-fold"></a>
## Fold targets and retain declarations

`resolve` collects primary targets before applying the overlay. Its admitted-key set is filled only by primary declarations, independently of later target changes. Both flat and named routes enter the same map. Every captured declaration receives an origin in source order; target assignments append to histories rather than erasing their predecessors. Definition origins have no target history, because definitions cannot be overridden. This is where a local binding change redirects several routes without granting a new key.

<!-- fragment «named-fold» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="248-358" parent="source-named" -->
````rust
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
````
<!-- /fragment -->

<a id="named-resolve"></a>
## Activate definitions and project commands

After the fold, every effective binding must name a definition whose template compiles. Compilation is cached per definition, and dormant definitions are never parsed as command words. Admitted routes then resolve through those bindings; local-only routes become explanatory non-admission records before reference lookup. Each successful command exposes its route, binding and template origins, both target histories, and the very words expansion uses. A flat replacement instead has only its literal target origin. Independent failures aggregate in source-role and byte order; any failure prevents a Templates snapshot.

<!-- fragment «named-resolve» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="359-494" parent="source-named" -->
````rust
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

````
<!-- /fragment -->

<a id="named-lookups"></a>
## Response-local identity

The final helpers order route histories before binding histories and locate captured origins and histories by identity. These lookups operate only on declarations already recorded by the fold; their expectations express that internal invariant. The response keeps native source paths and captured byte offsets, so deleting either input file cannot invalidate an origin or change a compiled word.

<!-- fragment «named-lookups» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="495-527" parent="source-named" -->
````rust
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
````
<!-- /fragment -->

[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)
