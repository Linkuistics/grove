# decisions-k19


## Goal

Reconcile `docs/adr/` into the minimum coherent current-state decision set for the modular design.



## Context

Decision records are not a chronological archive. Edit, merge, split, rename, or delete existing records in place based on current decision identity; add a record only when the choice is hard to reverse, surprising, or has a material rejected alternative.

## Done when

- `task-tree-transactions-fail-closed.md` reflects current-format-only ownership, removal of migration, and the checked finish/recovery boundary.
- `entry-name-is-the-only-seam.md` reflects crate ownership and delegation to `ordinal-fs-tree` without leaking Grove semantics into the generic crate.
- Supported-workspace and finish-related records express the Git/native-jj/colocated-jj common contract and lane refinements established by the models.
- At most one new semantic crate-boundary record is added, and only if the existing set cannot coherently own that durable tradeoff.
- Superseded narratives, duplicate decisions, stale module/path names, and broken citations are removed; surviving records describe one consistent present.
- Architecture, user guide, model READMEs, and ADRs link to one another without making the same claim authoritative in multiple places.

## Notes

Do not add numbered chronology, “superseded” tombstones, or a record for an obvious mechanical extraction. Stable slugs identify decisions; version history preserves history.
