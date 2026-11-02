# ravel-git-bounds-k72

## Goal

Bound Git acquisition in time, size and output. Deadlines and signals stop and
reap every acquisition child, and the refusals keep distinct remedies.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Settings" for
  `resolutionTimeoutMs`, "Bounds" and "Failures".
- The `Watch` latch, its deadline and its signal rules from
  `ravel-skill-bounds-k57`.

## Done when

- The resolution deadline and the cancellation latch cover each Git child. At
  the deadline or on INT, TERM or HUP, Ravel sends TERM, then KILL, reaps the
  child and removes the acquisition storage and the staging.
- A timeout refuses as `skill_resolution_timeout`, exit 124. It reports the
  effective deadline, the active source and the operation. It does not report
  an unavailable revision because time ran out.
- TERM during a controlled transport refuses as `skill_resolution_cancelled`,
  exit 143, and Ravel raises the signal again. A child that Ravel cannot reap
  refuses as `skill_cleanup_failed`, exit 5.
- Git diagnostic output is bounded to 256 KiB and redacted before display. No
  credential reaches a diagnostic or a manifest.
- The entry and byte bounds refuse from the tree listing before extraction
  writes past them. The bytes of all sources count local and Git sources
  together. A value equal to a bound passes.
- Fixtures cover the Git parts of "Slow valid source", "Cancellation" and
  "Bounds and frames", each with a control that removes the bound. A
  controlled delay comes from a fake transport or an internal seam, not from
  the network.
- The documentation of the bounds and failures matches. The affected Ravel
  checks pass.

## Notes

Ravel cannot interrupt one system call that storage holds. The refusal then
comes after that call returns. Fixtures must not depend on that interval.
