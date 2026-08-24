# task-tree-k7


## Goal

Model Grove's task-tree semantics in Alloy 6 without reimplementing generic ordinal-tree algebra.



## Context

The model belongs with `grove-task-tree`. Treat `ordinal-fs-tree` properties as an imported/assumed algebraic boundary and concentrate on Grove names, format ownership, legal selection, growth, retirement, and terminality.

## Done when

- The model represents current-format roots, Grove-owned entries, task kind/key identity, ordinal sibling order, active-leaf selection, decomposition, insertion/addition, retirement, and empty/root-terminal states.
- Assertions cover uniqueness, valid naming/format, stable selection, preservation of unrelated/opaque entries, legal mutation preconditions, fail-closed foreign roots, and terminal-state correctness.
- Temporal traces include normal progress and refused/invalid operations; satisfiable witnesses demonstrate every transition family.
- Bounds, assumptions about the ordinal component, runner command, claims, and at least one useful instance or counterexample are documented.
- Material observations are appended to Experiment 2 using the required six fields.

## Notes

If the model needs raw path manipulation to express a Grove rule, treat that as evidence that the semantic seam is wrong; do not silently pull filesystem mechanics into this component.
