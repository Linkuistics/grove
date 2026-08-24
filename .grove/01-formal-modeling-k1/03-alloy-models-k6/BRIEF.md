# alloy-models-k6 — brief


## Goal

Build an independent Alloy 6 behavioural account of the shared claims using mutable relational state and temporal operators.



## Context

Follow `model-contract-k5`. Alloy 6 is not limited to static structure: model transitions with `var` relations, primed state, temporal formulas, and lasso traces where appropriate. Do not inspect the Quint implementation before Alloy's three models are complete and green.

## Done when

- Runnable `.als` models exist for task tree, finish/recovery, and end-to-end lifecycle at the agreed component/system paths.
- Every claim has a named assertion/check or an explicitly documented reason it is not represented; every important state/action has a satisfiable witness.
- Commands pin tool/solver details, use meaningful scopes and trace bounds, and fail when the runner, assertion set, or witnesses silently execute zero work.
- Counterexample traces and resulting claim/design changes are retained in compact, reproducible form and logged in the formalism experiment.

## Notes

Record symmetry, exact-scope, liveness/fairness, and boundedness caveats. A successful bounded check is evidence about the stated bounds, not proof about arbitrary executions.
