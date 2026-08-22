# operations-model-k4

## Goal

Model `ordinal-fs-tree`'s **operations** in Quint, and check that each is total
and that each preserves the structural invariants the previous leaf settled.
Reconcile whatever it finds back into the architecture document.

## Context

- `docs/ordinal-fs-tree/models/structure.als` and whatever `structure-model-k3`
  corrected — the well-formedness predicate this leaf's invariants are stated
  against. Read its findings-log entry before starting; if it moved the data
  model, the document moved with it.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections **Operations**, **How an
  operation runs** and **Refusals**.
- `docs/formalism-findings.md` — append entry 003 before retiring, and this
  entry is the one that tests **H1** (the structural/behavioural split): say
  plainly whether the division of labour held, whether either tool wanted work
  the other had, and where the boundary was actually drawn versus where it was
  planned.
- Put the model at `docs/ordinal-fs-tree/models/operations.qnt`.

## Done when

- `operations.qnt` exists, typechecks, runs, and its invariants hold or their
  counterexample traces have been acted on.
- Every operation is total in the model — each refusal in the document's
  **Refusals** section is a modelled outcome, not an unmodelled assumption.
- Corrections land in the architecture document.
- `docs/formalism-findings.md` carries entry 003, and the operation set is fixed
  well enough for implementation leaves to be cut one per operation.

## What to check

- **`init` is a decision, not a detail.** Empty tree, or arbitrary well-formed
  tree? Entry 001 records that this question is exactly what caught a false
  invariant. Model *both* if they differ, because the difference is the answer to
  whether a hand-edited tree is in scope — and it is.
- **Preservation.** Every action preserves key uniqueness and ordinal
  distinctness from any reachable state.
- **Two-state properties.** These are the claims prose is worst at and traces are
  best at: an `insert` changes the ordinals of siblings at or after the target
  and *nothing else* — no key, label, attribute or descendant moves; a `promote`
  preserves the promoted entry's ordinal *and* key.
- **Shift ordering.** Renames run highest-ordinal-first so each destination is
  vacated before it is needed. Model the plan as an ordered sequence and show
  that any other order collides.
- **Partial application.** The interpreter applies effects one at a time and can
  fail at step *k*. Model that: after a failed apply, either every effect landed
  or none did, and rollback removes only what the run itself created. This is the
  claim the architecture document currently makes in prose and nothing checks.
- **Totality.** Insert past the last sibling; promote a node; promote where the
  domain has no distinguished child; rewrite that changes species; a destination
  already occupied. Each has a stated outcome in the document — each should be a
  modelled transition, so an unmodelled case shows up as a stuck state.

## Notes

`parts` is opaque to the library, so model it as an uninterpreted value with no
structure — a small set of distinct atoms is enough. If an action needs to look
inside `parts` to decide anything, that is a design finding: the seam has leaked
and the library has learned something about the domain it is not entitled to
know.

Do not model the filesystem. The plan interpreter's effects are the abstraction
boundary; below it is I/O and out of scope.

## Decisions (running log)

**Entries need a model-only identity, separate from the key.** Alloy's atoms
gave this free; Quint needs it declared. An `EntryId` behaves like an inode — a
rename moves a *name*, not an entry — and without it a two-state property can
only say "*some* entry has that key", never "*this* entry kept it". Rejected:
identifying entries by key, which the design itself admits may be duplicated.

**Refusals are transitions, not disabled actions.** The tool's own idiom (and
the `quint-modeling` skill's Step 4) expresses a precondition as a guard, which
makes a refusal simply *not happen*. That is the opposite of what this leaf
tests: a refusal modelled as an impossibility can never be shown to be reachable
*or* dead. Every operation is enabled for every argument and returns an outcome,
and totality becomes structural — the algebra returns a `Decision` for every
input, so an unmodelled case would be a missing `match` branch the typechecker
rejects. Six of the eight findings depend on this choice.

**The interpreter is a state machine, not a pure function.** Effects land one at
a time and any one can fail, so every intermediate state is a state the
invariants are evaluated at. Rejected: simulating the apply loop inside a pure
function and asserting over its result, which is cheaper and cannot observe what
an interruption leaves — the question this leaf exists to answer.

**Two answers to `init`, as two instances rather than two initial states.** The
tree starts empty and an arbitrary well-formed tree is *reached* through hand
edits — a human with `mv`, which is the design's own premise. `pristine` and
`hand_edited` then differ in exactly one property, density, which is the answer
to whether a hand-edited tree is in scope. Rejected: a nondeterministic initial
state, which would make the two answers indistinguishable in the output.

**The algebra's destination check is sequential.** Discovered by the checking:
the snapshot-wide check makes highest-first and lowest-first refuse in identical
cases, so the ordering question is vacuous under it. This is a design decision
the document had not made, and it is now made and written down.

**Walk order is out of scope, and the miss is recorded.** Reachability is
modelled; depth-first, distinguished-child-first ordering is not, so `by_key`'s
"first in walk order" tie-break on a duplicate-key tree is unchecked. Recorded
in the model's handoff and in findings entry 003 rather than worked around.

**The ADR test now passes for two decisions, and filing them is a separate
leaf.** Re-applying the three-part test at this close (the root brief scheduled
it): the **single-trait seam** and **no removal operation** now clear all three
limbs — the limb that failed at `plan-k1` was *hard to reverse*, and both are
now settled decisions with two checked models and a 700-line document built on
them. Filing them is blocked on a question the root brief leaves open — which
context maintains an `ordinal-fs-tree` record, since `CONTEXT-MAP.md` requires
one and there are only two. That is a decision about the repo's bounded
contexts, not about this leaf's goal, so it is cut as `records-k5` rather than
answered here. Two decisions were tested and rejected: **locking is invisible**
and **a rename is a rename** both fail *hard to reverse* — neither appears in a
signature, and the architecture document is where they land.

**Implementation leaves are planning's to cut, not this session's.** The
operation set is now fixed and the leaves are precisely stateable, which is the
fog-or-ticket test passed — but a `design` session cutting `impl` leaves has
drifted into planning's job. Externalised as `library-k6`.
