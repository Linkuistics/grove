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
