# The twelve verbs, and the two that are not
<!-- book-page id="the-verbs" slice="twelve-not-fourteen" order="15" -->
[Previous: Finishing](14-finishing.md) | [Contents](README.md) | [Next: One live driver per working tree](16-the-lease.md)

<a id="twelve-not-fourteen"></a>
## The rule: twelve verbs, and everything else says why it is not one

Chapter 14 deleted the grove. The fourteen chapters before this one reached that
ending through a surface they never stopped to read, and this chapter is that
surface: three small files, 516 lines, holding every function a session is
allowed to call and the reason each of them is allowed to be called.

> **The surface is twelve verbs, and everything else that touches the tree has to
> say why it is not one.** A count is only kept if the things standing beside it
> say why they are out of it. `verbs.rs` declares fourteen public functions and
> twelve verbs. `driver.rs` holds two more operations over the same tree and
> refuses to sit beside them. Each of the four exceptions states its own case, in
> its own doc comment, at the point of declaration — which is the only place a
> reader counting the surface will be standing.

This is the crate's most argued-in-situ material: 202 of `verbs.rs`'s 363 lines
begin with a comment marker, 42 of `driver.rs`'s 57, and 62 of `complete.rs`'s
96. The comments make the argument, and the fragments below quote them in full. What the prose adds is the connection
between them, the count checked against the declarations, and the measurements
no comment can state about itself.

**Say which tree.** The store's vocabulary reaches this chapter in exactly one
word: `Sought`, which `ordinal-fs-tree` uses for *a search that matched, or
matched nothing*. Everything else on this surface — a brief chain, a kind, an
outcome, a handle, finishing — is grove's, and that asymmetry is the chapter's
whole subject. Where a sentence below could belong to either vocabulary, it says
which.

The carried example reaches the step that is not a step: a session mid-task, with
the whole surface available to it, and every verb answering with the paths it
wrote.

```text
a session mid-task, holding one opening

  root_init(vacancy, plan, requirements)  -> Initialized { brief, first_leaf }
  pick(&tree)                             -> Sought<Selection>
  leaf_add(&tree, root, later, [impl])    -> Vec<PathBuf>
  leaf_retire(&tree, leaf)                -> PathBuf
  finish_commit(&workspace, finish-k5)    -> Commit
  complete(signal_file, done)             -> Signalled

                                             every one of them a path,
                                             a report of paths, or the
                                             fact that there was nothing
```

<a id="fourteen-declarations-twelve-verbs"></a>
## Fourteen declarations, twelve verbs

The count is the chapter, so it is worth taking from the declarations rather
than from any sentence about them. `verbs.rs` declares fourteen `pub fn` — at
lines 37, 70, 83, 95, 108, 122, 141, 196, 222, 247, 263, 300, 314 and 335 — and
one private one, `sought`, at line 358. Two of the fourteen are not verbs, and
neither leaves that to be inferred: `stale_cross_refs` opens its doc comment with
**Not a thirteenth verb**, and `signal_channel` with **Public because the order
matters**. Twelve remain.

Those twelve fall into five groups, and the groups are what the sections below
are:

| | Group | Verbs | |
|---:|---|---:|---|
| 1 | consumes a vacancy | 1 | `root_init` |
| 2 | reads under a shared lock | 4 | `pick`, `kind`, `brief_chain`, `resolve` |
| 3 | grows the tree | 2 | `leaf_add`, `leaf_insert` |
| 4 | changes a leaf's standing | 3 | `leaf_decompose`, `leaf_retire`, `leaf_prune` |
| 5 | reaches outward | 2 | `finish_commit`, `complete` |

**The header's own split is by signature, and it is exact.** Ten of the twelve
are *handed* their tree: `root_init` takes a `Vacancy`, the four readers take a
`&Tree`, and the five that mutate take a `&TreeWrite`. The remaining two take
neither — `finish_commit` takes a `&Workspace` and opens the tree itself, and
`complete` takes no tree at all. That is the ten the header counts. It is worth
saying plainly, because `finish_commit` unmistakably *touches* the tree — it
deletes it — and is counted among the two for where else it reaches rather than
for leaving the tree alone. Its own doc comment fixes the distinction at line
289: **It opens the tree itself**, unlike every other verb here.

<a id="what-the-surface-says-about-itself"></a>
## What the surface says about itself

The three roots this chapter owns are declared whole here, each as one
composite whose children are the items below in file order.

<!-- fragment «the-twelve-verbs» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="1-363" parent="source-verbs" -->
<!-- insert «verbs-surface-header» -->
<!-- insert «verbs-imports» -->
<!-- insert «verbs-root-init» -->
<!-- insert «verbs-initialized» -->
<!-- insert «verbs-pick» -->
<!-- insert «verbs-kind» -->
<!-- insert «verbs-brief-chain» -->
<!-- insert «verbs-resolve» -->
<!-- insert «verbs-leaf-add» -->
<!-- insert «verbs-leaf-insert» -->
<!-- insert «verbs-not-a-thirteenth-verb» -->
<!-- insert «verbs-leaf-decompose» -->
<!-- insert «verbs-decomposed» -->
<!-- insert «verbs-leaf-retire» -->
<!-- insert «verbs-leaf-prune» -->
<!-- insert «verbs-pruned» -->
<!-- insert «verbs-finish-commit» -->
<!-- insert «verbs-complete» -->
<!-- insert «verbs-signal-channel» -->
<!-- insert «verbs-signalled» -->
<!-- insert «verbs-sought» -->
<!-- /fragment -->
<!-- fragment «driver-operations» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="1-57" parent="source-driver" -->
<!-- insert «driver-not-fourteen-header» -->
<!-- insert «driver-imports» -->
<!-- insert «driver-transition-to-current» -->
<!-- insert «driver-materialize-finish» -->
<!-- /fragment -->
<!-- fragment «complete-verb» owner="twelve-not-fourteen" source="crates/grove-loop/src/complete.rs" lines="1-96" parent="source-complete" -->
<!-- insert «complete-header-in-plain-comments» -->
<!-- insert «complete-imports» -->
<!-- insert «complete-disposition» -->
<!-- insert «complete-disposition-token» -->
<!-- insert «complete-interpret» -->
<!-- insert «complete-signal» -->
<!-- /fragment -->

The file opens by saying what it is for, which is unusual only because the
answer is not a behaviour. Twelve of its functions forward almost immediately
to `task_tree`, `task_grow` or `tree_lifecycle`; what the module contributes is
that they are gathered, named in one vocabulary, and counted.

<!-- fragment «verbs-surface-header» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="1-12" parent="the-twelve-verbs" -->
````rust
//! **The twelve verbs a session invokes over its grove.**
//!
//! They live here rather than with the store because ten of the twelve touch the
//! tree and every one is stated in grove's vocabulary — brief chains, kinds,
//! outcomes, handles, finishing — none of which the store has a word for.
//! Co-locating them gives the handle grammar one owner and puts the driver and
//! the verbs on one definition of a kind. The two that reach outward reach the
//! VCS seam ([`finish_commit`]) and the runner ([`complete`]).
//!
//! The three shapes that recur across the surface — the lock in the signature,
//! [`Sought`] instead of an option, and the paths every verb returns — are the
//! crate root's, and stated there.
````
<!-- /fragment -->

The header states the seam in one clause — *none of which the store has a word
for* — and then does something the rest of the crate rarely does: it declines to
explain itself. The three shapes it names are the crate root's, and chapter 1
read them there. The lock in the signature, `Sought` instead of an option, and
the paths every verb returns are not restated here, and this chapter does not
restate them either.

<!-- fragment «verbs-imports» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="13-23" parent="the-twelve-verbs" -->
````rust

use std::path::{Path, PathBuf};

use ordinal_fs_tree::Sought;

use crate::{
    complete, task_grow, task_tree, tree_lifecycle, Commit, Error, Handle, Kind, Reference,
    Selection, Slug, Tree, TreeWrite, Vacancy, Workspace,
};

pub use task_tree::{Located, Resolution};
````
<!-- /fragment -->

Fifteen names come in from the crate root and two go back out. The re-export at
line 23 is the first sign of what this module actually is: `Located` and
`Resolution` are `task_tree`'s types, and they are published *here* because this
is where a caller meets them. The module owns almost no code and almost all of
the vocabulary.

<a id="the-one-that-consumes-a-vacancy"></a>
## The one that consumes a vacancy

The first verb, and the only one that creates the thing every other verb needs.
It takes the proof that no grove was there, hands the store a slug and a kind,
and turns the store's positional report into two named paths.

<!-- fragment «verbs-root-init» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="24-49" parent="the-twelve-verbs" -->
````rust

/// Scaffold a fresh grove: the charter brief and the first leaf, as **one**
/// store operation under one lock.
///
/// It takes the [`Vacancy`] rather than a path, so it cannot run over a live
/// grove — the refusal to clobber is the shape and not a check. `kind` defaults
/// to `requirements` at the CLI: one of the two leaves grove itself authors, and
/// the only kind default that survives anywhere.
///
/// # Errors
///
/// A `finish` kind, which is driver-reserved; or a store that could not create
/// the tree.
pub fn root_init(vacancy: Vacancy, slug: &Slug, kind: &Kind) -> Result<Initialized, Error> {
    let mut paths = tree_lifecycle::root_init(vacancy, slug, kind)?;
    // `root_init` reports the charter first and the leaf second, which is the
    // store's own distinguished-child-first ordering; it has already refused a
    // report of any other shape.
    let first_leaf = paths
        .pop()
        .ok_or_else(|| Error::msg("the store initialized a grove and reported no first leaf"))?;
    let brief = paths
        .pop()
        .ok_or_else(|| Error::msg("the store initialized a grove and reported no charter"))?;
    Ok(Initialized { brief, first_leaf })
}
````
<!-- /fragment -->

The refusal to clobber a live grove is not a check anywhere in this function; it
is the argument type. A caller that has a `Vacancy` has already proved, under the
lock that produced it, that there was no tree — which is chapter 5's opening
discipline arriving here as a signature. What the body does is the other half of
this module's job: the store reports a shape, and grove names its parts. The two
`ok_or_else` arms are not defensive noise but the crate's *principle 2* stance
applied to its own dependency — a store that reported something else is named and
refused, not repaired.

<!-- fragment «verbs-initialized» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="50-58" parent="the-twelve-verbs" -->
````rust

/// What [`root_init`] wrote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Initialized {
    /// The grove's charter, `.grove/BRIEF.md`.
    pub brief: PathBuf,
    /// The first leaf, which is what `pick` will answer next.
    pub first_leaf: PathBuf,
}
````
<!-- /fragment -->

`Initialized` is the first of four report types declared next to the verb that
returns them, and the pattern is uniform: a struct whose fields are paths, whose
doc comments say what each path *is* rather than what type it has. The comment on
`first_leaf` — *which is what `pick` will answer next* — is the one place the
scaffold and the walk are tied together in a type.

<a id="the-four-that-read"></a>
## The four that read

Four verbs read, and all four take a `&Tree` — the shared-lock opening chapter 5
read. The first of them answers the only question the loop asks between tasks.

<!-- fragment «verbs-pick» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="59-72" parent="the-twelve-verbs" -->
````rust

/// The next leaf to work, or the fact that there is none.
///
/// A recursive depth-first **pre-order** walk: the first live leaf, skipping
/// briefs and terminal leaves — retired (`DONE`) and abandoned (`ABANDONED`)
/// alike. [`Sought::Nothing`] is the **finish trigger**, and it is the store's
/// word rather than an option of the loop's own invention.
///
/// # Errors
///
/// A tree carrying a name grove refuses.
pub fn pick(tree: &Tree) -> Result<Sought<Selection>, Error> {
    Ok(sought(task_tree::select_in(tree)?))
}
````
<!-- /fragment -->

One line of body under twelve of comment, and the comment is doing the work the
signature cannot: `Sought::Nothing` is the **finish trigger**, and the sentence
that follows says why the type is the store's rather than the loop's. Chapter 14
consumed that trigger; chapter 7 read the walk that produces it. What this
chapter owns is the decision to spell *there is no more work* in a word the layer
below already had.

<!-- fragment «verbs-kind» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="73-85" parent="the-twelve-verbs" -->
````rust

/// The kind of a named leaf, or of the picked one when none is named.
///
/// [`Sought::Nothing`] only for the unnamed form over a grove with no live
/// leaves; a named path that is not a leaf is an error, because the caller
/// asserted it was one.
///
/// # Errors
///
/// A path that is not a current-format leaf of this tree.
pub fn kind(tree: &Tree, leaf: Option<&Path>) -> Result<Sought<Kind>, Error> {
    Ok(sought(task_tree::kind_in(tree, leaf)?))
}
````
<!-- /fragment -->

The second reads a leaf's session kind; the third reads its ancestors' charters,
root to leaf. Both take the leaf a caller names and both hand the work straight
to `task_tree` under the opening they were given.

<!-- fragment «verbs-brief-chain» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="86-97" parent="the-twelve-verbs" -->
````rust

/// Every `BRIEF.md` from the grove root down to the leaf, in that order.
///
/// A directory level with no `BRIEF.md` is skipped silently: a node is not
/// obliged to carry a charter.
///
/// # Errors
///
/// A path that is not a leaf of this tree.
pub fn brief_chain(tree: &Tree, leaf: &Path) -> Result<Vec<PathBuf>, Error> {
    Ok(task_tree::brief_chain(tree, leaf)?)
}
````
<!-- /fragment -->

Two verbs chapter 8 read from the other side. Each is the tree-opening half of a
`task_tree` function, and each carries the contract that function's comment
appeals to but does not own: that a named path which is not a leaf is an *error*
rather than a `Nothing`, because the caller asserted it was one; and that a
directory level with no `BRIEF.md` is skipped silently. The
[guide's account of the tree verbs](../../USAGE.md#usage-tree-verbs) states the
same two contracts for the operator, which is where they are owed, since a
session reads them from `--help` and never from this file.

<!-- fragment «verbs-resolve» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="98-110" parent="the-twelve-verbs" -->
````rust

/// What a session's reference names.
///
/// **Ambiguity is an answer, not an error**: the caller is a session that can
/// re-ask with a narrower reference, and [`Resolution::Ambiguous`] carries each
/// match's handle so it can.
///
/// # Errors
///
/// A malformed key reference, or a tree carrying a name grove refuses.
pub fn resolve(tree: &Tree, reference: &Reference) -> Result<Sought<Resolution>, Error> {
    Ok(task_tree::resolve_in(tree, reference)?)
}
````
<!-- /fragment -->

**Ambiguity is an answer, not an error** is this surface's sharpest single
decision, and the reason is in the next clause: the caller is a session that can
re-ask. Chapter 9 read the four spellings and the `Ambiguous` case that lists
every match's handle. The verb adds only the judgement that a list of candidates
belongs in the success channel.

<a id="the-two-that-grow"></a>
## The two that grow

Two verbs grow the tree, and both take a `&TreeWrite` — the write opening whose
guard a mutation consumes. The first appends; the second makes room.

<!-- fragment «verbs-leaf-add» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="111-134" parent="the-twelve-verbs" -->
````rust

/// Append one or more leaves under `parent`, all carrying `slug`, as **one**
/// unit: consecutive ordinals, consecutive keys, all of it or none of it.
///
/// A one-kind list is the ordinary add; the research pair is a three-kind one,
/// and the three tokens are the methodology's, not grove's.
///
/// # Errors
///
/// An empty kind list, a `finish` kind, a parent that is not a node, or a store
/// refusal.
pub fn leaf_add(
    tree: &TreeWrite,
    parent: &Reference,
    slug: &Slug,
    kinds: &[Kind],
) -> Result<Vec<PathBuf>, Error> {
    Ok(task_grow::leaf_add(
        tree.guard()?,
        parent.as_str(),
        slug,
        kinds,
    )?)
}
````
<!-- /fragment -->

`tree.guard()?` is where a `TreeWrite` becomes a store guard, and it appears in
all five mutating verbs and nowhere else in this module. The comment's *the three
tokens are the methodology's, not grove's* is the crate declining a decision it
could easily have made: a research pair is a three-kind list because a skill said
so, and `leaf_add` neither knows nor validates that.

<!-- fragment «verbs-leaf-insert» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="135-155" parent="the-twelve-verbs" -->
````rust

/// Take `target`'s slot, shifting it and every later sibling up by one.
///
/// # Errors
///
/// A `finish` kind, a target that is the root or a brief, or a store refusal.
pub fn leaf_insert(
    tree: &TreeWrite,
    target: &Reference,
    slug: &Slug,
    kind: &Kind,
) -> Result<Inserted, Error> {
    Ok(task_grow::leaf_insert(
        tree.guard()?,
        target.as_str(),
        slug,
        kind,
    )?)
}

pub use task_grow::{Inserted, Renumber};
````
<!-- /fragment -->

The re-export at line 155 sits deliberately after `leaf_insert` rather than with
the imports, because `Renumber` is the first argument of the thing that comes
next — and the thing that comes next is not a verb.

<a id="the-first-that-is-not-a-verb"></a>
## The first that is not a verb

What follows `leaf_insert` in the file is not the next verb. It is the lint that
finishes `leaf-insert`'s job, and it opens by disqualifying itself from the count
before it says anything about what it does.

<!-- fragment «verbs-not-a-thirteenth-verb» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="156-210" parent="the-twelve-verbs" -->
````rust

/// The stale position-prefixed references a [`leaf_insert`] left behind, one
/// `path:line: <old-name> (context)` line per hit, in path order.
///
/// **Not a thirteenth verb**: it is the second half of `leaf-insert`'s contract,
/// which the store cannot supply because nothing in it knows what a reference
/// is. It takes a tree of its own, because the tree it scans is the one the
/// shift *left* — a mutation consumes its guard, and a shifted node took its
/// whole subtree's paths with it.
///
/// # The lock is **shared**, and it is gone before the caller prints
///
/// The lint reads and never writes, so it takes the reading lock: it needs
/// writers held off while it walks, and holding *readers* off as well only
/// blocked `pick`, `kind` and `brief-chain` for the length of a whole-tree
/// content scan. And it hands back the hits rather than writing them, so the
/// caller's sink is written to with no tree lock held at all — a stalled sink
/// blocks the printing process alone, where under the previous shape it wedged
/// every grove process on the worktree.
///
/// Because the opening is its own, an unspent write guard on `tree` is given up
/// before it is taken: a second file description on one directory does not share
/// an `flock`, and holding both is the self-deadlock `TreeWrite`'s header warns
/// about. See
/// `task_grow::stale_cross_refs` for the argument, including what the weaker
/// claim gives up.
///
/// # Errors
///
/// A tree that could not be read — and **only** that. It is the same refusal a
/// reading verb states for a root that is not there, reached here after an
/// insert has already landed, so a caller that wants the mutation's report
/// regardless should say so rather than propagate.
///
/// A sink that cannot be written is no longer this function's concern at all:
/// it returns a value, and the decision to drop a failed write belongs to
/// whoever owns the stream — for `grove-llm leaf-insert` that is `report_insert`
/// in `crates/grove-llm/src/cli.rs`, which drops it because the insert has
/// landed and a lint that cannot print must not turn a reported mutation into a
/// failure.
pub fn stale_cross_refs(tree: &TreeWrite, renumbered: &[Renumber]) -> Result<Vec<String>, Error> {
    if renumbered.is_empty() {
        return Ok(Vec::new());
    }
    // The reading opening is a **second file description**, and two of them on
    // one directory do not share an `flock` — so an unspent write guard still
    // held here would block this call against its own process, forever. Giving
    // it up first is the whole of the fix, and it costs nothing: a `TreeWrite`
    // reopens for the next verb that asks either way.
    tree.relinquish();
    Ok(task_grow::stale_cross_refs(
        task_tree::read(tree.root())?,
        renumbered,
    ))
}
````
<!-- /fragment -->

Fifty-five lines, of which forty-four are comment: the longest item on the surface,
and the only one whose argument is mostly about locks. The claim that keeps it
off the count is precise — it is *the second half of `leaf-insert`'s contract,
which the store cannot supply because nothing in it knows what a reference is*.
A verb answers a question a session asked; this answers a question
`leaf-insert` left behind.

Three separate decisions are recorded here and each is a weakening. The lock is
**shared**, not exclusive, because holding readers off bought nothing but a
blocked `pick`. The hits are **returned**, not printed, because a stalled sink
under a tree lock wedged every grove process on the worktree. And the caller's
unspent write guard is **given up** before the second opening, because two file
descriptions on one directory do not share an `flock`.

That third one is the hazard this chapter meets four times in 516 lines, stated
once per item that has to live with it: here, where a guard is relinquished
(lines 176–181 and again at 200–204); at `finish_commit`, which may not be called
while one is held; and in `driver.rs`'s header, whose two operations take a
worktree path precisely because the driver has no opening to give them. One
`flock` fact, four sites, four different consequences — and no shared paragraph
anywhere, because each site needs a different half of it.

**The test is bounded because the regression is a hang.**
`the_cross_reference_lint_answers_through_an_unspent_write_guard`, at
`crates/grove-loop/tests/verbs.rs:345`, holds a `TreeWrite` open and unspent,
calls the lint from a second thread, and asserts that a 30-second channel receive
returns `Ok(true)`. Deleting `tree.relinquish()` does not make it fail an
assertion; it makes the receive time out. The assertion distinguishes three
outcomes — returned-and-`Ok`, returned-and-`Err`, and never returned — while its
failure message names only the third, so a lint that began refusing for an
unrelated reason would fail under a message about blocking.

**The test file calls it a verb.** That same test opens with *The one verb that
opens the tree while a `TreeWrite` is in hand*, eleven lines after the file it
tests declares that it is not a thirteenth verb. The distinguishing clause is
exact — `finish_commit` also opens the tree, and must *not* be called with one
held — so only the noun is loose. On a page whose subject is a count kept by
everything around it saying why it is out, the crate's own evidence file spending
the word is worth noticing rather than smoothing.

**The boundary named and not crossed.** Lines 190–195 hand the decision about a
failed write to `report_insert` in `crates/grove-llm/src/cli.rs` — a real
function, at line 688 of that file — and stop there. That binary is outside this
book, and this is the chapter that says so rather than following it.

<a id="the-three-that-change-a-leafs-standing"></a>
## The three that change a leaf's standing

Three verbs change what a live leaf is without moving its bytes anywhere a
reader has to follow. The first promotes it to a node; the other two mark it.

<!-- fragment «verbs-leaf-decompose» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="211-231" parent="the-twelve-verbs" -->
````rust

/// Turn a leaf into a node, its bytes becoming the node's charter, with one
/// first child.
///
/// `kind` **overrides** the inherited kind rather than defaulting it: with no
/// override the first child is driven as the decomposed leaf was.
///
/// # Errors
///
/// A path that is the root, a brief or an already-terminal leaf; a `finish`
/// kind; or a store refusal.
pub fn leaf_decompose(
    tree: &TreeWrite,
    leaf: &Path,
    first_child: &Slug,
    kind: Option<&Kind>,
) -> Result<Decomposed, Error> {
    let (brief, first_child) =
        tree_lifecycle::leaf_decompose(tree.guard()?, leaf, first_child, kind.cloned())?;
    Ok(Decomposed { brief, first_child })
}
````
<!-- /fragment -->

Its report is the second of the four declared beside their verb, and it names
the two paths a promotion produces.

<!-- fragment «verbs-decomposed» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="232-240" parent="the-twelve-verbs" -->
````rust

/// What [`leaf_decompose`] wrote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decomposed {
    /// The former leaf's bytes, now the node's charter.
    pub brief: PathBuf,
    /// The node's first child, so a node is never childless.
    pub first_child: PathBuf,
}
````
<!-- /fragment -->

*`kind` **overrides** the inherited kind rather than defaulting it* is the whole
of what this verb adds to chapter 12's promotion, and `Decomposed`'s second field
comment — *so a node is never childless* — is an invariant stated in a struct
rather than asserted in code.

<!-- fragment «verbs-leaf-retire» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="241-249" parent="the-twelve-verbs" -->
````rust

/// Mark one leaf `DONE` in place. Filename only — the file's bytes do not move.
///
/// # Errors
///
/// A path that is the root, a brief, a node or an already-terminal leaf.
pub fn leaf_retire(tree: &TreeWrite, leaf: &Path) -> Result<PathBuf, Error> {
    Ok(tree_lifecycle::leaf_retire(tree.guard()?, leaf)?)
}
````
<!-- /fragment -->

`leaf_retire` marks one leaf and returns one path. The bulk mark cannot promise
either, and its comment says so before its signature does.

<!-- fragment «verbs-leaf-prune» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="250-269" parent="the-twelve-verbs" -->
````rust

/// Mark abandoned work `ABANDONED` in place: one leaf, or every *live* leaf
/// beneath one node.
///
/// Filename only, and **not atomic across a subtree** — a subtree prune is *N*
/// rewrites under *N* guards, which is what
/// `docs/adr/bulk-marks-are-not-atomic.md` records and why the report below
/// names every path it marked.
///
/// # Errors
///
/// A path that is the root or a brief, or a store refusal partway through — in
/// which case the message names what had already been marked.
pub fn leaf_prune(tree: &TreeWrite, path: &Path) -> Result<Pruned, Error> {
    let result = tree_lifecycle::leaf_prune(tree.guard()?, path)?;
    Ok(Pruned {
        marked: result.marked,
        left_done: result.left_done,
    })
}
````
<!-- /fragment -->

The report is where the non-atomicity becomes usable rather than merely
admitted: two vectors, one of what was marked and one of what was deliberately
not.

<!-- fragment «verbs-pruned» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="270-279" parent="the-twelve-verbs" -->
````rust

/// What [`leaf_prune`] marked, and what it deliberately left alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pruned {
    /// Every leaf newly marked `ABANDONED`, in the order it was marked.
    pub marked: Vec<PathBuf>,
    /// Retired leaves in the subtree, left as they were: abandoning work that
    /// was finished would misreport it.
    pub left_done: Vec<PathBuf>,
}
````
<!-- /fragment -->

The pair chapter 13 read, and the one place on this surface where a decision
record is cited by name: `docs/adr/bulk-marks-are-not-atomic.md` accepts that a
subtree prune is *N* rewrites under *N* guards. The verb's obligation under that
record is discharged by the return type — `Pruned` names every path it marked, so
an interrupted run is legible — and `left_done`'s comment states the second rule
the type enforces, that finished work is never re-marked as abandoned.

<a id="the-two-that-reach-outward"></a>
## The two that reach outward

The last two verbs are the ones that do not take a tree from their caller,
because what each of them reaches is not the tree. The first reaches the version
control system.

<!-- fragment «verbs-finish-commit» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="280-302" parent="the-twelve-verbs" -->
````rust

/// Commit the teardown the finish session performed. **Reaches the VCS seam.**
///
/// The tree is revalidated under the exclusive lock, deleted through the store,
/// and the deletion committed — in that order, with the lock held right up to
/// the unlink. Grove takes a commit; it does not implement a transaction
/// (principle 1), so what puts a failed teardown back is `jj undo`, and every
/// refusal here says so.
///
/// **It opens the tree itself**, unlike every other verb here, because the
/// teardown's whole subject is a tree that stops existing — there is nothing to
/// hand back a [`TreeWrite`] over. So do not call it while holding one: two file
/// descriptions on one directory do not share an `flock`, and this would block
/// against the caller's own opening, forever. See [`TreeWrite`]'s header.
///
/// # Errors
///
/// A grove root that is absent, is not a directory, or is a symlink; live work
/// remaining; a handle that is not the live finish leaf's; an untracked tree,
/// which no `jj undo` could restore; or a commit that failed.
pub fn finish_commit(workspace: &Workspace, finish: &Handle) -> Result<Commit, Error> {
    Ok(tree_lifecycle::finish_commit(workspace, finish)?)
}
````
<!-- /fragment -->

The VCS seam, in three lines of code under nineteen lines of comment. Two of those
lines carry the crate's whole method: *Grove takes a commit; it does not
implement a transaction (principle 1), so what puts a failed teardown back is
`jj undo`, and every refusal here says so.* Chapter 14 read the teardown this
delegates to; what belongs to the surface is the ordering the comment fixes —
revalidate, delete, commit, with the lock held to the unlink — and the warning
that this is the one verb a caller must **not** invoke while holding a
`TreeWrite`. `finish_commit_tears_the_tree_down_and_commits_it` and
`finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, at
`crates/grove-loop/tests/verbs.rs:489` and `:512`, pin the two halves: the tree
is gone and the commit is named, or the tree is untouched and the handle is
refused.

<!-- fragment «verbs-complete» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="303-327" parent="the-twelve-verbs" -->
````rust

/// Write the relaunch flag to the signal file and return. **Reaches the
/// runner's channel.**
///
/// Ending the session is the loop driver's job — it is watching for this very
/// channel — so there is nothing else to do here. Outside a loop it is a no-op
/// that says so.
///
/// # Errors
///
/// A signal file that could not be written.
pub fn complete(signal_file: Option<&Path>, done: bool) -> Result<Signalled, Error> {
    let disposition = if done {
        complete::Disposition::Done
    } else {
        complete::Disposition::Relaunch
    };
    match signal_channel(signal_file) {
        Some(path) => {
            complete::signal(&path, disposition)?;
            Ok(Signalled::Wrote(path))
        }
        None => Ok(Signalled::NoLoop),
    }
}
````
<!-- /fragment -->

The twelfth verb, and the only one of the twelve that touches no tree. It maps a
`bool` onto a two-variant enum, asks where the channel is, and either writes or
reports that there was nowhere to write. *Ending the session is the loop driver's
job — it is watching for this very channel — so there is nothing else to do here*
is the sentence the whole of `complete.rs` exists to justify, and the last section
of this chapter reads that justification.

The question it asks first is the second thing on this file that is not a verb.

<a id="the-second-that-is-not-a-verb"></a>
## The second that is not a verb

`complete` asks one question before it writes, and the question is public.

<!-- fragment «verbs-signal-channel» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="328-341" parent="the-twelve-verbs" -->
````rust

/// Whether [`complete`] would signal, and where.
///
/// **Public because the order matters.** The caller admits this session against
/// the channel it is about to signal, and it has to ask *before* the write — so
/// the answer cannot be something [`complete`] returns.
#[must_use]
pub fn signal_channel(signal_file: Option<&Path>) -> Option<PathBuf> {
    signal_file.map(Path::to_path_buf).or_else(|| {
        std::env::var_os("GROVE_SIGNAL_FILE")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    })
}
````
<!-- /fragment -->

*Public because the order matters.* A verb is invoked to change or read the
tree; this is invoked to decide whether an invocation is admissible at all. The
caller admits its session against the channel it is about to signal, which it can
only do *before* the write — so the answer cannot be something `complete`
returns, and a private helper would put it out of reach of the only caller that
needs it.

Seven lines, and two of them are load-bearing in ways nothing local reveals. The
[loop control channel](../../../CONTEXT.md#loop-control-channel) is the
collision-resistant per-launch path the driver watches, and the glossary adds the
fact that makes the `or_else` necessary: *the foreground launch is the only site
that sets it, and every other harness spawn must scrub it.* A session is handed
the path in its environment, not in its arguments.

**The `.filter(|value| !value.is_empty())` is not defensive.** This repository is
a meta-grove — its test suite *is* the loop machinery, and it is normally run
from inside a live session whose environment carries a real
`GROVE_SIGNAL_FILE`. `.cargo/config.toml` force-clears that variable to the
**empty string** for everything cargo runs, and the file says why empty is
stronger than an inert path: a nonempty signal path is session-epoch authority.
The filter is what makes that empty string read as *no loop* rather than as a
path named `""`. `an_empty_signal_environment_is_no_loop_context`, in
`crates/grove-llm/tests/complete.rs`, asserts the variable is `Some("")` **first**
and then that `signal_channel(None)` is `None`, which is what makes it a test of
the filter rather than of the environment.

**And the `or_else` itself is observed by nothing, for a reason worth stating.**
Replacing the whole body with `signal_file.map(Path::to_path_buf)` — deleting the
environment fallback outright — leaves the suite exactly as an unmutated control
run of the same workspace copy found it: 558 tests, 547 passed, 11 failed, the
eleven being `crates/grove-loop/tests/prompt.rs` in a copy that is not a jj
repository. Not one test newly fails. That zero is not a gap in the suite but a
consequence of the guard above it: cargo force-clears the variable, so under
`cargo test` the `or_else` branch can only ever yield `None`, and no in-process
test can reach the arm without mutating an environment the crate is careful never
to mutate. The arm is live in production — every session this loop launches
receives its channel exactly this way — and unreachable under the harness by
construction.

The measurement is only worth the reading if the harness could have seen a
difference, so it was checked against a mutation that should be observed:
deleting the emptiness filter instead turns exactly three tests red —
`an_empty_signal_environment_is_no_loop_context`,
`no_channel_at_all_is_answered_rather_than_refused` and
`the_channel_can_be_asked_for_before_it_is_written` — at the same 558-test total,
so nothing failed to compile.

<!-- fragment «verbs-signalled» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="342-351" parent="the-twelve-verbs" -->
````rust

/// What [`complete`] did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Signalled {
    /// The flag was written to this channel; the loop driver will act on it.
    Wrote(PathBuf),
    /// There is no channel — this session is not running under the loop driver,
    /// and whoever started it ends it.
    NoLoop,
}
````
<!-- /fragment -->

`NoLoop` is the shape of the *outside a loop it is a no-op that says so*
promise: not an error, not a silent success, but a named variant whose comment
says who ends the session instead.

<a id="one-place-the-crate-crosses-that-boundary"></a>
## One place the crate crosses that boundary

The file ends on its only private function, and on the boundary the whole module
exists to keep.

<!-- fragment «verbs-sought» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="352-363" parent="the-twelve-verbs" -->
````rust

/// `Option` in, [`Sought`] out — the one place the crate crosses that boundary.
///
/// The modules behind these verbs answer `Option` because they are grove's own
/// internals and Rust's `Option` combinators are what they are written in. What
/// the *surface* answers is the store's word, so no consumer has to invent one.
fn sought<T>(found: Option<T>) -> Sought<T> {
    match found {
        Some(value) => Sought::Match(value),
        None => Sought::Nothing,
    }
}
````
<!-- /fragment -->

Six lines, private, and the reason `pick`, `kind` and `resolve` are one line
each. The comment states the rule as a division of ownership rather than a
convenience: `Option` is what grove's internals are written in, and `Sought` is
what the *surface* answers, so no consumer has to invent a word for a search that
matched nothing. It is the smallest item in the file and the only one that
exists purely to keep a vocabulary boundary in one place.

<a id="the-two-that-would-have-made-it-fourteen"></a>
## The two operations that would have made it fourteen

The second file is 57 lines long and holds two functions. Twenty-six of those
lines are the header arguing that the two do not belong next to the twelve.

<!-- fragment «driver-not-fourteen-header» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="1-26" parent="driver-operations" -->
````rust
//! **The two tree operations the loop driver performs that no session verb
//! exposes**, and the reason they are not in [`crate::verbs`].
//!
//! A verb is something a session invokes deterministically mid-task. These are
//! neither: [`transition_to_current`] runs before any session exists, and
//! [`materialize_finish`] creates the one leaf `leaf-add` is forbidden to create
//! (`finish` is driver-reserved). Putting them beside the twelve would say the
//! surface has fourteen verbs, which it does not.
//!
//! `loop-crate-driver-k22` moved `DriverLease`, `compose` and `run` into this
//! crate, so both of these are now [`crate::run`]'s internals and nothing
//! outside the crate calls either in production. **The module stayed public
//! anyway**, and the reason is stated rather than glossed: they are the only way
//! to put a tree into the two states the verb suite has to test against — the
//! pre-loop transition, and the driver-reserved `finish` leaf `verbs::leaf_add`
//! refuses to write. Making them crate-private would have bought nothing the
//! compiler can check and cost a second copy of `tests/verbs.rs`'s jj fixture
//! harness inside the crate.
//!
//! **Both open the tree themselves**, so neither may be called while a
//! [`crate::TreeWrite`] or [`crate::Tree`] is held: two file descriptions on one
//! directory do not share an `flock`, and the call would block against the
//! caller's own opening, forever. That is why they take a worktree path rather
//! than an opening — the driver has no opening to give them, because it runs
//! these *before* it has one.

````
<!-- /fragment -->

Fifty-seven lines, forty-two of them comment, and the whole file exists to hold
two functions apart from twelve others. The definition it turns on is one clause
long — *a verb is something a session invokes deterministically mid-task* — and
both operations fail it in different directions: `transition_to_current` runs
*before* any session exists, and `materialize_finish` writes the one leaf
`leaf-add` is forbidden to write. The sentence that names the cost is this
chapter's title: putting them beside the twelve *would say the surface has
fourteen verbs, which it does not*.

**Fourteen is the wrong answer twice over, by two different routes.** `verbs.rs`
declares fourteen public functions and twelve verbs, so a reader counting
declarations overcounts by two. `driver.rs` holds two more operations over the
same tree, so a reader counting tree operations reaches fourteen from the other
side. The two exceptions in `verbs.rs` and the two files' separation are the same
correction applied at both ends.

**The justification for staying public is stated, and half of it is observable.**
The header says the two are *the only way to put a tree into the two states the
verb suite has to test against*. One of those states is reached that way:
`crates/grove-loop/tests/verbs.rs` calls `driver::materialize_finish` at lines 495
and 516, in the two `finish_commit` tests, because nothing else may write a
`finish` leaf. The other is not. `driver::transition_to_current` has exactly one call
site in the whole workspace — `crates/grove-loop/src/loop_driver.rs:243`, which is
production — and the suite reaches a scaffolded grove through `verbs::root_init`
over a `Vacancy` instead, in the `scaffold` helper at `tests/verbs.rs:75`. So the
verb suite tests against one of the two states, not both, and the module's
publicity is earned by `materialize_finish` alone. The decision is still the
right one — a crate-private module would cost a second copy of the jj fixture
harness, as the header says — but the reason it gives covers one of its two
subjects.

The `flock` argument arrives here for the fourth time and in its most consequential
form: it is *why they take a worktree path rather than an opening*. The driver
runs these before it has an opening, so there is nothing to hand them.

<!-- fragment «driver-imports» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="27-32" parent="driver-operations" -->
````rust
use std::path::Path;

use crate::{task_tree, tree_lifecycle, Error, Selection};

pub use tree_lifecycle::CurrentTransition;

````
<!-- /fragment -->

The first brings a working tree to a state the loop can drive — a grove that was
already there, or a fresh one — and it runs before there is a session to invoke
it.

<!-- fragment «driver-transition-to-current» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="33-42" parent="driver-operations" -->
````rust
/// Bring a worktree to a state the loop can drive: a grove, or a fresh one.
///
/// # Errors
///
/// A tree that holds only its charter, or one whose entries are in no grammar
/// grove reads — both of which grove names and refuses rather than repairs
/// (principle 2).
pub fn transition_to_current(worktree: &Path) -> Result<CurrentTransition, Error> {
    Ok(tree_lifecycle::transition_to_current(worktree)?)
}
````
<!-- /fragment -->

The second writes the leaf the growing verbs refuse to write, and it is the only
function in either file that opens the tree in its own body.

<!-- fragment «driver-materialize-finish» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="43-57" parent="driver-operations" -->
````rust

/// The resumable `finish` leaf for an otherwise empty tree — or the live leaf
/// that turned up under the same lock, in which case nothing is written.
///
/// One observation for both halves: the re-selection that may return early and
/// the append that happens when it does not read the **same** snapshot, so
/// nothing can appear between them.
///
/// # Errors
///
/// A store refusal, or a sentinel created without a key.
pub fn materialize_finish(worktree: &Path) -> Result<Selection, Error> {
    let guard = task_tree::write(&worktree.join(".grove"))?;
    Ok(tree_lifecycle::materialize_finish(guard)?)
}
````
<!-- /fragment -->

Two bodies of one and two lines. `transition_to_current` forwards; only
`materialize_finish` does anything here, and what it does is open the tree, which
is the whole reason it cannot be a verb taking a `TreeWrite`. Its comment states
the snapshot discipline chapter 14 read from the other side — *the re-selection
that may return early and the append that happens when it does not read the
**same** snapshot, so nothing can appear between them* — which is question 2 of
the stated outcome, asked of an operation that has no session to ask on its
behalf.

<a id="the-child-side-of-the-loop"></a>
## The child side of the loop

The third file is the loop's child-side half: 96 lines whose entire output is one
word in one file. Its header explains why that is the correct amount of work.

<!-- fragment «complete-header-in-plain-comments» owner="twelve-not-fourteen" source="crates/grove-loop/src/complete.rs" lines="1-18" parent="complete-verb" -->
````rust
// The `grove-llm complete` verb — the in-loop completion signal (self-driving-loop).
//
// The agent runs this as its **last step** of a task (after commit + retire).
// It is the "external exit" an interactive `claude` cannot perform on itself:
// finishing a turn does not make `claude` quit, so the loop needs an
// out-of-band kill triggered on the agent's command.
//
// Realisation: this verb only writes the disposition into the signal file.
// The out-of-band kill itself is the loop driver's job (src/loop_driver.rs),
// which watches for this file and applies grace → SIGTERM → kill-grace →
// SIGKILL to the harness session it spawned (driver-side watcher). That split
// exists because an in-agent self-kill cannot be trusted under every harness
// sandbox: codex's Seatbelt denies a same-sandbox process signalling its own
// session (`(allow signal (target same-sandbox))`), so the previous
// self-spawned delayed killer silently failed under codex. The driver is the
// harness's own parent process, outside any sandbox the harness runs under,
// so it can always signal its child.

````
<!-- /fragment -->

**These seventeen lines are invisible to every instrument this book uses.**
`cargo doc --no-deps --document-private-items` reports thirty warnings across
`grove-loop` and **none** for `complete.rs` — but it sees only `///` and `//!`,
and this header is written in `//`. The clean result is evidence about the file's
four doc comments and about nothing at all above line 19. A module header in
plain comments is checked by reading it, and by nothing else.

Read, it argues one thing: the split. The agent writes a disposition; the driver
performs the kill. The ground is that *an in-agent self-kill cannot be trusted
under every harness sandbox*, with codex's Seatbelt named as the case that broke
the previous self-spawned killer.

`docs/ARCHITECTURE.md`'s `self-driving-loop` record argues the same split and
adds the escalation the comment reproduces — grace, SIGTERM, kill-grace, SIGKILL
— together with the reason it is the launcher's: it is the session's parent,
outside whatever sandbox the session runs under. Two things about that citation
are worth stating rather than assuming. The anchor is shared: `self-driving-loop`,
`do-is-sole-lifecycle-verb` and `fresh-grove-start-contract` all sit on one
section, so a bare parenthesised reference lands a reader on ground that argues
three records. And the record attributes *the launch, the watch and the kill* to
`crates/keyed-launch`, where this comment attributes them to `src/loop_driver.rs`.
Both hold, at different altitudes: `loop_driver.rs` chooses the control
directory, the variable name, the scrub list and the two graces, then hands the
supervision to `keyed_launch::run`. Chapter 20 owns that file. `keyed-launch` is
another crate with its own book, and this book names it and stops.

**One clause is in the wrong order.** *The agent runs this as its **last step** of
a task (after commit + retire)* names both prerequisites correctly, and the verb
does run after both. Read as a sequence it reverses the one the
[task commit boundary](../../../CONTEXT.md#task-commit-boundary) fixes: everything
the commit must cover — the artifact, whatever the grow verbs wrote, and the
`DONE` rename — is written by retire, so **retire precedes commit**, not the
other way round. The order is load-bearing rather than stylistic, because a
commit taken first cannot contain the rename that has not happened yet. The
parenthetical reads as a list of two prerequisites; it is only as a sequence that
it is wrong, and the corpus is frozen here, so the page says so and the file keeps
its bytes.

<!-- fragment «complete-imports» owner="twelve-not-fourteen" source="crates/grove-loop/src/complete.rs" lines="19-21" parent="complete-verb" -->
````rust
use anyhow::Result;
use keyed_launch::Token;

````
<!-- /fragment -->

Two things a finished session can mean, and the enum is deliberately not three.

<!-- fragment «complete-disposition» owner="twelve-not-fourteen" source="crates/grove-loop/src/complete.rs" lines="22-38" parent="complete-verb" -->
````rust
/// What a finished session tells the self-driving loop to do next. The agent
/// picks this when it signals; the loop driver reads it back from the signal
/// file (self-driving-loop). The third case — *no* signal at all — is the
/// *absence* of a [`Disposition`], represented by [`interpret`] returning
/// `None`, so the loop can tell a clean finish from an abnormal exit. That case
/// is only ever **reached** when the session process itself ends (a crash, or a
/// human `/exit`/Ctrl-C): an agent that simply forgets to signal does not get
/// there at all — see [`interpret`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    /// Relaunch with fresh context for the next task (the default — today's
    /// behaviour, fired after every per-task commit + retire).
    Relaunch,
    /// The whole grove is finished — stop the loop cleanly (the Finish cycle's
    /// last teardown action, `grove-llm complete --done`).
    Done,
}
````
<!-- /fragment -->

Two variants and a third case that is deliberately not a variant. The absence of
a signal is `None` from `interpret` rather than a `Disposition::Crashed`, and the
comment states the discrimination that buys: the loop can tell a clean finish
from an abnormal exit. The clause *an agent that simply forgets to signal does
not get there at all* is the trap the next fragment's comment spends twenty lines
on.

<!-- fragment «complete-disposition-token» owner="twelve-not-fourteen" source="crates/grove-loop/src/complete.rs" lines="39-52" parent="complete-verb" -->
````rust

impl Disposition {
    /// The sentinel written to / read from the signal file. Only `Done` needs a
    /// distinguished token; any other (or unrecognised, e.g. a stale binary's
    /// legacy `"complete"`) content reads back as `Relaunch`, the safe default.
    const DONE_TOKEN: &'static str = "done";

    fn token(self) -> &'static str {
        match self {
            Disposition::Relaunch => "relaunch",
            Disposition::Done => Self::DONE_TOKEN,
        }
    }
}
````
<!-- /fragment -->

Only `Done` needs a distinguished token. `Relaunch` is what any other content
reads back as, which makes the wire format asymmetric on purpose: an unrecognised
token is safe, and a corrupted `done` degrades to *keep going* rather than to
*stop the grove*.

<!-- fragment «complete-interpret» owner="twelve-not-fourteen" source="crates/grove-loop/src/complete.rs" lines="53-86" parent="complete-verb" -->
````rust

/// Interpret the token a finished session left in its completion channel.
///
/// **This is the whole of grove's stake in the channel's content.** The runner
/// carries the token opaquely — allocating the path, watching for it, reading
/// the bytes back — and hands it here without ever having decided what it
/// means; the meaning is one match, in one place, on grove's side of the seam.
///
/// `None` = no token at all, and the driver only ever observes that when the
/// session *process* ended without signalling: a human `/exit`/Ctrl-C, or a
/// crash → the loop stops.
///
/// An agent that finishes its work and forgets the verb is **not** that case,
/// and reading it as one is what made this failure mode hard to see. The
/// configured templates launch *interactive* harnesses (no `-p`, no `exec`), so
/// finishing a turn returns the session to its prompt and it never exits: the
/// runner's supervision sits on a channel that will never appear and a child
/// that will never exit, and the loop **stalls** rather than stopping. Nothing
/// downstream of here can distinguish that from a session still working — which
/// is why the reminder to signal is delivered at the moment of decision, on
/// `leaf-retire`/`leaf-prune`'s stderr.
///
/// `Some(Done)` = a clean whole-grove finish;
/// `Some(Relaunch)` = relaunch the next task (also the backward-compatible
/// reading of any present-but-unrecognised content, e.g. a stale binary's
/// legacy `"complete"`).
pub fn interpret(token: Option<&Token>) -> Option<Disposition> {
    let token = token?;
    if token.as_str().trim() == Disposition::DONE_TOKEN {
        Some(Disposition::Done)
    } else {
        Some(Disposition::Relaunch)
    }
}
````
<!-- /fragment -->

Thirty-four lines, of which eight are code. *This is the whole of grove's stake in
the channel's content* — the runner allocates the path, watches it and reads the
bytes without deciding what they mean, and the meaning is one `match` on grove's
side of the seam. That is the same division the glossary states from the
operator's side: the channel's **appearance alone** ends the session, with content
read only to tell `Relaunch` from `Done`.

The twenty lines in the middle are not about this function. They record a failure
mode — an agent that finishes and forgets the verb — and the reason it was hard to
see: the configured templates launch *interactive* harnesses, so finishing a turn
returns the session to its prompt, the child never exits, and the loop **stalls**
rather than stopping. The glossary's own `_Avoid_` for the
[loop control channel](../../../CONTEXT.md#loop-control-channel) states the same
trap and adds what settles it — which of the two a forgotten `complete` produces
is a property of the configured command, not of grove. Nothing downstream of
`interpret` can distinguish a stalled session from a working one, which is why
the fix named here is a reminder delivered somewhere else entirely, on
`leaf-retire`'s stderr.

Seven tests in `crates/grove-llm/tests/complete.rs` pin this file, and they round-trip
through a real `keyed_launch::Channel` rather than a file the test invents —
because, as that file's own header says, a test that wrote and parsed its own
framing would agree with itself while disagreeing with the driver.
`an_absent_token_is_none` and `unrecognised_signal_content_is_treated_as_relaunch`
take the two arms above; `relaunch_signal_is_read_back_as_relaunch` and
`done_signal_is_read_back_as_done` take the round trip.

**The `.trim()` is observed by nothing.** `Channel::read` already returns
`content.trim_end()`, so by the time a token reaches line 81 its trailing
whitespace is gone; the only thing the trim still does is tolerate *leading*
whitespace, and nothing in this workspace writes a token with any — `signal`
writes a bare literal. Removing it and re-running the same suite gives the
control's numbers unchanged: 558 tests, 547 passed, the same 11 `prompt.rs`
failures, nothing newly red. It is defence against a producer that does not exist,
which is a fair thing for a wire format to carry and not a fair thing to call
tested.

<!-- fragment «complete-signal» owner="twelve-not-fourteen" source="crates/grove-loop/src/complete.rs" lines="87-96" parent="complete-verb" -->
````rust

/// Write the disposition into the channel the driver allocated.
///
/// Written through `keyed_launch::signal`, the child-side half of that channel:
/// this process holds only the path it was handed, and the file's framing
/// belongs to whoever reads it back.
pub(crate) fn signal(path: &std::path::Path, disposition: Disposition) -> Result<()> {
    keyed_launch::signal(path, disposition.token())?;
    Ok(())
}
````
<!-- /fragment -->

The last four lines of the surface, and the only `pub(crate)` item in this
chapter. *This process holds only the path it was handed, and the file's framing
belongs to whoever reads it back* is the seam in one sentence: grove chooses the
word, `keyed-launch` chooses the bytes around it.

<a id="what-could-not-move-here"></a>
## What could not move

**On the way in — the names.** This surface is stated entirely in a grammar the
store cannot check. Every verb that names an entry takes a `Slug`, a `Kind`, a
`Reference` or a `Handle`, and `ordinal-fs-tree` has a word for none of them — it
has entries, ordinals and keys. The price is visible in the module header's first
sentence and paid in `sought`: twelve functions that mostly forward, existing so
that the vocabulary has one owner and the driver and the verbs run on one
definition of a kind.

**On the way through — the preconditions.** The surface's contribution to
question 2 is structural rather than procedural, and `root_init` is the clearest
case in the crate: it takes a `Vacancy` instead of a path, so *the refusal to
clobber is the shape and not a check*. Where a precondition could not be made
into a type it is made into a warning at the point of declaration —
`finish_commit` may not be called under a `TreeWrite`, `stale_cross_refs` gives
one up, `driver`'s two take a worktree path because there is no opening yet —
which is the same `flock` fact told four times because four items must act on
different halves of it.

**On the way out — the policy.** Two choices here could not have been defaulted
by anything underneath. That the surface is twelve rather than fourteen is one:
nothing in a store, a runner or a version control system could have decided that
a lint following an insert and a question asked before a write are not verbs, and
the cost of the decision is four doc comments that exist only to argue it. That
ending a session is the *driver's* job and not the agent's is the other, and its
cost is a whole file — `complete.rs` — whose 96 lines write a single token, plus
the split that puts the kill in a process the sandbox cannot deny.

The surface is twelve verbs. Two functions beside them and two files apart from
them say why they are not, and the count survives because they do.

[Previous: Finishing](14-finishing.md) | [Contents](README.md) | [Next: One live driver per working tree](16-the-lease.md)
