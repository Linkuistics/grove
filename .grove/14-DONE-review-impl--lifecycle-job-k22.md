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

- **Scope and evidence.** Review the committed change from `0c920099` through
  `657f1ffe`, including the three producer children. This is an inspection-only
  review. No test, build, lint or format command runs here. The producer logs
  report their checks; this review does not reproduce those runs.
- **Graph limitation.** The graph CLI refused `list_projects`: a pre-coordination
  or unverified generation is active. No project, generation or coverage result
  was available. Use exact source and committed diffs for this bounded review.
- **Error ownership.** The loss of caught signals on error is real and is already
  assigned to `interrupted-errors-k10`. Record the concrete loss paths here so
  its implementation can preserve typed causes as well as diagnostic text.
- **Book evidence.** Exact source fragments do not establish the surrounding
  prose. The changed chapters still contain false source counts, obsolete
  descriptions of the scrub list, and stale source coordinates. Record them as
  findings rather than repairing the book in this review.
- **Handoff.** Cut `lifecycle-job-k23` immediately before the next live sibling,
  `interrupted-errors-k10`. Its charter names this review without copying the
  findings, and leaves triage and every fix to the integration. This review
  changes no production or test code and no durable design decision.

## Findings

Coordinates below name the source at `657f1ffe`. F1 is the known follow-up of
`interrupted-errors-k10`, rather than a new regression assigned to the producer.
The integration must triage the findings before applying any fixes.

### F1 — P1: errors stop the loop but lose the signal that must end the driver

`crates/grove/src/cli.rs:55` reports an error and returns exit 1. Its lifecycle
call at line 100 uses `?`, so only an `Ok(LoopOutcome::Interrupted)` reaches
`reraise` at line 105. The caller contract at
`docs/specs/harness-selection-and-execution.md:298` instead requires an error
report followed by death from the caught signal.

There are three distinct loss paths:

- A failed spawn or wait carries the signal in `JobError::interrupted()`
  (`ravel_job.rs:183`). `launch_session` retains that error as a cause, but
  propagation through `drive` and the CLI never uses the signal.
- A successful reap can carry `JobEnd::interrupted`. If epoch invalidation
  fails, `complete_post_reap_epoch_handoff` discards the whole end value in its
  `(Ok(_), Err(...))` arm (`loop_driver.rs:354`). The signal is already drained
  from the latch, so a final latch check alone cannot recover it.
- If launch and invalidation both fail, line 355 keeps the launch error only
  as formatted text in the invalidation error's context. Its typed signal is
  lost. Separately, a signal that arrives during reset, invalidation, selection
  or another fallible step can remain in the latch while `?` returns before
  the checks at lines 113 and 180 can consume it.

`interrupted-errors-k10` must account for `loop_driver::run` at line 88, the
handoff at line 343, the error boundary in `lib.rs:387`, and CLI reporting and
outcome handling at `cli.rs:54` and line 100. Its fallible-path audit must include
lease revalidation (121 and 144), transition and selection (125–128), ending
command construction (132–133), launch-directory and capture allocation
(146–147), stdout duplication (155), epoch activation (322), launch (334) and
the handoff return (170). These errors already stop; the missing behaviour is
preserving and re-raising the signal while retaining the error report.

### F2 — P2: the lease-test helper does not ensure that it has no terminal

`crates/grove-loop/src/driver_lease.rs:965` runs the test shell directly through
`Shape::Foreground`. Its comment promises tests without a terminal, but the
helper never detaches the test process. The self-exec isolation helper at line
1520 uses `Command::output()` without `setsid`; capturing streams does not
remove the controlling terminal. Some callers, such as
`paired_witness_failure_preserves_real_launch_and_admission` at line 1111, do
not use even that self-exec helper.

When these tests are invoked from a foreground terminal, `Terminal::open`
(`ravel_job.rs:575`) can open the developer's `/dev/tty`. `hold_before_spawn`
at line 674 can then lend that terminal to `/bin/sh`. This violates the
producer's explicit requirement that the shell fixtures run without a terminal
and check notices only. Parallel tests can also compete over the same terminal.
Run the notice fixtures in a process that has no controlling terminal; changing
stdin alone does not meet the gate used by this implementation. This finding
comes from source inspection, without a live terminal experiment.

### F3 — P2: the book still describes removed control and interpretation rules

The following source claims do not hold. Several predate the producer, but the
review charter explicitly includes the old claims and the changed book's prose.

| Claim location | Unsupported claim and current source |
|---|---|
| `grove-loop/19-the-loop.md:1762` | Says the version probe and terminal reset will move later, and both callers retain scrub lists. Both already call `ravel_job::scrub`: `loop_driver.rs:387` and `crates/grove/src/dispatch/ravel.rs`. The earlier paragraph of this same chapter correctly describes the shared list. |
| `grove-loop/19-the-loop.md:1182` | Says teardown is interpreted before Ravel's returned result. `interpret_launch` reads the successful supervisor's result at `loop_driver.rs:207` and accepts exact force-stop before teardown at line 215. The summary needs the force-stop precedence that the detailed section already gives. |
| `grove-loop/20-what-could-not-move.md:70`, `:121`, `:236` | Attributes an environment name and cross-binary agreement to `CHANNEL_VAR`. No such symbol exists in `crates/grove-loop/src`; the active list is `ravel_job::CONTROL_ENV`, and lifecycle authority is `GROVE_LAUNCH_DIR`. The table's nine-test assertion therefore has no present subject. |
| `grove-loop/20-what-could-not-move.md:365` | Attributes escalation and a complementary loop scrub list to chapter 19. The producer removed the escalation section and the loop's list. The shared module forwards cancellation to Ravel and owns the one list. |

Book paths in this table are relative to `docs/walkthroughs/`. These are prose
defects; the exact fragment checker cannot establish their truth.

### F4 — P3: source counts, exclusivity claims and navigation still disagree

| Claim location | Current evidence |
|---|---|
| `grove-loop/01-orientation.md:18`, `:37`, `:436`; chapter 20 line 52 | The book still says seven dependencies. `crates/grove-loop/Cargo.toml:32` declares five ordinary dependencies and one dev dependency after removal of `keyed-launch`. |
| `grove-loop/01-orientation.md:123` | Says the manifest comment no longer names `keyed-launch` at all. `Cargo.toml:21` still names it in the comparison of error conventions. The dependency is gone; the name is not. |
| `grove-loop/01-orientation.md:244` | Says `lib.rs` has 418 lines and 327 after its documentation. The quoted root and current source have 420 lines, with documentation ending at line 49. |
| `grove-loop/16-the-lease.md:255` | Says five of six imported OS traits/types occur nowhere else in the crate, except `AsRawFd`. `ravel_job.rs:17` also imports `RawFd`, and `session_result.rs:5` also imports `MetadataExt`. The `task_tree.rs` import cited as line 42 is at 43. |
| `grove-loop/16-the-lease.md:696` | Acquisition steps cite control-directory lookup at 155, root pinning at 157–168, lease acquisition at 171, nonce at 172, revalidation at 184 and initial epoch at 186. Those operations are at `driver_lease.rs:148`, 150–161, 164, 165, 178 and 180 respectively. The newly corrected cleanup citation at 182–189 does not repair the earlier steps. |
| `grove-loop/16-the-lease.md:1029` | Cites loop revalidations at 239 and 281. The calls are at `loop_driver.rs:120` and 143. |
| `grove-loop/15-the-verbs.md:810` | Cites `transition_to_current` at `loop_driver.rs:243`. The call is at 125. The review prompt's suggested replacement, 120, is stale too: line 120 is lease revalidation. |
| `grove-loop/18-the-core.md:455` | Says `loop_driver.rs` calls `label()` four times, including argv construction. It calls it three times, at 240, 316 and 337, all for diagnostics. Public argv construction is in `session_command::lifecycle`. |

These examples also confirm the producer's logged limitation: prose coordinates
were not kept in step with the recut fragments. The integration should reconcile
the affected prose and its summary tables, rather than repairing only the cited
fragment metadata. Paths in the first column are relative to
`docs/walkthroughs/`.

### F5 — P3: the result-reader explanation gives the wrong byte bound

`docs/walkthroughs/grove-loop/19-the-loop.md:1353` says the reader takes at most
seventeen bytes and derives that bound from `grove-force-stop` plus one byte.
The literal is fifteen bytes. `session_result.rs:84` computes sixteen bytes.
The implementation is correct; the changed explanation is not. State the
formula or its correct value so the walkthrough describes the actual bound.

## Claims checked

| Review doubt | Inspection result |
|---|---|
| Foreground handover and recovery | Holds in source. The three reclaim rules are in `ravel_job::Lease::reclaim` at line 718, rather than in `reset_terminal`. Ordinary exit and failed wait reclaim only from the Ravel group. Signal death uses the broader rule with the own-group and session-leader exclusions. Saved modes return only when Grove holds the foreground. `reset_terminal` separately guards tty stdin and foreground ownership and scrubs `stty`. Failed exec after handover remains the explicitly accepted residue. |
| Ctrl-C and the latch | Holds at the successful-job boundaries inspected. `ignore_interrupts` ignores driver INT; the child resets seven dispositions. `watch` forwards only to Ravel's PID. `freeze` drains the latch even after a signal was sent. The loop checks between sessions and again after reset, invalidation and directory removal, so a signal caught there is not hidden by the frozen `JobEnd`. Error paths are F1. |
| Post-reap order and authority | Holds. Reset precedes invalidation; the handoff's reading closure cannot run after failed invalidation. `interpret_launch` checks interruption, reads a successful supervisor's result once, accepts force-stop, then teardown, then continuation. A failed status does not read stdout and cannot acquire force-stop or continuation authority. It can still finish on explicit teardown, as required. Removing `Group` removed only the rejected survivor guard. |
| Lease notices and cleanup | Holds, subject to F2's test isolation. `Started` follows successful spawn at `ravel_job.rs:282`. `Reaped` follows successful `try_wait` and freeze at 331, before terminal recovery. Spawn failure emits neither; wait failure emits no Reaped. `supervise_launch` at `driver_lease.rs:324` releases unstarted or reaped witnesses and retains a started, unreaped witness. Abandoned launch directories are still cleaned after lease acquisition and initial invalidation at 185; no old channel cleanup remains there. |
| Changed overview passages | The new end-value description in chapter 04 matches `JobEnd`. Chapter 03 gives the required interpretation precedence; its statement about unsuccessful status follows the earlier teardown rule. Neither passage relies on the removed group field. |

## Test evidence and limits

The producer logs report `bash scripts/check.sh` and the installed-Ravel tasks
passing with Ravel 1.1.0. I inspected those records and the assertions, without
running verification commands in this review.

The deterministic interpretation and lease-notice cases alone can pass with
broken OS job control. The full default workspace suite is different:
`scripts/check.sh:151` runs ordinary workspace tests, including the lifecycle
PTY tests, with installed Ravel selected at line 46. Those tests are not ignored.
Thus the default repository check does exercise the installed-Ravel composition;
it is not confined to synthetic `JobEnd` values. The separate shared-function
PTY case is ignored by default and is reached by `task ravel:job`, which
`ravel:activation` includes.

| Break | Installed-Ravel case that checks it |
|---|---|
| Missing foreground handover or wrong child signal defaults | `the_harness_is_the_foreground_job_grove_launched` (`crates/grove/tests/loop_driver.rs:2605`) checks the harness's foreground group and entry signals. `an_interrupt_typed_at_the_terminal_ends_the_job_during_selection_and_during_execution` at 2804 types Ctrl-C at both stages and checks that the driver itself was not interrupted. |
| Missing ordinary reclaim | `the_foreground_shape_hands_the_harness_the_terminal_and_takes_it_back` (`crates/grove-loop/tests/ravel_job_terminal.rs:184`) checks the foreground before, during and after the job. |
| Missing broad reclaim after signal death | `dispatch_death_recovers_the_terminal_while_its_raw_harness_survives` (`loop_driver.rs:3310` in Grove's test directory) checks foreground and saved modes while the orphan is still alive. |
| Missing signal forwarding, cleanup wait or saved-mode restoration | `a_driver_term_cancels_dispatch_and_restores_the_terminal_after_reaping_the_harness` at 3367 checks TERM, gone descendants, recovered modes and the driver's signal wait status. |
| A post-reap signal hidden by teardown or force-stop | `a_post_reap_interrupt_overrides_teardown_and_force_stop` at 3060 pauses `stty`, sends TERM after reap, and requires signal death for both supported result values. |
| A driver that steals a terminal it never held | `a_driver_started_in_the_background_takes_no_foreground` at 3403 checks the shell's foreground and modes. |

No inspected case injects a failed wait to check that narrow reclaim branch at
runtime. Its source matches the contract; no runtime result is claimed here.
The error-handoff unit cases currently check diagnostic text and suppression of
interpretation, not retention of the interruption. They do not close F1.
