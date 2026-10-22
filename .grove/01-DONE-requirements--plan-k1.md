# plan-k1

## Goal

<!-- tc: depart synonym-rotation because the issue title is quoted text and keyed-launch is a crate name -->
Establish what this grove builds, in the owner's own words. The owner's words
are the GitHub issue Linkuistics/grove#22, "Remove the local keyed-launch runner
and start ravel as an ordinary child", opened on 2026-10-21. This leaf is the
bootstrap requirements leaf of the grove, so the issue is its only input.

## Context

- Linkuistics/grove#22: the statement of the work and of the owner's decision.
- `docs/specs/harness-selection-and-execution.md` and
  `docs/adr/dispatch-supervises-the-harness.md`: the two records that the issue
  names as statements of the decision that this work reverses.
- Ravel's book, chapter 9, "Jobs, signals and the terminal", at
  `docs/book/src/09-jobs-signals-and-the-terminal.md` in the Ravel repository:
  the guarantees that `ravel start-session` gives its caller.

## Done when

- The root brief records the requirements in the owner's words.
- Each open question has an answer from the owner, or a named leaf that owns it.
- The next leaf exists, so the driver can continue the loop.

## Notes

The issue names two call sites. The crate has more consumers in production
code, and the requirements must cover each of them:

- `Argv`, in the session command builder.
- `LaunchEvent`, in the launch supervision of the driver lease.
- `Channel::discard_abandoned`, which removes `signal-` channel files that
  older drivers left in the control directory.
- `regular_file_at`, which reads each standalone output through the held work
  directory.
- `take_interrupt` and `reraise`, the signal latch and the exit by signal.

## Decisions (running log)

- **The issue settles the direction (owner, 2026-10-21).** Grove removes its
  local `keyed-launch` runner, which is the whole crate. Grove starts `ravel` as
  an ordinary child process, as a shell does. `ravel` runs in a process group of
  its own, and Grove makes that group the foreground group of the terminal. Job
  control of the managed process stays with Ravel. The change needs no change in
  Ravel. Each record that states the reversed decision gets a rework.
- **The issue gives five points to the design (owner, 2026-10-21).** The design
  settles them:
  - The handling of SIGINT, SIGTERM and SIGHUP while `ravel` runs, and the
    report of an interrupted session.
  - The start and reap notices that the driver lease needs.
  - The replacement for the check of `Group::Present`.
  - The scrub list and the `GROVE_LAUNCH_DIR` grant.
  - The removal of the crate with its tasks, tests and documents, and the
    rework of the records.
- **Ravel alone gives the cleanup assurance (owner, 2026-10-22).** Grove keeps
  only the behaviour of a job-control shell. That is a foreground process group
  for `ravel`, the forwarding of a signal to it, terminal recovery, and the
  existing report of an interruption. A successful Ravel status with an exact
  result is the only evidence of cleanup. Grove sends no KILL to `ravel` and
  probes no process group after the reap. Rule 4 of the lifecycle
  interpretation, the survivor guard of `grove run` and the KILL bounds of 10
  and 5 seconds go.
- **Grove requires Ravel 1.1.0 (owner, 2026-10-22).** The version check accepts
  stable Ravel 1.x from 1.1.0, and it refuses 1.0.x with the existing upgrade
  remedy. Grove now relies on the failure paths of Ravel, and 1.1.0 corrects
  five of them. One correction stops a session when Ravel cannot check that it
  reaped its policy worker. The survivor check of Grove used to catch that case.
- **Two test seams, and only real Ravel checks job control (owner,
  2026-10-22).** The first seam is the post-reap interpretation. Unit tests
  drive `interpret_launch` with a launch result that Grove owns, and they need
  no OS process. The second seam is the installed Ravel 1.1.0 with fake agents,
  in the tasks that need `RAVEL`, for the lifecycle and the standalone shapes.
  Those tests check the job control of the new launcher. No PTY test uses a
  stand-in child, so the default suite does not check job control. The existing
  resolver tests also get the 1.1.0 floor.
