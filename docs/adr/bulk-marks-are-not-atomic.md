# Bulk marks are not atomic

`leaf-prune` given a node directory marks every live leaf in its subtree, and
each mark is one `ordinal-fs-tree` `rewrite` under its own exclusive guard. A
mutating method **consumes** its `WriteGuard`
([`docs/ordinal-fs-tree/ARCHITECTURE.md`](../ordinal-fs-tree/ARCHITECTURE.md),
*A mutation consumes its guard*), so *N* marks are *N* critical sections where
Grove once had one. Grove accepts that rather than asking the library for a
batched rewrite.

Two things keep the cost small enough to accept, and they are the whole of the
decision:

- **The plan is still validated before anything moves.** The whole subtree is
  walked, classified and checked against the *first* guard's snapshot, and only
  then is the first rename taken. A leaf the mark cannot address, a `finish`
  leaf, a leaf whose destination the tree already occupies — every one of those
  fails the call with nothing renamed, exactly as before.
- **A half-finished prune is repaired by running it again.** The marks *are* the
  state, an already-`ABANDONED` leaf is skipped silently, and a `DONE` leaf is
  reported and left alone — so `leaf-prune <node>` is re-runnable and converges.
  That is what an operator does when a prune stops half way, and it is why the
  residue needs no recovery procedure of its own.

What is genuinely lost is the window *between* guards: another writer that
arrives mid-run, or a filesystem fault on the fourth of six marks, now leaves
part of the subtree marked. `crates/grove-loop/src/tree_lifecycle.rs`'s
`pruning_a_node_takes_one_guard_per_mark` asserts the guard count, so the cost is
a number a later change moves rather than a paragraph it can quietly contradict.

`loop-crate-verbs-k21` **re-checked this and moved nothing.** A subtree prune is
still *N* rewrites under *N* guards, and the count that test asserts is
unchanged. What the leaf added is a name for the window: `grove_loop::TreeWrite`
is a caller's *right to be the writer* rather than one guard, and it hands out
the guard it opened with before reopening for the next verb. That is this record
made explicit rather than a second instance of it — the gap it exposes is the
same one, and no verb gained a guard.

`lint-lock-scope-k32` added the one affordance that shape needed. The
cross-reference lint reads under a **shared** lock of its own, which is a second
file description — so it would have met the deadlock the rejected
*hold Grove's own exclusive guard around the whole run* option below describes,
against its own process, whenever the caller still held an unspent guard. `TreeWrite::relinquish` gives that guard up before the second
opening is taken, which is how a verb obeys *one opening at a time* rather than
an exception to it. Still no verb holds two.

## The trade-off

The guard is consumed for a reason the library argues at length: every operation
is planned *from* the snapshot, so a guard surviving its own mutation would
describe a tree that no longer exists, and refreshing the snapshot instead would
make a successful mutation return the error of the re-read that followed it.
Grove's bulk arity is the one shape that pays for it, and it pays in a place
where the payment is cheap: `leaf-prune` on a node is a human-confirmed, rare
decision to abandon a whole line of work, taken once and visible on disk
immediately afterwards.

Against that, a batched rewrite is a change to a **checked** library, and this
workstream's standing rule is that **the model leads** wherever the models reach
at all: Quint is written before the operation is implemented, and where the model
and a test disagree the model wins and the test changes. A batched rewrite is
squarely inside that boundary — it is a plan of renames, which is what
`operations.qnt` is about — unlike `delete`, whose exclusion
[`ARCHITECTURE.md`](../ordinal-fs-tree/ARCHITECTURE.md) states under *The
models*. So `models/operations.qnt` would
move first, then the algebra, then the filesystem layer, then grove — for a
property no current consumer needs except in a failure mode that already
self-repairs. Doing it here, mid-flip, would be the failure the leaf brief
existed to prevent: discovering the need for a library change and satisfying it
inline.

## Considered options

- **Re-plan the prune as one operation the library can express.** Rejected
  because there is none. `append_many` is a run of creations planned from one
  snapshot; there is no `rewrite_many`, and a promotion is not a mark. **The
  operation set is not closed** — `initialize` was added to it after this record
  was first written and `delete` after that, and neither creating a tree nor
  destroying one is a mark — so what rules this option out is the absence of a
  *batched rewrite* specifically, and not a surface nobody may extend. The
  distinction is the difference between this option being impossible and it
  being unbuilt. It is unbuilt, and the option below is the route to building
  it.
- **Escalate: ask `ordinal-fs-tree` for a batched rewrite**, as a leaf of its
  own, model first. Not rejected on the merits — rejected as *not yet earned*.
  Reopen when a second consumer needs it, or when a partial prune is observed in
  the wild and re-running the verb turns out not to be the repair. That reopening
  is a library leaf and a `models/operations.qnt` change, never an inline
  widening.
- **Hold Grove's own exclusive guard around the whole run** and let the library
  take its guard inside it. Rejected because it deadlocks: both `flock` the
  directory containing the tree root, and two open file descriptions on one
  directory do not share a lock
  ([`docs/ARCHITECTURE.md`](../ARCHITECTURE.md#one-lock-and-it-is-the-librarys),
  *One lock, and it is the library's*).
- **Perform the first mark through the library and the rest with `fs::rename`,**
  under one guard. Rejected outright: it is Grove re-implementing the algebra the
  flip exists to delete, and the renames it took would be the ones the library's
  own plan never checked.

## Why this is hard to reverse

The direction that is hard is the one back. Undoing the acceptance means adding
an operation to a library whose behaviour is checked by a model that leads it,
and whose one-guard-one-mutation rule is argued from plan atomicity rather than
chosen for convenience — so a batched rewrite is not a new
method but a new answer to *what a plan is*, with the reference domain, the
syllabus CLI and the conformance kit all downstream of it. Accepting the cost, by
contrast, is reversed by that same work whenever it is worth doing; nothing here
forecloses it, which is why the escalation is written down as a live reopening
condition rather than a closed door.
