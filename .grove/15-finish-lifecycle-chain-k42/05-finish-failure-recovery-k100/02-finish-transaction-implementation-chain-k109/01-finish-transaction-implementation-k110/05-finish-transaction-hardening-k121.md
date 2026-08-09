# finish-transaction-hardening-k121

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
