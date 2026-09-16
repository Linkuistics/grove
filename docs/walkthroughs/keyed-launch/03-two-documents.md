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
composition remains pending; inactive profiles pass structural validation.

<a id="one-entry-point"></a>
## One validation path

Both public loading routes reach Catalog's validator. The constructor checks
the vocabulary, then captures both explicit document results even when the
primary fails. It reports read, syntax, shape and duplicate failures before
template-semantic failures. With valid structure, unused flat templates still
undergo eager checking. Errors are ordered by primary/overlay, byte position
and key; a bad document never causes fallback to another configuration.

<!-- fragment «templates-load» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="166-291" parent="source-templates" -->
<!-- insert «templates-load-three-promises» -->
<!-- insert «templates-load-primary» -->
<!-- insert «templates-load-overlay» -->
<!-- insert «templates-load-value» -->
<!-- /fragment -->

The opening fragment states the eager flat validation and fail-closed contract.
Its output is a Catalog rather than an already merged map.

<!-- fragment «templates-load-three-promises» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="166-175" parent="templates-load" -->
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
the start of resolution. Each accessor returns the captured optional `select`
declaration from its source. Absence returns `None`; a present empty list retains
its declaration span. Any nonempty explicit selection fails: unknown names report `unknown_profile`,
while known names report that composition is not yet supported and relate the
profile definition span. Every selected entry has a diagnostic occurrence with its
zero-based selection index; external selections have no invented span. An empty
selection resolves the captured base declarations.
The consumer chooses that list. The convenience loader always supplies an empty
selection, ignoring captured declarations rather than choosing between them.

<!-- fragment «templates-load-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="176-252" parent="templates-load" -->
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

    /// Fold captured base targets with primary authority and local replacement.
    /// Validate effective references and return an independently owned snapshot.
    pub fn resolve(&self, selection: &Selection) -> Result<Templates, ConfigError> {
        if !selection.profiles.is_empty() {
            let diagnostics = selection.profiles.iter().enumerate().map(|(index, profile)| {
                let definition = self.captured.primary.named.profiles.get(profile);
                let mut diagnostic = if let Some(definition) = definition {
                    let mut diagnostic = Diagnostic::new("shape", format!(
                        "profile composition is not yet supported: `{profile}` at selection index {index}"
                    ), "Resolve with an empty selection until profile composition is available; inactive profiles may remain in primary policy.");
                    diagnostic.related.push(definition.clone());
                    diagnostic
                } else {
                    Diagnostic::new("unknown_profile", format!(
                        "unknown profile `{profile}` at selection index {index}"
                    ), "Use a declared profile name; resolution currently requires an empty selection until profile composition is available.")
                };
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

<!-- fragment «templates-load-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="253-253" parent="templates-load" -->
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

<!-- fragment «templates-load-value» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="254-291" parent="templates-load" -->
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

<!-- fragment «reading-and-whole-document-validation» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="474-682" parent="source-templates" -->
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

<!-- fragment «compile-vocabulary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="474-511" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «read-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="512-536" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «read-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="537-556" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «parse-and-validate» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="557-595" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «validate-document-nodes» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="596-618" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «validate-document-duplicates» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="619-651" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «validate-document-report» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="652-682" parent="reading-and-whole-document-validation" -->
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

The private named module captures command definitions, binding targets, optional route targets and shared/route parameter maps with their original spans. `is_wrapper` distinguishes a child-bearing `config` from a flat key named config. `parse` checks namespaces and document roles without compiling dormant templates. Flat keys enter the route duplicate table before wrapper traversal, so textual order cannot hide a duplicate. Command children declare parameters and optional defaults; their names, shapes and uniqueness are checked even when the command stays dormant. `parse_values` and route parsing share `parse_patches`, which captures assignments and removals in either source and rejects duplicate mentions even when one is an `unset`. Parameter-only routes are structurally valid; personal target authority is checked during resolution. Profile definitions are primary-only. `ParseSource` groups the source path, captured text and document role for
`parse_scope`, which shares patch validation while
using a fresh duplicate namespace for each profile. It restricts profile children
to include lists and patches; nested definitions and selection declarations fail.
The captured KDL retains each complete profile, and the declaration map retains
its span. Empty selection never folds these patches, so inactive references and
include cycles cannot change the base command or authorize a local-only key.

`parse_selection` validates both the source selection and a profile include
list without resolving names. Each scope has its own list slot. It preserves repeats and an empty list, rejects properties, types and
children, and reports duplicate declarations against both source spans. This
lets a consumer inspect the declaration before deciding policy; Grove uses that
boundary to refuse declarations until its selection policy is implemented.

<!-- fragment «named-capture» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="1-505" parent="source-named" -->
````rust
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
    pub(super) profiles: BTreeMap<String, SourceSpan>,
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
        parse_scope(
            ParseSource {
                path,
                text: source,
                role,
            },
            children,
            false,
            &mut result,
            &mut routes,
            diagnostics,
        );
    }
    for diagnostic in &mut diagnostics[first_diagnostic..] {
        if diagnostic.category == "shape" {
            diagnostic.remedy = "Use command declarations, bind/route targets, values blocks and at most one select list inside config; select takes only valid profile-name strings. Profiles belong in primary policy and accept include lists and values/bind/route patches.";
        }
    }
    result
}

struct ParseSource<'a> {
    path: &'a Path,
    text: &'a str,
    role: DocumentRole,
}

// Profiles share patch validation with the base, but never its duplicate namespace.
fn parse_scope(
    source: ParseSource<'_>,
    children: &KdlDocument,
    profile_scope: bool,
    result: &mut Declarations,
    routes: &mut BTreeMap<String, SourceLocation>,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let ParseSource {
        path,
        text: source,
        role,
    } = source;
    for child in children.nodes() {
        let loc = location(source, child);
        let kind = child.name().value();
        if profile_scope && !matches!(kind, "include" | "values" | "bind" | "route") {
            diagnostics.push(at_node(
                loc,
                format!("unsupported profile node `{kind}`; use include, values, bind or route"),
            ));
            continue;
        }
        if kind == "profile" {
            parse_profile(path, source, child, role, &mut result.profiles, diagnostics);
            continue;
        }
        if kind == "select" || kind == "include" && profile_scope {
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
            diagnostics.push(at_node(loc, format!("unsupported config node `{kind}`")));
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

fn parse_profile(
    path: &Path,
    source: &str,
    node: &KdlNode,
    role: DocumentRole,
    profiles: &mut BTreeMap<String, SourceSpan>,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let loc = location(source, node);
    let name = node
        .entries()
        .first()
        .and_then(|entry| entry.value().as_string());
    let Some(name) = name.filter(|name| valid_name(name)) else {
        diagnostics.push(at_node(
            loc,
            "profile requires a valid profile-name string and a child block".into(),
        ));
        return;
    };
    if role == DocumentRole::Overlay
        || !plain(node)
        || node.entries().len() != 1
        || node.children().is_none()
    {
        diagnostics.push(at_node(loc, "profile requires one name and a child block in primary policy, without properties or types".into()));
        return;
    }
    let span = SourceSpan {
        source: role.source(path),
        start: loc.start,
        end: loc.end,
    };
    if let Some(previous) = profiles.insert(name.into(), span) {
        let mut earlier = source_location(source, previous.start);
        earlier.end = previous.end;
        duplicate(name, earlier, loc, diagnostics);
    }
    if let Some(children) = node.children() {
        // Validate without folding; the captured KDL retains the complete profile.
        parse_scope(
            ParseSource {
                path,
                text: source,
                role,
            },
            children,
            true,
            &mut Declarations::default(),
            &mut BTreeMap::new(),
            diagnostics,
        );
    }
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
        diagnostics.push(at_node(loc, format!("{} requires zero or more valid profile-name strings, without properties, types or children", node.name().value())));
        return;
    };
    if let Some(previous) = selection
        .as_ref()
        .and_then(|selection| selection.origin.as_ref())
    {
        let mut earlier = source_location(source, previous.start);
        earlier.end = previous.end;
        duplicate(node.name().value(), earlier, loc, diagnostics);
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

````
<!-- /fragment -->

<a id="named-diagnostics"></a>
## Named semantic refusals

`problem` attaches a target’s source and byte range to a semantic diagnostic. Reference resolution adds the affected names. This gives a failed binding a useful file location even though it was resolved after capture, and no file needs to be reopened.

<!-- fragment «named-diagnostics» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="506-521" parent="source-named" -->
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

`resolve` collects primary targets and shared values before applying the overlay. Its admitted-key set is filled only by primary explicit targets, independently of later target or value changes. Both flat and named routes enter the same map. Shared values fold per command and parameter: assignment replaces the value, while `unset` removes it from the effective map. Neither operation changes a declaration default. A personal parameter-only route produces `missing_target` before the overlay can supply authority. Route maps fold separately from shared maps, preserving exceptions across binding changes. Literal replacement clears a map and records a reset for each historical route parameter; switching a literal to a binding starts fresh. A parameter-only patch on a final literal is refused. Every captured declaration receives one origin in source order; several resets can share that origin while retaining distinct assignment orders. Target and value operations append to histories rather than erasing predecessors. Definition origins have no target history, because definitions cannot be overridden. This is where a local binding change redirects several routes, or a local value changes their arguments, without granting a new key.

<!-- fragment «named-fold» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="660-900" parent="source-named" -->
````rust
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
````
<!-- /fragment -->

<a id="named-resolve"></a>
## Activate definitions and project commands

After the fold, every effective values target must name a command, and surviving assignments must name its parameters and contain no NUL. Removals need not match the schema. These checks do not compile dormant templates. Every effective binding then names a definition whose template compiles; compilation is cached per definition. Admitted routes resolve through those bindings; local-only routes become explanatory non-admission records before reference lookup.

Every admitted route must have values for all declared parameters, including unused ones. A route assignment wins over the shared value and default; `unset` exposes the next lower scope. Surviving route names are checked against the final command schema, while removed old-schema names remain only in history. Non-admitted local routes skip semantic resolution. For example, personal `mode = primary` followed by local `unset mode` yields the declaration default, while both operations remain in the shared history. Missing values and NUL-bearing defaults report the declaration and related route/binding spans; invalid route assignments identify the assignment and final command. A malformed shared map prevents downstream route-completeness noise for that command. Only a complete route instantiates its fragments. Command views link all route histories, including removals and resets for names absent from the final schema; word origins contain only the declaration and winning value contributors.

Each successful command exposes its route, binding and template origins, target and parameter histories, resolved parameters, and the very words expansion uses. A parameter keeps its declaration origin plus the winning shared assignment, if present, and both default and shared histories where they exist. A word adds the template origin and deduplicates contributors when a parameter repeats. A flat replacement instead has only its literal target origin and no parameter map. Independent failures aggregate in source-role and byte order; any failure prevents a Templates snapshot.

<!-- fragment «named-resolve» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="901-1224" parent="source-named" -->
````rust
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

````
<!-- /fragment -->

<a id="named-lookups"></a>
## Response-local identity

The final helpers order route histories before binding histories and locate captured origins and histories by identity. These lookups operate only on declarations already recorded by the fold; their expectations express that internal invariant. The response keeps native source paths and captured byte offsets, so deleting either input file cannot invalidate an origin or change a compiled word.

<!-- fragment «named-lookups» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="1225-1259" parent="source-named" -->
````rust
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
````
<!-- /fragment -->

[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)
