# quint-models-k10 — brief


## Goal

Build an independent, executable Quint account of the shared claims using explicit state, guarded actions, total refusals, and crash/restart scenarios.



## Context

Follow `model-contract-k5`, but do not inspect the Alloy models until all three Quint models have a complete green version. Use Quint simulation and checking for what they establish: `run` witnesses reachability; invariants and temporal properties require `verify` or an explicitly documented backend/limit.

## Done when

- Runnable `.qnt` models exist for task tree, finish/recovery, and end-to-end lifecycle at the agreed component/system paths.
- Models typecheck and have deterministic test/scenario coverage plus non-zero randomized/simulation witnesses; invariant/property verification runs where supported.
- Every action is total through an explicit refusal/outcome rather than disappearing when a guard is false, and crash/restart is a first-class behaviour where relevant.
- The repository runner fails on missing tools, zero tests, zero witnesses, or skipped verification and records the exact seed/trace needed to replay failures.
- Claims, abstractions, limits, counterexamples, and observations are durable and linked from Experiment 2.

## Notes

Avoid encoding implementation control flow merely because Quint is executable. The model should make domain state transitions clearer than the Rust code, not mimic it.
