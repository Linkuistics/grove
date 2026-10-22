# grove.gh-issue-22 — brief

## Goal

Remove the local `keyed-launch` runner of Grove. Grove then starts `ravel` as
an ordinary child process, as a shell starts a foreground job. Ravel keeps job
control of the managed process and does not change. The owner's statement of
the work is the GitHub issue Linkuistics/grove#22.

## Done when

- `crates/keyed-launch` no longer exists, and its tasks, tests, walkthrough book
  and book structure spec are gone with it.
- Both callers start `ravel` through one shared Grove function, and each item
  under *Requirements* holds.
- Grove refuses Ravel 1.0.x and accepts stable Ravel 1.x from 1.1.0.
- The tests at the two agreed seams pass. See *Pointers*.
- Each record that stated the reversed decision describes the new arrangement.
- `bash scripts/check.sh` passes on the result.
- The decisions under *Requirements* live in durable records, and not only in
  `.grove/`.

## Requirements

The owner settled these in `plan-k1`. Its running log holds the reasons.

### Settled by the issue

- Grove removes `crates/keyed-launch` with its tasks, tests and documents.
- Grove starts `ravel` in a process group of its own, and makes that group the
  foreground group of the terminal, as a job-control shell does.
- Ravel keeps job control of the managed process. The change needs no change in
  Ravel.
- Each record that states the reversed decision gets a rework.

### Settled with the owner

- **Ravel alone gives the cleanup assurance.** A successful Ravel status with an
  exact result is the only evidence of cleanup. Grove sends no KILL to `ravel`
  and probes no process group after the reap. Rule 4 of the lifecycle
  interpretation goes, so a surviving group no longer stops the loop. The
  survivor guard of `grove run` and the KILL bounds of 10 and 5 seconds also go.
- **Grove requires Ravel 1.1.0.** The version check accepts stable Ravel 1.x
  from 1.1.0. It refuses 1.0.x with the existing upgrade remedy.
- **Two test seams, and only real Ravel checks job control.** *Pointers* lists
  them.

### Behaviour that stays

- A typed Ctrl-C reaches the harness and never ends the driver. The driver
  still ignores SIGINT. `ravel` starts with the default disposition for each
  job-control signal, so the ignore of the driver cannot reach the harness.
- A TERM or HUP to the driver during a session goes to `ravel`. Grove waits for
  `ravel` to exit. Then the loop stops, and `grove` dies of the same signal.
- A TERM or HUP between sessions, or after the reap, also stops the loop, and
  `grove` dies of that signal.
- The rest of the lifecycle precedence does not change. After the reap and the
  epoch invalidation, driver interruption comes first. A successful status with
  `grove-force-stop` comes next, then the teardown record.
- A successful status with `grove-continue` then starts the next pick, and
  anything else stops the loop.
- After `ravel` exits, the driver takes back the foreground of the terminal and
  leaves the terminal usable, as it does now.
- `grove run` cancels on Ctrl-C, TERM or HUP. It publishes its outputs only
  after a successful Ravel status with `grove-continue`. Its transcript relay
  works as it does now.
- Every spawn removes the loop control variables. Only the lifecycle session
  receives `GROVE_LAUNCH_DIR`.
- The driver lease receives a Started notice after a successful spawn, and a
  Reaped notice after a successful wait.

## Decomposition

- `01` `plan-k1` (requirements): the requirements above, settled with the owner.
- `02` `ravel-child-k2` (design): the mechanics and the records that the design
  reworks. Its last act cuts the leaf that follows it.

## Pointers

- Issue: Linkuistics/grove#22, with its *Known work* section.
- ADRs: `docs/adr/dispatch-supervises-the-harness.md`,
  `docs/adr/the-launched-child-is-a-job.md`,
  `docs/adr/one-live-driver-per-working-tree.md` and
  `docs/adr/evidence-outlives-the-instrument.md`.
- Specs: `docs/specs/harness-selection-and-execution.md`, especially its
  *Lifecycle interpretation* and *Caller acceptance* sections, and decision 7 of
  `docs/specs/module-decomposition.md`.
<!-- tc: depart synonym-rotation because Launch directory is a defined glossary term -->
- Glossary terms in play: **Driver lease**, **Session epoch**, **Launch
  directory**, **Run ending**, **Teardown record** and **Session witness** (see
  `CONTEXT.md`).
- The guarantees of Ravel to its caller: chapter 9, "Jobs, signals and the
  terminal", of the book of Ravel, at
  `docs/book/src/09-jobs-signals-and-the-terminal.md` in the Ravel repository.
  The v1.1.0 entry of the Ravel `CHANGELOG.md` lists the five corrected failure
  paths.
- Test seams agreed with the owner:
  - **The post-reap interpretation.** Unit tests drive `interpret_launch` with a
    result value that Grove owns, and they need no OS process.
  - **The installed Ravel 1.1.0 with fake agents**, in the tasks that need
    `RAVEL`, for the lifecycle and the standalone shapes. These tests check the
    job control of the shared Grove function. No test uses a stand-in child, so
    the default suite does not check job control.
  - The existing resolver tests also get the 1.1.0 floor. That extends a seam
    and adds none.

## Notes

- **Consumers outside the issue.** Production code also uses `Argv`,
  `LaunchEvent`, `Channel::discard_abandoned`, `regular_file_at`,
  `take_interrupt` and `reraise`. The tests of the driver lease use
  `run_observed` and `Channel`. Grove does not use the confinement module of the
  crate.
- **More records state the reversed decision.** Beyond the two that the issue
  names, these include `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`,
  `docs/RELEASING.md` with its count of product packages, and `Taskfile.yml`.
- **The books quote the code exactly.** The `keyed-launch` walkthrough book goes
  with the crate. Chapters of the `grove-loop` and overview books quote the
  changed code, and `scripts/check.sh` gates them. So each code change lands
  with its book edits.
- **Each edited record needs a conversion first.** No record that this work
  edits passes the controlled-English checker today. The
  `technical-communication` skill converts each one as its own change before the
  edit. A large document converts only the sections that the edit touches.
- **The project Taskfile.** `Taskfile.yml` is the entry point for repeatable
  work. Its `test:keyed-launch` task goes with the crate, and so does the use of
  that task in `ravel:activation`.

## On the horizon

- The planning of the code, book and record changes, after the design settles
  the mechanics.
