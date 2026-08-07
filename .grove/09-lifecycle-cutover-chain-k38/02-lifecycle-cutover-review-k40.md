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

## Done when

- Findings are recorded here with severity, observable fake-command/tree
  evidence, and the threatened contract, or an explicit no-finding result.
- At least one invalid-config mutation guard and one mandate-versus-insert case
  are independently exercised.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `lifecycle-cutover-integrate-k41` owns
fixes. Finish behavior remains explicitly out of this review.
