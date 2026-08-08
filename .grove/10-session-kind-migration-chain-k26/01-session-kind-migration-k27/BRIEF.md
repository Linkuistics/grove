# session-kind-migration-k27 — brief

## Goal

Implement restart-safe automatic conversion of every accepted legacy tree into
the current filename-kind format, including exact partial-scaffold recovery and
focused Git/jj commits.

## Context

- Depends on `session-config-integrate-k21` and
  `session-kind-tree-integrate-k25`, plus the universal mutation seam established
  by `tree-access-lock-integrate-k56`.
- Binding design: `docs/specs/config-driven-sessions.md` sections "Fresh tree"
  and "Legacy migration", plus
  `docs/adr/promotion-transactions-fail-closed.md`.
- Primary code surfaces: `src/tree_migrate.rs`, `src/tree_lifecycle.rs`,
  `src/tree_access.rs`, `src/tree_rename.rs`, `src/repo.rs`, and
  `tests/migrate.rs` / `tests/root_init.rs` / Git-jj fixtures.
- Lifecycle ordering is wired by `lifecycle-cutover-k39`; expose one tested
  transition interface here rather than duplicating it in the driver.

## Done when

- Migration accepts the original directory tree, v1 dotted-flat tree, and v2
  body-kind tree; preserves positions/keys/outcomes/foreign files and stable
  relationships; maps aliases, standalone research, and structural vendor
  pairs exactly as specified; and rejects every ambiguous or unknown input.
- One fail-closed `MIGRATING-session-kinds` transaction stages a complete plan,
  blocks other readers/mutators, resumes every interruption point, rolls back
  reported pre-commit failures, verifies post-commit recovery, and writes
  `FORMAT` last.
- An exact partial scaffold left by current-format root initialization resumes
  under the universal mutation seam and writes `FORMAT` last; foreign partial
  trees are refused. Ordinary scaffold and grow-verb writing remain owned by
  `session-kind-tree-k23` rather than duplicated here.
- Migration commits only `.grove/` in plain Git and jj, excludes the witness,
  handles tracked deletion and unborn repositories where specified, and
  preserves unrelated staged/working-copy changes.
- Tests cover all mappings, collisions, terminal leaves, kind-prefixed legacy
  slugs, transaction recovery/rollback failure, exact partial root recovery,
  Git pathspecs, and native/colocated jj behavior.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

This node builds the deterministic lifecycle transition. It does not yet make
bare `grove` the sole caller.

## Decomposition

- `session-kind-plan-k93`: compute and validate the complete current-format
  destination without mutating the tree.
- `session-kind-transaction-k94`: stage, land, roll back, and recover the
  fail-closed migration transaction.
- `partial-root-recovery-k95`: recognize and complete only the exact fresh-tree
  partial scaffold.
- `migration-scoped-commit-k96`: record migration through focused plain-Git and
  Jujutsu commits without absorbing unrelated work.
- `migration-transition-k97`: expose and exercise the single lifecycle
  transition interface consumed by the later driver cutover.
