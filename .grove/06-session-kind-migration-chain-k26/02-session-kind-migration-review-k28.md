# session-kind-migration-review-k28

**Kind:** review-impl
**Reviews:** session-kind-migration-k27

## Goal

Adversarially review `session-kind-migration-k27` and record concrete findings for its integration step.

## Context

- Review `session-kind-migration-k27` against the accepted-input table,
  current-format witness, transaction/recovery protocol, and scoped VCS commit
  contract in the spec.
- Attack interruption between every filesystem/index/commit phase, partial
  root misclassification, key or relationship loss, vendor-pair ambiguity,
  collision handling, witness visibility, and unrelated-work preservation.

## Done when

- Findings are recorded here with severity, a deterministic reproducer or
  trace, and the threatened contract, or an explicit no-finding result.
- Both Git and jj paths and at least one recovery path are independently
  exercised through observable tree/VCS outcomes.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `session-kind-migration-integrate-k29`
owns fixes.
