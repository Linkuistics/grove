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
- The in-scope source still matches the node's frozen corpus. If an accepted
  source change landed during authoring, every affected slice has been updated
  and exhaustive fragment validation has been rerun against the new bytes.
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
- `CONTEXT-MAP.md` and `docs/ordinal-fs-tree/CONTEXT.md` register the completed
  book as an artifact of the ordinal-filesystem-tree context.

## Notes

The review leaf is created only after verification evidence and the complete
artifact exist. Its body must be specific enough that the reviewer can identify
the book commit by this handle and distinguish technical accuracy from the later
editorial read.

## Decisions (running log)

The assembly page owns no source fragments. It synthesises the already-owned
source into six local surfaces: the four-stage architecture, invariant scope,
failure and refusal outcomes, model evidence and limits, explicit design
trade-offs, and reproducible final verification. This closes the book without
duplicating source or making an internal link carry required context.

Model claims are stated as bounded evidence rather than proofs of the Rust or
filesystem implementation. The page distinguishes Alloy's single-state shape
checks, Quint's reachable transition checks, the behaviours each model omits,
and the executable crate tests that cover those omitted boundaries.

The completed book is registered as an ordinal-fs-tree artifact by linking it
from the context map's ordinal-fs-tree entry and from the context glossary's
opening artifact paragraph. No new term or durable architectural decision is
introduced, so neither the glossary vocabulary nor the ADR set changes.
