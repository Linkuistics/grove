# driver-exclusivity-review-k15

**Kind:** review-design
**Reviews:** driver-exclusivity-k14

## Goal

Adversarially review `driver-exclusivity-k14` and record concrete findings for its integration step.

## Context

- Attempt to disprove the ownership design under concurrent driver starts,
  driver crash/restart, child exec inheritance, stale sessions, and grove
  deletion/recreation in the same working tree.

## Done when

- Findings identify duplicate-launch windows, deadlocks with session tree
  mutations, hidden durable state, generation-reuse failures, and missing Git/jj
  or process-lifecycle tests; no fixes are applied.

## Notes
