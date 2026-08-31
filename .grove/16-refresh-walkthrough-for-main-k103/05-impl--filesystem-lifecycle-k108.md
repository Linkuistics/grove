# filesystem-lifecycle-k108


## Goal

Refresh the filesystem boundary, lifecycle, rollback, and error exposition
against the current interpreter sources.


## Context

- Source roots: `crates/ordinal-fs-tree/src/error.rs`, `src/fs/mod.rs`,
  `src/fs/apply.rs`, and `src/fs/remove.rs`. `src/fs/remove.rs` is a new
  production root absent from the pre-rebase fixed inventory and must be added
  to the book system.
- Book surfaces: the shared filesystem fragments in `04-read-path.md`,
  `06-filesystem-interpreter.md`, and `source-index.md`.
- This child owns all fragments of `src/fs/mod.rs` so its cross-chapter root is
  reconciled atomically.

## Done when

- All four owned roots tangle byte-for-byte and their inventory entries are
  current.
- Exposition accurately covers current guards, lock lifecycle, application,
  rollback, deletion, reports, errors, and concurrency trade-offs.
- Full validation has no mismatch for an owned root.

## Notes

Treat updated error and report surfaces as semantic changes requiring prose,
not only fragment replacement.
