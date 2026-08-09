# auxiliary-cleanup-markers-k126

**Kind:** impl

## Goal

Make finish Git-index backup and success images attempt-keyed, independently
marked, and safely recoverable or reapable.

## Context

- Consume the quarantine cleanup identity/marker vocabulary from
  `quarantine-cleanup-core-k125` without turning auxiliary bytes into a finish
  receipt.
- Plain Git and colocated jj create different subsets of the two auxiliary
  roles, but both must bind cleanup to the exact finish handle and attempt.

## Done when

- Backup and success-image paths and markers are attempt-keyed and collision
  checked before repository mutation.
- Recovery validates the marker, no-follow file identity, role, handle, and
  attempt before restoring, activating, discarding, or exposing an orphan to
  cleanup.
- Partial discard or activation leaves enough independently valid evidence for
  later cleanup, while a replaced file or marker remains untouched with an
  actionable warning.
- Plain-Git/native-jj/colocated-jj tests cover creation, recovery, activation,
  partial deletion, old attempts, and absent-index behavior.

## Notes

Do not wire driver lifecycle cleanup or add process-level acceptance coverage
in this child.
