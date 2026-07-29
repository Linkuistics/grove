# chain-node-integrate-k11

**Kind:** integrate-review-impl

## Goal

Triage `chain-node-review-k10`'s findings and apply the real ones.

## Context

This is a **flat** chain, so nothing links these leaves but adjacency — read
`chain-node-k9` for the shape being built and `chain-node-review-k10` for what was
looked for.

## Done when

Every finding is classified and dispatched, per the receiving-review discipline:
*a contract stated unclearly* (fix the contract — here, the design record in
`docs/specs/task-kind-taxonomy.md` or `docs/adr/task-tree-scheme.md`), *a real
issue* (fix the code or prose), *a real trade-off* (accept it visibly, in the
record), or *noise raised for want of context*. Verify each rather than
performatively agreeing.

The build and tests are green, and the verbs are exercised end-to-end against a
real `.grove/` fixture.

## Notes

**A finding against the *design* is legitimate here and should go to the design
record, not be worked around in code.** The decision was made in one session
(`chain-as-node-k7`) reversing a thrice-recorded prior decision; if the
implementation surfaces something that decision missed, editing
`docs/specs/task-kind-taxonomy.md` in place is the correct response — the ADR/spec
set is a minimum coherent set describing the current design, never an append-only
log. Reconcile `CONTEXT.md` and any citation the rework leaves dangling.

**Retiring this leaf closes the chain.** Under the shape being built it would also
close a node; under the flat shape it was cut in, it closes nothing structural.
