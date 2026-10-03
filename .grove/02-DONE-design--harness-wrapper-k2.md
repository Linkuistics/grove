# harness-wrapper-k2


## Goal

Establish how `harness-dispatch` becomes the process wrapper the root brief's
settled requirements describe, and deliver it as the reworked area
specification and ADR set: answer every question the brief leaves to design,
keep every guarantee it lists, and carry the four agreed seams into the
specification.

## Context

- `plan-k1`'s running log (`D1`–`D9`, in its retired leaf beside this one)
  holds each requirement in the owner's own words, with the options declined.
  Synthesise from it; do not re-interview (`SPEC-FORMAT.md`).
- Today's launch, end to end: the loop builds the `harness-dispatch run` argv
  and drives the epoch around the spawn in `crates/grove-loop/src/loop_driver.rs`;
  `crates/keyed-launch` owns the job, terminal handover, channel, escalation
  and confinement (`run.rs`, `channel.rs`, `confinement.rs`); dispatch's
  front commits the handoff and execs in `crates/harness-dispatch/src/run.rs`,
  with `cancellation.rs` holding the linearization point;
  `crates/grove/src/standalone.rs` runs `inspect`, confines through the
  runner and publishes on the `done` token; `grove-llm complete` writes the
  token. The session witness rides the runner's `Started`/`Reaped` events.
- What reads the parameters D1/D2 remove: the sample policy
  (`crates/harness-dispatch/worker/sample/policy.ts`) places `repo` in
  `--add-dir` and `session_name` in `claude -n`, and refuses without them; the
  typecheck fixtures under `crates/harness-dispatch/worker/typecheck/` use
  `repo` too.
- The seams' current suites: `crates/harness-dispatch/tests/` (`run.rs` drives
  a PTY), `crates/keyed-launch/tests/`, `crates/grove/tests/loop_driver.rs`
  (PTY) and `standalone.rs`, `crates/grove-llm/tests/complete.rs` and
  `standalone_completion.rs`.
- Beyond the brief's pointers: `docs/ARCHITECTURE.md` (*Process ownership*,
  and its harness-dispatch section), the area's visual document under
  `docs/design/harness-selection-and-execution/`, the methodology's signal
  contract (the prompt in `crates/grove-loop/src/prompt.rs`, the spine's
  `references/driver.md`, the conformance rows in
  `plugins/grove/conformance/rules.tsv`), and `docs/USAGE.md`.

## Done when

- `docs/specs/harness-selection-and-execution.md` states the supervised
  design as its contract, with the four agreed seams in its seams section,
  and `docs/specs/standalone-invocations.md` and decisions 7 and 9 of
  `docs/specs/module-decomposition.md` agree with it.
- The ADR set is reworked in place — no superseding record — and every
  citation of a merged, split or deleted record is reconciled.
- Each question under the brief's *Questions left to design* is answered in
  the specification or an ADR, and each listed guarantee is either kept or
  explicitly put back to the owner.
- The area's visual document matches the reworked specification.
- A `planning` leaf exists for a fresh session, after any review chain this
  session judges the reworked specification needs.

## Notes

- If a review is warranted, cut `review-design` with this stem first and the
  `planning` leaf after it, in this session: a review that finds something
  then `leaf-insert`s its integration ahead of planning, so planning reads an
  agreed design.
- The owner's major-release decision (D8) follows the breaking changes:
  whichever release ships them is the major one, if planning spreads the work
  over more than one grove.

## Decisions (running log)

**W1 — two nested jobs, one per supervisor.** Grove's runner makes
`harness-dispatch run` a foreground job (own process group, the terminal handed
to it, default terminal-signal dispositions), and dispatch does the same for the
harness it spawns. The worker joins dispatch's group, as today. So during
selection a typed Ctrl-C reaches dispatch and its worker and cancels the
selection, and while the harness runs it reaches the harness's group alone.
Each launcher saves the terminal's attributes when it hands the terminal over
and restores them when it takes it back. Rejected: dispatch in the driver's own
group (the driver would be in the foreground group during selection, and the
driver could no longer address dispatch's group if dispatch died); the harness
in dispatch's group (a typed Ctrl-C would reach the wrapper, which the brief
forbids).

**W2 — the runner stays `keyed-launch`, shared.** Dispatch depends on it for
the harness's job, its exit channel, the escalation and confinement; Grove keeps
it for running dispatch (the job and terminal, launch events, cancellation
forwarding, the noninteractive mode for `grove run`) and no longer allocates a
channel or confines anything. `keyed-launch` is domain-free, so the dependency
keeps dispatch free of Grove's domain, task-tree and jj packages; an extraction
takes `keyed-launch` with dispatch, and Grove then depends on it from there. The
runner's token goes: the exit signal carries nothing. Rejected: moving the
runner into dispatch (Grove still needs a job runner for dispatch, so two copies
of the job code), and leaving Grove the escalation (D4).

**W3 — the exit channel lives where the caller says, else in a private run
directory.** `run --exit-dir DIR` makes dispatch allocate the run's fresh
128-bit exit channel in a directory the caller already has; without it dispatch
allocates it in a private per-run directory it creates under TMPDIR and removes.
Dispatch cannot know where a harness's own sandbox lets it write, and the caller
can: Grove passes its launch directory under the workspace's `.jj/grove/`, the
location today's signal file has proved writable from inside the session's own
sandbox. Under `--confine` dispatch grants the exit channel's directory to the
harness. It is not a selection parameter: it never reaches the policy, whose
worker must not learn the channel (it would carry the authority to end the run).
Rejected: a policy-chosen directory (every owner policy would carry launch
mechanics, and one that forgot would stall its sessions); dispatch deriving a
directory from the cwd (inside the working copy jj snapshots it, and `.jj/`
would be jj knowledge in dispatch, against D1); an inherited descriptor (a
harness's tool runner need not pass it through); signalling dispatch's PID
(sandboxes deny signals that leave the sandbox).

**W4 — `harness-dispatch exit`.** The exit verb takes no argument and carries
nothing: it creates the file `HARNESS_DISPATCH_EXIT_FILE` names, and an existing
one is success. Outside a supervised run (variable unset or empty) it is a safe
no-op that says so, exit 0, matching today's `complete`. It reads no owner
setting, policy or record, so it works inside confinement. Dispatch publishes
the variable to every harness beside `HARNESS_DISPATCH_RUN_ID` and
`HARNESS_DISPATCH_STATE_DIR`, replacing an inherited value, and never to the
worker; a nested run's harness therefore ends only the nested run.

**W5 — the ending is reported in the existing observation format.**
`run --ending-file PATH` (absent at input validation, parent a directory) makes
dispatch write, once the harness is reaped, the same end observation it appends
to the run (D7): an observation document whose measurements carry the new
`ending` (`exit_signal`, `harness_exit` or `cancelled`), `exit`, `duration` and
`executionConfirmation`. Without the flag the observation is still appended,
and a one-line end notice goes to stderr either way. No run, no ending: a
refusal or a cancellation at the linearization point writes none. A failed
append does not withhold the file or change the exit status. Cancellation
takes precedence over the exit signal when both occurred. Rejected: the exit
status as the report (a harness that quits with 0 and one ended through the
exit signal collide, which is exactly the case Grove must tell apart); a report
on stderr (the terminal's in a lifecycle session); a descriptor the caller
passes (a new runner feature for no gain over a file only the caller reads after
reap).

**W6 — dispatch's exit status.** A harness that exits on its own: its code, or
dispatch dies of its signal. The exit-signal ending: 0 when the escalation ended
the harness or it exited 0 within the grace, otherwise the harness's own status.
Cancellation: dispatch dies of the cancelling signal. Pre-launch exits keep
their codes. So `grove run` publishes on the exit-signal ending with status 0,
which keeps today's rule that a harness failing after acknowledging publishes
nothing.

**W7 — the graces are dispatch's constants.** Grace 2 s and kill-grace 5 s, as
today's driver constants, not owner settings, flags or policy fields. On
cancellation an interactive run forwards the cancelling signal to the harness's
group and kills it after the kill-grace; a confined run kills the harness's
group at once, as the noninteractive runner does today, so nested cancellation
finishes inside an outer grace. Grove forwards its own TERM or HUP to dispatch
and waits up to 10 s, longer than dispatch's kill-grace and record lock wait
together, before killing dispatch's group. Rejected: an owner setting (no owner
has needed one, and it would be one more key read before every launch); select
result fields (a contract change for a value the policy is gone before it
matters); caller flags (the caller does not know the harness's shutdown).

**W8 — when dispatch dies.** INT, TERM and HUP cancel a run at any stage unless
ignored at entry, so only an ending dispatch does not survive (SIGKILL, abort)
leaves a harness running: it is orphaned in its own group, nothing escalates it,
and its run stays as it stood (D7). Grove sees dispatch dead with no ending, so
the ending is not the exit signal and the loop stops with the leaf live — it
never relaunches onto a tree a surviving harness may still be changing — and
says that the harness may survive. Admission already refuses a surviving
session's tree verbs once the epoch is invalidated. Rejected: Linux's parent
death signal (one platform, the direct child only); Grove escalating a group it
learns from dispatch (D4).

**W9 — no `exec` verb.** D3 permits one; nothing calls it, Grove never would,
and it would keep a second launch path alive for no caller. Reopen when a caller
needs the harness to keep dispatch's PID. The entry signal state machinery
stays, because the spawned harness receives it.

**W10 — Grove's launch directory.** The driver allocates a fresh owner-only
directory `launch-<128-bit hex>` in its control directory per launch and
publishes it as `GROVE_LAUNCH_DIR` (replacing `GROVE_SIGNAL_FILE`). The session
epoch binds its path where it bound the signal path, so admission is unchanged
in shape: same comparison, control directory from the path's parent. It holds
the teardown record and dispatch's ending file, and dispatch's exit channel is
allocated in it. The driver removes it after interpreting the launch; a
replacement driver removes abandoned ones after taking the lease and
invalidating the old epoch, reading nothing in them, so no record outlives its
launch or crosses drivers (the one-live-driver record's rejected finish
tombstone stays rejected).

**W11 — `grove-llm record-teardown`.** The teardown verb, named for the record
it writes so that it is not read as performing a teardown (that is
`finish-commit`). Admitted under the session epoch like any tree verb; refuses
while `.grove/` still exists, naming `finish-commit`; creates `teardown` in the
launch directory, and an existing record is success; outside a loop a no-op
that says so. `grove-llm complete` is removed with no stub: the prompt, which
the binary composes, always names the current verbs. The finish ends with two
commands, `grove-llm record-teardown` then `harness-dispatch exit`.

**W12 — how the loop reads a launch.** After reaping dispatch and invalidating
the epoch: the driver's own TERM/HUP → interrupted (re-raised); else a teardown
record → finished, whatever the ending (the record is the deliberate act, so a
recorded teardown followed by the harness exiting on its own finishes); else an
`exit_signal` ending → relaunch; else → stopped with the leaf live (the
harness's own exit, a cancellation of dispatch, a refusal, dispatch's death, a
missing or unreadable ending file).

**W13 — `grove run` through `run --confine`.** Grove stages the private working
directory and runs `harness-dispatch run --confine` there, with any
`--runtime-read` grants and an `--ending-file` in Grove's private root outside
the sandbox, noninteractively (its own session, null stdin, output to the
transcript), in Grove's environment minus loop-control variables. Under
`--confine` dispatch confines writes to its cwd and its private run directory,
grants reads of the selected executable, its own executable and the runtime
grants, and gives the harness a minimal environment (HOME, USER, LOGNAME, PATH,
LANG, LC_*, a private TMPDIR, and its three reserved variables) — the scrub that
moves behind selection, so the policy keeps the owner's grants. The prompt names
dispatch's canonical path for `exit`. Grove publishes only on the exit-signal
ending with status 0 and no cancellation. The run record notes the
confinement and its grants.
