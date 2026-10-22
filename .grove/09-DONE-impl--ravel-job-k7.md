# ravel-job-k7

## Goal

Add the one shared function that starts `ravel` as an ordinary child. Add it
beside `keyed-launch` so that nothing breaks. No production caller uses it yet.
This is the expand step of an expand, migrate and contract sequence. The two
callers move in the next two leaves.

## Context

- The root brief: *Design*, section *The shared function*, and the table of
  *New homes*. The caller contract
  `docs/specs/harness-selection-and-execution.md` states the mechanics under
  *Jobs and authority* and *Lifecycle interpretation*. The decisions of
  `ravel-child-k2` give the reason for each choice.
- The current runner is `crates/keyed-launch/src/run.rs`. Move its working
  behaviour, and not its group and escalation parts. The terminal rules move
  unchanged: the gate, the handover in both processes, the re-lend at each
  poll, the saved modes and the three reclaim rules.
- The book of `grove-loop` has the whole of `crates/grove-loop/src/**/*.rs` as
  its corpus. A new module needs book coverage in the same change.
  `docs/specs/grove-loop-book-structure.md` governs a new chapter or slice.
  `book-specs-k14` edits that spec, so record the new chapter in the running
  log.

## Done when

<!-- tc: depart synonym-rotation because Launch directory is a defined glossary term -->
- A new module of `grove-loop` owns the function. It takes the command, the
  working directory, the held stdout file, an observer of the notices and a
  shape. The foreground shape carries the launch directory. The detached shape
  carries the transcript file.
- It returns the end of the job: the wait status, the interruption and the
  elapsed time. The end has no group field.
- A failed spawn or wait returns an error that carries the interruption. A
  failed spawn sends no notice. A failed wait sends no Reaped and no KILL.
- The function installs the handlers at its first call. Its latch catches INT,
  TERM and HUP, except a signal that is ignored. It never clears the latch
  before the spawn. At each poll it passes the foreground on, sends each
  latched signal to `ravel`, and checks for an exit without blocking.
- The child gets the default disposition of the seven job-control signals.
- The module owns the one scrub list of ten names, a helper that scrubs a
  `Command`, `LaunchEvent`, `take_interrupt` and `reraise`. Only the foreground
  shape grants `GROVE_LAUNCH_DIR`.
- `Argv` is a value type in `session_command` with the program, the arguments
  and `words()`. The builder returns it. The two callers still start `ravel`
  through `keyed-launch`, and each converts the value at its one call site.
  The migration leaves delete these conversions.
- The existing `grove-loop` tests `session_command.rs` and
  `session_result_installed.rs` run through the new function, with one case per
  shape. No test uses a stand-in child to check job control.
- The book of `grove-loop` covers the new module and every changed line.
- The `Argv` conversion at the call site in `crates/grove/src/standalone.rs`
  changes source that the `overview` book quotes. The fragments are
  `standalone-launch-context` and `standalone-supervision` of chapter 06. The
  indexes and any moved line counts change with them. Chapter 06 matches that
  change in this leaf, so `book-check` passes at this boundary. The conversion
  of chapter 06 may already exist from `ravel-floor-k6`.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set:
  `ravel:commands`, `ravel:result` and `test:grove-loop`.

## Notes

- Convert each edited record as its own change before the edit. The
  documents of this leaf are the book chapters that it touches.
  The caller contract spec already passes the checker.
- The `loop_driver.rs` call site converts `Argv` too. The book of `grove-loop`
  quotes it, and this leaf covers it.
- Decision 7 of `docs/specs/module-decomposition.md` records the shipped
  interface. That edit waits for `design-records-k12`, after the callers move.
- Names are the implementation's to choose. Record the chosen names in the
  running log so the next two leaves use them.
- The function holds the signal and terminal contracts, and every later leaf
  relies on it. Cut a `review-impl` leaf at the end of this leaf.

## Decisions (running log)

- **Names.** The module is `grove_loop::ravel_job`, public so that the
  integration tests and `crates/grove` reach it. The function is
  `ravel_job::launch(command: &Argv, cwd: &Path, stdout: File, observer: &mut
  dyn FnMut(LaunchEvent), shape: Shape<'_>) -> Result<JobEnd, JobError>`.
  `Shape` is `Foreground { launch_dir: &Path }` or `Detached { transcript: File }`.
  `JobEnd` holds `status`, `interrupted: Option<i32>` and `elapsed`. `JobError`
  has `interrupted()`. The module also owns `LaunchEvent { Started, Reaped }`,
  `take_interrupt`, `reraise`, `CONTROL_ENV` (the one list of ten names) and
  `scrub(&mut Command)`. The next two leaves use these names.
- **`Argv`.** It is `session_command::Argv` with `new`, `program`, `args` and
  `words`. It has no `arg0` field, because no caller of Grove uses one. The two
  callers convert it to `keyed_launch::Argv` with one line at the call site, in
  `launch_session` and in `standalone::execute`. The contract leaves delete those
  lines.
- **Wait.** The function polls with `Child::try_wait`, which reaps at the poll
  that sees the exit and never reports a stop. The runner needed `waitid` with
  `WNOWAIT` only to kill the group before the reap. Grove no longer kills a
  group, so the plain call is enough.
- **Entry.** One `OnceLock<bool>` runs at the first call. It installs the INT,
  TERM and HUP handlers over every disposition that is not an ignore, and it
  repairs an inherited ignored SIGCHLD. The bool says whether SIGCHLD was ignored,
  so the child gets that disposition back. Both effects share one lock so that
  neither can run twice.
- **Early errors.** A non-regular stdout or transcript, and a failed descriptor
  scan, return before the spawn. They pass through the same constructor as a
  failed spawn, so they take the latch into the error too.
- **Tests.** `tests/ravel_job.rs` checks the notices, the environment grant, the
  errors and the latch with no terminal. It uses `/usr/bin/env` and
  `/usr/bin/true` only for the environment and the order of notices. Job control
  is checked only by the installed-Ravel cases of `session_command.rs` and
  `session_result_installed.rs`, one case per shape. The latch test was seen to
  fail when `launch` stopped taking the latch into the error.
- **Book.** No new chapter. `ravel_job.rs` is one more block of chapter 19, beside
  `launch_directory`, `session_result` and `session_command`, under the slice
  `four-things-a-runner-cannot-choose`. `book-specs-k14` must record that block in
  `docs/specs/grove-loop-book-structure.md`, where chapter 19 still says that all
  spawning is `keyed-launch`'s. No new slice is cut.
- **Review.** `ravel-job-k17` (`review-impl`) is inserted ahead of
  `grove-run-job-k8`, so the read comes before any caller moves. The root brief's
  *Decomposition* list carries the new numbers. This session has no provider
  evidence, so the review has no Creator line.
- **Result.** `bash scripts/check.sh` passes all 20 checks. `task ravel:commands`,
  `task ravel:result` and `task test:grove-loop` pass with the installed Ravel
  1.1.0. A pseudo-terminal run of the lifecycle case engaged the handover and
  passed. The `grove-loop` book now has 18 roots and 15,244 lines, and `overview`
  chapter 06 carries the `Argv` bridge.
