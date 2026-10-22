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

## Design

The design leaf `ravel-child-k2` settled the mechanics. The durable records hold
the design. The first is the caller contract,
`docs/specs/harness-selection-and-execution.md`, in its sections *Independent
packages*, *Jobs and authority*, *Lifecycle interpretation*, *Standalone
interpretation* and *Caller acceptance*. The others are
`docs/specs/standalone-invocations.md` and the ADRs
`docs/adr/the-launched-child-is-a-job.md` and
`docs/adr/dispatch-supervises-the-harness.md`.

The running log of `ravel-child-k2` gives the reason for each decision. This
section holds what the planning needs and what dies with `.grove/`.

### The shared function

- A new module of `grove-loop` owns the one function that starts `ravel`. The
  implementation chooses its names. Decision 7 of
  `docs/specs/module-decomposition.md` then records the shipped interface.
<!-- tc: depart synonym-rotation because Launch directory is a defined glossary term -->
- The function takes the command, the working directory, the held stdout file,
  an observer of the notices and a shape. The foreground shape carries the
  launch directory, and the detached shape carries the transcript file.
- It returns the end of the job: the wait status of `ravel`, the interruption
  and the elapsed time. The end has no group field.
- A failed spawn or a failed wait returns an error that carries the
  interruption. The function takes the latch into that error, as it does at
  the reap. A failed spawn sends no notice. A failed wait sends no Reaped and
  no KILL.
- The function installs the handlers at its first call. A signal that it
  catches after that call belongs to the driver, even when the next spawn
  fails.
- Each error that stops the loop keeps a caught signal. The CLI of `grove`
  reports the error, then dies of the signal from the launch error or the
  latch. A failed invalidation follows the same rule. The section *Lifecycle
  interpretation* of the caller contract states it.
- It polls at a constant interval, as the runner does now. At each poll it passes
  the foreground on, sends each latched signal to `ravel` and checks for an
  exit without blocking. A stop is not an exit.

### New homes

| Use of the crate | New home |
|---|---|
| `Argv` | A value type in `session_command`, with the program, the arguments and `words()` |
| `LaunchEvent` | The new module. `DriverLease::supervise_launch` keeps its shape |
| `take_interrupt` and `reraise` | The new module, which installs the handlers. The re-export in `lib.rs` follows them |
| `Channel::discard_abandoned` | None. The cleanup goes |
| `regular_file_at` | A private function in the standalone module of `grove` |
| `Launch::scrub` and the two copies of the list | One list in the new module, with a helper for the version probe and `stty` |
| `End`, `Ended`, `Group` and `Escalation` | The end value of the new module. `Group` and `Escalation` go |

### Removal

- Remove `crates/keyed-launch` and its entries in `Cargo.lock`.
- Remove its member line and its decision comment from the root `Cargo.toml`.
- Remove its dependency lines from `crates/grove-loop/Cargo.toml` and
  `crates/grove/Cargo.toml`.
- In `Taskfile.yml`, remove `test:keyed-launch` and its call in
  `ravel:activation`. Reword "both stdout runners" in `ravel:fixtures`.
- Remove the member `keyed-launch` from `grove_sources` in `testing/support.rs`.
- Remove the book `docs/walkthroughs/keyed-launch/`, its structure spec
  `docs/specs/keyed-launch-book-structure.md` and its row in
  `docs/specs/walkthrough-books.md`.

The tests of the crate go with it. The two agreed seams cover what Grove needs
from them.

### Records to rework

Each record below states the reversed decision or quotes changed code. A book
chapter lands with the code that it quotes, because `scripts/check.sh` gates the
quotes.

- `docs/specs/module-decomposition.md`: decision 1, with its table and prose,
  and decision 7 with the shipped interface.
- The book structure specs `grove-loop-book-structure.md`,
  `overview-book-structure.md` and `grove-llm-book-structure.md`, and
  `user-guide-coverage.md`.
- The book `docs/walkthroughs/grove-loop/`: chapters 01 and 16 to 20, with its
  concept index, source index and README.
- The other books: chapters 01, 05 and 06 of `docs/walkthroughs/overview/`, and
  chapter 07 of `docs/walkthroughs/jj-workspace/`.
- `docs/adr/one-live-driver-per-working-tree.md`. Its passage on orphaned
  processes credits Grove's job decision with the reap of the session's group,
  but Ravel's guarantee does that. Its "two generic runner events" become the
  two notices of the new module. Convert the touched sections first.
- The entry *Run ending* in `CONTEXT.md`. The rows and paragraphs of
  `CONTEXT-MAP.md` on the crate, and its "no surviving group".
- `docs/ARCHITECTURE.md` and `docs/grove-delivery-contraction.md`.
- `docs/RELEASING.md`: seven product packages become six. The anchor stays.
- `docs/USAGE.md`. Its standalone section names the survivor group of
  `grove run`, and its lifecycle section names a surviving supervisor group.
  Its section on stopping the loop sends the forwarded signal to Ravel's
  process group, but Grove sends it to the `ravel` process.
- `plugins/grove/skills/grove/references/driver.md`. Its post-reap order names
  a surviving supervisor group and a gone group.
- The comments that cite the crate in `crates/grove-llm/tests/removed_surface.rs`,
  `crates/jj-workspace/tests/environment.rs`,
  `crates/grove/tests/reference_navigation.rs` and
  `crates/grove/tests/loop_driver.rs`.
- The current-discussion panel in
  `docs/design/harness-selection-and-execution/diagrams.json`. Remove it once
  the code ships.

Dated evidence of bounded runs stays unchanged, by
`docs/adr/evidence-outlives-the-instrument.md`. These records are
`docs/ravel-caller-surface-audit.md`, `docs/ravel-caller-surface-inventory.tsv`,
`docs/grove-ravel-docs-evidence.md`, `docs/preservation-baseline.md` and
`docs/grove-session-control-evidence.md`. Released entries of `CHANGELOG.md`
stay too.

### The 1.1.0 floor in the release inputs

The release target tests pin the Ravel 1.0.0 archives, and Grove refuses 1.0.x.
Move each pin to the Ravel 1.1.0 archives, with SHA-256 digests measured from
the published release. That release has the same three targets.

- `scripts/ravel_target_inputs.py` and `docs/ravel-target-inputs.md`, with its
  table of digests and its consumer paths. Its dated measurement of a run
  stays.
- The archive names in `scripts/ravel-target-smoke.sh`,
  `scripts/ravel-target-smoke-target.sh`,
  `scripts/release-registration.test.sh`,
  `scripts/release-floor-backend-probe-host.sh` and
  `scripts/release-smoke-target.sh`.
- The version check in `scripts/grove-installed-smoke.sh`.
- The description of `ravel:targets:prepare` in `Taskfile.yml`, and the Ravel
  inputs that `docs/RELEASING.md` names.

### Tests

- Port `crates/grove-loop/tests/session_command.rs` and
  `session_result_installed.rs` to the new function, with one case per shape.
- Port the driver-lease tests that spawn `/bin/sh` to the new function. They
  check the order of the notices, not job control, and they run without a
  terminal. A launch directory replaces the `Channel` that named their path.
- Rewrite the two interpretation tests in `loop_driver.rs` without `Group`.
- Add unit cases at the post-reap seam for the errors. A launch error that
  carries a signal keeps the error for the report and ends with the signal. A
  failed invalidation after an interrupted job does the same.
- Drop the `signal-` fixtures from the loop-driver case on housekeeping. The
  cases on abandoned launch directories keep that cleanup covered.
- Add one standalone case under a pseudo-terminal. A Ctrl-C typed at
  `grove run` cancels it and publishes nothing. The foreground stays with the
  group of `grove run`.
- Give the resolver tests the 1.1.0 floor. `ravel 1.0.0` and `ravel 1.0.9`
  refuse, and `ravel 1.1.0` and `ravel 1.12.3` pass. Fixtures that print
  `ravel 1.0.0` as a valid version change to 1.1.0.
- The loop-driver cases under a pseudo-terminal stay the acceptance of the
  foreground job. Each must still pass with the installed Ravel 1.1.0.

### Residues

- A failed exec after the handover in the child leaves the foreground with the
  group of the failed child, as today. The driver exits, and its shell takes
  the terminal back.
- A TERM before the first session still ends the driver by the default action,
  as today. The function installs the handlers, so they exist only from the
  first job on.

## Decomposition

- `01` `plan-k1` (requirements): the requirements above, settled with the owner.
- `02` `ravel-child-k2` (design): the mechanics and the records that the design
  reworks.
- `03` `ravel-child-k3` (review-design): an adversarial read of the design.
- `04` `ravel-child-k5` (integrate-review-design): the findings of the review,
  applied to the design records.
- `05` `ravel-child-plan-k4` (planning): the leaves of the code, book and record
  changes.
- `06` `ravel-child-plan-k15` (review-planning): an adversarial read of the
  plan below, before any leaf of code runs.
- `07` `ravel-floor-k6` (impl): the Ravel 1.1.0 floor in the resolver and the
  release inputs.
- `08` `ravel-job-k7` (impl): the shared function and its book coverage, beside
  `keyed-launch`.
- `09` `grove-run-job-k8` (impl): `grove run` on the detached shape.
- `10` `lifecycle-job-k9` (impl): the loop driver and the driver lease on the
  foreground shape.
- `11` `interrupted-errors-k10` (impl): every error that stops the loop keeps a
  caught signal.
- `12` `keyed-launch-removal-k11` (impl): the crate, its tasks, tests, book and
  structure spec.
- `13` `design-records-k12`, `14` `user-records-k13` and `15` `book-specs-k14`
  (impl): the records that state the reversed decision.

Each leaf from `08` to `12` carries the book edits for the code that it
changes, because `book-check` gates the quotes. The order is expand (`08`),
migrate (`09`, `10`, `11`) and contract (`12`). The records follow the code.

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
- **Each edited record needs a conversion first.** The `technical-communication`
  skill converts each record as its own change before the edit. A large document
  converts only the sections that the edit touches. The design converted the two
  specs and the two ADRs that it reworked, and the README of the caller views.
  These pass the checker now. No other record on the list above passes it yet.
- **The project Taskfile.** `Taskfile.yml` is the entry point for repeatable
  work. Its `test:keyed-launch` task goes with the crate, and so does the use of
  that task in `ravel:activation`.
