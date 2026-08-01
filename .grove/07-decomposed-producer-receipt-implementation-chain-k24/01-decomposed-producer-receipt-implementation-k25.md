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
  reviewed decomposition ancestors while the factual leaf is still live,
  regardless of whether that closing descendant is a producer, review, or
  integration kind.
- New receipts name the reviewed producer, the factual source session, and the
  independently computed producer generation as designed. Readers ignore unknown
  keys; legacy direct-leaf receipts still read, while legacy node receipts are
  uncheckable.
- Supported node reopen/reclose cannot make a stale receipt compare as current.
- The structured routing peek validates and returns review evidence under its
  existing shared guard, nests historical routing under `producer-target`, and
  launch performs no second unlocked metadata read.
- Post-`DONE` materialisation re-reads the review task and preserves edits made
  after preparation. It replaces receipts only in live reviews; a terminal
  review remains byte-identical, is never reactivated, and yields a skip
  diagnostic with reason `review-terminal`. A close cascade has at most one live
  linked review.
- A producer node closed by pruning remains deliberately uncheckable; tests and
  guidance distinguish pruning the producer (review runs next) from pruning the
  enclosing chain (all live steps close).
- Relationship/wire tests cover zero, duplicate, malformed, and non-leaf
  claimants; every required field and accepted type; unknown keys; both legacy
  directions (new-reader derivation and the pre-rule strict reader's malformed
  result for new fields); producer/relationship mismatch; and
  source-session/generation divergence.
- Lifecycle tests cover nested close cascades, at-most-one live linked review,
  reorder-stable and reopen-changed generation, failed `DONE` with no receipt,
  post-`DONE` write failure, preservation of an edit made after preparation,
  restart, pruning scope, and terminal-review byte preservation.
- Loop tests cover checkable source-session warning rendering, nested
  `producer-target` evidence, launch-window notice scope, historical
  configuration changes, and unchanged non-blocking review launch.
- Reconcile the exact canonical surfaces without creating another authority:
  `CONTEXT.md`; `content/SKILL.md`, `content/driving.md`, and
  `content/TASK-FORMAT.md`;
  `plugins/linkuistics/skills/doubt-driven-development/SKILL.md`;
  `docs/ARCHITECTURE.md`; `grove-llm --help`, `grove-llm kind --help`, and
  `grove-llm leaf-promote-chain --help`; `docs/USAGE.md`;
  `docs/CONFIGURATION.md`; and the spec.

## Notes

Keep review-leaf decomposition (`doubt-grove-implementation-review-k8 F6`)
outside this increment; it is a distinct relationship-carrier problem.
