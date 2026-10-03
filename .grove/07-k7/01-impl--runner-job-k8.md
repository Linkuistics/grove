# runner-job-k8

## Goal

Give `keyed-launch` the job contract that decision 7 states for both
supervisors, so that Grove's own launches gain it now:

- **The group ends with every launch.** The runner observes the child's exit,
  never a stop, without reaping it, whether the launch is interactive,
  noninteractive or confined and whatever ended it. While the unreaped child
  still reserves the group's ID, it sends SIGKILL to what remains of the
  group, and again after a short pause. It then reaps the child, whose status
  stays the launch's, and, querying only, confirms within 1 s that the group
  is gone. A group still present is reported to the caller beside the status.
- **The terminal comes back as it was handed over.** The runner saves the
  terminal's attributes at the handover and restores them on reclaim. After
  the reap it takes the terminal back from the child's group. When the child
  died of a signal, it takes it from whichever group then holds it, unless that
  group is the launcher's own or the session leader's. Either way it does this
  with SIGTTOU ignored. A launcher that never held the foreground during the
  launch takes nothing.
- **Entry signal state is respected.** No handler is installed over a
  disposition the launcher ignores. A launcher that inherited SIGCHLD ignored
  restores SIGCHLD's default for itself, never a handler, so the child is not
  reaped unwatched.
- **Cancellation has its modes.** INT cancels a launch with no terminal. An
  interactive or noninteractive child's group is sent the cancelling signal and,
  after the kill-grace, SIGKILL. A confined child's group is killed at once.

## Context

- Decision 7 of `docs/specs/module-decomposition.md`. The spec's
  *Supervision*: the paragraphs on taking the terminal back, *The group ends
  with the run*, and *When dispatch dies*.
- `harness-wrapper-k5`'s I1, I3 and I6 hold the measured platform facts.
  macOS `waitid(WEXITED|WNOWAIT)` also reports a stopped child. A member forked
  as the first kill lands can survive it on macOS. `kill(-pgid, 0)` answers
  EPERM for a group of zombies on macOS, so only ESRCH confirms the group gone.
  An inherited ignored SIGCHLD auto-reaps the child and frees its group's ID.
- Today only a detached launch drains its group, in `drain_group`, and an
  interactive launch reclaims the terminal only from its direct child's group.
  Both live in `crates/keyed-launch/src/run.rs`.
- Suites: `crates/keyed-launch/tests/` (`launch.rs` drives a PTY; also
  `interrupt.rs`, `noninteractive.rs` and `confinement.rs`) and
  `crates/grove/tests/loop_driver.rs`. Book: `docs/walkthroughs/keyed-launch/`,
  with `docs/specs/keyed-launch-book-structure.md`.

## Done when

- `keyed-launch`'s suite shows each of the following under a PTY where a
  terminal matters:
  - A TERM-ignoring descendant is gone before the launch returns, whether its
    child exits on the escalation's TERM, within the grace, or on its own. The
    child's own status is still the launch's.
  - A child stopped by SIGSTOP is neither reaped nor killed as though it had
    exited.
  - A raw-mode child killed by the escalation leaves the terminal back with the
    launcher, its saved modes restored.
  - A child that exits normally leaves the foreground with any other group then
    holding it.
  - After a death by signal, the launcher takes the terminal back from a group
    that is not its child's.
  - A launcher started in the background takes no foreground.
  - A launcher with SIGCHLD ignored still supervises its child to an end.
  - A launcher with HUP ignored at entry installs no HUP handler.
  - INT cancels a launch with no terminal.
- Grove's launch-boundary suite passes unchanged.
- The `keyed-launch` book and its structure spec follow the source (P2), and
  `## Unreleased` records what Grove's launches now do at their end.
- `bash scripts/check.sh` passes.

## Notes

- Not here: the channel's optionality and its token, granted variables, and the
  transparent caller's entry signal state. `dispatch-supervises` adds the
  runner options its caller needs, and the token goes at `launch-cutover`.
  `run_confined` writing to the launcher's own output is `confined-run`'s.
- The repeated SIGKILL and the 1 s confirmation are the design's measured
  answers. Do not replace them with a single kill, or with a reap before the
  kill: once the group empties, its ID can be reused (I1).
