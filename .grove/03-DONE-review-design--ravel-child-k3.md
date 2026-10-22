# ravel-child-k3

**Reviews:** ravel-child-k2

## Goal

Read the design of `ravel-child-k2` adversarially, and report findings without
fixes. The design removes the local runner of Grove and starts `ravel` as an
ordinary child process.

## Context

- The design commit of `ravel-child-k2`, and the running log of decisions in
  that leaf.
- The reworked records. The first is the caller contract,
  `docs/specs/harness-selection-and-execution.md`, in its sections *Jobs and
  authority* with their subsections, *Lifecycle interpretation*, *Standalone
  interpretation* and *Caller acceptance*. The others are
  `docs/specs/standalone-invocations.md`,
  `docs/adr/the-launched-child-is-a-job.md`,
  `docs/adr/dispatch-supervises-the-harness.md` and the views in
  `docs/design/harness-selection-and-execution/`.
- The root brief: the *Requirements* that the owner settled, and the *Design*
  that holds the work orders for planning.
- The current runner, `crates/keyed-launch/src/run.rs`, and its two callers.
  They show the behaviour that the design keeps or changes.
- Chapter 9 of the book of Ravel at v1.1.0, for the guarantees that the design
  relies on.

## Done when

- Each claim under *Notes* has a finding, or a statement that it holds.
- The edited records form a minimum coherent set, with no dangling citation and
  no restatement between the spec and the ADRs.
- Each finding names its record and section, and states how the record fails.

## Notes

The owner settled the *Requirements*. A finding against them is out of scope,
unless the design contradicts them.

Read these claims hardest. Each is a narrow claim about signals or the terminal,
and no compiler checks it.

- **The spawn never clears the latch.** The current runner clears it, to keep a
  stale signal off a fresh child. The design sends such a signal to the new
  `ravel` instead. Check that no path spends a signal on a session that it does
  not concern.
- **One rule for INT in both shapes.** The function catches INT unless its
  disposition is the ignore. Check the lifecycle without a terminal, a driver
  under `nohup`, and `grove run` from inside a harness.
- **Signals go to the `ravel` process only.** Check this against the
  cancellation contract of Ravel, during selection and during supervision.
  Check also a `ravel` that dies before it installs its handlers.
- **No KILL and no bound on the wait.** Check what a stuck `ravel` now costs the
  driver and `grove run`, and whether the records say so.
- **The detached job for `grove run`.** Check its reason against the foreground
  alternative.
- **The terminal residues.** The design accepts the failed exec and the TERM
  before the first session as they are today. Check that the spec and the root
  brief state them accurately.
- **The removal of the `signal-` cleanup.** Check that no current path creates
  or reads those files.

## Findings

Reviewed producer commit `7cf610b9737fde3ffefadb3f682de79b527da4a2`.
The findings concern the proposed contract, before implementation.

### F1 — Preserve interruption when the shared function returns an error (P2)

**Locations:** `docs/specs/harness-selection-and-execution.md:165`,
*Signals and interruption*; `:205`, *Launch notices*; `:267`, *Lifecycle
interpretation*; the root brief, *The shared function* and *Residues*.

The successful end value carries the interruption, but the design does not give
an owner to that information when spawn or wait fails. The reap drains the
latch only on success. A failed wait instead returns an error naming Ravel's
PID. The lifecycle's signal precedence is specified only after a successful
reap and epoch handoff.

Consider TERM caught after the final loop check, followed by a failed spawn.
There is no first poll to forward TERM. This can happen if the executable
disappears after the version probe, a possibility the spec explicitly leaves
open. In the current caller, `complete_post_reap_epoch_handoff` propagates the
launch error at `crates/grove-loop/src/loop_driver.rs:236`, before the latch
check at `:246`. The CLI then propagates the error at
`crates/grove/src/cli.rs:100`, bypassing `reraise` at `:105`. Keeping that caller
flow with the proposed latch rule loses the required death by the original
signal.

A failed wait has a second form of the same problem: a poll can consume and
forward TERM, then fail to obtain a status. The old runner demonstrates this
sequence at `crates/keyed-launch/src/run.rs:1263` and `:1281`. Its error has no
interruption field (`crates/keyed-launch/src/error.rs:9`), and it cannot return
the signal in an end value. Moving the latch alone does not settle this path.

**Smallest useful correction:** define how interruption survives a failed
spawn or wait, including one already consumed by the polling function, and how
the lifecycle preserves it through error and epoch-invalidation handling to its
final exit. Preserve the failure diagnostic, the absence of an invented
`Reaped` notice, and the owner's no-KILL decision. The durable spec should also
state the handler-installation boundary: TERM before the first call takes its
entry disposition, while TERM caught after installation belongs to the driver
even if spawn fails. The root brief currently states only the first residue.

This is a missing failure contract, supported by source inspection. It is not
a claim that the proposed implementation has been run and failed.

### F2 — Keep executable contract rules in the spec (P3)

**Locations:** `docs/adr/the-launched-child-is-a-job.md:3`, opening paragraphs;
`docs/adr/dispatch-supervises-the-harness.md:25`, *Grove's side of the boundary*,
and `:49`, *Resolution and costs*.

The reworked ADRs still reproduce procedures that the caller spec now defines
in detail. The job ADR repeats the handover timing, reclaim cases, signal
forwarding and detached shape from *Jobs and authority*. The supervision ADR
repeats the complete lifecycle precedence, standalone publication rule, ending
commands, legacy-session exception and executable-resolution procedure from
the caller spec. The supervision ADR also repeats the job and terminal
arrangement of the job ADR.

This fails this review's explicit no-restatement completion condition. A later
change to a reclaim condition or interruption precedence requires several
independent prose edits, with no declared rule for resolving a disagreement.
The spec even says the job ADR owns the caller side at
`docs/specs/harness-selection-and-execution.md:132`, while also stating those
same mechanics itself.

**Smallest useful correction:** retain each ADR's decision, reason, material
costs and rejected alternatives, and cite the spec sections for the procedures.
The two decisions remain independent: invoking an external supervisor does not
itself choose the foreground and detached arrangements. Two ADRs are justified;
the duplicated operating contracts are not.

## Checks of the requested claims

1. **The spawn never clears the latch:** supported for successful launches.
   The previous job drains its latch at reap, and the loop checks again after
   terminal reset and before the next iteration. A signal after the final
   check therefore cancels the impending job and stops its driver, rather than
   becoming an acknowledgement. The unsuccessful-launch path needs F1.
2. **One INT rule:** coherent with the stated inherited-ignore exception.
   The lifecycle explicitly ignores INT even without a terminal; resetting the
   child's dispositions prevents that ignore from reaching Ravel. A driver
   under `nohup` retains an inherited ignored HUP. A nested `grove run` catches
   nonignored INT and preserves an inherited ignore. The rule does not establish
   a default SIGINT disposition for every possible enclosing harness. With the
   usual default, the detached shape routes terminal Ctrl-C to `grove run`,
   which can cancel Ravel before publication.
3. **Signals target Ravel's PID:** supported. At tag `v1.1.0`, Ravel installs
   cancellation handling before policy selection in `crates/ravel/src/run.rs`.
   Chapter 9, *Handle cancellation*, covers selection and both supervision
   modes. Before those handlers exist, default-action death has no managed job
   to orphan. Death after workers or the managed job exist is handled by Ravel;
   an uncatchable death remains the documented limit.
4. **No KILL and no wait bound:** an explicit accepted trade-off, not a finding
   against the requirements. A stuck Ravel can hold either caller indefinitely.
   *Signals and interruption*, the job ADR's rejected bounded-wait option, and
   the standalone spec state the unbounded wait. The standalone spec also
   qualifies completion within an outer supervisor's grace. Ravel's own bounds
   do not establish a wall-clock bound for Grove.
5. **Detached standalone job:** supported. It keeps Grove's transcript relay
   in the foreground and routes typed INT to the caller that controls
   publication. A foreground Ravel job would put that relay in a background
   group, where terminal writes can stop it. EOF stdin and descriptor closure
   preserve the existing isolation. Confinement remains Ravel's responsibility.
6. **Terminal residues:** the failed-exec account matches the current source.
   The child's handover precedes exec, but the parent records the child's group
   only after successful spawn; its spawn-error path performs no reclaim.
   *Limits* and the root brief state that the driver exits and its shell
   recovers the terminal. The first-TERM residue also matches current handler
   installation in `run_with_output`; its durable failure-boundary contract
   needs the clarification in F1. The ordinary-exit, signal-death and
   unknown-status reclaim rules otherwise match the current lease.
7. **Remove `signal-` cleanup:** supported for the current production callers.
   The lifecycle passes `channel: None`; the standalone launch has no channel.
   The current launch-directory reader reads only `teardown`, and allocation
   uses `launch-` names. The remaining `Channel` allocations and publications
   found outside its own crate serve tests. The cleanup itself still scans old
   names, which is the code being removed; it is not a current completion
   consumer. Keeping launch-directory cleanup and porting the lease fixtures
   covers the continuing responsibilities.

## Scope and evidence limits

The inspection covered the producer diff, its decision log, the requirements,
the two caller specs, the two reworked ADRs and their caller views, the current
runner, both callers, and the relevant lease and launch-directory paths.
The changed local citations retain their targets; no ADR was removed or renamed.
The recorded implementation work for other books and records remains future
work, rather than evidence that the code has already changed.

The shared function hides the process mechanics and scrub list at an
appropriate boundary. Its two shapes encode real caller differences. The
post-reap interpretation and installed-Ravel acceptance seams match the owner's
choice. F1 identifies the missing state ownership at that function's failure
boundary. No additional abstraction or test seam is proposed.

Ravel evidence was read from tag `v1.1.0` in the local Ravel repository
(`2095cbd8`), including chapter 9, the changelog and `crates/ravel/src/run.rs`.
The issue was read through `gh issue view 22 --repo Linkuistics/grove`.
The web fetches of the GitHub pages failed, so they supply no independent
verification. The graph CLI refused `list_projects` because a
"pre-coordination or unverified CBM generation is active". No project or
generation could be confirmed, and no graph coverage claim is made. Direct
source reads and bounded searches supplied the code evidence instead.

No test, build, lint or format command was run. This review supplies inspection
findings; the integration leaf owns fixes and their verification.

## Decisions (running log)

- **The error contract needs a finding (review, 2026-10-22).** Record F1
  because the current caller's error propagation bypasses interruption
  reporting, and the proposed successful end value does not define the failure
  case. Do not change the settled cleanup ownership or add a KILL bound.
- **The ADR decisions remain separate (review, 2026-10-22).** Record F2
  against duplicated procedures. The external-supervisor decision and the
  choice of job arrangements retain independent reasons and alternatives.
