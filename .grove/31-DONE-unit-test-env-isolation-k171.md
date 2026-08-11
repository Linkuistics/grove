# unit-test-env-isolation-k171

**Kind:** impl

## Goal

Stop the remaining in-crate unit tests from mutating process-global environment
variables that production code reads, so a sibling test running in parallel
cannot be handed another test's value.

## Context

- Surfaced while fixing `cleanup-barrier-readiness-flake-k165`, which proved
  the failure mode is real rather than theoretical: two cleanup tests set
  `GROVE_TEST_FINISH_CLEANUP_PAUSE_AT` / `_BARRIER` under a private mutex, and
  every *other* cleanup test read them without taking it. A victim that reached
  the named step published to the setter's barrier and blocked the full 30s
  pause timeout. Signature: a lib run of ~8s stretching to 30.2s with 1–10
  failures, victims varying per run (`corrupt_cleanup_markers_...`,
  `a_stale_marker_...`, `a_symlink_substituted_before_unlink_...`). Removing
  the env dependence gave 8/8 clean ~7.9s runs.
- Two instances remain, each guarded by its own mutex — which excludes only the
  tests that take that mutex, not the readers:
  - `src/driver_lease.rs` `SignalEnvironment` sets `GROVE_SIGNAL_FILE`
    (`ENV_LOCK`). Production readers: `src/driver_lease.rs:762`,
    `src/finish_transaction.rs:1173`, `src/complete.rs:82`.
  - `src/provision.rs` `skill_dirs_follow_each_harness_layout` sets `HOME` to
    `/home/x` (`ENV_LOCK_FOR_HOME`). Production readers: `src/provision.rs:72`,
    `src/loop_driver.rs:89`.
- Both windows are short (a few pure-function assertions), and neither has been
  observed failing — this is removal of a proven hazard class, not a chase.
- The fix shape that worked for the cleanup barrier: split the seam so the
  env-reading half stays in production and the half a test needs to pin takes
  its input as an argument (`finish_cleanup.rs`'s `cleanup_test_checkpoint`
  versus `pause_at_cleanup_barrier`). `skill_dir_for` and the signal resolvers
  may already admit the same split by taking the resolved path/home.
- Integration binaries are not in scope: `tests/support/mod.rs` already carries
  `lock_env`/`EnvGuard` for them, and each is a separate process.

## Done when

- No `#[cfg(test)]` module under `src/` calls `env::set_var` / `env::remove_var`
  for a variable production code reads, or each remaining call is shown to be
  unreachable by any parallel sibling.
- The env-var plumbing those tests covered is still covered — by the black-box
  integration tests that set the variable on a real subprocess, or by a test of
  the reading half that names the variable.
- `cargo test --locked` passes repeatedly, with lib-binary wall time steady.

## Notes

Rust is moving the same way: `std::env::set_var` is `unsafe` in edition 2024
precisely because another thread may be reading. Removing these calls is not
only flake removal — it is what the next edition bump will otherwise force.
