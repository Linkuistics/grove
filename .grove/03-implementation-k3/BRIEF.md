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
- Old root modules, reach-through imports, compatibility shims, duplicated tests, and unused dependencies are removed only after their durable obligations have moved to the new owners. (`TODO.finish_process.md` is already gone — its four questions are dispositioned **keep** (Q2, Q3) and **defer** (Q1, and Q4's three cleanup rows) in [`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md).)

## Notes

Implementation order is contractual: remove migration; implement a model-earned ordinal lifecycle leaf if one was inserted; extract methodology; extract task tree; extract workspace identity; extract finish without changing its behaviour; apply separately inserted model-proven finish simplifications; collapse the application root; then perform the full contract sweep.

Use test-first slices and real repositories for VCS behaviour. The compiler should enforce dependency direction; tests should exercise crates through their public seams. A smaller line count is welcome but is not an acceptance criterion—fewer concepts, smaller interfaces, clearer ownership, and fewer unverified state transitions are.

## Promoted from `TODO.finish_process.md` before it was deleted

**No finish simplification is model-earned, so no leaf was inserted before
`collapse-application-k27`.** The implementation-order sentence above still reads
"apply separately inserted model-proven finish simplifications"; there are none,
and that step is a no-op rather than an omission
([`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)).
**And "none" now means "none earned yet" rather than "none possible".**
`finish-verdicts-k78` moved Q1 and Q4's three cleanup rows to `defer` and
commissioned the control that would decide them, so this step stays a no-op only
while that control has not been run. If it returns `delete/replace`, the leaf
that owns the commission inserts here, and this paragraph is what it edits.
The same is true of the ordinal-lifecycle step: the verdict was contested and
upheld, and root *creation* was rejected with it, so no leaf sits before
`extract-task-tree-k24` either.

**`tests/lifecycle_cutover.rs` (1,884 lines) is the live end-to-end driver suite
— launch, config reload, spawn failure, build pairing, re-provisioning — under a
name that reads as history.** Renaming it is unrelated to the finish process and
costs only churn. Carried here so it is not lost, and recorded as *considered and
not worth doing on its own*: if `contract-sweep-k28` is touching those files
anyway, the rename is free; otherwise leave it.
