# decomposed-producer-receipt-k20

**Kind:** design

## Goal

Define the producer-target receipt semantics for a reviewed producer that has
been decomposed into a multi-session node.

## Context

`leaf-retire` materialises a receipt only when the retiring leaf has one sibling
whose `**Reviews:**` relationship names that leaf. A producer leaf promoted or
proactively created in a review chain may later become a brief-carrying node;
its last child closes the node implicitly, so the outer review launches with
`producer-receipt-missing`. This surfaced while retiring
`composition-guidance-k17`; the warning is advisory, so current lifecycle
correctness is intact, but there is no obvious single historical target for an
artifact produced across several sessions.

## Done when

- The design decides whether a decomposed producer should remain deliberately
  uncheckable or record a well-defined target, and names whose target that is.
- The choice preserves the factual-pick identity checks, node-close contract,
  task-tree-only state, and restart behavior.
- Any implementation/test/documentation work is decomposed into later leaves
  rather than absorbed into this design session.

## Notes

Do not infer the producer from position or filename suffix. Reconcile the
review-target-receipts ADR in place if the decision changes its current scope.

## In-session doubt reconciliation

- **Actionable:** review receipt/source/generation validation now belongs to the
  same shared tree read as the structured routing peek. The retained result is
  still a forecast; the session's factual pick wins after a launch-window tree
  change.
- **Actionable:** a prepared receipt stores facts, not pre-`DONE` task content.
  Materialisation re-reads the review task after `DONE` before replacing its
  receipt line, so edits made during preparation are preserved.
- **Visible trade-off:** a receipt is evidence, not scheduling authority.
  Reopening a producer after its review is terminal does not reactivate that
  review; new review work must be named explicitly.
- **Contract clarification:** the writer establishes which factual descendant
  closed a node; launch can validate only the terminal shape, descendant source,
  and generation. Hand-edited Markdown remains advisory and unauthenticated.
- **Contract misread:** zero or duplicate review claimants were already
  uncheckable in the spec; the ADR now states the exact-one leaf claimant rule
  explicitly.
