# finish-transaction-hardening-k121 — brief

**Kind:** impl

## Goal

Close the fail-closed transaction's adversarial transition, tamper, topology,
and cleanup matrix across all supported repository shapes.

## Context

- Treat any newly found product concern as a new Grove leaf; this leaf owns only
  missing cases of the reviewed finish-transaction contract.
- Keep outcome proof guarded through rollback or quarantine handoff and restore
  quarantine atomically when a forward disposition changes.

## Done when

- Tests cover every prepared/evacuated/rollback/quarantine interruption point,
  manifest/content corruption, foreign entries, symlinks, special files,
  collisions, tracked witness, root replacement, repository races, rollback
  failure, operator-restorable recovery, disposal safety, and cleanup failure.
- Diagnostics name the witness, recorded and observed topology, and exact-start
  versus exact-result operator procedures.
- No test reaches past the transaction interface except deterministic internal
  transition/failure-seam tests.

## Notes

Fix only contract gaps demonstrated by a failing test.

## Decomposition

- `finish-transaction-integrity-k136`: harden the task-root, witness, manifest,
  and evacuated-content integrity boundary before repository disposition.
- `finish-transaction-transition-recovery-k137`: make every evacuation and
  rollback prefix recoverable, including topology races and actionable pending
  diagnostics.
- `finish-transaction-handoff-hardening-k138`: keep exact-result proof guarded
  through quarantine handoff and cover collision, restoration, disposal, and
  cleanup-failure behavior.
