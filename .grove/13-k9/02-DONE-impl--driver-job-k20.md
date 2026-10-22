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

- **Driver.** `drive` and `launch_session` in `loop_driver.rs` call
  `ravel_job::launch` with `Shape::Foreground { launch_dir }`. The bridge, the
  `keyed_launch` imports, `ESCALATION`, `LOOP_CONTROL_ENV`, `scrub_list` and
  `scrub_loop_control_env` are gone. `reset_terminal` calls `ravel_job::scrub`,
  so one list remains. The file has 773 lines, down from 918.
- **Interpretation.** `interpret_launch` takes `&JobEnd` and reads
  `ended.interrupted`. The `Group` arm is gone. The order is unchanged:
  interruption, a successful `grove-force-stop`, teardown, then
  `grove-continue`. A surviving group no longer stops the loop.
- **Tests.** The two interpretation tests build a `JobEnd` directly. The first is
  now `launch_reading_precedence_over_interrupts`. The row that expected a stop
  for a surviving group is gone, because that rule is gone.
- **Dependency.** `grove-loop` no longer names `keyed-launch`, and `Cargo.lock`
  follows. `lib.rs` has `pub use ravel_job::reraise`. `cargo fmt` moved that line
  below `prompt::{compose, Mandate}`, so `lib.rs` has 420 lines.
- **Comments.** The doc comment of `session_result.rs` names `ravel_job::launch`
  and has no survivor check. The comment at line 355 of
  `crates/grove/tests/loop_driver.rs` names `ravel_job::launch`.
- **Book.** The re-cut of fragments was a script that aligned the old and new
  source with `difflib`, with an explicit table for `loop_driver.rs`. The script
  is not in the repository. Five fragments are gone: `loop-control-env`,
  `loop-scrub-list`, `loop-scrub-helper`, `loop-drive-survivor` and
  `loop-escalation`. Chapter 19 lost the section *The cancellation bound* and
  keeps the section *The third choice* as a short pointer to `ravel_job::scrub`.
  Totals: `Cargo.toml` 66, `lib.rs` 420, `loop_driver.rs` 773, all roots 15,074.
  Slice `allowed-to-mean` is 486 and slice `four-things-a-runner-cannot-choose`
  is 1,912. Chapter 18 needed no change: its claims concern `prompt.rs`. The
  concept index lost five entries whose anchors or subjects are gone.
- **Conversion.** One earlier change converted the findings in the touched
  paragraphs of chapters 01 and 19. I did not convert paragraphs that this leaf
  then deleted. Chapter 20 has one touched row, and its findings stay.
- **Finding.** Chapter 20 still names `CHANNEL_VAR` (lines 70, 121 and 236), which
  no source defines. Chapter 15 cites `loop_driver.rs:243` for
  `transition_to_current`, and the call is at line 120. Chapter 18 says
  `label()` is reached four times in `loop_driver.rs`, and it is reached three
  times. These claims predate this leaf. `book-specs-k14` or the review may
  correct them.
- **Result.** `bash scripts/check.sh` passes all 20 checks. `task
  ravel:activation`, `task ravel:fixtures` and `task ravel:result` pass with the
  installed Ravel 1.1.0 in `RAVEL`. The pseudo-terminal cases of
  `crates/grove/tests/loop_driver.rs` pass.
