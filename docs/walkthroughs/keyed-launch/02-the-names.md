# The names a template is written against
<!-- book-page id="the-names" slice="rules-about-names" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Two documents, neither one assembled](03-two-documents.md)

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

The chapter owns two blocks. `src/vocabulary.rs` is forty-four lines and three
types, and it is the whole vocabulary a consumer has for talking to this crate
about names. The first ninety-one lines of `src/templates.rs` are the imports and
the eight shapes a loaded configuration compiles into; chapters 3, 4 and 5 are
the functions that build and consume those shapes, and each of those chapters is
shorter for having them already on the page. Nothing in either block reads a
file, spawns a process, or produces a diagnostic of its own. This is the chapter
of declarations.

<a id="an-input-to-load"></a>
## The vocabulary is an input to `load`, not to `expand`

`src/vocabulary.rs` opens with the argument itself. Nine of its first twelve
lines are the doc comment on `Vocabulary`, and they are the only place under
`src/` where the position is stated rather than relied on — everywhere else in
the crate it is treated as settled. The whole file follows here in four
fragments, cut at its own type boundaries.

<!-- fragment «vocabulary» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="1-44" parent="source-vocabulary" -->
<!-- insert «vocabulary-supplied-at-load» -->
<!-- insert «vocabulary-slot-rule» -->
<!-- insert «vocabulary-requirement» -->
<!-- insert «vocabulary-cardinality-and-message» -->
<!-- /fragment -->

The first fragment is the type and its reason. `Vocabulary` is one field — a
borrowed slice of `SlotRule` — and the nine lines above it are why that slice is
a parameter of the loader rather than of expansion.

<!-- fragment «vocabulary-supplied-at-load» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="1-13" parent="vocabulary" -->
````rust
/// The slot vocabulary a consumer's templates are written against.
///
/// **Supplied at load, because every template rule is a rule about slot
/// *names*.** That a substitution occupies a whole word, that it names a
/// declared slot, that a required slot appears exactly once and an optional one
/// at most once — none of them is checkable by a loader that will not learn the
/// names until expansion. Handed the vocabulary at load, the whole of both
/// documents is checked before anything is spawned, and expansion is left with
/// one obligation: that the values offered fill the slots declared here.
pub struct Vocabulary<'a> {
    pub slots: &'a [SlotRule<'a>],
}

````
<!-- /fragment -->

The comment's central sentence lists three rules and says none of them is
checkable by a loader that will not learn the names until expansion. The
mechanism behind that sentence is one table threaded through five functions, and
chapters 3 and 4 read all five. `load`'s first act is `compile_vocabulary`, which
copies the borrowed rules into the owned `Vec<SlotSpec>` the `Templates` value
keeps. That slice is then a parameter of `parse_and_validate`, of
`validate_document`, of `validate_node`, of `validate_template` and of
`parse_template_word` — the last two being the only places in the crate that ask
a `Requirement` or a slot name anything. Every one of those calls happens inside
`load`, before it returns, and for both documents.

What a loader without the names answers is not a weaker version of that; it is a
different answer. `parse_template_word` looks a name up in the slot table and
returns `Word::Slot(index)` when it finds it; when it does not, it records
`unknown substitution` and returns the word as a **literal**. A table with no
matching name in it therefore turns every substitution into a literal, and the
rule immediately after it — that word zero must be a literal executable — stops
firing, because a `${prompt}` standing alone as word zero now *is* a non-empty
literal. The template `impl "${prompt}"` is refused when the loader holds the
name and accepted when it does not.
`word_zero_must_be_a_literal_executable` in
`crates/keyed-launch/tests/templates.rs` pins the refusal the table makes
possible. That is the shape of the cost, and it is not laxity: the answers move
in both directions at once, since the same empty table also reports a perfectly
good `${prompt}` as an unknown substitution.

The alternative is a vocabulary supplied per call, as an argument to `expand`. It
takes a parameter off `load`, and it would let one loaded configuration be
expanded against more than one slot set. Its cost is that every rule above
becomes just-in-time and per key. A document would load whatever it contained;
`review-impl`'s missing `${prompt}` would surface the first time a launch of that
key was attempted, which is a running program's failure rather than a
configuration's. `load`'s own doc comment states the property the current shape
buys — that a malformed template for a key this run will never reach still fails
at load, before anything is spawned — and that sentence is not true under the
alternative. Chapter 3 owns that comment and the function under it.

`docs/specs/module-decomposition.md`'s decision 7 is the record this chapter
keeps. It settles the vocabulary's position in the signature on exactly this
ground, and states the consequence in the same terms the comment does: a
vocabulary supplied per call would make every template rule just-in-time. The two
lines that keep it are `load`'s third parameter and `validate_template`'s closing
loop over the slot table. The book names that record and does not link it; a
book's local link targets are its own pages, its own roots, the guide and the
glossary, and a specification is none of those.

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

<!-- fragment «vocabulary-slot-rule» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="14-20" parent="vocabulary" -->
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
`whole_substitution` is the four lines that convert between the two — it strips
`${` and `}` and rejects a word holding a second `}` — and chapter 4 reads them.
There is no second spelling: no `$name`, no `%name%`, no nesting, and no escape
for a literal `${`. A word that contains `${` without being one is what chapter
4 refuses as `substitutions must occupy a complete shell word`.

The comment's closing clause is the crate's whole position on meaning, and it is
checkable rather than aspirational: a slot name is compared with `==` in three
places and interpreted in none. `compile_vocabulary` compares names to reject a
duplicate, `parse_template_word` compares them to resolve a substitution, and
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

<!-- fragment «vocabulary-requirement» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="21-27" parent="vocabulary" -->
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
the only caller of either is `validate_template`, in chapter 4.

<!-- fragment «vocabulary-cardinality-and-message» owner="rules-about-names" source="crates/keyed-launch/src/vocabulary.rs" lines="28-44" parent="vocabulary" -->
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
predicate is false. `validate_template` compiles a template's words, counts how
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
verbatim by `schema_and_template_failures_are_aggregated_with_source_locations`
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

The example the book carries is grove's own configuration, and this chapter takes
the first step of it: the vocabulary value, and what holding it lets `load`
refuse. The primary document is the two lines chapter 1 put on disk, at
`~/.config/grove/config.kdl`. The vocabulary is grove's four slots — `prompt`
declared `ExactlyOnce`, and `session_name`, `worktree` and `repo` declared
`AtMostOnce` — built once as a slice of four `SlotRule`s and handed to `load` as
its third argument. The crate learns their spelling and their cardinality, and
nothing else; the four names are meaningful to grove and opaque here.

Loaded as chapter 1 wrote them, both lines pass. Now suppose the operator edits
the second line and drops its substitution, which is the single most likely
mistake a template invites, because the line still reads as a complete command.

```text
impl "claude --model opus ${prompt}"
review-impl "codex exec --model gpt-5"
```

That document is well-formed KDL. Both nodes have one positional string
argument, neither declares a property or a child block, there is no duplicate
key, and `shell_words::split` splits the second line into four ordinary words
with nothing to object to. Every check that does not consult the vocabulary
passes it. The load nevertheless fails, and it fails at the moment grove reads
its configuration rather than at the moment it launches a review.

```console
invalid configuration at ~/.config/grove/config.kdl:
  - ~/.config/grove/config.kdl:2:1: key `review-impl`: command template must contain `${prompt}` exactly once
```

Those two lines are the observable end of this chapter's example, and every part
of them comes from a different place in the crate. `invalid configuration at`
comes from `DocumentRole::Primary`'s noun, in this chapter's last fragment but
one. The location `2:1` is a `SourceLocation`, computed from the node's byte
offset. The ``key `review-impl`:`` prefix is `at_template`. And the sentence itself
is `Requirement::ExactlyOnce`'s arm of `violation`, from
`src/vocabulary.rs` line 39. Chapters 3 and 4 own the functions that assemble
them; this chapter owns the two types that supply the last two.

The refusal is available at load because — and only because — the loader was
holding the names. The table below is what the vocabulary buys, rule by rule.

| Rule about a substitution | Decided by | The diagnostic | Needs the slot table |
|---|---|---|---|
| it occupies a complete word | `parse_template_word` | ``substitutions must occupy a complete shell word, got `pre${prompt}` `` | no |
| the name inside it is declared | `parse_template_word` | ``unknown substitution `${prmopt}` `` | yes |
| it appears as often as its cardinality allows | `validate_template`, through `Requirement::admits` | ``command template must contain `${prompt}` exactly once`` | yes |

Read the last column downward: one of the three rules survives a loader that does
not know the names, and it is the one that decides nothing on its own. It is also
the precondition for the other two — `parse_template_word` asks
`whole_substitution` first, and only a word that is *nothing but* `${name}`
reaches the lookup at all, so a partial substitution never becomes a slot and
never gets counted. The three are one discipline with one gate, and two thirds of
it exists only while the loader holds the vocabulary.

That is the sense in which this refusal is checkable only here. `load` is one of
two functions in the crate's public surface that takes a `Vocabulary` — the other
is `conformance::check`, chapter 9's, which exists to run this same validation
over a consumer's real configuration from that consumer's test suite. `expand`
does not take one and cannot: by the time it runs, the names have been compiled
away into indices, which the rest of this chapter reads.

<a id="the-compiled-shapes"></a>
## What a loaded configuration is

The rest of the chapter is the head of `src/templates.rs`: eight type
declarations and the imports above them, which are between them the entire data
model of the crate's configuration half. Four of the eight are the loaded
configuration — `Templates`, and the three types it is built out of — and four
are the machinery that validates a document on the way to producing one. Exactly
one of the eight, `Templates` itself, is public; the rest are visible only to the
three chapters that follow. The block is read here, ahead of the functions,
because chapters 3, 4 and 5 all build or consume these shapes and none of them
declares one.

<!-- fragment «template-shapes» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="1-91" parent="source-templates" -->
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

<!-- fragment «template-shapes-imports» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="1-14" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

The standard-library imports say what the file does: read a path to a string,
build ordered and unordered maps, hold an `OsString`, and write into a `String`
with `fmt::Write`. `BTreeMap` and `BTreeSet` are ordered rather than hashed, and
`HashMap` appears once beside them; chapter 3 reads the reason each is where it
is. The three internal imports are the seam: this file names `Argv` and `Slot`
from the argv module, `ConfigError` from the error module, and `Requirement` and
`Vocabulary` from the vocabulary module — and it does not name `run`, `Channel`
or anything else from the launch half. That absence is the compile-time half of
the two-halves claim chapter 1 made; chapter 5 reads the other half at
`Argv::new`.

`Templates` is the type a consumer holds after a successful load, and its five
fields are the whole of what survives validation. The doc comment states the
crate's spine in the form it takes here: one complete command template per key,
read whole out of one file.

<!-- fragment «template-shapes-templates» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="15-28" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

Every field is private, and the type's five public methods — `load`, `source`,
`require`, `expand` and `keys` — are chapters 3 and 5's. Read as a data model,
the five fields answer five different questions. `primary` and `overlay` are kept so a refusal can name the files by path rather
than describing them; `slots` is the owned table `compile_vocabulary` built, and
it is the reason `Templates` needs no lifetime parameter despite being built from
a borrowed `Vocabulary<'a>`. `templates` is the resolution result, keyed by the
consumer's opaque strings and ordered because it is a `BTreeMap`, which is what
makes `keys()` — chapter 5's ten lines — return names in a stable order for a
diagnostic to print.

`overlay_only` is the field with an argument attached, and its comment makes it:
the keys an overlay declares that the primary does not are **kept** rather than
dropped, purely so that a later refusal can distinguish a key that is misspelled
from a key that is written down in the wrong file. Nothing reads the set except
`unresolved`, and nothing about resolution would change if the set were discarded
at load. It exists for the wording of one error message, and chapter 3 shows the
two sentences it chooses between.

`SlotSpec` is the owned form of a `SlotRule`, and it carries no comment because
it needs none once its counterpart has one.

<!-- fragment «template-shapes-slot-spec» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="29-33" parent="template-shapes" -->
````rust
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

`Template` is two fields, and the second is this chapter's other argued claim. A
compiled template carries the path of the file it came from, per key, rather than
the configuration carrying one path for all of them.

<!-- fragment «template-shapes-per-key-source» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="34-45" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

The claim rests on a fact about resolution that chapter 3 proves and this chapter
only needs stated: an overlay replaces a key's template whole, key by key. After
a load with an overlay, `impl` may have come from the overlay and `review-impl`
from the primary, and the two are equally valid outcomes of one call. There is
therefore no single answer to *which file did this configuration come from*, and
a diagnostic that named the configuration's path would be naming a file that, for
the failing key, may never have held a template at all.

The cost of the alternative is a specific and quiet one. A per-configuration path
is one `PathBuf` instead of one per key, and it would be right whenever no
overlay is in play — which is most of the time, and is why the defect would
survive a casual test. It would be wrong exactly when an overlay is in use, which
is the case an operator is least able to reason about unaided, because the file
they are reading is not the only file in play.
`an_overlay_replaces_a_whole_template_and_reports_its_own_path` in
`crates/keyed-launch/tests/templates.rs` is the test that holds it: it loads a
primary declaring two keys and an overlay declaring one, and then asserts that
`source` returns the primary's path for one key and the overlay's for the other
in the same loaded value.

The public reader for this field is `Templates::source`, and chapter 3 reads it
together with `load`. What the field costs is one `PathBuf` per key, cloned from
the document's path as each template is inserted; the crate accepts that
duplication rather than the ambiguity.

<a id="a-word-and-a-role"></a>
## A word by index, and a role that is only a noun

`Word` is the compiled form of one shell word, and it is where the vocabulary
finally disappears from the data.

<!-- fragment «template-shapes-word» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="46-52" parent="template-shapes" -->
````rust
/// A compiled template word: a literal, or the slot it stands for by index into
/// [`Templates::slots`].
enum Word {
    Literal(String),
    Slot(usize),
}

````
<!-- /fragment -->

A slot is a `usize`, not a name. The index is a position in `Templates::slots`,
the table `compile_vocabulary` built at the top of `load`, and it closes the
thread this chapter began. The vocabulary is supplied at load, so a single owned
table exists for the life of the configuration; because that table exists, a
compiled word can refer to a slot by position; and because it refers by position,
expansion never needs to compare a name against a template again — chapter 5's
`match_values` builds a vector of offered values indexed by the *same* table and
`expand` reads it with `offered[*index]`. Each step is a consequence of the one
before it, and none of them is available to a loader handed its vocabulary per
call.

The two variants also settle what a template is not. There is no `Word::Command`,
no `Word::Concat` and no variant for a word that is part literal and part slot: a
compiled word is either bytes from the file or one whole substituted value, which
is the compiled restatement of the whole-word rule chapter 4 enforces and chapter
5 relies on.

`DocumentRole` is the last of the validation shapes to carry an argument, and the
argument is about what it does *not* change.

<!-- fragment «template-shapes-document-role» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="53-73" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

The comment's claim is checkable in one pass: `role` is threaded through
`parse_and_validate` into `validate_document`, and inside `validate_document` it
is used in exactly one place — the call to `render_diagnostics` that builds the
failure message. It never reaches `validate_node`, `validate_template` or
`parse_template_word`, so no rule can consult it. An overlay is held to the same
syntax, the same node shape, the same duplicate check and the same slot rules as
the primary, and the single asymmetry between the two documents — that an overlay
overrides a key and never supplies one — is applied after both have passed, in
`load`. `an_invalid_overlay_fails_the_load_against_its_own_path` is the test:
it puts a template with no `${prompt}` in the overlay alone, and the load fails
with `invalid configuration overlay at` and the overlay's own path.

`noun` is the whole of the type's behaviour, and its two strings are the only
words in the crate that distinguish the documents. A reader who gets
`invalid configuration overlay at …` knows which of two files to open, which is
the entire purpose of carrying the role that far.

<a id="what-a-diagnostic-carries"></a>
## What a diagnostic carries

The last three types are the shape of a validation report, and they are the
chapter's clearest case of source that states nothing and means a good deal.
There is no comment on any of them, and what they are for is legible only from
their fields.

<!-- fragment «template-shapes-diagnostics» owner="rules-about-names" source="crates/keyed-launch/src/templates.rs" lines="74-91" parent="template-shapes" -->
````rust
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

````
<!-- /fragment -->

`SourceLocation` is a one-based line and column, `Copy` so it can be recorded in
several places without ceremony. It is computed once per node from a byte offset
and then carried, which is why every diagnostic about a key points at the key's
own line rather than at the character the problem was noticed on.

`ValidationDiagnostic` has an **optional** location, and the `Option` has one
source, which is worth naming because it is not a document-level case. Both
constructors, `at_node` and `at_template`, always fill it. The only other
construction is `validate_document`'s duplicate-key finding, which takes
`locations.first().copied()` from a vector the surrounding code has already
established is non-empty; the compiler cannot see that, so the field absorbs the
`Option` that `first` returns. The `None` is unreachable and the type carries it
anyway, which is why `render_diagnostics` — chapter 4's — branches on a location
that is always there.

The report itself is a `Vec<ValidationDiagnostic>` rather than a first error, and
that is the same decision from the other side: a finding that has to survive
alongside its neighbours cannot be a return value.

`NodeValidation` is what one node yields, and its four fields are what makes an
aggregate report possible. It carries the `key` and its `location` **whether or
not** the node was valid, so the duplicate-key check can compare keys across
nodes that individually failed; a `template` that is `Some` only when the node
compiled; and this node's own diagnostics. A validator that returned
`Result<Template, Error>` per node could not do the duplicate check at all,
because a node that failed for some other reason would have left no key behind to
compare. Chapter 3 owns `validate_document`, where those four fields are drained
into one report, and chapter 4 owns `validate_node`, which fills them.

Every shape the next three chapters need is now on the page, and each of them is
a rule about a name or the residue of one: a table of names, a template compiled
until only indices into that table remain, a role that changes a noun and no
rule, and a diagnostic that can say where without being asked what any of it
meant.

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Two documents, neither one assembled](03-two-documents.md)
