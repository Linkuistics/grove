# tree-access-lock-k52

**Kind:** impl

## Goal

Move every current task-tree observation and mutation onto one advisory lock on
the open working-tree root, including serialization of `.grove/` creation.

## Context

- Depends on `session-kind-tree-integrate-k25`.
- Binding design: `docs/adr/promotion-transactions-fail-closed.md` and
  `docs/specs/config-driven-sessions.md` section "Process ownership and session
  epochs" under the universal Tree access seam.
- Primary code surfaces: `src/tree_access.rs`, tree readers/grow/lifecycle/
  promotion entry points, driver selection, and root-init/concurrency fixtures.
- Migration and finish do not exist yet; expose the same lock-neutral internal
  seam those later slices can consume without retrofitting existing verbs.

## Done when

- Readers take a shared advisory lock and mutators an exclusive lock on an open
  working-tree-root descriptor; contention reports once then waits, aliases
  contend on one identity, and every descriptor is close-on-exec.
- Root initialization holds the exclusive lock across absence checks and the
  complete `.grove/` scaffold, so creation is serialized even though the grove
  directory did not exist at acquisition.
- Current pick/resolve/brief-chain, grow, lifecycle, terminal, and promotion
  operations acquire once and pass the guard into lock-neutral helpers; pending
  promotion remains fail-closed and keeps its own recovery witness.
- Driver selection copies its factual result and releases the shared guard
  before configuration reload or launch, so a session can mutate without
  deadlock.
- Tests cover root-init races, reader/mutator contention, alias paths, process
  exit, descriptor inheritance, and unchanged Git/jj behavior; `cargo fmt
  --check` and `cargo test --locked` pass.

## Notes
