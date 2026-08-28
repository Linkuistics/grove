# read-path-k14

## Goal

Follow a tree read from directory entries through validation and snapshot
construction into traversal, lookup, ancestor, and distinguished-chain behavior.

## Context

- Inputs: `reference-domain-k13`, `book-system-k6`, and this subtree's brief.
- Primary source emphasis: `src/snapshot.rs`, `src/fs/read.rs`, and the read side
  of `src/fs/mod.rs` as assigned by the design ledger.

## Done when

- The filesystem boundary, unfollowed file-kind observation, foreign-name skip,
  malformed/reserved halt, recursion, and deterministic sibling order form one
  explicit data-flow account.
- Snapshot storage and borrowed views explain roots versus entries, levels,
  walk order, lookup by key, predicates, ancestors, and distinguished chains.
- Read locking and the lifetime of the immutable snapshot are related directly
  to the public `ReadGuard` behavior.
- Error/refusal distinctions encountered on reads are stated locally with their
  recovery implications.
- Assigned fragments tangle exactly and scoped source, Markdown/link, and crate
  verification pass.

## Notes

Use one complete read example before expanding individual query operations.
