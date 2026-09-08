# Which files take part
<!-- book-page id="which-files" slice="whose-file-and-whether" order="18" -->
[Previous: Which calls the lease admits](17-the-epoch.md) | [Contents](README.md) | [Next: The guaranteed core](19-the-core.md)

<a id="whose-file-and-whether"></a>
## The rule: everything a template *is* is the runner's; what is left is whose file, and whether the second one is admissible

Chapters 16 and 17 read a file whose whole subject was state the tree must not
hold. This one reads a much smaller root, and its subject is the opposite: not
state at all, but a **choice** — which document on this machine decides what
grove executes, and whether a second document sitting beside it in the checkout
is allowed to have a say.

> **Everything a template *is* belongs to `keyed-launch`. What is left here is
> whose file, and whether the second one is admissible.** The personal file's
> path, the two roots the configuration delta is searched at, the four slots
> grove's templates are written against, and the refusal of a **tracked** delta.
> The last of those could not move because it is a question about grove's
> worktree, answered through grove's version-control seam, and it is the
> boundary between an untrusted repository and arbitrary code execution.

That last clause is why this chapter matters more than its 358 lines suggest. A
delta names a program to execute. A repository that could ship one would choose
what grove spawns in every checkout of it, and no amount of documentation makes
that safe. So the one read-only question grove asks the version-control system
outside the finish path is asked here, and it is asked before anything reads the
file it is asked about.

This is also the chapter the book's stated outcome points at. The
what-could-not-move test asks three questions of a layer that stayed behind when
a domain-free library was extracted from underneath it, and the third is *on the
way out — the policy: what does this layer choose that nothing beneath it could
have defaulted?* Its named pin is
`the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one`, and that test
is about this root. Four slot names and their cardinalities are the whole of what
grove tells the runner about its own domain; the runner could not have guessed
them, and grove cannot delegate them without ceasing to be the thing that knows
what a session is.

The carried example reaches the moment a kind becomes a command.

```text
~/.config/grove/config.kdl          declares: requirements, design, impl, finish
<worktree>/.grove.kdl               declares: impl        (untracked, ignored)
<main repo>/.grove.kdl              not reached — the first candidate won

  loop iteration, kind `impl` selected
    -> delta search: <worktree>/.grove.kdl holds something  -> that is the delta
    -> tracked?      no                                     -> admissible
    -> require("impl")                                      -> resolves
    -> expand("impl", {prompt, session_name, worktree, repo})
                                                            -> Argv, and only then a spawn
```

Two files, searched in one order, and a single yes/no question standing between
the second one and a launch. Everything else on the page is that sentence in
Rust.

<a id="what-the-instruments-see"></a>
## What the instruments see here, for once

Four chapters running have had to warn that a clean reading was a blind spot
rather than a result. This root is the exception, and it is worth stating plainly
because the next two chapters go back to the general case.

**The block is 358 lines and holds no test at all.** There is no `#[cfg(test)]`
module and no `#[test]` function in it. So the book's *supply the claim*
obligation, which attaches to inline test blocks, has nothing here to attach to —
chapter 18 is not on the structure brief's list for it, and this chapter takes
*do not restate* across its whole block, the same instruction chapters 1, 5, 10
and 15 carry.

**156 of the 358 lines are comment prose — 43.6%**, counting lines whose first
non-space characters are `//`. The structure brief's table says 44% for this
root, so the two agree at the rounding, unlike the one-point disagreement chapter
15 had to record for `verbs.rs` and `driver.rs`.

**Every one of those 156 lines is `///` or `//!`. The root has no plain `//`
comment anywhere.** That is the exact opposite of `task_grow.rs`, whose 49-line
module header is entirely `//`, and of `tree_lifecycle.rs`, which contains no
`//!` at all — and it is why the instrument reaches everything here. With no
`#[cfg(test)]` module either, both of the blind spots chapters 16 and 17 had to
work around are simply absent from this root.

So the clean run means something. `cargo doc --no-deps --document-private-items
-p grove-loop` reports twenty-six warnings over the crate and names
`session_config.rs` in none of them — and that is a measurement rather than an
instrument looking away, because planting a broken intra-doc link in this
module's header takes the count to twenty-seven and names the file and the line.
None of the four ambiguity warnings chapter 15 counted in `verbs.rs` is here
either, and neither was any of the five unresolved intra-doc links this book
adjudicated elsewhere — `unresolved-doc-links-k151` has since repaired all five,
so that class no longer appears anywhere in the count.

**One thing `cargo doc` still cannot see, and this root carried an instance.**
It checks *intra-doc* links — the `` [`Foo`] `` form — and says nothing whatever
about a Markdown link with an explicit URL target. There was one of those in this
block and it was broken; `requirement-six-citation-k189` has since replaced it
with the backticked path the rest of the file cites by, so the two Markdown links
left in the 358 lines are both the intra-doc form — `` [configuration
delta](`DELTA_FILE_NAME`) ``, at line 9 and again at line 71 — and the instrument
does reach both. The `delta_is_tracked` section below reads the one that was
broken, and it is worth reading for the blind spot rather than for the defect:
what the clean run above means is bounded by the class of link the instrument
inspects, and this root happened to hold the other class.

**The evidence is twenty tests, and the brief pins four of them.**
`crates/grove-loop/tests/session_config.rs` is 713 lines and twenty `#[test]`
functions in four labelled sections — the slot vocabulary, the just-in-time
presence rule, the cross-crate seam, and the configuration delta. It is
`tests/`, so it is evidence and not a root: this chapter cites those tests and
reproduces none of them, and the count above is taken from the file rather than
from any prose list of it.

<a id="the-block-declared"></a>
## The block, declared

The 358 lines are one composite whose children are the file's own items in file
order, because the file is already in the order the concept wants: what the
module kept, the two names, the vocabulary, the two roots, the source, the
configuration, and then the two free functions that decide whether a second file
is allowed to speak.

<!-- fragment «whose-file» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="1-358" parent="source-session-config" -->
<!-- insert «config-header» -->
<!-- insert «config-imports» -->
<!-- insert «config-two-paths» -->
<!-- insert «config-four-slots» -->
<!-- insert «config-vocabulary» -->
<!-- insert «config-expansion-context» -->
<!-- insert «config-delta-roots» -->
<!-- insert «config-template-source» -->
<!-- insert «config-template-source-open» -->
<!-- insert «config-from-env» -->
<!-- insert «config-personal-path» -->
<!-- insert «config-template-source-load» -->
<!-- insert «config-session-config» -->
<!-- insert «config-path-and-candidates» -->
<!-- insert «config-load» -->
<!-- insert «config-read» -->
<!-- insert «config-load-for-worktree» -->
<!-- insert «config-source-and-require» -->
<!-- insert «config-expand» -->
<!-- insert «config-find-delta» -->
<!-- insert «config-refuse-tracked» -->
<!-- insert «config-delta-is-tracked» -->
<!-- /fragment -->

<a id="the-header-and-what-left"></a>
## The header, and the two things it says did not go

The module header does the spine's work for this chapter and does it in two
moves: a list of everything that *went* to the runner, and then the short list of
what stayed. The second list is the chapter's contents page.

<!-- fragment «config-header» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="1-17" parent="whose-file" -->
````rust
//! Grove's side of the template configuration: **which** files take part, and
//! whether the second one is admissible.
//!
//! Everything a template *is* — the KDL grammar, the slot rules, whole-word
//! expansion, aggregate diagnostics with source locations, and the rule that a
//! key resolves only if the primary file declares it — belongs to
//! [`keyed_launch`], which knows nothing about grove. What is left here is the
//! part that is grove's alone: the personal file's path, the two roots the
//! [configuration delta](`DELTA_FILE_NAME`) is searched at, the refusal of a
//! **tracked** delta, and the four slots grove's templates are written against.
//!
//! The trackedness refusal in particular could not move. It is a question about
//! grove's worktree, answered through grove's version control seam, and it is
//! the boundary between an untrusted repository and arbitrary code execution
//! (`docs/adr/untracked-configuration-delta.md`). A runner that took it would be
//! taking a security decision it has no standing to make.

````
<!-- /fragment -->

Note which of the two lists is longer. The KDL grammar, the slot rules,
whole-word expansion, aggregate diagnostics with source locations, and the rule
that a key resolves only if the primary file declares it are all
`keyed-launch`'s, and that crate knows nothing about grove — its own book reads
them. What is left is four things, and the header ranks them by how hard they
were to move: the personal file's path, the two roots, the tracked refusal, and
the four slots.

The paragraph on the trackedness refusal is the one to read twice. It does not
claim the refusal is *important*; it claims it **could not move**, and gives the
reason in the form the whole book is built on — a question about grove's
worktree, answered through grove's version-control seam. A runner that took it
would be taking a security decision it has no standing to make. That is the
spine's sentence, in this module's own words, and the chapter does not restate
it below.

<a id="what-it-imports"></a>
## What it imports, and the two crates named in one line

The imports are the seam, visible without reading a function.

<!-- fragment «config-imports» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="18-25" parent="whose-file" -->
````rust
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use jj_workspace::Workspace;
use keyed_launch::{Argv, Requirement, Slot, SlotRule, Templates, Vocabulary};

````
<!-- /fragment -->

`jj_workspace::Workspace` and six names from `keyed_launch` on adjacent
lines is the whole of this module's outward dependence: the version-control seam
answers one question, the runner owns everything a template is. Grove supplies
`Vocabulary` and receives `Argv`, and between those two types it never learns
what a word in a template means.

<a id="two-paths-one-public"></a>
## Two paths, and only one of them is public

Both constants are file names, and the asymmetry between them is deliberate.

<!-- fragment «config-two-paths» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="26-30" parent="whose-file" -->
````rust
const CONFIG_PATH: &str = ".config/grove/config.kdl";
/// The configuration delta's fixed name, searched at the two roots
/// [`DeltaRoots`] carries (`docs/adr/untracked-configuration-delta.md`).
pub const DELTA_FILE_NAME: &str = ".grove.kdl";

````
<!-- /fragment -->

`CONFIG_PATH` is private and used once, at `SessionConfig::path`.
`DELTA_FILE_NAME` is `pub` — and, enumerated across the workspace, it is reached
from nowhere outside this file: three uses here, plus the two doc comments that
link to it. Its publicness buys the intra-doc link the header opens with rather
than a caller.

<a id="the-four-slots"></a>
## The four slots, and the claim that there is no fifth

This is the outcome's third question in twenty-five lines: the values this
layer chooses that nothing beneath it could have defaulted.

<!-- fragment «config-four-slots» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="31-56" parent="whose-file" -->
````rust
/// The four slots grove's command templates are written against, and the whole
/// of what grove tells the runner about its own domain.
///
/// `${prompt}` is required because a launch that does not carry the prompt
/// launches a session with no mandate; the other three are conveniences a
/// template may take or leave. There is no fifth, and adding one is a change to
/// this list and to `docs/CONFIGURATION.md` together.
const SLOTS: [SlotRule<'static>; 4] = [
    SlotRule {
        name: "prompt",
        requirement: Requirement::ExactlyOnce,
    },
    SlotRule {
        name: "session_name",
        requirement: Requirement::AtMostOnce,
    },
    SlotRule {
        name: "worktree",
        requirement: Requirement::AtMostOnce,
    },
    SlotRule {
        name: "repo",
        requirement: Requirement::AtMostOnce,
    },
];

````
<!-- /fragment -->

The doc comment makes three claims and the code makes one of them
checkable. That `${prompt}` is required *because a launch that does not carry the
prompt launches a session with no mandate* is an argument, and
`Requirement::ExactlyOnce` is where it binds. That the other three are
conveniences a template may take or leave is `AtMostOnce`, three times. The
third claim — **there is no fifth** — is the array's length, and it is the one a
reader should hold loosely.

`the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one` is the test.
It builds a template per case and asserts on the refusal each produces: a missing
`${prompt}` and a doubled one both give *must contain `${prompt}` exactly once*;
each of the three optional slots doubled gives *may appear at most once*; and
`${settings}` gives *unknown substitution*. It then loads `runner ${prompt}` and
expands it, so the three optional slots are shown to be genuinely optional rather
than merely permitted.

**What it would still pass under.** The refusals it matches are produced inside
`keyed-launch`, not here, so what the test observes is grove's vocabulary *seen
through the runner's validator* — the honest form, and the same shape chapters 7
and 14 had to use for the grammar. And *there is no fifth* is pinned only against
the one name the test tries: adding a fifth `SlotRule` called `settings` turns it
red, and adding one called anything else leaves it green. The claim the array
actually holds is that these four have these cardinalities.

The comment's other coupling — *adding one is a change to this list and to
`docs/CONFIGURATION.md` together* — is true today and held by nobody. That
document's *Substitutions* table lists exactly these four names with exactly
these cardinalities, and no test in the workspace compares the two. It is a
coupling kept by discipline, and the comment is the only place it is written
down.

<a id="supplied-at-load"></a>
## Supplied at load, and public for a reason the tests show

The vocabulary reaches the runner through one function, and the reason it
is handed over at load rather than at expansion is the whole of its doc
comment.

<!-- fragment «config-vocabulary» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="57-63" parent="whose-file" -->
````rust
/// Grove's slot vocabulary, supplied at load so every template rule is checked
/// before anything is spawned (`docs/specs/module-decomposition.md`, decision 7).
#[must_use]
pub fn vocabulary() -> Vocabulary<'static> {
    Vocabulary { slots: &SLOTS }
}

````
<!-- /fragment -->

The doc comment's citation of `docs/specs/module-decomposition.md`,
decision 7, checks out exactly: that decision's sketch of `Vocabulary` carries
the same sentence — *supplied at load, because every template rule is checked
there*. It is the one bare citation in this block that names a record holding
precisely the claim on which the comment depends.

What the comment does not say is why the function is `pub`. Production reaches it
once, at `read` below. Its only other callers are the two conformance-kit tests,
which hand grove's vocabulary to `keyed_launch::conformance::check` — so the
public modifier exists so that *grove's configuration can be held to the runner's
own kit from outside the runner*, which is what
`a_grove_configuration_conforms_to_the_runners_own_kit` does.

**What that test would still pass under.** It writes one document using all four
slots and asserts the kit passes it. The kit checks a document against a
vocabulary, so it would pass just as well if `SLOTS` had the wrong cardinalities
— a document with one `${prompt}` conforms whether `prompt` is `ExactlyOnce` or
`AtMostOnce`. It pins *this document and this vocabulary are kit-clean*, and the
cardinalities are pinned by its sibling above. `the_kit_and_grove_refuse_the_same_document`
is what closes the pair, refusing one document from both sides.

<a id="the-four-values"></a>
## The four values, and the two roots that are taken rather than derived

`ExpansionContext` is the other half of the vocabulary: four names above,
four values here, checked against each other at expansion.

<!-- fragment «config-expansion-context» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="64-70" parent="whose-file" -->
````rust
pub struct ExpansionContext<'a> {
    pub prompt: &'a str,
    pub session_name: &'a str,
    pub worktree: &'a Path,
    pub repository: &'a Path,
}

````
<!-- /fragment -->

No doc comment, and it needs none — the field names are the slot names.
**Seven of the block's twenty-five items carry none**, and in a root at 44%
comment prose that set is worth naming rather than guessing at: two are `impl`
blocks, which conventionally take no comment of their own, and the other five are
the mechanical ones — the private `CONFIG_PATH`, this struct and `SessionConfig`,
whose fields are their own documentation, the one-line `SessionConfig::path`, and
the private `read` whose documented public wrapper sits directly above it. Every
item in the block that carries an argument carries a comment.

<!-- fragment «config-delta-roots» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="71-84" parent="whose-file" -->
````rust
/// The two roots the [configuration delta](`DELTA_FILE_NAME`) is searched at,
/// **in that order** — the same two `${worktree}` and `${repo}` expand to.
///
/// They are *taken*, never re-derived here. A second notion of "the repository
/// root" computed inside this module is exactly the drift that would let the
/// search order disagree with what `${repo}` expands to in the very template it
/// selected; the VCS seam's `main_repo` is the one derivation, and its result
/// arrives through this struct. Naming both fields also makes a caller-side swap
/// of two same-typed paths impossible.
pub struct DeltaRoots<'a> {
    pub worktree: &'a Path,
    pub repository: &'a Path,
}

````
<!-- /fragment -->

This doc comment is the module's strongest argument and the chapter does
not paraphrase it. The claim is that a second notion of *the repository root*,
computed inside this module, is the drift that would let the search order
disagree with what `${repo}` expands to **in the very template it selected** —
so the seam's `main_repo` is the one derivation and its result arrives through
this struct. The second claim is smaller and pays for itself: naming both fields
makes a caller-side swap of two same-typed paths impossible.

`loop_driver.rs` is where that lands. It builds one `DeltaRoots` per iteration
from the workspace the binary already resolved, and the same two paths become
`${worktree}` and `${repo}` at expansion. One resolution, three consumers.

<a id="a-source-not-a-snapshot"></a>
## A source rather than a snapshot — and the two reads its comment counts

This is the type the book carried an adjudication for from chapter 1's cast
onward, and the count in its doc comment has since been corrected at source.
Line 89 reads *twice per iteration*; it said *once* while this book was drafted.

<!-- fragment «config-template-source» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="85-101" parent="whose-file" -->
````rust
/// **Where each iteration's launch templates are read from** — the third
/// argument [`crate::run`] takes.
///
/// It is a *source* rather than a snapshot, and that is deliberate. The loop
/// reads the configuration twice per iteration: once before the tree is
/// mutated, so the just-in-time presence rule is asked against the document
/// that was live then, and once after the leaf is selected, so a session that
/// added a kind to `config.kdl` is launched from the document as it stands
/// (`docs/adr/complete-session-configuration.md`). A loaded [`SessionConfig`]
/// handed in once could express neither.
///
/// The delta roots are not held here: they come from the workspace `run` is
/// given, so there is one derivation of them and nothing to keep in step.
pub struct TemplateSource {
    home: PathBuf,
}

````
<!-- /fragment -->

**The design argument was never the defective half, and it is why the comment is
worded the way it is.** *A source rather than a snapshot* is exactly what the
type is for, and the two consequences it names are both real: the just-in-time
presence rule is asked against the document that was live before the tree was
mutated, and a session that adds a kind to `config.kdl` is launched from the
document as it stands. A loaded `SessionConfig` handed in once could express
neither.

**What the fix changed is that the sentence now counts the reads it was already
naming.** `loop_driver.rs` calls `templates.load(&delta_roots)` at line 241 and
again at line 260, both inside the loop body: the first is bound to
`pre_transition_config` and is what the finish sentinel's presence rule is asked
against, *before* `transition_to_current` mutates anything; the second is taken
after the leaf is selected, and is the document the launch expands the selected
kind's template from. Those were always the two clauses of this sentence, which
is why the defect was a count and not a design — the comment described a two-read
loop and then said *once* in its own first clause. The corrected wording keeps
both clauses and attaches each to the read it belongs to, which is why it reads
as an enumeration rather than as a single consequence with two effects.

The claim is stated on two pages of this book rather than one, and that is what
made it fixable here at all. **Chapter 20 owns those two calls and shows the
count against the bytes**; this chapter owns the sentence. While the corpus was
frozen neither page could touch the comment, so both adjudicated it instead, and
`template-source-read-count-k86` carried the fix once this book had landed — the
source change, the fragment above and both adjudicating paragraphs in a single
commit. The reader can hold this fragment and chapter 20's side by side and count
the calls, so nothing outside this book had to be taken on trust — the same
property [chapter 6](06-paths.md#canonicalise-to-compare)'s canonicalisation
clause had, where the header's claim and the second call site sat in one file —
and it took the same route, adjudicated on the page and then corrected at source
by a leaf of its own.

The last paragraph of the comment is unaffected and worth keeping in view: the
delta roots are *not* held on this type. They come from the workspace `run` is
given, which is the same one-derivation rule `DeltaRoots` argued for one item
above.

<a id="four-methods-one-unused"></a>
## Four methods, and one of them has no caller

The impl block opens on the constructor that takes a home directory
outright, which is the plainest of the four and the only one nothing uses.

<!-- fragment «config-template-source-open» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="102-108" parent="whose-file" -->
````rust
impl TemplateSource {
    /// The personal configuration under `home`.
    #[must_use]
    pub fn under(home: impl Into<PathBuf>) -> Self {
        Self { home: home.into() }
    }

````
<!-- /fragment -->

`under` takes a home directory directly, and **nothing in the workspace
calls it** — not production, not a test, not another crate. Enumerated across
every `.rs` file, the only match for the name is an unrelated function in
`ordinal-fs-tree`'s test suite. It is public surface with no consumer, and it is
the constructor a test *would* use if the tests did not build a home directory
and go through `SessionConfig::load` instead.

<!-- fragment «config-from-env» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="109-123" parent="whose-file" -->
````rust
    /// The personal configuration under `$HOME`.
    ///
    /// # Errors
    ///
    /// `$HOME` unset, which leaves nothing to locate the file from.
    pub fn from_env() -> Result<Self, crate::Error> {
        let home = std::env::var_os("HOME").map(PathBuf::from).ok_or_else(|| {
            crate::Error::msg(
                "$HOME is not set; cannot locate ~/.config/grove/config.kdl. Set it, or run \
                 `grove` from a login shell.",
            )
        })?;
        Ok(Self { home })
    }

````
<!-- /fragment -->

The refusal is the whole item. `$HOME` unset leaves nothing to locate the
file from, and rather than defaulting to a path it names the remedy — set it, or
run `grove` from a login shell. `crates/grove/src/cli.rs` line 47 is the one
production call site, so this refusal is the human binary's front door.

<!-- fragment «config-personal-path» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="124-130" parent="whose-file" -->
````rust
    /// The personal file's path — the one a diagnostic names when nothing
    /// overrode it.
    #[must_use]
    pub fn personal_path(&self) -> PathBuf {
        SessionConfig::path(&self.home)
    }

````
<!-- /fragment -->

*The one a diagnostic names when nothing overrode it* — used once, at
`loop_driver.rs` line 215, to hold the fallback path every launch diagnostic
falls back to when a kind resolved from no file at all.

<!-- fragment «config-template-source-load» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="131-136" parent="whose-file" -->
````rust
    /// Read the personal file, and at most one delta laid over it.
    pub(crate) fn load(&self, roots: &DeltaRoots<'_>) -> Result<SessionConfig> {
        SessionConfig::read(&self.home, roots)
    }
}

````
<!-- /fragment -->

`pub(crate)`, and the narrowest surface on the type. Its production call
sites are exactly three in the workspace: the two in the loop body that the
adjudication above counts, and `load_for_worktree` below. Every one of them
passes roots it did not derive.

<a id="the-configuration-itself"></a>
## The configuration itself, and the two searched paths

The loaded configuration is a newtype over the runner's `Templates`, and
everything below it is grove deciding which files that type is built from.

<!-- fragment «config-session-config» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="137-140" parent="whose-file" -->
````rust
pub struct SessionConfig {
    templates: Templates,
}

````
<!-- /fragment -->

One field, and it is the runner's type. Everything a template is lives
behind it.

<!-- fragment «config-path-and-candidates» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="141-155" parent="whose-file" -->
````rust
impl SessionConfig {
    pub fn path(home: &Path) -> PathBuf {
        home.join(CONFIG_PATH)
    }

    /// Where the delta is looked for, in search order: the worktree root, then
    /// the main repository root. The two coincide in a single-worktree
    /// repository, which is harmless — the first candidate found wins outright.
    pub fn delta_candidates(roots: &DeltaRoots<'_>) -> [PathBuf; 2] {
        [
            roots.worktree.join(DELTA_FILE_NAME),
            roots.repository.join(DELTA_FILE_NAME),
        ]
    }

````
<!-- /fragment -->

`delta_candidates` is `pub` and, like `DELTA_FILE_NAME`, reached from
nowhere outside this file — `find_delta` at line 277 is its only caller. The
order in the returned array **is** the search order, and the doc comment closes
the case a reader would otherwise wonder about: the two roots coincide in a
single-worktree repository, which is harmless because the first candidate found
wins outright and the two are never merged.

That the two paths *diverge* in the secondary-workspace family is what makes the
pair worth having at all, and it is the same fact the version-control seam's own
book states about `main_repo`.

<!-- fragment «config-load» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="156-171" parent="whose-file" -->
````rust
    /// The personal file, then at most one delta laid over it per kind.
    ///
    /// All-or-nothing in both halves: the personal file is read and fully
    /// validated whatever a delta says, and an unreadable, unparseable, invalid
    /// or **tracked** delta fails the load rather than falling back to the very
    /// policy its owner was moving work away from.
    ///
    /// # Errors
    ///
    /// A personal file that is missing, unparseable or invalid; a delta that is
    /// any of those or **tracked**; or a candidate whose state could not be
    /// established at all.
    pub fn load(home: &Path, roots: &DeltaRoots<'_>) -> Result<Self, crate::Error> {
        Ok(Self::read(home, roots)?)
    }

````
<!-- /fragment -->

*All-or-nothing in both halves* is the sentence that carries the record.
The personal file is read and fully validated whatever a delta says; an
unreadable, unparseable, invalid or **tracked** delta fails the load rather than
falling back to *the very policy its owner was moving work away from*. That last
clause is the argument: a fallback here would silently hand a session back to the
configuration the delta existed to displace, which is a worse outcome than
refusing.

The decision record `complete-session-configuration` is what the file is holding
to, and it holds: it states that the whole of the personal file and of any second
source is validated eagerly before every tree mutation and again before every
launch, and that only *presence* is asked at the moment of use.

<!-- fragment «config-read» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="172-181" parent="whose-file" -->
````rust
    fn read(home: &Path, roots: &DeltaRoots<'_>) -> Result<Self> {
        let path = Self::path(home);
        let delta = find_delta(roots)?;
        if let Some(delta) = &delta {
            refuse_a_tracked_delta(delta)?;
        }
        let templates = Templates::load(&path, delta.as_deref(), vocabulary())?;
        Ok(SessionConfig { templates })
    }

````
<!-- /fragment -->

Nine lines, and the order in them is the security property. The delta is
found, then refused if tracked, and only then does `Templates::load` open
anything. The refusal runs against the path the search already selected and
before any byte of it is parsed — which is the difference between validating a
candidate and choosing one, and the two free functions at the end of the chapter
are those two steps in that order.

<a id="the-same-order-from-a-verb"></a>
## The same order, reached from a verb

The loop is not the only thing that loads a configuration. A `grove-llm`
verb does too, and this is the entry point that keeps the two agreeing.

<!-- fragment «config-load-for-worktree» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="182-200" parent="whose-file" -->
````rust
    /// Load from the worktree a verb is running in, resolving `$HOME` and the
    /// two delta roots the same way the loop driver does.
    ///
    /// The seam's `main_repo` is the one derivation of *the repository root*, so
    /// a verb and the driver search the delta in the same order.
    ///
    /// # Errors
    ///
    /// `$HOME` unset, a working tree that is not a jj workspace, or any refusal
    /// [`Self::load`] makes.
    pub fn load_for_worktree(worktree: &Path) -> Result<Self, crate::Error> {
        let source = TemplateSource::from_env()?;
        let workspace = Workspace::resolve(worktree).map_err(anyhow::Error::from)?;
        Ok(source.load(&DeltaRoots {
            worktree,
            repository: workspace.main_repo(),
        })?)
    }

````
<!-- /fragment -->

This is the second entrance, and its doc comment states why it exists:
so that *a verb and the driver search the delta in the same order*. It resolves
`$HOME` and the two roots the same way the loop driver does, taking `main_repo`
from the seam rather than computing a second answer — the `DeltaRoots` argument
again, discharged at the one place a caller could have got it wrong.

`crates/grove-llm/src/cli.rs` line 843 is its only caller in the workspace, and
it is followed immediately by `require`. That pairing is the just-in-time
presence rule reaching the agent-side binary.

<!-- fragment «config-source-and-require» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="201-220" parent="whose-file" -->
````rust
    /// The file the resolved template for `kind` was read from — the personal
    /// file, or the delta that overrode it.
    pub fn source(&self, kind: &str) -> Option<&Path> {
        self.templates.source(kind)
    }

    /// Does this kind resolve to exactly one complete template?
    ///
    /// The just-in-time half of `docs/adr/complete-session-configuration.md`:
    /// asked before grove writes a leaf of this kind, and again — through
    /// [`Self::expand`] — before it launches one.
    ///
    /// # Errors
    ///
    /// A kind no template resolves for, or one whose template is incomplete.
    pub fn require(&self, kind: &str) -> Result<(), crate::Error> {
        self.templates.require(kind).map_err(anyhow::Error::from)?;
        Ok(())
    }

````
<!-- /fragment -->

`source` answers *which file did this kind actually come from*, and
`loop_driver.rs` uses it so that every launch diagnostic names the file that held
the failing template rather than the personal file that did not.

`require` is *the just-in-time half* of `complete-session-configuration`, and the
comment's account of it is exact in a way worth checking rather than trusting.
The record says presence is asked before grove writes a leaf of kind K and again
before it launches kind K. The first ask is this function, called directly. The
second is *through* `expand` — and that is a fact about the runner's code, not a
convention grove keeps: `keyed_launch`'s `Templates::expand` opens with
`self.require(key)?` as its first statement, so a `SessionConfig::expand` that
reached a template a kind does not resolve for is not reachable. The comment says
*and again — through `Self::expand`*, and the chain holds.

<a id="handed-on-not-flattened"></a>
## Handed on rather than flattened

The last method on `SessionConfig`, and the one whose argument is about
types rather than behaviour.

<!-- fragment «config-expand» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="221-260" parent="whose-file" -->
````rust
    /// The launch this kind names, as the runner's own `Argv`.
    ///
    /// Handed on rather than flattened to a word list: `Argv` has no
    /// constructor, so passing it through to `keyed_launch::run` is what makes
    /// *nothing reaches a spawn that a template did not author* a fact about
    /// the types rather than a convention grove keeps.
    ///
    /// # Errors
    ///
    /// As [`Self::require`], plus a slot the template spells that grove's
    /// vocabulary does not supply.
    pub fn expand(&self, kind: &str, context: &ExpansionContext<'_>) -> Result<Argv, crate::Error> {
        let argv = self
            .templates
            .expand(
                kind,
                &[
                    Slot {
                        name: "prompt",
                        value: context.prompt.as_ref(),
                    },
                    Slot {
                        name: "session_name",
                        value: context.session_name.as_ref(),
                    },
                    Slot {
                        name: "worktree",
                        value: context.worktree.as_os_str(),
                    },
                    Slot {
                        name: "repo",
                        value: context.repository.as_os_str(),
                    },
                ],
            )
            .map_err(anyhow::Error::from)?;
        Ok(argv)
    }
}

````
<!-- /fragment -->

The doc comment's claim is that handing `Argv` on rather than flattening
it to a word list is what makes *nothing reaches a spawn that a template did not
author* a fact about the types: `Argv` has no constructor, so the only way to
obtain one is expansion, and the only thing that expands is a template. Grove
cannot assemble a command even by mistake, and the compiler is what says so.

The four `Slot` values are the vocabulary's four names again, matched by name
rather than by position — and `worktree` and `repo` go in as `OsStr` rather than
`str`, so a path that is not valid UTF-8 survives the round trip into the spawn.

<a id="only-notfound-is-absence"></a>
## Only `NotFound` is absence

The first of the two free functions: selection. It decides *which* file,
and refuses to guess.

<!-- fragment «config-find-delta» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="261-293" parent="whose-file" -->
````rust
/// The first of the two searched paths that **holds anything at all**; the other
/// is not read, and the two are never merged with each other.
///
/// `symlink_metadata` rather than `is_file`, so a broken symlink or a directory
/// at the searched path is a candidate that then fails closed on read, not an
/// absence that silently resolves to the personal file.
///
/// **Only `NotFound` is absence** (`docs/adr/untracked-configuration-delta.md`).
/// Any other error means this candidate's state could not be established, and
/// the two things a caller would otherwise do with it are both wrong: at the
/// worktree root it would move on and read the repository root, inverting the
/// search precedence that record fixes, and at the repository root it would fall
/// through to the very personal file the delta exists to move work away from. An
/// unresolvable candidate is therefore the same refusal an unreadable delta
/// already is, reported against the path whose state is unknown.
fn find_delta(roots: &DeltaRoots<'_>) -> Result<Option<PathBuf>> {
    for candidate in SessionConfig::delta_candidates(roots) {
        match fs::symlink_metadata(&candidate) {
            Ok(_) => return Ok(Some(candidate)),
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "failed to determine whether a Grove configuration delta is present at {}",
                        candidate.display()
                    )
                })
            }
        }
    }
    Ok(None)
}

````
<!-- /fragment -->

Two choices here that a shorter implementation would have got wrong, and
the comment argues both. `symlink_metadata` rather than `is_file` means a broken
symlink or a directory at the searched path is a *candidate* that then fails
closed on read, rather than an absence that silently resolves to the personal
file. And **only `NotFound` is absence**: any other error means this candidate's
state could not be established, and the comment enumerates the two wrong things a
caller could do with it — at the worktree root, move on and read the repository
root; at the repository root, fall through to the personal file.

`a_candidate_grove_cannot_stat_fails_closed_instead_of_reading_the_next_one` is
the test, and it is built to make the first of those two visible: it writes a
delta at the *repository* root, then chmods the worktree root to `0o000` so the
first candidate cannot be stat'd, and asserts the refusal names the unreadable
candidate. **What it would still pass under**: it asserts only that the error
names the worktree candidate's path, so an implementation that named that path
and then went on to read the repository delta anyway would fail the load for a
different reason and still satisfy it — what makes the test conclusive is the
delta deliberately placed at the second root, which a passing load would have
had to return.

**One adjudication was owed here, and it was a citation rather than a
behaviour.** While this book was drafted the comment said the wrong move at the
worktree root would invert *the search precedence requirement 6 fixes*. **There
was no requirement 6.** The string was unique in the whole repository, and the
document the phrase pointed at, `docs/specs/module-decomposition.md`, numbers its
**decisions** 1 to 11 and gives its four requirements names rather than numbers.
Read as *decision* 6 it did not fit either: that decision moves the completeness
quantifier and states that an overlay overrides and never supplies, and it says
nothing about which of two roots is searched first — at the revision the comment
was written as much as today.

**The rule the sentence described was right; only its address was wrong**, which
is the same shape as the stale module path chapter 6 adjudicated. What does fix
the search precedence, and states this very absence rule in almost these words —
*only a candidate Grove positively establishes to be absent lets the search move
on* — is `docs/adr/untracked-configuration-delta.md`, which this same file cites
correctly three times elsewhere. `requirement-six-citation-k189` landed the fix
the fragment above now shows, and the shape of the repair is worth reading off
the bytes: the record's path did not go where `requirement 6` had been. It went
onto the paragraph's own topic sentence, as a parenthesised backticked path in
the file's own convention, which leaves *that record* as the antecedent the later
clause now has. The paragraph had to be reflowed rather than patched, because it
came out forty-four characters longer than it was and still had to land on
exactly the eight lines it already occupied — a ninth would have moved every
fragment range below it in this chapter.

<a id="the-refusal-and-its-remedy"></a>
## The refusal, and a remedy nothing holds

The second free function: validation. It runs on the file the search
already chose, and never to choose it.

<!-- fragment «config-refuse-tracked» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="294-326" parent="whose-file" -->
````rust
/// Refuse the selected delta if it is **tracked**, before anything reads it.
///
/// Trackedness is validated on the delta the search already selected, never used
/// to select it: a tracked file at the worktree root is a refusal, not a reason
/// to read the repository root. The probe runs only because a candidate file
/// exists, so a checkout with no delta pays nothing for it, and a probe that
/// cannot be completed fails closed like any other unresolved validation.
///
/// The remedy names the ignore line *first* because jj enforces that order:
/// `jj file untrack` refuses a path that is not already ignored, so the
/// otherwise natural untrack-then-ignore sequence fails on its first step
/// (`jj file untrack --help`, jj 0.44.0 — "Paths to untrack. They must already
/// be ignored.").
fn refuse_a_tracked_delta(path: &Path) -> Result<()> {
    let tracked = delta_is_tracked(path).with_context(|| {
        format!(
            "checking whether the Grove configuration delta at {} is tracked",
            path.display()
        )
    })?;
    if tracked {
        bail!(
            "refusing the Grove configuration delta at {path}: it is tracked in version control, \
             and a tracked delta lets a repository choose what Grove executes in every checkout \
             of it.\n  Untrack it (`jj file untrack {name}`, after adding `/{name}` to \
             `.gitignore` — jj refuses to untrack a file it would immediately re-add).",
            path = path.display(),
            name = DELTA_FILE_NAME
        );
    }
    Ok(())
}

````
<!-- /fragment -->

*Trackedness is validated on the delta the search already selected, never
used to select it* is the sentence the whole ordering rests on: a tracked file at
the worktree root is a refusal, not a reason to read the repository root. And
because the probe runs only when a candidate file exists, a checkout with no
delta pays nothing for it.

`a_snapshotted_jj_delta_is_refused_in_both_jj_shapes` is the test, and its
fixture is the interesting part. It builds a real jj repository in each of the
two jj shapes — `git.colocate=false` and `true` — writes an unignored delta, and
then runs `jj status`, because **jj snapshots the working copy on any ordinary
command, which is exactly the moment an unignored delta becomes tracked**. The
refusal is then asserted in both shapes. Its sibling
`an_ignored_jj_delta_is_read_in_both_jj_shapes` writes the ignore line first and
expands the delta's template, so the pair holds both directions.

**What that test does not pin, and it looks as though it does.** It makes three
assertions: the error contains *tracked*, it contains the candidate's full path,
and it contains `/.grove.kdl`. The third cannot fail while the second passes — the
full path ends in exactly those bytes — so it adds nothing. Confirmed by
mutation: deleting the entire `Untrack it (…)` sentence from the `bail!` leaves
all twenty tests in the evidence file green. **The remedy is held by nothing.**
The command, the ordering, and the four lines of doc comment above explaining
*why* the ignore line is named first are argument the suite does not reach;
`jj file untrack` occurs nowhere in the workspace but this file's own comment and
its own message.

**The argument itself is sound, and its cited evidence still holds.** The comment
quotes `jj file untrack --help` at jj 0.44.0 — *"Paths to untrack. They must
already be ignored."* Run against the jj on this machine, 0.45.1, the help emits
that sentence verbatim, so the quotation survives the version bump even though
the version named is older than the one to hand. The otherwise natural
untrack-then-ignore sequence really does fail on its first step, which is why the
remedy names the ignore line first.

<a id="one-question-asked-of-the-vcs"></a>
## The one question grove asks the version-control system

The last item, and the boundary the module header opened on.

<!-- fragment «config-delta-is-tracked» owner="whose-file-and-whether" source="crates/grove-loop/src/session_config.rs" lines="327-358" parent="whose-file" -->
````rust
/// Is the delta at `path` **tracked** by the workspace it sits in?
///
/// The one read-only question grove asks the version control system outside the
/// finish path, and the enforcement behind the untracked configuration delta
/// (`docs/adr/untracked-configuration-delta.md`): a delta names a program to
/// execute, so a repository that could ship one would choose what Grove spawns
/// in any checkout of it. Documentation cannot establish that boundary and
/// neither can an ignore rule — a file already committed stays tracked when an
/// ignore line is added.
///
/// Anchored to the candidate's **own** directory rather than to the leased
/// worktree, because the two searched roots may live in different workspaces (a
/// secondary jj workspace) and the one that owns the file is the one whose
/// working-copy commit can hold it.
///
/// **No workspace at all answers `false` rather than refusing.** This is the one
/// place absence is an answer rather than a precondition failure: nothing owns
/// the file, so nothing tracks it, and the hostile repository this guards
/// against has a marker by definition. That is why the refusal is discarded
/// instead of propagated — resolution declines for exactly one reason a caller
/// can act on, *this is not a Jujutsu working tree*, and here that reason is the
/// answer. A probe that cannot be *completed* — the binary missing, the command
/// failing — is still an error, and its caller fails closed.
fn delta_is_tracked(path: &Path) -> Result<bool> {
    let directory = path
        .parent()
        .with_context(|| format!("candidate path has no parent directory: {}", path.display()))?;
    let Ok(workspace) = Workspace::resolve(directory) else {
        return Ok(false);
    };
    Ok(workspace.is_tracked(path)?)
}
````
<!-- /fragment -->

Three claims, and each is load-bearing.

**Why it is asked at all**: a delta names a program to execute, so a repository
that could ship one would choose what grove spawns in any checkout of it.
Documentation cannot establish that boundary and neither can an ignore rule — a
file already committed stays tracked when an ignore line is added.

**Where it is anchored**: the candidate's *own* directory rather than the leased
worktree, because the two searched roots may live in different workspaces and the
one that owns the file is the one whose working-copy commit can hold it. That is
the secondary-workspace family again, and it is why `Workspace::resolve` is
called here on `path.parent()` rather than taken from the caller.

**Why absence answers `false` rather than refusing**: nothing owns the file, so
nothing tracks it, and the hostile repository this guards against has a marker by
definition. The comment is careful to distinguish that from a probe that cannot be
*completed* — the binary missing, the command failing — which is still an error,
and whose caller fails closed.

`a_trackedness_probe_that_cannot_be_completed_fails_closed` is the test for that
last distinction, and it is the one place in this chapter where the assertion is
weaker than the name. Its fixture creates a `.jj` **directory with no repository
inside it**, so `Workspace::resolve` succeeds on the marker walk and `is_tracked`
then fails, and the test asserts the error contains *is tracked*.

**What it would still pass under.** That substring is produced at two different
places: the `with_context` wrapper in `refuse_a_tracked_delta`, which is what
fires here, and the `bail!` immediately below it, which reads *it is tracked in
version control*. So a genuinely tracked delta satisfies the same assertion. The
mutation settles which one is carrying it — replacing the context message with
one not containing the phrase turns exactly this test red, and the error it then
reveals is jj's own *Failed to read commit backend type … No such file or
directory*, with no occurrence of *tracked* anywhere in the chain. What the test
establishes is therefore *the load failed while asking about trackedness rather
than resolving to the personal file*, which is the property that matters; what it
does not establish is *the probe was unanswerable*, which is what its name says.

**And one more citation was repaired here, of the same class as
`requirement 6`.** While this book was drafted this doc comment cited the
untracked-configuration-delta record as a Markdown link to
`../docs/adr/untracked-configuration-delta.md`. Every other document citation in
the file — six of them then, and seven now that `requirement 6`'s replacement has
joined them — is a backticked path in prose, which is the file's own convention
and is stable. That one was a link, and it resolved from nowhere: rustdoc emits
the target verbatim, so from the rendered page at
`grove_loop/session_config/fn.delta_is_tracked.html` it pointed inside the
generated documentation tree, where no `docs/adr/` exists; and read as a path
relative to the source file it would be `crates/grove-loop/docs/adr/`, which does
not exist either. **`cargo doc` reported nothing**, because it checks intra-doc
links and never an explicit URL target — which is why this was a different blind
spot from the five unresolved intra-doc links the crate warned about while this
book was drafted, and why the clean run this chapter opened with was not evidence
about this line. `unresolved-doc-links-k151` repaired those five and left this
address untouched, because no `cargo doc` run had ever seen it.

The record it names is real and says what the comment says it says, so
`requirement-six-citation-k189` changed nothing but the form: the link became the
parenthesised backticked path the fragment above now shows, and the paragraph
reflowed onto the seven lines it already occupied. Nothing found this one either
— no instrument in this repository reads an explicit URL in a doc comment, and it
surfaced only because writing this section meant enumerating every citation in
the file by hand.

<a id="what-could-not-move-here"></a>
## What could not move

The book's question, asked of a root whose whole subject is a choice rather
than state.

**On the way in — the names.** Four of them, and they are the clearest instance
of an owned vocabulary in the crate. `prompt`, `session_name`, `worktree` and
`repo` are not Unix facts and not runner facts; they are the whole of what grove
tells `keyed-launch` about its own domain, and the runner is built to receive
them without understanding any of them. The cost is the one the outcome names: a
chosen value has to be stated where a reader can find it, and this layer pays it
twice — once in `SLOTS`, once in `docs/CONFIGURATION.md` — with nothing
mechanical keeping the two in step.

**On the way through — the preconditions.** The order inside `read` is the whole
of it: find, refuse, and only then parse. The trackedness question is asked
against the candidate the search already selected, and the search itself refuses
to treat *state unknown* as absence. Both are the outcome's second cost — *the
check must run against the same snapshot the operation then plans from* — in a
form where the snapshot is a file on disk rather than a tree.

**On the way out — the policy.** This is the chapter the outcome's third question
was written for, and the honest account of its evidence is thinner than the
argument above it. Twenty tests cover the vocabulary, the presence rule, the
seam and the delta. Against that: the refusal's entire remedy is held by nothing
and survives deletion; *there is no fifth slot* is pinned only against the single
name a test happens to try; the coupling between `SLOTS` and the operator
document is written down in a comment and checked by no one; and three public
items — `DELTA_FILE_NAME`, `delta_candidates` and `TemplateSource::under` — have
no consumer outside this file, `under` having none anywhere. None of that makes
the code wrong. It is the shape of the evidence under a module whose arguments,
at 44% comment prose, are unusually good.

**And the thing this chapter is really for.** Every other chapter in Part V is
about coordination — who holds the lease, which calls are admitted, what a
runner may not choose. This one is about *trust*, and it is the only place in the
crate where grove asks the version-control system a question for a reason that
has nothing to do with committing anything. The answer decides whether a file
that arrived with the repository is allowed to name a program. That question
could not move to `keyed-launch`, which knows nothing about worktrees, and it
could not move to `jj-workspace`, which knows nothing about what a delta is for.
It is the boundary itself, and it stayed here because there was nowhere else for
it to be.

[Previous: Which calls the lease admits](17-the-epoch.md) | [Contents](README.md) | [Next: The guaranteed core](19-the-core.md)
