# reading-k9

## Goal

The first leaf that touches a filesystem: the advisory lock, the snapshot that
turns a directory tree into an in-memory tree of names, and the five reading
operations over it. After this leaf the crate can be pointed at a real directory
and asked what is in it.

## Context

Beyond the brief chain:

- `ARCHITECTURE.md` — *The parse trichotomy*, *What is not in the trait, and
  why* (locking), *Operations → Reading*, and the invariants *No recognised name
  is silently skipped* and *Species agreement*.
- The root brief's `Pointers`, which carry three facts about the lock that came
  from reading the working implementation and must survive the extraction:
  the lock is `flock` on the **parent** of the tree root, not the root; paths
  are deliberately **never canonicalised**, because on macOS `/var` and
  `/private/var` name the same inode and canonicalising would make the mere
  presence of a lock rewrite every path a read verb returns; and locking follows
  inode identity through the descriptor while output preserves the caller's
  spelling.
- `src/repo.rs` and `src/tree_access.rs` for prior art on the guards. Prior art
  only.
- `operations.qnt` — the `unparseable` instance, and the invariant that a
  `Malformed` or `Reserved` name anywhere a walk reaches halts every operation.

## Done when

- Shared and exclusive lock guards exist, taken on the directory containing the
  tree root, with the no-canonicalisation rule preserved and its reason recorded
  where the next reader will meet it.
- A snapshot reads a whole tree, classifies every name through the consumer's
  `parse` with what the listing found — unfollowed — and halts on `Malformed` or
  `Reserved` carrying the consumer's own error.
- `walk`, `find`, `by_key`, `ancestors` and `distinguished_chain` behave as the
  *Reading* table states, including `ancestors` ending at the root, which is a
  node and not an entry, so its element type is not the entry type.
- A test shows a foreign **directory** is skipped whole, and a malformed one
  halts rather than vanishing. That pair is the failure this design exists to
  prevent and it should be visible in the test names.
- Each test names the model claim it discharges, or says it has none.
- An entry in `docs/formalism-findings.md`.

## Notes

**Walk order is unmodelled, and this is the leaf that owns it.** `operations.qnt`
models reachability and not the depth-first, distinguished-child-first ordering,
so `by_key`'s documented tie-break on a duplicate-key tree — *the first in walk
order* — rests on prose alone. Either implement the order the document states and
test it against hand-built trees, or make the case unreachable. Do not implement
it and describe it as checked: entry 003 already warns that a model can satisfy
a property by construction and look exactly like one that verified it, and this
is the same confusion arriving from the other side.

**Occupancy is decided without following links.** A symbolic link carrying an
entry's name is `Malformed`, not occupying, because `parse` sees what the
listing found and halts at the snapshot before any destination is computed. That
is a snapshot-layer property, so it is this leaf's to get right even though it
only pays off in the mutation leaves.

**Snapshot scope is whole-tree, and that is a decision, not an accident.** It is
why one unparseable name anywhere freezes the whole tree. Narrowing it later is
an invisible refinement; widening the blast radius is not. Do not narrow it here.
