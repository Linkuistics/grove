# Dispatch supervises the harness

`harness-dispatch run` spawns the harness it selected as its own child and
stays its parent until it has reaped it, for every caller. It makes the harness
a foreground job of its own, watches a fresh per-run exit channel, applies the
kill escalation, confines the harness when asked, appends its own observation
of how the run ended, and reports that **run ending** to its caller. A session
ends its run with `harness-dispatch exit`, which carries nothing. Grove keeps
what is Grove's: it selects the leaf, composes the prompt, gives the session a
launch directory, records a teardown itself, and decides from the run ending
and that record whether the loop goes on. The
[area specification](../specs/harness-selection-and-execution.md#execution-contract)
owns the contracts; [policy evaluation precedes the launch](policy-evaluation-precedes-the-launch.md)
keeps the worker that selects apart from the process that supervises.

The trade-off settled is **who owns a harness run's end**. Under the `exec`
handoff the end was split: Grove's runner watched a completion channel and
escalated, while dispatch had replaced itself and could say nothing after the
handoff, so every run stayed an attempt of unknown execution. Supervision puts
the whole end in one process, for any caller, at the price of a process that
lives as long as the harness and of nested job control.

## What it buys

- **One owner for the end.** Dispatch knows the harness started and how it
  ended, so a run's record confirms execution and carries its ending, exit and
  duration, in the existing observation format with dispatch as its source. A
  caller other than Grove gets the same supervision without writing a runner.
- **The exit signal is pure.** Its appearance is the whole of it. What the loop
  does next is Grove's to decide, from a **teardown record** Grove writes in its
  own launch directory, which a finish session records before its final exit
  signal, so teardown stays a deliberate last act.
- **One launch path for a confined invocation.** `grove run` launches through
  `run --confine`: selection runs outside the sandbox with the owner's grants and
  bounds, the harness inside it with a minimal environment, and the invocation
  gets a run record and its harness a run ID. The scrub that keeps the confined
  harness's environment small moves behind selection, which is why it could not
  move while the confined launch was Grove's.
- **No window of unknown execution under signals.** Dispatch keeps its handlers
  across the spawn, so a signal arriving after the linearization point is a
  cancellation dispatch handles and records, where under `exec` it could end
  the process with the attempt's execution unknown.

## What it costs

- **A process for the harness's lifetime, and two layers of job control.**
  Grove's runner makes dispatch a job, and dispatch makes the harness one, as a
  shell does; each hands the terminal down and takes it back with its modes
  restored ([the launched child is a job](the-launched-child-is-a-job.md)).
- **Dispatch's death orphans the harness.** Only dispatch knows the harness's
  process group. Dispatch survives every catchable signal, so only an uncatchable
  end — SIGKILL, an abort — leaves the harness running, its run as it stood.
  The caller sees no ending, and Grove stops rather than relaunching onto a tree
  a surviving harness may still be changing. Grove's runner takes the terminal
  back from the orphan's group, which can then no longer read it
  ([the launched child is a job](the-launched-child-is-a-job.md)).
- **The harness no longer has the caller's child's PID, nor its pending
  signals.** A spawned child starts with none pending, so a signal the caller
  blocked and sent to dispatch stays pending in dispatch, where the `exec`'d
  harness kept it; the entry mask and dispositions still pass on. No current
  caller needs either. Forwarding pending signals was declined: it needs a relay
  polling a mask the harness may change, and loses the signal's information and
  real-time queueing.
- **The caller names where a sandboxed harness can write.** The exit channel
  must be writable from inside the harness's own sandbox, which dispatch cannot
  know. A caller that knows names the directory with `--exit-dir`, and Grove
  names its launch directory, under the workspace's `.jj/grove/`, where a
  session's writes are already proved to land. Grove's launch therefore carries
  two run-mechanics flags, `--exit-dir` and `--ending-file`, beside the kind,
  task file, task identity and prompt. Neither reaches the policy, whose worker
  must not learn a channel that carries the authority to end the run, and
  neither is a selection parameter.
- **Grove reads one thing back.** The ending file is the first output of
  dispatch's that Grove reads. It reads no run record, provider or command.
- **A finish ends with two commands**, `grove-llm record-teardown` and then
  `harness-dispatch exit`. A finish session that sends only the second relaunches
  the loop onto a torn-down grove, the residue the loop's own record already
  states for a `finish` session whose skill is unread
  ([decision 9](../specs/module-decomposition.md)).

The runner stays `keyed-launch`, a crate with no Grove, task-tree or jj
dependency. Dispatch uses it for the harness's job, exit channel, escalation and
confinement; Grove uses it to run dispatch. An extraction takes it with
dispatch, and Grove then depends on it from there. The escalation's graces are
dispatch's own constants, the values the driver held before: they are about how
long the exit verb's call takes to return and how long a harness takes to shut
down, which dispatch, the owner of both, knows better than any caller.

## Considered options

- **Keep the `exec` handoff and Grove's runner.** Rejected by the owner's
  direction: dispatch alone catches the harness's exit, so that the end of a run
  has one owner and any caller gets it. Reopen only if a supervisor in front of
  every harness is shown to cost a harness something it cannot do without.
- **Supervise only when the caller asks.** Rejected: two launch paths, and the
  default would keep the split this record removes.
- **A separate verb that keeps the `exec` handoff beside `run`.** The owner
  permits one; it is not built, because nothing calls it and Grove never would,
  and it would keep a second launch path and its linearization alive for no
  caller. It would catch nothing by construction. Reopen when a caller needs the
  harness to keep dispatch's PID.
- **Leave the channel and escalation in Grove's runner and have dispatch only
  spawn.** Rejected: it keeps the end split across a command boundary, and every
  other caller would need a runner of its own.
- **Carry Grove's disposition in the exit signal, opaque to dispatch.** Rejected
  by the owner in favour of Grove recording teardown itself, so the signal is the
  same for every caller and means only that the run is over.
- **Infer a teardown from `.grove/` being gone.** Rejected, as
  [one live driver owns each working tree](one-live-driver-per-working-tree.md)
  rejects it: absence carries no disposition and attests no confirmation.
- **Report the ending in dispatch's exit status.** Rejected: a harness that
  quits with 0 and one ended through the exit signal would read the same, and
  telling those two apart is the reason the report exists. The exit status still
  reproduces the harness's own when it exits on its own.
- **An inherited descriptor for the exit channel or for the report.** Rejected
  for the channel because a harness's own tool runner need not pass a descriptor
  through to the command that signals, and for the report because a file the
  caller reads after the reap needs no new runner feature.
- **Let the policy choose the exit channel's directory, or derive one from the
  cwd.** Rejected: every owner's policy would carry launch mechanics, and one
  that forgot would stall its sessions; a directory in the working copy is
  snapshotted by jj, and `.jj/` would put jj knowledge in dispatch.
- **Signal dispatch's PID instead of writing a file.** Rejected: a sandbox
  commonly denies a signal that leaves it, which is why the escalation was the
  parent's job in the first place.
- **Let the owner, the caller or the policy set the graces.** Rejected until an
  owner needs a different value: an owner setting is one more key read before
  every launch, a `select` result field would be a contract change for a value
  needed after the policy is gone, and a caller does not know the harness's
  shutdown. Reopen with a harness that cannot shut down in the kill-grace.
- **Move the runner into dispatch.** Rejected: Grove still runs dispatch as a
  job, so the job code would exist twice.
- **Have the harness die with dispatch through a parent-death signal.** Rejected:
  Linux alone has one, and it reaches the direct child and none of its group.
