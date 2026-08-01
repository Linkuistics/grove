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
