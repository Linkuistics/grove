# admission-fixtures-k8


## Goal
Migrate grove-llm admission/shared helpers and grove-tui witnessed fixtures to
modular form; finish the downstream consumer inventory.



## Context
Inspect `crates/grove-llm/tests/` including support, session_kind_tree, leaf and
removed_surface helpers, plus `crates/grove-tui/`. Earlier children own grove-loop
and grove CLI. Enumerate any other consumers beyond these starting points.

## Done when
Retained fixtures are modular with unchanged admission and execution assertions;
compatibility-specific cases are identified for k3; downstream tests and
`bash scripts/check.sh` pass. Reconcile source-exact walkthroughs if touched.

## Notes
Preserve fake executables. Local files cannot define commands; personal policy
must admit every kind. Escape literal dollars according to the named scanner.
