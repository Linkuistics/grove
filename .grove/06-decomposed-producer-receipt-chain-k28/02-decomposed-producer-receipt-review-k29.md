# decomposed-producer-receipt-review-k29

**Kind:** review-design
**Reviews:** decomposed-producer-receipt-k20

## Goal

Adversarially review `decomposed-producer-receipt-k20` and record concrete
findings for its integration step.

## Context

Review the updated `review-target-receipts` ADR and the producer-handoff,
advisory-diversity, module-interface, test-seam, compatibility, and out-of-scope
sections of `docs/specs/doubt-grove-review-mechanics.md`. Challenge whether the
node-closing handoff target and permanent-key generation preserve factual pick,
node reopen, restart, non-blocking metadata, and direct-producer compatibility.

## Done when

- The source-session and generation invariants are tested against nested close
  cascades, supported reopen, reordered children, and stale or hand-edited
  receipts.
- Guard scope and launch-window forecast semantics are checked for races.
- Receipt materialization is checked for lost Markdown edits around `DONE`.
- The choice of one handoff target, rather than every contributing target, is
  challenged as an explicit advisory trade-off.
- Findings are severity-ranked and recorded here for
  `decomposed-producer-receipt-integrate-k30`.

## Notes

Produce findings only; do not implement fixes.
