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

## Done when

- A regression reproduces the empty-payload read deterministically, without
  changing production timing.
- The barrier publishes name and existence in one observable step, or every
  waiter is proven to block until the payload is complete.
- The full `finish_lifecycle` binary and `cargo test --locked` pass repeatedly.

## Notes

Do not widen this into the finish transaction: the barrier is a test seam, and
the cleanup behaviour it pauses was not implicated.
