# documentation-k2 — brief


## Goal

Turn the checked formal design into one coherent current-state explanation for users and maintainers before implementation begins.



## Context

This phase consumes the formal claim catalogue, counterexamples, placement decision, and synthesis. It consolidates existing documents rather than creating a second architecture narrative or a historical design-plan archive.

## Done when

- `README.md` gives a concise orientation and installation path, then points to authoritative guides.
- `docs/USAGE.md` is an end-to-end user guide covering normal work, human and agent flows, supported workspace layouts, current-format roots, finish outcomes, interruption, recovery, and ownership-conflict diagnostics.
- `docs/CONFIGURATION.md` accurately owns settings, defaults, and overrides without duplicating workflow prose.
- `docs/ARCHITECTURE.md` owns the crate DAG, deep public interfaces, state/error ownership, filesystem delegation rule, formal-model correspondence, and a practical “where to change this concern” map.
- The ADR directory is the minimum coherent current decision set; obsolete or duplicate decisions are edited, merged, or removed rather than retained as a changelog.
- Component model READMEs are colocated with their crates, cross-component models have a clear owner, and all old links/paths are reconciled.
- An implementation-ready plan names exact seams, files, tests, gates, deletion work, and any model-earned conditional leaves. It contains no unresolved product semantics.

## Notes

Documentation may describe the target system before the Rust layout exists on this branch because implementation immediately follows and the branch must land atomically. Mark no document as aspirational in the final result: by the contract sweep, all surviving prose describes reality.

Prefer `docs/ARCHITECTURE.md` as the maintainer/change guide instead of adding another document. Keep model-specific mechanics in crate-local model READMEs and user consequences in the user guide.
