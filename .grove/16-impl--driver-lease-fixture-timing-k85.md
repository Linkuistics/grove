# driver-lease-fixture-timing-k85

## Goal

Make two fixtures in `crates/grove-loop/tests/driver_lease.rs` —
`a_second_driver_refuses_before_tree_access_or_launch` and
`a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` — pass
reliably under `cargo test --locked --workspace`, the form `scripts/check.sh`
runs them in.

## Context

- Observed at `the-surface-k78`, a documentation leaf whose diff touches nothing
  these fixtures read. Under the workspace-wide run both panicked in
  `testing/support.rs`'s `wait_for_ready` with *nothing wrote first-ready: the
  process behind it is still running after 120s*. Rerun alone, each passes — in
  117s and 103s. The whole `grove-loop` test crate passes alone, and so does
  `crates/grove/tests/loop_driver.rs`, whose own run under the same load had
  one fixture fail and another never return.
- The shape is a readiness deadline of 120 seconds guarding work that needs
  over 100 seconds on an idle machine. Under the parallel load of every crate's
  tests at once it overruns. Whether the 100 seconds is inherent to the fixture
  — a driver waiting out a real lease or epoch interval — or is itself a
  sleep-and-poll that could be event-driven is the first thing to establish;
  raising the deadline is the fallback, not the fix.
- The first sign was a stalled background run of `scripts/check.sh` that sat
  for 2h40m with no output and no child process after `loop_driver.rs`'s
  `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal` stopped
  reporting. That fixture passes alone in 38s. A test that can wedge without a
  child to wait on is the same family of problem and belongs in this leaf's
  reading.
- **Reproduced at `forward-commitment-tests-k131`, and one hypothesis measured
  out.** A `scripts/check.sh` run wedged for over an hour in that same fixture,
  with the `loop_driver` binary at 0% CPU and no children. The obvious suspect
  was `GROVE_SIGNAL_FILE`, which is set in every session's environment; it is
  **not** implicated. Run alone with the variable pointed at a scratch path, all
  11 `loop_driver` fixtures pass in 41.5s and nothing writes the file; run alone
  with it unset they pass in 39s; run under full workspace parallelism with it
  unset, `an_orphaned_epoch_guard_...` and
  `a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver`
  both fail in 104s. Load is the whole variable, as this leaf already says. A
  later full `scripts/check.sh` on an idle machine passed `cargo test`
  outright — so the wedge is intermittent under load, not a hard stop, and a
  green run is not evidence the deadline is adequate.

## Done when

- The two named fixtures pass under `cargo test --locked --workspace` on three
  consecutive runs, and the reason each needed over 100 seconds is stated in
  the fixture or removed.
- `bash scripts/check.sh` passes.
