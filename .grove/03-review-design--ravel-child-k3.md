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
