# tui-k7

**Integrates:** tui-k6

## Goal

Triage the `review-planning` findings recorded in `tui-k6`'s own commit and
apply the real ones to the planned subtree — the root brief's design outline
and the implementation leaves `tree-viewer-k3`, `markdown-viewer-k4` and
`live-viewer-k5` — so the first implementation session opens on a plan that
fits one session and names every obligation it carries.

## Context

- Read the findings from `tui-k6` (`.grove/03-DONE-review-planning--tui-k6.md`
  in this tree; the review's commit names the handle). They are the reviewer's
  list, not this leaf's charter: reject a finding on its merits where the
  evidence does not hold, and say why in the running log.
- The producer is `tui-k2`; the root brief owns the requirements and test
  seams, which are settled and not reopened here.
- The findings cite `docs/specs/walkthrough-books.md`,
  `docs/ordinal-fs-tree/ARCHITECTURE.md`, `docs/ARCHITECTURE.md` (§Tree access
  lock), `crates/grove-llm/tests/tree_lock.rs`,
  `crates/book-validation/tests/corpus_validation.rs`, `release.toml` and
  `CHANGELOG.md`. Verify each against current source before acting on it.

## Done when

- Every finding has a disposition in this leaf's running log: applied,
  rejected with a reason, or narrowed.
- The first implementation leaf, as it now stands, fits one focused session
  and is runnable on its own; where the review's recommended cut is taken,
  the new leaves are in tree order with their own bodies and the root brief's
  decomposition list is reconciled.
- Every implementation leaf that edits a book-reconstructed source names the
  affected walkthrough book(s), the changelog, and any release metadata the
  new crate needs.
- Recorded design rules the plan reverses are named in the plan together with
  where the reversal is written down.
- Behavior the review found unspecified (node outcome labels, in-process
  state, restoration evidence) is specified in the root brief or the owning
  leaf.
- No production or test code is written here; this leaf edits `.grove/` only.

## Notes

The review sits at `03`; this leaf was inserted ahead of the first
implementation sibling so the plan is reconciled before consumers build on it.
Leaf paths shifted by one position on insertion; handles did not.
