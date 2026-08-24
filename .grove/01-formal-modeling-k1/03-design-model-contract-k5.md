# model-contract-k5


## Goal

Define one tool-neutral semantic contract and claim catalogue that the independently built Quint and Alloy models can both test.



## Context

The contract is an abstraction boundary, not a pseudo-implementation. It must distinguish task-tree semantics, finish/recovery semantics, the external VCS/filesystem environment, and the system lifecycle while retaining enough detail to represent every load-bearing concern in `TODO.finish_process.md`.

## Done when

- Shared vocabulary precisely defines identities, task states, root states, finish-attempt identity, confirmation intent, artifacts, observations, stable/transient states, and terminal outcomes.
- Actions and total refusal outcomes are named independently of tool syntax. `RecoveryPending` means a correlated Grove-owned attempt is incomplete; `OwnershipConflict` means state is unrelated, ambiguous, or cannot be proved safe to mutate.
- Git, native jj, and colocated jj have explicit environmental assumptions and lane-specific obligations while sharing one abstract outcome contract.
- The finish claims cover confirmation, persisted intent, external correlation ticket, witness, evacuation before `.grove` deletion, quarantine, branch/bookmark preservation, merge/removal, crash/restart, idempotent recovery, no unrelated mutation, and both successful exits.
- The task-tree claims cover name/ordinal/format invariants, selection, mutation, terminality, root identity, fail-closed ownership, and the boundary delegated to `ordinal-fs-tree`.
- The system claims connect completed sessions, exhaustion, explicit finish entry, interruption, recovery, and root absence.
- Exact model paths, runner entry point, claim identifiers, evidence format, and experiment logging convention are fixed. Neither model family needs to invent a semantic decision.

## Notes

Do not encode Rust module names or current helper functions as state. Do not read or alter product implementation while defining the contract; use the approved requirements, baseline, current public behaviour, and `TODO.finish_process.md`.
