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
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `session-kind-migration-integrate-k29` owns every fix
  and all post-fix verification.

## Done when

- Findings are recorded here with severity, a deterministic reproducer or
  trace, and the threatened contract, or an explicit no-finding result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded Git, jj, and recovery evidence rather than re-running it.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `session-kind-migration-integrate-k29`
owns fixes.
