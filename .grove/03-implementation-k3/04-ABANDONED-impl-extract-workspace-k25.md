# extract-workspace-k25


## Goal

Extract supported repository discovery, identity, layout, and VCS capabilities into a focused `grove-workspace` crate.



## Context

This crate gives application and finish code a stable, validated description of Git, native jj, or colocated jj. It must not absorb generic filesystem transactions, task-tree operations, CLI rendering, or the finish state machine.

## Done when

- Tests first cover discovery/identity and supported operations in real temporary Git, native-jj, and colocated-jj repositories, including unsupported/ambiguous layouts and ownership changes.
- A small API returns a validated `Workspace`/lane identity and semantic VCS operations needed by finish/runtime, with typed errors and explicit external-command boundaries.
- Colocated jj/Git is represented as one coordinated workspace mode, not two unrelated repositories.
- Callers no longer rediscover roots, parse VCS identity independently, or run ad-hoc Git/jj commands; root helpers and duplicated fixtures are removed after migration.
- No generic read/write/rename/remove/path utility surface appears, and task-tree/finish policy stays with their semantic owners.

## Notes

Prefer capability methods and immutable discovered identity over exposing command runners. Provide test doubles only at the external process seam, not around all domain logic.
