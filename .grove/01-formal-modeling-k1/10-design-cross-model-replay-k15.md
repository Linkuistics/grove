# cross-model-replay-k15


## Goal

Cross-read the independently completed models and replay their unique evidence instead of merely comparing their prose.



## Context

This is the first task allowed to inspect both model families together. Reconciliation may align claim names and abstractions, but it must not erase a useful difference just to make the models look identical.

## Done when

- Every unique counterexample, unreachable state, ambiguous requirement, derived test, and proposed simplification from one formalism is attempted in the other.
- Successful replays link exact commands/seeds/scopes and traces; failures state whether the behaviour is inexpressible, abstracted away, outside bounds, tool-limited, or exposes a model defect.
- Both runners prove that tools, assertions/tests, witnesses, and verification actually executed non-zero work.
- The comparison identifies overlap, unique leverage, misleading greens, counterexample readability, synchronization burden, and how each model changes the Rust/test plan.
- Real current implementation behaviour is replayed for high-risk findings where a bounded fixture can decide whether the counterexample is merely abstract or product-relevant.
- Experiment 2 receives evidence-linked entries for all material replay outcomes.

## Notes

Correct either model when replay finds a defect, retain the failing regression scenario, and rerun its whole model family. Do not change product code or architecture/user documentation here.
