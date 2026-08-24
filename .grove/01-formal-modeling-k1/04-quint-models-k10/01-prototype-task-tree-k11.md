# task-tree-k11


## Goal

Model Grove task-tree actions and refusals in Quint against the shared task-tree claims.



## Context

The model belongs with `grove-task-tree`; generic ordinal ordering is assumed through the `ordinal-fs-tree` contract. Build without reading Alloy's task-tree model.

## Done when

- Typed state and actions cover current-format initialization, selection, decomposition, insertion/addition, retirement, invalid/foreign roots, opaque entries, and terminality.
- Refused operations leave protected state unchanged and return a classified observation; they are exercised rather than filtered out of traces.
- Invariants cover identity/name/format validity, sibling order assumptions, stable selection, unrelated-entry preservation, legal mutation, and correct exhaustion.
- Tests and simulation witnesses cover each transition family; verification commands, seeds, trace limits, abstractions, and ordinal assumptions are documented.
- Material Quint-specific catches, misses, costs, and derived tests are appended to Experiment 2.

## Notes

Do not grow a second filesystem model. If a Grove claim cannot be expressed above the ordinal seam, report that seam problem explicitly.
