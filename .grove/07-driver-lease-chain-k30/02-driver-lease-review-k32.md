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
  lock/path replacement races, descriptor inheritance, and Herdr disposition
  ordering.

## Done when

- Findings are recorded here with severity, race/interleaving evidence or a
  black-box reproducer, and the violated contract, or an explicit no-finding
  result.
- Same-tree contention and different-workspace independence are independently
  exercised in at least Git and jj shapes.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `driver-lease-integrate-k33` owns fixes.
