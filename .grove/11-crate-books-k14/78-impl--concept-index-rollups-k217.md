# concept-index-rollups-k217

## Goal

Bring every book's `concept-index.md` under the roll-up check
`ledger-rollup-check-k207` built, or record why it cannot be — the one surface
left stating ledger-derived figures with nothing re-deriving them.

## Context

- **The check exists and this surface is outside it.** `k207` added the
  `<!-- rollup «quantity» -->` directive and `F011`
  (`docs/specs/walkthrough-books.md`, *Ledger roll-ups in prose*): one or more
  directives on their own lines mark the paragraph below, and each declared
  quantity's derived figure must occur in it in digits. Twelve quantities, four
  of them taking an `of="…"` argument.
- **A concept-index row is a list item, not a paragraph.** The directive's scope
  is the contiguous nonblank block after the run, and an HTML comment between two
  list items ends the list in rendered Markdown. So the mechanism as built cannot
  mark these without changing how the page renders.
- **What is uncovered, exactly.** Rows such as `grove-llm`'s *Twenty-five
  ownership blocks and fourteen early-use rows, closed* and `keyed-launch`'s
  *Twenty blocks over nine roots, and the three splits that explain the count* and
  *Ten early-use rows, nine required by the manifest and one added*. Enumerate the
  whole class rather than working from that sample — the same condition may sit in
  a `README.md` contents line or a chapter's own prose.
- **Every one of them was true at `k207`.** This is a structural gap rather than a
  standing defect: a second copy of a figure the assembly chapter now states under
  a mark, free to go stale on its own.

## Done when

- The class is enumerated across every book, not sampled, and each instance is
  covered or explicitly excluded with its reason.
- Either the directive gains a scope that reaches a list item without changing
  what the page renders as, or the rows are restated so a marked paragraph holds
  them, or the specification records that this surface is deliberately unheld and
  says what stands in its place.
- Whatever is chosen has been **watched to fail**: mutate a ledger and see the
  concept-index claim go red, then restore.
- `bash scripts/check.sh` is green.

## Notes

**Do not widen the check's subject.** `k207`'s scope decision holds: only
ledger-derived figures. A concept-index row that summarises an argument is not a
roll-up and is not this leaf's business.

**Prose and tooling only, so the frozen-corpus rule does not bite.**

## Decisions (running log)
