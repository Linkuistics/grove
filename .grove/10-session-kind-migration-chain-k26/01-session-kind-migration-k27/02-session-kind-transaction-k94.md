# session-kind-transaction-k94

**Kind:** impl

## Goal

Execute a complete migration plan through one fail-closed,
process-interruption-recoverable `MIGRATING-session-kinds` transaction.

## Context

- Consumes the deterministic plan from `session-kind-plan-k93`.
- Binding design: `docs/specs/config-driven-sessions.md` section "Fail-closed
  transaction and recovery" and ADR `promotion-transactions-fail-closed`.
- Hold the universal exclusive Tree access guard for the entire operation.

## Done when

- The witness stages untouched rollback sources, the complete destination, and
  a deterministic source/destination plan before any source path changes.
- Other readers and mutators refuse while the witness exists and identify bare
  `grove` as the recovery path.
- Landing and recovery infer progress from source, staged, and final locations;
  reported pre-commit failures roll back, rollback failure leaves the tree
  blocked, and post-commit recovery verifies finals before removing the witness.
- `FORMAT` is atomically written last, after every final file verifies.
- Deterministic tests exercise every interruption boundary, successful retry,
  rollback, rollback failure, and final-tree mismatch.

## Notes

Use internal fault injection at filesystem transition seams; do not expose
test-only failure controls as user configuration or environment variables.
