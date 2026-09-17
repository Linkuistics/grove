# The names a template is written against
<!-- book-page id="the-names" slice="rules-about-names" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Two documents and explicit targets](03-two-documents.md)

<a id="rules-about-names"></a>
## Rules about names

Chapter 1 read the three files that say what this crate refuses. This chapter
reads the two that say what it accepts, and both of them are lists of *names*.

Every rule `keyed-launch` applies to a command template is a rule about a slot's
name: that a `${…}` occupies a complete word rather than part of one, that the
name inside it is one the consumer declared, and that a name declared required
appears exactly once in a template while an optional one appears at most once.
None of the three says anything about what a name means. `prompt` might carry a
session's mandate, a filename, or an empty string; the crate counts its
occurrences, replaces it with whatever value it is handed, and never looks
inside.

That is what this stage must not add and must not interpret, and it has one
structural consequence, which is this chapter's whole argument: **the names have
to be in the crate's hands before a template can be checked at all.** So the
vocabulary is a parameter of `Templates::load` and not of `Templates::expand`.
This is the one place in the book where the crate takes a position a reader could
reasonably take the other way, and it is argued here rather than asserted, with
the alternative named and its cost stated.

The chapter owns the vocabulary declarations and the shapes that hold captured
and resolved configuration. Chapters 3, 4 and 5 build and consume these shapes.
Neither block reads a file or spawns a process.

<a id="an-input-to-load"></a>
## The vocabulary is an input to `load`, not to `expand`

`src/vocabulary.rs` opens with the argument itself. Nine of its first twelve
lines are the doc comment on `Vocabulary`, and they are the only place under
`src/` where the position is stated rather than relied on — everywhere else in
the crate it is treated as settled. The whole file follows here in four
fragments, cut at its own type boundaries.

<!-- fragment «vocabulary» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="1-46" parent="source-vocabulary" -->
<!-- insert «vocabulary-supplied-at-load» -->
<!-- insert «vocabulary-slot-rule» -->
<!-- insert «vocabulary-requirement» -->
<!-- insert «vocabulary-cardinality-and-message» -->
<!-- /fragment -->

The first fragment is the type and its reason. `Vocabulary` is one field — a
borrowed slice of `SlotRule` — and the nine lines above it are why that slice is
a parameter of the loader rather than of expansion.

<!-- fragment «vocabulary-supplied-at-load» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="1-15" parent="vocabulary" -->
````rust
/// The slot vocabulary a consumer's templates are written against.
///
/// **Supplied at load, because every template rule is a rule about slot
/// *names*.** That a substitution occupies a whole word, that it names a
/// declared slot, that a required slot appears exactly once and an optional one
/// at most once — none of them is checkable by a loader that will not learn the
/// names until expansion. Capture retains the vocabulary; resolution checks
/// active commands before anything is spawned. Expansion then checks that the
/// values offered fill the slots declared here, without reparsing their bytes.
/// Names beginning with `param.` are reserved for configuration parameters and
/// are refused by both Catalog and Templates loading, before source I/O.
pub struct Vocabulary<'a> {
    pub slots: &'a [SlotRule<'a>],
}

````
<!-- /fragment -->

The comment's central sentence lists rules that require the consumer's slot
names before expansion. `Catalog::load` first calls `compile_vocabulary`, copying
the borrowed rules into an owned table. Structural capture does not need those
names; the resolver supplies them to `named::compile` for every effective command
definition. A slot name is checked there, and its cardinality is checked before
the resolved Templates snapshot is returned.

A vocabulary supplied only to `expand` would move those failures to the first
attempt to launch a key. The current API can instead validate a selected
configuration before launching any of its commands. Dormant definitions remain
structurally checked without compiling their templates; an effective binding is
what activates compilation. The module-decomposition spec records this placement
of the vocabulary at load, and chapter 4 explains the compiler's count check.

There is a second cost, and the signature shows it. `Vocabulary<'a>` and
`SlotRule<'a>` borrow, because a consumer's rules are normally a `const` array
with a `'static` lifetime, and `compile_vocabulary` copies them into owned
`SlotSpec`s exactly once per load. One copy per configuration is what lets a
compiled template refer to a slot by its **position** in that table rather than
by its name — which is the thread this chapter picks up again at `Word`.

<a id="named-bare"></a>
## One slot, named bare

`SlotRule` is the vocabulary's element, and it is two fields: a name and a
cardinality. The two comment lines above it fix the spelling, which is the only
piece of syntax this crate defines for itself.

<!-- fragment «vocabulary-slot-rule» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="16-22" parent="vocabulary" -->
````rust
/// One slot, named bare. A slot named `prompt` is written `${prompt}` in a
/// template; the crate never learns what the name means.
pub struct SlotRule<'a> {
    pub name: &'a str,
    pub requirement: Requirement,
}

````
<!-- /fragment -->

A slot is named bare in the vocabulary and written `${name}` in a template.
`named::compile` recognizes `${name}` during its left-to-right dollar scan and
requires a runtime slot to fill a whole argument. `$$` escapes one literal dollar,
so `$${name}` remains text. Parameter references use the distinct `${param.name}`
namespace and may fill part of a word; chapter 4 explains both forms.

The comment's closing clause is the crate's whole position on meaning, and it is
checkable rather than aspirational: a slot name is compared with `==` in three
places and interpreted in none. `compile_vocabulary` compares names to reject a
duplicate, `named::compile` compares them to resolve a substitution, and
`match_values` compares them to pair an offered value with a declared slot.
Chapter 5 reads the third. No fourth site exists, and no site anywhere in the
crate branches on a particular name.

<a id="a-cardinality"></a>
## A cardinality with two cases, and the two messages it owns

`Requirement` answers one question, and its comment names which one: how often a
slot may appear in one template. It is not a statement about whether a value must
be offered when that template is expanded, and the two are easy to conflate
because the words are the same. The distinction is settled at the end of this
section.

<!-- fragment «vocabulary-requirement» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="23-29" parent="vocabulary" -->
````rust
/// How often a slot may appear in one template.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Requirement {
    ExactlyOnce,
    AtMostOnce,
}

````
<!-- /fragment -->

Two cases and no third. There is no `AtLeastOnce`, no bounded count and no
per-slot default, so the whole cardinality language a consumer can write is
*required once* and *optional*. Two of the five derives are load-bearing inside
the crate: `Clone` and `Copy` are why `admits` and `violation` can take `self` by
value and why `SlotSpec` stores a `Requirement` rather than borrowing one.
`PartialEq`, `Eq` and `Debug` have no caller anywhere in the workspace. They are
there because the type is public and a field-less public enum that cannot be
compared or printed is awkward for a consumer, which is a claim about an exported
API rather than about this crate's own needs.

The two methods below are the whole of the type's behaviour, and both are
`pub(crate)`. A consumer constructs a `Requirement` and never asks it anything;
the only caller of either is `named::compile`, in chapter 4.

<!-- fragment «vocabulary-cardinality-and-message» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="30-46" parent="vocabulary" -->
````rust
impl Requirement {
    pub(crate) fn admits(self, occurrences: usize) -> bool {
        match self {
            Self::ExactlyOnce => occurrences == 1,
            Self::AtMostOnce => occurrences <= 1,
        }
    }

    pub(crate) fn violation(self, name: &str) -> String {
        match self {
            Self::ExactlyOnce => {
                format!("command template must contain `${{{name}}}` exactly once")
            }
            Self::AtMostOnce => format!("`${{{name}}}` may appear at most once"),
        }
    }
}
````
<!-- /fragment -->

`admits` is the predicate and `violation` is the sentence for the case where the
predicate is false. `named::compile` compiles a template's words, counts how
many of them resolved to each slot, and then walks the slot table calling
`admits` with that count; a `false` sends it straight to `violation` for the
message. Splitting a predicate from its message this way is what keeps both arms
of the wording in one file with the rule they belong to, rather than at the call
site where the count happens to be in scope.

Both messages name the slot in its **template** spelling rather than its bare
one, which is why `ExactlyOnce`'s arm needs the doubled braces of
`` `${{{name}}}` `` to emit a single `${` and a single `}` around the
interpolated name. The operator reading the message is looking at a file that
contains `${prompt}`, not at a vocabulary that contains `prompt`.

| Case | `admits(n)` is true when | The sentence `violation` builds for `prompt` |
|---|---|---|
| `ExactlyOnce` | `n == 1` | ``command template must contain `${prompt}` exactly once`` |
| `AtMostOnce` | `n <= 1` | ``` `${prompt}` may appear at most once ``` |

That table is the type's entire contract: the middle column is what decides, and
the right-hand column is what an operator reads. Both strings are asserted
verbatim by `template_failures_are_aggregated_with_source_locations`
in `crates/keyed-launch/tests/templates.rs`, which loads one document with five
faulty nodes and requires the single report to carry a finding for each.

The conflation this section opened on is worth closing before the example.
`AtMostOnce` bounds how often a name may appear in a template; it says nothing
about expansion, where the rule is different and stricter. Expansion requires one
value for **every** declared slot, whatever its cardinality, so a vocabulary of
four slots is expanded with four values even when the template being expanded
mentions one of them. `expansion_refuses_values_that_do_not_fill_the_vocabulary`
pins that: a `label` slot declared `AtMostOnce`, with no value offered, produces
`no value offered for declared slot: label`. Chapter 5 owns the function that
raises it, and the reason the obligation is stated over the vocabulary rather
than over the template's own words.

<a id="the-four-slots"></a>
## The four slots

Grove supplies four slots: `prompt` is `ExactlyOnce`, while `session_name`,
`worktree` and `repo` are `AtMostOnce`. The runner knows these names and counts,
without knowing what their values mean. Consider an active review definition
whose author omitted the prompt:

```kdl
config {
    command "reviewer" "codex exec --model gpt-5"
    bind "review" "reviewer"
    route "review-impl" "review"
}
```

Its declaration shape and quoting are valid, but compilation reports that the
command template must contain `${prompt}` exactly once. The diagnostic points
at the command definition's captured span. The binding activates compilation
even before any caller requests expansion of `review-impl`.

| Rule about a runtime substitution | Decided by | Needs the slot table |
|---|---|---|
| It occupies a complete argument | `named::compile` | no |
| Its name is declared | `named::compile` | yes |
| Its count satisfies its requirement | `named::compile`, through `Requirement::admits` | yes |

Parameter references are checked against the command's declarations instead of
the runtime vocabulary. They may occupy fragments inside an argument, while a
runtime slot remains a whole argument. Both preserve boundaries when their
values are eventually filled.

Both `Catalog::load` and the delegating `Templates::load` take the vocabulary.
Conformance uses the vocabulary already captured by Catalog. Expansion receives
only values: compiled words already identify validated runtime slot names.

<a id="inspection-records"></a>
## Explaining a captured resolution

A caller borrowing `Templates::inspect()` needs to distinguish a winning command
from the assignments it replaced. These output records make that distinction
explicit. They are owned by the snapshot and remain available after Catalog and
its files disappear. Chapter 3 builds target histories; chapter 5 consumes the
same compiled words at expansion.

<!-- fragment «inspection-records» owner="rules-about-names" source="crates/keyed-launch/src/inspection.rs" lines="1-104" parent="source-inspection" -->
<!-- insert «inspection-assignments» -->
<!-- insert «inspection-words» -->
<!-- insert «inspection-commands» -->
<!-- insert «inspection-snapshot» -->
<!-- /fragment -->

`Origin` locates a declaration in captured input. `Setting` names the scope
being assigned, and `AssignmentHistory` keeps every applied value with its total
order and origin ID. A route or binding replacement adds another `Set` to its
target history. Parameter assignments use their own setting scopes; `Unset`
removes an override while retaining its place in the history.

<!-- fragment «inspection-assignments» owner="rules-about-names" source="crates/keyed-launch/src/inspection.rs" lines="1-44" parent="inspection-records" -->
````rust
//! Owned explanation records. These are output, never authority to construct an Argv.
use crate::{Occurrence, Selection, Source, SourceSpan};

/// One declaration's location. IDs are response-local indices into `origins`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Origin {
    pub id: usize,
    pub span: SourceSpan,
    pub occurrence: Option<usize>,
}

/// The scope and name of an assigned setting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Setting {
    BindingTarget { binding: String },
    RouteTarget { key: String },
    ParameterDefault { command: String, parameter: String },
    CommandParameter { command: String, parameter: String },
    RouteParameter { key: String, parameter: String },
}

/// An assignment sets a value or removes an override to expose inheritance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssignmentValue {
    Set(String),
    Unset,
}

/// An application in total fold order, referencing an origin ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    pub order: usize,
    pub value: AssignmentValue,
    pub origin: usize,
}

/// All assignments to a setting, including overwritten values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignmentHistory {
    pub id: usize,
    pub setting: Setting,
    pub assignments: Vec<Assignment>,
}

````
<!-- /fragment -->

`CompiledWord` is the representation validation builds and expansion reads.
Inspection copies those words into `WordView` and associates origin IDs; it never
parses a display string. Every word references its command definition and any
contributing parameter origins; `Slot("prompt")` still awaits a runtime value.

<!-- fragment «inspection-words» owner="rules-about-names" source="crates/keyed-launch/src/inspection.rs" lines="45-59" parent="inspection-records" -->
````rust
/// A compiled word, shared by template validation, inspection and expansion.
/// A runtime slot stays symbolic until expansion supplies its native value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompiledWord {
    Literal(String),
    Slot(String),
}

/// One word and every declaration contributing to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WordView {
    pub word: CompiledWord,
    pub origins: Vec<usize>,
}

````
<!-- /fragment -->

`CommandView` exposes an admitted key's executable-first words and the IDs
needed to explain them. Every successful command has a binding and a named
command definition, with resolved parameters when that definition declares them. `NonAdmittedKey` keeps an overlay-only declaration visible without
making it launchable; its reason explains the missing primary authority.

<!-- fragment «inspection-commands» owner="rules-about-names" source="crates/keyed-launch/src/inspection.rs" lines="60-88" parent="inspection-records" -->
````rust
/// A resolved parameter and its contributing origin and history IDs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterView {
    pub name: String,
    pub value: String,
    pub origins: Vec<usize>,
    pub histories: Vec<usize>,
}

/// One admitted command, executable first, with its binding and definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandView {
    pub key: String,
    pub binding: String,
    pub command: String,
    pub parameters: Vec<ParameterView>,
    pub words: Vec<WordView>,
    pub origins: Vec<usize>,
    pub histories: Vec<usize>,
}

/// A declared key that primary policy did not authorize.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonAdmittedKey {
    pub key: String,
    pub origins: Vec<usize>,
    pub reason: String,
}

````
<!-- /fragment -->

`Inspection` owns the response arrays. Origin and history IDs index those
arrays within this response, while spans address the original UTF-8 source bytes
and source paths remain native. The view carries no runtime values and grants no
way to create an `Argv`; only validated Templates can expand one.

<!-- fragment «inspection-snapshot» owner="rules-about-names" source="crates/keyed-launch/src/inspection.rs" lines="89-104" parent="inspection-records" -->
````rust
/// A captured resolution's explanation, independent of later source changes.
/// Sources follow primary then overlay order. Origins and assignments follow
/// base, included/selected patches and overlay, in source order within each patch.
/// Commands and non-admitted keys follow key order; histories follow setting order.
/// Origin/history IDs index their respective vectors; spans address the original
/// UTF-8 source bytes. The view is never accepted as input to expansion or launch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inspection {
    pub sources: Vec<Source>,
    pub selection: Selection,
    pub profile_occurrences: Vec<Occurrence>,
    pub commands: Vec<CommandView>,
    pub non_admitted_keys: Vec<NonAdmittedKey>,
    pub origins: Vec<Origin>,
    pub histories: Vec<AssignmentHistory>,
}
````
<!-- /fragment -->

<a id="the-compiled-shapes"></a>
## What a loaded configuration is

The following shapes separate original inputs from the resolved command map.
Catalog captures the documents and vocabulary; Templates retains that capture
and its winning commands. The validation helper types remain private. Chapters
3 and 5 construct and consume these shapes respectively.

<!-- fragment «template-shapes» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="1-150" parent="source-templates" -->
<!-- insert «template-shapes-imports» -->
<!-- insert «template-shapes-templates» -->
<!-- insert «template-shapes-slot-spec» -->
<!-- insert «template-shapes-per-key-source» -->
<!-- insert «template-shapes-word» -->
<!-- insert «template-shapes-document-role» -->
<!-- insert «template-shapes-diagnostics» -->
<!-- /fragment -->

The imports are a short inventory of what a validating loader needs, and they are
worth one reading because the file's dependencies are the chapter's claim in
miniature. Two of the crate's three external dependencies are absent: `libc` and
`shell-words` belong to other files — `shell_words::split` is called through its
full path in chapter 4 rather than imported — so the only third-party name here
is `kdl`, and the import names exactly two of its types, a document and a node.
A third, `kdl::KdlError`, is named by full path in chapter 3, where the only
parse call is.

<!-- fragment «template-shapes-imports» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="1-19" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

The standard-library imports support file reading, ordered template maps,
duplicate lookup and native runtime strings. The internal imports connect argv,
error records and vocabulary to the validator; no launch operation is imported.

`SourceRole`, `Source` and `SourceSpan` identify an input and a byte range;
Selection carries a profile list and an optional declaration origin. A caller's
list normally has no origin. An empty document returns no selection declaration.
Resolution expands explicit selections and includes into distinct occurrences;
unknown names and active-stack cycles retain the path that reached them.

`Catalog` owns an `Arc<Captured>`. Each CapturedDocument keeps its original text,
parsed KDL and compiled declarations, so overwritten and overlay-only commands
are not erased by resolution. Templates owns the merged map and shares this
capture. Its underscore-prefixed retained fields are intentionally unread by the
base resolver; inspection uses their captured declarations without loading files again.

<!-- fragment «template-shapes-templates» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="20-86" parent="template-shapes" -->
````rust
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

/// Retain the loaded bytes and parse tree alongside captured declarations.
/// Resolution uses `named`; source spans retain offsets without reading these bytes.
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

````
<!-- /fragment -->

Templates' fields are private, and its five public methods — `load`, `source`,
`require`, `expand` and `keys` — are chapters 3 and 5's. Read as a data model,
the resolved fields answer the consumer's questions. `primary` and `overlay` are
kept so a refusal can name the files by path rather than describing them;
`slots` is the owned table `compile_vocabulary` built, and
it is the reason `Templates` needs no lifetime parameter despite being built from
a borrowed `Vocabulary<'a>`. `templates` is the resolution result, keyed by the
consumer's opaque strings and ordered because it is a `BTreeMap`, which is what
makes `keys()` — chapter 5's ten lines — return names in a stable order for a
diagnostic to print.

`overlay_only` retains each non-admitted key and its captured declaration span.
`unresolved` uses membership to explain why the key cannot run; `require` adds
the span as a related location. This keeps the refusal useful after the overlay
file disappears and distinguishes a typo from a declaration in the wrong file.

`SlotSpec` is the owned form of a `SlotRule`, and it carries no comment because
it needs none once its counterpart has one.

<!-- fragment «template-shapes-slot-spec» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="87-92" parent="template-shapes" -->
````rust
#[derive(Clone)]
struct SlotSpec {
    name: String,
    requirement: Requirement,
}

````
<!-- /fragment -->

The difference from `SlotRule` is the whole of the type's reason for existing: a
`String` where the vocabulary had a `&'a str`. `Requirement` is `Copy` and needs
no conversion. `compile_vocabulary` performs the copy once, refusing a duplicate
name as it goes, and from that moment the consumer's slice may be dropped while
the `Templates` value outlives it — which is what a configuration loaded at
startup and expanded much later requires.

<a id="the-file-it-was-read-from"></a>
## The file it was read from

`Template` carries compiled words and the command definition's source path.
The path serves `source()` and runtime errors. Inspection separately retains
route, binding and parameter spans, including declarations in the overlay.

<!-- fragment «template-shapes-per-key-source» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="93-101" parent="template-shapes" -->
````rust
/// One key's compiled words and the command definition's source path.
/// Runtime expansion errors name that definition; inspection retains the
/// separate route, binding and parameter origins that contributed to the words.
#[derive(Clone)]
struct Template {
    words: Vec<Word>,
    source: PathBuf,
}

````
<!-- /fragment -->

Definitions are personal-only, so even a route redirected by an overlay uses a
personal command definition. `Templates::source` reports that definition's file,
not the source of the route or parameter override. The inspection record is the
surface for explaining the full composition; a single path cannot express it.
The per-key path remains beside the compiled words so expansion can report the
relevant definition without rereading configuration.

<a id="a-word-and-a-role"></a>
## A named word, and a role that is only a noun

`Word` aliases `CompiledWord`, so validation, expansion and inspection share
one representation. The alias keeps the validator concise without introducing
a second word format.

<!-- fragment «template-shapes-word» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="102-104" parent="template-shapes" -->
````rust
/// Validation and expansion use the same literal/slot representation as inspection.
type Word = CompiledWord;

````
<!-- /fragment -->

A slot retains its validated name. `compile_vocabulary` owns the names and
cardinalities before validation, so a compiled slot can only name a declared
runtime value. `match_values` later checks the offered values against that same
vocabulary; expansion associates them by name and copies each native value as
one word. Inspection can expose that name directly without reconstructing it
from a private index.

The two variants also settle what a template is not. There is no `Word::Command`,
no `Word::Concat` and no variant for a word that is part literal and part slot: a
compiled word is either bytes from the file or one whole substituted value, which
is the compiled restatement of the whole-word rule chapter 4 enforces and chapter
5 relies on.

`DocumentRole` is the last of the validation shapes to carry an argument, and the
argument is about what it does *not* change.

<!-- fragment «template-shapes-document-role» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="105-133" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

DocumentRole supplies the human noun and structured SourceRole. Parsing uses it
to attach the correct file identity and to restrict command and profile
definitions to personal policy. Both sources receive structural checks; active
personal route authority is applied later during resolution. A malformed local
declaration names the overlay, while compiled command text comes from a personal
definition. The role therefore carries both diagnostic identity and the
source restrictions, without making the runner discover either file itself.

<a id="what-a-diagnostic-carries"></a>
## What a diagnostic carries

The last two types are the shape of a validation report. There is no comment
on any of them, and what they are for is legible only from their fields.

<!-- fragment «template-shapes-diagnostics» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="134-150" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

`SourceLocation` carries one-based line/column coordinates for human output and
zero-based start/end byte positions for structured reports. The parser supplies
the node span; computing columns counts Unicode scalar values rather than bytes.
The captured document stays immutable, so those positions refer to its original
UTF-8 text.

`ValidationDiagnostic` carries a category, remedy, optional key and location,
and related declaration locations. `at_node` supplies a concrete node location;
modular duplicate checks attach the earlier declaration as a related span.
`render_diagnostics` turns these internal values into public records, and Catalog
combines independent findings across documents before resolution.

Every shape the next three chapters need is now on the page, and each of them is
a rule about a name or the residue of one: a table of names, a template compiled
into literal words and validated slot names, a role that changes a noun and no
rule, and a diagnostic that can say where without being asked what any of it
meant.

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Two documents and explicit targets](03-two-documents.md)
