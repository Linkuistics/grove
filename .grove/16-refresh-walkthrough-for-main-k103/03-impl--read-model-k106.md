# read-model-k106


## Goal

Refresh the snapshot and filesystem-read walkthrough slices against the current
read model.


## Context

- Source roots: `crates/ordinal-fs-tree/src/snapshot.rs` and
  `crates/ordinal-fs-tree/src/fs/read.rs`.
- Book surfaces: `04-read-path.md` and `source-index.md`.
- `src/fs/mod.rs` is shared with filesystem lifecycle and remains owned by
  `filesystem-lifecycle-k108` so one child reconciles that root atomically.

## Done when

- The snapshot and filesystem-read roots tangle byte-for-byte and their
  inventory entries are current.
- The chapter accurately explains current discovery, validation, snapshot
  construction, queries, and read-side failures.
- Full validation has no mismatch for either owned root.

## Notes

Do not partially refresh the shared `src/fs/mod.rs` root.
