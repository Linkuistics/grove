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

## Notes

**Outcome.** Placement went to `tests/support/mod.rs`, as the Context expected;
`readiness` and its panicking `wait_for_ready` form are now `pub` there and both
`driver_lease` and `loop_driver` reach them, with no local re-declaration left in
either. The seam's three regressions stayed in `tests/driver_lease.rs` rather
than moving beside it: `tests/support/mod.rs` is compiled into every consumer
binary, so a `#[test]` there would run one copy per binary, and `driver_lease` is
the binary whose fixtures can hold a producer back past the removed deadline.
Both files now say so, and `finish_lifecycle`'s surviving `wait_for` carries a
note that it is deliberately *not* the seam.

`loop_driver`'s driver is now spawned with both streams to a file instead of
`Stdio::null()`. The report path was verified by breaking the fixture — a
configured command that exits 7 without writing the marker — which failed in
1.5s quoting the exit status, the fake harness's stderr, and the driver's own
`session ended without a completion signal` line. Under the old twenty-second
`Path::exists` wait the same failure read `timed out waiting for the configured
session to launch`, with the account discarded.

Verification: three consecutive clean `cargo test --locked` runs, plus eight
concurrent copies each of `loop_driver` and `driver_lease` (all 16 green). The
stress load is real — `driver_lease` stretched 9s to 45s and `loop_driver` 43s to
63s — which is the regime that expired the old budgets.

**Checked and cleared, so no leaf cut for it:** `tests/tree_access.rs`'s
`wait_until_contended` also carries a five-second budget, but it is not this
family. Its predicate is stderr *content* rather than file existence, it already
samples the child's liveness and reports an ended one with its status and stderr,
and its child is a cheap `grove-llm` tree verb rather than the driver's cold
start (skill provisioning, `grove-llm --version`, tree transition). Under the
same 16-way load the whole four-test binary finishes in ~1.0s — a 5× margin.
