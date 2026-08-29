# root-delete-k26

## Goal

Let the store remove the tree root and everything beneath it, and report what it
removed honestly.

## Context

`docs/specs/module-decomposition.md`, decision 2, stated verbatim:

```rust
impl<N: EntryName> WriteGuard<N> {
    /// Remove the tree root and everything beneath it, following no symlink.
    pub fn delete(self) -> Result<Removed, Error<N>>;
}

pub struct Removed { pub root: PathBuf, pub entries: Vec<PathBuf> }
```

**Why paths, where every other mutation reports names.** Design-review finding 3,
and the spec states it as the operation's own asymmetry rather than an oversight:
the existing `Report` has a created bucket and a renamed bucket, both keyed by
`N`, because every other mutation acts on entries the domain named. Deletion acts
on the *root* and therefore on everything beneath it — **including the entries
the domain deliberately declines to parse as `N`**, which the walk already skips
and which the report has no `N` to name. A third bucket of `N` would still be
unable to say what it removed. `Removed` is the honest postcondition: the paths
that are gone, which is exactly what a caller needs to say what it destroyed and
is the whole of what the operation knows.

## Done when

- `delete` consumes the write guard, removes the root and everything beneath it,
  and **follows no symlink**. The no-symlink rule is a security property, not a
  detail: assert it in a test with a symlink pointing outside the root.
- `Removed { root, entries }` reports paths **in the order they went**, foreign
  entries included.
- `docs/adr/entries-are-never-removed.md` is **amended with one clause**
  distinguishing removing an *entry* from deleting the *root*. Its argument is
  untouched — removing an entry lowers the visible key maximum and the next
  allocation re-issues a live key — but its **opening sentence** says the library
  offers no removal operation, and that is what stops being true. Brief principle
  5 already states the distinction; the record does not.
- Deleting a vacancy is not expressible (it is a method on the tree guard, which
  a vacancy is not) and a test or doc test says so.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, `CONTEXT.md` and `CLI.md` reconciled;
  `models/operations.qnt` and `models/structure.als` gain root removal or state
  why they do not.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it depends on `open-shape-k25` only for `WriteGuard` to be
reachable through the new `Writing` shape.

**This is the operation that makes the finish teardown the store's.** Grove's
`finish-commit` currently removes `.grove/` itself; after this it can ask the
store, under the lock, and get back the paths it needs to name in a commit
message. Wiring that up is `loop-crate-verbs-k21`'s, not this leaf's.

**Do not add a recovery path.** Principle 1: the version control system owns
safety and history — `jj undo` restores a deleted root, as `minimalism-k1`
measured directly. A `delete` that quarantines, backs up, or stages a rollback is
the machinery this whole grove exists to remove.

## Decisions (running log)

**The removal walks the filesystem itself rather than the snapshot, and it is
post-order.** The snapshot holds only the entries the domain parsed as `N`, and
`Removed` is obliged to report the foreign ones too — so the walk is a second
listing pass, sharing `fs::read`'s own `listing` (sorted, and asking
`DirEntry::file_type`, which does not follow a link) so that the two cannot
drift on either determinism or link-following. Children go before the level that
holds them, and within a level in the listing's sorted order; the root goes last
and is `Removed::root` rather than an element of `Removed::entries`. The walk is
an explicit worklist and not recursion, for the reason `read::snapshot` gives:
the depth of a tree on disk is the operator's to choose and a stack overflow is
not a refusal any consumer can handle.

**There is no order that buys a property here, unlike the highest-first shift.**
Every order lowers the visible key maximum and leaves a tree the library did not
build, so the sorted order is for *determinism* and nothing else — an interrupted
deletion leaves a partial tree whose recovery is the consumer's, not an admitted
shape like a gapped level. Said so where `ARCHITECTURE.md` argues the shift's
order, so the absence of a claim is visible beside the place a claim is made.

**A removal cannot be rolled back, so it needs an error variant of its own:
`Error::RemovalStopped`.** `Error::Failed`'s whole promise is *the tree is as it
was found — every effect this operation had applied was undone*, and a deletion
has nothing to put back. `FailedPartiallyRolledBack` is equally wrong: nothing
was unwound, because unwinding was never on the table. The new variant carries
the paths that had already gone, so a caller can still say what it destroyed, and
its `Display` branches on that list being empty — which is the honest difference
between *the tree is as it was found* (the first listing failed) and *the tree is
in neither state*. No new exit code: the CLI maps the empty case to `6` and the
non-empty case to `7`, which is exactly the distinction those two rows already
draw.

**Neither model gains root removal, and the reason is the boundary they both
stop at rather than cost.** Three things make `delete` what it is: a directory
can be removed only once it is empty, a symbolic link is unlinked rather than
followed, and foreign entries are removed *and reported*. The second and third
are a link and a path, and neither model holds either. The first is the one an
ordering claim would rest on — and the **filesystem checks it directly**, because
a wrong order makes `remove_dir` refuse, so every deletion the suite performs is
a test of it, where a model with no filesystem would have to assume the very fact
it wanted to check. Adding a `Remove` to a *forward* plan would also make
`inv_atomicity` false by construction, and a second interpreter is what
`ARCHITECTURE.md` calls a finding about the plan rather than a licence. Written
into both model files, into `ARCHITECTURE.md`'s *The models*, and into `lib.rs`,
which now says that an operation the models do not reach is declared rather than
skipped. `docs/formalism-findings.md` gains nothing: it is a closed campaign log
whose own header says the models it reports on no longer exist.

**`root-lifecycle-stays-with-its-receipt` is reversed rather than amended, and
renamed to `root-lifecycle-belongs-to-the-store`.** That record deferred its own
re-taking to this leaf in as many words, and its creation half was already false
— `open-shape-k25` landed `Vacancy::initialize` without touching it. Both halves
now go the other way. The three arguments behind the destruction rejection were
all about coordinating a destroy with an external effect through a four-point
callback, and `delete-finish-transaction-k8` deleted that machinery; what is left
is *a destroy whose only verdict is the filesystem's own*, which is the record's
own stated reopen condition. The receipt argument survives and turns out to
decide something narrower than it was read to decide: it says where the receipt
lives — outside the container, in the repository's history — and not who performs
the removal. `Removed` is a postcondition and not a receipt, which is the
distinction the old record collapsed. The coordinated form of destruction stays
rejected on the original argument, and the new record says so, so what a future
consumer could reopen is the coordinator and not this decision.

**Citations reconciled, and one of them moved context.** `CONTEXT-MAP.md` had the
record under the **grove** context; it is now under **ordinal-fs-tree**, and that
move *is* the decision rather than bookkeeping beside it.
`entry-name-is-the-only-seam` carried a passage saying destruction needed a
callback, an uncheckable obligation and a receipt with nowhere to go — rewritten
to say the rule took the pressure without widening, and that what would have
widened it is the *coordinated* destroy that remains rejected.
`docs/formalism-findings.md`'s entry 047 keeps its verdict and gains a
`[reversed by root-delete-k26]` note in the log's own bracket convention, because
nothing in that entry is wrong: the rejection was sound against the machinery it
was argued about.

**`grove-does-not-stage-its-own-renames` needed nothing; `bulk-marks-are-not-atomic`
needed one clause.** Re-checked, not assumed. The first already says a `mkdir`
and an `rmdir` are working-copy changes Jujutsu snapshots exactly as `rename(2)`
is — written at `open-shape-k25`, ahead of the operation, and true now. The
second's *the operation set is not closed* clause names `initialize` as the
counterexample and gains `delete` beside it; its *the model leads* sentence read
as a rule that Quint is written for every operation, which this leaf falsifies,
and is narrowed to *wherever the models reach at all* with the boundary named.

**Grove's own error-reachability table gains two rows, one of them a gap this
leaf did not open.** `docs/ARCHITECTURE.md` enumerates every non-`Io` `Error`
variant, so a new variant leaves it incomplete: `RemovalStopped` is **no** —
grove calls nothing that removes anything until `loop-crate-verbs-k21` wires the
teardown up, which is the leaf that re-takes the row. `RootIsNotATree` was
missing too, from `open-shape-k25`, and is added rather than left: a table that
claims to enumerate is falsified by one absence.

**The CLI gains `delete --yes`, and the confirmation is a flag rather than a
prompt.** `CLI.md` argued that no verb needed confirming *because* nothing was
destructive, so that reasoning had to be re-taken rather than patched. A prompt
is unanswerable by the contract tests and scripts that are this binary's whole
audience; a flag is the same confirmation in a form both they and an operator can
give, and it keeps *no prompt* true. No `--force` beside it: `--yes` overrides no
safety check, it **is** the confirmation, and `cli-tool-design` keeps those two
concepts one spelling apart. No `rm` alias either — an alias would make the one
destructive verb the shortest thing to type.

**The leaf's one in-session reviewer was spent on the deletion walk's safety
claims, and it found three critical defects that share one cause.** The
allowance went on a single fresh context given `remove.rs`, the opening it sits
behind, the error, the report and the tests, with five named claims to attack —
no symlink followed, the report honest, the failure report exact, no stack
overflow or cycle, and no escape from the root. Every finding below was
**reproduced here before being acted on**, at the CLI, and the reproduction is
what makes the classification a fact rather than a report.

*Valid and actionable, fixed — and all three are the same defect.* The walk
built every path by joining names onto **the caller's own root spelling**, and
two accepted spellings make that unsound. **(a)** `--root <a symlink to the
tree>` — `read_dir` follows the link, so the target's contents were unlinked;
then `rmdir` on the link failed with `ENOTDIR`, leaving the tree emptied, the
link standing and an error. **(b)** `--root <a symlink>/` — the trailing slash
makes `symlink_metadata` **resolve** the link, so the same walk ran and on macOS
`rmdir("link/")` removed the *target directory* and returned **`Ok`**: a tree
outside anything the caller named, destroyed silently, with `Removed.root`
naming a link that is still there. **(c)** `--root <tree>/<node>/..` — the walk
removed `<node>`, which is a component of the spelling, and every path after
that failed with `ENOENT` on files that were plainly present; the tree was left
half destroyed with no spelling able to finish it.

*The fix is a precondition, not a repair.* Deletion is the one operation that
acts on the root as an **object** rather than as the directory things are in, so
it is the one for which the last component decides *what gets destroyed*. It now
refuses a root whose last component is a symbolic link or is not a name at all,
and one containing a `..` that cancels a name — before anything is removed,
as `Error::RootIsNotSpelledDirectly`, which says which spelling it was.
Principle 2: the anomaly is named and stopped, and this library does not choose
between a link and its target on a caller's behalf. Every other operation still
accepts both spellings, deliberately, and `reading_on_disk.rs` still holds them.

*The `..` rule is coarser than the danger, and that is stated rather than
hidden.* A `..` cancelling a component *above* the tree is harmless and is
refused with the rest of the class. Separating them means resolving the path to
learn which components are inside the tree, and this module resolves nothing —
so the coarse rule costs one message asking for a direct spelling, where the
precise one would cost a resolution step on the destructive path. A **leading**
`..` cancels nothing and is accepted, which keeps `../course` ordinary; a CLI
contract test drives it from a working directory, because that is the only place
a relative spelling is real.

*Valid and actionable, fixed.* `a_removal_that_gets_nowhere_says_the_tree_is_as_it_was_found`
asserted only about the **report** — an implementation that removed half the
tree and reported an empty `removed` would have passed it, which is precisely the
direction it exists to hold. It now lists the root. The permission probe left a
`.probe` file behind on the one path where it succeeds; it cleans up.

*A contract stated unclearly, reworded.* The module header said *nothing here
follows a link, anywhere*, which was true of the entries and false of the root's
own last component — the sentence that hid all three defects. It now says which
is which, and names the refusal.

*Noise for this leaf, and recorded so the next reader does not re-find it.*
`a_tree_the_domain_cannot_read_cannot_be_deleted_either` judges **reachability**
and never enters the removal — true, useful, and now saying so in its own
docstring rather than reading as a test of `delete`. A bind mount of an ancestor
into a descendant would make the walk descend forever; it needs root to set up,
the reviewer did not test it, and no ordinary process can create a directory
cycle on either platform this crate targets.

*What the reviewer attacked and could not break, which is the half worth
keeping.* Links **inside** the tree, on every path it tried; the
report/removal correspondence and the post-order for a direct spelling; the
*empty when something went* direction of the failure report; the worklist's
bound and the absence of a cycle; an escape by way of anything `read_dir` can
yield, since it cannot produce a name containing a separator, an empty name, `.`
or `..`; `remove_file` as the right call for every non-directory; and the
guard's lifetime, which keeps the lock — held on a directory that is provably not
the one being removed — for the whole operation.

**No second reviewer, and the rule is why.** A second pass would be the signal to
cut a `review-impl` leaf. It is not needed here: the fix is a precondition
checked before any effect, and it is **conclusively covered by an executable test
seam** — four tests over the three refused spellings and the accepted one, each
of which fails if the check is removed.
