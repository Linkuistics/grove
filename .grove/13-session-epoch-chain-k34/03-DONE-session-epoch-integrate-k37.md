# session-epoch-integrate-k37

**Kind:** integrate-review-impl
**Integrates:** session-epoch-review-k36

## Goal

Apply the verified findings from `session-epoch-review-k36` while preserving the reviewed artifact's contract.

## Context

- Verify every `session-epoch-review-k36` finding against the protocol's
  cooperative-workflow and process-interruption scope.
- Keep the lease, epoch, and Tree access lock as three distinct interfaces with
  the specified acquisition order.

## Done when

- Every finding has a recorded disposition; verified races are fixed with a
  deterministic barrier/event-trace or black-box regression.
- No test-only clock, randomness, lock, or grace control becomes user-visible
  environment configuration.
- `cargo fmt --check` and `cargo test --locked` pass.

## Review dispositions

- **F1 — accepted and fixed.** A successful exclusive liveness probe now
  unlocks immediately, before descriptor/path identity validation. The
  `a_successful_liveness_probe_releases_the_lease_before_validation` barrier
  regression fails if the probe transiently impersonates a live driver again.
- **F2 — accepted as a test gap.** The production stop-before-signal ordering
  was already correct and is now enforced by a post-invalidation continuation
  test that cannot interpret the signal when invalidation fails. The
  process-level
  `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`
  regression SIGKILLs a TERM-ignoring foreground parent while its admitted
  background command holds the shared epoch guard, then proves the fixed
  30-second handoff returns before a 35-second watchdog, leaves the signal
  present, launches exactly once, and emits exactly one real contention line.
  A focused diagnostic regression also pins that the report names both lock
  mode and operation. No timeout or lock control was added to user
  configuration.
- **F3 — accepted and fixed.** Post-reap reconciliation keeps epoch
  invalidation authoritative, but a simultaneous launch failure is now retained
  in the same diagnostic instead of being discarded.
- **F4 — accepted as a documentation mismatch.** The code matches the approved
  spec: ambient `complete` must resolve the current directory to the session's
  exact working tree before signaling. `content/SKILL.md` now requires running
  the final command from that working tree rather than claiming any directory
  is valid.
- **F5 — accepted and fixed.** An inactive epoch now says it is inactive;
  rotated active epochs retain the distinct path-mismatch diagnostic.

## Notes

Any lifecycle policy change belongs to `lifecycle-cutover-k39`.
