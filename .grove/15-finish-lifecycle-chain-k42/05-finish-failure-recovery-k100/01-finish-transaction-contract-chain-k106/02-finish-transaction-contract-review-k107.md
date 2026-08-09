# finish-transaction-contract-review-k107

**Kind:** review-design
**Reviews:** finish-transaction-contract-k105

## Goal

Adversarially review `finish-transaction-contract-k105` and record concrete findings for its integration step.

## Context

- Binding artifacts: `docs/specs/config-driven-sessions.md` sections
  "Pre-commit transaction and recovery", "Crash and retry semantics", and
  "Scoped Git and Jujutsu commits"; ADR
  `task-tree-transactions-fail-closed`; glossary terms Finish transaction,
  Complete finish cycle, and Tree access lock.
- The producer's first in-session reviewer already disproved four assumptions;
  this scheduled review must attack the integrated corrections, not merely
  repeat the old findings.
- Test genuine commit proof when the generated finish leaf was never committed;
  unexpected Git/jj topology, message, or tree deltas; reserved-name collisions,
  foreign entries and symlinks; index restore/activation failure; atomic
  root-to-quarantine finalization; quarantine disposal failure; and disabled
  Git hooks.
- Review is inspection-only. Record findings here; the integration leaf owns
  every artifact change.

## Done when

- Findings have severity, an exact Git/native-jj/colocated-jj or filesystem
  failure sequence, and the threatened contract, or the task records an
  explicit no-finding result.
- The review proves that `Committed`, `Not committed`, and `Recovery pending`
  are mutually safe across every transition and that no ambiguous state can be
  selected, treated as fresh before commit proof, or rolled back over changed
  history.
- The review checks that post-commit quarantine is cleanup-only and cannot
  become a lifecycle receipt or enter the scoped deletion commit.
- No spec, ADR, glossary, source, or test artifact is changed.

## Notes
