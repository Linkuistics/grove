# ravel-two-release-k304 — brief

## Goal

Publish Ravel 2.0.0 once, through the keg-only `ravel@2` only. This is tap
state B of the coordinated release design. The released line carries the
channel and promotion tooling. Its book describes 2.0.0 in all eleven
chapters after the editorial pipeline.

## Done when

- Ravel's release tasks carry one explicit `CHANNEL` through preparation,
  rendering, publication, installation, verification and every resume mode.
  `release:promote` exists. The lifecycle fixtures of both pass, and neither
  task has published anything.
- Chapters 7 to 11 passed draft, copy-edit, art and proof. They describe
  2.0.0, including the channels and the promotion. The proof read the whole
  book. Each book check passes.
- The 2.0.0 changelog names the configuration-format change, the
  `config_format_retired` refusal and chapter 4.
- The line is on Ravel `main`, and `main` is pushed. Every Ravel check passed
  on that commit.
- The cut ran once. The tag `v2.0.0`, the three archives, the versioned book
  and `Formula/ravel@2.rb` are public. The installed `ravel@2` prefix and the
  live book pass their verification. Unversioned `ravel` stays 1.1.0.
- `/opt/homebrew/Cellar/ravel/1.1.0` is complete, and its `ravel` still
  reports 1.1.0 and refuses `end-session` outside a session.
- The receipts are in Ravel's release evidence and in Grove's
  `docs/coordinated-release-evidence.md`. They name the channel, the tag's
  commit, each asset's SHA-256, the book commit, the tap commit and the
  installed result.

## Decomposition

1. `ravel-release-channel`: thread `CHANNEL` through the existing release
   tasks.
2. `ravel-release-promote`: the separate promotion task. Finish runs it after
   the Grove release.
3. `ravel-book-inside`, five steps: draft of chapters 7 to 9, draft of
   chapters 10 and 11, copy-edit, art and proof. The tooling comes first, so
   chapter 11 describes tasks that exist. The draft of chapters 7 to 9 is the
   node `ravel-book-inside-k307`, with one draft for each chapter.
4. `ravel-main-landing`: rebase, full checks, changelog, specification status
   and the landing on `main`.
5. `ravel-two-publication`: the one cut and its verification.

A producer of release tooling that cuts a `review-impl` inserts it before the
first draft. Its integration then runs before the book describes the tasks.

## Pointers

- Ravel `docs/specs/release-channels.md` owns the channel and promotion
  mechanics. Grove `docs/specs/coordinated-release.md` owns the order between
  the products and the recovery table.
- Ravel decision records: `a-caller-transition-publishes-a-versioned-formula-first`
  and `configuration-script-uses-rhai`.
- Ravel book chapter 11, section "Recover a partial release without another
  bump", owns the exact recovery commands.
- The book's structure brief and prose contract: Ravel `docs/book/contract.md`.
  Its lessons: Ravel `docs/documentation-lessons.md`.
- The paused charter of this book work, read only:
  `Ravel.gh-issue-2/.grove/20-k34/_rhai-book-inside.md`. The latest reader
  findings F17 and F18 are in `Ravel.gh-issue-2/.grove/19-requirements--rhai-book-use-k74.md`.

## Notes

All work before the cut happens in the workspace
`~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`, on the
bookmark `skill-directory-delivery`. At planning its tip was `576811bcb4b0`,
above Ravel `main` `24a545236185`. The tip includes
ravel-packaging-guidance-k301 and ravel-stop-eperm-k302. The cut must include
both. The release task itself runs from Ravel's default colocated workspace,
`~/Development/AgenticOrchestration/Ravel`.

Do not edit the tree of `Ravel.gh-issue-2`, and do not run its leaves. Its
`release-ravel-2-0-0-k37` would run `task release:major` a second time. The
paused-grove handoff leaf of the parent node gives that workstream its
brief-update text.

The human chose this book scope on 2026-11-11. The editorial pipeline runs
here over chapters 7 to 11. The owner's Read 1 and Read 2 stay with the paused
workstream. This node marks none of that workstream's goals done.

The tap clone is `/Users/antony/Development/Delivery/homebrew-taps`. Supply
`RAVEL_TAP_DIR` and `GROVE_TAP_DIR`, because the scripts' default path is
absent. Set `HOMEBREW_NO_INSTALL_CLEANUP=1` on each Homebrew mutation. Never
run `brew cleanup`, `brew autoremove`, an uninstall or a reinstall of `ravel`.
The running driver ends through `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`.

A change to Ravel's Rust source changes the build identity. Staging accepted
the build `58d479ed…`. That difference is expected. The leaf
`published-source-acceptance` resolves the sources again with the published
archives. A change to a skill source is out of scope here.

## Handed forward

- **art**, chapter 7, “Find each part in the source”: the embedded
  `source-components` figure omits two parts that the chapter now teaches.
  They are skill preparation, `crates/ravel/src/skills/`, before the worker
  starts, and the reader thread with the runtime of the host bridge. Its
  description “Checks configuration script and its response” lacks an article.
- **art**, chapter 7, “Watch the parts in one session”: the contract asks for
  a process-and-authority figure in chapter 7. The chapter links to chapter
  2's `process-authority` figure, which omits harness runs. In the `ravel-parts`
  example, the Ravel CLI is the parent of the harness run `fake-advisor` and
  of the managed process `fake-agent`. Each has its own result channel. A figure of those processes would serve the chapter.
- **art**, chapter 7, “The host bridge carries each request to the Ravel CLI”:
  the chapter reuses chapter 4's `evaluation-harness` figure, whose harness is
  `claude`. The text says that the figure is general. Decide whether the
  chapter needs its own view with the reader thread.
- **art**, chapter 8, “Trace the example”: the `session-code-path` figure
  still draws chapter 3's session and the worker of Ravel 1.x. Its rows
  “Change to /”, “Register embedded modules”, “evaluate: script path and
  request”, “Script description”, “judge: check description” and “select:
  request command” name steps and frames that 2.0.0 does not have. “Print
  prompt to caller stderr” is the action of chapter 3's fake. The chapter now
  follows `ravel-parts`. The figure needs these steps, in this order:
  1. `hello` and its identity check.
  2. The reader thread's read of `config.rhai`.
  3. The `evaluate` frame with the source.
  4. The `host` frame `run_harness`.
  5. The harness run `fake-advisor`, with its own result channel and
     `RAVEL_CONFIG_RUN=1`.
  6. The `reply` frame, then the `selection` frame and `reply_judge`.
  7. The worker's own exit and its reap.

  The text routing notice comes after the channel allocation and before
  `try_block_and_check`. The key line “Example without selection context or a
  terminal” stays true. The alt text is general and stays true.
- **art**, chapter 8, “Follow the harness run through the host bridge”: an
  8-row table follows the harness call across the worker, the runtime thread,
  the reader thread and the runner. Chapter 7's `evaluation-harness` entry
  above asks the same question for chapter 7. One sequence with the runtime
  thread and the reader thread as separate lifelines could serve both chapters.
- **art**, chapter 8, “Cancel during selection”: `selection-cancellation`
  agrees with the new text and the code. Harness cleanup comes before worker
  teardown, the worker receives TERM, and no managed process exists. The figure does
  not show that the report carries `worker_failed` under “original failure:”,
  and that `settle` then returns `session_cancelled`. The text states both.
  Decide whether the figure needs them.
- **art**, chapter 8, “Cancel during supervision”: `supervision-cancellation`
  shows chapter 5's `candidate` and agrees with the code. The chapter's example
  shows `session_result_invalid` before `session_cancelled`. A note could show
  that `result_after` gives the first refusal.
- **art**, chapter 8, “Choose the final outcome”: `outcome-precedence` omits
  the exception that a cancelled skill resolution keeps
  `skill_resolution_cancelled`. The table beside the figure states it.
- **proof**, chapter 3, “Run the first session”: a Homebrew installation
  writes `share/ravel/installation.json`. With no `skills.sources`, an
  installed Ravel then reads the default skill directory, also under `env -i`
  with an empty `HOME`. If that directory holds a source, the fake prints the
  prepared prompt, with its preamble, and the claimed standard error differs.
  The checkout build has no record, so `task docs:examples` cannot see this.
  Chapter 7 states the case for its own example. Lesson L75 records the check.
- **art**, chapter 9, “Stop the evaluation at its deadline” and “Let a signal
  win over the deadline”: the contract asks chapter 9 for a cancellation
  sequence for each phase. The chapter embeds neither sequence. The new
  text follows `ravel-parts-deadline`. A timing or sequence figure of that
  stop would serve both sections. Its lifelines are the Ravel CLI with its
  latch, the runner, the harness run `fake-advisor` with its group, and the
  Rhai worker. Its events, in this order:
  1. The 1-second deadline expires, and `Host::settle` sets the
     `StopRequest`.
  2. The runner sends TERM to the run's group and the run. The run's trap
     writes its note. In the second example, the trap also sends TERM to the
     Ravel CLI, and the latch keeps it.
  3. After the 5-second kill grace, the runner sends KILL to the group and the
     run. Then come the two group KILLs, the reap and the group query.
  4. TERM to the worker, KILL after one second if needed, and its reap.
  5. Removal of the private directories, then the final latch read:
     `selection_timeout`, 124, without the signal, and `selection_cancelled`,
     143, with it.
  No managed process starts. The figure can reuse the lifelines of
  `selection-cancellation`, which chapter 8 embeds.
- **art**, chapter 9, “Follow the escalation after a result”:
  `escalation-timing` still agrees with the text for the managed process. Its
  case A draws a managed child that ignores the TERM signal. The text now says
  that each harness run has the same stages, with checks every 20 milliseconds
  instead of 500 milliseconds. The example `ravel-parts-escalation` records TERM in a trap and
  stays, which is the figure's case A. Decide whether the figure needs a note
  for harness runs.
- **art**, chapter 9, “Lend and recover the terminal”:
  `terminal-lending-recovery` agrees with the text. The text now adds that a
  harness run never receives the terminal. The figure needs no change for
  that.
- **proof**, chapter 9, “Know the guarantees and their limits”: the chapter
  states the old limit “detached configuration script children” in 2.0.0
  terms. The worker starts no process, so the limit is a process that leaves
  a harness run's group. Contract section 1, “Emphasis”, still says
  “detached configuration script children”. The contract is the owner's
  text, so the draft did not change it. Check that the two agree in meaning,
  or raise the wording with the owner.
- **art**, chapter 11, “Cut and deliver a release”: `release-delivery`
  draws the 1.x order. Its alt text and the paragraph above it say “eight”
  recovery exits. The task now has a channel on the archive build, the
  publication and the installation, and two new steps after the tap update:
  the installation with cleanup suppressed, then
  `scripts/release-evidence.sh`. The recovery table has eleven rows, with rows
  for the release evidence and for a promotion that stopped before or after
  its tap write. The figure, its alt
  text, `figures.md` and the paragraph “The eight grey exits” change
  together.
- **art**, chapter 11, “Choose a formula channel” and “Promote a versioned
  release”: the text uses two tables for the tap state before the cut, after
  the cut and after promotion. A figure of the formula files, the kegs and
  the shared `bin/ravel` in those three states could serve both sections.
  The contract's figure needs ask chapter 11 for a C4 component and UML
  deployment view. Chapter 11 embeds none, and `installed-files` omits
  `share/ravel/installation.json`, which chapter 6's table lists.
- **art**, chapter 11, “Package a skill source for the default directory”:
  a deployment view of the payload `libexec/skill-sources/ID/`, the source
  alias `share/ravel/skills/ID` and the link in the shared prefix could
  serve. The editable design view in `docs/design/skill-discovery/`, anchor
  `#diagram-homebrew-directory`, draws the same layout.
- **proof**, contract section 1, “Omissions and unknowns”: “The site is
  unchecked until the first release that carries the book.” Ravel 1.1.0
  carried the book, and the site serves `v1.1.0/`. Chapters 1, 3 and 11 now
  say so. The contract is the owner's text, so the draft did not change it.
  Raise the wording with the owner. Lesson L81 records the case.
