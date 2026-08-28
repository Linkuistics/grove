# remove-migration-k22


## Goal

Remove legacy task-tree migration completely as the one approved external behaviour change.



## Context

Do this before crate extraction so compatibility machinery is not fossilized into new APIs. Current-format initialization remains supported; absent, legacy, malformed, or foreign formats must be classified without rewriting a live tree.

## Done when

- Tests first specify fresh current-format creation, current-format operation, and fail-closed legacy/foreign/malformed handling with actionable diagnostics.
- Migration CLI verbs/options/help, dispatch, `tree_migrate`, `tree_migration_transaction`, repository migration commits, compatibility adapters, fixtures, and migration-only dependencies are removed.
- No discovery/validation path stamps or rewrites a non-current live root as a side effect of classification.
- User guide, configuration, architecture, ADRs, changelog/release notes, scripts, and tests contain no promise or example of migration.
- Existing non-migration CLI/config/output contracts pass unchanged, and real Git/jj fixtures prove failure leaves repository and `.grove` state untouched.

## Notes

Preserve any generic transaction primitive still required by current-format operations or finish; delete it only when caller and fault-test evidence proves it migration-only.
