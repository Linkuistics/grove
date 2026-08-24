# finish-k12


## Goal

Model finish execution, interruption, and recovery as explicit Quint actions and outcomes.



## Context

Cover the same shared contract as the Alloy finish model but derive the Quint state/action design independently. The environment must be able to fail, restart, race ownership, and expose Git/native-jj/colocated-jj differences.

## Done when

- Typed state covers confirmation, attempt identity, correlation ticket, witness/evacuation/quarantine, `.grove`, VCS lane, owned and unrelated repository state, and both terminal exits.
- Actions cover each protocol boundary plus injected failure, restart, recovery, ownership ambiguity, classified refusal, preserve, merge, and owned cleanup.
- Invariants and scenarios test no mutation without proof, evacuation before root deletion, persistent recovery correlation, monotonic evidence, idempotent recovery, correct error taxonomy, and common outcomes across all VCS lanes.
- Seeds/traces reproduce every counterexample; verification limits and fairness/environment assumptions are explicit.
- Material observations and implementation-test candidates are appended to Experiment 2.

## Notes

A helper that makes an unsafe state unconstructable is useful only if the real environment is constrained the same way. Keep environmental nondeterminism visible.
