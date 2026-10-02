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

## Decisions (running log)

**The mechanism is confirmed by reproduction.** A scratch test ran this test's
own selection and lock probe 3,000 times per setting, alone in the binary: the
probe was refused 0 times with nothing spawning, 157 with four threads spawning
`/usr/bin/true` and 589 with eight. The descriptor count read zero in every
iteration of the same run. The scratch test is not committed.

**The fix asks the descriptor count, not the isolated child.** The crate has
two answers to this window. `driver_lease.rs` re-runs a test in a child process,
and `task_grow/tests.rs` counts this process's descriptors on the directory with
`descriptors_held_on`. The book's chapter 17 already names the second as what an
assertion of this shape asks, it needs no subprocess, and a lock needs a
descriptor to live on. `TreeLifetime` pins `.grove`, not the containing
directory, so a retained pin does not disturb the count. Cost: `task_grow`'s
test module and that one function became visible to the crate.

**The control sits before selection.** With the sentinel after `picked`, a
mutant that leaked the guard failed at the unlabelled control (2, not 1). Moved
ahead, the same mutant fails at the assertion that names the fault.
