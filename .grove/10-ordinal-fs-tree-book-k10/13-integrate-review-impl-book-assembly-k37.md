# book-assembly-k37

**Integrates:** book-assembly-k36

## Goal

Verify and fix every finding the editorial review recorded, then rerun the
complete book, crate, Quint, and Alloy checks and record their actual results.

## Context

- The findings: the `## Findings` section of
  `.grove/10-ordinal-fs-tree-book-k10/12-DONE-review-impl-book-assembly-k36.md`.
  Twelve findings, each naming an exact book path and line, the contract clause
  or cross-page statement it violates, and the correction.
- Prose contract: `docs/specs/ordinal-fs-tree-book.md`, sections
  *Self-containedness*, *Local context and repetition*, *Source-fragment
  introductions*, *Worked examples*, and *Early-use ledger*. These are the
  normative clauses the findings cite; the review invented no criteria.
- The two prior steps: `10-DONE-review-impl-book-assembly-k34.md` and
  `11-DONE-integrate-review-impl-book-assembly-k35.md`. Findings 1 and 2 are the
  editorial residue of that review's findings 6 and 3 — the technical fixes were
  correct and are not to be reverted.
- Authoritative production corpus: the fifteen files in the node brief and in
  `docs/ordinal-fs-tree/book/source-index.md`. The corpus is frozen. No file
  under `crates/ordinal-fs-tree/` is modified by this session; a defect found in
  crate source is externalised as a leaf beside this one.

## Done when

- Every one of the twelve findings is independently verified against the book
  text and the specification clause it cites, then fixed or — with the reason
  written into the running log — argued down. Verify before fixing: a finding
  accepted without checking is the same defect the review exists to catch.
- Finding 1's three corrections are applied and then checked against *every*
  page that prints a tree, not only the three named, so no page is left
  asserting a continuity the display breaks. If the root spelling is unified,
  all four dependent strings on `04-read-path.md` change together.
- Finding 2 raises all seven named page-06 introductions to answer the five
  questions in *Source-fragment introductions*. Before finishing it, re-run the
  same enumeration over all 68 literal fragments on all eight pages rather than
  over the review's list of seven: the review reached its list by enumerating
  the whole surface, and the fix is complete only if the same sweep comes back
  clean. This is the exact narrowing that left finding 2 behind.
- Finding 4's correction reduces the duplicated enforcement split on one page
  only, leaves both seven-obligation lists intact, and either adds the
  `conformance` early-use row or removes the behavioural claim that would
  require it. If a row is added, `source-index.md#early-uses` keeps its
  documented sort order and its `Status` is correct for a resolved owner.
- Findings 3, 5, and 11 change headings, anchors, and index entries. Every
  moved or added anchor keeps a matching `concept-index.md` entry, no anchor ID
  collides, no heading level is skipped, and no local link is left dangling.
- Finding 12 is either applied or closed as an accepted trade-off with the
  reason in the running log. Closing it is a legitimate outcome; leaving it
  unaddressed is not.
- The final book validator in `--final` mode, the complete crate verification
  including its doc tests, and both the Alloy and Quint runners are run, and
  `08-invariants-and-trade-offs.md#final-verification` states what actually
  happened — including any harness caveat, at whatever length finding 12 is
  settled at.
- No production source under `crates/ordinal-fs-tree/` is modified.

## Notes

Every finding is a prose, heading, anchor, or index change. None requires a
fragment boundary to move, so the fragment graph, the ownership ledger, the
per-slice line totals, and the 6,929 total should be unchanged at the end; if a
fix appears to require moving a boundary, that is a signal to re-read the
finding rather than to move it.

Findings 6 and 8 are single-sentence rewrites whose correct wording already
exists elsewhere in the book — `08-invariants-and-trade-offs.md:98-103` for the
highest-first rationale, and the four enumerated sentences at `08:174-179` for
the omission list. Prefer aligning to the existing precise statement over
inventing a third phrasing.

Finding 9 is a readability rewrite of six introductions that already satisfy the
contract. It is the lowest-value item here and the easiest to get wrong: the
five answers must survive the split. If it cannot be done without losing one,
leave the introductions as they are and say so in the running log.

This session may spend one narrow reviewer if a fix needs judgement the text
cannot settle. Substantial rework of a page belongs in a new producer chain
beside this leaf rather than inside it.

If the checks all pass and every finding is settled, this node's `Done when` is
met and no further review step is created: the editorial review is the last one
the node brief schedules.
