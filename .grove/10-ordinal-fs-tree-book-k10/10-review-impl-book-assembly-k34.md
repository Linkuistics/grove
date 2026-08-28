# book-assembly-k34

**Reviews:** book-assembly-k18

## Goal

Perform a fresh-context technical review of the complete committed
`ordinal-fs-tree` code-walkthrough book against its authoritative source,
executable evidence, formal models, repository contracts, coverage ledger, and
Grove requirements.

Read the whole book at `docs/ordinal-fs-tree/book/`, not only the producer
commit's added assembly page. Review the artifact as committed by
`book-assembly-k18`; do not edit files and do not run verification commands.

## Context

- Requirements: `.grove/BRIEF.md`,
  `.grove/10-ordinal-fs-tree-book-k10/BRIEF.md`, and
  `.grove/10-ordinal-fs-tree-book-k10/09-DONE-impl-book-assembly-k18.md` in the
  producer commit.
- Book-system contract: `docs/specs/ordinal-fs-tree-book.md`.
- Authoritative production corpus: the fifteen files listed in the node brief
  and in `docs/ordinal-fs-tree/book/source-index.md`.
- Technical design and vocabulary: `docs/ordinal-fs-tree/{CONTEXT,ARCHITECTURE,CLI}.md`,
  the ordinal-fs-tree ADRs registered in `CONTEXT-MAP.md`, and
  `docs/formalism-findings.md`.
- Executable evidence: the crate's unit, integration, CLI-contract, conformance,
  fault-injection, and no-filesystem-boundary tests.
- Formal evidence: `docs/ordinal-fs-tree/models/{structure.als,operations.qnt}`
  and their runners' stated claim sets and limitations.
- Mechanical coverage evidence: the complete fragment graph, ownership blocks,
  fragment index, early-use ledger, and the final assembly verification record
  in `08-invariants-and-trade-offs.md`.

Inspect technical accuracy, completeness, source-to-prose correspondence,
invariant scope, refusal/error classification, concurrency and recovery limits,
model attribution, CLI behavior, exact fragment ownership, and
self-containedness of required technical context. Distinguish source-backed
facts from model-backed evidence and from stated filesystem assumptions.

## Done when

- Every finding names an exact book path and line, states the contradicted
  source, test, model, document, ledger row, or requirement, and explains the
  technically correct replacement or missing content.
- The review checks all eight numbered pages plus contents, concept index, and
  source index; it does not treat the assembly page as the whole artifact.
- Findings are limited to technical accuracy and completeness. Clarity,
  ordering, cognitive load, repetition/link choices, and prose style belong to
  the later editorial review unless they make a technical statement ambiguous
  or non-self-contained.
- No repository file is edited and no verification command is run.
- If real findings exist, append an adjacent `integrate-review-impl` leaf with
  bare stem `book-assembly`, carrying `**Integrates:** book-assembly-k34`, and
  require it to verify and fix every finding, rerun final book validation plus
  the complete crate and relevant model checks, then commission the editorial
  reviewer as its final act.
- If no technical findings exist, append the editorial `review-impl` leaf
  directly with bare stem `book-assembly`. Its `**Reviews:**` line names
  `book-assembly-k18`, its body explicitly scopes the read to the whole book,
  and it reviews clarity, concept order, cognitive load, self-containedness,
  repetition/link choices, and the declarative prose contract without editing
  or running checks.

## Notes

The editorial reviewer must name the session that last wrote the book. If a
technical integration is created, that integration handle replaces
`book-assembly-k18` in the editorial leaf's `**Reviews:**` line. An editorial
integration leaf is created only for real editorial findings and reruns the
whole-book and crate checks after fixing them.
