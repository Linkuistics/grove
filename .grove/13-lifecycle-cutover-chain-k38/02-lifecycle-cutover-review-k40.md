# lifecycle-cutover-review-k40

**Kind:** review-impl
**Reviews:** lifecycle-cutover-k39

## Goal

Adversarially review `lifecycle-cutover-k39` and record concrete findings for its integration step.

## Context

- Review `lifecycle-cutover-k39` against the exact ordered flow in the spec and
  the complete-session-configuration ADR.
- Attack hidden launch policy, config-validation timing, double selection,
  mutation before tool/version/config checks, direct-exec argv boundaries,
  prompt authority, config reload, launch-window insertion, and status/elapsed
  diagnostics.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `lifecycle-cutover-integrate-k41` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity, observable fake-command/tree
  evidence, and the threatened contract, or an explicit no-finding result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded invalid-config and mandate-versus-insert evidence rather than
  re-running it.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `lifecycle-cutover-integrate-k41` owns
fixes. Finish behavior remains explicitly out of this review.
