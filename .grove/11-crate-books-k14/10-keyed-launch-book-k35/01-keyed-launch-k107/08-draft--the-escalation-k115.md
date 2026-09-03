# the-escalation-k115

## Goal

Draft chapter 8 of the `keyed-launch` book — *The watch and the escalation*,
`08-the-escalation.md`, slice `the-launchers-job` — owning `src/run.rs` lines
124–243 (120) and 449–607 (159): 279 lines.

## Context

- Draft stage, child 8 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *8 · The watch and the escalation — the launcher's job*.
- `Watch` as the supervisor's state machine; `watch`'s three observables and **the
  honest statement that they are the only three ways a launch ends** — a child that
  finishes its work and never signals reaches none of them, and the launch stalls
  rather than ends. The source names it as a real failure mode with no cheap fix
  and **the book must not soften it**; chapter 10 closes on it.
- Why ending an interactive child is the *launcher's* job — it is the child's
  parent, outside whatever sandbox the child runs under, and a child asked to end
  itself may be denied silently; the escalation addressed to `-pgid` as well as
  the pid, and what that reaps; `supervise` taking the terminal back only from the
  job this launch owned, and why returning while the terminal belonged to a dead
  group would be a SIGTTOU stop rather than an error anybody could read; `kill`'s
  deliberately ignored failure as *the shell's `kill … 2>/dev/null`, written down*.
- Then the launcher's own signals — `INTERRUPTED_BY` as process-global because a
  disposition is, latched because the launch on which the child finally exits still
  has to report it, carrying the *number* because a launcher that re-raises SIGTERM
  for a SIGHUP has told its parent the wrong thing, and cleared immediately before
  each spawn because **a latch that outlives its launch is a loaded gun**;
  `take_interrupt` for the signal that arrives between launches; `reraise` and why
  an exit code cannot express *was signalled* at all; and SIGINT's deliberate
  absence from the handler.
- **Prose obligation 3 is *do not restate*.** Same instruction as chapter 7, same
  reason. Connect and name the test.
- This chapter completes the early-use row first used at chapter 7 — move its
  status to `explained` — and produces the `End::Interrupted` chapter 7 deferred.
- Required example anchor: `the-two-graces` — the token appearing through grace →
  SIGTERM → kill-grace → SIGKILL, and the launcher's own SIGTERM as
  `End::Interrupted`.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  the-launchers-job --check all` is valid: 1,836 resolved lines, 237 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

This chapter closes `src/run.rs`: after it, all four blocks of that root are
resolved and only `src/channel.rs` 272–404 and `src/conformance.rs` remain.
