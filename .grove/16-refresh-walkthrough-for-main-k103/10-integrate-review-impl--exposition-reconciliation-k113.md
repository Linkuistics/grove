# exposition-reconciliation-k113

**Integrates:** `exposition-reconciliation-k112`

## Goal

Triage the findings recorded by `exposition-reconciliation-k112` against the
refreshed `docs/ordinal-fs-tree/book`, apply the ones that are real, and carry
the post-fix verification the review was forbidden to run.


## Context

- Read the findings from the review's own commit rather than from this file.
  The review classified each item as a technical error, an editorial defect, or
  a visible trade-off, and recorded the contract it holds each one against;
  rejecting an item on its merits is a legitimate outcome and is what makes
  this a triage rather than a work list.
- The review is inspection-only, so no book file has been edited since
  `exposition-reconciliation-k110` committed it and
  `book-validation-diagnostic-fixtures-k111` landed beside it. Every cited
  `path:line` still points where the reviewer left it, provided no other work
  intervenes.
- The book contract is unchanged: seventeen fixed source roots, 8,720 owned
  lines, exact byte-for-byte recursive expansion, complete local navigation,
  and declarative, direct, self-contained exposition.

## Done when

- Every finding is either applied or rejected with a stated reason; none is
  left silently unaddressed.
- Any edit preserves exact source reconstruction — no fragment body, ownership
  range, or ledger count changes as a side effect of an exposition or
  navigation fix.
- `book-check --final --check all` passes, and the checks the refresh node
  names remain green.
- The applied changes are committed against this leaf's handle.

## Notes

The review found no technical error, so the expected shape of this session is
small and navigational rather than a re-authoring pass. Resist widening it:
work that does not serve this leaf's goal goes to the tree as its own leaf.
