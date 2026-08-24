# ordinal-root-lifecycle-k14


## Goal

Determine experimentally whether atomic root lifecycle is a deep, domain-independent `ordinal-fs-tree` capability or should remain private to Grove finish.



## Context

The user prefers filesystem mechanics to be delegated to `ordinal-fs-tree`, but this is not permission to turn it into a bag of Grove-shaped callbacks. Evaluate lock ownership, root identity, opaque/foreign entry preservation, crash states, rollback/recovery evidence, and whole-root evacuation/removal against both completed model families.

## Done when

- A candidate contract can be stated without Grove, Git, jj, session kinds, task handles, finish tickets, branches, or bookmarks—or the experiment records that it cannot.
- The candidate owns ordering, locking, identity validation, opaque/foreign preservation, durable staging/rollback states, and recovery-safe failure semantics behind a small interface; it exposes neither raw syscall choreography nor arbitrary callbacks that leak the operation.
- A focused prototype/model is tested against representative task-tree and finish counterexamples from both formalisms.
- The alternatives—extend `ordinal-fs-tree`, keep a finish-private adapter, or introduce no new abstraction—are compared for interface depth, misuse resistance, portability, and synchronization cost.
- The decision is keep, defer, or reject with evidence. No production implementation is performed here.

## Notes

If the abstraction earns implementation, `formal-synthesis-k16` must insert a dedicated implementation leaf before `extract-task-tree-k24`. If it does not, document the precise semantic exception that remains in `grove-finish`.
