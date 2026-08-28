# component-locality-k20


## Goal

Place component-specific models and explanatory artifacts beside their owning crates and leave the repository root responsible only for cross-component material.



## Context

Formal tasks may already have created target model directories. Reconcile the older `docs/ordinal-fs-tree/` layout and every reference without changing Rust product behaviour.

## Done when

- `ordinal-fs-tree` architecture/model material is colocated under its crate using the same convention chosen for `grove-task-tree` and `grove-finish`.
- Each component model directory has a short README stating scope, claim mapping, commands, bounds/assumptions, counterexample artifacts, and implementation/test links.
- `models/system/` owns only lifecycle composition and has an explicit maintainer/runner contract.
- One root model command discovers and executes all component/system models, detects missing/dead tools and zero work, and is documented from the repository entry points.
- All document links, scripts, comments, manifests, CI/release inputs, tests, and old paths are reconciled; duplicate or empty component-document directories are removed.

## Notes

Locality is about semantic ownership, not forcing every artifact into a crate package. Exclude model files from published crates only when packaging requires it, without moving their conceptual owner back to the root.
