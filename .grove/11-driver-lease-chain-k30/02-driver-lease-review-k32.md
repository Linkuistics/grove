# driver-lease-review-k32

**Kind:** review-impl
**Reviews:** driver-lease-k31

## Goal

Adversarially review `driver-lease-k31` and record concrete findings for its integration step.

## Context

- Review `driver-lease-k31` against
  `docs/adr/one-live-driver-per-working-tree.md`.
- Attack control-directory aliasing, symlink/canonicalization behavior,
  Git-common-dir leakage, jj workspace sharing, external selector influence,
  lock/path replacement races, descriptor inheritance, and final disposition
  ordering.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `driver-lease-integrate-k33` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity, race/interleaving evidence or a
  black-box reproducer, and the violated contract, or an explicit no-finding
  result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded Git and jj contention evidence rather than re-running it.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `driver-lease-integrate-k33` owns fixes.
