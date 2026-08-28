# book-assembly-k18

## Goal

Complete and assemble the book, close the source ledger, prove every mechanical
and crate-level criterion, and commission fresh-context technical review.

## Context

- Inputs: every preceding child of `ordinal-fs-tree-book-k10`, the book-system
  design, validators, repository source/tests/models/docs, and the root contract.
- This leaf owns synthesis and missing cross-cutting explanation, not a rewrite
  of already intelligible slices.

## Done when

- Cross-cutting invariants, model evidence, failure/refusal map, architecture
  summary, and explicit design trade-offs are present where the chapter sequence
  makes them understandable.
- Every page is independently intelligible at its entry point, navigation is
  complete, and repetition/link choices satisfy the logical-integration rule.
- Exhaustive fragment validation reconstructs exactly all fifteen source files
  in the subtree brief, with no extra, missing, duplicated, unresolved, cyclic,
  or unreachable fragment.
- Markdown structure and every local link pass validation.
- The crate's complete existing verification, including CLI contract tests and
  the no-filesystem boundary guard, passes; relevant Alloy/Quint runners are run
  when the book makes claims they uniquely support.
- A technical `review-impl` leaf is appended inside this node as the final act,
  uses the bare stem `book-assembly`, carries
  `**Reviews:** book-assembly-k18`, and instructs the reviewer to inspect the
  committed book against source, tests, models, docs, coverage, and requirements
  without editing or running checks.
- The created technical-review task carries the conditional handoff in the node
  brief: integrate real findings first, then commission editorial review; if no
  technical findings exist, commission editorial review directly.

## Notes

The review leaf is created only after verification evidence and the complete
artifact exist. Its body must be specific enough that the reviewer can identify
the book commit by this handle and distinguish technical accuracy from the later
editorial read.
