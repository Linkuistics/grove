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

## Done when

- Findings are recorded here with severity and a concrete interleaving/event
  trace or black-box reproducer, or an explicit no-finding result.
- At least one admitted-old/replacement-driver race and one orphan timeout are
  independently exercised.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `session-epoch-integrate-k37` owns fixes.
