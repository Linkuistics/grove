# collapse-application-k27


## Goal

Collapse the root package to application/runtime composition and the two shipped binaries after semantic ownership has moved to crates.



## Context

All model-earned finish simplification leaves must have run before this task. The root may coordinate use cases and render CLI results, but must not retain shadow domain models or reach into crate internals.

## Done when

- Root `src/` has a legible application/runtime structure that wires configuration, methodology, task tree, workspace, finish, and CLI/binary adapters through public crate interfaces.
- The public library surface retains `run_human`, `run_agent`, and only the embed/provisioning seam justified by packaging; binaries remain thin and keep names/help/output/exit behaviour.
- Cross-cutting orchestration is expressed as use cases with typed inputs/outcomes, not a new “utils” or “services” dumping ground.
- No root module duplicates crate-owned state/errors/policy, no reach-through imports or path-based coupling remain, and dependency direction is enforced by manifests/compiler tests.
- End-to-end CLI, config, model-derived, embed/install, and real VCS tests pass through the collapsed application boundary.

## Notes

Optimize the source tree for finding a concern, not for matching historical filenames. Preserve stable public seams; internal module moves need no compatibility facade.
