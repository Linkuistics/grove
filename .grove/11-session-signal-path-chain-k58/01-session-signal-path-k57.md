# session-signal-path-k57

**Kind:** impl

## Goal

Give every foreground launch an independently random loop-control channel in
the workspace administration area, with collision retry and crash cleanup
behind the live driver lease.

## Context

- Depends on `driver-lease-integrate-k33`.
- Binding design: `docs/adr/one-live-driver-per-working-tree.md` and
  `docs/specs/config-driven-sessions.md` section "Process ownership and session
  epochs" through signal-path lifecycle.
- Primary code surfaces: the process-ownership module, `src/loop_driver.rs`,
  `src/complete.rs`, and deterministic randomness/cleanup fixtures.
- This slice establishes fresh signal channels under the lease. The next epoch
  slice binds the accepted path to ambient `grove-llm` authority and moves
  abandoned cleanup after its inactive-record handoff.

## Done when

- Each launch draws an independent OS-random 128-bit signal suffix in the
  exact workspace control directory, rejects occupied draws, and never derives
  a path from PID, time, address, iteration, or task identity.
- The current driver removes only its accepted channel after reap and signal
  interpretation; a replacement may clean abandoned channels only after it
  owns the driver lease, through an operation the epoch slice can sequence after
  exclusive invalidation.
- Injected randomness and filesystem seams deterministically cover occupied
  draws, retries, cleanup, and old-signal isolation without becoming supported
  environment configuration.
- Documentation records the accepted one-in-`2^128` cross-restart collision
  bound rather than claiming literal non-reuse.
- `cargo fmt --check` and `cargo test --locked` pass with the existing
  completion protocol still green.

## Notes
