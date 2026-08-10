# finish-task-root-identity-integrate-k144

**Kind:** integrate-review-impl
**Integrates:** finish-task-root-identity-review-k143

## Goal

Apply the verified findings from `finish-task-root-identity-review-k143` while preserving the reviewed artifact's contract.

## Context

- Integrates F1–F4 from `finish-task-root-identity-review-k143`; F5 remains the
  explicitly accepted optimization rather than a contract fix.
- The common cause of F1/F2 is phase ownership: repository-preparation
  auxiliaries must either travel into a ready witness or be explicitly aborted
  before task-tree mutation.
- A narrow post-fix review additionally found that rollback cleanup must remain
  best-effort after entries are restored, and that synchronous witness
  materialization failures cross the same ownership boundary.

## Done when

- Both task-root refusal gates leave plain-Git preparation retryable under the
  same attempt identity without overwriting late-staged work.
- Colocated-Jujutsu abort restores the pre-preparation Git index and removes both
  attempt-bound auxiliary roles, with a mutation-sensitive regression test.
- Recovery types do not claim an unused task-root descriptor invariant.
- Rollback remains recoverable when advisory auxiliary disposal refuses a
  substituted artifact.
- Synchronous witness creation failures remove only the exact partial layout
  just created and abort repository preparation; crash-consistent publication is
  externalized as `finish-witness-materialization-recovery-k146`.
- Focused finish tests and the full locked suite pass from the final tree.

## Notes

`driver-lease-readiness-flake-k145` records the unrelated process-test flake
observed during full-suite verification.
