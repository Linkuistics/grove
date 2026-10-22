# lifecycle-job-k22

**Reviews:** lifecycle-job-k9

## Goal

Read the lifecycle move of `lifecycle-job-k9` adversarially, and report findings
without fixes. The loop driver now launches `ravel` as an ordinary foreground
child. The terminal and signal contracts of the lifecycle changed in this node,
and `interrupted-errors-k10` builds on the result.

## Context

- The producer commits of `lifecycle-job-k9`. Find them from the handle
  `lifecycle-job-k9`, and from the handles of its three children:
  `lease-job-k19`, `driver-job-k20` and `interpretation-prose-k21`. Read the
  running log of each child.
- The code: `crates/grove-loop/src/loop_driver.rs` (`drive`, `launch_session`,
  `interpret_launch`, `reset_terminal`, `complete_post_reap_epoch_handoff`),
  `crates/grove-loop/src/ravel_job.rs` (`launch`, `Shape::Foreground`, the latch,
  `scrub`, `reraise`), `driver_lease.rs` with `driver_lease/observation.rs` and
  `driver_lease/witnesses.rs`, and `crates/grove-loop/Cargo.toml`.
- The tests: `crates/grove/tests/loop_driver.rs` under a pseudo-terminal, and the
  tests of the driver lease that spawn `/bin/sh`.
- The caller contract `docs/specs/harness-selection-and-execution.md`, in its
  section *Lifecycle interpretation* and the sections on jobs and authority. The
  decision `docs/adr/the-launched-child-is-a-job.md`.
- The root brief, section *Behaviour that stays*.

## Done when

- Each claim under *Notes* has a finding, or a statement that it holds.
- Each finding names its file and line and states how the code fails the
  contract.
- Check the walkthrough prose that the node changed against the source it
  describes. These are `grove-loop` chapters 01, 16, 17 and 19, and `overview`
  chapters 03 and 04. Report each claim that the source does not support.

## Notes

The doubt: the terminal and signal contracts of the lifecycle. Read these claims
hardest. Each is narrow, and no compiler checks it.

- **The foreground handover.** After the reap, the driver must take back the
  foreground of the terminal and leave the terminal usable. Check the three
  reclaim rules in `reset_terminal` against a normal exit, a death by signal and
  a failed wait. Check that `stty` uses `ravel_job::scrub`, so the one scrub list
  covers it.
- **The latch.** A typed Ctrl-C must reach the harness and never end the driver.
  A TERM or HUP between sessions, or after the reap, must stop the loop. Check
  that no path spends a signal on a session that it does not concern. Check
  that `JobEnd::interrupted` cannot hide a signal that arrives after the reap.
- **The post-reap order.** The order is driver interruption, then a successful
  status with exactly `grove-force-stop`, then the teardown record, then
  `grove-continue`. Check that `interpret_launch` keeps it. Check that Grove
  reads the result once, and only after the epoch invalidation. Check that a failed
  status grants no authority. Check that removing `Group` lost no stop that
  another rule relied on.
- **The errors that stop the loop.** The behaviour of the errors stays as it was
  here. Check that a launch error and a failed invalidation still stop the loop.
  Say which of them loses a caught signal. `interrupted-errors-k10` fixes
  that loss. List each place that leaf must change.
<!-- tc: depart synonym-rotation because Started is the name of a lease notice -->
- **The lease notices.** The driver lease receives Started after a successful
  spawn and Reaped after a successful wait. Check the order of the notices, the
  abandoned-directory cleanup that stays, and the tests that now run without a
  terminal.
- **The tests.** The pseudo-terminal cases are the acceptance of the foreground
  job. Check whether the default suite can pass while the foreground handover or
  the latch is broken. Say which installed-Ravel case would catch each break.
- **Book claims known to be stale.** `driver-job-k20` logged three that predate
  it. Chapter 20 names `CHANNEL_VAR`, which no source defines. Chapter 15 cites
  `loop_driver.rs:243` for `transition_to_current`, and the call is at line 120.
  Chapter 18 says that `loop_driver.rs` calls `label()` four times, and the code
  calls it three times. Chapter 20 also lists "the watch and the escalation" as
  chapter 19 content, and the cancellation bound section is gone. Check each,
  and report any other claim of the same kind.

## Decisions (running log)
