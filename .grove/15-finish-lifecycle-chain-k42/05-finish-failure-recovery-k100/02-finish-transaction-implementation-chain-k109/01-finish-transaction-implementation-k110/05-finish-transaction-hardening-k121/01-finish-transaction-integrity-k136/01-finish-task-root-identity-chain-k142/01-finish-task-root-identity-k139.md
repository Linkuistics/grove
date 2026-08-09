# finish-task-root-identity-k139

**Kind:** impl

## Goal

Carry the exact no-follow `.grove/` directory opened by preflight across
repository preparation, and reject a substituted path at the preparation and
evacuation phase gates before task-tree mutation.

## Context

- The current preflight opens and identity-checks `.grove/`, but previously
  discarded that authority before preparation and path-based evacuation.
- This leaf proves phase-boundary substitution detection. It does not close a
  direct external replacement after a successful gate; descriptor-relative
  witness creation and source moves belong to `finish-witness-identity-k140`.
- Repository preparation may leave attempt-bound auxiliary cleanup evidence,
  but does not mutate the task tree. This leaf promises a task-root identity
  gate before task-tree mutation, not zero repository-adapter side effects.
- Witness-directory and manifest/content integrity are separate leaves under
  `finish-transaction-integrity-k136`.

## Done when

- A failing regression proves replacing `.grove/` after preflight is refused
  before witness creation without touching the replacement.
- A failing regression proves replacing `.grove/` after preparation is refused
  before evacuation without moving replacement bytes.
- Transaction setup carries the opened task-root directory across repository
  preparation; the prepared transaction then owns it and revalidates path
  identity at both phase gates. Callers still use the existing deep
  `finish`/`recover_pending` interface.

## Notes

Do not mistake phase gates for descriptor-relative mutation. Witness-child,
manifest-file, and partial-transition recovery have dedicated leaves.
