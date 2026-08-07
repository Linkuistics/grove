# session-epoch-k35

**Kind:** impl

## Goal

Bind each foreground launch and ambient `grove-llm` operation to a live driver
epoch, move all tree operations onto the universal working-tree lock, and make
the loop signal path independently random per launch.

## Context

- Depends on `driver-lease-integrate-k33`.
- Binding design: `docs/adr/one-live-driver-per-working-tree.md`,
  `docs/adr/promotion-transactions-fail-closed.md`, and the process-ownership
  and toolchain sections of `docs/specs/config-driven-sessions.md`.
- Primary code surfaces: the process-ownership module, `src/tree_access.rs`,
  `src/loop_driver.rs`, `src/complete.rs`, `src/llm_cli.rs`, `src/launch.rs`,
  `.cargo/config.toml`, and `tests/support/mod.rs` / env-hygiene / concurrency
  fixtures.

## Done when

- The driver installs inactive/active/inactive epoch records at the three
  specified points under separately scoped exclusive guards, with exact nonce,
  worktree identity, and fresh random signal path; guards never cross spawn or
  another acquisition.
- Ambient tree operations and `complete` hold one shared epoch admission guard
  through tree access, validate exact context, probe lease liveness, reject
  stale/wrong-worktree calls, and leave manual calls without loop context
  available; `--version` and `report-turn` remain exempt.
- Every epoch acquisition reports contention once and has the fixed internal
  30-second bound. Deterministic backend barriers/event traces prove open-lock-
  stat retries, guard lifetimes, handoff ordering, and the orphan bounded-stop
  path without exposing test controls as user configuration.
- Tree readers/mutators lock the open working-tree root across root-init,
  migration, ordinary verbs, promotion, and finish deletion; driver reads
  release that guard before launch and every descriptor is close-on-exec.
- Each launch draws an independent OS-random 128-bit signal suffix, retries
  occupied paths, cleans abandoned signals only after exclusive crash handoff,
  and records the accepted collision bound without claiming literal non-reuse.
- Meta-grove env guards scrub the live signal/epoch authority from the suite;
  tests cover admitted-old/invalidated-new calls, orphan timeout, old signals,
  finish-delete/root-recreate handle reuse, and no operation/launch overlap.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Preserve promotion and migration witnesses: the lock serializes live processes
but does not replace their interruption-recovery transactions.
