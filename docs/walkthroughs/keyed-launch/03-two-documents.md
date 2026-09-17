# Two documents and explicit targets
<!-- book-page id="two-documents" slice="never-assembled" order="3" -->
[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)

<a id="never-assembled"></a>
## Capture once, resolve without reading

`Catalog::load` reads the explicit primary and optional overlay, validates document structure and retains original bytes, parsed KDL and modular
declarations.
`Catalog::resolve` folds binding and route targets, applies primary authority,
and compiles effective named commands from those captured declarations. `Templates::load` is the convenience composition
of those operations with an empty selection. None of these operations decides
which files a consumer should supply.

The running example resolves each key to one whole template. Capture preserves
both original declarations when an overlay replaces a command, and Templates
owns its snapshot independently. Changing or removing a file cannot change
expansion, inspection or diagnostic locations. Inspection explains named reference chains and parameter
composition, including every selected profile occurrence. Inactive profiles pass
structural validation without activating their references.

<a id="one-entry-point"></a>
## One validation path

Both public loading routes reach Catalog's validator. The constructor checks
the vocabulary, then captures both explicit document results even when the
primary fails. It reports read, syntax, shape and duplicate failures before
template-semantic failures. Effective bindings determine which command
templates compile. Errors are ordered by primary/overlay, byte position
and key; a bad document never causes fallback to another configuration.

<!-- fragment «templates-load» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="158-259" parent="source-templates" -->
<!-- insert «templates-load-three-promises» -->
<!-- insert «templates-load-primary» -->
<!-- insert «templates-load-overlay» -->
<!-- insert «templates-load-value» -->
<!-- /fragment -->

The opening fragment states the structural validation and fail-closed contract.
Its output is a Catalog rather than an already merged map.

<!-- fragment «templates-load-three-promises» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="158-167" parent="templates-load" -->
````rust
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
````
<!-- /fragment -->

The next fragment captures the documents and implements selection accessors and
the start of resolution. Each accessor returns the captured optional `select`
declaration from its source. Absence returns `None`; a present empty list retains
its declaration span. Resolution delegates to the named resolver for base,
selected occurrences and local patches. External selections have no invented
span; declared lists retain their original location. An empty selection resolves
the captured base declarations.
The consumer chooses that list. The convenience loader always supplies an empty
selection, ignoring captured declarations rather than choosing between them.

<!-- fragment «templates-load-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="168-220" parent="templates-load" -->
````rust
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

    /// Fold base, selected profile occurrences and local patches with primary authority.
    /// Validate effective references and return an independently owned snapshot.
    pub fn resolve(&self, selection: &Selection) -> Result<Templates, ConfigError> {
````
<!-- /fragment -->

<a id="overrides-never-supplies"></a>
## An overlay overrides and never supplies

Resolution delegates to the private named resolver, which handles both literal
and binding targets. Primary declarations authorize keys before local replacements;
a local-only route is retained for an explanatory refusal. The declaration fold
and reference validation are explained [below](#named-fold).

<!-- fragment «templates-load-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="221-221" parent="templates-load" -->
````rust
        let (templates, overlay_only, inspection) = named::resolve(&self.captured, selection)?;
````
<!-- /fragment -->

The resolved value owns the winner map and slot table and shares captured inputs
through `Arc`. `slot_names` lends the captured vocabulary to conformance.
Templates' convenience constructor delegates directly to the same Catalog path,
so it cannot drift into a second reader or different vocabulary rules.

The resolver records original declarations before projecting the admitted commands.
Origins follow base, profile applications and overlay, with source order inside
each patch; histories retain replaced targets and repeated applications. Empty documents still appear in `sources`. The returned Templates owns
that explanation alongside compiled words, without reopening any source path.

<!-- fragment «templates-load-value» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="222-259" parent="templates-load" -->
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
`named::compile` turns each effective command definition into argument fragments
and runtime slots, checking the consumer vocabulary before expansion. Chapter 4
explains those template rules.

`crates/keyed-launch/tests/catalog.rs` uses the unrelated key `opaque` and runtime
slot `payload` to test the same boundary. It edits the primary and removes the
overlay before resolution and conformance, then removes the primary and drops
Catalog before expansion. Exact expected words prove both snapshot independence
and equality with the convenience loader; native non-UTF-8 slot bytes survive.

<a id="the-second-ending"></a>
## A local declaration cannot admit a key

A valid overlay-only command is retained but does not appear in `keys()` and has
no `source()`. `require` and `expand` refuse it, naming the primary file where the
key must be declared. Validation and admission are distinct: malformed overlay declarations prevent
loading even when they cannot admit a key. A structurally valid overlay-only
route is retained as non-admitted while other admitted keys resolve normally.

<a id="the-slot-table-first"></a>
## The slot table first, and the duplicate it refuses

The rest of the chapter is the four capture helpers `load` reaches, in the order it
reaches them. They are free functions rather than methods because none of them
needs a `Templates` — they run before one exists — and all four are private:
`Catalog::load` is their entry point; Catalog resolution then produces
Templates without invoking them again. The block is read in four fragments.

<!-- fragment «reading-and-whole-document-validation» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="442-571" parent="source-templates" -->
<!-- insert «compile-vocabulary» -->
<!-- insert «read-primary» -->
<!-- insert «read-overlay» -->
<!-- insert «parse-and-validate» -->
<!-- /fragment -->

`compile_vocabulary` is the first, and it validates names before copying them: it
copies the consumer's borrowed `SlotRule`s into the owned `SlotSpec` table that
`Templates` will keep, and it refuses duplicate names and names in the reserved `param.` namespace. Its comment is the only
statement anywhere in the crate of why the duplicate is a refusal rather than a
tolerated redundancy, and the reason is that the failure it would otherwise cause
is silent and lands on the wrong file.

<!-- fragment «compile-vocabulary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="442-479" parent="reading-and-whole-document-validation" -->
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
against its own cardinality rule: `named::compile` walks the slot table and
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

<!-- fragment «read-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="480-504" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «read-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="505-524" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «parse-and-validate» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="525-571" parent="reading-and-whole-document-validation" -->
````rust

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
        templates: BTreeMap::new(),
        named,
    })
}
````
<!-- /fragment -->

`named::parse` rejects every top-level node other than a wrapper before checking
wrapper contents. The rejection retains each node's span and the modular-form
remedy. It applies in both sources, including mixed documents and former flat
keys named `config`; empty documents have nothing to reject. Capture stores the
modular declarations only when the parser reports no findings.

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
and validated declarations together. The parsed spans therefore still refer to the exact
bytes that were read, even after the path changes.

A syntax error stops validation of that document because there is no parsed
node list to walk. Its parser byte range becomes a `kdl_syntax` diagnostic.
Catalog still collects errors from the other explicit document. DocumentRole
attaches source identity to syntax and node reports. It also restricts named
command definitions to primary policy.

<a id="one-refusal"></a>
## Every diagnostic in one refusal

`parse_and_validate` gives the modular parser one diagnostic vector. Any finding
returns an error instead of a partial capture; `render_diagnostics` sorts the
records by source position and key. Catalog combines the two explicit document
results before deciding whether structural validity permits resolution.

The template aggregation test supplies structurally valid active definitions
with missing, doubled, unknown and embedded runtime slots. Their independent
semantic failures appear together with locations. The diagnostics suite checks
the separate structural phase and cross-file ordering. Neither path can expose
an invalid Templates snapshot to expansion.

<a id="named-capture"></a>
## Named declarations before resolution

The private named module captures command definitions, binding targets, optional route targets and shared/route parameter maps with their original spans. `is_wrapper` distinguishes a child-bearing `config` from a flat key named config. `parse` checks namespaces and document roles without compiling dormant templates. Unsupported top-level nodes are rejected before wrapper traversal. Modular routes have their own duplicate namespace. Command children declare parameters and optional defaults; their names, shapes and uniqueness are checked even when the command stays dormant. `parse_values` and route parsing share `parse_patches`, which captures assignments and removals in either source and rejects duplicate mentions even when one is an `unset`. Parameter-only routes are structurally valid; personal target authority is checked during resolution. Profile definitions are primary-only. `ParseSource` groups the source path, captured text and document role for
`parse_scope`, which shares patch validation while
using a fresh duplicate namespace for each profile. It restricts profile children
to include lists and patches; nested definitions and selection declarations fail.
The declaration map retains each profile’s parsed patch and definition span;
its optional include list carries the original include span. Empty selection never folds these patches, so inactive references and
include cycles cannot change the base command or authorize a local-only key.

`parse_selection` validates both the source selection and a profile include
list without resolving names. Each scope has its own list slot. It preserves repeats and an empty list, rejects properties, types and
children, and reports duplicate declarations against both source spans. This
lets a consumer inspect the declaration before deciding policy; Grove uses that
boundary to refuse declarations until its selection policy is implemented.

<!-- fragment «named-capture» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="1-533" parent="source-named" -->
````rust
//! Named commands and profiles: capture structure, expand occurrences, fold targets.
use super::{
    at_node, contains_shell_comment_start, source_location, Assignment, AssignmentHistory,
    AssignmentValue, BTreeMap, Captured, CommandView, ConfigError, Diagnostic, DocumentRole,
    Inspection, KdlDocument, KdlNode, NonAdmittedKey, Origin, Path, Selection, Setting, SlotSpec,
    SourceLocation, SourceRole, SourceSpan, Template, ValidationDiagnostic, Word, WordView,
};
use crate::{Occurrence, ParameterView};

#[derive(Default)]
pub(super) struct Declarations {
    pub(super) selection: Option<Selection>,
    profiles: BTreeMap<String, Profile>,
    commands: BTreeMap<String, Command>,
    bindings: BTreeMap<String, Target>,
    routes: BTreeMap<String, RoutePatch>,
    values: BTreeMap<String, Values>,
}

struct Profile {
    span: SourceSpan,
    patch: Declarations,
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
    chain: Vec<Occurrence>,
}

impl Target {
    fn applied(&self, chain: &[Occurrence]) -> Self {
        Self {
            chain: chain.to_vec(),
            ..self.clone()
        }
    }

    fn occurrence(&self) -> Option<usize> {
        self.chain.last().map(|o| o.id)
    }
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
    for node in document.nodes().iter().filter(|node| !is_wrapper(node)) {
        diagnostics.push(at_node(
            location(source, node),
            format!(
                "unsupported top-level declaration `{}`",
                node.name().value()
            ),
        ));
    }
    if diagnostics.len() != first_diagnostic {
        return result;
    }

    let mut routes = BTreeMap::new();
    for node in document.nodes() {
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
            chain: Vec::new(),
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
    profiles: &mut BTreeMap<String, Profile>,
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
    let mut patch = Declarations::default();
    if let Some(children) = node.children() {
        parse_scope(
            ParseSource {
                path,
                text: source,
                role,
            },
            children,
            true,
            &mut patch,
            &mut BTreeMap::new(),
            diagnostics,
        );
    }
    if let Some(previous) = profiles.insert(name.into(), Profile { span, patch }) {
        let mut earlier = source_location(source, previous.span.start);
        earlier.end = previous.span.end;
        duplicate(name, earlier, loc, diagnostics);
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
        chain: Vec::new(),
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

<!-- fragment «named-diagnostics» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="534-550" parent="source-named" -->
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
    diagnostic.occurrence_chain.clone_from(&target.chain);
    diagnostic.primary = Some(target.span.clone());
    diagnostic.source = Some(target.span.source.clone());
    diagnostic
}

````
<!-- /fragment -->

<a id="named-fold"></a>
## Fold targets and retain declarations

`expand_profiles` creates an occurrence for each selected name and each include
edge. Enter/exit events replace recursion: entering assigns an ID and checks the
ancestor chain for cycles; exiting schedules the patch after its includes. The
same profile reached twice gets two IDs. Expansion errors are collected before
folding, so a later assignment cannot hide an unknown include or a cycle.

For example, `left` includes `base` and overrides its value; `right` includes
`base` again. Selecting `left`, then `right` schedules `base`, `left`, `base`,
`right`. The second base assignment replaces the left override. Its source span
matches the first base assignment but its occurrence ID differs. Parent links
and selection indices explain how each application was reached.

`resolve` folds personal base, scheduled profile patches and the overlay. Its
admitted-key set is filled only by personal explicit targets. At the boundary
before local patches, `check_personal_targets` refuses any surviving personal
parameter-only route without a target. A local target cannot supply authority;
a later personal profile can. The fold still contains literal-route branches,
but capture rejects flat input and leaves its literal template map empty; only
named routes can reach those maps from supported input.

Shared values fold per command and parameter; route maps fold separately,
preserving route exceptions across binding changes. `unset` removes only its
scope's override. Literal replacement clears the route map and records resets;
switching a literal to a binding also records resets before applying its own
parameters. A parameter-only patch on a final literal is refused.

Every applied declaration receives an origin keyed by span and occurrence.
Several resets may share that origin while retaining distinct assignment orders.
Effective targets retain their occurrence chains for diagnostics and winning
word origins. Histories append instead of erasing predecessors. Definitions have
no occurrence because profiles cannot redefine them. This is where a selected
binding redirects several routes without duplicating their declarations.

<!-- fragment «named-fold» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="689-1078" parent="source-named" -->
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

fn occurrence_chain(occurrences: &[Occurrence], mut id: Option<usize>) -> Vec<Occurrence> {
    let mut chain = Vec::new();
    while let Some(index) = id {
        let occurrence = &occurrences[index];
        chain.push(occurrence.clone());
        id = occurrence.parent;
    }
    chain.reverse();
    chain
}

// Enter/exit events keep deep includes off the call stack. IDs follow discovery;
// applications follow postorder, so every include precedes its owner's patch.
fn expand_profiles(
    declarations: &Declarations,
    selection: &Selection,
) -> Result<(Vec<Occurrence>, Vec<usize>), ConfigError> {
    enum Visit {
        Enter(String, Option<usize>, usize, Option<SourceSpan>),
        Exit(usize),
    }
    let mut pending: Vec<_> = selection
        .profiles
        .iter()
        .enumerate()
        .rev()
        .map(|(index, name)| Visit::Enter(name.clone(), None, index, selection.origin.clone()))
        .collect();
    let mut occurrences = Vec::new();
    let mut applications = Vec::new();
    let mut diagnostics = Vec::new();
    while let Some(visit) = pending.pop() {
        let Visit::Enter(profile, parent, selection_index, via) = visit else {
            if let Visit::Exit(id) = visit {
                applications.push(id);
            }
            continue;
        };
        let ancestors = occurrence_chain(&occurrences, parent);
        let cycle = ancestors.iter().any(|o| o.profile == profile);
        let id = occurrences.len();
        occurrences.push(Occurrence {
            id,
            profile: profile.clone(),
            parent,
            selection_index,
            via: via.clone(),
        });
        let definition = declarations.profiles.get(&profile);
        if cycle || definition.is_none() {
            let chain = occurrence_chain(&occurrences, Some(id));
            let names = chain
                .iter()
                .map(|o| o.profile.as_str())
                .collect::<Vec<_>>()
                .join(" -> ");
            let mut diagnostic = if cycle {
                Diagnostic::new(
                    "include_cycle",
                    format!("profile include cycle: {names}"),
                    "Remove an include edge from the reported cycle.",
                )
            } else {
                Diagnostic::new(
                    "unknown_profile",
                    format!("unknown profile `{profile}` in {names}"),
                    "Declare the profile in primary policy, or correct the selection/include name.",
                )
            };
            diagnostic.source = via.as_ref().map(|s| s.source.clone());
            diagnostic.primary = via;
            diagnostic.related = chain.iter().filter_map(|o| o.via.clone()).collect();
            diagnostic.occurrence_chain = chain;
            diagnostics.push(diagnostic);
            continue;
        }
        pending.push(Visit::Exit(id));
        if let Some(include) = definition.and_then(|d| d.patch.selection.as_ref()) {
            pending.extend(include.profiles.iter().rev().map(|name| {
                Visit::Enter(
                    name.clone(),
                    Some(id),
                    selection_index,
                    include.origin.clone(),
                )
            }));
        }
    }
    if !diagnostics.is_empty() {
        diagnostics.sort_by(|a, b| diagnostic_order(a).cmp(&diagnostic_order(b)));
        return Err(ConfigError::from_diagnostics(diagnostics));
    }
    Ok((occurrences, applications))
}

fn check_personal_targets(routes: &BTreeMap<String, Route>, diagnostics: &mut Vec<Diagnostic>) {
    for (key, route) in routes {
        if let Route::Missing(target) = route {
            let mut diagnostic = problem(
                "missing_target",
                target,
                format!("key `{key}` has a personal parameter patch but no personal target"),
            );
            diagnostic.key = Some(key.clone());
            diagnostic.remedy = "Add an explicit target for this key in personal policy, or deselect/remove the personal parameter patch; a local target cannot authorize it.".into();
            diagnostics.push(diagnostic);
        }
    }
}

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
    let (occurrences, applications) = expand_profiles(&captured.primary.named, selection)?;
    view.profile_occurrences = occurrences;
    view.sources.push(super::Source {
        role: SourceRole::Primary,
        path: captured.primary.path.clone(),
    });
    if let Some(overlay) = &captured.overlay {
        view.sources.push(super::Source {
            role: SourceRole::Overlay,
            path: overlay.path.clone(),
        });
    }
    let empty_templates = BTreeMap::new();
    let layers = std::iter::once((
        &captured.primary.named,
        &captured.primary.templates,
        None,
        true,
    ))
    .chain(applications.iter().map(|id| {
        let occurrence = &view.profile_occurrences[*id];
        (
            &captured.primary.named.profiles[&occurrence.profile].patch,
            &empty_templates,
            Some(*id),
            true,
        )
    }))
    .chain(
        captured
            .overlay
            .iter()
            .map(|document| (&document.named, &document.templates, None, false)),
    )
    .collect::<Vec<_>>();
    for (named, literals, occurrence, primary) in layers {
        // Personal authority is settled before any local target can repair it.
        if !primary {
            check_personal_targets(&routes, &mut diagnostics);
        }
        let chain = occurrence_chain(&view.profile_occurrences, occurrence);
        let mut declarations = Vec::new();
        for (command, values) in &named.values {
            value_targets.insert(command.clone(), values.target.applied(&chain));
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
                            chain: chain.clone(),
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
        for (name, command) in &named.commands {
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
        for (binding, target) in &named.bindings {
            bindings.insert(binding.clone(), target.applied(&chain));
            declarations.push((
                &target.span,
                Some(Setting::BindingTarget {
                    binding: binding.clone(),
                }),
                AssignmentValue::Set(target.value.clone()),
            ));
        }
        for (key, patch) in &named.routes {
            let effective = route_values.entry(key.clone()).or_default();
            if let Some(binding) = &patch.binding {
                if matches!(routes.get(key), Some(Route::Literal(_))) {
                    effective.clear();
                    literal_patches.remove(key);
                    for history in &view.histories {
                        if matches!(&history.setting, Setting::RouteParameter { key: owner, .. } if owner == key)
                        {
                            declarations.push((
                                &patch.declaration.span,
                                Some(history.setting.clone()),
                                AssignmentValue::Reset,
                            ));
                        }
                    }
                }
                routes.insert(
                    key.clone(),
                    Route::Binding(Target {
                        chain: chain.clone(),
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
                    .or_insert_with(|| Route::Missing(patch.declaration.applied(&chain)));
                declarations.push((&patch.declaration.span, None, AssignmentValue::Unset));
                if matches!(routes.get(key), Some(Route::Literal(_))) {
                    literal_patches.insert(key.clone(), patch.declaration.applied(&chain));
                }
            }
            for (name, parameter) in &patch.parameters {
                let value = if let Some(value) = &parameter.value {
                    effective.insert(
                        name.clone(),
                        Target {
                            chain: chain.clone(),
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
        for (key, template) in literals {
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
            let origin = if let Some(origin) = view
                .origins
                .iter()
                .find(|o| &o.span == span && o.occurrence == occurrence)
            {
                origin.id
            } else {
                let id = view.origins.len();
                view.origins.push(Origin {
                    id,
                    span: span.clone(),
                    occurrence,
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
    if captured.overlay.is_none() {
        check_personal_targets(&routes, &mut diagnostics);
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

Each successful command exposes its route, binding and template origins, target and parameter histories, resolved parameters, and the very words expansion uses. A parameter keeps its declaration origin plus the winning shared assignment, if present, and both default and shared histories where they exist. A word adds the template origin and deduplicates contributors when a parameter repeats. The unreachable literal-replacement branch has only a literal target origin and no parameter map. Independent failures aggregate in source-role and byte order; any failure prevents a Templates snapshot.

<!-- fragment «named-resolve» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="1079-1419" parent="source-named" -->
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
                diagnostic.occurrence_chain.clone_from(&target.chain);
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
        let route_origin = origin_id(
            &view,
            route.span(),
            match &route {
                Route::Binding(t) | Route::Missing(t) => t.occurrence(),
                Route::Literal(_) => None,
            },
        );
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
                            chain: Vec::new(),
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
                        diagnostic.occurrence_chain = if route.chain.is_empty() {
                            binding.chain.clone()
                        } else {
                            route.chain.clone()
                        };
                        diagnostic.related = vec![route.span.clone(), binding.span.clone()];
                        diagnostic.key = Some(key.clone());
                        diagnostic.binding = Some(route.value.clone());
                        diagnostic.command = Some(binding.value.clone());
                        diagnostic.parameter = Some(name.clone());
                        diagnostic.remedy = "Supply a NUL-free declaration default, shared values assignment or route override in primary or local configuration.".into();
                        diagnostics.push(diagnostic);
                        continue;
                    }
                    let mut origins = vec![origin_id(&view, &parameter.span, None)];
                    if let Some(assigned) = assigned {
                        origins.push(origin_id(&view, &assigned.span, assigned.occurrence()));
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
                        word.instantiate(
                            &parameters,
                            origin_id(&view, &definition.template.span, None),
                        )
                    })
                    .collect();
                let origins = vec![
                    route_origin,
                    origin_id(&view, &binding.span, binding.occurrence()),
                    origin_id(&view, &definition.template.span, None),
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
        let template_origin = origin_id(&view, &template.span, None);
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

<!-- fragment «named-lookups» owner="never-assembled" source="crates/keyed-launch/src/templates/named.rs" lines="1420-1454" parent="source-named" -->
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

fn origin_id(view: &Inspection, span: &SourceSpan, occurrence: Option<usize>) -> usize {
    view.origins
        .iter()
        .position(|origin| &origin.span == span && origin.occurrence == occurrence)
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
