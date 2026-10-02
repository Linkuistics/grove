# fork-sensitive-pin-test-k18

## Goal

`selected_root_pin_releases_tree_guard_and_survives_path_replacement` in
`crates/grove-loop/src/loop_driver.rs` stops failing when a sibling unit test
forks at the wrong moment.

## Context

- The test selects a leaf, then takes a non-blocking exclusive `flock` on the
  working tree to show that selection released its guard. A lock survives a
  fork until the child execs, so a parallel test that spawns a process can hold
  the selection's shared lock a moment past its release.
- `crates/grove-loop/src/driver_lease.rs` already states that mechanism and
  answers it for its own tests:
  `fork_sensitive_driver_lease_test_body_runs_here` re-runs the assertion in a
  child test process with no parallel siblings.
- Seen in `lifecycle-launch-k12`: one failure in eleven runs of
  `cargo test -p grove-loop --lib`, at that `flock`, under a full workspace
  run. Six runs of the `loop_driver` tests alone passed.

## Done when

- The test's lock assertion cannot be extended by another test's fork.
- The `grove-loop` book is valid for what changed.
- `bash scripts/check.sh` passes.

## Notes

- Nothing here belongs to the dispatch work. It was cut from that grove because
  it surfaced there.
- Confirm the mechanism before fixing it. The evidence is one failure and a
  matching comment, not a reproduction.
