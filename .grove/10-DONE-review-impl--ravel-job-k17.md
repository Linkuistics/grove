# ravel-job-k17

**Reviews:** ravel-job-k7

## Goal

Read the shared function of `ravel-job-k7` adversarially, and report findings
without fixes. Two callers will rely on it, and each of them relies on its
signal and terminal contracts.

## Context

- The commit of `ravel-job-k7`, and the running log of decisions in that leaf.
- The module `crates/grove-loop/src/ravel_job.rs` and its tests in
  `crates/grove-loop/tests/ravel_job.rs`, `session_command.rs` and
  `session_result_installed.rs`.
- The caller contract `docs/specs/harness-selection-and-execution.md`, in its
  sections *Jobs and authority* with their subsections, and the decision
  `docs/adr/the-launched-child-is-a-job.md`.
- The runner that the module replaces, `crates/keyed-launch/src/run.rs`. The
  terminal rules must match it. The group and escalation parts must be absent.
- Chapter 9 of the book of Ravel at v1.1.0, for the guarantees that the function
  relies on.

## Done when

- Each claim under *Notes* has a finding, or a statement that it holds.
- Each finding names its file and line and states how the code fails the
  contract.
- The review checks the book section of `ravel_job.rs` against the source it
  quotes, and reports any claim that the source does not support.

## Notes

Read these claims hardest. Each is narrow, and no compiler checks it.

- **The function never clears the latch before the spawn.** A signal that arrives
  between sessions goes to the new `ravel` at the first poll. Check that no path
  spends a signal on a session that it does not concern. Check that every error
  path takes the latch into the error.
- **One rule for INT in both shapes.** The first call installs the handler unless
  the disposition is the ignore. Check a driver under `nohup`, a driver that
  ignores SIGINT, and `grove run` from inside a harness.
- **`try_wait` replaces `waitid` with `WNOWAIT`.** The old code needed the
  unreaped child to reserve the group ID. Check that no path now signals a
  recycled ID. Check that a stopped `ravel` is never an exit, on macOS and on
  Linux.
- **The three reclaim rules.** They moved unchanged. Check the failed wait, the
  death by signal and the ordinary exit against the lease and its saved modes.
- **The pre-exec closure.** It must stay async-signal-safe. Check every call in
  it. Check the order of `setsid`, the descriptor close and the handover.
- **An ignored SIGCHLD.** The restore runs once, at the first call. Check that the
  child still gets the ignore back. Check that a second call reads the saved
  state and not its own restore.
- **The tests.** No test uses a stand-in child to check job control. Check
  whether the default suite can pass while the foreground handover is broken.
  Say which installed-Ravel case would catch it.

## Decisions (running log)

- **Boundary finding.** Both uses of `or_else(take_interrupt)` skip the latch
  when an earlier interruption exists. A second signal can then escape the job
  boundary. This contradicts the caller contract at reap and failure.
- **Test finding.** The installed-Ravel cases exercise both shapes but assert
  no terminal ownership. The existing terminal assertions still reach
  `keyed-launch`. This leaves the new handover without an acceptance assertion
  before either caller migrates.
- **Inspection boundary.** This review runs no tests, builds, linters or
  formatters. The producer's passing checks remain recorded evidence from the
  producer, rather than measurements from this review.
- **Handoff.** `ravel-job-k18` will triage this review before either caller
  migrates. Its task names this review and does not restate the findings.

## Findings

Reviewed producer commit `7caefade0daee8c9fb9849a525de305bc93eb6c8`.
Both findings have severity P2: they concern the contract or its acceptance
before production callers adopt the function.

### F1 — P2: An earlier interruption prevents the final latch drain

**Coordinates:** `crates/grove-loop/src/ravel_job.rs:330` and `:186`.

When `interrupted` already contains a signal, `or_else(take_interrupt)` does
not call `take_interrupt`. The error constructor has the same behaviour when
`failure.sent` contains a signal. Thus, neither boundary always takes the
remaining latch into the job's report, as the caller contract requires.

Consider TERM consumed and forwarded at one poll. At a later poll, HUP can
arrive after the latch check but before `try_wait` returns an exit. The report
contains TERM, but HUP remains in the latch. The caller can then mistake this
pre-reap HUP for a later signal. The existing caller checks its late latch
before interpreting the launch at `crates/grove-loop/src/loop_driver.rs:247`.
Retaining that order during migration would select HUP instead of the job's
first interruption, TERM. A caller that inspects only `JobEnd` can instead
leave HUP for a subsequent launch.

The failed-wait path has the same defect. If TERM was already forwarded,
the final error constructor skips a pending HUP. Early errors with no
forwarded signal do drain the latch correctly.

**Contract:** `docs/specs/harness-selection-and-execution.md:173` assigns the
first interruption to the job and drains the latch at reap. Its error clause
at `:177` requires the corresponding drain at failure. The implementation
must retain the earlier interruption while consuming signals at these
boundaries. Signals received after the boundary must remain available to the
caller.

The book also overstates this behaviour at
`docs/walkthroughs/grove-loop/19-the-loop.md:2080` and `:2258`.
Its source quotations faithfully reproduce the defect.

### F2 — P2: The installed cases do not assert the new handover

**Coordinates:** `crates/grove-loop/tests/session_command.rs:129` and
`crates/grove-loop/tests/session_result_installed.rs:206`.

These cases call the new function with real Ravel, but they create no
pseudo-terminal and make no assertion about foreground ownership. Their
managed shells print output, inspect files and publish a result. None requires
terminal input or observes the foreground process group.

A headless run of every named task can therefore pass with the new handover
disabled. Merely running the lifecycle case under a pseudo-terminal also
adds no assertion that distinguishes a successful handover from no handover.
This matters because terminal ownership determines which process receives a
typed Ctrl-C and whether the harness can read input.

The existing installed-Ravel case
`the_harness_is_the_foreground_job_grove_launched`, at
`crates/grove/tests/loop_driver.rs:2606`, does assert foreground ownership.
However, it reaches `keyed_launch::run_observed_with_stdout` through
`crates/grove-loop/src/loop_driver.rs:407` in this commit. It cannot detect a
defect confined to the new function. After lifecycle migration, that case
would detect a missing handover. The terminal interrupt case at
`crates/grove/tests/loop_driver.rs:2805` would also exercise its consequence.

The review mandate asks for an installed case that catches this defect.
There is no such assertion through the new function at this boundary.
Acceptance must use real Ravel and observe the foreground group through the
new function before caller migration. This requires no stand-in job-control
test and adds no new process seam.

The book's evidence section at
`docs/walkthroughs/grove-loop/19-the-loop.md:2768` distinguishes the later
loop-driver acceptance. Its recorded pseudo-terminal run establishes no
assertion about ownership. The stronger claim in
`crates/grove-loop/tests/ravel_job.rs:4` is unsupported by the named cases.

## Disposition of the required claims

- **No clear before spawn:** holds. `ENTRY` installs handlers before all
  fallible preparation, and no path clears the latch before spawn. The first
  poll forwards a signal left between sessions. F1 qualifies the final drains.
- **One INT rule:** holds by inspection. `enter` preserves ignored INT, TERM
  and HUP, including a HUP ignored under `nohup`. A lifecycle driver that
  ignores INT keeps that ignore. A standalone caller with default INT receives
  the latch handler. The child restores all seven required defaults in both
  shapes, so the driver's ignored INT does not reach Ravel or the harness.
  A nested caller's explicitly ignored signal remains ignored as required.
- **Wait and identity:** holds for this function's ownership of its child.
  The sole numeric signal operation precedes `try_wait`. Success and error
  return without another signal attempt. The function sends no group signal
  and performs no group probe. Rust documents `try_wait` as a nonblocking
  exit check that reaps on Unix. An ordinary stopped child supplies no exit
  status on macOS or Linux. This review did not measure either platform.
- **Three reclaim rules:** hold by comparison with the old runner.
  `Lease::reclaim` retains the foreground-held gate, the narrow ordinary-exit
  and unknown-status rules, and the wider signal-death rule. Saved modes
  return only when Grove's group holds foreground. The function also retains
  the accepted failed-exec residue described by the caller contract.
- **Pre-exec closure:** holds by inspection. Captured values require no
  allocation or lock. Its system calls are `fcntl`, `setsid`, `signal`,
  `getpid` and `tcsetpgrp`. Error construction reads errno without formatting.
  Detached descriptors receive close-on-exec flags before `setsid`.
  The child retains Rust's spawn-error pipe until exec. Foreground grouping
  precedes the callback, and the handover precedes the seven disposition
  resets. The detached shape performs no terminal handover.
- **Ignored SIGCHLD:** holds. One `OnceLock<bool>` stores the inherited ignore
  before restoring the parent's default disposition. Every later launch reads
  that saved value. The pre-exec closure restores the child's ignore from it.
  The parent therefore does not mistake its own repair for inherited state.
- **Tests:** the new nonterminal cases do not use a stand-in to assert job
  control. The default suite can pass while the new handover is broken.
  F2 identifies the installed acceptance gap and the existing cases that
  become relevant after migration.

## Evidence and limits

The graph CLI refused `list_projects` because an incompatible server generation
was active. No project, generation or coverage result was available.
This review used direct source reads, targeted text searches and the
producer's committed diff. It makes no graph-completeness claim.

The review read Ravel's chapter 9 from the local repository at tag `v1.1.0`.
The public fetch failed. That chapter places managed-group cleanup inside
Ravel and excludes abrupt supervisor death from successful completion.
These boundaries match Grove's chosen reliance on Ravel.

The source quotations for `ravel_job.rs` in chapter 19 were compared as text.
Every quoted block matched its declared source range. Together, the blocks
cover the complete source file. This comparison ran no book checker.
F1 names the prose claims that the quoted source does not support.

The wait assessment also uses the official
[Rust documentation for `Child::try_wait`](https://doc.rust-lang.org/std/process/struct.Child.html#method.try_wait).
The producer recorded passing repository checks and installed-Ravel tasks in
`ravel-job-k7`. This review did not rerun them or reproduce timing-dependent
signal delivery. Its findings derive from source control flow and assertions.
