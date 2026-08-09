# post-teardown-restart-contract-review-k103

**Kind:** review-impl
**Reviews:** post-teardown-restart-contract-k102

## Goal

Adversarially review `post-teardown-restart-contract-k102` and record concrete
findings for its integration step.

## Context

- Review against `docs/specs/config-driven-sessions.md` sections "Fresh tree",
  "Existing live tree", and "Crash and retry semantics", plus ADR
  `one-live-driver-per-working-tree`.
- Attack accidental finish inference, hidden durable state, VCS-specific
  classification drift, loss of the child's real no-signal outcome, stale epoch
  access after `plan-k1` reuse, false-positive retry proof from older teardown
  history, jj successor-working-copy mistakes, and methodology wording that
  treats task-root absence itself as a receipt.
- Inspect the producer's committed diff and recorded evidence. Produce findings
  only; implementation belongs to the integration leaf.

## Done when

- Findings cite exact source, test, methodology, or design locations and name
  the threatened contract, or record an explicit no-finding result.
- Git and jj restart shapes, including unrelated successor work in jj, are
  considered without inventing a history-based lifecycle discriminator.
- No production, test, or documentation artifact outside this task file is
  changed.

## Notes
