# book-system-k25

**Reviews:** book-system-k6

## Goal

Adversarially review the committed ordinal-filesystem-tree book-system design
for missing requirements, ambiguous formats, misplaced seams, and validator or
authoring contracts that still require redesign downstream.

## Context

- Review the `book-system-k6` commit, especially
  `docs/specs/ordinal-fs-tree-book.md`, against the producer task, `plan-k1`,
  `walkthrough-method-k5`, the `ordinal-fs-tree-book-k10` brief and its eight
  leaf contracts, and the `book-validation-k7` brief and both validator leaves.
- Inspect the frozen fifteen-file source corpus where a design claim depends on
  its actual bytes or structure. Treat repository docs, tests, ADRs, and models
  as evidence, not as substitutes for a self-contained book contract.
- A first in-session adversarial read caused the producer to tighten the shared
  byte lexer, normative tables, diagnostic records, page ownership, repository
  loading, scope enumeration, and prose-example placement. Re-derive whether
  the committed design is sufficient; do not treat that list as findings to
  preserve or as proof that adjacent gaps are closed.
- This is findings-only. Do not edit the design, source, tests, task tree, or
  other production artifacts, and do not run build, test, lint, or format
  commands.

## Done when

- The review tries to disprove that the raw-Markdown grammar has one
  unambiguous byte interpretation, including directive/fence contexts, exact
  source preservation, recursive expansion, deferral, ownership and
  reachability.
- It mechanically checks that the ownership blocks form gapless,
  non-overlapping partitions of exactly the fifteen source roots and 6,618
  lines, and that scoped states can progress through all seven source-owning
  slices before final assembly requires zero deferrals.
- It tests the page table, conceptual order, low-resolution and full-resolution
  insert tours, navigation, early-use ledger, source/fragment indexes, and prose
  review questions against every book-authoring leaf's contract.
- It tests whether fragment and Markdown validation share one lexical seam,
  whether their CLI, scope, input loading, diagnostics, JSON, and link behavior
  are deterministic, and whether `book-validation-k7` can implement them
  without deciding missing policy.
- Every finding names severity, exact artifact location, violated requirement,
  and a concrete repair. Contract ambiguities are distinguished from accepted
  trade-offs and preferences; absence of findings is stated explicitly.
- If findings warrant changes, an `integrate-review-design` leaf with bare stem
  `book-system` is placed according to Grove's directory-local adjacency rule
  and carries `**Integrates:** book-system-k25`. If no findings exist, no
  integration leaf is created.

## Notes

The review is fresh-context and inspection-only. It reviews the committed
artifact and its present requirements rather than validating the producer's
reasoning or repeating the producer's verification commands.
