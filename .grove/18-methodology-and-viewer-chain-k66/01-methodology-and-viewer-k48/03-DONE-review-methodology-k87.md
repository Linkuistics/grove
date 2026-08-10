# review-methodology-k87

**Kind:** impl

## Goal

Reconcile Grove and doubt guidance around one-review ownership, scheduled review
chains, and the removal of receipt-era target inference.

## Context

- Depends on `session-kind-methodology-k86`.
- Primary artifacts: `content/SKILL.md`, `content/TASK-FORMAT.md`,
  `content/driving.md`,
  `plugins/linkuistics/skills/doubt-driven-development/SKILL.md`, and
  composition-guidance tests.
- Keep review inspection-only and integration responsible for fixes and
  post-fix verification.

## Done when

- Grove and doubt state the same mandate-based one-review ownership predicate
  and scheduled-chain escalation boundary.
- Producer launch receipts, generations, target comparisons, and diversity
  warnings are absent while stable `Reviews` / `Integrates` and promotion
  remain documented.
- Canonical guidance, composition guidance, `cargo fmt --check`, and
  `cargo test --locked` checks pass.

## Notes

This increment can land after the taxonomy because it changes orchestration
policy without reopening lifecycle or filename behavior.
