# finish-k8


## Goal

Model the complete finish and recovery protocol in Alloy 6, including hostile/interrupted environmental behaviour.



## Context

This model must cover every load-bearing concern in `TODO.finish_process.md`, not only the happy path. Model the external repository and filesystem environment explicitly enough to distinguish Git, native jj, and colocated jj refinements and to prevent ownership assumptions from becoming facts by declaration.

## Done when

- State represents attempt identity, confirmation intent, external ticket, witnesses, evacuated state, `.grove` presence, quarantine, VCS lane, branch/bookmark/worktree ownership, merge/removal progress, and stable terminal outcomes.
- Transitions include prepare, persist/correlate, evacuate, remove root, quarantine, preserve exit, merge exit, cleanup, injected failure at each boundary, restart, recover, refusal, and ownership loss/ambiguity.
- Assertions cover evacuation before deletion, durable correlation, no unrelated mutation, at-most-one owned cleanup target, monotonic recovery evidence, idempotent recovery, correct `RecoveryPending`/`OwnershipConflict` classification, and both successful exits.
- Lane-specific checks show the common protocol is refined correctly by Git, native jj, and colocated jj; colocated state is not treated as two independent repositories.
- Witnesses and counterexamples are reproducible, and every material observation is logged in Experiment 2.

## Notes

Challenge the current architecture's external ticket, quarantine, witness, and recovery mechanisms but remove none merely because a smaller model can omit it. A simplification is earned only when its environment assumptions and replacement claim are explicit.
