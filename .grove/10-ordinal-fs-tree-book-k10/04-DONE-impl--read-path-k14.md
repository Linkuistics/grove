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

## Decisions (running log)

Use the reference-domain syllabus tree as the complete read example. Follow it
from `fs::read` through containing-directory locking, unfollowed listings,
consumer classification, the explicit directory worklist, builder finalisation,
walk order, and representative queries before expanding the individual APIs.

Refine `snapshot.rs` into storage, builder/order, entry views, containers, and
queries/walk fragments. Refine `fs/read.rs` into discovery, listing, and lock
location fragments. Keep each interleaved read-owned `fs/mod.rs` range as its
stable top-level literal fragment so later write/interpreter ownership remains
byte-adjacent and unambiguous.

State advisory-lock scope locally: cooperating readers share, cooperating
writers exclude them, and an uncooperating process can still change the
filesystem. Treat every snapshot view as borrowing the immutable snapshot owned
by `ReadGuard`, which keeps the lock descriptor alive for the same lexical
lifetime.

Present the worked walk as the public `(depth, entry.name())` result, using
indentation derived from depth rather than synthesising paths the snapshot API
does not return. Introduce every literal fragment immediately before its source
with the owning actor, input/output transition, invariant, and role in that
worked example.
