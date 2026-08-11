# cleanup-barrier-readiness-flake-k165

**Kind:** impl

## Goal

Make the finish-cleanup pause barrier publish its payload atomically, so a test
that waits on the barrier cannot read a created-but-empty file.

## Context

- Surfaced while verifying `post-teardown-restart-contract-k102`; that diff adds
  only two `finish_lifecycle` tests and touches no cleanup or barrier code.
- `cargo test --locked` failed
  `process_cleanup_does_not_unlink_a_substituted_non_directory_entry` at
  `tests/finish_lifecycle.rs` with `paused cleanup entry` — `find_entry_named`
  found nothing for the name read out of the barrier. It passed immediately in
  isolation, so the trigger is parallel-suite load, not the assertion.
- Leading hypothesis, not yet proven: `finish_cleanup.rs`'s pause hook publishes
  with `fs::write(&barrier, detail)`, which creates and then writes, while the
  test's `wait_for` polls `Path::exists`. A waiter scheduled between the two
  reads zero bytes and then searches the quarantine for the empty name.
- The same create-then-write/exists-then-read pair is shared by every
  `GROVE_TEST_FINISH_CLEANUP_PAUSE_AT` and rebind barrier consumer, so the fix
  belongs at the seam rather than in one test.
- Distinct from `driver-lease-readiness-flake-k145`: that one is a readiness
  *deadline* on a fake harness in the `driver_lease` binary; this is a
  partially-observable *payload* in the cleanup barrier. Same family, different
  mechanism — check whether one fix covers both before assuming it does not.
- **Answered on that leaf's close: one fix does not cover both.** k145 replaced
  its deadline with a wait conditioned on the producing child's liveness, which
  decides only *whether the file will ever appear*; it still returns on the
  first `exists`, so an existing-but-empty barrier reads exactly as before. The
  two are independent — publish the payload atomically here regardless of what
  the waiter does.
- **Second sighting**, while verifying
  `adoption-migration-session-kind-transition-k98` (a test-only diff touching
  neither cleanup nor the barrier). One `cargo test --locked` aborted at
  `passed: 661`; since cargo stops at the failing *binary* and the preceding
  ones total 603, the failure was `finish_lifecycle`'s 59-test binary at 58
  passed + 1 failed. The failing test's name was not captured, so treat this as
  corroboration of the binary and the parallel-load trigger, not of the specific
  assertion. Seven consecutive full runs after it were clean, which sets the
  observed rate low enough that a fix needs the deterministic reproducer the
  `Done when` already asks for — a plain re-run proves nothing here.

## Done when

- A regression reproduces the empty-payload read deterministically, without
  changing production timing.
- The barrier publishes name and existence in one observable step, or every
  waiter is proven to block until the payload is complete.
- The full `finish_lifecycle` binary and `cargo test --locked` pass repeatedly.

## Notes

Do not widen this into the finish transaction: the barrier is a test seam, and
the cleanup behaviour it pauses was not implicated.

**Outcome.** The hypothesis held. `fs::write` is create-then-write, and the
waiter's readiness test was `Path::exists`; publication now stages a sibling and
`rename(2)`s it onto the barrier name, through one `src/test_barrier.rs` shared
by the cleanup and promotion seams. Both regressions were checked against the
pre-fix code: the seam test read the entry name off the hard-linked witness, and
the waiter test observed `left: []` — the flake's own empty payload.

The rename made the publish four syscalls instead of one, and that widened a
*second*, pre-existing hazard into a ~50%-per-run failure: two cleanup unit
tests set `GROVE_TEST_FINISH_CLEANUP_*` process-globally while every sibling
cleanup test read them, so a victim published to the setter's barrier and
blocked the 30s pause timeout. Fixed here by splitting `pause_at_cleanup_barrier`
out of `cleanup_test_checkpoint`, which removed both tests' env dependence.
`unit-test-env-isolation-k171` carries the two instances of that class left in
`src/` (`GROVE_SIGNAL_FILE`, `HOME`).
