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
