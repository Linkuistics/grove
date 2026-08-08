# session-epoch-review-k36

**Kind:** review-impl
**Reviews:** session-epoch-k35

## Goal

Adversarially review `session-epoch-k35` and record concrete findings for its integration step.

## Context

- Review `session-epoch-k35` as a concurrency protocol against both cited ADRs
  and the spec's explicit lock order.
- Attack self-deadlock, open/lock/path replacement races, lease-transfer versus
  shared admission, guard leakage across exec, orphaned descendants, signal
  reuse/cleanup timing, wrong-worktree aliases, and accidental authentication
  claims.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `session-epoch-integrate-k37` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity and a concrete interleaving/event
  trace or black-box reproducer, or an explicit no-finding result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded race and orphan-timeout evidence rather than re-running it.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `session-epoch-integrate-k37` owns fixes.
