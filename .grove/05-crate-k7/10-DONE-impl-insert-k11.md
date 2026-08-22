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

## Decisions (running log)

**The sequential destination check was already in place, so this leaf's stated
first job was a no-op.** `interpreter-k22` landed `Plan::guarded` as a *fold*
through the snapshot — `src/plan.rs`'s `refusal`, with
`a_plan_is_folded_through_the_snapshot_and_not_checked_against_it` beside it —
rather than as a snapshot-wide check. Verified by reading both before writing
anything, because the ordering rule buys nothing without it. Recorded so a later
reader of this task file does not go looking for the change it asks for.

**`Refusal::NoOccupantAtOrdinal` carries the level's greatest ordinal, so one
refusal can give two pieces of advice.** The model has one outcome and two
witnesses — `wit_insertPastTheEnd` and `wit_insertIntoAGap` — and the two want
opposite advice: past the last sibling, call `append`; into a gap, no operation
fills it and it can be occupied only by hand. Carrying `greatest: Option<Ordinal>`
lets `Display` decide which is true rather than offering the reader a fork, and
`None` (a level with no positioned children) falls to the `append` half, where
every ordinal is past the last sibling.

**The ordering rule is tested through the plan's intermediate states, not
through a collision.** A test asserting *the other order collides* would be
testing the architecture document's corrected predecessor — findings entry 003 is
where the model contradicted it. So the tests fold the plan's landings over the
level and assert every intermediate state has distinct ordinals, with the same
landings replayed lowest-first as the control that reaches a duplicate
(`wit_shiftTransientlyDuplicatesAnOrdinal`, live only in the `lowest_first`
instance). An interruption is not something a passing test observes; a plan is a
value, so every state an interruption could stop at can be read off it.

**The content-for-a-node refusal belongs to `insert` too, and the document had
named only `append` and `append_many`.** Not a model disagreement — content is
unmodelled in both models by design — so `ARCHITECTURE.md`'s *Refusals* bullet
was corrected to state the rule for every operation that creates an entry, with
the reason (a node is a directory and has nowhere to hold bytes) rather than a
list that the next operation would fall off. Recorded in
`docs/formalism-findings.md` entry 012 as a place the models could not help.

**A review chain is cut.** `promote-k12` and `rewrite-k13` both build on the
`MoveTo` path and on the plan-ordering conventions this leaf established, and
every impl leaf in this node so far has had a review find something. The
in-session allowance is left unspent, as a producer with a scheduled review
spends none.
