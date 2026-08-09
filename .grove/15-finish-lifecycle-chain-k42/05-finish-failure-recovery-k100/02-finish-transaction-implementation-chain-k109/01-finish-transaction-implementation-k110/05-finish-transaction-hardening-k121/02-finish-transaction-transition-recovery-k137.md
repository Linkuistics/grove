# finish-transaction-transition-recovery-k137

**Kind:** impl

## Goal

Make every evacuation and rollback prefix restart-safe and operator-recoverable.

## Context

- Consume the validated transaction object delivered by
  `finish-transaction-integrity-k136`.
- This leaf owns pre-commit filesystem transitions and the not-committed path;
  forward quarantine handoff belongs to `finish-transaction-handoff-hardening-k138`.

## Done when

- Deterministic internal tests interrupt every evacuation and rollback prefix
  and prove retry either restores the byte-identical live tree or retains the
  only copies beneath the blocking witness.
- Destination collisions, missing or foreign entries, digest mismatch,
  rollback failure, and repository changes before or after rollback remain
  `Recovery pending` without overwriting unrelated bytes.
- Pending diagnostics name the witness, recorded and observed topology, and
  both exact-start and exact-result operator procedures.

## Notes

Do not add a process-level case where a deterministic transaction seam is the
more direct proof; acceptance remains in `finish-transaction-docs-acceptance-k122`.
