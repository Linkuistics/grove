# The tokens, and the four verdicts
<!-- book-page id="the-tokens" slice="four-verdicts" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Kind, slug, handle](03-kind-slug-handle.md)

<a id="four-verdicts"></a>
## The rule: a task-shaped name that is wrong is Malformed, never Foreign

Chapter 1 read the two files in which this crate claims the right to mean
something. This chapter is the first place that claim is spent, at the narrowest
point in the crate where it can be checked: a string in a directory listing, and
the judgement grove reaches about it.

`crates/grove-loop/src/task_name.rs` is grove's implementation of the store's
`EntryName` trait, and the whole of the seam between grove's task tree and the
library that walks it. The store hands this module two things — a name, and what
the filesystem found at it — and takes back a verdict. The store's classification
has four cases and grove's implementation produces three of them; the split
between two of those three is the rule this chapter exists for:

> A name that is *task-shaped* and does not parse is **Malformed**, which halts
> the walk. It is never **Foreign**, which skips it — and skips the whole subtree
> beneath it when the name is a directory.

That asymmetry is what could not move. The store can see that a listing entry
exists and what species the filesystem found; it cannot see that
`01-impl-domain-k29.md` was meant to be one of grove's own and was written under
a grammar grove no longer has. Nothing beneath this crate has a word for a
session kind, so nothing beneath it can tell a stray `README.md` from a task file
whose middle is spelled wrong. Grove keeps the grammar because grove is the only
layer that can tell those two apart, and it pays for keeping it by having to
classify rather than merely to parse.

This chapter owns 451 of the file's 1,743 lines in three blocks: the module
header, the imports, the three constants, `Outcome` and `TokenError`, and
`refuse_token` — lines 1 to 220 — and the two labelled sections of the file's own
inline test module that hold this chapter's claims, lines 1,207 to 1,228 and
1,342 to 1,550. The named parts of a leaf's name are chapter 3's and the parsing
itself is chapter 4's; where the header argues about either, this page says which
chapter proves it and does not prove it here.

**The carried example enters here.** `01-requirements--plan-k1.md` is the first
*leaf* `root_init` writes into a new grove; the charter `BRIEF.md` is written
before it and is itself an entry, which is why the first test on this page is
about the charter. Chapter 11 reads the verb that writes both. This chapter is where that string stops being a filename and becomes one of
four verdicts. The example's second ending enters here too, and it is a name
grove refuses: the store would have accepted the entry, and grove's grammar is
what declines it.

<a id="the-only-grammar"></a>
## The only grammar grove has

The block opens on seventy-five lines of module header, and they are the densest
argument in the crate. The header is reproduced in eight fragments because it
makes eight separate claims, four of which are proved in later chapters.

The composite below is this chapter's first ownership block. It expands, in
order, to lines 1 through 220 of the file, and the sixteen fragments it names run
from here to the end of the production half of this page.

<!-- fragment «tokens-and-verdicts» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1-220" parent="source-task-name" -->
<!-- insert «name-the-only-grammar» -->
<!-- insert «name-three-on-disk-shapes» -->
<!-- insert «name-canonicity-departure» -->
<!-- insert «name-no-caller-hands-a-path» -->
<!-- insert «name-classification-loses-data» -->
<!-- insert «name-both-words-one-rule» -->
<!-- insert «name-handle-is-this-grammar» -->
<!-- insert «name-handle-terminal-substring» -->
<!-- insert «name-imports» -->
<!-- insert «name-brief-and-key-mark» -->
<!-- insert «name-separator» -->
<!-- insert «name-outcome» -->
<!-- insert «name-outcome-infix-and-strip» -->
<!-- insert «name-token-error» -->
<!-- insert «name-token-error-traits» -->
<!-- insert «name-refuse-token» -->
<!-- /fragment -->

The first passage states what the module is and how it got here. The expand,
migrate and contract sequence it describes is the reason the header can make an
unqualified claim: while both name models were live, a reader had to check a call
site's `use` line to know which grammar a given call meant, and the header used
to enumerate the hazards that created. Deleting the other side is what turned
*the only grammar grove has* from a policy into a fact about the code.

<!-- fragment «name-the-only-grammar» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1-12" parent="tokens-and-verdicts" -->
````rust
// Grove's implementation of `ordinal_fs_tree::EntryName` — the whole seam
// between the task tree and the library that drives it (gh issue #13,
// increment 2).
//
// **This is the only grammar grove has.** It was written in the *expand* stage
// against the library's conformance kit while grove's own path-walking name
// model was still live, each verb group moved onto it in its own leaf through
// the *migrate* stage, and `sweep-k37` deleted the other side. So there is no
// longer a call site whose `use` line has to be read to know which model it
// means, and the two-grammar hazards this header used to enumerate are history
// (`docs/ARCHITECTURE.md`, *The withdrawn tree algebra*).
//
````
<!-- /fragment -->

Three shapes, and they are the whole of what grove writes into a `.grove/`
directory. Read them as the specification the rest of the file implements: a leaf
is a file with a position, an optional outcome infix, a session kind, a slug and
a key; a node is a directory with a position, a slug and a key and no infix and
no kind; and the charter is a fixed name with none of those parts.

<!-- fragment «name-three-on-disk-shapes» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="13-21" parent="tokens-and-verdicts" -->
````rust
// The three on-disk shapes, as `grammar-separator-k15` left them — that leaf
// put the `--` between a leaf's session kind and its slug and renamed every
// entry in this repo's own tree onto it, in the same session as the release
// that can read it:
//
//     leaf       NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md
//     node dir   NN-<slug>-k<key>
//     brief      BRIEF.md                     (the containing node's charter)
//
````
<!-- /fragment -->

The canonicity paragraph states the stakes of chapter 4, and it is the reason
this crate carries a grammar rather than a parser. `format(parse(f)) == f` is the
store's obligation on any domain that supplies names, and the withdrawn model
broke it in the mildest available way — by accepting a hand-typed `5` where it
would render `05`. The consequence is not a cosmetic one: two spellings that
parse to the same position and the same key are two files that name one entry,
and every count, every walk and every rename over that tree is then reading a
tree grove believes has one entry where the filesystem has two. Chapter 4 owns
the argument, the conformance kit that checks it and the refusal that replaces
the leniency.

<!-- fragment «name-canonicity-departure» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="22-31" parent="tokens-and-verdicts" -->
````rust
// **The grammar is canonical, and that was the departure from the model it
// replaced.** The withdrawn one was deliberately lenient on padding — it
// accepted a hand-typed `5` and rendered `05` — so `format(parse(f)) == f`
// failed there and one entry could occupy two files, sharing a key and a
// position. That is the library's *canonicity* obligation broken, and
// `docs/ordinal-fs-tree/models/structure.als` draws the picture under
// `witness_two_filenames_name_one_entry`. Here a lenient spelling is a refusal
// that names the spelling grove writes. The decision, its cost and the
// alternative are `docs/adr/task-names-are-canonical.md`.
//
````
<!-- /fragment -->

The second departure is smaller and is the one a caller feels. A trailing `/` on
a node's name is what a caller passing a path would produce; a listing never
produces one. Tolerating it would be a second spelling, which is the same defect
as the padding, so the tolerance was removed and the trimming pushed back to the
caller. The consequence is visible in the classification tests below, where
`01-verbs-k2/` is Foreign rather than a node.

<!-- fragment «name-no-caller-hands-a-path» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="32-37" parent="tokens-and-verdicts" -->
````rust
// A second, smaller departure, and the reason no caller may hand this a path:
// the withdrawn parser tolerated a trailing `/` on a node name for callers
// passing one. A `parse` fed by a directory listing never sees one, and
// tolerating it would be a second spelling of one name — exactly what
// canonicity forbids. Trimming a caller's argument is the caller's job.
//
````
<!-- /fragment -->

<a id="the-four-verdicts"></a>
## The classification, and where a name grammar loses data

The header's fifth passage is the one this chapter is named for. It states
where the classification's care goes and why, and it lists the three mappings
that follow from it.

<!-- fragment «name-classification-loses-data» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="38-47" parent="tokens-and-verdicts" -->
````rust
// The classification is where a name grammar loses data, so it is where the care
// goes. `Verdict` has four outcomes and the load-bearing split is between two of
// them: `Foreign` is skipped **recursively**, taking a whole subtree with it when
// the name is a directory, while `Malformed` and `Reserved` halt. So:
//
//   - `BRIEF.md`                              -> the distinguished child
//   - `NN-…-k<key>[.md]`                      -> an entry, or `Malformed` if it
//                                                does not parse completely
//   - `README.md`, anything else              -> `Foreign`
//
````
<!-- /fragment -->

The comment names the split; what it leaves for this page is what each side
costs. `Foreign` is a disclaimer — grove saying the name is not its own — and the
store acts on it by skipping the entry, and everything beneath it when the entry
is a directory. `Malformed` is a report — grove saying the name *is* its own and
is broken — and the store halts on it wherever in the tree it sits. So
classifying a task-shaped name as Foreign is not a lenient reading of it: it is a
silent deletion of every leaf under it from grove's view of the tree, while the
walk reports a healthy tree.
That is the loss the header means by *where a name grammar loses data*, and it is
why the care is concentrated at the classification rather than at the parse.

**`Reserved` is the store's, and grove never returns it.** The capitalised
variant occurs exactly twice in this file and both are comments — the header's
sentence above, and a line in the doc comment on `TaskNameError` that chapter 4
reproduces. Neither is a `Verdict::Reserved` this module constructs. `Reserved`
is how the store's other consumers name a transaction witness or a lock marker,
and grove's `.grove/` holds none: every name in it is the charter, an entry, a
broken entry, or not grove's. The verdict a reader should expect never to see is
worth knowing about precisely because the chapter is named for four of them, and
chapter 4 owns the arms of `parse` that show only three are reachable.

The four are the chapter's subject and they are argued a paragraph at a time
above, so here they are as one partition. Read the third column rather than the
first: what separates these verdicts is not what each says about a name but what
the store does next, and the difference between skipping a subtree and halting is
the whole of why the care goes to the classification.

| Verdict | What grove is saying | What the store does with it | Returned by this module |
|---|---|---|---|
| `Entry` | the name is grove's and parses — the charter, or a positioned entry | reads it | yes |
| `Foreign` | a disclaimer: the name is not grove's | skips the entry, and everything beneath it when the entry is a directory | yes |
| `Malformed` | a report: the name *is* grove's and is broken | halts, wherever in the tree it sits | yes |
| `Reserved` | nothing — it is how the store's other consumers name a transaction witness or a lock marker | — | **never** |

**Seven names this page uses before the chapter that explains them.** The verdict
is produced by the `EntryName` implementation, which is chapter 4's, so every
symbol on the path to one is named here and read there.

- **`TaskName`** is one parsed entry name: the charter, or a positioned entry
  carrying its ordinal, its key and its parts. It renders back to the bytes it
  was parsed from, or it refuses to be computed at all.
- **`TaskNameError`** is the refusal a task-shaped name that does not parse
  produces. Its variants are what the tests on this page match on; chapter 4
  defines them and the messages they render.
- **`Verdict`** is the store's own four-way classification — `Entry`, `Foreign`,
  `Malformed` and `Reserved` — and it is the store's type rather than grove's.
  Grove supplies which case a given name reaches, and the two that carry an error
  carry grove's own error type.
- **`verdict`, `entry` and `malformed`** are the three test helpers this file's
  inline tests reach a verdict through. `verdict` calls `TaskName::parse` and
  returns the classification whole; `entry` asserts the classification was
  `Entry` and unwraps the parsed name; `malformed` asserts it was `Malformed` and
  unwraps the error. Both of the latter are `#[track_caller]`, so a failure is
  reported at the assertion rather than inside the helper. Chapter 4 reproduces
  all three with the block that defines them.
- **`TaskName::Brief`** is the distinguished value supplied by the lifecycle
  callers: chapter 11 initializes with it and chapter 12 promotes into it.
  The first test below checks its classification and rendered filename.

<a id="both-words-one-rule"></a>
## Both of a leaf name's words, under one rule

The sixth passage explains why the last item in this block is a free function
over strings rather than a method on either of the two types it serves.

<!-- fragment «name-both-words-one-rule» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="48-56" parent="tokens-and-verdicts" -->
````rust
// **Both of a leaf name's words are this module's**, and since `open-kind-k20`
// they are validated by one rule. `Kind` was a compiled enum living in a module
// of its own, justified by being the key a command template is configured under;
// what it actually was, once the set opened, is the other half of the shape
// `Slug` already had — and the canonicity of a leaf name depends on the two
// obeying the *same* rule, which is a grammar fact and belongs here. So
// `src/leaf.rs` went, `refuse_token` states the shape once, and `Kind::new` and
// `Slug::new` are two nouns over it.
//
````
<!-- /fragment -->

This passage is the reason `refuse_token` exists at the end of this block rather
than a validator on each of the two types. Before `open-kind-k20` a kind was a
compiled enum checked against a closed set and a slug was a shape, so the two
words of a leaf name were validated by two unrelated rules; once the kind set
opened, the only thing left to check about a kind was the same thing already
checked about a slug. The canonicity of a leaf name depends on both words obeying
that rule, which makes it a fact about the grammar rather than about either type,
and the header places it here for exactly that reason. `Kind` and `Slug`
themselves are chapter 3's, and so are the two constructors the passage's last
sentence names: `Kind::new` and `Slug::new` each hand their string to
`refuse_token` and return the token or the one `TokenError`, which is what makes
*one rule* a fact about the code rather than an agreement between two types.

<a id="the-handle-in-this-grammar"></a>
## The handle is inside this grammar, not beside it

The seventh passage is the longest, and everything it claims is proved in
chapter 3. It is reproduced here because it belongs to the module header, and it
is read here because it states what ownership of a grammar means in this crate:
not that the rules are written down, but that there is exactly one place each of
them can be executed.

<!-- fragment «name-handle-is-this-grammar» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="57-70" parent="tokens-and-verdicts" -->
````rust
// **The handle is part of this grammar, not a second one** (`name-ownership-k14`,
// `docs/specs/module-decomposition.md` decision 4). `<slug>-k<key>` — the
// position-free identity that crosses every module boundary, from the store that
// produces it, through the prompt, to the verbs a session hands it back to — was
// spelled by four `format!`s outside this file and by both arms of the renderer
// inside it, and peeled by `split_shape` here and by `task_tree::handle_key`
// there, whose own comment conceded it *"mirrors the filename grammar"*. None of
// them was behind a type.
// It is now [`Handle`], and the ownership is structural rather than
// disciplinary: [`Handle::render`] is the only `write!` the grammar appears in,
// [`peel_key`] the only place it is taken apart, and **both of [`TaskName`]'s
// renderings end in a call to the former**. So a filename and a handle saying
// different things is not a bug this module can have — it is not expressible.
//
````
<!-- /fragment -->

The claim in the last sentence is structural, and it is worth separating the two
halves of it. *`Handle::render` is the only `write!` the grammar appears in* and
*`peel_key` is the only place it is taken apart* are facts about this file that a
reader can check by searching it. *A filename and a handle saying different
things is not expressible* is the consequence, and it holds only because both of
`TaskName`'s positioned rendering arms end in a call to the former. Chapter 3
owns `Handle` and the test that asserts the consequence rather than reviewing for
it.

Three names in the passage are read in later chapters, and none of the three is
public. `Handle::render` is a private associated function on chapter 3's
`Handle`: the renderer, reached by `Handle`'s own `Display` and by both arms of a
positioned `TaskName`'s. The `KEY_MARK` constant read in the next section has
exactly three uses in the crate — its own definition, this renderer's `write!`,
and the `strip_suffix` in `peel_key` — which is the header's claim in a form a
reader can grep for. The claim is about production code: the inline tests build
fixture names with `format!`, as any other caller writing a filename would.
`peel_key` and `split_shape` are private free functions in chapter 4's block —
`peel_key` at lines 1,012 to 1,019 returns what precedes a terminal `-k<digits>`
and the digit run, leaving each caller to judge an over-wide key for itself, and
`split_shape` at lines 967 to 975 splits a task-shaped stem into position digits,
an unexamined middle and key digits by *calling* `peel_key` rather than finding
the key itself. That call is why the header can say there is one peel.

A fourth name in the passage is read in no chapter at all. `task_tree::handle_key`
is what the sentence is written against: it was the second peel, it was deleted
with the rest of the other model, and what survives it in this crate is comments
recording that it is gone. It is the one name here the book cannot promise a
reader anywhere to go for.

<!-- fragment «name-handle-terminal-substring» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="71-76" parent="tokens-and-verdicts" -->
````rust
// The same fact read the other way: the handle is a **contiguous terminal
// substring** of every name that has one, a leaf's followed only by the `.md`
// its species takes. That is the property `grammar-separator-k15` bought with
// its rename, and with one renderer it cost that leaf one `write!` and one
// `split_once`.

````
<!-- /fragment -->

The header closes on the same fact stated from the reader's side rather than the
renderer's, and it is the form a reader can check against a directory listing:
strip `.md` from a leaf's name and the handle is what the string ends with. That
is what makes `<slug>-k<key>` findable in a filename without parsing it, and
chapter 9 reads the verb that relies on it. The parenthesis records the price:
because one renderer writes the handle, the rename that introduced the separator
cost that leaf a single `write!` and a single `split_once`.

<a id="the-three-constants"></a>
## The imports, and the three constants the grammar is spelled with

The declarations begin. Six lines, and the second `use` is the seam.

<!-- fragment «name-imports» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="77-82" parent="tokens-and-verdicts" -->
````rust
use core::fmt;

use ordinal_fs_tree::{
    EntryName, Found, Key, NameView, Ordinal, PositionedSpecies, Species, Triple, Verdict,
};

````
<!-- /fragment -->

Nine names come from the store, and `core::fmt` is the only import that is not
the store's. This module is the one that converts between the two vocabularies,
so it takes the store's `Key`, `Ordinal`, `Species` and `Found` and republishes
none of them. `EntryName` and `Verdict` are the two that carry the seam — the
trait grove implements, and the classification that implementation returns.

The grammar's own markers are three string constants, all of them in this block,
and only one carries an argument. Two further `&str` constants appear later in
the file — the two reserved kind labels at lines 246 and 249 — and they are
chapter 3's, because a reserved kind is a fact about the kind token rather than
about the grammar's punctuation.

<!-- fragment «name-brief-and-key-mark» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="83-91" parent="tokens-and-verdicts" -->
````rust
/// The name of a node's distinguished child: the charter every node directory is
/// headed by.
pub const BRIEF: &str = "BRIEF.md";

/// The permanent key's delimiter — the terminal `-k<digits>` of every positioned
/// name (task-tree-scheme, amending the original `[<key>]`: brackets are
/// shell-glob metacharacters and `-k` is glob-safe).
const KEY_MARK: &str = "-k";

````
<!-- /fragment -->

`BRIEF` is public because the charter's name is a fact other modules act on;
`KEY_MARK` is private because nothing outside this file may spell the key
delimiter. The parenthesis on `KEY_MARK` records a change of spelling rather than
of meaning: the original scheme wrote the permanent key as `[<key>]`, and square
brackets are shell-glob metacharacters, so an operator could not name a task file
in a shell without quoting it.

<!-- fragment «name-separator» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="92-104" parent="tokens-and-verdicts" -->
````rust
/// The separator between a leaf's session kind and its slug
/// (`grammar-separator-k15`, `docs/specs/module-decomposition.md` decision 3).
///
/// A single `-` cannot delimit them: both tokens are hyphenated words, so
/// `design-decomposition` reads as kind `design` + slug `decomposition` **and**
/// as kind `design-decomposition` + empty slug. Matching the middle against a
/// closed kind set was the only thing that resolved that, and `open-kind-k20`
/// took the set away — so the separator is now the *whole* of what says where
/// the kind ends, and a name without it has no reading at all rather than two.
/// The middle splits at the **first** `--`; neither token may contain one, which
/// is why [`refuse_token`] refuses it for both.
const SEPARATOR: &str = "--";

````
<!-- /fragment -->

The separator's twelve lines are the one place in this block where the comment
argues rather than records, and the argument is the ambiguity that a single dash
leaves behind. Both of a leaf name's words are hyphenated, so `design-decomposition`
in the middle of a name has two readings under a single-dash rule and nothing in
the string decides between them. A closed kind set decided it, by matching the
middle against nineteen labels; when `open-kind-k20` removed the set, the
separator became the whole of what says where the kind ends. The consequence
stated in the last sentence is what makes the grammar total rather than
ambiguous: a name without a `--` has no reading at all, which is a refusal grove
can report, rather than two readings, which is a choice grove would have to
guess at. `a_leaf_without_the_separator_is_refused_and_the_refusal_names_the_grammar`
and `a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading` are the
two tests that hold it, and both are on this page.

<a id="the-outcome"></a>
## The outcome, and the mapping written both ways

`Outcome` is the first of this block's two types, and it is the smaller half of
what a leaf's name carries beyond its position and its key.

<!-- fragment «name-outcome» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="105-120" parent="tokens-and-verdicts" -->
````rust
/// A leaf's outcome: live, retired (`DONE`), or abandoned (`ABANDONED`) —
/// mutually exclusive by construction, so the impossible fourth state cannot be
/// written. A node directory never carries one; its done-ness is the absence of
/// a live leaf in its subtree, which is why [`Parts::Node`] has no such field
/// rather than a field constrained to one value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Not yet retired or abandoned — what `pick` returns.
    Live,
    /// Work completed — the `DONE-` infix.
    Done,
    /// Work rejected, closed, not going to happen — the `ABANDONED-` infix. The
    /// *why* lives in the ADR set, not the filename.
    Abandoned,
}

````
<!-- /fragment -->

An outcome is a property of a leaf and of nothing else. The last sentence records
a design choice a reader can check against chapter 3's `Parts`: its `Node`
variant carries a slug and nothing else, having no outcome field at all rather
than a field constrained to one value, so a node carrying an outcome is not a
state the type can hold. Grove's node — a directory of numbered children — is
done when no live leaf remains anywhere in its subtree, and that is a fact about
the subtree rather than a mark on the directory. The test that refuses the mark
is `a_node_wearing_an_outcome_infix_is_malformed`, below.

`Live`'s own line names the verb the distinction is for. `pick` is grove's answer
to *what next*: a depth-first pre-order walk that returns the first leaf still
live, skipping briefs and every leaf marked `DONE` or `ABANDONED`. Chapter 7
reads that walk. This enum is the half of it the walk consults, which is why the
variant that has no infix is the one a verb is documented in terms of.

<!-- fragment «name-outcome-infix-and-strip» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="121-148" parent="tokens-and-verdicts" -->
````rust
impl Outcome {
    /// The infix this outcome takes, immediately after the position. Empty for
    /// [`Outcome::Live`], which is the absence of a mark rather than a mark.
    const fn infix(self) -> &'static str {
        match self {
            Self::Live => "",
            Self::Done => "DONE-",
            Self::Abandoned => "ABANDONED-",
        }
    }

    /// The outcome an infix names, and how much of the name it consumed.
    ///
    /// The inverse of [`Outcome::infix`], and paired with it here for the reason
    /// `cli-k16` found the hard way (`docs/formalism-findings.md` entry 019): a
    /// domain whose token mapping runs one way only gets the other direction
    /// written a second time by its first consumer, and two spellings of one
    /// mapping drift.
    fn strip(rest: &str) -> (Self, &str) {
        for outcome in [Self::Done, Self::Abandoned] {
            if let Some(after) = rest.strip_prefix(outcome.infix()) {
                return (outcome, after);
            }
        }
        (Self::Live, rest)
    }
}

````
<!-- /fragment -->

`infix` and `strip` are one mapping written in both directions, and the doc
comment states why they are adjacent rather than why either is correct. The
finding it cites is a general one: a domain that publishes only the rendering
direction gets the parsing direction written a second time by whichever consumer
first needs it, and the second copy drifts from the first. `strip` is that second
copy, brought back beside the first and defined in terms of it — it iterates over
the two marked outcomes and asks `infix` for each one's text rather than
restating `"DONE-"` and `"ABANDONED-"`. `Outcome::Live` is not in that loop
because it has no infix to strip; it is the fallthrough, which is the same
statement as `infix` returning the empty string for it.

<a id="one-refusal"></a>
## One refusal, because there is one rule

The block's last three items are one type and one function, and together they
are the whole of what this module says about a badly spelled word.

<!-- fragment «name-token-error» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="149-165" parent="tokens-and-verdicts" -->
````rust
/// Why a string is not a well-formed [`Slug`] or [`Kind`].
///
/// **One refusal because there is one rule.** A leaf name is built from two
/// words — the session kind and the slug — and whatever else they mean, each has
/// to survive being written into a filename beside the `--` that separates them
/// and read back as the same two words. A second error type would be a second
/// statement of that shape, and the two would drift the first time either word's
/// character set moved. `open-kind-k20` is what made the sharing possible: until
/// then a kind was checked against a closed set rather than against a shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenError {
    /// What is wrong with it, phrased for whoever has to fix the filename, and
    /// **naming the offending character** where there is one — "not one of
    /// those" about a forty-character name is a hunt.
    pub reason: String,
}

````
<!-- /fragment -->

`TokenError` carries a single `String`, and the field's own comment states the
obligation that string is under: it names the offending character where there is
one. That is not a courtesy. The names this grammar refuses are usually long, and
a message that says a token holds characters outside a set, without saying which
character was outside it, leaves the operator comparing a forty-character name
against a rule by eye.

<!-- fragment «name-token-error-traits» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="166-173" parent="tokens-and-verdicts" -->
````rust
impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for TokenError {}

````
<!-- /fragment -->

`Display` forwards to the single field, and `Error` is implemented without a
`source`, because a token refusal has no underlying cause to chain to: the string
is the whole diagnosis. `TokenError` is also the only error type in this file
that is not a variant of `TaskNameError`; chapter 4 reads how a refusal produced
here is carried up into one.

`refuse_token` is the rule itself. It takes the noun being checked and the
string offered for it, and returns the reason it is not usable, or nothing.

<!-- fragment «name-refuse-token» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="174-220" parent="tokens-and-verdicts" -->
````rust
/// The shape a leaf name's two words share — or the reason this string has not
/// got it.
///
/// `noun` is the word being refused, so one rule produces a message that reads
/// as though it had been written for the word in hand. Every clause here is
/// load-bearing on the grammar rather than on taste:
///
/// * **empty** — a missing word would move the `--` and change where the name
///   splits;
/// * **the grammar's own markers** — reserved so a name cannot spell one;
/// * **a leading or trailing dash** — a kind ending in one renders `impl---slug`,
///   which splits at the *first* `--` and reads back as kind `impl`, slug
///   `-slug`. Canonicity is what this clause protects, not tidiness;
/// * **the separator** — either word containing `--` gives the name two readings;
/// * **the character set** — everything outside it either blurs a name boundary
///   (`.`, `/`) or collides with the uppercase outcome infixes.
fn refuse_token(noun: &str, token: &str) -> Option<String> {
    if token.is_empty() {
        return Some(format!("a {noun} may not be empty"));
    }
    if matches!(token, "BRIEF" | "DONE" | "ABANDONED") {
        return Some(format!(
            "`BRIEF`, `DONE` and `ABANDONED` are reserved: the grammar's own markers, and a \
             {noun} spelling one would name a marker instead"
        ));
    }
    if token.starts_with('-') || token.ends_with('-') {
        return Some(format!("a {noun} may not start or end with a dash"));
    }
    if token.contains(SEPARATOR) {
        return Some(format!(
            "a {noun} may not contain `{SEPARATOR}`: that is the separator between the session \
             kind and the slug"
        ));
    }
    if let Some(refused) = token
        .chars()
        .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-'))
    {
        return Some(format!(
            "a {noun} holds lowercase ASCII letters, digits and dashes only, and {refused:?} is \
             none of those"
        ));
    }
    None
}

````
<!-- /fragment -->

The five guards are in the order the doc comment lists them, and the doc's five
bullets are one-for-one with them. Two are worth reading against the grammar
rather than against a style rule.

The **leading or trailing dash** guard is a canonicity clause, and the comment
says so. A kind ending in a dash renders `impl---slug`; that name splits at the
first `--`, which leaves kind `impl` and slug `-slug`, so the name parses to
different parts than it was composed from and no longer renders back to itself.
Refusing the dash at composition is what keeps the round trip total.

The **character set** guard rejects everything outside lowercase ASCII letters,
digits and dashes, and the comment gives two distinct reasons for two distinct
groups. `.` and `/` blur a name boundary — `.` is what separates the `.md`
suffix, and `/` is a path separator. Uppercase collides with the outcome infixes,
which are the uppercase words `DONE` and `ABANDONED`; the reserved-word guard
above already refuses those three exactly, and the character-set guard is what
stops a token from approaching them at all. `noun` is threaded through every
message so that one rule produces refusals phrased for the word actually in hand,
which is the whole of what the two nouns cost over one.

<a id="classified"></a>
## The four verdicts, in two tests

The first of this chapter's two test blocks is twenty-two lines, and it is the
file's own labelled section for the classification. It holds two tests, one for
each of the two verdicts a name can reach without ever being parsed as a
positioned entry.

<!-- fragment «classification-verdict-tests» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1207-1228" parent="source-task-name" -->
<!-- insert «name-tests-the-charter» -->
<!-- insert «name-tests-foreign» -->
<!-- /fragment -->

The first test takes the charter, which is the one name in the grammar that
carries no position, no key and no parts.

<!-- fragment «name-tests-the-charter» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1207-1214" parent="classification-verdict-tests" -->
````rust
    // ---- classification: the four verdicts ---------------------------------

    #[test]
    fn the_charter_is_the_distinguished_child() {
        assert_eq!(entry("BRIEF.md", Found::File), TaskName::Brief);
        assert!(matches!(TaskName::Brief.view(), NameView::Distinguished));
    }

````
<!-- /fragment -->

**What it establishes.** `BRIEF.md` parses as `Brief`, whose view is
`Distinguished`. Grove passes this value explicitly when initializing and
promoting; the library does not manufacture the name.

**What it would still pass under.** A caller could supply the wrong name while
this parsing test stayed green. The lifecycle tests exercise those callers and
check the files they create; the conformance kit samples the name laws.

<!-- fragment «name-tests-foreign» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1215-1228" parent="classification-verdict-tests" -->
````rust
    #[test]
    fn a_name_that_is_not_task_shaped_is_foreign() {
        for name in [
            "README.md",
            "notes",
            "01-k3.md",     // no `-k` key delimiter
            "impl-a-k1.md", // unpositioned
            "01-verbs-k2/", // a path argument's trailing slash is the caller's to trim
            ".gitignore",
        ] {
            assert_eq!(verdict(name, Found::File), Verdict::Foreign, "{name:?}");
        }
    }

````
<!-- /fragment -->

**What it establishes.** Six names that are not grove's reach `Foreign`.
`split_shape` has three ways to return nothing, and the fixtures are spread
unevenly across them rather than one to a point. `README.md` and `notes` have no leading
digit run at all. `01-k3.md` needs reading against `split_shape`, because its
inline comment is looser than the mechanism: the string does carry a `-k` before
its digit, but the position's terminating dash is consumed first, so what reaches
`peel_key` is `k3`, and `k3` has no `-k` to strip. `impl-a-k1.md` has both a middle and a key but is
unpositioned: the characters before its first dash are not a digit run. `01-verbs-k2/` carries the trailing
slash a caller passing a path would produce, which puts a non-digit after the key
and is exactly the tolerance the header's second departure removed. `.gitignore`
has neither marker. A name is grove's when it is positioned *and* keyed;
`README.md`, `notes` and `.gitignore` fail on the same first test — no dash at
all, so no digit run to read — and only the middle three fixtures each remove
exactly one of the two markers.

**What it would still pass under.** The test asserts only the Foreign side of the
boundary, so it would pass while some genuinely task-shaped name was *also*
classified Foreign — which is the failure that matters, because it is silent: the
walk would report a healthy tree with a subtree missing from it. Nothing in these
six fixtures could detect that. The other half of the boundary is held by the
labelled section below, and the two sections are complementary halves of one
claim rather than two independent claims. The test would also pass under a
grammar that classified a seventh, genuinely foreign shape as Malformed, halting
the walk on a file grove has no interest in — and nothing in this book covers
that direction, because a fixture list can only speak for the names on it.

<a id="refusals-inside-the-shape"></a>
## Refusals inside the shape

The second test block is 209 lines and holds six tests. Every fixture in it is
task-shaped — positioned and keyed — and all but two of them are refused; the two
that parse are in the round-trip test, which needs a name that succeeds in order
to show that moving the separator produces a second name rather than a second
reading. This is the section that holds the chapter's rule, and it is where the
second ending of the carried example lives: each refused name is one the store
would have accepted as an entry, and grove is what declines it.

**Five names this section uses before the chapter that explains them.**
`Parts::leaf` is the constructor for the leaf half of chapter 3's `Parts` — the
named parts a positioned leaf name decomposes into, taking an outcome, a kind and
a slug. `a_kind` and `slug` are two test helpers defined with the conformance kit
in chapter 4's block: each takes a label, builds the corresponding token type,
and panics if the label is not well-formed, so an invalid fixture is a test bug
rather than a compile error. The `to_string()` calls on an error are
`TaskNameError`'s `Display`, chapter 4's renderer at line 727, which writes each
refusal's recovery advice and not merely its detection — three of this section's
six tests assert on that rendered text, so the advice is part of what they pin
rather than commentary beside it. And the `to_string()` calls in the round-trip
test below are `TaskName`'s `Display`, chapter 4's other renderer: it writes a
parsed name back to its filename bytes, both arms of its positioned case ending
in the call to `Handle::render` the module header claimed. All five are read
where chapter 4 and chapter 3 reproduce the blocks that define them.

<!-- fragment «shape-refusal-tests» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1342-1550" parent="source-task-name" -->
<!-- insert «name-tests-kind-not-a-token» -->
<!-- insert «name-tests-missing-separator» -->
<!-- insert «name-tests-one-reading» -->
<!-- insert «name-tests-node-wearing-outcome» -->
<!-- insert «name-tests-bad-slug» -->
<!-- insert «name-tests-species-mismatch» -->
<!-- /fragment -->

The first of the six takes the session kind, which is the word `open-kind-k20`
changed most: it was checked against a closed set of nineteen labels and is now
checked against a shape.

<!-- fragment «name-tests-kind-not-a-token» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1342-1387" parent="shape-refusal-tests" -->
````rust
    // ---- refusals inside the shape -----------------------------------------

    /// A task-shaped leaf whose kind is not a well-formed **token** is
    /// Malformed, never Foreign: skipping it is lost work.
    ///
    /// **A shape refusal, and it names the character it refused**
    /// (`open-kind-k20`). It used to be a membership refusal that listed all
    /// nineteen labels — which is why `01-wrok--a-k1.md` was the fixture here
    /// and is now in the test above, parsing. What is left to refuse is what
    /// cannot be written and read back: an empty token, and one carrying a
    /// character the grammar does not spell.
    #[test]
    fn a_session_kind_that_is_not_a_token_is_malformed() {
        for (name, token, expected) in [
            // The degenerate kind: a separator with nothing before it.
            ("01---a-k1.md", "", "may not be empty"),
            ("01-Impl--a-k1.md", "Impl", "'I'"),
            ("01-DONE-my_kind--a-k1.md", "my_kind", "'_'"),
            ("01-impl.rs--a-k1.md", "impl.rs", "'.'"),
        ] {
            let error = malformed(name, Found::File);
            let TaskNameError::BadKind {
                name: reported,
                kind,
                error: token_error,
            } = &error
            else {
                panic!("{name:?} should be a bad-kind refusal, got {error:?}")
            };
            assert_eq!(reported, name, "{name:?}");
            assert_eq!(kind, token, "{name:?}");
            let advice = error.to_string();
            assert!(advice.contains("malformed Grove leaf"), "{advice}");
            assert!(advice.contains(&format!("{token:?}")), "{advice}");
            assert!(
                token_error.reason.contains(expected),
                "the refusal must name what it refused: {advice}"
            );
            // The advice states the grammar, and states no set — there is none.
            assert!(
                advice.contains("<session-kind>--<slug>-k<key>.md"),
                "{advice}"
            );
        }
    }

````
<!-- /fragment -->

**What it establishes.** A task-shaped leaf whose kind is not a well-formed token
is Malformed rather than Foreign, and the refusal carries three things an
operator needs: the name as given, the token that was refused, and a reason —
which names the offending character wherever there is one to name. Three of the
four fixtures have one; the first is the empty token, whose reason is a phrase
because there is no character to point at. The four fixtures exercise the two clauses
of `refuse_token` that a kind can realistically trip — the empty token, from a
name whose middle begins with the separator, and three characters outside the
set. `01-DONE-my_kind--a-k1.md` carries an outcome infix, which additionally
shows the infix is stripped before the kind is read.

**What it would still pass under.** Every assertion here is about the error value
and its rendering, so the test would pass in full while the store never halted on
a Malformed verdict — the halt is the library's behaviour and is not exercised
here. The assertion that the reason names the character is a `contains` check,
and so is the assertion that the advice states the grammar; both would pass on a
message that said the right thing and then also said a wrong thing. In
particular, nothing here would fail if the advice appended a list of accepted
kinds. What rules that out is that `open-kind-k20` left no set to print, which is
a fact about the code rather than an assertion in this test — the doc comment
above it records that this fixture list itself used to contain `01-wrok--a-k1.md`
and that the name now parses.

<!-- fragment «name-tests-missing-separator» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1388-1424" parent="shape-refusal-tests" -->
````rust
    /// **The scenario `grammar-separator-k15` exists to refuse.** A task-shaped
    /// leaf with no `--` is every name the old grammar wrote, so the refusal has
    /// to carry the canonical form and not merely the fact of failure — a tree
    /// written yesterday would otherwise be unreadable with no stated way back
    /// (principle 2: the advice is part of the error).
    ///
    /// The last two are the degenerate ones: a middle that is empty entirely
    /// still carries both marks grove recognises its own names by, so it is
    /// Malformed rather than Foreign — skipping it is lost work, and a whole
    /// subtree of it when the name is a directory.
    #[test]
    fn a_leaf_without_the_separator_is_refused_and_the_refusal_names_the_grammar() {
        for name in [
            "01-impl-a-k1.md",
            "01-DONE-design-decomposition-k2.md",
            "02-integrate-review-design-module-decomposition-k4.md",
            "01-a-k1.md",
            "01--k1.md",
        ] {
            let error = malformed(name, Found::File);
            assert_eq!(
                error,
                TaskNameError::MissingSeparator {
                    name: name.to_string()
                },
                "{name:?}"
            );
            let advice = error.to_string();
            assert!(advice.contains(name), "{advice}");
            assert!(
                advice.contains("NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md"),
                "{advice}"
            );
            assert!(advice.contains("rename it"), "{advice}");
        }
    }

````
<!-- /fragment -->

**What it establishes.** A task-shaped leaf with no `--` is Malformed, and the
refusal names the canonical form and tells the operator to rename. Five fixtures:
the first three are names the previous grammar wrote and are the migration case
the doc comment names; the last two are degenerate — a middle of one word, and a
middle that is empty entirely. Both degenerate names still carry the leading
digit run and the terminal `-k<digits>`, which are the two markers grove
recognises its own names by, so both are Malformed rather than Foreign. The
fixtures here are files, and the doc comment's reason for insisting on Malformed
is about directories: a name with an empty middle borne by a directory would, if
disclaimed, take its whole subtree out of the walk. The unsuffixed `01--k1` is
not a fixture of this test — it appears two tests below, under `Found::Dir`,
where it reaches a different refusal.

**What it would still pass under.** The three assertions on the message are
`contains` checks against fixed substrings, so the test would pass while the
advice also carried a second, contradictory grammar. More importantly, nothing
here checks that acting on the advice produces a name that parses: the test
asserts that a rename is *demanded*, not that any rename *succeeds*. The round
trip in the next test is what supplies that, and the conformance kit in chapter 4
is what supplies it in general. Every fixture is also `Found::File`, and that is
not the coverage gap it looks like: a node name carries no `--` by construction,
so there is no directory bearing *this* defect. What the fixtures do leave
unpinned is precedence — a `.md`-suffixed name handed to the walk as a directory
reaches this refusal before the species check runs, and nothing here says which
of the two a reader should expect.

<!-- fragment «name-tests-one-reading» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1425-1486" parent="shape-refusal-tests" -->
````rust
    /// The spec's own round-trip scenario, by name: *a multi-word kind beside a
    /// multi-word slug* (`docs/specs/module-decomposition.md`, requirement *a
    /// leaf filename has exactly one reading*). This is the name that had four
    /// readings under the old grammar and has one under this one, and the rival
    /// splits are spelled out so the assertion is about *which* reading, not
    /// merely that some reading happened.
    #[test]
    fn a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading() {
        let filename = "04-integrate-review-design--module-decomposition-k5.md";
        let name = entry(filename, Found::File);
        assert_eq!(
            name,
            TaskName::Positioned {
                ordinal: Ordinal::new(4),
                key: Key::new(5),
                parts: Parts::leaf(
                    Outcome::Live,
                    a_kind("integrate-review-design"),
                    slug("module-decomposition"),
                ),
            }
        );
        assert_eq!(name.to_string(), filename);
        // The old spelling — no separator at all — is the one that had four
        // readings, and it now has none: there is nothing to match the middle
        // against since `open-kind-k20`, so it is refused rather than guessed at.
        assert!(
            !matches!(
                verdict(
                    "04-integrate-review-design-module-decomposition-k5.md",
                    Found::File
                ),
                Verdict::Entry(_)
            ),
            "a name with no separator must not parse"
        );
        // Moving the separator does **not** give a rival reading of this name;
        // it gives a *different name*, which parses to different parts and
        // renders back to itself. That is the requirement — *a leaf filename has
        // exactly one reading* — and not *only one placement of `--` is legal*.
        // The distinction was invisible while the kind set was closed, because
        // `integrate-review` was not a kind and the name simply failed; with an
        // open token it is one, and the property still holds because the two
        // spellings are two files.
        let moved = "04-integrate-review--design-module-decomposition-k5.md";
        let other = entry(moved, Found::File);
        assert_eq!(
            other,
            TaskName::Positioned {
                ordinal: Ordinal::new(4),
                key: Key::new(5),
                parts: Parts::leaf(
                    Outcome::Live,
                    a_kind("integrate-review"),
                    slug("design-module-decomposition"),
                ),
            }
        );
        assert_ne!(other, name, "two filenames must never name one entry");
        assert_eq!(other.to_string(), moved);
    }

````
<!-- /fragment -->

**What it establishes.** This is the specification's own round-trip scenario, and
the only test in this section whose fixtures parse — the charter test above is
the page's other one. `04-integrate-review-design--module-decomposition-k5.md`
has one reading: kind `integrate-review-design`, slug `module-decomposition`, and
it renders back to the bytes it was parsed from. The separator-less spelling of
the same words — the name that had four readings under the closed-set grammar —
now has none. And moving the separator does not produce a rival reading of one
name; it produces a different name, which parses to different parts and renders
back to itself. The assertion `assert_ne!(other, name)` states the requirement
precisely: *a leaf filename has exactly one reading*, not *only one placement of
`--` is legal*.

**What it would still pass under.** Every assertion here would pass unchanged
under a grammar that split at the **last** `--` rather than the first. Both
fixtures contain exactly one separator, so the two grammars agree on both, and
the `moved` fixture does not discriminate between them despite looking as though
it should — it is a second *placement*, not a second separator. What actually
separates first-split from last-split is a fixture with two, and the only one on
this page is `01-impl--a--b-k1.md`, two tests below: under a last-`--` split that
name yields kind `impl--a`, which `refuse_token` refuses, so the refusal arrives
as `BadKind` and that test's `BadSlug` match fails. The property this test is
named for is therefore held jointly with one it does not mention.

The negative assertion is weaker than the paragraph above it reads. It rules out
`Verdict::Entry` and nothing else, so it is satisfied by `Verdict::Foreign` — the
test would pass while the separator-less migration name was silently skipped,
taking its subtree with it if it were a directory. That is the exact failure this
chapter is named for, and no assertion in this test excludes it; what excludes it
is the preceding test, where the same shape is asserted equal to
`MissingSeparator`. Separately, `assert_ne!(other, name)` compares parsed values
and not renderings, so it would pass while both names rendered to the same
string, which is the canonicity defect this whole grammar exists to prevent; the
two `to_string()` assertions either side of it are what close that, and each pins
its own name to its own bytes.

<!-- fragment «name-tests-node-wearing-outcome» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1487-1510" parent="shape-refusal-tests" -->
````rust
    /// A directory wearing an outcome infix keeps the diagnostic it has today,
    /// wording included: it is one of the better error messages in the codebase
    /// and it names the real damage.
    #[test]
    fn a_node_wearing_an_outcome_infix_is_malformed() {
        for name in ["07-DONE-grove-flip-k28", "07-ABANDONED-grove-flip-k28"] {
            let error = malformed(name, Found::Dir);
            assert_eq!(
                error,
                TaskNameError::NodeWearsOutcome {
                    name: name.to_string()
                },
                "{name:?}"
            );
            let advice = error.to_string();
            assert!(
                advice.contains("malformed Grove node directory"),
                "{advice}"
            );
            assert!(advice.contains("hides every leaf under it"), "{advice}");
            assert!(advice.contains("Drop the infix"), "{advice}");
        }
    }

````
<!-- /fragment -->

**What it establishes.** A node directory wearing `DONE-` or `ABANDONED-` is
Malformed, and the diagnostic names the damage rather than the rule. In grove's
tree a node is done when no live leaf remains in its subtree, so an outcome infix
on a directory is not an unsupported spelling but a contradictory one — and the
error text says what it would cost, which is that the directory hides every leaf
under it.

**What it would still pass under.** The test asserts the error and three
substrings of its message for two names under `Found::Dir`, so it would pass
while the same infix on a *file* was mishandled in either direction — that case
is the leaf grammar's, and `01-DONE-requirements--plan-k1.md` in chapter 4's
conformance fixture is what exercises it. The wording assertions pin the message
and not the behaviour: the test would pass while nothing in the walk halted on
the verdict, and while the parse reached `NodeWearsOutcome` for the wrong reason,
since the fixtures differ only in which infix they carry.

<!-- fragment «name-tests-bad-slug» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1511-1526" parent="shape-refusal-tests" -->
````rust
    #[test]
    fn a_slug_the_grammar_cannot_read_back_is_malformed() {
        for (name, found, bad) in [
            ("01-impl--Domain-k1.md", Found::File, "Domain"),
            ("01-impl--a_b-k1.md", Found::File, "a_b"),
            ("01-impl--a--b-k1.md", Found::File, "a--b"),
            ("01--k1", Found::Dir, ""),
            ("01-BRIEF-k1", Found::Dir, "BRIEF"),
        ] {
            match malformed(name, found) {
                TaskNameError::BadSlug { slug, .. } => assert_eq!(slug, bad, "{name:?}"),
                other => panic!("{name:?}: {other:?}"),
            }
        }
    }

````
<!-- /fragment -->

**What it establishes.** The one rule that governs a kind governs a slug, on both
species. The five fixtures cover an uppercase letter, an underscore, an embedded
separator, an empty token and a reserved word, and the last two are directories —
where the whole middle is the slug, because a node name carries no kind and no
infix. The reported token is the offending word itself in each case, including
`a--b`, which is what remains after the middle splits at its first separator.

**What it would still pass under.** The match discards the error's own message
with `..` and checks only the `slug` field, so this test would pass while the
reason named the wrong character or named none at all — the obligation to name
the offending character is asserted for a kind in the first test of this section
and, for a slug, nowhere. What carries it is that both nouns run through the same
`refuse_token`, which is the structural claim the header's *one refusal because
there is one rule* passage makes; this test depends on that sharing rather than
re-checking it. The test would also pass while a leaf's slug and a node's slug
were validated by two different rules that happened to agree on these five
fixtures.

<!-- fragment «name-tests-species-mismatch» owner="four-verdicts" source="crates/grove-loop/src/task_name.rs" lines="1527-1550" parent="shape-refusal-tests" -->
````rust
    /// The species half of the obligation, both ways round. A directory wearing
    /// a leaf's name and a file wearing a node's are each a malformed *tree*,
    /// not a foreign entry — the library can see the contradiction and has no
    /// domain error to report it with, so the judgement lives here.
    #[test]
    fn a_species_mismatch_is_malformed_in_both_directions() {
        for (name, found, declares) in [
            ("02-impl--domain-k29.md", Found::Dir, Species::Leaf),
            ("07-grove-flip-k28", Found::File, Species::Node),
            ("BRIEF.md", Found::Dir, Species::Distinguished),
            ("02-impl--domain-k29.md", Found::Other, Species::Leaf),
        ] {
            assert_eq!(
                malformed(name, found),
                TaskNameError::SpeciesMismatch {
                    name: name.to_string(),
                    declares,
                    found,
                },
                "{name:?} under {found}"
            );
        }
    }

````
<!-- /fragment -->

**What it establishes.** The `.md` suffix is what a name declares its species to
be, and `BRIEF.md` declares the distinguished species. When the filesystem
contradicts the declaration the result is a malformed tree rather than a foreign
entry, in both directions and for all three species. The doc comment states why
the judgement is grove's: the library can see the contradiction perfectly well,
and has no domain error type to report it with, so the domain that owns the
grammar owns the report.

**What it would still pass under.** All four fixtures are otherwise well-formed
names under a contradicting `found`, so the test says nothing about precedence: a
name that is both species-mismatched and token-malformed could report either
error and this would pass. `Found::Other` appears once, on a leaf, so the test
would pass while a node name or the charter under `Found::Other` was skipped as
Foreign rather than reported. And it asserts the error value only — that the
store halts on it is the library's contract, not this test's.

Four verdicts, then, and one rule about which of them a task-shaped name may
reach. What this chapter has not done is read a name apart: `Kind`, `Slug`,
`Handle` and `Parts` have been named on every page of the header and defined
nowhere. Chapter 3 defines them, and asserts the property the header claims for
the handle — that a positioned name's rendering ends in its own handle's
rendering, so a filename and a handle saying different things is not a bug this
module can have.

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Kind, slug, handle](03-kind-slug-handle.md)
