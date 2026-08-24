# formal-modeling-k1 — brief


## Goal

Use Quint and Alloy 6 to make the modular design precise before documentation or Rust implementation. Model the same claims independently enough to expose different blind spots, then reconcile them through explicit replay rather than treating agreement as proof.



## Context

Required scopes are Grove task-tree semantics, the finish/recovery protocol described by `TODO.finish_process.md`, and the lifecycle joining session completion, tree exhaustion, finish, interruption, and recovery. Alloy 6 must use temporal/behavioural modelling, not merely static relational snapshots. Quint must expose executable guarded actions and refusal outcomes.

Component models should live beside the crate they constrain (`crates/grove-task-tree/models/`, `crates/grove-finish/models/`, and the existing ordinal component); cross-component lifecycle models live in `models/system/`. A single repository runner must execute every model and fail if a tool silently did no work.

## Done when

- A shared claim catalogue defines state, actions, observations, environment assumptions, stable states, transient states, refusal outcomes, and recovery obligations without encoding either tool's syntax.
- Independently constructed Alloy 6 and Quint models cover all three scopes, include satisfiable witnesses, and check their documented safety/liveness properties with reproducible commands.
- Each model documents tool version, bounds or trace limits, solver/backend, fairness assumptions, abstractions, deliberately omitted details, and what a green run does not prove.
- Every unique counterexample or design finding is replayed in the other formalism or recorded as inexpressible with a reason.
- `docs/formalism-findings.md` contains a pre-registered, evidence-linked comparison and a bounded synthesis of when each formalism helped here.
- Formal conclusions settle model placement, the crate-facing semantic contracts, the fate of every proposed finish simplification, and whether ordinal root lifecycle deserves an implementation task.
- No material semantic question is deferred to the documentation or implementation phase.

## Notes

Do not alter product behaviour in this phase. Model/test harness work and factual experiment documentation are in scope; architecture/user documentation and Rust refactoring are not.

Required experiment fields for every material observation: Situation, Formalism, Caught or missed, Cost, Counterfactual, and Verdict. Pre-register comparison of Alloy 6 temporal behaviour with Quint actions; unique versus overlapping findings; counterexample quality; test derivation; component-local versus system-level placement; synchronization burden; state-space/tooling cost; and failure modes that could create false confidence. Both model families remain required regardless of the comparison result.
