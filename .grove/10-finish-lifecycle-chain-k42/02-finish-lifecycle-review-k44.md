# finish-lifecycle-review-k44

**Kind:** review-impl
**Reviews:** finish-lifecycle-k43

## Goal

Adversarially review `finish-lifecycle-k43` and record concrete findings for its integration step.

## Context

- Review `finish-lifecycle-k43` against the finish eligibility, confirmation
  boundary, universal lock, and scoped VCS commit contracts.
- Attack duplicate/hidden finish state, starvation, work appearing after
  launch, terminal-verb bypass, unrelated staged/working-copy consumption,
  unborn Git behavior, jj intermediate snapshots, and premature done signals.

## Done when

- Findings are recorded here with severity, exact tree/VCS reproducer, and the
  threatened contract, or an explicit no-finding result.
- Both a post-launch insertion refusal and an unrelated-work preservation case
  are independently exercised.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `finish-lifecycle-integrate-k45` owns
fixes.
