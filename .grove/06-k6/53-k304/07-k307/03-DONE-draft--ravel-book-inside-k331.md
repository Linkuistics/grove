# ravel-book-inside-k331

## Goal

Draft chapter 9 of the Ravel book for 2.0.0, “Jobs, signals and the
terminal”. Check the claim that it changes least, and change what 2.0.0
changed.

## Context

- Read the node brief `_ravel-book-inside.md` and its pointers. The structure
  brief is Ravel `docs/book/contract.md`, sections 1 and 2.
- The paused charter says that the runner is unchanged, so chapter 9 changes
  least. Test that claim. Compare `crates/keyed-launch/` at the revision of
  the last thorough revision of chapter 9 with the current code. Then read
  each statement of chapter 9 that names the worker, the CLI's signal repair
  or the configuration-output reader threads against the Rhai code.
- The paused leaf `rhai-book-inside-k35` asked for two subjects. One is the
  ordered stop with a harness run in progress. The other is the precedence of
  a signal over the selection deadline. The harness runs are in
  `crates/ravel/src/rhai_worker/harness.rs`.
- Chapter 7 and chapter 8, as drafted by `ravel-book-inside-k329` and
  `ravel-book-inside-k330`, set the words and the example. Restate them.

## Done when

- Chapter 9 describes 2.0.0. The running log states the check of the
  “changes least” claim and its result.
- `task docs:examples`, `task docs:ste` and `task docs:breaks` pass for
  chapter 9. Its links to chapters 7 and 8 resolve.
- The chapter follows each Read 1 pattern of contract section 2.
- Figure needs for the art step are in the parent node brief's
  `## Handed forward`.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.

## Decisions (running log)

- 2026-11-12: The structure brief is `docs/book/contract.md` section 1. It
  states the reader (“The reader”, “What the reader can do afterwards”), the
  ordered section plan (“The section plan”, chapter 9's row) and the emphasis
  and exclusions.
- 2026-11-12: The claim “chapter 9 changes least, because the runner is
  unchanged” is false for the runner. The baseline is Ravel `b8903f3a8537`
  (2026-10-21), the last copy-edit of the whole book. Later edits to chapter 9
  changed only the subjects that they named. From that baseline to `f6ee31358634`, `crates/keyed-launch/`
  changed in 19 files, with 1225 insertions and 762 deletions. The changes:
  - `rhai-harness-run-k23` added a third launch mode, `run_captured` with
    `CapturedLaunch`, `Captured` and `StopRequest`, and `capture.rs`. A
    captured job runs in a new POSIX session with pipes that a runner thread
    reads. The runner installs no signal handler for it and does not read
    its latch. It polls every 20 milliseconds. A `StopRequest` or an output byte beyond
    the bound sends TERM to the group and the child, then KILL after the kill
    grace. Chapter 9 named none of this.
  - The confinement removal (`dc162664101a`) and the EPERM watch
    (`576811bcb4b0`) changed the runner too. Chapter 9 already describes both.
  - The rest is the edition-2024 reformatting.
  Thus, chapter 9 changes in the sections about the launch modes,
  cancellation, the entry signal state and the guarantees. The process group,
  the escalation stages, the reap order and the terminal rules stay true.
- 2026-11-12: Checked statements of the old chapter against the code:
  - Still true, with the constant that holds each value:
    - the 500 ms poll of the managed process (`POLL_INTERVAL`).
    - the 2 s and 5 s escalation (`run::ESCALATION`), for the managed process
      and each harness run.
    - the 2 s bound after the first KILL (`ERROR_SETTLEMENT`).
    - the two group KILLs 20 ms apart (`SECOND_KILL_PAUSE`).
    - the 1 s group query (`GROUP_CONFIRMATION`).
    - the 10 ms poll after KILL (`KILLED_POLL_INTERVAL`).
  - Incomplete: the selection row of the cancellation table omits the harness
    run, which Ravel stops before the worker. The runner's handlers are
    installed only for the managed launch. During evaluation, the latch of the
    Ravel CLI (`cancellation.rs`, `on_signal`) holds the signal, and
    `serve::Watch::stopped` reads it every 50 ms (`worker::POLL`).
  - Incomplete: the SIGCHLD repair and the entry signal state also apply to
    each harness run. `run_with_output` calls `restore_child_watching` for
    every mode, and `harness::execute` passes `transparent: Some(entry)`.
  - Incomplete: the masked threads are now the `config-stdout` and
    `config-stderr` readers, the `ravel-reader` thread (`Shield::block` in
    `reader.rs`) and the runner's `keyed-launch-pipes` thread. The reader
    thread creates that pipe thread, so it inherits the block.
  - Stale: the guarantee row about detached children of the configuration
    script. In 2.0.0 the worker creates no process. The only processes of the evaluation
    are harness runs, so the limit is a descendant that leaves a run's group.
    The contract's Emphasis still uses the old words. That text is the
    owner's contract, so this leaf does not change it.
  - Stale: the paragraph that gives `run_noninteractive` a supervising child.
    It has no production caller. The nested supervisor that the forwarded
    signal serves is a nested `ravel start-session` in the managed process.
  - The worker stays in the process group of the Ravel CLI. `worker.rs`
    spawns it with no group change.
- 2026-11-12: The two subjects of the paused leaf come from
  `docs/specs/configuration-evaluation.md` “Cancellation and deadlines”, from
  `Host::settle` and `worker::supervise`, and from three trial runs through
  the book checker on build `f6ee31358634`. A fake harness that records TERM
  and stays gave `selection_timeout`, status 124, after about 6.5 s. The same
  fake, with `kill -TERM "$PPID"` in its trap, gave `selection_cancelled`,
  status 143, with `selection_timeout` under “original failure:”. In both
  runs the fake was gone afterwards, the agent never ran and `tmp` was
  empty. The same trial showed `session_cancelled` after the
  `selection_cancelled` report and a `Terminated: 15` line from `/bin/sh`.
- 2026-11-12: The remedy of `selection_timeout` says “an await on work that
  never settles”. That is a TypeScript phrase in Ravel's code, outside this
  book. The chapter does not quote that line.
- 2026-11-12: Chapter 9 now follows `ravel-parts`, the example of chapters 7
  and 8. It has three complete examples, each with one changed file:
  `ravel-parts-escalation` (the agent stays after TERM, status 0),
  `ravel-parts-deadline` (the harness run stays at the deadline, status 124)
  and its one-line variant (the run's trap sends TERM to the Ravel CLI,
  status 143). New sections: “Stop a harness run”, “Stop the evaluation at
  its deadline” and “Let a signal win over the deadline”. “Separate the three
  launch modes” replaces the two-mode section. The four headings with
  inbound links stay unchanged.
- 2026-11-12: The leaf's one in-session reviewer read the new and changed
  statements against the source. All 11 findings were valid and actionable,
  and each fix follows the cited source lines. Two statements gave a wrong
  reason or actor: the second TERM leaves the run alone because the runner
  never reads the CLI latch for a captured job, and a stop abandons a read
  in its own step. The other nine were incomplete:
  - three more causes of an evaluation stop: the script's output limit,
    worker death and a protocol break.
  - `timed_out` only for a `timeoutMs` below the time that remains.
  - the 8-second `SETTLE` ceiling and an unconfirmed reap after it.
  - no TERM to the worker after `harness_cleanup_failed`.
  - the runner's second stop flag for the output limit.
  - entry-ignored signals never reach the latch.
  - closed descriptors above 2 in a captured launch.
  - result-channel removal failures.
  - the exact terminal relationship after `setsid`.
  No second review was needed, because each fix is a correction against a
  named source line, and the examples check covers the claimed runs.
- 2026-11-12: Checks on the final chapter 9, digest `71430792e442` before and
  after. `task docs:examples` passed all 26 examples of the book, and chapter
  9's 3 examples passed four more times. A control copy that claimed status
  124 for the signal example failed. `task docs:ste --strict` and
  `task docs:breaks --strict` found 0 problems. `task docs:excerpts` passed
  22 excerpts. A link check of every page found no broken file or anchor,
  and a planted bad anchor in chapter 9 failed it. `task docs:book:build`
  passed, and the chapter 9 page has 12 section headings with 12 ids.
  `task test:runner` passed 112 tests, with 17 skipped, on aarch64 macOS
  27.0.1.
- 2026-11-12: Lessons L78 and L79 are in `docs/documentation-lessons.md`. The
  figure needs and the contract wording question are in the release node
  brief's `## Handed forward`. The copy-edit leaf `ravel-book-inside-k309`
  already stands, so this leaf cuts no copy-edit leaf.
- 2026-11-12: Ravel commit `47c6422effa0` on `skill-directory-delivery` holds
  chapter 9 and the lessons. The bookmark is local, as for the earlier leaves.
  This leaf is the last child of `ravel-book-inside-k307`, and that node's
  `Done when` holds: chapters 7 to 9 describe 2.0.0 and pass their checks.
