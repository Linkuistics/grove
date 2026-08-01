# decomposed-producer-receipt-implementation-review-k26

**Kind:** review-impl

**Reviews:** decomposed-producer-receipt-implementation-k25

## Goal

Adversarially review the decomposed-producer receipt implementation for
lifecycle correctness, stale-generation safety, compatibility, and fidelity to
the design.

## Context

Review `decomposed-producer-receipt-implementation-k25` against
`decomposed-producer-receipt-k20`, the updated `review-target-receipts` ADR, and
the receipt contract in `docs/specs/doubt-grove-review-mechanics.md`. Produce
findings only.

## Done when

- Direct leaves, one-level and nested node-close cascades, supported reopen,
  legacy receipts, and advisory failure paths are challenged.
- Factual-pick/worktree/routed-session checks and DONE-first ordering are
  verified at public seams.
- Documentation and code use producer identity, source session, and producer
  generation consistently.
- Findings are severity-ranked, reproducible, and recorded here for
  `decomposed-producer-receipt-implementation-integrate-k27`.

## Notes

Do not broaden this into the distinct decomposed-review relationship-carrier
finding.
