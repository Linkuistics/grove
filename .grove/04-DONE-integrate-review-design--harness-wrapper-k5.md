# harness-wrapper-k5

**Integrates:** harness-wrapper-k3

## Goal

Triage `harness-wrapper-k3`'s design findings against the settled requirements
and apply those that hold, leaving a coherent design that
`supervised-dispatch-k4` can plan. This session owns design corrections and
their verification, not implementation of the harness wrapper.

## Context

- Read the findings in `harness-wrapper-k3`'s committed review artifact, and
  its diff against its parent. Classify each finding on evidence rather than
  treating it as an agreed work list.
- The reviewed producer is `harness-wrapper-k2`, commit `f653dd72`. Its
  specification, ADRs, related specs, glossary and visual design are the
  artifacts to reconcile where triage supports a correction.
- The root brief carries the owner's requirements and four agreed seams.
  `plan-k1`'s running log holds the owner's answers; `harness-wrapper-k2`'s
  running log explains the design choices. Planning follows this task.

## Done when

- Every finding has an explicit, evidenced disposition in this task's running
  log, including any rejection or accepted trade-off.
- Supported corrections are applied consistently to the design artifacts and
  their acceptance obligations, and verified by this kind's procedure.
- Any remaining requirement-level trade-off is resolved or externalized before
  planning consumes the design.

## Notes

This grove still runs the installed v22 binaries and skills. Do not change the
installed signal contract as part of design integration; end this session with
the verb its launch prompt and installed skill name.

## Decisions (running log)

Findings read from `harness-wrapper-k3`'s commit (`3e80f44f`), graded against
`harness-wrapper-k2`'s artifacts, the current source and the root brief.

**I1 — F1 holds: a real issue; the artifact is fixed. A run's job ends with its
run.** Reproduced from the source: the interactive `watch` in
`crates/keyed-launch/src/run.rs` returns on the leader's reap and drains the
group only when `detached`, and the spec selects the ending at that reap. A
harness that exits on TERM leaves a TERM-ignoring tool alive, and an
`exit_signal` ending then relaunches beside it. Correction, in the runner and so
for both callers: in every launch, interactive or confined and whatever the
ending, the runner observes its child's exit without reaping it, sends SIGKILL
to what remains of the child's group while the unreaped child still reserves
the group's ID, reaps the child (its status stays the run's exit), and then,
querying only, confirms within 1 s that the group is gone. Dispatch reports the
ending only after that. A group still present is a supervision failure: the end
observation is appended as observed, no ending file is written, stderr says
members of the group may survive, and dispatch exits 5, so Grove stops and
`grove run` publishes nothing. Killing on `harness_exit` too is a visible
trade-off: a harness that wants a descendant to outlive it must detach it into a
group of its own, which is already outside the contract, and uniformity stops a
lingering `grove-llm` from holding shared epoch admission after any ending.
Grove's 10 s wait still exceeds kill-grace 5 s, lock wait 2 s and drain 1 s.
Rejected: a second grace for the remaining members after the harness exits (the
harness's own shutdown is where a descendant gets its time, and a second grace
lengthens every such ending and Grove's bound with it); reaping first and then
signalling the group (once the group empties its ID can be reused). Seam 1 gains
a TERM-exiting harness with a TERM-ignoring descendant, a harness that exits
within the grace leaving one, and one that exits on its own leaving one; seam 4
gains an acknowledged clean harness that leaves a writer, which is stopped
before anything publishes.

**I2 — F2 holds: a real issue; the artifact is fixed. `--confine` refuses an
overlap with owner data.** Reproduced from the source: both backends grant
whole roots, Seatbelt a writable `subpath` and bubblewrap a read-write bind,
and each runtime read is a literal grant (`crates/keyed-launch/src/confinement.rs`);
nothing excludes a protected path beneath a granted root, so a cwd of HOME puts
the default policy, settings and store under the writable root. Correction:
before selection, `run --confine` compares canonical paths, refusing with exit
2 when a granted path overlaps a protected one, one being the other or lying
inside it. The granted paths are the cwd, the exit directory, the private run
directory and each runtime read; the protected ones are the directory holding
the selected policy entry, the owner settings file and the state directory. The
two executables dispatch grants are exempt: the harness must run them. Grove's
staged directory overlaps none. Rejected: exclusions inside the sandbox
(Seatbelt can deny beneath an allowed subpath, but bubblewrap's bind has no
exclusion short of masking mounts, and two backend-specific masks are more to
prove than a refusal no legitimate caller meets). Seam 1 gains the refusals,
because they precede the backend and so run on every platform: a policy in the
cwd, a store in a granted exit directory, a runtime read naming the settings
file, and one reached through a symlinked alias.

**I3 — F3 holds: a real issue; the artifact is fixed. Reclaiming the terminal
is launch-scoped.** Reproduced from the source: the runner reclaims only when
the foreground is its direct child's group (`run.rs`, the recovery closure), and
the driver's reset does nothing unless the driver already holds the foreground
(`reset_terminal` in `crates/grove-loop/src/loop_driver.rs`). After dispatch's
death the foreground is the orphaned harness's group, so neither acts, against
the spec's "takes back and restores the terminal if it is still held".
Correction, in the runner for both callers: a launcher that handed the terminal
to its child during a launch takes it back, once the child is reaped, from
whichever group then holds it, unless that group is its own or the session
leader's, and restores the attributes it saved at that handover. A launcher
that never handed it over takes nothing. The orphaned group is then in the
background, where POSIX fails an orphaned group's terminal reads with EIO, so
it can no longer take the human's input. (Not its attribute changes: POSIX
admits those from a background process that ignores or blocks SIGTTOU, so the
spec claims only the reads.) The session-leader exclusion is what keeps the rule from stealing: once
the launcher has handed the terminal down, only its descendants can hold it, or
the shell after stopping the launcher's job, and the shell leads its session in
an ordinary terminal, a tmux pane and an ssh login. Accepted residue: a
job-controlling shell nested in another within one session that reclaims the
terminal after stopping the driver loses it again at the reap. Seam 3's
dispatch-death row gains the observation: with a raw-mode fake still alive, the
driver's group holds the foreground with the handed-over modes restored; and a
driver started in the background takes no foreground.

**I4 — F4 holds: a contract stated unclearly; the contract is fixed.** The spec
says only `record observe` migrates a version-1 store and every other command
writes either version as it is, yet `run` now appends its end observation, and
the shared append (`append_observation` in `crates/harness-dispatch/src/store.rs`)
already migrates inside its own exclusive transaction. Correction: a version-1
store is migrated by the first observation appended to it, an import or `run`'s
end observation, inside that append's own transaction, and a refused or failed
append leaves it at version 1. Seam 2 gains a supervised run against an existing
version-1 store that records its end observation and reads
`execution_confirmed` with no import between.

**I5 — F5 holds, resolved as a visible trade-off; the contract is fixed.**
POSIX `fork` gives the child an empty pending set, and `exec` preserves the
caller's, so the old `exec` handoff kept a caller-blocked signal pending in the
process that became the harness, and a spawn leaves it pending in dispatch. The
root brief guarantees the entry mask and dispositions, not pending signals, and
no caller depends on one. Accepted: the harness starts with the entry mask and
dispositions and nothing pending, like any spawned child, and a caller-blocked
signal sent to dispatch stays pending there undelivered. The spec's
"transparent to signal state" narrows to mask and dispositions. Rejected:
forwarding dispatch's pending blocked signals (consume each with `sigtimedwait`
and re-send it): it needs a relay polling a mask the harness may change, loses
the signal's information and real-time queueing, and serves no caller. Seam 1
gains: a blocked signal sent to dispatch before the spawn stays pending in
dispatch, and the harness starts with it blocked and not pending.

**I6 — the one in-session reviewer, spent on I1's and I3's rules.** One fresh
context was given the corrected supervision text, decision 7's prose, the job
ADR and the brief's guarantees, and asked to disprove the reclaim rule and the
group cleanup against POSIX job control on macOS and Linux. It measured on both
platforms and returned sixteen findings, each classified here:

- **Applied, they tighten I1's rule.** (3) macOS `waitid(WEXITED|WNOWAIT)` also
  reports a stopped child, so the rule says that a stop is not an exit.
  (5) On macOS, a member forked while the group SIGKILL lands can survive it,
  measured in about two runs in three of a tight fork loop. Linux restarts the
  fork. So the kill is repeated while the unreaped harness still reserves the
  ID. (10) `kill(-pgid, 0)` answers EPERM for a member the caller cannot signal,
  and macOS gives EPERM for a group of zombies too, so only ESRCH confirms the
  group gone. (11) An inherited ignored SIGCHLD auto-reaps the child and removes
  the ID reservation, so a supervisor restores SIGCHLD's default for itself
  while still passing the entry disposition to its child. (15a) The ending
  *file* is what a caller acts on, so the text says so. (1) Verified here: this
  session's own shell commands run as session leaders outside Claude Code's
  group, so neither the escalation nor the cleanup reaches them. That predates
  this design, but it makes "an agent's own in-flight command dies with the
  session" overclaim. The spec, the job ADR, the lease ADR and the standalone
  spec now name a harness's own sessions as outside the group's reach. The
  standalone spec's existing sentence, that a group kill proves no process tree,
  already covers publication.
- **Applied, they narrow I3's rule.** (12, 6) Reclaiming from any group except
  one's own and the session leader's would let dispatch, reaping a harness that
  exited normally, take the terminal from a nested job-control shell that took
  it after the driver died, and would let a nested dispatch in an orphaned group
  take it from Grove. The broad take-back now applies only when the child *died
  of a signal*, the one way a supervisor between can leave a delegated terminal
  unreclaimed. Otherwise the launcher takes the terminal back from its child's
  group alone, as today. (9) The take-back is made with SIGTTOU ignored, as the
  handover already is. (15c) "Never handed it over" becomes "never held the
  foreground during the launch", which the per-tick handover can decide. (7)
  Under a subreaper the orphan's group is not orphaned, and a read stops it
  rather than failing, so the spec says the read "fails or stops the reader".
  The seam drops the case of a harness handing the terminal to a group of its own
  and gains a normal exit that leaves a group not its child's in the foreground.
  The rule is covered by executable seam cases, so it does not force a second
  review.
- **Pre-existing, accepted visibly in the spec's Out of scope.** (2)
  Job-control stops are not relayed: a harness stopped by a typed Ctrl-Z under
  ISIG, by suspending itself or by a background read keeps the terminal, and
  neither supervisor waits for stops, as Grove's runner never has. (4)
  `grove &` then `fg` hands the terminal down but continues no stopped child.
  (8) The handover gate checks the launcher's terminal, not that its stdin is
  that terminal. Each predates this design, the owner asked for none, and each
  can be stated precisely, so the spec records them with what would reopen them
  rather than growing this leaf.
- **Noise for want of context, or accepted as failing safe.** (13) The child
  already hands itself the terminal before exec, in the runner's pre-exec hook,
  so there is no handover race. (14) A confined harness starts in a session of
  its own with null stdin, which the excerpt omitted. (15b) A query after the
  reap that hits a reused ID reads as "present" and fails safe. (16) A member in
  uninterruptible sleep is a supervision failure, which fails safe; restoring
  screen modes is the driver's reset's job, which it already does.

**I7 — nothing requirement-level is left for the owner; planning may read the
design.** Every guarantee in the root brief still holds, and two are stronger.
Descendants die under the escalation, and now at every ending, within the
stated limit of the group. The entry mask and dispositions pass on; pending
signals were never promised. The confined harness cannot reach the policy,
settings or store, which a refusal now enforces rather than assumes. The
terminal comes back restored, a dispatch's death included. The accepted
trade-offs are design-level and visible where a reader meets them: killing the
group on `harness_exit` (I1), spawn's pending-signal semantics (I5), and the
pre-existing job-control limits in the spec's Out of scope (I6). Verified: the
links and anchors of every changed document resolve; `diagrams.json` parses;
all nine views render in the design viewer with the new states and messages; a
sweep for each replaced claim is clean now and dirty against `f653dd72` with
the same instrument; and every broad take-back of the terminal is conditioned on
a death by signal.
