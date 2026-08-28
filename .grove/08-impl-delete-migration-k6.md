# delete-migration-k6

## Goal

Delete the migration path and the legacy tree format outright. No legacy tree
needs either, and a genuinely legacy tree is meant to fail on its **names** —
through `TaskNameError`, which already carries what is on disk and what it
should be — rather than be repaired.

## Context

- `docs/specs/module-decomposition.md`, `## Out of scope` — *"Migration.
  Deleted rather than preserved."*
- `decomposition-k2`'s `## Decisions (running log)`, the paragraph beginning
  **"`.grove/FORMAT` is deleted with migration"** — the warrant, including why
  the discriminator is not load-bearing on anything that survives.
- `minimalism-k1`'s `## Deletion list`, *Contained* row 2: `tree_migrate`,
  `tree_migration_transaction`, `repo/migration_commit`, **3,373 non-test
  lines**, with **no caller outside the deleted set** — 4 + 4 sites, all in
  `tree_lifecycle` and in each other; `migration_commit` has one caller,
  `repo.rs`.

Suites: `tests/migration_commit.rs` (663), `tests/migration_transition.rs`, and
`tests/lifecycle_cutover.rs` (1,946) — `minimalism-k1` names the last as a suite
for deleted machinery; confirm that from its contents rather than from the name
before removing it.

## Done when

- `src/tree_migrate.rs`, `src/tree_migration_transaction.rs` and
  `src/repo/migration_commit.rs` are gone, with every `tree_lifecycle` and
  `repo.rs` call site reconciled.
- `.grove/FORMAT` is deleted from this tree, and nothing writes, reads or
  requires it. `src/tree_format.rs` goes with it or shrinks to whatever survives
  the discriminator's removal — see the open question below.
- The migration suites are deleted; `cargo test` and
  `cargo clippy --all-targets` are clean.
- Any auto-repair function that existed only to unwind a half-run migration goes
  with it, becoming nothing (principle 2: a message, not machinery — and here
  there is no longer even an anomaly to report).
- `CHANGELOG.md`'s `## Unreleased` records the removal.

## Notes

**Lands green.** This is a contained deletion: the caller counts in
`minimalism-k1` were taken against `src/` on 2026-08-28 and exclude the deleted
set's own subtree. Re-derive them rather than trusting them — the count is a
year-zero measurement, not a contract.

**One open question the design log settles against the requirements.**
`minimalism-k1`'s *"Not deleted, and worth saying so"* lists `tree_format` as
surviving; `decomposition-k2` then decided `.grove/FORMAT` is deleted with
migration. The later decision governs the *file*. Whether the *module* has
anything left to do once `require_current` and the discriminator go is this
session's call — if it does not, delete it and say so; the requirements sentence
was written before the FORMAT decision existed and is superseded, not
contradicted.

**Do not touch the finish transaction here.** It is `delete-finish-transaction-k8`'s,
and the two share a shape (`*_transaction.rs`, quarantine, rollback) that makes
it easy to over-reach. This leaf's blast radius is migration and FORMAT only.
