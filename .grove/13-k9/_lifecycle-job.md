# lifecycle-job-k9 — brief

## Goal

Move the loop driver and the driver lease to the shared function in its
foreground shape. After this leaf the lifecycle sessions run as ordinary
children, with no group and no KILL.

## Context

- The function of `ravel-job-k7`, and the first caller of `grove-run-job-k8`.
  Read their running logs.
- The root brief: *Requirements* (all items), *Design* sections *New homes*
  and *Tests*, and *Residues*.
- The section *Lifecycle interpretation* of the caller contract. Rule 4 goes.
- The code: `loop_driver.rs`, `driver_lease.rs`, `driver_lease/observation.rs`
  and `driver_lease/witnesses.rs` of `grove-loop`, and `lib.rs` with its
  re-export of `reraise`.
- The book of `grove-loop`: chapters 01 and 16 to 20, with the concept
  index, the source index and the README.
- Current prose that describes the removed interpretation:
  `docs/walkthroughs/overview/03-three-steps.md:814` ("a surviving Ravel
  group ... stops"), `docs/walkthroughs/overview/04-proving-a-negative.md:263`
  (the surviving-group combinations as a runner-result input) and the doc
  comment at `crates/grove-loop/src/session_result.rs:26-30` (keyed-launch's
  entry point, "survivor checks").

## Done when

<!-- tc: depart synonym-rotation because Launch directory is a defined glossary term -->
- The loop driver starts `ravel` through the shared function. It uses the
  foreground shape with the launch directory. `GROVE_LAUNCH_DIR` goes to the
  lifecycle session only.
- `Group` and `Escalation` are gone from the interpretation. A surviving group
  no longer stops the loop. The order stays: driver interruption, then a
  successful status with `grove-force-stop`, then the teardown record, then
  `grove-continue`.
- The driver lease takes `LaunchEvent` from the new module. It receives Started
  after a spawn and Reaped after a wait. The call to
  `Channel::discard_abandoned` is gone. The cleanup of abandoned launch
  directories stays.
- The copy of the scrub list in `loop_driver.rs` is gone, so one list remains
  after `grove-run-job-k8` removed the other. `stty` uses the helper.
- The tests of the driver lease that spawn `/bin/sh` use the new function. They
  check the order of the notices and run without a terminal. A launch
  directory replaces the `Channel`.
- The two interpretation tests in `loop_driver.rs` are rewritten without
  `Group`. The `signal-` fixtures are gone from the housekeeping case.
- `crates/grove-loop/Cargo.toml` no longer depends on `keyed-launch`, because
  this leaf removes the last use.
- `reraise` is re-exported from the new module. The conversions of `Argv` at the
  call sites are gone.
- The loop-driver cases of `crates/grove/tests/loop_driver.rs` under a
  pseudo-terminal still pass with the installed Ravel 1.1.0. These cases are the
  acceptance of the foreground job.
- The `grove-loop` book matches every changed line. The conversions of the
  chapters land first as their own changes.
- Chapters 03 and 04 of the `overview` book no longer describe a surviving
  group as a stop condition or a result input. The `session_result.rs` doc
  comment describes the held file and the shared function, with no survivor
  check, and the chapter fragment that quotes it matches. The conversions of
  chapters 03 and 04 land first as their own changes.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set:
  `ravel:activation`, `ravel:fixtures` and `ravel:result`.

## Notes

- This is the largest leaf. Edits to the book are mechanical but exact.
  If it proves bigger than one session, decompose it. The natural seam is the
  lease with its tests first, and the loop driver with its tests second.
  `LaunchEvent` and the observer then move in the first child. That child
  needs a bridge. The old loop runner still emits `keyed_launch::LaunchEvent`,
  and the migrated lease expects the new type. So the first child converts the
  observer at its call site. It keeps the `keyed-launch` dependency of
  `grove-loop`. The second child removes the last use and the dependency line.
- Keep the behaviour of the errors as it is. `interrupted-errors-k10` adds the
  rule that an error keeps a caught signal.
- Cut a `review-impl` leaf at the end of this leaf. The terminal and signal
  contracts of the lifecycle change here.

## Decomposition

The leaf proved bigger than one session. The touched book chapters hold about
950 hard findings of the prose checker. Three test files spawn `/bin/sh`
through the old runner. Chapter 17 has more than 5,000 lines. The cut is the
seam that the notes above name.

- `01` `lease-job-k19` (impl): the driver lease and its tests move to
  `LaunchEvent` of `ravel_job`, with a bridge at the call site of the loop driver.
- `02` `driver-job-k20` (impl): the loop driver moves to the foreground shape. The
  bridge, `Group`, `Escalation` and the dependency on `keyed-launch` go.
- `03` `interpretation-prose-k21` (impl): chapters 03 and 04 of `overview`. It cuts
  the `review-impl` leaf at the end.
