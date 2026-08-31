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

## Decisions (running log)

The existing source-coherent fragment IDs remain stable. The snapshot split
keeps storage, builder, entry views, containers, and queries as its five literal
children, with the container/query boundary moving to `540/541`; the filesystem
read split keeps discovery, listing, and lock-location children, with shared
listing errors ending at line 155 and root presence plus lock routing occupying
lines 156–407.

This leaf updates read-model exposition that depends on current `src/fs/read.rs`
and `src/snapshot.rs`, including `Sought`, shared listing errors, root presence,
and dangling-symlink handling. It does not refresh or repartition any
`src/fs/mod.rs` fragment; that shared source root remains wholly owned by
`filesystem-lifecycle-k108`.

The validator's fixed corpus, its test fixture, and the book-system spec are
current-state mirrors of the source ledger rather than historical snapshots.
This leaf updates the two owned root lengths and full-file ownership blocks in
all three, including the derived `read-path-k14` and corpus line totals in the
spec, so validation can assess the refreshed fragments instead of rejecting the
ledger against stale constants.
