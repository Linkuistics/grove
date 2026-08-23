# migration-k36

## Goal

Cut `tree_migrate` free of `tree_id`. Migration is out of the library's surface
by decision — the root brief says *everything grove's tree modules do except
migration* — but its **output** is a current-format tree, so it must render names
through the domain type rather than through the parser the sweep is about to
delete. This leaf is what lets `sweep-k37` run.

## Context

- `src/tree_migrate.rs` — the planner, its private per-layout recognisers, and its
  four uses of `tree_id`: `parse`, `validate_slug`, `Outcome`, and `Entry::name`
  at line 453, which is where a migrated name is rendered.
- `src/tree_migration_transaction.rs` — the transaction that consumes the planner,
  and its `tree_format` / `tree_access` / `tree_lifecycle` calls.
- `src/lib.rs`'s module-header essay on why `tree_migrate` is crate-private, and
  in particular the rule it settled: **a frozen grammar kept whole** stopped being
  an exemption once nothing read the grammar, and what survives is the
  *recognition* — a private matcher per withdrawn layout — because that is the
  part whose absence loses a workstream rather than a lookup.
- `src/tree_lifecycle.rs:107` and `:343` — migration's live entry point is
  `transition_to_current`, and `has_explicit_legacy_kind`.
- Suites: `migration_transition`, `migration_commit`, `lifecycle_cutover`.

## Done when

- `tree_migrate` names no item from `tree_id`. Recognising a **withdrawn** layout
  stays where it is — private matchers inside `tree_migrate` — and rendering a
  **current** name goes through the domain type from `domain-k29`.
- Migration still produces byte-identical output for the same input. This is the
  one place in the increment where *pure refactor* can be checked directly rather
  than argued: the existing migration tests carry expected on-disk names.
- The whole suite passes; changed tests recorded in the node brief.

## Notes

**This is the leaf where the lenient grammar could bite, and it is the reason
this leaf is not merely mechanical.** If `domain-k29` tightened the grammar
(question 2), then migration's rendering must produce the canonical spelling —
and if any withdrawn layout's position could carry a form the new grammar
refuses, migration has to normalise rather than pass through. `tree_id::Entry::name`
padded `{:02}` unconditionally, so the current output is already canonical; the
check is that nothing on the *input* side reaches the renderer unparsed.

**Do not fold this into the sweep.** They look like one job — both are about
`tree_id` going away — but they fail differently: this one can silently change
what a migration writes, and the sweep cannot. A migration bug lands on a tree
someone has been working in for months, which is the worst blast radius in the
whole increment.

**Migration is reached automatically**, through `transition_to_current`, not
through a verb an operator types. So a regression here does not announce itself;
it happens during someone else's session.
