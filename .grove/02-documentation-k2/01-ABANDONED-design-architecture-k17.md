# architecture-k17


## Goal

Rewrite the architecture documentation as the authoritative map of the checked modular system.



## Context

Consume the formal synthesis and model ownership decisions. Consolidate `docs/ARCHITECTURE.md` and existing component architecture prose instead of adding a parallel redesign document.

## Done when

- `docs/ARCHITECTURE.md` shows the Cargo crate DAG and explains why each dependency points in that direction.
- Each crate has a small public semantic interface, owned state/errors, non-responsibilities, test seam, and model correspondence. The root package is clearly application/runtime composition plus binaries.
- The lifecycle from command/session through task-tree mutation and finish/recovery is explained at component boundaries, including `RecoveryPending` and `OwnershipConflict`.
- The filesystem rule is explicit: generic ordered-tree/storage mechanics go to `ordinal-fs-tree`; `grove-workspace` supplies repository identity/VCS capabilities; any Grove-owned filesystem exception is named and justified.
- A concise change map tells maintainers and LLMs where to modify methodology, task semantics, repository discovery, finish, CLI/runtime, models, and user docs without listing unstable internal helpers.
- `README.md` provides a short orientation and links here; stale diagrams, module names, and duplicate architecture claims are removed or reconciled.

## Notes

Prefer one dependency diagram plus prose over directory dumps. Describe deep interfaces and invariants, not every source file. All surprising formal assumptions should link to the relevant component model README.
