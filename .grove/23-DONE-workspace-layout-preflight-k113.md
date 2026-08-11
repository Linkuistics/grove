# workspace-layout-preflight-k113

**Kind:** design

## Goal

Settle when Grove validates that a workspace can supply the untracked,
same-device control-directory quarantine required for atomic finish cleanup, so
an unsupported layout does not first become visible at teardown.

## Context

- Externalized from `finish-transaction-contract-review-k107` F6 while
  integrating `finish-transaction-contract-integrate-k108`.
- The finish transaction already preflights the quarantine before tree mutation
  and safely refuses with the live tree unchanged. The remaining concern is
  lifecycle usability: a linked Git worktree may place its canonical worktree
  administration directory on a different filesystem from the working tree,
  and the current design discovers that only after the workstream is complete.
- Coordinate with the existing workspace-control resolver, driver lease,
  root-init/config-validation ordering, and finish-time revalidation. Do not add
  a second lifecycle command, durable capability marker, or fallback that makes
  post-commit root removal non-atomic.

## Done when

- The durable design states the earliest safe validation point, the diagnostic,
  and why finish must still revalidate a layout that can change after startup.
- Git main/linked worktrees, native jj, and colocated jj have explicit supported
  and refused layout cases without weakening task-tree non-mutation on failure.
- The minimum coherent spec/ADR/glossary set and implementation decomposition
  are reconciled, with acceptance seams for early refusal and finish-time
  revalidation.

## Notes

This is a lifecycle-support decision, not part of the finish transaction's
commit classification or rollback state machine.
