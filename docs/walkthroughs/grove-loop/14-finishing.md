# Finishing
<!-- book-page id="finishing" slice="the-tree-deletes-itself" order="14" -->
[Previous: Outcomes are marked in place](13-outcomes.md) | [Contents](README.md) | [Next: The twelve verbs, and the two that are not](15-the-verbs.md)

<a id="the-tree-deletes-itself"></a>
## The rule: the driver writes the last leaf, and the ending deletes the tree

Chapter 13 left the grove in the state only chapter 13 could produce: every leaf
terminal, nothing live, and the whole of that fact spelled in filenames. This
chapter takes that tree and ends it.

> **The driver is the only author of the leaf that ends the grove, and the ending
> deletes the tree.** Every other leaf in a grove is written by an operator verb
> at somebody's request. The last one is written by the loop itself, because by
> the time it is needed there is no session left to ask — and the session it
> launches exists to delete the directory all of this has been kept in.

Two halves, and they are the chapter's two production concerns. The **transition**
brings a working tree to a state the loop can drive and, when the work is over,
materializes the one leaf `leaf-add` is forbidden to write. The **teardown**
revalidates that leaf, deletes `.grove/` and takes the commit that records the
deletion. Between them they are the only operations in the crate that create a
grove without being asked and destroy one on purpose.

**Say which tree.** In `ordinal-fs-tree`'s vocabulary the sentinel is an `append`
at the root level and the teardown is a `delete` of the root itself; the library
has an entry, an ordinal and a key, and no word at all for *finish*, for a
*driver*, or for a grove being *over*. In grove's vocabulary a *leaf* is one
session's whole work and the finish leaf is a session that has not happened yet.
The store will create that entry as readily as any other — what stops every
operator verb from writing one is a check on grove's side of the seam, and this
chapter owns the deliberate exception to it. Where a sentence below could belong
to either vocabulary, it says which.

The carried example reaches its last step, and its observable end is that it
stops existing.

```text
<worktree>/.grove/
├── BRIEF.md
├── 01-DONE-requirements--plan-k1.md      every leaf terminal
└── 02-build-k3/
    ├── BRIEF.md
    └── 01-DONE-impl--step-k4.md

materialize_finish(guard)          no live work under an exclusive guard

<worktree>/.grove/
├── BRIEF.md
├── 01-DONE-requirements--plan-k1.md
├── 02-build-k3/
│   ├── BRIEF.md
│   └── 01-DONE-impl--step-k4.md
└── 03-finish--finish-k5.md        the one leaf that leaf-add may not write
                                   "# finish-k5", and inside it:
                                   "grove-llm finish-commit finish-k5"

finish_commit(workspace, finish-k5)

<worktree>/                        no .grove at all
  ⇒ Ok( <commit: "finish-k5: remove completed grove task tree"> )
```

This is why the book is not illustrated from a live `.grove/`, and the reason is
this page rather than a preference: `finish-commit` removes the tree, so a book
that pointed at a real one would be citing an artifact its own last chapter
destroys.

<a id="the-file-opens-here"></a>
## Why the book's last chapter owns the file's first line

`tree_lifecycle.rs` is 2,725 lines, the largest root in the corpus, and it is
divided into nine ownership blocks across four chapters. The file is ordered by
Rust convention — items, then `#[cfg(test)] mod tests` — and the book is ordered
by concept, so the two orders disagree. They disagree most sharply here.

**The file opens on finishing and the book closes on it.** This chapter owns
`tree_lifecycle.rs` lines 1 to 331 — the module header, the imports, and every
item of the finish lifecycle — which is the file's *first* block and the largest
of its five production blocks. It also owns lines 1,467 to 1,665, the eight
inline tests that exercise the transition and the sentinel. 530 lines in all,
and the chapter that reads them is the twentieth of the book's twenty
source-owning pages.

The inversion is not an accident of layout. A module that holds *beginning,
outcomes and ending* has to declare the ending's types before anything can
return them, and `CurrentTransition` sits at the top because the transition is
the first thing the loop calls. The book cannot open there, because finishing is
the one lifecycle operation whose whole meaning is the state the other three
leave behind: there is nothing to finish until a grove has begun, grown and been
worked through.

Two consequences run through the rest of this page, and both are the shape of
this part rather than a mis-cut.

**Chapter 11 owns items this chapter is the main consumer of.** `RootShape` and
`root_shape`, `default_root_slug`, `grove_name` and `initialize_grove` are all
declared in [chapter 11's blocks](11-a-grove-begins.md#one-operation-or-none) and
read here. `default_root_slug` and `root_shape` have exactly one caller each
anywhere in the workspace, and both callers are in this chapter's block, at lines
82 and 87; `grove_name` has two, of which line 81 is one and chapter 11's
`initialize_grove` is the other. Chapter 11 reproduces and explains all three and
points forward. What is owed here is the account of *what the driver's own
scaffold is for*, which is this chapter's and no other's.

**And this chapter's tests are split from its production across a chapter
boundary.** Every test helper the eight tests below call — `worktree`,
`guard_at`, `root_init_at`, `touch`, `body`, `name_of`, `list`, `a_kind` — is
defined in chapter 11's support block at lines 1,077 to 1,466. So is
`leaf_retire`, which two of them call to reach a tree with no live work; that is
[chapter 13's verb](13-outcomes.md#two-verbs-one-mark). This page reads none of
them and depends on all of them.


<a id="the-module-header"></a>
## The header, and two claims in it that do not hold

The file's first forty-three lines are its module header, and they are the
crate's clearest statement of what the domain-free libraries left behind.

<!-- fragment «finish-transition» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1-331" parent="source-tree-lifecycle" -->
<!-- insert «finishing-module-header» -->
<!-- insert «finishing-imports» -->
<!-- insert «finishing-default-slug» -->
<!-- insert «finishing-current-transition» -->
<!-- insert «finishing-transition-contract» -->
<!-- insert «finishing-transition-body» -->
<!-- insert «finishing-materialize-contract» -->
<!-- insert «finishing-materialize-body» -->
<!-- insert «finishing-new-finish-leaf» -->
<!-- insert «finishing-finish-handle» -->
<!-- insert «finishing-finish-slug» -->
<!-- insert «finishing-finish-body» -->
<!-- insert «finishing-commit-contract» -->
<!-- insert «finishing-commit-classify» -->
<!-- insert «finishing-commit-revalidate» -->
<!-- insert «finishing-delete-contract» -->
<!-- insert «finishing-delete-body» -->
<!-- insert «finishing-recoverable-contract» -->
<!-- insert «finishing-recoverable-body» -->
<!-- /fragment -->
Nineteen fragments reconstruct the block, in file order: the header and the
imports, then the eleven items, with the four longest split at the line where
their doc comment ends and their body begins.

<!-- fragment «finishing-module-header» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1-44" parent="finish-transition" -->
````rust
// **This module is what happens to a grove that the store has no word for: it
// begins, its leaves reach terminal outcomes, and it ends.** Everything about
// ordinals, keys and locks is `ordinal-fs-tree`'s, and since
// `collapse-tree-access-k13` that includes the lock — the grove's own creation
// was the last thing here that reached past the store, and it goes through
// `Vacancy::initialize` now.
//
// The **lifecycle verbs** (task-tree-scheme) — `root-init`, `leaf-decompose`,
// `leaf-retire`, `leaf-prune`, and the driver-owned finish lifecycle — expressed against the real **directory
// tree**, built on the name grammar (`src/task_name.rs`) and the grow verbs
// (`src/task_grow.rs`), both of which run through `ordinal-fs-tree` since the
// flip (gh issue #13, increment 2). Keeps task-tree-scheme's *semantics* (a fresh grove
// starts with one live leaf so it is never mistaken for finished; decompose
// enforces a first child; retire is leaves-only and done-ness is marked in
// place; prune marks abandonment in place, pruning) and changes the
// *mechanics* to the filesystem's shape:
//
//   * `root-init` creates the grove whole — the root, its `BRIEF.md` (the one
//     unkeyed singleton) and a first **requirements** leaf
//     `01-requirements-<slug>-k1.md`, a 2-digit per-level position — as one
//     store operation under one lock;
//   * `leaf-decompose` turns the leaf *file* `NN-<kind>--<slug>-k<key>.md` into a node
//     *directory* `NN-<slug>-k<key>/` (**key preserved** — the entity that was the
//     leaf becomes the node), renaming the leaf body in as the node's `BRIEF.md`
//     and growing a first child atomically so a node is never childless;
//   * `leaf-retire` adds a `DONE` infix in place (`NN-<kind>--<slug>-k<key>.md` →
//     `NN-DONE-<kind>--<slug>-k<key>.md`), keeping the retired leaf in its directory at its
//     position — no `done/` directory;
//   * `leaf-prune` adds an `ABANDONED` infix in place, symmetric with retire, but
//     — per pruning — accepts a **node** too: marking every *live* leaf
//     in the subtree (bulk, since one decision can kill many leaves at once),
//     leaving `DONE` leaves alone.
//   * the finish lifecycle materializes one resumable finish leaf for an otherwise
//     empty tree, then revalidates and commits the tree's removal after explicit
//     human confirmation.
//
// **Position-free headers:** a leaf/brief header is the stable handle
// `# <slug>-k<key>` (`# … — brief` for a node), so `leaf-retire`/`leaf-prune`
// leave the file's content byte-identical (the outcome infix is filename-only)
// and `leaf-decompose` only appends ` — brief` to the handle.
//
// These are the verbs `llm_cli` dispatches; the flat-scheme `leaf_lifecycle` they
// replaced is gone.

````
<!-- /fragment -->

The header is the spine's own words for this module: *what happens to a grove
that the store has no word for*. Everything about ordinals, keys and locks is
`ordinal-fs-tree`'s — and since `collapse-tree-access-k13` that includes the
lock, so the last thing in this file that reached past the store was the grove's
own creation, and it goes through `Vacancy::initialize` now. What is left is
three events the store cannot name: a grove **beginning**, its leaves reaching
terminal **outcomes**, and it **ending**. Those three are chapters 11 to 14, and
the header's five bullets are the book's Part III in the order the file wrote
them.

**Two of the header's claims do not hold, and the page settles both here rather
than repeating them.** The corpus is frozen — this book fixes nothing in
`crates/grove-loop/` — so an adjudication is what a reader gets instead of a
correction, and each defect is held by a leaf of its own.

**The first is an address.** Line 42 reads *These are the verbs `llm_cli`
dispatches*. There is no `llm_cli` anywhere in this workspace and there is no
`src/llm_cli.rs`; the module that dispatches these verbs is
`crates/grove-llm/src/cli.rs`, whose `run` matches `Command::RootInit`,
`Command::LeafDecompose`, `Command::LeafRetire`, `Command::LeafPrune` and
`Command::FinishCommit` onto the five `cmd_*` functions that call them. **The
behaviour the sentence claims is correct and only the address is stale** — the
verbs really are the ones that binary dispatches — which is why this is a clause
on the page and not a leaf. The same spelling survives in four comments across
the crate; chapter 6 adjudicated the one at `task_tree.rs` line 1,070 and
[chapter 11](11-a-grove-begins.md#what-the-tests-establish) the one at line
1,147 of this file, on the same ground.

**The second is a citation, and it is the more interesting of the two.** Line 8
tags the lifecycle verbs `(task-tree-scheme)`, and line 12 says the module
*keeps task-tree-scheme's semantics*, listing four: that a fresh grove starts
with one live leaf so it is never mistaken for finished; that decompose enforces
a first child; that retire is leaves-only and done-ness is marked in place; and
that prune marks abandonment in place. A bare parenthesised word in this crate's
comments is an anchor citation into `docs/ARCHITECTURE.md`, and that document's
`task-tree-scheme` section is *Task-tree data model*. Read it, and it states the
last of the four and none of the first three: it gives the grammar, the diagram,
positions, keys, handles, the one-node-species rule, malformed against foreign,
and the sentence *`DONE` and `ABANDONED` are terminal filename infixes, so
picking and rendering need not parse file contents*. It says nothing about a
first live leaf, nothing about decompose enforcing a first child, and nothing
about retire being leaves-only.

The record for the first of those is a different anchor in the same document —
`fresh-grove-start-contract`, on *Lifecycle and resumption*, which argues it
exactly: a fresh grove creates a first leaf and not just a brief, **because
`pick` skips briefs**, so a brief-only tree would report no live leaves and be
indistinguishable from a finished one. That is this module's own rule, and the
comment cites the data model where the contract has its own record two sections
away. The lesson generalises past this line: a parenthesised word is cheap to
write and reads like a proof, and the only way to know what it carries is to
open the section and read it. This is the fourth such citation the book has
checked — chapter 13 found `(pruning)` cited five times for an arity asymmetry
its section does not argue.

**A third thing about this header is not a defect but is worth a reader's
attention, because nothing would report it.** The forty-three lines above are
written in `//`, not `//!`. `cargo doc --no-deps --document-private-items` sees
only `///` and `//!`, so none of this text is documentation as far as the
toolchain is concerned: it renders nowhere, its links are never resolved, and
its claims are never checked against anything. Run over this crate the command
emits thirty warnings — ten in `task_name.rs`, nine in `lib.rs`, six in
`task_tree.rs`, four in `verbs.rs` and one in `prompt.rs` — and **not one of them
names `tree_lifecycle.rs`**. That clean result is true and says nothing whatever
about the file's first forty-three lines. Both stale claims above sit inside
them. A module header written in `//` is checked by reading it, and by nothing
else.


Then the imports, which are the whole file's and not this chapter's alone.

<!-- fragment «finishing-imports» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="45-53" parent="finish-transition" -->
````rust
use crate::task_grow;
use crate::task_name::{Handle, Kind, Outcome, Parts, Slug, TaskName};
use crate::task_tree::{self, Guard, Opening, TreeVacancy};
use anyhow::{bail, Context, Result};
use jj_workspace::{Commit, Workspace};
use ordinal_fs_tree::{Entry, EntryName, Found, Key, NewEntry, Report, Snapshot, Target, Verdict};
use std::fs;
use std::path::{Path, PathBuf};

````
<!-- /fragment -->

Eight `use` lines carry the four chapters of this file. `Handle`, `Kind`,
`Outcome`, `Parts`, `Slug` and `TaskName` are [chapters 2 to
4's](02-the-tokens.md#the-four-verdicts) grammar; `Guard`, `Opening` and
`TreeVacancy` are [chapter 5's](05-opening.md#the-four-openings) openings; the
`ordinal_fs_tree` line names six library types the store hands back. Two
dependencies are this chapter's alone, and they are the only two in the file:
`jj_workspace::{Commit, Workspace}` is imported for the teardown and used
nowhere else, which is the shape of the seam this chapter reaches and no other
chapter of this file does.


<a id="one-opening-one-lock"></a>
## The transition: one opening, one lock, one operation

Before any session exists, the loop has to know what it is looking at. A working
tree may hold a grove, or hold nothing, or hold something that is not a grove at
all, and the answer decides whether there is anything to drive. That question is
`transition_to_current`'s, and its answer is two variants wide.

The constant above it is the first item in the file and belongs to the
transition rather than to `root-init`.

<!-- fragment «finishing-default-slug» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="54-57" parent="finish-transition" -->
````rust
/// The slug a grove scaffolded by the driver is named with. `root-init` takes
/// one from its operator; the lifecycle transition has nobody to ask.
const DEFAULT_ROOT_SLUG: &str = "plan";

````
<!-- /fragment -->

**The comment states the asymmetry that makes this constant necessary.**
`root-init` is an operator verb and takes a slug from whoever ran it; the
lifecycle transition has nobody to ask, because it runs before a session exists.
So the driver needs a name it can supply itself, and `"plan"` is it — which is
why the first entry of every grove this book has shown is
`01-requirements--plan-k1.md`, and why that literal is checkable rather than
illustrative.

Two things about it are worth a reader's care, and both are recorded rather than
argued here. The wrapper `default_root_slug` that validates it into a `Slug` is
[chapter 11's](11-a-grove-begins.md#the-value-nothing-holds), sixty lines below
in the file, and its own doc comment opens *The slug `root-init` uses when
nobody supplied one* — which `root_init` does not, taking a validated `Slug`
from its caller; the one caller `default_root_slug` has anywhere is line 82 of
this block. And the string `"plan"` is spelled a second time, independently, as
`#[arg(default_value = "plan")]` in `crates/grove-llm/src/cli.rs`. Chapter 11
established by mutation that each literal is pinned by its own test and their
*agreement* by none. Both findings are chapter 11's to carry; this chapter needs
only that the constant exists because the driver has nobody to ask.

<!-- fragment «finishing-current-transition» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="58-63" parent="finish-transition" -->
````rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrentTransition {
    RootInitialized,
    AlreadyCurrent,
}

````
<!-- /fragment -->

Two variants, and the type is the whole of what the transition reports. There is
no third for *refused*: a refusal is an `Err`, and the enum carries only the two
outcomes in which a tree is now drivable. `AlreadyCurrent` is the overwhelmingly
common case — every iteration of a running loop but the first — and
`RootInitialized` happens once in a grove's life.

The type is `pub` here and re-exported by `crate::driver`, which is [chapter
15's](README.md#contents) module and the only thing outside this crate that
names it.

<!-- fragment «finishing-transition-contract» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="64-74" parent="finish-transition" -->
````rust
/// Classify or perform the one lifecycle transition needed before current-tree
/// selection: a working tree with no grove gets one.
///
/// **One opening, one lock, one operation.** This used to be two phases —
/// classify under grove's own guard, then append the first leaf under the
/// library's — because the two `flock` the same directory through different
/// open file descriptions and would have deadlocked nested. Grove holds no lock
/// of its own since `collapse-tree-access-k13`, and the store answers *there is
/// no tree here* as a **shape** rather than an error, so the whole transition is
/// one [`task_tree::write_or_vacancy`] and the vacancy arm creates the grove
/// under the lock it already holds.
````
<!-- /fragment -->

**The comment argues a shape, and the argument is a deadlock that used to
happen.** This was two phases: classify the root under grove's own guard, then
append the first leaf under the library's. Both `flock` the same directory
through *different open file descriptions*, and an `flock` is not reentrant
across descriptions — so the second acquisition blocked on the first, held by the
same thread, forever. The fix was not to order the two locks but to stop having
two: grove has held no lock of its own since `collapse-tree-access-k13`, and the
store answers *there is no tree here* as a **shape** rather than an error. That
is what makes the whole transition one call.

The phrase *the vacancy arm creates the grove under the lock it already holds* is
the load-bearing one. A `Vacancy` is a lock over a root that holds no tree, and
it is consumed by the creation — so between deciding a tree is absent and
creating it there is no window at all, and no partial scaffold for a later
reader to recognise. The type system carries that: `root_init` **cannot** run
over a live grove, because it takes a `Vacancy` and a live grove does not yield
one. That is [chapter 1's stated claim](01-orientation.md#the-cast) and [chapter
5's opening](05-opening.md#the-four-openings).

<!-- fragment «finishing-transition-body» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="75-108" parent="finish-transition" -->
````rust
pub(crate) fn transition_to_current(worktree: &Path) -> Result<CurrentTransition> {
    let grove_root = worktree.join(".grove");
    match task_tree::write_or_vacancy(&grove_root)? {
        Opening::Vacancy(vacancy) => {
            initialize_grove(
                vacancy,
                &grove_name(&grove_root),
                &default_root_slug(),
                &Kind::requirements(),
            )?;
            Ok(CurrentTransition::RootInitialized)
        }
        Opening::Tree(tree) => match root_shape(&tree)? {
            RootShape::ATree => Ok(CurrentTransition::AlreadyCurrent),
            RootShape::Taskless => bail!(
                "the task tree in {} holds no task, only its charter. Grove creates a grove \
                 whole — the root, its `BRIEF.md` and a first leaf in one operation — so this \
                 is a tree something emptied rather than one Grove left half-built, and Grove \
                 does not repair a tree in place: put it back with `jj undo`, or move {} aside \
                 and let `grove` scaffold a fresh one",
                grove_root.display(),
                grove_root.display()
            ),
            RootShape::Unrecognised(names) => bail!(
                "the task tree in {} holds no Grove entries, only {}. Grove reads \
                 `NN-<kind>--<slug>-k<key>` names and does not migrate older layouts: rename these \
                 entries to that grammar, or move them aside and let `grove` scaffold a fresh tree",
                grove_root.display(),
                names.join(", ")
            ),
        },
    }
}

````
<!-- /fragment -->

Thirty-four lines, one `match`, and four outcomes. The shape is worth naming
before the arms are read: **one opening decides everything**.
`task_tree::write_or_vacancy` takes the exclusive lock once and returns either a
`Vacancy` — no tree here — or a `Tree`, and every answer below is derived from
that single observation. Nothing re-opens, nothing re-reads, and there is no
second look for a state to change under.

- **`Opening::Vacancy`** hands the vacancy straight to `initialize_grove`, with a
  name from `grove_name`, a slug from `default_root_slug` and the fixed kind
  `Kind::requirements()`. All four of those are chapter 11's; the answer is
  `RootInitialized`.
- **`Opening::Tree`** classifies with `root_shape` — also chapter 11's — and the
  three shapes are the three remaining answers.
- **`RootShape::ATree`** is the ordinary case: a grove is here, nothing to do,
  `AlreadyCurrent`.
- **`RootShape::Taskless`** and **`RootShape::Unrecognised`** are refusals, and
  they are refusals rather than repairs on purpose.

**The two refusals are not the same refusal, and the difference is what each
message has to say.** `Taskless` is a root holding its charter and no task — a
tree *something emptied*, not one grove left half-built, because grove creates a
grove whole and there is no partial scaffold it could have produced. So the
message says exactly that and offers `jj undo`, which is the operation log's job
rather than grove's. `Unrecognised` is a root holding entries in no grammar
grove reads: the message names them, quotes the shape a name should have had,
and says grove does not migrate older layouts. Neither arm repairs anything, and
the reason is a standing principle in this repository — a message naming what is
wrong and how to fix it, rather than machinery that guesses.

**`Unrecognised` is this chapter's alone, and it is the only arm of this
function that is.** Chapter 11 established the split by mutation and this chapter
re-ran it; the table is in *What the refusals are worth* below. In short: replacing the `Taskless` arm with a silent panic reddens one
test, and it is in chapter 11's block; replacing `Unrecognised` reddens one test,
and it is in this chapter's; replacing `ATree` reddens two, one on each side of
the boundary. The evidence for this one function is split across two chapters of
the book, which is what the file's inversion costs.

**A record describes this function as a four-row table, and the code has five
distinguishable answers.** `docs/ARCHITECTURE.md`'s *Lifecycle and resumption*
carries a transition table — no `.grove/` creates the scaffold; a root short of a
whole grove refuses; live leaves mean no transition; no live leaf appends or
reuses the finish leaf. Three of those four are this function's `Vacancy`,
`Taskless`-or-`Unrecognised`, and `ATree` arms; the fourth is `materialize_finish`
below, which this function never calls. The table's second row is one row over
two arms that carry different messages and are held by different tests, and its
last two rows are one arm here plus a decision taken elsewhere — `AlreadyCurrent`
is returned for *any* readable grove, and whether live work remains is a question
the loop asks afterwards. The table is a good summary of the loop's behaviour and
a lossy index of this function's; a reader tracing an arm should count the arms.


<a id="the-sentinel"></a>
## The one leaf no operator verb may write

Every other leaf in a grove exists because somebody asked for it. `leaf-add`
appends one, `leaf-insert` puts one at a position, `leaf-decompose` grows a
node's first child — and each takes a kind from its caller. The finish leaf is
the exception in both directions: nobody asks for it, and its kind is one the
verbs that could have written it are required to refuse.

<!-- fragment «finishing-materialize-contract» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="109-112" parent="finish-transition" -->
````rust
/// Materialize the driver-owned finish sentinel after a shared selection found
/// no live work. The exclusive re-selection closes the gap between that read
/// and allocation: newly inserted ordinary work wins, and an existing finish
/// is reused.
````
<!-- /fragment -->

**The first sentence names the precondition and the second names the race it
closes.** The loop reaches this function only after a *shared* selection found no
live work — and a shared read is exactly the kind of observation that can be
stale by the time anything acts on it. So the function does not trust it: it
re-selects under its own exclusive guard, and the re-selection and the allocation
that may follow read the **same** snapshot. Nothing can appear between finding no
live work and creating the sentinel, and an existing sentinel is returned rather
than duplicated.

That second property is what makes the driver's finish **resumable**. A finish
session that is declined, or that exits before teardown begins, writes no signal
and leaves the leaf live; the next `grove` comes back through here and gets the
same leaf rather than a second one. The
[guide's account of finishing](../../USAGE.md#usage-finish) states the operator's
half of that contract.

<!-- fragment «finishing-materialize-body» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="113-139" parent="finish-transition" -->
````rust
pub(crate) fn materialize_finish(tree: Guard) -> Result<crate::task_tree::Selection> {
    if let Some(selection) = task_tree::select_in_write(&tree)? {
        return Ok(selection);
    }

    // **The sentinel is the one leaf grove creates that no operator verb may.**
    // The library will `append` anything `Parts` can express, so what keeps a
    // `finish` kind out of `leaf-add` is `task_grow::refuse_finish_kind` on
    // grove's side, and this verb is the deliberate exception rather than a hole
    // in the check.
    let key = task_tree::next_key(tree.snapshot());
    // The body embeds the handle, and the handle embeds the key the library has
    // not allocated yet — so the key is predicted here and held to account by
    // `task_grow::allocated` below, exactly as every grow verb does
    // (`docs/ARCHITECTURE.md#tree-access-lock`).
    let entry = new_finish_leaf(key)?;
    let report = tree
        .append(Target::Root, entry)
        .map_err(task_tree::raised)?;
    let path = task_grow::allocated(report.created(), &[key])?.remove(0);
    Ok(crate::task_tree::Selection {
        path,
        handle: finish_handle(key.context("a finish sentinel was created without a key")?)?,
        kind: Kind::finish(),
    })
}

````
<!-- /fragment -->

Twenty-seven lines, and three things happen in them.

**The early return is the resumable half.** `select_in_write` under this guard
either finds work — newly inserted ordinary work, or the finish leaf a previous
iteration already created — and returns it untouched, or finds nothing and falls
through to the append. Ordinary work wins because it is simply what the selection
returns; there is no priority rule, and the finish kind gets no special handling
on the way out.

**The comment states the exception explicitly, which is the point of it.** The
library will `append` anything `Parts` can express, and `Parts` can express a
`finish` kind perfectly well — a kind is an open token, and the store has no
opinion about which ones exist. What keeps a `finish` leaf out of `leaf-add` is
`task_grow::refuse_finish_kind`, on grove's side of the seam, which is [chapter
10's](10-growing.md#a-list-is-not-n-calls). This verb is *the deliberate
exception rather than a hole in the check*: the rule and its one exemption are
both grove's, stated in two places that know about each other.

**And the key is predicted before it is allocated, because the body embeds it.**
`next_key` reads the greatest key in the snapshot and adds one; `new_finish_leaf`
builds the entry with that prediction already written into its bytes; the library
then allocates for real, and `task_grow::allocated` compares what was created
against what was predicted and refuses on a disagreement. This is the same
prediction-and-check every grow verb performs, and chapter 10 reads the check
itself. Here it matters more than usual, because a leaf whose predicted key was
wrong would not merely be misnamed — it would contain an instruction telling an
operator to run `grove-llm finish-commit` on a handle that does not exist.

<!-- fragment «finishing-new-finish-leaf» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="140-153" parent="finish-transition" -->
````rust
/// The driver's finish sentinel, as an entry to create.
///
/// A `None` key is an exhausted keyspace, which is
/// [`Refusal::KeysExhausted`](ordinal_fs_tree::Refusal) and the library's to
/// state; the entry carries no bytes then and never needs any, because a refusal
/// writes nothing (`task_grow::new_leaf` says the same of an ordinary leaf).
fn new_finish_leaf(key: Option<Key>) -> Result<NewEntry<Parts>> {
    let parts = Parts::leaf(Outcome::Live, Kind::finish(), finish_slug()?);
    Ok(match key {
        Some(key) => NewEntry::new(parts, finish_body(&finish_handle(key)?).into_bytes()),
        None => NewEntry::empty(parts),
    })
}

````
<!-- /fragment -->

**A `None` key is the exhausted keyspace, and the comment's clause 3 is the
design decision worth stopping on.** `next_key` returns `None` when the tree's
greatest key is already `u32::MAX`. Grove does not raise its own error for that.
It builds an entry carrying **no bytes** — `NewEntry::empty` — hands it to the
library, and lets the library state the condition, because key exhaustion is
`Refusal::KeysExhausted` and the library's fact to report. The parenthetical
notes that `task_grow::new_leaf` says the same of an ordinary leaf, so this is
the crate's standing habit rather than a local choice.

The reasoning behind the empty entry is *a refusal writes nothing*, and it is
sound: there is no point composing a body around a key that does not exist, for
an append that is going to be refused. It has a consequence the next fragment's
prose returns to — the `.context` guard in `materialize_finish` that would report
a sentinel created without a key is, because of this ordering, unreachable.

<!-- fragment «finishing-finish-handle» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="154-160" parent="finish-transition" -->
````rust
/// The driver's own leaf wears the handle `finish-k<key>`, composed through
/// [`Handle`] rather than spelled here — one of the five hand-rolled produce
/// sites `name-ownership-k14` retired.
fn finish_handle(key: Key) -> Result<Handle> {
    Ok(Handle::new(finish_slug()?, key))
}

````
<!-- /fragment -->

Three lines, and the comment's clause is a claim about the whole crate rather
than about this function: the handle is *composed through `Handle` rather than
spelled here*. `<slug>-k<key>` is written in exactly one place in production
code, and `name-ownership-k14` retired five hand-rolled sites that used to spell
it themselves. This was one of them. [Chapter 2](02-the-tokens.md#the-handle-in-this-grammar)
reproduces the header that makes the claim and chapter 3 reads the renderer; the
value here is that the driver's own leaf gets its name from the same grammar as
every operator-written leaf, so there is no second spelling to keep in step.

<!-- fragment «finishing-finish-slug» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="161-168" parent="finish-transition" -->
````rust
/// `finish`, validated as a [`Slug`]. A constant that has to go through the
/// validating constructor is still one constant, and going through it is what
/// keeps `Slug`'s guarantee — *a `Slug` that exists renders and re-parses* —
/// true of every `Slug` in the process rather than of most of them.
fn finish_slug() -> Result<Slug> {
    Slug::new("finish").map_err(|error| anyhow::anyhow!("slug \"finish\": {error}"))
}

````
<!-- /fragment -->

**A constant that goes through a validating constructor is still one constant,
and the comment says why that is not waste.** `Slug::new("finish")` cannot fail —
`finish` is lowercase ASCII with no dash at either end, no `--` inside it, and
not one of the three reserved words `BRIEF`, `DONE` and `ABANDONED` that
[chapter 3's `refuse_token`](03-kind-slug-handle.md#one-place-the-grammar-is-spelled)
rejects. The point is not that the check might catch something. It is that
`Slug`'s guarantee — *a `Slug` that exists renders and re-parses* — is a property
of **every** `Slug` in the process, and a single hand-built exception would make
it a property of most of them instead. A type whose invariant holds except in one
place has no invariant.

The measurable consequence is that this function's error arm is dead code that
earns its place: replacing the `map_err` with a silent panic changes no test's
result, because nothing can reach it.

<!-- fragment «finishing-finish-body» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="169-180" parent="finish-transition" -->
````rust
fn finish_body(handle: &Handle) -> String {
    format!(
        "# {handle}\n\n\
         ## Goal\n\n\
         Propose the complete finish cycle and wait for explicit human confirmation.\n\n\
         ## Done when\n\n\
         - Promote durable material from the grove briefs.\n\
         - Run `grove-llm finish-commit {handle}`.\n\
         - Run `grove-llm complete --done` as the last action.\n"
    )
}

````
<!-- /fragment -->

**The only item in this block with no doc comment, and the only one that writes
prose a human will read.** Eleven lines of `format!` produce the finish leaf's
whole body: a `# <handle>` header, a `## Goal` naming the one thing a finish
session does, and a `## Done when` of three steps.

Those three steps are the finish cycle, and the leaf is where an operator meets
them: promote durable material out of the briefs, run
`grove-llm finish-commit <handle>`, then run `grove-llm complete --done` as the
last action. The [guide's account](../../USAGE.md#usage-finish) states the same
three in the same order, and calls this grove's one routine human confirmation
point — because step two deletes the workstream tree.

**The handle appears twice in eleven lines, and that duplication is the point of
the next section.** Once as the header, which is the leaf's identity, and once
inside a command an operator is expected to type. Both are interpolated from the
same `Handle`, and the header is position-free by the same rule every leaf and
brief in the tree follows: [chapter 13](13-outcomes.md#the-pair-that-makes-in-place-mean-something)
depends on that, because it is what lets `leaf-retire` rename a file without
touching a byte of it.


<a id="three-spellings"></a>
## Three spellings of one key, and what holds them together

The sentinel's key is written down three times before any of them is checked
against the tree: in the entry's **filename**, composed by the library from
`Parts` and the key it allocates; in the `Selection`'s **handle**, composed by
grove from the key it predicted; and inside the leaf's **body**, twice, in the
header and in the `finish-commit` command. Nothing in the type system makes those
agree. If the library allocated a key different from the one grove predicted, the
result would be a leaf on disk called `03-finish--finish-k5.md` whose text told
an operator to run `grove-llm finish-commit finish-k4`.

What holds them together is not a check on the body. It is
`task_grow::allocated`, which compares the created entry's key against the
prediction and refuses to report success on a disagreement — so the body is
never *validated*, it is made unnecessary to validate, because the only key it
could contain is the one that was checked. `materialize_finish_writes_a_handle_that_matches_its_own_filename`
is the test that pins the three spellings agreeing, and it is read in full
below.


<a id="the-teardown"></a>
## The teardown: what is left when the transaction is gone

The last three items in the block are the ending itself, and they are best read
against what used to be here. The teardown once hand-built a transaction: a
witness file, a manifest, an evacuation, a rollback proof, a quarantine
directory and a recovery path. `delete-finish-transaction-k8` deleted all of it.

<!-- fragment «finishing-commit-contract» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="181-194" parent="finish-transition" -->
````rust
/// Revalidate the complete finish cycle's tree facts, delete `.grove/`, and
/// take the commit that records the deletion. This is a deterministic
/// last-moment guard; whether a human confirmed teardown is the calling finish
/// session's responsibility.
///
/// **The tree and VCS facts are the verb; the transaction around them is gone**
/// (`delete-finish-transaction-k8`). What survives is exactly what only grove
/// can say: that the live leaf is the driver-owned finish leaf the caller named,
/// that no ordinary work slipped in, and that only `.grove/` is deleted and
/// committed. What went is the witness, the manifest, the evacuation, the
/// rollback proof, the quarantine and the recovery path — jj snapshots the
/// working copy before every command and its operation log is the transaction
/// record, so a teardown that does not commit is restored by `jj undo`, which is
/// what the seam's own refusal says ([`jj_workspace::Workspace::commit`]).
````
<!-- /fragment -->

**The comment's second paragraph is the argument, and it is the crate's sharpest
statement of a boundary it keeps everywhere.** *The tree and VCS facts are the
verb; the transaction around them is gone.* What survives is exactly what only
grove can say — that the live leaf is the driver-owned finish leaf the caller
named, that no ordinary work slipped in, and that only `.grove/` is deleted and
committed. What went is everything that was re-implementing a guarantee jj
already provides: jj snapshots the working copy before every command and its
operation log **is** the transaction record, so a teardown that does not commit
is restored by `jj undo`.

The first paragraph draws a second boundary, and it is the one a reader is most
likely to want stated: *whether a human confirmed teardown is the calling finish
session's responsibility*. This function is a deterministic last-moment guard on
tree and VCS facts, and it is not a gate on human authority. It cannot be — a
library function has no way to know what a person was asked or what they said.
The guide calls teardown grove's one routine human confirmation point; that
confirmation lives in the session, and this verb assumes it happened.

<!-- fragment «finishing-commit-classify» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="195-226" parent="finish-transition" -->
````rust
pub(crate) fn finish_commit(workspace: &Workspace, finish: &Handle) -> Result<Commit> {
    let worktree = workspace.root();
    let grove_root = worktree.join(".grove");
    // **The root is classified before the tree is opened, because two of the
    // three answers are not the library's to give.** A `.grove` that is a
    // *symlink* to a directory is one the library would happily read — it
    // follows links, as every reader does — while this verb must refuse it
    // unfollowed, since it may not delete a directory elsewhere as if it were
    // its own tree. The guard below is still the authority on the tree; this is
    // the wording that authority cannot supply.
    match fs::symlink_metadata(&grove_root) {
        // Absence is now a plain refusal. It used to be routed to a proof that
        // the repository's immediate result *was* this attempt's teardown
        // commit, because a death mid-transaction exposed exactly this shape and
        // grove had to decide whether to re-run. There is no transaction to die
        // inside any more, so the answer is the operator's and the operation log
        // is where they read it.
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => bail!(
            "no Grove task tree at {}\n\n\
             If a `finish-commit` was interrupted after the deletion, `jj op log` shows whether \
             it committed and `jj undo` restores the tree if it did not.",
            grove_root.display()
        ),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("checking grove root {}", grove_root.display()))
        }
        Ok(metadata) if !metadata.file_type().is_dir() => {
            bail!("grove root is not a directory: {}", grove_root.display())
        }
        Ok(_) => {}
    }
````
<!-- /fragment -->

**The root is classified before the tree is opened, and the comment says why two
of the three answers are not the library's to give.** The load-bearing case is
the symlink. A `.grove` that is a *symlink to a directory* is one the library
would read perfectly happily, because readers follow links — and this verb must
refuse it **unfollowed**, since a verb whose next act is `delete` may not remove
a directory somewhere else on the strength of a link pointing at it. So
`fs::symlink_metadata` is used rather than `metadata`, and the distinction is the
whole reason this block exists ahead of the opening. The comment is careful about
what it is claiming: *the guard below is still the authority on the tree; this is
the wording that authority cannot supply.*

The absence arm carries a second piece of deleted history. It *used to be routed
to a proof that the repository's immediate result was this attempt's teardown
commit*, because a death mid-transaction produced exactly this shape — no
`.grove/`, and no way to tell a finished teardown from an interrupted one — and
grove had to decide whether to re-run. There is no transaction to die inside any
more, so the question is not grove's to answer: the message names `jj op log` and
`jj undo` and hands it to the operator. The refusal wording is grove's and the
remedy is jj's, which is the division this whole section keeps.

**Four arms, and the last is empty on purpose.** `Ok(_) => {}` is the ordinary
case falling through to the opening below; the three before it are the three
things that are wrong before a lock is worth taking.

<!-- fragment «finishing-commit-revalidate» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="227-259" parent="finish-transition" -->
````rust
    let tree = task_tree::write(&grove_root)?;
    let selection = task_tree::select_in_write(&tree)?
        .context("the requested finish leaf is no longer live")?;
    if !selection.kind.is_finish() {
        bail!(
            "cannot finish while live work remains: {} ({})",
            selection.handle,
            selection.path.display()
        );
    }
    // **The refusal quotes what the operator asked for; everything downstream
    // uses the tree's own handle.** `Handle::parse` — which the caller ran to
    // get here — is deliberately lenient on the key's spelling, so
    // `finish-k0001` is the live `finish-k1` and compares equal, which is right:
    // the operator meant that leaf. But the teardown commit is a permanent
    // record and must name the work item by the handle a name on disk actually
    // wore (`CONTEXT.md`, *Work-item handle*), so `selection.handle` is what
    // goes past this point.
    if &selection.handle != finish {
        bail!(
            "requested finish handle {finish} does not match the live finish leaf {}",
            selection.handle
        );
    }

    // The exclusive guard is held across the teardown: nothing else may observe
    // `.grove/` between the revalidation above and the commit below. It is
    // dropped when this function returns — after the root it names is gone,
    // which the lock does not care about, because the lock is on the directory
    // *containing* the root.
    delete_and_commit(workspace, tree, &selection.handle)
}

````
<!-- /fragment -->

**This is the revalidation, and every line of it is a fact only grove holds.**
The tree is opened exclusively, the live leaf is selected under that guard, and
three things are then checked in order: that there *is* a live leaf, that its
kind is `finish`, and that its handle is the one the caller named. Only then is
the guard handed on to `delete_and_commit`.

The second check is the one an operator meets most often. *Cannot finish while
live work remains* names the work it found, and the
[guide](../../USAGE.md#usage-finish) shows the same message: new work appearing
after the finish session launched makes teardown refuse and leaves the tree
untouched for the next iteration. That is the whole of the *no ordinary work
slipped in* guarantee, and it is possible only because the check and the deletion
happen under one guard.

**The third check's comment is a distinction worth the eight lines it takes.**
`Handle::parse` is deliberately lenient about a key's spelling, so an operator
who types `finish-k0001` is understood to mean the live `finish-k1`, and the two
compare equal. That leniency is right at the front door: the operator meant that
leaf. But the teardown **commit message** is a permanent record, and a permanent
record must name the work item by the handle a name on disk actually wore. So the
refusal quotes `finish` — what the operator asked for — and everything downstream
of this point uses `selection.handle`, the tree's own. Two handles that compare
equal are not two spellings of one string, and the code is explicit about which
of them survives into history.

The closing comment covers a question a careful reader will have about the lock.
The exclusive guard is held across the teardown, so nothing observes `.grove/`
between the revalidation and the commit — and it is dropped when the function
returns, *after the root it names is gone*. That is not a leak: the lock is taken
on the directory **containing** the root, which is the working-tree root rather
than `.grove/` itself. `CONTEXT.md`'s
[tree access lock](../../../CONTEXT.md#tree-access-lock) states the reason
directly — the root rather than `.grove/`, because it exists before the task tree
is created and through its deletion, so root initialization, finish allocation
and deletion all share one invariant. This chapter is the only place in the book
where the *through its deletion* half of that sentence is exercised.

<!-- fragment «finishing-delete-contract» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="260-264" parent="finish-transition" -->
````rust
/// Delete `.grove/` and commit the deletion. Two steps, and each names the
/// operation-log command that puts the tree back if it is the one that failed.
///
/// The refusal wording is grove's and the remedy is jj's: the seam has no
/// `.grove/` to speak about, and grove has no operation log to repair with.
````
<!-- /fragment -->
The body is the two steps the contract promised, and three comments inside it
carry more argument than the code does.

<!-- fragment «finishing-delete-body» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="265-307" parent="finish-transition" -->
````rust
fn delete_and_commit(workspace: &Workspace, tree: Guard, finish_handle: &Handle) -> Result<Commit> {
    let grove_root = tree.root().to_path_buf();
    let grove_root = grove_root.as_path();
    require_recoverable_grove(workspace, grove_root)?;
    // What makes a half-removed tree recoverable is that the snapshot holding it
    // is already in the operation log, not anything done here: `jj restore
    // .grove` returns what was removed — measured on jj 0.44.0
    // (`minimalism-k1`). Edits made since the last snapshot are outside that
    // log, which is a fact about a working copy jj has not seen rather than a
    // guarantee grove could hand-build, and principle 1 is that grove does not
    // hand-build it.
    //
    // **The removal is the store's.** `root-lifecycle-belongs-to-the-store`
    // moved both halves of a root's lifetime across, and
    // [`WriteGuard::delete`](ordinal_fs_tree::fs::WriteGuard::delete) is the
    // half this verb needs: it consumes the guard — so the lock is held right up
    // to the unlink and released with nothing left to guard — refuses a root
    // spelled through a link, and reports what went. A bare `remove_dir_all`
    // had none of that, and grove kept its own spelling of a store operation
    // for no reason but history.
    tree.delete().map_err(|error| {
        anyhow::anyhow!(
            "{error}\n\n\
             Jujutsu still holds the tree: restore what was removed with `jj restore .grove`, \
             then fix what blocked the deletion and rerun \
             `grove-llm finish-commit {finish_handle}`."
        )
    })?;
    let commit = workspace
        .commit(
            &[grove_root],
            &format!("{finish_handle}: remove completed grove task tree"),
        )
        .with_context(|| {
            format!(
                "the {finish_handle} teardown was not committed, and `.grove/` is deleted in \
                 the working copy. Once the tree is back, fix what made the commit fail and \
                 rerun `grove-llm finish-commit {finish_handle}`."
            )
        })?;
    Ok(commit)
}

````
<!-- /fragment -->

**Two steps, and the first comment is careful to say what does *not* make the
tree recoverable.** It is not anything this function does. What makes a
half-removed tree recoverable is that the snapshot holding it is already in the
operation log — `jj restore .grove` returns what was removed, measured on jj
0.44.0 — and edits made since the last snapshot are outside that log. The comment
states that limit rather than glossing it: it is *a fact about a working copy jj
has not seen rather than a guarantee grove could hand-build*, and the crate's
first principle is that grove does not hand-build it.

**The removal is the store's, and the comment argues why that mattered enough to
change.** `root-lifecycle-belongs-to-the-store` moved both halves of a root's
lifetime across the seam, and `WriteGuard::delete` is the half this verb needs.
Three properties come with it and none came with the `remove_dir_all` it
replaced: it **consumes the guard**, so the lock is held right up to the unlink
and released with nothing left to guard; it refuses a root spelled through a
link; and it reports what went. The comment's own verdict on the old code is
unusually blunt — *grove kept its own spelling of a store operation for no reason
but history*.

**Both error paths name the command that puts the tree back, and they name
different ones because different things failed.** A failed delete means jj still
holds the tree, so the advice is `jj restore .grove` and a rerun. A failed commit
is worse and the message says so: the tree is *already deleted in the working
copy*, so the advice is to get it back, fix what made the commit fail, and rerun.
Each message names the operation-log command for its own failure, which is what
the contract above promises.

<!-- fragment «finishing-recoverable-contract» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="308-317" parent="finish-transition" -->
````rust
/// The one precondition deletion has: jj can only put back what it tracks.
///
/// **This is not a surviving piece of the transaction.** A transaction promises
/// to undo its own work; this promises nothing and repairs nothing. It is the
/// gate that makes the version control system's guarantee *applicable* — an
/// untracked `.grove/` is outside the operation log, so no `jj undo` would
/// return it and the deletion below would be the unrecoverable kind. Principle
/// 2's answer to that is a message naming what is wrong and how to fix it,
/// which is what this is. One read-only probe, and nothing is written to record
/// it.
````
<!-- /fragment -->
The function itself is four lines: ask the seam whether jj tracks anything under
the root, return if it does, and otherwise refuse.

<!-- fragment «finishing-recoverable-body» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="318-331" parent="finish-transition" -->
````rust
fn require_recoverable_grove(workspace: &Workspace, grove_root: &Path) -> Result<()> {
    if workspace.is_tracked(grove_root)? {
        return Ok(());
    }
    bail!(
        "Jujutsu tracks nothing under {}, so deleting the task tree could not be undone\n\n\
         Grove takes a commit; it does not implement a transaction, and the operation log \
         can only restore what it tracks. Commit or track the task tree and rerun:\n      \
         jj commit -m \"track the grove task tree\" root:.grove\n\n\
         Nothing was deleted or changed.",
        grove_root.display()
    );
}

````
<!-- /fragment -->

**The doc comment exists to refuse a reading, and that is why it is ten lines for
a four-line function.** Something that runs before a deletion and refuses it
under a condition looks like the last surviving piece of the transaction that was
deleted. It is not, and the comment draws the distinction precisely: *a
transaction promises to undo its own work; this promises nothing and repairs
nothing*. It is the gate that makes the version control system's guarantee
**applicable**. An untracked `.grove/` is outside the operation log, so no
`jj undo` would return it and the deletion would be the unrecoverable kind.

The refusal is the crate's second principle in its purest form — a message naming
what is wrong and how to fix it, with the exact command to run — and its last
line is the one an operator most needs: *Nothing was deleted or changed.* The
whole function is one read-only probe, and the comment closes by saying nothing
is written to record it, which is what keeps the check from becoming state.

**This is the last item in the block, and it is the last line of grove's own
code that runs before a grove stops existing.**


<a id="what-the-eight-tests-establish"></a>
## What the eight tests establish, and what each would pass under

The chapter's second block is `tree_lifecycle.rs` lines 1,467 to 1,665 — 199
lines holding eight tests, at 17% comment prose against 15% across the file's
test module. Four exercise `materialize_finish` and four `transition_to_current`,
and every helper they call belongs to chapter 11's support block above them.

**A boundary worth stating before the tests are read.** These eight are not all
the inline tests of the two functions above. Three more `transition_to_current`
tests sit at lines 1,335, 1,385 and 1,447 — inside `root-init-tests`, which is
chapter 11's block — and they hold arms of this chapter's production. A test
belongs to the block its line number falls in, not to the verb it exercises, and
this file is where that is easiest to get wrong, because its concerns and the
book's chapters are deliberately inverted.

**And a larger absence, which shapes everything below.** Of the eleven items this
chapter owns, the three that perform the ending — `finish_commit`,
`delete_and_commit` and `require_recoverable_grove` — have **no inline tests at
all**. Not one of the eight below calls any of them, and none of the 245 inline
tests in this crate does. Their observers are all out of process, in
`crates/grove-llm/tests/finish_commit.rs` and `crates/grove-loop/tests/verbs.rs`,
which shell out to the binary and assert on its stderr. Those directories are the
book's evidence rather than its roots, so this page cites them and does not
reproduce them. The measurement section below is where that split does its work.

<a id="the-three-spellings-pinned"></a>
### The sentinel, and the three spellings of its key

<!-- fragment «finish-tests» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1467-1665" parent="source-tree-lifecycle" -->
<!-- insert «finishing-test-three-spellings» -->
<!-- insert «finishing-test-last-key» -->
<!-- insert «finishing-test-last-ordinal» -->
<!-- insert «finishing-test-reuse» -->
<!-- insert «finishing-test-already-current» -->
<!-- insert «finishing-test-malformed-name» -->
<!-- insert «finishing-test-no-grove-entries» -->
<!-- insert «finishing-test-dangling-symlink» -->
<!-- /fragment -->
The first test is the chapter's named pin, and the one the structure brief
nominates for the rule this page carries.

<!-- fragment «finishing-test-three-spellings» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1467-1495" parent="finish-tests" -->
````rust
    /// The driver's sentinel embeds its own key in its handle, its body and the
    /// `finish-commit` command it tells the session to run — so a key the library
    /// allocated differently would leave a leaf instructing an operator to commit
    /// a handle that does not exist. The prediction is checked against the report
    /// by `task_grow::allocated`; this pins the three spellings agreeing.
    #[test]
    fn materialize_finish_writes_a_handle_that_matches_its_own_filename() {
        let (_t, wt) = worktree();
        root_init_at(&wt, "plan").unwrap();
        let grove_root = wt.join(".grove");
        leaf_retire(
            guard(&grove_root),
            &grove_root.join("01-requirements--plan-k1.md"),
        )
        .unwrap();

        let selection = materialize_finish(guard_at(&wt)).unwrap();

        assert_eq!(name_of(&selection.path), "02-finish--finish-k2.md");
        assert_eq!(selection.handle.to_string(), "finish-k2");
        assert_eq!(selection.kind, a_kind("finish"));
        let body = body(&selection.path);
        assert!(body.starts_with("# finish-k2\n"), "got {body:?}");
        assert!(
            body.contains("grove-llm finish-commit finish-k2"),
            "got {body:?}"
        );
    }

````
<!-- /fragment -->

**What it establishes.** The key the library allocated, the handle grove returns
in the `Selection`, the leaf's filename, its `# <handle>` header and the
`grove-llm finish-commit <handle>` command inside its body are all the same key.
The doc comment states the failure it is written against, and it is a good one:
*a key the library allocated differently would leave a leaf instructing an
operator to commit a handle that does not exist.*

**What it would pass under while the property was broken.** The fixture is a
fresh grove with its single leaf retired, so the tree holds exactly one key and
`next_key` can only predict `2`. An implementation that ignored the prediction
and hard-coded `finish-k2` passes every assertion here. So does one that
allocated first and *then* composed the body from the allocated key — which would
agree on every tree, and would agree for a different reason than the one the
comment gives. What this test pins is **agreement at one key value on one tree
shape**, not the mechanism that produces the agreement. The doc comment is
careful about this and says so in its last sentence: the prediction is checked
against the report by `task_grow::allocated`, and *this pins the three spellings
agreeing*. The disagreement case belongs to `allocated`'s own tests, which are
[chapter 10's](10-growing.md#the-prediction-held-to-account).

<a id="the-two-boundaries"></a>
### The two boundaries, and a test that pins less than its neighbour

The next two tests take the sentinel to the two exhaustion boundaries the
library can refuse at — the key space and the ordinal space — and they are not
equally strong.

<!-- fragment «finishing-test-last-key» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1496-1522" parent="finish-tests" -->
````rust
    /// **The two refusals this leaf's own table row predicts, transcribed.**
    /// `materialize-finish` is an `append` at the root level, so it reaches
    /// exactly what `leaf-add` reaches — and from no argument at all, the verb
    /// taking none. Grove predicts `None` for an exhausted keyspace, hands the
    /// library no bytes, and lets it state the condition (clause 3).
    #[test]
    fn a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch(
            &grove_root,
            "01-DONE-impl--old-k4294967295.md",
            "old-k4294967295",
        );

        let error = materialize_finish(guard_at(&wt)).unwrap_err().to_string();

        assert!(error.contains("greatest a key can be"), "got {error}");
        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            2,
            "a refusal writes nothing"
        );
    }

````
<!-- /fragment -->

**What it establishes.** On a tree whose greatest key is already `u32::MAX`, the
sentinel is refused rather than wrapping; the refusal carries the library's own
words; and nothing is written — the directory still holds its two entries.

**What it would pass under.** Any refusal at all whose message contains *greatest
a key can be* would satisfy it, and that string is `ordinal-fs-tree`'s, in its
`plan.rs`. That is precisely what the doc comment claims — *Grove predicts `None`
for an exhausted keyspace, hands the library no bytes, and lets it state the
condition* — so the assertion is checking that grove **did not** write its own
message here, which is a real property and an unusual one to test for.

**And it has a consequence the test cannot show.** Because
`new_finish_leaf(None)` succeeds — building an empty entry — and the `append`
that follows is what refuses, control never reaches the `.context("a finish
sentinel was created without a key")` eight lines further down
`materialize_finish`. That guard is unreachable through this function: by the
time execution passes the append, the append has succeeded, and an append cannot
succeed on the tree whose `next_key` returned `None`. This test looks like the
one that covers the `None` key and is in fact the demonstration that grove's own
handling of it is dead. The measurement below confirms it.

<!-- fragment «finishing-test-last-ordinal» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1523-1539" parent="finish-tests" -->
````rust
    #[test]
    fn a_root_level_at_the_last_ordinal_refuses_the_sentinel_rather_than_wrapping() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch(&grove_root, "4294967295-DONE-impl--last-k1.md", "last-k1");

        assert!(materialize_finish(guard_at(&wt)).is_err());

        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            2,
            "nothing was created"
        );
    }

````
<!-- /fragment -->

**What it establishes.** The same boundary on the other axis: a root level whose
greatest ordinal is `4294967295` refuses the sentinel and creates nothing.

**What it would pass under, and this one is worth flagging.** It asserts
`is_err()` and a directory count, and nothing else — no message, no error kind.
It is the only test in this block that would pass under **any** error whatsoever,
including one raised before the tree was ever read, and including one whose
message told the operator something false. Its neighbour twelve lines above tests
the sibling condition and asserts on the message; this one does not, and the two
sit under one section comment as though they were a pair. They are a pair in
intent and not in strength.


<a id="the-resumable-half"></a>
### Reuse, and the observation count

The fourth `materialize_finish` test is the one that pins resumability, and it
does it with two assertions that have to be read together.

<!-- fragment «finishing-test-reuse» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1540-1562" parent="finish-tests" -->
````rust
    /// The re-selection and the allocation read one snapshot under one exclusive
    /// guard, so nothing can appear between finding no live work and creating the
    /// sentinel — and an existing sentinel is returned rather than duplicated,
    /// which is what makes the driver's finish resumable.
    #[test]
    fn materialize_finish_reuses_an_existing_sentinel_under_one_guard() {
        let (_t, wt) = worktree();
        root_init_at(&wt, "plan").unwrap();
        let grove_root = wt.join(".grove");
        leaf_retire(
            guard(&grove_root),
            &grove_root.join("01-requirements--plan-k1.md"),
        )
        .unwrap();
        let first = materialize_finish(guard_at(&wt)).unwrap();
        crate::task_tree::reset_read_count();

        let second = materialize_finish(guard_at(&wt)).unwrap();

        assert_eq!(first.path, second.path);
        assert_eq!(crate::task_tree::read_count(), 1);
    }

````
<!-- /fragment -->

**What it establishes.** Two properties at once, and it needs both assertions.
The path equality says a second `materialize_finish` returns the *existing*
sentinel rather than appending a second one — which is what makes the driver's
finish resumable. The read count says the second call took exactly **one**
observation of the tree, which is what makes the early return and the append read
one snapshot rather than two.

**What it would pass under.** An implementation that read the snapshot once and
appended a duplicate at a new path fails the first assertion; one that returned
the right path after two reads fails the second. Together they are hard to
satisfy accidentally. But note what the fixture is: the counter is reset *after*
the first call, so both assertions describe the **reuse** path only. Nothing here
observes that the re-selection and the allocation share a snapshot on the
**creating** path — the case the doc comment above `materialize_finish` is
actually written about, where a live leaf appearing between the two would matter.
On the reuse path there is no allocation for a race to open a window in front of.
The claim the comment makes is broader than the claim this test pins, and the
narrower half is the half that is checked.

<a id="the-four-transitions"></a>
### The transition's four tests, two of which never reach it

The block's last four tests exercise `transition_to_current`. Two of them reach
its `match` and two, as the measurement below shows, never do.

<!-- fragment «finishing-test-already-current» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1563-1581" parent="finish-tests" -->
````rust
    #[test]
    fn transition_leaves_a_current_grove_unchanged_and_ready_for_pick() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        let leaf = touch(&grove_root, "01-impl--task-k1.md", "task-k1");
        crate::task_tree::reset_read_count();

        let outcome = transition_to_current(&worktree).unwrap();

        assert_eq!(outcome, CurrentTransition::AlreadyCurrent);
        assert_eq!(crate::task_tree::read_count(), 1);
        assert_eq!(
            crate::task_tree::tests::pick(&grove_root).unwrap(),
            Some(leaf)
        );
    }

````
<!-- /fragment -->

**What it establishes.** A working tree that already holds a readable grove is
answered `AlreadyCurrent`, in one observation, and `pick` afterwards returns the
leaf that was there.

**What it would pass under — and the name promises more than the body delivers.**
The test is called *leaves a current grove unchanged*, and it never checks that
the grove is unchanged. It takes no listing before the call and compares none
afterwards. A transition that wrote a second brief, created a file `pick` skips,
or touched every mtime in the directory would pass all three assertions, because
what is asserted is the returned variant, the observation count, and `pick`'s
answer. Its sibling at the bottom of this block *does* take a listing and compare
it — `assert_eq!(list(&grove_root), before, "a refusal writes nothing")` — so the
technique is present in the same block and simply is not used here. The
*unchanged* in the name is carried by the read count, which is good evidence and
is not the same claim.

<!-- fragment «finishing-test-malformed-name» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1582-1612" parent="finish-tests" -->
````rust
    /// **A tree grove cannot read is not a tree grove scaffolds over.** A
    /// malformed name is an entry — held badly — and appending a first leaf
    /// beside it would hand the operator two problems.
    ///
    /// **What moved is where the operator hears about it.** The transition used
    /// to answer *already current* and leave the next read to state the real
    /// problem, because grove classified the root by hand off its own listing.
    /// The store halts the whole tree on a name the domain refuses, so the
    /// transition itself now carries the domain's own words — one refusal
    /// instead of a success followed by one (principle 2: a message, not
    /// machinery).
    #[test]
    fn transition_does_not_scaffold_over_a_name_grove_refuses() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch(&grove_root, "01-task-k1.md", "task-k1");

        let error = transition_to_current(&worktree).unwrap_err();

        assert!(
            format!("{error:#}").contains("01-task-k1.md"),
            "the refusal must name the file on disk: {error:#}"
        );
        assert!(
            !grove_root.join("01-requirements--plan-k1.md").exists(),
            "a refused name must not be scaffolded past"
        );
    }

````
<!-- /fragment -->

**What it establishes.** A `.grove/` holding a task-*shaped* name that is not a
task name — `01-task-k1.md`, positioned and keyed but with no `--` separating a
kind from a slug — is refused, the refusal names the file on disk, and no first
leaf is scaffolded beside it.

**Where the refusal actually comes from, which is not where the name suggests.**
This test appears among `transition_to_current`'s tests and asserts on
`transition_to_current`'s error, but the function's own `match` is never reached.
A malformed name halts the store's read of the whole tree, so the failure comes
out of `task_tree::write_or_vacancy` on the function's second line — before there
is an `Opening` to match on. The measurement below establishes this rather than
inferring it: replacing *every* arm of this function with a silent panic, one at
a time, leaves this test green in all four runs. The honest description is that
this is **chapter 4's grammar, observed through chapter 14's verb** — and the doc
comment's second half says as much, noting that the transition used to answer
*already current* and leave the next read to state the real problem, until the
store began halting on a name the domain refuses.

**What it would pass under.** Any refusal whose text contains `01-task-k1.md`.
The assertion that carries real weight is the second — that
`01-requirements--plan-k1.md` does not exist afterwards — because that is the
*scaffolds past* failure the test is named for, and it is the one an
implementation could plausibly get wrong.

<!-- fragment «finishing-test-no-grove-entries» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1613-1641" parent="finish-tests" -->
````rust
    /// **A tree grove cannot spell at all stops with a sentence.** The layouts
    /// grove wrote before the current grammar are positioned but unkeyed, so
    /// every one of their names is `Foreign` — invisible to the reader rather
    /// than refused by it. Left at that, an old tree would read as an empty
    /// grove and the driver would materialize a finish sentinel into it. So the
    /// listing having no Grove entry at all is itself the anomaly, and it is
    /// named rather than repaired (principle 2; migration is gone).
    #[test]
    fn transition_refuses_a_root_holding_no_grove_entry_at_all() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "030-ship.md", "030-ship");
        fs::create_dir(grove_root.join("020-spec")).unwrap();
        let before = list(&grove_root);

        let error = transition_to_current(&worktree).unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("holds no Grove entries"), "{message}");
        assert!(message.contains("020-spec"), "{message}");
        assert!(message.contains("030-ship.md"), "{message}");
        assert!(
            message.contains("NN-<kind>--<slug>-k<key>"),
            "the refusal must say what a name should look like: {message}"
        );
        assert_eq!(list(&grove_root), before, "a refusal writes nothing");
    }

````
<!-- /fragment -->

**What it establishes.** A root holding entries that are all *foreign* — an
unkeyed file and an unkeyed directory, the shape grove's own older layouts wore —
is refused with a sentence, and the refusal names both entries, quotes the
grammar a name should have had, and writes nothing.

**This is the only observer of `RootShape::Unrecognised` anywhere in the
workspace**, and the mutation below establishes it as such rather than by
counting names. That makes it the one test standing between this arm and silence,
and it is worth reading the doc comment for the failure it prevents, which is not
the obvious one. Foreign names are *invisible* to the reader rather than refused
by it, so an old tree would read as an **empty** grove — and the driver would
then materialize a finish sentinel into somebody's unmigrated work and report the
grove finished. The anomaly being tested is the listing having no Grove entry
*at all*, and it is named rather than repaired because migration is gone.

**What it would pass under.** The fixture holds *only* foreign entries, so the
test cannot distinguish *no grove entries* from *fewer grove entries than
expected*: a `root_shape` that answered `Unrecognised` whenever any entry was
foreign would pass here and would break every real grove with a stray file in it.
The complementary case — a tree with both foreign and grove entries — is not in
this block. Of the four message assertions, three name strings the arm
interpolates and one names the grammar template it hard-codes; that last one is
the assertion that would notice the message losing its actionable half.

<!-- fragment «finishing-test-dangling-symlink» owner="the-tree-deletes-itself" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1642-1665" parent="finish-tests" -->
````rust
    #[cfg(unix)]
    #[test]
    fn transition_does_not_classify_a_dangling_grove_symlink_as_absent() {
        use std::os::unix::fs::symlink;

        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        symlink(worktree.join("missing-grove"), &grove_root).unwrap();

        let error = transition_to_current(&worktree).unwrap_err();

        // The store's own sentence, not grove's *not found*: something is at
        // the root, and `is_dir` reading a dangling link as absent is exactly
        // the mistake `task_tree::restate` orders its clauses to avoid.
        assert!(
            format!("{error:#}").contains("a tree is a directory"),
            "unexpected error: {error:#}"
        );
        assert!(fs::symlink_metadata(grove_root)
            .unwrap()
            .file_type()
            .is_symlink());
    }

````
<!-- /fragment -->

**What it establishes.** A `.grove` that is a symlink pointing at nothing is not
classified as *absent*. The refusal is the store's own sentence — *a tree is a
directory* — and the symlink is still a symlink afterwards, so nothing followed
or replaced it.

**Where it refuses, and why the distinction is the point.** Like the malformed
name above, this never reaches `transition_to_current`'s `match`; the same four
mutations leave it green. The failure it guards against is a classification bug
one layer down, and its own comment names it exactly: `is_dir` follows links and
reports `false` for a dangling one, which is indistinguishable from *not there* —
and *not there* is the answer that would make the transition **scaffold a fresh
grove over it**. The ordering of `task_tree::restate`'s clauses is what avoids
that, and it is [chapter 5's](05-opening.md#one-spelling-of-the-root).

**What it would pass under.** Any error containing *a tree is a directory*. The
second assertion is the load-bearing one: it re-`symlink_metadata`s the path and
requires it still be a symlink, which is what rules out a transition that
"helpfully" replaced the dangling link with a real directory.

**It is also the only `#[cfg(unix)]` in the book's entire corpus.** Thirteen
roots and 10,533 lines carry exactly one platform-guarded item, and it is this
test — because a dangling symlink is the one fixture in the crate that cannot be
built portably. The block's other seven tests are platform-neutral.


<a id="what-the-refusals-are-worth"></a>
## What the refusals are worth, measured

This chapter's production carries fifteen refusal and fallback arms, and reading
them says nothing about whether any test would notice if one stopped working. So
the arms were measured rather than read. The procedure is the one this part of
the book has used throughout: copy the workspace to a scratch directory, build
the `grove` binary first so `CARGO_BIN_EXE_grove` is set, run
`cargo test --no-fail-fast -p grove-loop -p grove-llm` to get a control, then
replace one arm at a time with a panic that **says nothing** and read each mutant
as the difference against that control.

Three details of that procedure are what make the readings mean anything, and
each was established the hard way by an earlier chapter of this part.

- **The control is not zero.** The scratch copy is not a jj repository, so eleven
  tests — all of `crates/grove-loop/tests/prompt.rs` — fail before any mutation.
  558 tests run in all, and a mutant is the `comm` difference against those
  eleven.
- **The panic must carry no message.** `bail!(…)` → `panic!(…)` preserves the
  format string, and every observer of the three teardown functions is an
  out-of-process test asserting on *stderr substrings* — which cannot tell a
  panic from a refusal when both print the same words and exit non-zero. Each
  mutation below replaces the **whole macro call** with `panic!("MUTANT")`.
- **A mutant that does not compile reads exactly like a clean one**, since it
  prints no per-test lines at all. Every mutant below ran 558 tests, matching the
  control, which is what rules that out.

Fifteen arms, fifteen mutants, one control.

| # | Arm | Newly failing | Where the observer lives |
|---|---|---:|---|
| 1 | `transition_to_current` — `Opening::Vacancy` scaffolds | 1 | `transition_initializes_an_absent_grove_under_one_exclusive_guard` (1,335 — **ch. 11**) |
| 2 | `transition_to_current` — `RootShape::ATree` | 2 | `one_process_creating_and_reading_a_grove_never_waits_on_itself` (1,385 — **ch. 11**); `transition_leaves_a_current_grove_unchanged_and_ready_for_pick` (1,564 — ch. 14) |
| 3 | `transition_to_current` — `RootShape::Taskless` | 1 | `a_taskless_root_is_refused_with_advice_rather_than_completed` (1,447 — **ch. 11**) |
| 4 | `transition_to_current` — `RootShape::Unrecognised` | 1 | `transition_refuses_a_root_holding_no_grove_entry_at_all` (1,621 — ch. 14) |
| 5 | `materialize_finish` — sentinel created without a key | **0** | — |
| 6 | `finish_slug` — `"finish"` is not a valid slug | **0** | — |
| 7 | `finish_commit` — no `.grove` at all | 1 | `finish_commit_on_an_absent_tree_names_the_operation_log` (`grove-llm/tests/finish_commit.rs`) |
| 8 | `finish_commit` — `symlink_metadata` failed for another reason | **0** | — |
| 9 | `finish_commit` — the root is not a directory | 1 | `finish_commit_refuses_a_symlinked_task_root_before_deleting_anything` (same file) |
| 10 | `finish_commit` — no live leaf remains | **0** | — |
| 11 | `finish_commit` — live work remains | 1 | `finish_commit_refuses_when_ordinary_work_appeared` (same file) |
| 12 | `finish_commit` — the handle is not the live finish leaf | 3 | `finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf` (in **both** `grove-llm/tests/finish_commit.rs` and `grove-loop/tests/verbs.rs`); `a_refused_handle_is_quoted_as_the_operator_wrote_it` |
| 13 | `delete_and_commit` — the store's `delete` failed | **0** | — |
| 14 | `delete_and_commit` — the commit failed | **0** | — |
| 15 | `require_recoverable_grove` — jj tracks nothing under the root | 1 | `finish_commit_refuses_an_untracked_task_tree_naming_how_to_track_it` (same file) |

**Nine arms are held and six are not**, and three things in that table matter
more than the ratio.

**First: the evidence for this chapter's own transition is split across the
chapter boundary, and this is where the file's inversion is finally paid for.**
Four of the eight arms with observers belong to `transition_to_current`, and of
the five tests holding them, **three sit in chapter 11's block and two in this
one**. `Taskless` is pinned only from chapter 11; `Unrecognised` only from here;
`ATree` from both sides; and the `Vacancy` arm — the arm that creates a grove,
which is as much this chapter's production as anything on the page — is held
**only** from chapter 11. Neither chapter can state this function's coverage
from its own block. Chapter 11 carries the table for the arms it can see and this
one carries the arms it can; the whole picture exists only across the two.

**Second: every observer of the ending is out of process.** Arms 7, 9, 11, 12 and
15 are the five held arms of `finish_commit`, `delete_and_commit` and
`require_recoverable_grove`, and not one of their observers is an inline test.
They live in `crates/grove-llm/tests/finish_commit.rs`, which runs the binary and
reads its stderr. That is why the message-preserving panic is not a mutation
here: a `bail!` turned into a `panic!` with the same format string prints the same
sentence and exits non-zero, and every one of these five tests would have stayed
green — recording the arms as unheld. They are held, and the silent panic is what
shows it.

<a id="the-six-that-hold-nothing"></a>
### The six unheld arms are three different things

A zero in that table means *no test distinguishes this arm*. It does not mean *no
test could*, and the six split cleanly by why.

**Two are unreachable, and one of them is unreachable by design.**

Arm 5 — `materialize_finish`'s `.context("a finish sentinel was created without a
key")` — cannot fire. The only way `key` is `None` is an exhausted keyspace; but
`new_finish_leaf(None)` succeeds and builds an empty entry, and the `append` that
follows is refused by the library, so the function returns before the `.context`
is evaluated. `a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping`
exercises exactly that path and asserts on the **library's** wording, which is
the demonstration that grove's own message for the condition is dead. Deleting
the `.context` would change no test's result and no operator's experience.

Arm 6 — `finish_slug`'s `map_err` — cannot fire either, and the doc comment
argues that this is the right code anyway. `Slug::new("finish")` is a literal
that satisfies every clause of `refuse_token`. The error arm exists so that
`Slug`'s invariant is a property of every `Slug` in the process rather than of
most of them, and the price of that is one unreachable line. This is the clearest
case in the chapter of a zero that is not a gap.

**Three require an environment failure no fixture builds.** Arm 8 is
`symlink_metadata` failing for a reason other than absence — a permissions
failure on the containing directory, say. Arm 13 is the store's `delete`
failing. Arm 14 is `jj` refusing the commit *after* the deletion has already
happened. All three are genuinely reachable and all three need the filesystem or
the version control system to fail on demand, which nothing in this workspace can
arrange. Arm 14 is the one worth naming: its message is the only place in the
crate that tells an operator the tree is **already deleted in the working copy**,
and it is the message most likely to be read in a genuine emergency and least
likely to have been read by anyone before.

**And one is an ordinary operator-facing refusal with no test at all.** Arm 10 —
`finish_commit` finding no live leaf, and answering *the requested finish leaf is
no longer live* — needs nothing but a tree whose leaves are all terminal and
which holds no finish leaf. That is one `touch` and one `leaf_retire` away from
the fixtures already in this file. Its neighbours in the same function are each
held by a test in `finish_commit.rs`: the absent root, the non-directory root,
the live work, the mismatched handle and the untracked tree. **The asymmetry is
the finding, not the absence** — five of `finish_commit`'s six operator-facing
refusals have a test, the sixth is reached by the same fixture shape as the
others, and nothing explains the difference. A missing test is worth stating that
way rather than as a bare gap, because *this line is untested* is true of a great
deal of defensive code and *this one line of six is untested for no reason* is
not.

None of the newly-failing sets above names a `driver_lease.rs` or `prompt.rs`
test, so none of these readings is a wedged-producer timeout wearing the costume
of an observer, and none needed a re-run on that ground.


<a id="what-could-not-move"></a>
## What could not move

The book's question is what a layer keeps when a domain-free library is extracted
from underneath it, and this chapter answers it with the two operations that
bracket a grove's whole existence.

**On the way in — the names.** The finish leaf is a name the store would have
written without complaint. `Parts` can express a `finish` kind, `append` would
place it, and `ordinal-fs-tree` has no opinion whatever about which kinds exist —
it has an entry, an ordinal and a key, and no word for a *session kind*. So the
rule that only the driver may write this leaf cannot live in the store, and it
does not: it lives as `task_grow::refuse_finish_kind` on grove's side, with this
chapter's `materialize_finish` as its single stated exemption. The cost is that
the rule and its exception are two pieces of code that have to know about each
other, and the comment at line 118 is the crate paying it — *this verb is the
deliberate exception rather than a hole in the check*.

**On the way through — the preconditions, and against which snapshot.** Both
halves of this chapter are the same discipline applied twice. The transition
takes one opening and derives four answers from it, because the two-phase version
deadlocked against its own lock. The teardown opens exclusively, revalidates
three facts, and hands the guard on to the deletion without releasing it, so
nothing observes `.grove/` between the check and the unlink. And
`materialize_finish` re-selects under its own exclusive guard rather than
trusting the shared read that sent it there — the check runs against the same
snapshot the operation then plans from, which is the whole of what
*question 2* asks of a layer. A refused run consumes nothing: three of this
chapter's eight tests assert on a directory listing or a file count after a
refusal, and every refusal in the block writes nothing before it returns.

**On the way out — the policy.** This is the chapter where grove's chosen values
are least defensible from anything beneath it, and most clearly stated.
`DEFAULT_ROOT_SLUG` is `"plan"` because a driver that must name a grove has
nobody to ask. `requirements` is the fixed first kind because a brand-new grove's
first session has nothing on disk but a human's own words. The finish leaf's body
is eleven lines of `format!` that tell an operator to do three things in an
order. None of those could have been defaulted by a store, a runner or a version
control system, and each is stated where a reader can find it rather than
computed.

**And the chapter's own answer to *what stayed here* is the sharpest in the
crate, because it is mostly a list of what left.** The teardown once had a
witness, a manifest, an evacuation, a rollback proof, a quarantine and a
recovery path — a transaction, hand-built, sitting on top of a version control
system whose operation log already was one. `delete-finish-transaction-k8`
deleted all of it, and what remains is three facts nothing else can state: that
the live leaf is the finish leaf the caller named, that no ordinary work slipped
in, and that only `.grove/` is deleted and committed. The refusal wording is
grove's and the remedy is jj's. That division is the crate's whole method in one
function, and it is why this chapter's last item is a ten-line doc comment
insisting that a four-line function *promises nothing and repairs nothing*.

The grove is gone. Everything it knew that was worth keeping was promoted out of
its briefs before step two, which is the finish session's first instruction and
the reason the tree can be deleted rather than archived: a task tree that has
been read out is a scaffold, and grove's last act is to take the scaffold down.

[Previous: Outcomes are marked in place](13-outcomes.md) | [Contents](README.md) | [Next: The twelve verbs, and the two that are not](15-the-verbs.md)
