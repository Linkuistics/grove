# The session verbs, and the driver operations
<!-- book-page id="the-verbs" slice="twelve-not-fourteen" order="15" -->
[Previous: Finishing](14-finishing.md) | [Contents](README.md) | [Next: One live driver per working tree](16-the-lease.md)

<a id="twelve-not-fourteen"></a>
## The rule: the surface states which operations are verbs

Chapter 14 deleted the grove. This chapter reads the public surface that names
each session operation and the separate driver operations.

`verbs.rs` declares thirteen public functions: twelve verbs and the
`stale_cross_refs` helper that completes `leaf-insert`'s contract. `driver.rs`
holds two operations that prepare the tree before a session owns it. The surface
has twelve verbs after completion moves to harness-dispatch.

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
  record_teardown(worktree, launch_dir) -> Recorded

                                             paths or a report of paths
```

Read the right-hand column rather than the left. The six calls are the surface in
the order the fourteen chapters before this one met them, and the figure is here
for what they answer with rather than for what they do: a path, a report of
paths, or the fact that there was nothing. That the whole surface reports in one
shape is what makes the sections below a reading of a *surface* rather than of
fourteen unrelated functions.

<a id="fourteen-declarations-twelve-verbs"></a>
## Thirteen declarations, twelve verbs

Ten verbs receive a tree opening: one vacancy, four shared readers, and five
writers. `finish_commit` receives a workspace and opens the tree itself;
`record_teardown` receives the worktree and an optional launch directory. These
two reach beyond the tree opening: one to the VCS seam, one to the launch record.

| Group | Verbs |
|---|---|
| consumes a vacancy | `root_init` |
| reads under a shared lock | `pick`, `kind`, `brief_chain`, `resolve` |
| grows the tree | `leaf_add`, `leaf_insert` |
| changes a leaf's standing | `leaf_decompose`, `leaf_retire`, `leaf_prune` |
| reaches outward | `finish_commit`, `record_teardown` |

<a id="what-the-surface-says-about-itself"></a>
## What the surface says about itself

The two roots this chapter owns are declared whole here, each as one
composite whose children are the items below in file order.

<!-- fragment «the-twelve-verbs» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="1-331" parent="source-verbs" -->
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
<!-- insert «verbs-sought» -->
<!-- insert «verbs-record-teardown» -->
<!-- insert «verbs-recorded» -->
<!-- /fragment -->
<!-- fragment «driver-operations» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="1-59" parent="source-driver" -->
<!-- insert «driver-not-fourteen-header» -->
<!-- insert «driver-imports» -->
<!-- insert «driver-transition-to-current» -->
<!-- insert «driver-materialize-finish» -->
<!-- /fragment -->


The file opens by saying what it is for, which is unusual only because the
answer is not a behaviour. Twelve of its functions forward almost immediately
to `task_tree`, `task_grow` or `tree_lifecycle`; what the module contributes is
that they are gathered, named in one vocabulary, and counted.

<!-- fragment «verbs-surface-header» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="1-13" parent="the-twelve-verbs" -->
````rust
//! **The twelve verbs a session invokes over its grove.**
//!
//! They live here rather than with the store because ten of the twelve touch the
//! tree and every one is stated in grove's vocabulary — brief chains, kinds,
//! outcomes, handles, finishing — none of which the store has a word for.
//! Co-locating them gives the handle grammar one owner and puts the driver and
//! the verbs on one definition of a kind. One reaches the VCS seam
//! ([`finish_commit`]); [`record_teardown`] writes into the current launch
//! directory without ending the harness run.
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

<!-- fragment «verbs-imports» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="14-24" parent="the-twelve-verbs" -->
````rust

use std::path::{Path, PathBuf};

use ordinal_fs_tree::Sought;

use crate::{
    task_grow, task_tree, tree_lifecycle, Commit, Error, Handle, Kind, Reference, Selection, Slug,
    Tree, TreeWrite, Vacancy, Workspace,
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

<!-- fragment «verbs-root-init» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="25-50" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-initialized» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="51-59" parent="the-twelve-verbs" -->
````rust

/// What [`root_init`] wrote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Initialized {
    /// The grove's charter, `.grove/_BRIEF.md`.
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

<!-- fragment «verbs-pick» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="60-73" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-kind» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="74-86" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-brief-chain» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="87-96" parent="the-twelve-verbs" -->
````rust

/// Every ancestor node file, root-first: `_BRIEF.md`, then `_<slug>.md`.
/// The guarded opening refuses missing or misplaced node files.
///
/// # Errors
///
/// A path that is not a leaf of this tree.
pub fn brief_chain(tree: &Tree, leaf: &Path) -> Result<Vec<PathBuf>, Error> {
    Ok(task_tree::brief_chain(tree, leaf)?)
}
````
<!-- /fragment -->


The public `kind` and `brief_chain` functions consume an already-open tree.
A supplied path that is not a leaf is an error. Every successfully opened level
has exactly one correctly placed node file, so a brief chain contains the root
file followed by each positioned ancestor's titled file. Missing files refuse
at open, before either function can return a partial answer.


<!-- fragment «verbs-resolve» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="97-109" parent="the-twelve-verbs" -->
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

**Ambiguity is an answer, not an error** is this surface's most consequential
decision, and the reason is in the next clause: the caller is a session that can
re-ask. Chapter 9 read the four spellings and the `Ambiguous` case that lists
every match's handle. The verb adds only the judgement that a list of candidates
belongs in the success channel.

<a id="the-two-that-grow"></a>
## The two that grow

Two verbs grow the tree, and both take a `&TreeWrite` — the write opening whose
guard a mutation consumes. The first appends; the second makes room.

<!-- fragment «verbs-leaf-add» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="110-133" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-leaf-insert» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="134-154" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-not-a-thirteenth-verb» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="155-209" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-leaf-decompose» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="210-230" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-decomposed» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="231-239" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-leaf-retire» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="240-248" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-leaf-prune» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="249-268" parent="the-twelve-verbs" -->
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

<!-- fragment «verbs-pruned» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="269-278" parent="the-twelve-verbs" -->
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

The last three verbs are the ones that do not take a tree from their caller,
because what each of them reaches is not the tree. The first reaches the version
control system.

<!-- fragment «verbs-finish-commit» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="279-301" parent="the-twelve-verbs" -->
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

The other outward-reaching verb, `record_teardown`, records an absent tree in
the current launch directory. It does not end a harness run. Dispatch owns that
last effect through `harness-dispatch exit`, so Grove's twelve-verb surface has
one teardown-record operation and no completion-token operation.

<a id="one-place-the-crate-crosses-that-boundary"></a>
## One place the crate crosses that boundary

The helper crosses the module's vocabulary boundary, and on the boundary the whole module
exists to keep.

`record_teardown` delegates the absent-root check and record creation to the
launch-directory module. It takes the worktree and optional directory and returns
`Recorded::Wrote(path)` or `Recorded::NoLoop`; the CLI holds the admitted epoch
through the call. This twelfth verb separates teardown disposition from dispatch exit. The directory module and the driver that reads it are explained
in [The launch directory carries identity and teardown](19-the-loop.md#launch-control-area).

<!-- fragment «verbs-sought» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="302-313" parent="the-twelve-verbs" -->
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

The teardown operation takes the worktree and optional launch directory, checks
that `.grove/` is absent through the launch-directory module, and returns the
recorded path. It runs while the CLI retains the shared epoch guard. Dispatch
exit can therefore end the harness without conflating its receipt with this
Grove filesystem fact.

<!-- fragment «verbs-record-teardown» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="314-324" parent="the-twelve-verbs" -->
````rust

/// Record the completed teardown in the current launch directory.
/// Outside a loop this is a no-op. The CLI holds epoch admission across the write.
///
/// # Errors
/// `.grove/` still exists, or the record cannot be created.
pub fn record_teardown(worktree: &Path, launch_dir: Option<&Path>) -> Result<Recorded, Error> {
    Ok(crate::launch_directory::record_teardown(
        worktree, launch_dir,
    )?)
}
````
<!-- /fragment -->

`Recorded` names the two successful effects: a record was written, or no loop
context was supplied. Neither variant requests that a harness exit.

<!-- fragment «verbs-recorded» owner="twelve-not-fourteen" source="crates/grove-loop/src/verbs.rs" lines="325-331" parent="the-twelve-verbs" -->
````rust

/// The effect of recording a teardown, independent of ending the harness run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Recorded {
    Wrote(PathBuf),
    NoLoop,
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

The second file is 59 lines long and holds two functions. Twenty-six of those
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
declares thirteen public functions and twelve verbs, so a reader counting
declarations overcounts by two. `driver.rs` holds two more operations over the
same tree, so a reader counting tree operations reaches fourteen from the other
side. The two exceptions in `verbs.rs` and the two files' separation are the same
correction applied at both ends.

**The justification for staying public is stated, and half of it is observable.**
The header says the two are *the only way to put a tree into the two states the
verb suite has to test against*. One of those states is reached that way:
`crates/grove-loop/tests/verbs.rs` calls `driver::materialize_finish` at lines 495
and 516, in the two `finish_commit` tests, because nothing else may write a
`finish` leaf. The other is not. `driver::transition_to_current` has one
production call site, `crates/grove-loop/src/loop_driver.rs:243`, and this crate's
verb suite does not use it at all: `tests/verbs.rs` reaches a scaffolded grove
through `verbs::root_init` over a `Vacancy` instead, in the `scaffold` helper at
line 75. So the verb suite tests against one of the two states, not both, and the
justification the header gives is earned by `materialize_finish` alone.

**The publicity is nonetheless spent, and by a caller the header does not
describe.** `default-root-slug-two-spellings-k159` added
`both_scaffolding_doors_name_the_first_leaf_the_same` to
`crates/grove-llm/tests/root_init.rs`, which calls
`grove_loop::driver::transition_to_current` to scaffold a grove the way the driver
does and compare its first leaf against `root-init`'s ([chapter
11](11-a-grove-begins.md#the-value-nothing-holds) carries why). That is a *second
package's* test rather than this crate's verb suite, so it discharges nothing the
header claims — but it is the first thing outside this crate to call the
operation, which is exactly what a `pub` on a module nothing outside called was
paying for in advance. The decision is still the right one — a crate-private
module would cost a second copy of the jj fixture harness, as the header says —
and the gap between the reason given and the use made is the finding, not the
`pub`.

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
it. It takes the worktree and nothing else. No kind is asked about before the
first leaf is written: whether anything can launch a `requirements` session is
the owner's policy's to say, when that leaf launches.

<!-- fragment «driver-transition-to-current» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="33-44" parent="driver-operations" -->
````rust
/// Bring a worktree to a state the loop can drive: a grove, or a fresh one.
/// No kind is checked: whether the owner's policy routes the first leaf's kind
/// is asked when that leaf launches.
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

<!-- fragment «driver-materialize-finish» owner="twelve-not-fourteen" source="crates/grove-loop/src/driver.rs" lines="45-59" parent="driver-operations" -->
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

`transition_to_current` hands the worktree to the lifecycle, which opens the tree
once under its own lock; `materialize_finish` opens the tree itself, which
is the whole reason it cannot be a verb taking a `TreeWrite`. Its comment states
the snapshot discipline chapter 14 read from the other side — *the re-selection
that may return early and the append that happens when it does not read the
**same** snapshot, so nothing can appear between them* — which is question 2 of
the stated outcome, asked of an operation that has no session to ask on its
behalf.

<a id="what-could-not-move-here"></a>
## What could not move

Grove owns twelve verbs because only it names brief chains, kinds, outcomes,
handles and finishing. `stale_cross_refs` is the second half of `leaf-insert`,
not an additional verb. The two driver operations sit separately because they
prepare the tree a session is about to receive. Harness completion belongs to
dispatch; recording Grove teardown belongs here because only Grove can check
that `.grove/` is gone.

[Previous: Finishing](14-finishing.md) | [Contents](README.md) | [Next: One live driver per working tree](16-the-lease.md)
