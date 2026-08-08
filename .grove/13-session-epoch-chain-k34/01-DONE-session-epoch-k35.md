# session-epoch-k35

**Kind:** impl

## Goal

Bind each foreground launch and ambient `grove-llm` operation to a live driver
epoch, closing stale-session and crash-handoff races around the already-landed
lease, signal-channel, and Tree access seams.

## Context

- Depends on `driver-lease-integrate-k33`,
  `tree-access-lock-integrate-k56`, and `session-signal-path-integrate-k61`.
- Binding design: `docs/adr/one-live-driver-per-working-tree.md`,
  `docs/adr/promotion-transactions-fail-closed.md`, and the process-ownership
  and toolchain sections of `docs/specs/config-driven-sessions.md`.
- Primary code surfaces: the process-ownership module, `src/loop_driver.rs`,
  `src/complete.rs`, `src/llm_cli.rs`, `src/launch.rs`,
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
  available; `--version` remains exempt.
- Every epoch acquisition reports contention once and has the fixed internal
  30-second bound. Deterministic backend barriers/event traces prove open-lock-
  stat retries, guard lifetimes, handoff ordering, and the orphan bounded-stop
  path without exposing test controls as user configuration.
- Epoch and Tree access acquisition obey the fixed order: an ambient command
  retains one shared epoch guard through one separately acquired tree operation,
  while the driver releases exclusive epoch and Tree access guards before
  spawn or another acquisition. Abandoned signal cleanup occurs only after the
  replacement installs its inactive epoch.
- Meta-grove env guards scrub the live signal/epoch authority from the suite;
  tests cover admitted-old/invalidated-new calls, orphan timeout, old signals,
  direct tree removal followed by root-init handle reuse, and no
  operation/launch overlap. Finish-helper deletion coverage belongs to
  `finish-lifecycle-k43`.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Preserve promotion and migration witnesses: the lock serializes live processes
but does not replace their interruption-recovery transactions.
