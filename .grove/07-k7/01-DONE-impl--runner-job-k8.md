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

## Decisions (running log)

**R1 — measured here, macOS 26 arm64, before building on it.** After the
foreground group's last member is reaped, `tcgetpgrp` still answers that
group's ID, and `kill(-id, 0)` answers ESRCH. So "take the terminal back from
the child's group" works after the reap, as the spec orders it, with no
dead-group special case. `waitid(P_PID, WEXITED|WNOHANG|WNOWAIT)` reports a
stopped child as `CLD_STOPPED` on every poll and nothing once it is continued,
and then reports the real exit (`CLD_KILLED`) once it dies. So the runner
treats only `CLD_EXITED`, `CLD_KILLED` and `CLD_DUMPED` as an exit.

**R2 — a group still present is a field, not an error.** `Ended` gains
`group: Group` (`Gone` or `Present { pgid }`), so a caller holds the child's
status beside the survivor report, as decision 7 asks and as dispatch's end
observation will need. The old `drain_group` error, for detached launches only,
goes. Both current callers act on it: `grove run` publishes nothing, and the
loop stops with the leaf live rather than relaunching or finishing beside a
survivor.

**R3 — the shape of the end.** In every mode the runner polls for exit with
`WNOWAIT`, including after the escalation's SIGKILL (no blocking reap). On
exit it sends SIGKILL to `-pgid`, pauses 20 ms, sends it again, reaps the
child, emits `Reaped`, takes the terminal back, and then confirms within 1 s
that the group is gone. Only ESRCH confirms it.

**R4 — the terminal.** The runner saves the attributes the first time it sees
itself in the foreground during a launch, whether at the spawn or at a later
tick (`grove &`, then `fg`). That moment is also what "held the foreground"
means. A launch that never held it takes nothing and restores nothing. On
reclaim, after a death by signal it takes the terminal from any group except
its own and the session leader's (`getsid(0)`). Otherwise it takes it only from
the child's group. Either way it does so with SIGTTOU ignored. It restores the
saved attributes whenever it ends up in the foreground, also with SIGTTOU
ignored.

**R5 — entry signal state.** TERM, HUP and, for a launch with no terminal, INT
are caught only when their current disposition is not `SIG_IGN`. "No terminal"
means a detached launch, or an interactive one whose `/dev/tty` would not open.
SIGCHLD's entry disposition is latched once per process, because the runner
changes it. If it was ignored, the launcher gets `SIG_DFL` (flags cleared) and
the child is put back to `SIG_IGN` in `pre_exec`, so it still gets the
disposition it would have inherited.

**R6 — cancellation modes.** The job carries its mode. A cancelled interactive
or noninteractive launch forwards the signal and starts the kill-grace. A
confined launch is sent SIGKILL at once. This reverses `run_noninteractive`'s
immediate kill. That kill was for `grove run`, which launches confined, so
nothing it does changes. `noninteractive.rs`'s cancellation cases move to the
new mode, and a confined case is added beside them.
