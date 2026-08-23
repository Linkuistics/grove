# reading-k31

## Goal

Move grove's whole reading surface onto the library: `pick`, `select`,
`brief-chain`, `kind`, `resolve`, and the shared read guard. The first leaf of
the **migrate** stage, and the one that establishes the two things every later
leaf uses — the snapshot-shaped read, and path construction.

## Context

- `crates/ordinal-fs-tree/src/snapshot.rs` — `Snapshot`, `Entry`, `Container`,
  `Walk`, and the reading operations: `walk`, `find`, `by_key`, `ancestors`,
  `distinguished_chain`.
- `crates/ordinal-fs-tree/src/fs/mod.rs` — `fs::read` and its `ReadGuard`.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, *Reading*: walk order is computed from
  the names, never from the order the directory was read in, and the tie-breaks
  are load-bearing. Note the document's own warning — **walk order is unmodelled**,
  so `by_key`'s tie-break on a duplicate-key tree rests on prose and on tests.
- `docs/ordinal-fs-tree/CLI.md`, *What `cli-k16` should watch* and *found*: the
  library's reading surface returns **no paths**, and building them in the
  consumer is safe because every name a snapshot admits has already been checked
  to render as one path component. `cli-k16` explicitly refused to answer this by
  adding `path()` to the algebra.
- `src/tree_read.rs` — every verb this leaf replaces, and `read_level`, which must
  stay alive for the leaves that have not flipped yet.
- `src/tree_access.rs` — the read guard, and `refuse_pending_*`.
- Suites that are the net here: `pick`, `resolve`, `brief_chain`, `kind`,
  `session_kind_tree`, `tree_access`.

## Done when

- `pick`, `select`, `brief-chain`, `kind` and `resolve` all read through
  `fs::read` and a `Snapshot`, and grove's read guard is the library's.
- Path construction exists in **exactly one** place — the root's own spelling,
  plus each ancestor node's rendered name, plus the entry's — and every later
  leaf uses it. Do not canonicalise: the library deliberately never does, and
  the root brief carries why (on macOS `/var` and `/private/var` name the same
  inode, so canonicalising would make the mere presence of a lock rewrite every
  path a read verb returns).
- The pending-transaction refusals come from the grammar — a `Verdict::Reserved`
  halting the read — rather than from a hand-written pre-check, or the leaf says
  why they could not and what still holds them.
- `tree_read`'s remaining exports are only those the unflipped leaves still need.
- The whole suite passes. Any test that had to change is recorded in the node
  brief with the reason.

## Notes

**`resolve` is the interesting one**, because it is grove's only lookup that is
not by key. It searches live, `DONE` and `ABANDONED` entries by key, by bare
slug, and by full handle, and reports ambiguity by listing each match's key. The
library offers `by_key` and `find` with a caller-supplied predicate — and *no*
lookup by label, deliberately and unavoidably: the trait names no label type, so
a `by_label` would have nothing to take. So slug lookup becomes a `find`-shaped
walk over the snapshot with grove's own predicate over its own `Parts`, and
ambiguity detection needs the whole walk rather than the first hit. That is a
straightforward fit, but it is the one place the seam's narrowness is felt from
grove's side — if it is awkward, that is a finding about the seam and belongs in
`docs/formalism-findings.md`, not a thing to work around.

**`brief-chain` is `ancestors` plus `distinguished_chain`**, and the library
already skips levels with no distinguished child, which is exactly
`brief-chain`'s documented *a directory level with no `BRIEF.md` is skipped
silently*.

**Watch the root's element type.** `ancestors` ends at the tree root, which is a
node and not an entry, so its element type is not the entry type — grove's
`.grove/BRIEF.md` is the root's distinguished child and comes back through the
root `Container`, not through an `Entry`.

**One lock, not two.** grove's guard and the library's both `flock` the directory
*containing* the tree root, and two open file descriptions on one directory do
not share a lock. A verb that acquires both deadlocks against itself. So a verb
uses one guard or the other, and this leaf converts them a verb at a time.
