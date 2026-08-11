# loop-driver-readiness-deadline-k170

**Kind:** impl

## Goal

Put the remaining driver-process readiness wait, in `tests/loop_driver.rs`, on
observable child state, and decide where the shared readiness seam lives.

## Context

- Surfaced while fixing `driver-lease-readiness-flake-k145`, which replaced
  `tests/driver_lease.rs`'s fixed five-second `wait_for` with a `readiness`
  helper that conditions on the producing child's liveness and reports an ended
  producer with its exit status and captured output.
- `tests/loop_driver.rs:536` (`a_sigtermed_driver_stops_and_reaps_its_child`) is
  the same shape one binary over: a driver spawned with both streams nulled, and
  `wait_for(&launched, …)` polling `Path::exists` against a fixed twenty-second
  deadline. It has not been observed failing — twenty seconds is four times the
  budget that did fail — so this is a latent instance, not a live flake.
- The decision this leaf actually owns is placement. `readiness` is currently
  private to the `driver_lease` binary; a second consumer makes it either a
  duplicated helper or an item in `tests/support/mod.rs`, which every
  process-driving binary already compiles in.
- Deliberately **not** in scope: the barrier waits in `tests/finish_lifecycle.rs`
  and `tests/leaf_promote_chain.rs`, owned by
  `cleanup-barrier-readiness-flake-k165`, and
  `tests/finish_lifecycle.rs:1574`'s wait on a file written by an *orphaned
  grandchild* after its driver was reaped — that one has no live process handle
  to condition on and needs its own answer.

## Done when

- `a_sigtermed_driver_stops_and_reaps_its_child` waits on child liveness rather
  than a fixed deadline, and reports an ended driver with its diagnostics.
- The readiness seam has one home, and any binary that drives a process reaches
  it there rather than re-declaring it.
- `cargo test --locked` passes repeatedly, including under a parallel-copies
  stress of the affected binaries.
