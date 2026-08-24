# implementation-k3 — brief


## Goal

Implement the formally checked and documented modular design as deep Rust crates, preserving the external contract except for the explicitly approved removal of migration.



## Context

Models and current-state documents are now the source of truth. The initial leaf list is an ordered scaffold, not permission for a monolithic refactor: `implementation-plan-k21` must decompose any wide extraction into expand → migrate → contract slices before code changes begin.

## Done when

- The workspace contains `grove-methodology`, `grove-task-tree`, `grove-workspace`, and `grove-finish` with small semantic interfaces and acyclic dependencies; the root package contains application/runtime composition and the two binaries.
- `ordinal-fs-tree` owns generic ordered filesystem-tree mechanics. Direct filesystem access in other semantic crates is absent or explicitly justified and tested as a domain-specific exception.
- Migration code, command surfaces, compatibility tests, dependencies, and stale documentation are gone; current-format initialization works and other formats fail closed.
- Finish behaviour conforms to both model families across Git, native jj, and colocated jj, including fault injection, crash/restart, ticket correlation, evacuation, quarantine, recovery, and ownership conflicts.
- CLI/config/output/release/MSRV/platform contracts in the preservation ledger are verified against the baseline.
- Old root modules, reach-through imports, compatibility shims, duplicated tests, unused dependencies, and `TODO.finish_process.md` are removed only after their durable obligations have moved to the new owners.

## Notes

Implementation order is contractual: remove migration; implement a model-earned ordinal lifecycle leaf if one was inserted; extract methodology; extract task tree; extract workspace identity; extract finish without changing its behaviour; apply separately inserted model-proven finish simplifications; collapse the application root; then perform the full contract sweep.

Use test-first slices and real repositories for VCS behaviour. The compiler should enforce dependency direction; tests should exercise crates through their public seams. A smaller line count is welcome but is not an acceptance criterion—fewer concepts, smaller interfaces, clearer ownership, and fewer unverified state transitions are.
