# insert-k11

## Goal

`insert`: add a child at an occupied ordinal, shifting the occupant and every
later sibling up by one. One rename per shifted sibling, each carrying its whole
subtree, plus one create.

## Context

Beyond the brief chain:

- `ARCHITECTURE.md` — the `insert` row of *Operations → Mutating*, *Why the
  shift runs highest-first*, the `insert` refusals, and the invariant *Subtree
  preservation under shift*.
- `operations.qnt` — the `corrupted` and `lowest_first` instances. Both payoffs
  of the ordering rule are reached in `lowest_first` and nowhere else, so that
  instance is what the ordering rule's tests should be derived from.
- `docs/formalism-findings.md` entry 003, the first two findings and the first
  miss. All three bear directly on this operation.
- `src/tree_grow.rs` for prior art on grove's own sibling renumber. Prior art
  only — and note that the current code puts the shift-ordering rule inside
  filesystem code, which is the shape `ARCHITECTURE.md` names and rejects.

## Done when

- `insert` works, targeting by key, with the shift running highest-ordinal-first.
- Its stated refusals are implemented and tested: a target that is not a node; a
  key naming no entry; **inserting past the last sibling**, which is `append`'s
  job and is refused rather than quietly redirected; and **inserting into a gap**
  in a hand-edited level, which the same refusal covers and which no operation
  fills. The gap case has a rationale the document does not give it, and a
  reader hitting it deserves to be told a gapped ordinal can be occupied only by
  hand.
- The plan is checked by folding it through the snapshot, so each destination is
  met in the state the interpreter will meet it — not against the snapshot as
  found. If `interpreter-k10` left this as a snapshot check, fixing it is this
  leaf's first job, because the ordering rule buys nothing without it.
- The half of subtree preservation that is checkable is checked: **an insert's
  plan names no descendant**. The other half is `rename(2)`'s and is assumed;
  say which is which in the test, and do not let an assumption wear a checked
  test's name.
- The ordering rule is tested for the reason that actually applies — the
  **intermediate state**. Highest-first leaves a level merely gapped at every
  step; the other order passes through a duplicate ordinal. Collision is the
  wrong reason and applies only to a tree that already violates key uniqueness.
- Each test names the model claim it discharges, or says it has none.
- An entry in `docs/formalism-findings.md`.

## Notes

**The document's first stated reason for the shift order was wrong**, and the
model is what caught it. A name embeds a tree-unique key, so two siblings never
want the same filename and *no* order collides on a well-formed tree. If a test
here asserts "the other order collides", it is testing the corrected document's
predecessor. The live claim is about what an interruption leaves, and an
interruption is not something a passing test observes — reason about the plan,
which is a value, and assert on its order.

**A shifted node is one directory rename.** Nothing inside it is touched: no
child name, no child key, no file content. A test that rebuilds a subtree, or
walks into one to rename its children, has misread the design and should fail
review.
