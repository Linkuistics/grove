# lease-job-k19

## Goal

Move the driver lease and its tests to `LaunchEvent` of `ravel_job`. This is the
first half of `lifecycle-job-k9`. The loop driver still starts `ravel` through
`keyed-launch` after this leaf.

## Context

- The brief of `lifecycle-job-k9` and the running logs of `ravel-job-k7` and
  `grove-run-job-k8`. They give the chosen names: `ravel_job::launch`, `Shape`,
  `JobEnd`, `JobError`, `LaunchEvent` and `scrub`.
- `crates/grove-loop/src/driver_lease.rs`, `driver_lease/observation.rs` and
  `driver_lease/witnesses.rs`. The tests in these files spawn `/bin/sh` through
  `keyed_launch::run_observed` with a `Channel`.
- `crates/grove-loop/src/loop_driver.rs`, `launch_session`. It hands the lease
  an observer. The lease now expects the new type, and the old runner still
  emits `keyed_launch::LaunchEvent`.
- The book of `grove-loop`: chapters 16 and 17, with the concept index, the
  source index and the README where they name the changed lines. The corpus is
  every `.rs` file of `crates/grove-loop/src`.
- Measured before the cut: chapter 16 has 217 hard findings of the checker, and
  chapter 17 has 205. Convert only the sections that this leaf edits.

## Done when

- `DriverLease::supervise_launch` takes `ravel_job::LaunchEvent`. It receives
  Started after a spawn and Reaped after a wait, as before.
<!-- tc: depart synonym-rotation because Launch directory is a defined glossary term -->
- The call to `Channel::discard_abandoned` is gone. The cleanup of abandoned
  launch directories stays. Its comment no longer names `keyed-launch`.
- `launch_session` in `loop_driver.rs` converts the observer at its one call
  site. It keeps the `keyed-launch` dependency of `grove-loop`. The next leaf
  removes the conversion and the dependency.
- The tests of the lease that spawn `/bin/sh` call `ravel_job::launch` with the
  foreground shape. They check the order of the notices and run without a
  terminal. A launch directory replaces the `Channel`. No test of the lease
  uses `Channel`, `run_observed`, `Launch` or `Escalation`.
- `keyed_launch` appears in these three files only in history comments that are
  still true, and preferably not at all.
- The `grove-loop` book matches every changed line. The conversions of the
  touched sections land first, each as its own change.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set:
  `test:grove-loop` and `ravel:activation`.

## Notes

- Keep the behaviour as it is. The leaf `driver-job-k20` owns the loop driver.
- The tests need a launch directory. `LaunchDirectory::allocate` in
  `launch_directory.rs` makes one under the control directory of the lease.
- `ravel_job::launch` needs a regular file for standard output. `SessionResult`
  or `tempfile` gives one.
- Record any change of name or shape in the running log, so the next leaf uses it.

## Decisions (running log)

- **Lease.** `DriverLease::supervise_launch` takes `ravel_job::LaunchEvent`. The
  call to `Channel::discard_abandoned` and its comment are gone. The existing
  call of `LaunchDirectory::discard_abandoned` is the whole cleanup.
- **Bridge.** `launch_session` in `loop_driver.rs` wraps the observer in a closure
  that maps `keyed_launch::LaunchEvent` to the new type. It adds six lines to
  `loop_driver.rs`. `driver-job-k20` removes the closure and the dependency.
- **Test helper.** `driver_lease::tests::launch_foreground` runs a program through
  `ravel_job::launch` in the foreground shape. It sends standard output to a
  scratch file. The tests of `observation.rs` and `witnesses.rs` reach it as
  `super::super::tests::launch_foreground`. A `LaunchDirectory` from the control
  directory replaces each `Channel`. The shell exits by itself, so the old
  `keyed_launch::signal` call is gone.
- **Fixtures.** The tests that do not spawn keep their `signal-test` paths, which
  are only strings that name a launch directory. I did not rename them.
- **Housekeeping case.** The case `a_done_signal_finishes_the_loop_once_and_housekeeping_stays_advisory`
  in `crates/grove/tests/loop_driver.rs` expected the removed channel cleanup, so
  it failed. It now plants an unremovable `launch-` directory (mode 0500) and an
  unrelated name. It checks the warning `could not clean abandoned launch
  directories`, the clean finish, and that the unrelated name stays. It no longer
  asserts no removal of an abandoned directory, because the cleanup stops at the
  first error and the directory order is not fixed. `replacement_driver_removes_abandoned_launches_without_interpreting_contents`
  covers the removal. This was the `signal-` item of `driver-job-k20`, so that leaf
  has less to do.
- **Book.** Fragments and ledgers moved by script from a diff of the old and new
  source. The script is not in the repository. It remapped the cut points of each
  fragment, rewrote the manifest rows and the ledgers of `source-index.md`, and
  recomputed the slice totals. Totals now: `driver_lease.rs` 2,074 lines,
  `observation.rs` 2,117, `witnesses.rs` 697, `loop_driver.rs` 918. The book has
  15,221 lines. Chapter 20 and the README carry the new total and the chapter
  sequence. `driver-job-k20` must recompute them again.
- **Prose.** I changed only the claims that this change made false. They are the
  use of `keyed_launch` in the imports of the lease, step 6 of the acquisition
  with its two paragraphs, and the doc comment of `control_dir`. They also
  include four sentences of chapter 17, one sentence of chapter 19 and one entry
  of the concept index. The earlier change `lease-job-k19: convert the touched
  passages` holds the conversions of those passages.
- **Finding.** Many source line numbers in the prose of chapters 16 and 17 already
  disagreed with the source before this leaf. Example: step 5 cites lines 184 and
  186 for lines 178 and 180. Nothing gates them. I changed only step 6 and left
  the rest. A later leaf or the review may decide whether to update them.
- **Result.** `bash scripts/check.sh` passes all 20 checks. `task ravel:activation`
  passes with the installed Ravel 1.1.0, and it includes `test:grove-loop`,
  `ravel:job`, `test:grove-llm`, `test:keyed-launch` and `test:authority`.

