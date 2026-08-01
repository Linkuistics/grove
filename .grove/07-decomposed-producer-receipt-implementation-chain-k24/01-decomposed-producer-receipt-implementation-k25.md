# decomposed-producer-receipt-implementation-k25

**Kind:** impl

## Goal

Implement the reviewed producer-receipt semantics for a producer that closes as
a multi-session decomposition node.

## Context

Bootstrap from `decomposed-producer-receipt-k20`, the updated
`review-target-receipts` ADR, and the receipt sections of
`docs/specs/doubt-grove-review-mechanics.md`. Preserve the existing advisory
DONE-first handoff and explicit `Reviews` relationship; do not infer from names
or positions.

## Done when

- Receipt preparation recognises direct leaf producers and newly closing
  reviewed decomposition ancestors while the factual leaf is still live.
- New receipts name the reviewed producer, the factual source session, and the
  producer generation as designed; legacy direct-leaf receipts still read.
- Supported node reopen/reclose cannot make a stale receipt compare as current.
- The structured routing peek validates and returns review evidence under its
  existing shared guard; launch performs no second unlocked metadata read.
- Post-`DONE` materialisation re-reads the review task and preserves edits made
  after preparation; receipts never reactivate terminal review work.
- A producer node closed by pruning remains deliberately uncheckable.
- Focused lifecycle/relationship/loop tests cover nested close cascades,
  generation changes, failure-after-DONE, restart, and unchanged non-blocking
  review launch.
- The glossary, methodology, architecture, CLI-facing docs, and spec describe
  the implemented current-state contract without creating another authority.

## Notes

Keep review-leaf decomposition (`doubt-grove-implementation-review-k8 F6`)
outside this increment; it is a distinct relationship-carrier problem.
