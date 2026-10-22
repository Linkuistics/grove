# driver-job-k20

## Goal

Move the loop driver to `ravel_job::launch` in the foreground shape, and remove
the last use of `keyed-launch` from `grove-loop`. This is the second half of
`lifecycle-job-k9`.

## Context

- The brief of `lifecycle-job-k9`, the running log of `lease-job-k19` and the
  running logs of `ravel-job-k7` and `grove-run-job-k8`.
- The root brief: *Requirements*, *Design* sections *New homes* and *Tests*, and
  *Residues*. The caller contract
  `docs/specs/harness-selection-and-execution.md` has the section *Lifecycle
  interpretation*. Rule 4 goes.
- The code: `crates/grove-loop/src/loop_driver.rs`, `lib.rs` with its re-export
  of `reraise`, `session_result.rs` and `crates/grove-loop/Cargo.toml`.
- `crates/grove/tests/loop_driver.rs`: the comment at line 355 that names
  `keyed-launch`. `lease-job-k19` already rewrote the housekeeping case.
- The book of `grove-loop`: chapters 01, 18, 19 and 20, with the concept index,
  the source index and the README. Chapter 19 quotes `loop_driver.rs`.
- The checker found these hard findings before the cut.
  Chapter 01 has 96, chapter 18 has 141, chapter 19 has 33, and chapter 20 has
  129. Convert only the sections that this leaf edits.

## Done when

<!-- tc: depart synonym-rotation because Launch directory is a defined glossary term -->
- The loop driver starts `ravel` through `ravel_job::launch`. It uses the
  foreground shape with the launch directory. `GROVE_LAUNCH_DIR` goes to the
  lifecycle session only.
- `Group` and `Escalation` are gone from the interpretation. A surviving group
  no longer stops the loop. The order stays: driver interruption, then a
  successful status with `grove-force-stop`, then the teardown record, then
  `grove-continue`.
- The copy of the scrub list in `loop_driver.rs` is gone, so one list remains.
  `stty` uses the helper `ravel_job::scrub`.
- The two interpretation tests in `loop_driver.rs` are rewritten without
  `Group`. They use a result value that Grove owns.
- `crates/grove-loop/Cargo.toml` no longer depends on `keyed-launch`, and
  `Cargo.lock` follows. The conversion of the observer is gone.
- `reraise` is re-exported from `ravel_job`. The conversions of `Argv` at the
  call sites are gone.
- The doc comment at `session_result.rs` describes the held file and the shared
  function, with no survivor check. The chapter fragment that quotes it matches.
- The loop-driver cases of `crates/grove/tests/loop_driver.rs` under a
  pseudo-terminal still pass with the installed Ravel 1.1.0. These cases are the
  acceptance of the foreground job.
- The `grove-loop` book matches every changed line. The conversions of the
  touched sections land first, each as its own change.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set:
  `ravel:activation`, `ravel:fixtures` and `ravel:result`.

## Notes

- `loop_driver.rs` carries a bridge from `lease-job-k19` in `launch_session`.
  Remove it with the `keyed-launch` dependency. Chapter 19 has one sentence about
  it after the first fragment of that function.
- The book totals changed in `lease-job-k19`. Read its running log first.
- Keep the behaviour of the errors as it is. `interrupted-errors-k10` adds the
  rule that an error keeps a caught signal.
- `ravel:activation` still calls `test:keyed-launch` until
  `keyed-launch-removal-k11`.
- The prose of chapters 03 and 04 of `overview` belongs to
  `interpretation-prose-k21`, not to this leaf.

## Decisions (running log)

