# quarantine-cleanup-core-k125

**Kind:** impl

## Goal

Make completed task-root quarantines independently recognizable and safely
retryable without deleting a substituted filesystem object.

## Context

- The exact repository result is already proven before quarantine handoff; the
  cleanup marker is disposal evidence only and must never classify lifecycle
  state.
- Immediate finish cleanup and later lease-owned cleanup must use the same deep
  disposal seam.
- Keep Git-index backup/success auxiliaries and bare-driver integration in the
  later children of `finish-cleanup-reaping-k124`.

## Done when

- Before the atomic task-root rename, Grove writes a versioned cleanup marker
  naming the finish handle, attempt identity, and no-follow task-root identity;
  a partial cleanup always retains that marker until all other entries are
  gone.
- Cleanup atomically claims a marked quarantine, revalidates the claimed
  no-follow object against the open descriptor, and removes descendants
  descriptor-relatively without following symlinks.
- A replaced or ambiguous quarantine is left untouched with an actionable
  warning; an interrupted `REAPING` quarantine can be retried to completion.
- Immediate successful finish uses the new seam, while cleanup failure remains
  best-effort and leaves a complete retryable quarantine.
- Unit tests cover valid and foreign quarantines, partial retry, replacement
  identity, symlink targets, marker corruption, and persistent cleanup failure.

## Notes

Do not add lifecycle classification, Git-index auxiliary cleanup, or broaden
the repository, driver-lease, or session-epoch interfaces in this child.
