# extract-task-tree-k24


## Goal

Extract Grove task-tree semantics into `grove-task-tree`, delegating generic ordered filesystem mechanics to `ordinal-fs-tree`.



## Context

Use the exact slices created by `implementation-plan-k21`; decompose this leaf before coding if expand/migrate/contract cannot be completed safely in one session. The crate may depend on `grove-methodology` and `ordinal-fs-tree`, never on CLI/runtime, VCS workspace discovery, or finish orchestration.

## Done when

- Public contract tests cover root initialization/validation, selection, brief chain, kind lookup, add/insert/decompose/retire, terminality, opaque-entry preservation, and classified fail-closed errors through a small facade.
- Grove owns names, handles, current-format identity, session-kind semantics, selection policy, and legal task transitions; `ordinal-fs-tree` owns ordinal/path/storage mechanics and any model-earned root lifecycle capability.
- Direct filesystem calls in the new crate are eliminated or listed as reviewed semantic exceptions with fault tests and architecture/model justification.
- Root CLI/runtime consumers migrate without output/config/exit changes, and old modules, adapters, duplicated tests, and now-unused dependencies are contracted away.
- Component models and derived counterexample tests map to public behaviours and remain green.

## Notes

Do not mirror the old root module tree in a crate. The target is a deeper interface such as a validated task-tree/store facade, not a renamed collection of path helpers.
