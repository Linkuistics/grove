# Opening, contention and refusal
<!-- book-page id="opening" slice="one-spelling-of-grove" order="5" -->
[Previous: The name, and canonicity](04-the-name.md) | [Contents](README.md) | [Next: Paths, and addressing](06-paths.md)

<a id="one-spelling-of-grove"></a>
## The rule: a guard is proof the tree was there when it was opened

Chapters 2 to 4 read a grammar that judges one name at a time. It has no notion
of a directory, a parent or an order — `parse` takes a string and a `Found` and
nothing else — so nothing so far can *find* anything. This chapter opens the tree
those names live in, and the rule it exists for is a limit rather than a
capability:

> A guard is proof the tree was there when it was opened, and no more than that.

That is weaker than it looks, and the crate says so about itself. `lib.rs`'s own
header, read in chapter 1, splits the claim in two: `verbs::root_init` consumes a
`Vacancy`, so it **cannot** run over a live grove, and that half is absolute
because the vacancy is consumed by the type system. The other half — that holding
a `Tree` means there is a tree — is weaker, and this chapter is where the
weakness becomes concrete: the guard proves an acquisition that already happened,
at a moment that has passed.

The carried example reaches its fifth step here. A caller has a worktree; grove
joins `.grove` to it, takes one lock, and answers with a `Tree` — or with a
refusal that names what is absent.

```text
<worktree>
  │
  ├─ join ".grove"            (`lib.rs::grove_root`, for the public openings)
  │
  ├─ announce_contention      probe flock(2) non-blocking, in the caller's mode
  │    ├─ acquired            release; say nothing
  │    └─ EWOULDBLOCK/EAGAIN  "waiting for active Grove tree operation"
  │
  ├─ ordinal_fs_tree::fs::read::<TaskName>(<worktree>/.grove)   ← blocks here
  │
  ├─ Ok(Reading::Tree)        ⇒ Vacant::Tree(tree)    ── read ⇒ Ok(tree)
  ├─ Ok(Reading::Vacant)      ⇒ Vacant::Nothing       ── read ⇒ Err(absent_tree)
  │                             (an answer at this level; the refusal is read's)
  └─ Err(error)               ⇒ restate(root, &error)
                                 ├─ RootIsNotATree  → the store's sentence
                                 ├─ not a directory → "grove root not found: …"
                                 ├─ Malformed/Reserved → the store's sentence + path
                                 └─ otherwise       → the store's sentence
```

Read downwards, the figure is the chapter. Two things in it are grove's and the
rest is the store's: the probe on the way in, which buys a diagnostic the
library's interface cannot express, and the precedence on the way out, which
chooses *which* condition an operator is told about first. Everything between
them — the lock, the walk, the decision to refuse — moved out to
`ordinal-fs-tree` and did not come back.

**What did not move is narrower than the module is.** The header names it
exactly: the semantics are unchanged and *what changes is who owns the walk*. So
this chapter's answer to the book's first question is not *the grammar* — that
was chapters 2 to 4 — but two smaller things that a store with no domain could
not have supplied: **path construction**, because the library returns no paths,
and **refusal precedence**, because an absent root is a condition grove states in
its own words.

This chapter owns 303 lines of `task_tree.rs` in 1 block.
The source index records their current ranges; the fragments below reconstruct
every owned byte.

<a id="one-spelling-of-the-root"></a>
## The header, and the one place paths are built

The composite below is this chapter's whole ownership block. It expands, in
order, to lines 1 through 303 of the file, and the seventeen fragments it names
run from here to the end of the chapter.

<!-- fragment «tree-opening» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="1-303" parent="source-task-tree" -->
<!-- insert «tree-header-who-owns-the-walk» -->
<!-- insert «tree-header-paths-here» -->
<!-- insert «tree-header-no-canonicalising» -->
<!-- insert «tree-header-refusal-precedence» -->
<!-- insert «tree-imports» -->
<!-- insert «tree-alias-and-read-count» -->
<!-- insert «tree-vacant-and-read-or-vacant» -->
<!-- insert «tree-guard-opening-vacancy» -->
<!-- insert «tree-read» -->
<!-- insert «tree-write» -->
<!-- insert «tree-write-or-vacancy» -->
<!-- insert «tree-reopen-write» -->
<!-- insert «tree-open-write» -->
<!-- insert «tree-absent-tree» -->
<!-- insert «tree-raised» -->
<!-- insert «tree-announce-contention» -->
<!-- insert «tree-restate» -->
<!-- /fragment -->

The header opens on the migration rather than on the module, which is why the
first thing a reader meets here is a claim about ownership rather than about
types.

<!-- fragment «tree-header-who-owns-the-walk» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="1-13" parent="tree-opening" -->
````rust
// Grove's **reading surface**, expressed through `ordinal-fs-tree` (gh issue
// #13, increment 2, the first leaf of the *migrate* stage).
//
// `pick`, `select`, `brief-chain`, `kind` and `resolve` all read one
// [`Snapshot`](ordinal_fs_tree::Snapshot) taken under the library's shared lock,
// and their semantics are unchanged: first live leaf in walk order, ancestor
// briefs root→leaf, reference-by-permanent-key. What changes is who owns the
// walk. The path-walking reader that used to own it survived only as long as
// there were verbs still under grove's own exclusive guard — that guard and the
// library's cannot be nested (both `flock` the directory containing the tree
// root, and two open file descriptions on one directory do not share a lock) —
// and `sweep-k37` deleted it once the last of them had moved across.
//
````
<!-- /fragment -->

The parenthesis about nesting is the reason there was a deletion rather than a
layer. Grove used to hold a guard of its own beside the library's, and the two
`flock` the same directory through different open file descriptions — which do
not share a lock — so a verb holding grove's guard and calling into the library
blocked on itself. Phases were the workaround and a torn tree was the price.

Both halves of that are held by tests, and neither of them is in this crate.
`the_librarys_tree_lock_is_taken_from_exactly_one_module` and
`no_production_lock_grove_takes_for_itself_ever_blocks` live in
`crates/grove-llm/tests/tree_lock.rs`, because they scan every `.rs` file grove
ships and so belong to the package that ships them all. The first asserts that
the non-comment mentions of `ordinal_fs_tree::fs` across that scan are exactly
six, all in `crates/grove-loop/src/task_tree.rs` — **and all six are in this
chapter's block**: the import, the two guard aliases, the blocking shared and
exclusive acquisitions, and the quiet observer acquisition. The count is the control, so a
rename that hid the call sites fails the test rather than passing it clean. The
second cuts each file at its inline `mod tests`, skips the releases, and reports
any remaining `libc::flock` acquisition that does not carry `LOCK_NB`. Its
control is that it reached an acquisition in at least **two files** — grove's
two lockers being two files, and a bare count of matches unable to tell one of
them being hidden from the other having grown a lock. It could not tell before
`lock-scan-blind-to-contention-probe-k190`: the cut was taken at each file's
first `#[cfg(test)]`, which in this file is the test-only `READ_COUNT`
thread-local at line 60 so the probe below was never read and the lease's four
matches cleared the old floor alone.

The second passage is the one an early-use row is anchored on, and it is the
chapter's answer to *what did not move* on the way in.

<!-- fragment «tree-header-paths-here» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="14-23" parent="tree-opening" -->
````rust
// # Path construction lives here, and in exactly one place
//
// The library's reading surface returns **no paths** — `cli-k16` refused to add
// a `path()` to the algebra and said why (`docs/ordinal-fs-tree/CLI.md`, *What
// `cli-k16` should watch*) — so a consumer builds them: the caller's own
// spelling of the root, then each ancestor node's rendered name, then the
// entry's. [`entry_path`] is that one place, and every later flip leaf uses it.
// It is safe without a check because every name a snapshot admits has already
// been checked to render as one path component (`Error::NameIsNotOneComponent`).
//
````
<!-- /fragment -->

`entry_path` is named here and read in chapter 6. The minimum this page owes for
it is the sentence the header already makes: it is the one place an entry's
absolute path is built, because the library's reading surface returns none, and
it is safe without a check because every name a snapshot admits has already been
checked to render as one path component. What chapter 6 adds is the function, and
the `Target` machinery around it that decides which entry a path is being built
*for*.

The composition the header describes — the caller's own spelling of the root,
then each ancestor node's rendered name, then the entry's — is why the second
clause holds. Every component but the first comes from a `TaskName` the snapshot
admitted, and chapter 4's canonicity check is what makes admitting one
equivalent to having rendered it.

The third passage is the reason the first clause of that composition says *the
caller's own spelling*.

<!-- fragment «tree-header-no-canonicalising» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="24-30" parent="tree-opening" -->
````rust
// Nothing here canonicalises for **output**. The library deliberately never
// does: on macOS `/var` and `/private/var` name the same inode, so
// canonicalising would make the mere presence of a lock rewrite every path a
// read verb returns. Canonicalisation happens only where a caller's path is
// resolved to an entry — in [`target`] and in [`leaf_entry`], nowhere else
// here — and only to *compare*, as the path-walking reader did too.
//
````
<!-- /fragment -->

This is the chapter's most consequential small decision, and it concerns a
platform. On macOS `/var` is a symbolic link to `/private/var`, so the two spell
one inode; a reader that canonicalised for output would answer `/private/var/…`
to a caller that asked about `/var/…`, and it would do so *only when a lock had
been taken*, because that is when a canonicalising reader would have resolved the
path. The user-visible effect is that the mere presence of a lock rewrites every
path grove prints. Chapter 6 carries the full account beside `entry_path`.

`target` and `leaf_entry` are both named here and neither is this chapter's:
chapter 6 reads the first and chapter 8 the second. The minimum is the exception
the passage states — canonicalisation happens where a caller's path is resolved
to an entry and nowhere else in this module, and there only to **compare** a
caller's spelling against the tree's, never to produce a path grove hands back.
Comparing is not the same operation as normalising for output, and the
difference is exactly that a comparison's result is a boolean the caller never
sees the two sides of.

**The passage named only one of the two until this book read the other.** It
used to say *Canonicalisation appears once, in `leaf_entry`* — a uniqueness
claim written from the shape of the design rather than from an enumeration of
the code, and refuted from inside the same file, since `target`'s own doc
comment says it canonicalises *exactly as `leaf_entry` does*. Chapter 6 owns
that counterexample and adjudicated the claim beside the fragment that
reproduces it; `canonicalisation-sites-k149` then widened the clause to both
functions. What did not change is the operation the second half named, which was
always true of both — compare, and never report — nor the appeal to the
path-walking reader that did the same. Its noun did change: the old clause
compared *a caller's spelling of a leaf*, and `target` resolves a node directory
or the grove root as readily as a leaf, so the corrected clause says *a caller's
path*.

The fourth passage is the chapter's other answer to *what did not move*, on the
way out, and the whole of `restate`'s charter.

<!-- fragment «tree-header-refusal-precedence» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="31-40" parent="tree-opening" -->
````rust
// # Refusal precedence is grove's, and the halt is the library's
//
// The library halts the whole tree on a name grove recognises and refuses,
// wherever it sits. That is the decision, and it is taken under the lock. But
// the library can only say *this filename is wrong*, and an absent root is a
// condition grove states in its own words. So [`restate`] re-states a *failed*
// read in the order grove owes its operator: root, then the library's own
// message. Only the wording is chosen here; the refusal itself already
// happened.

````
<!-- /fragment -->

The split this passage names — decision under the lock, wording chosen after it —
is what lets the last function in the chapter run its checks **unlocked** without
that being a race. It is also why the split exists at all: a root is grove's
concept, so the library has no sentence for its absence, while the library's own
refusals already carry advice grove would only make worse by re-wording. The last
section reads the function that discharges it.

The imports are the seam in one screen, and they carry a name collision worth
noticing before it confuses a reader two chapters from now.

<!-- fragment «tree-imports» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="41-52" parent="tree-opening" -->
````rust
use std::collections::HashMap;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use ordinal_fs_tree::fs::{Reading, TryReading, Vacancy, Writing};
use ordinal_fs_tree::{Entry, EntryName, Error, Found, Key, Snapshot, Sought, Verdict};

use crate::task_name::{self, Handle, Kind, Outcome, Parts, TaskName};
use crate::Reference;

````
<!-- /fragment -->

`Reading`, `Writing` and `Vacancy` here are the **store's**, from
`ordinal_fs_tree::fs`. Chapter 1 read enums called `Reading` and `Writing` in
`lib.rs`, with variants called `Tree` and `Vacant` and `Tree` and `Vacancy`
respectively — and those are grove's own types, not these. The two pairs share
their names and most of their variant names and are unrelated as types; `lib.rs`
maps one onto the other in the bodies of `read` and `write` that chapter 1
reproduced. Inside this module the names always mean the store's, which is also
why the aliases below rename one of them: `Opening` exists so that `Writing` can
go on meaning what the import says it means.

<a id="the-four-openings"></a>
## Four openings, and the two acquisitions under them

The first alias is the read side, and the note about `root` is a genuine
ambiguity rather than a caution.

<!-- fragment «tree-alias-and-read-count» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="53-65" parent="tree-opening" -->
````rust
/// The task tree, read once under the library's shared lock.
///
/// Derefs to its `Snapshot`, so the reading operations are called on it
/// directly. Note that `Tree::root` is the **path** the caller spelled, while
/// `Snapshot::root` is the root *level* — the inherent method wins, and both are
/// wanted here.
pub(crate) type Tree = ordinal_fs_tree::fs::ReadGuard<TaskName>;

#[cfg(test)]
thread_local! {
    static READ_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

````
<!-- /fragment -->

The deref is what creates the ambiguity the comment then declines to remove: two
different `root`s are in scope at once, the inherent one wins, and the comment
says **both are wanted** rather than warning about the shadowing. That is not
carelessness. The caller's own spelling is exactly what the header's path
composition starts from, so a `Tree` that hid it behind the snapshot's root level
would break `entry_path` two chapters later.

`READ_COUNT` is test-only bookkeeping, and it is worth following because it is
the thing declared in this chapter that reaches furthest past it. It is declared
here and incremented in `read_or_vacant` and `open_write`, so it measures
blocking acquisition calls rather than verbs run. The observer does not enter
this counter. Its accessors, `reset_read_count` and `read_count`, sit in
the block chapter 9 owns, and **nine assertions read them**: one in chapter 7's
`pick-tests`, six in `tree_lifecycle.rs` spread across the blocks chapters 11, 12,
13 and 14 own, and two in the `task_grow` test file chapter 10 can only cite by
name. The nearest is chapter 7's
`select_returns_path_handle_and_kind_from_one_guarded_observation`, which asserts
`read_count()` is `1` so that selection cannot reopen the tree to derive the
launch facts it returns. What those nine have in common is the property this
declaration exists to instrument, and it is crate-wide rather than local: **one
verb, one lock**.

The read-side opening comes next, and its vacancy enum serves the blocking
facade. The observer's `try_read` goes straight to the store's nonblocking
acquisition, translating Busy and ready Tree/Vacant into the public facade's
types. It skips `announce_contention` entirely: a terminal viewer supplies its
own waiting indicator and retry deadline. A successful observer still holds the
real shared descriptor, so its copied names and file bytes have the same guard
boundary as the blocking reader.

<!-- fragment «tree-vacant-and-read-or-vacant» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="66-107" parent="tree-opening" -->
````rust
/// The whole of what a shared opening found: the tree, or the fact that there is
/// none — under the lock either way.
///
/// The read-side twin of [`Opening`], and named here for the same reason: the
/// crate root's [`crate::read`] is the one caller that has to distinguish the
/// two arms, and it does so without reaching for the library's `fs` module
/// itself.
pub(crate) enum Vacant {
    Tree(Tree),
    Nothing,
}

/// Read the task tree under a shared lock, **or** find that there is none.
///
/// An absent root is an answer here rather than the refusal [`read`] states,
/// because `grove` asks this of a fresh checkout on every iteration.
pub(crate) fn read_or_vacant(grove_root: &Path) -> Result<Vacant> {
    #[cfg(test)]
    READ_COUNT.with(|count| count.set(count.get() + 1));

    announce_contention(grove_root, libc::LOCK_SH);
    match ordinal_fs_tree::fs::read::<TaskName>(grove_root)
        .map_err(|error| restate(grove_root, &error))?
    {
        Reading::Tree(tree) => Ok(Vacant::Tree(tree)),
        Reading::Vacant => Ok(Vacant::Nothing),
    }
}

/// Quiet observer acquisition: no diagnostic probe and no blocking fallback.
pub(crate) fn try_read(grove_root: &Path) -> Result<crate::TryReading> {
    match ordinal_fs_tree::fs::try_read::<TaskName>(grove_root)
        .map_err(|error| restate(grove_root, &error))?
    {
        TryReading::Busy => Ok(crate::TryReading::Busy),
        TryReading::Ready(Reading::Tree(tree)) => {
            Ok(crate::TryReading::Ready(crate::Reading::Tree(tree)))
        }
        TryReading::Ready(Reading::Vacant) => Ok(crate::TryReading::Ready(crate::Reading::Vacant)),
    }
}

````
<!-- /fragment -->

`Vacant` is named in this module rather than matched on the store's own type
because `crate::read` — the crate's public entry point, in `lib.rs` — is the one
caller that has to tell the two arms apart, and the module boundary the
five-call-site test holds is what stops it reaching for `ordinal_fs_tree::fs`
itself to do so. The reason the vacancy is an *answer* here rather than the
refusal `read` states is in the doc comment and is operational: `grove` asks this
of a fresh checkout on every iteration, so a driver that treated an empty root as
an error would fail on every worktree that has not been scaffolded yet.

Note the order inside the body. The probe runs first, then the acquisition, and
the acquisition's error goes through `restate` before it leaves. That order is
the figure at the top of this chapter, and it is the same in both write-side
entry points.

The write side takes three aliases rather than one, and each is named here for a
different caller.

<!-- fragment «tree-guard-opening-vacancy» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="108-136" parent="tree-opening" -->
````rust
/// The task tree, read once under the library's **exclusive** lock — the
/// surface every mutation is on.
///
/// A mutating method *consumes* this guard (`crates/ordinal-fs-tree/src/fs/mod.rs`,
/// *A mutation consumes its guard*), so one guard is one operation. That is the
/// whole of what a bulk verb has to work around; `tree_lifecycle::leaf_prune`
/// carries the consequence and `docs/adr/bulk-marks-are-not-atomic.md` records
/// what Grove chose to do about it.
pub(crate) type Guard = ordinal_fs_tree::fs::WriteGuard<TaskName>;

/// The whole of what an exclusive opening found: the tree, or the **vacancy**
/// a tree may be created in — under the lock either way.
///
/// Named here, and only here, so that the one verb group that needs the vacancy
/// arm — the grove's own creation, in [`crate::tree_lifecycle`] — can match on
/// it without reaching for the library's `fs` module itself. That is the same
/// rule the guard aliases above follow, and
/// `the_librarys_tree_lock_is_taken_from_exactly_one_module` is what holds it.
pub(crate) type Opening = Writing<TaskName>;

/// The exclusive lock over a root that holds no tree, and the affordance to
/// create one under it.
///
/// [`Vacancy::initialize`] is the whole of grove's tree creation since
/// `collapse-tree-access-k13`: the root, its charter and the first leaf are one
/// store operation under one lock, where they used to be two phases either side
/// of a lock grove could not nest inside the library's.
pub(crate) type TreeVacancy = Vacancy<TaskName>;

````
<!-- /fragment -->

`Guard` carries the store's rule that a mutation **consumes** its guard, so one
guard is one operation. That is a constraint on bulk verbs rather than on single
ones: a verb that marks *N* entries needs *N* guards, which means *N* separate
acquisitions and no atomicity across them. `tree_lifecycle::leaf_prune` is where
that lands, and chapter 13 reads it; the minimum here is that pruning a node
marks each entry under a guard of its own, so a run interrupted part way leaves
some entries marked and some not. `docs/adr/bulk-marks-are-not-atomic.md` records
what grove chose to do about it, and is named rather than linked, as every record
in this book is.

`Opening` and `TreeVacancy` exist for the same structural reason `Vacant` does,
and the comment states the rule outright: the one verb group that needs the
vacancy arm is the grove's own creation, in `tree_lifecycle`, and naming the type
here is what keeps that group from importing the library's `fs` module. The
comment names the test that holds it, which is the five-call-site scan above.

`TreeVacancy`'s own comment reaches furthest forward of any in the block:
`Vacancy::initialize`
is the whole of grove's tree creation since `collapse-tree-access-k13`, so the
root, its charter and the first leaf are one store operation under one lock
where they used to be two phases either side of a lock grove could not nest.
Chapter 11 is where that becomes a verb.

Then the four openings themselves, which are a square rather than a list. The
axes are the lock mode and what the absence of a tree means — an answer, or a
refusal.

|  | Answers the vacancy | Refuses an absent tree |
|---|---|---|
| **Shared** | `read_or_vacant` | `read` |
| **Exclusive** | `write_or_vacancy` | `write` |

`read` is the shared refusing arm, and it is a wrapper rather than an
acquisition.

<!-- fragment «tree-read» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="137-155" parent="tree-opening" -->
````rust
/// Read the task tree under a shared lock, refusing a root that holds no tree.
///
/// One lock and one snapshot, and a reading verb takes exactly one of these.
/// The shared/exclusive split is the whole of what the two entry points differ
/// by: both keep the tree quiescent against **writers**, and only this one lets
/// other readers through. So a verb that reads takes this, and takes it even
/// when it is reporting on a mutation the same command just made — see
/// [`crate::verbs::stale_cross_refs`], which is the one such caller.
///
/// [`read_or_vacant`] is the same acquisition with the vacancy left as an
/// answer; **this** entry point is for verbs that act on a tree that must
/// already be there, exactly as [`write`] is on the other side.
pub(crate) fn read(grove_root: &Path) -> Result<Tree> {
    match read_or_vacant(grove_root)? {
        Vacant::Tree(tree) => Ok(tree),
        Vacant::Nothing => Err(absent_tree(grove_root)),
    }
}

````
<!-- /fragment -->

The consequence worth carrying is the one the comment draws about
`verbs::stale_cross_refs`: a verb that *reads* takes a shared guard even when it
is reporting on a mutation the same command just made. Chapter 15 reads that
function; here it is the evidence that the read/write split visible in a
signature is about the lock and not about what the command is doing — which is
what `lib.rs`'s *the lock a verb needs is visible in its signature* rests on, and
the half of it chapter 1 could only state.

Note what `read` does *not* do: it takes no lock of its own and prints no
diagnostic, because `read_or_vacant` already did both. The refusal it adds is
purely a re-reading of the same result.

`write` is the exclusive refusing arm, and unlike `read` it announces in its own
body.

<!-- fragment «tree-write» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="156-168" parent="tree-opening" -->
````rust
/// Read the task tree under an exclusive lock, announcing contention first.
///
/// The write-side twin of [`read`]. A root that holds no tree is the error
/// [`absent_tree`] states: **this** entry point is for verbs that act on a tree
/// that must already be there, and a verb that made one on the way past would
/// turn a mistyped root into a second workstream. Creating one is
/// [`write_or_vacancy`]'s, and `grove new` and the driver's own scaffold are its
/// only callers.
pub(crate) fn write(grove_root: &Path) -> Result<Guard> {
    announce_contention(grove_root, libc::LOCK_EX);
    reopen_write(grove_root)
}

````
<!-- /fragment -->

The doc comment's second half is a design claim rather than a description: a verb
that created a tree on the way past would turn a mistyped root into a second
workstream. That is the failure the split exists to prevent, and it is prevented
by the *type* rather than by a check, because a caller holding a `Guard` never
had a vacancy to create anything in.

<!-- fragment «tree-write-or-vacancy» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="169-177" parent="tree-opening" -->
````rust
/// The exclusive opening **with** its vacancy arm, announcing contention first.
///
/// The one opening that can answer *there is no tree here* without refusing, so
/// the caller can create one under the lock it is handed.
pub(crate) fn write_or_vacancy(grove_root: &Path) -> Result<Opening> {
    announce_contention(grove_root, libc::LOCK_EX);
    open_write(grove_root)
}

````
<!-- /fragment -->

The comment's *one opening that can answer there is no tree here without
refusing* is true of the exclusive side rather than of the file. `read_or_vacant`
answers the same thing without refusing, and its own comment says so in as many
words; what is unique here is the rest of the sentence. This is the only opening
that hands its caller the **affordance to create**, and *under the lock it is
handed* is why that matters — the vacancy carries the exclusive lock, so a caller
creating a tree in it is not racing a second creator.

The fifth `pub(crate)` function is not a fifth opening. It is `write` with the
diagnostic removed.

<!-- fragment «tree-reopen-write» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="178-191" parent="tree-opening" -->
````rust
/// [`write`] without the waiting diagnostic, for the second and later guards of
/// one verb.
///
/// A bulk mark is *N* rewrites under *N* guards, and every one of them would
/// otherwise probe and print. The verb announces once, through [`write`], and
/// takes the rest through here: the diagnostic is about the command's wait, not
/// about each lock it happens to need.
pub(crate) fn reopen_write(grove_root: &Path) -> Result<Guard> {
    match open_write(grove_root)? {
        Writing::Tree(tree) => Ok(tree),
        Writing::Vacancy(_) => Err(absent_tree(grove_root)),
    }
}

````
<!-- /fragment -->

The comment's rule is a claim about what the message *is about* — a command's
wait, not a lock — and that is a policy no signature can carry, which is why the
function exists at all rather than a flag on `write`.

**Nothing pins it, and the near miss is worth following.** The obvious
candidate is `mutator_waits_for_a_shared_worktree_reader_before_allocating_a_leaf`
in `crates/grove-llm/tests/tree_lock.rs`: it holds an external shared lock on the
worktree, runs `leaf-add`, and asserts on the child's stderr that the waiting
diagnostic appears exactly once — `assert_eq!(…count(), 1)` rather than a
`contains`, so a second copy would fail it. But count the guards. `reopen_write`
has three callers: `write` itself which announced before it got
here; the `leaf-decompose` retitle at `tree_lifecycle.rs` line 534; and
`apply_prune`'s loop at line 909. Only the last takes *N* of them, and `leaf-add`
is not it — a one-guard verb prints exactly once under a per-guard policy and a
per-command one alike, so that assertion cannot tell the two apart. The run that
could distinguish them is a bulk `leaf-prune` against a contended tree, and no
test watches stderr while doing it. `reopen_write`'s reason for existing is
argued in its own comment and held nowhere.

And the sixth and last of them is private, which is what makes `reopen_write`
possible at all.

<!-- fragment «tree-open-write» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="192-199" parent="tree-opening" -->
````rust
/// The exclusive acquisition itself, shared by both write-side entry points.
fn open_write(grove_root: &Path) -> Result<Opening> {
    #[cfg(test)]
    READ_COUNT.with(|count| count.set(count.get() + 1));

    ordinal_fs_tree::fs::write::<TaskName>(grove_root).map_err(|error| restate(grove_root, &error))
}

````
<!-- /fragment -->

Here is the asymmetry between the two sides, stated once. On the read side the
announcement is *inside* the acquisition, so every shared opening announces. On
the write side it is in the two entry points and not in the shared acquisition
beneath them, so an opening that should not announce is expressible — and
`reopen_write` is that opening. There is no `reopen_read` because there is
nowhere to put it: a second shared acquisition would announce, and no read verb
takes more than one anyway.

<a id="what-a-refusal-names"></a>
## What a refusal names, and the one it does not re-word

Two of the three error paths are small and neither is a rewording of the store.

<!-- fragment «tree-absent-tree» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="200-212" parent="tree-opening" -->
````rust
/// The diagnostic for a root that holds no tree — **moved, not redesigned**.
///
/// It is the sentence Grove's own lock layer produced when it met a missing
/// grove root, and the one [`restate`] still
/// produces for the *other* conditions it re-words. What changed at
/// `open-shape-k25` is only where the condition is decided: the library used to
/// meet a missing root as an I/O failure and Grove re-worded it, and now the
/// library has a word of its own for it — a vacancy — so Grove reads the shape
/// instead of the error and says the same thing.
fn absent_tree(grove_root: &Path) -> anyhow::Error {
    anyhow!("grove root not found: {}", grove_root.display())
}

````
<!-- /fragment -->

*Moved, not redesigned* is a claim about a user-visible string, and it is the
cheapest illustration in the chapter of the division of labour above: what
`open-shape-k25` changed is which side **decides** the condition, and the
operator's sentence was held still across that move precisely so the change would
be invisible from outside. A refactor called pure is one that has to be, and a
one-line function producing a fixed sentence is where that is easiest to check.

Two functions in this file produce that sentence. `absent_tree` is
one; `restate`'s absence clause is the other, and the two are the
same format string written twice with nothing holding them to each other. Eight
assertions pin it across the crate — four in later blocks of this file, at lines
1,311 1,518, 1,648 and 1,870, which chapters 7, 8 and 9 own, and four more in
`tree_lifecycle.rs`, in the `task_grow` test file, and in `grove-llm`'s own
integration suite — and **every one of the eight is a `contains`**. So not one of
them can tell the two spellings apart, and none would report it if one drifted.
The surface is wider than this file, besides: `grove-llm` writes the same opening
words a third time with different advice after them, and nothing holds that one
to these two either.

<!-- fragment «tree-raised» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="213-223" parent="tree-opening" -->
````rust
/// Turn a library error raised by a *mutation* into Grove's own.
///
/// Not [`restate`]: that one re-states a failed **read**, whose precedence
/// question is which of several conditions to name. A mutation's guard has
/// already read the tree successfully, so what arrives here is a `Refusal`, a
/// failed apply, or an unwind — and every one of those is printed unchanged
/// (`docs/ARCHITECTURE.md#library-refusals`, clause 3).
pub(crate) fn raised(error: Error<TaskName>) -> anyhow::Error {
    anyhow!("{error}")
}

````
<!-- /fragment -->

`raised` is `restate` seen from the end where the question has already been
answered, which is why it does nothing: a precedence question exists only where
several conditions compete to be named, and a mutation's guard has already
settled the one that competes — the tree was there.
`docs/ARCHITECTURE.md#library-refusals`, clause 3, is where the pass-through is
recorded, and the book names it rather than linking to it.

<a id="told-only-when-waiting"></a>
## Announcing the wait, and the probe that is never a decision

The longest item in the chapter is fifty lines, thirty of them comment — the
highest proportion of argument to code anywhere in the block. It is also the use
the crate's manifest names first when it explains `libc`.

<!-- fragment «tree-announce-contention» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="224-272" parent="tree-opening" -->
````rust
/// Say that this process is waiting, before it blocks.
///
/// Ordinary library reads and writes block; observers use the separate quiet
/// [`try_read`] path. Grove has always told an operator why
/// it appears to have hung, and losing that is a user-visible regression in what
/// the node brief calls a pure refactor. So the diagnostic is bought outside the
/// library: one non-blocking acquisition of the same mode on the same directory
/// the library will lock, released immediately. `mode` is the caller's own —
/// [`libc::LOCK_SH`] before a read and [`libc::LOCK_EX`] before a write, because
/// a shared probe taken before an exclusive acquisition succeeds while another
/// reader holds the tree and would swallow the very message it exists to print.
///
/// Best-effort by construction. Between releasing this probe and the library
/// taking its own lock a contender can arrive, and then this process blocks
/// silently; the probe is a diagnostic and never a decision, so that window
/// costs a message and nothing else. Everything that can go wrong with it —
/// including the directory not existing — is silence, because the library is
/// about to report the same condition properly.
fn announce_contention(grove_root: &Path, mode: libc::c_int) {
    // `<root>/..` and not `Path::parent`: the same spelling the library locks,
    // resolved by the kernel, so the probe asks about the directory the library
    // will actually contend for rather than a lexical parent of the string.
    //
    // The lexical parent is the fallback, and it is not a second-best: the
    // library's own `containing_directory` falls back to exactly it when there
    // is nothing at the root, because a root that is not there has no `..` to
    // resolve. Without this arm every fresh grove's creation would block
    // silently, since the tree it is about to create is by definition absent.
    let handle = File::open(grove_root.join("..")).or_else(|error| match grove_root.parent() {
        Some(parent) => File::open(parent),
        None => Err(error),
    });
    let Ok(handle) = handle else {
        return;
    };
    let descriptor = handle.as_raw_fd();
    // SAFETY: `descriptor` is open for the whole call — `handle` owns it and
    // outlives both `flock`s — and `flock` touches nothing else.
    if unsafe { libc::flock(descriptor, mode | libc::LOCK_NB) } == 0 {
        unsafe { libc::flock(descriptor, libc::LOCK_UN) };
        return;
    }
    let error = std::io::Error::last_os_error();
    if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
    {
        eprintln!("waiting for active Grove tree operation");
    }
}

````
<!-- /fragment -->

**The argument turns on a constraint the comment declines to fight.** The
library's locking is invisible in its interface, and the comment calls that the
architecture's own decision rather than a defect — which is what makes buying the
diagnostic *outside* the library the move rather than a workaround. Everything
odd about the function follows from that one choice: it duplicates the library's
acquisition because it has no way to ask the library a question about it.

**`mode` is the caller's own, and the reason is not symmetry.** A shared probe
taken before an exclusive acquisition *succeeds* while another reader holds the
tree, and would therefore swallow the very message it exists to print. So the
three call sites pass the mode they are about to contend in:
`libc::LOCK_SH` in `read_or_vacant`, and `libc::LOCK_EX` in `write` and
`write_or_vacancy`. `read`, `reopen_write` and `open_write` deliberately have
none. The observer also has none: its single attempt returns Busy to the caller.

That is held by `worktree_readers_share_the_lock_without_reporting_contention`,
which takes an external **shared** lock on the worktree, runs `pick`, and asserts
that the diagnostic is *absent* from stderr. A probe that ignored `mode` and
always asked for `LOCK_EX` would fail there, and it is the only one of that
file's four lock tests that asserts the message's absence rather than its
presence — the other three each assert `assert_eq!(…count(), 1)`.

**The two arms of the directory lookup are not a primary and a degradation**, and
that is the thing about this function likeliest to be read backwards. Kernel
resolution is right whenever there is something to resolve; the lexical parent is
right in the case grove meets constantly, because a `root-init` on a fresh
worktree is by definition opening a directory that is not there. Delete the
fallback as dead code and the announcement breaks on exactly the path where an
operator is most likely to be waiting.

**The probe is best-effort in both directions, and the source states one of
them.** Between releasing this probe and the library taking its own lock a
contender can arrive, and then the process blocks with nothing printed; that is
the window the comment names. The same window runs the other way — a holder can
release inside it, so a process that printed the message may not wait at all —
and neither costs anything, because of the line the comment does state: the probe
is a diagnostic and never a decision. Nothing branches on its result except the
`eprintln!`.

**The `SAFETY` note discharges exactly one obligation**, that `descriptor` is
open for the whole call, and it is discharged by ownership: `handle` owns the
descriptor and outlives both `flock`s. The second clause — that `flock` touches
nothing else — is what makes the first sufficient.

**Both `flock` calls carry a flag the source scan reads, and it reads them
differently.** The first passes `mode | libc::LOCK_NB`, which is the acquisition
`no_production_lock_grove_takes_for_itself_ever_blocks` requires; the second
passes `libc::LOCK_UN`, which leaves that scan as a release rather than
satisfying it. That test's claim is not *this probe is correct*; it is that
**no** `flock` grove takes in production waits, so the deleted layer cannot grow
back silently — its symptom would be a hang rather than a failure, which is the
kind of regression a test has to catch before a human does.

**Silence on every failure is `restate`'s division of labour applied one step
earlier.** The library decides and grove only chooses wording, so a probe that
reported its own errors would be grove deciding — about a condition the library
is a few lines from stating properly. It is also what lets the three call sites
invoke this unconditionally, with no guard and no result to check.

**On `libc`.** `Cargo.toml`'s dependency comment names this probe first among
three, and until `manifest-dependency-clauses-k133` it named it alone —
[chapter 1](01-orientation.md#the-package) reads the corrected clause and
explains why the probe is no longer allowed to stand for the crate. `libc` is
reached from three production modules: this probe, the lease's own locking and
close-on-exec descriptors in `driver_lease.rs`, and the terminal and signal calls
in `loop_driver.rs` that chapter 20 reads. What is true of this function is
narrower than the dependency and still worth having: it is the use the manifest
names first, and it is the one that would be hardest to justify without the
argument above, because a probe that is never a decision appears redundant until
its diagnostic purpose is stated.

<a id="refusal-precedence"></a>
## Refusal precedence, and the order grove owes its operator

The last function discharges the header's third claim, and its shape is a
precedence list rather than a match.

<!-- fragment «tree-restate» owner="one-spelling-of-grove" source="crates/grove-loop/src/task_tree.rs" lines="273-303" parent="tree-opening" -->
````rust
/// Re-state a failed read in the order grove owes its operator.
///
/// Each clause below is a condition grove states in its own words, tried in the
/// precedence the path-walking reader had. The checks are unlocked,
/// deliberately: the decision to refuse was already taken under the lock, and
/// only the wording is chosen here.
fn restate(grove_root: &Path, error: &Error<TaskName>) -> anyhow::Error {
    // **Before the absence clause**, because `is_dir` reads two of these as
    // absent: a dangling symbolic link at the root, and a root the library
    // refused for something a component below it. The library's own sentence
    // names what is there and says a tree is a directory, which is strictly more
    // than grove's *not found* — so it stands, and clause 3 keeps grove's
    // wording only where grove knows more.
    if matches!(error, Error::RootIsNotATree { .. }) {
        return anyhow!("{error}");
    }
    if !grove_root.is_dir() {
        return anyhow!("grove root not found: {}", grove_root.display());
    }
    match error {
        // The domain's own advice *is* the message (`Error`'s `Display` says
        // why), and it names the offending filename but not where in the tree it
        // sits. A tree deep enough to have two levels is a tree where that
        // matters, so the path is appended rather than the message rewritten.
        Error::Malformed { path, .. } | Error::Reserved { path, .. } => {
            anyhow!("{error} ({})", path.display())
        }
        _ => anyhow!("{error}"),
    }
}

````
<!-- /fragment -->

**The first clause is placed by a hazard rather than by importance.**
`RootIsNotATree` is tested *before* the absence clause because `is_dir` reads two
quite different conditions as absent: a dangling symbolic link at the root, and a
root the library refused for something a component below it. In both cases the
library's own sentence names what is actually there and says that a tree is a
directory, which is strictly more than grove's *not found* — so it stands. The
comment states the rule that generalises it: grove's wording is kept only where
grove knows more.

**The second clause is the one place grove genuinely knows more**, because a root
is grove's concept. It produces the sentence `absent_tree` produces, by writing
it out a second time.

**The third appends rather than rewrites**, and the comment gives the reason as a
property of the message: the domain's own advice *is* the message, and it names
the offending filename but not where in the tree it sits. A tree deep enough to
have two levels is a tree where that matters — so the path is appended in
parentheses and the store's sentence is left intact. `Malformed` and `Reserved`
are the two verdicts chapter 2 established halt the whole tree, which is why they
are the two that need a location.

**The fourth passes the store's sentence through unchanged**, which is the same
default `raised` applies on the mutation side.

The whole function is unlocked, and the doc comment defends that rather than
apologising for it: the decision to refuse was already taken under the lock, and
only the wording is chosen here. The `is_dir` call it makes is therefore
racy by construction and harmlessly so — a root deleted between the refusal and
this call changes which sentence an operator reads about a failure that has
already happened.

The tree is open. What a caller holds is a snapshot under a lock, and the one
thing that snapshot will not give them is a path — the library refused to add a
`path()` to its algebra and said why, and this module's header answered by naming
the single place a consumer builds one. Chapter 6 reads `entry_path` and the
addressing machinery around it: which entry a path is being built for, which keys
are reachable by a walk and which are not, and the promotion whose intermediate
state is the reason those two questions are not the same.

Nine of this file's ten ownership blocks remain, and chapters 6 through 9 resolve
them.

<a id="captured-observation"></a>
## Captured names without a standing lock

A viewer needs to keep displaying a tree after allowing its next mutation.
`try_observe` takes the exact worktree and an ordered list of preferred selected
keys. It returns names, the selected file's bytes and a retained `TreeLifetime`;
all advisory locks have gone before the caller builds rows. `grove-tui` compares
two such captures and owns folds, lifecycle totals and reading positions.
The item-status observation design in `docs/specs/item-status.md` separates
this capture from a subsequent runtime sample. Its independent result can
establish Idle; witnessed Running remains a later protocol increment.

<!-- fragment «observation-tree» owner="one-spelling-of-grove" source="crates/grove-loop/src/observation.rs" lines="1-177" parent="source-observation" -->
<!-- insert «observation-imports» -->
<!-- insert «observation-lifetime» -->
<!-- insert «observation-values» -->
<!-- insert «observation-capture» -->
<!-- /fragment -->

The observer composes the existing quiet reader and canonical path helper. Its
imports name the ownership boundary: filesystem capture is the loop's, while
`Snapshot` remains the tree library's value representation.

<!-- fragment «observation-imports» owner="one-spelling-of-grove" source="crates/grove-loop/src/observation.rs" lines="1-11" parent="observation-tree" -->
````rust
//! One captured tree and selected file, with no advisory guard returned.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context};
use ordinal_fs_tree::Snapshot;

use crate::{entry_path, Error, Parts, Reading, TaskName, TryReading};

````
<!-- /fragment -->

`TreeLifetime` opens a directory read-only, nonblocking and close-on-exec. It
holds no advisory lock. Its `at` comparison detects a changed path and `same`
compares retained device/inode pairs, allowing alias spellings. Keeping the
old directory open prevents its identity being reused while the viewer retains
it. Metadata failures remain errors; a missing or nondirectory root opens as
None. The viewer also uses these operations around capture to discard stale
item state even when a writer is busy.

<!-- fragment «observation-lifetime» owner="one-spelling-of-grove" source="crates/grove-loop/src/observation.rs" lines="12-62" parent="observation-tree" -->
````rust
/// A retained directory descriptor prevents inode reuse while a capture lives.
/// This value holds no tree lock, epoch guard or driver authority.
#[derive(Debug)]
pub struct TreeLifetime(File);

impl TreeLifetime {
    /// Open the exact task-root directory without creating or locking anything.
    ///
    /// # Errors
    /// An existing directory cannot be opened. Missing/nondirectory roots are None.
    pub fn open(worktree: &Path) -> Result<Option<Self>, Error> {
        // custom_flags adds OS flags without changing read-only access.
        // https://doc.rust-lang.org/std/os/unix/fs/trait.OpenOptionsExt.html#tymethod.custom_flags
        match OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NONBLOCK | libc::O_CLOEXEC)
            .open(worktree.join(".grove"))
        {
            Ok(file) => Ok(Some(Self(file))),
            Err(error) if matches!(error.raw_os_error(), Some(libc::ENOENT | libc::ENOTDIR)) => {
                Ok(None)
            }
            Err(error) => Err(anyhow::Error::new(error).into()),
        }
    }

    /// Whether this retained directory still occupies the observed task-root path.
    ///
    /// # Errors
    /// Metadata cannot be read for the descriptor or current path.
    pub fn at(&self, worktree: &Path) -> Result<bool, Error> {
        let current = match std::fs::metadata(worktree.join(".grove")) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(anyhow::Error::new(error).into()),
        };
        let old = self.0.metadata().map_err(anyhow::Error::new)?;
        Ok(current.is_dir() && old.dev() == current.dev() && old.ino() == current.ino())
    }

    /// Compare two still-pinned directory identities, independent of path spelling.
    ///
    /// # Errors
    /// Metadata cannot be read for either descriptor.
    pub fn same(&self, other: &Self) -> Result<bool, Error> {
        let a = self.0.metadata().map_err(anyhow::Error::new)?;
        let b = other.0.metadata().map_err(anyhow::Error::new)?;
        Ok(a.dev() == b.dev() && a.ino() == b.ino())
    }
}

````
<!-- /fragment -->

`TreeObservation` separates vacancy and writer contention from a captured tree;
invalid trees remain errors. `CapturedTree` holds the captured snapshot and root
path, a selected permanent key (None for the root), and bytes or a file-read
error. A failed selected file therefore leaves the names available for browsing.
The consuming `ReadGuard::into_snapshot` operation supplies owned names without
cloning, reparsing or retaining the store's lock.

<!-- fragment «observation-values» owner="one-spelling-of-grove" source="crates/grove-loop/src/observation.rs" lines="63-93" parent="observation-tree" -->
````rust
/// The tree portion of one quiet observation. Errors remain a separate Result.
pub enum TreeObservation {
    Ready(CapturedTree),
    Vacant,
    Busy,
}

/// Names and selected bytes copied under the tree guard, then released from it.
/// The snapshot's paths use the caller's spelling. The lifetime pins that capture.
pub struct CapturedTree {
    root: PathBuf,
    snapshot: Snapshot<TaskName>,
    pub lifetime: TreeLifetime,
    /// First surviving requested key, or the root (None).
    pub selected: Option<u32>,
    /// A file failure does not discard a readable tree.
    pub content: Result<Vec<u8>, String>,
}

impl CapturedTree {
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub fn snapshot(&self) -> &Snapshot<TaskName> {
        &self.snapshot
    }
}

````
<!-- /fragment -->

The capture pins the task root before the quiet read, then validates that pin
against the path before and after copying selected bytes. Whole-tree
`select_snapshot` validation runs before choosing content, so duplicate keys or
multiple live finish leaves cannot be hidden by a selected item. Preferred keys
are tried in order; the root brief is the fallback. A branch resolves to its
node brief through the same `entry_path` helper used for leaves. Moving the
snapshot out releases the shared tree lock before returning the value.

These identity comparisons detect observed non-cooperating replacement; they
do not make arbitrary filesystem edits atomic. `grove-tui` additionally compares
the two retained lifetimes alongside rows and bytes. Cooperating writers use the
tree lock, which spans this capture's selected-file read but no caller work.

`ObservationGuard` now separates tree failure from runtime failure. The loop
finishes `capture` and releases its tree guard before entering runtime observation;
the two private callbacks mark those boundaries for deterministic lock tests.
No callback is exposed to viewers. `ActivityObservation` can establish Idle,
report a contended epoch as Busy, or withhold evidence as Unavailable. The
[current runtime reader](17-the-epoch.md#runtime-observation) explains why an
older active epoch cannot establish RUNNING. Browsing consumes `tree` regardless
of that independent result. The viewer accepts activity separately across two
captures and shows the shared selector's NEXT only for accepted Idle; changing
activity withholds the pair without discarding a consistent tree.

<!-- fragment «observation-capture» owner="one-spelling-of-grove" source="crates/grove-loop/src/observation.rs" lines="94-177" parent="observation-tree" -->
````rust
/// Runtime evidence at this sample. Legacy active records cannot identify a mandate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivityObservation {
    Idle,
    Busy(String),
    Unavailable(String),
}

/// Independent results, captured values and a tree pin; no advisory lock escapes.
pub struct ObservationGuard {
    pub tree: Result<TreeObservation, Error>,
    pub activity: ActivityObservation,
}

/// Capture the tree and selected bytes, release its guard, then sample runtime.
/// Candidate keys are ordered by preference; None requests the root brief.
/// Runtime failures preserve the tree. Neither operation grants session authority.
pub fn try_observe(worktree: &Path, candidates: &[Option<u32>]) -> ObservationGuard {
    observe_with(worktree, candidates, || {}, || {})
}

pub(crate) fn observe_with(
    worktree: &Path,
    candidates: &[Option<u32>],
    after_capture: impl FnOnce(),
    in_epoch: impl FnMut(),
) -> ObservationGuard {
    let tree = capture(worktree, candidates).map_err(Error::from);
    after_capture();
    let activity = crate::driver_lease::observation::observe(worktree, in_epoch);
    ObservationGuard { tree, activity }
}

fn capture(worktree: &Path, candidates: &[Option<u32>]) -> anyhow::Result<TreeObservation> {
    // Pin before reading so a changed root cannot be attached to old names.
    let lifetime = TreeLifetime::open(worktree)?;
    let tree = match crate::try_read(worktree)? {
        TryReading::Busy => return Ok(TreeObservation::Busy),
        TryReading::Ready(Reading::Vacant) => return Ok(TreeObservation::Vacant),
        TryReading::Ready(Reading::Tree(tree)) => tree,
    };
    let lifetime = lifetime.context("tree changed during observation; retrying")?;
    ensure!(
        lifetime.at(worktree)?,
        "tree changed during observation; retrying"
    );
    crate::select_snapshot(tree.root(), tree.snapshot(), None)?;
    let brief = tree
        .snapshot()
        .root()
        .distinguished()
        .context("root has no brief")?;
    let mut files = vec![(None, entry_path(tree.root(), brief))];
    for entry in tree.walk() {
        let Some(triple) = entry.triple() else {
            continue;
        };
        let file = match triple.parts {
            Parts::Leaf { .. } => entry,
            Parts::Node => entry
                .contents()
                .and_then(|level| level.distinguished())
                .context("branch has no brief")?,
        };
        files.push((Some(triple.key.get()), entry_path(tree.root(), file)));
    }
    let (selected, path) = candidates
        .iter()
        .find_map(|key| files.iter().find(|(candidate, _)| candidate == key))
        .unwrap_or(&files[0]);
    let content = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()));
    ensure!(
        lifetime.at(worktree)?,
        "tree changed during observation; retrying"
    );
    let root = tree.root().to_path_buf();
    Ok(TreeObservation::Ready(CapturedTree {
        root,
        snapshot: tree.into_snapshot(),
        lifetime,
        selected: *selected,
        content,
    }))
}
````
<!-- /fragment -->

The public-seam tests in `crates/grove-loop/tests/observation.rs` exercise
leaf, branch and root content, fallback selection, duplicate-key refusal,
vacancy, writer contention and replacement with an old capture still alive.
The writer test tries an exclusive nonblocking lock on an independent worktree
descriptor while two captures remain alive. Deliberately leaking the reader's
lock makes that assertion fail; normal descriptor drop makes it pass. The
existing Viewer application suite checks that the migrated caller preserves its
interaction and retry behavior.

[Previous: The name, and canonicity](04-the-name.md) | [Contents](README.md) | [Next: Paths, and addressing](06-paths.md)
