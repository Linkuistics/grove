# ravel-book-inside-k308

## Goal

Draft chapters 10 and 11 of the Ravel book for 2.0.0: develop Ravel and
deliver Ravel, including the release channels and the promotion.

## Context

- The paused charter, read only: `Ravel.gh-issue-2/.grove/20-k34/_rhai-book-inside.md`
  and its second draft leaf `02-draft--rhai-book-inside-k36.md`.
- Ravel `docs/book/contract.md` and `docs/documentation-lessons.md`.
- Ravel `docs/specs/release-channels.md` and the commands that
  ravel-release-channel-k305 and ravel-release-promote-k306 delivered.
- Ravel's design evidence, section "Packaging record of a default source",
  from ravel-packaging-guidance-k301. Keep that guidance and the keg layout.

## Done when

- Chapter 10 names the current toolchain, source tree and tasks. It names no
  Bun worker, no worker build, no SDK and no TypeScript typecheck, except in a
  statement of what was removed. Bun stays only where the book tools use it.
- Chapter 11 describes the 2.0.0 delivery: one executable, the installation
  record, the archives, the formula channels `ravel@2` and `ravel`,
  `CHANNEL`, `release:promote`, cleanup suppression and the retained old
  keg.
- Section "Recover a partial release without another bump" gives the exact
  recovery commands with the selected channel. It covers each row of the
  release-channel recovery, the promotion boundary included.
- The book has one complete example that shows the refusal of a lone
  `policy.ts` as `config_format_retired`.
- `task docs:examples`, `task docs:excerpts` and `task docs:ste` pass. Figure
  needs for the art step are listed.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.

## Decisions (running log)

- 2026-11-12: Structure brief present. Ravel `docs/book/contract.md`
  section 1 states the reader, the section plan (chapter 10: prepare, build,
  seams, `task check`, add a test, evidence; chapter 11: versions, install and
  smoke, archives, formula, release tasks, targets) and the emphasis.
- 2026-11-12: One session drafts both chapters. The paused charter allows a
  split by chapter. The facts of both chapters came from one read of the
  Taskfile and the release scripts, so no split.
- 2026-11-12: Chapter 10 rewritten. Removed: worker build, `typecheck`,
  `worker`, `deps`, `probes`, `runtime:integrity`, `test:hostile`, the SDK and
  `libexec` rows. Added: the Rhai seams (`rhai_*`, `rhai_llm_calls`,
  `skills_*`), `rhai:upgrade-proof`, `notices`, the harness smoke tasks, the
  current `check.sh` stages and a fifth sample test with a fake model.
- 2026-11-12: Chapter 10 trial at working-copy commit `a1b9dda0e8c4`
  (content of `47c6422effa0`): `task setup` 3 s, `task check` 2 min 49 s with
  713 passed and 48 skipped, the five `book_` tests passed in a scratch edit
  that `jj restore` removed. The control that expected `deep-route` failed.
  `task install PREFIX=target/install-trial` 12 s and `task smoke` 7 s passed.
- 2026-11-12: The lone `policy.ts` complete example goes in chapter 11, under
  the promotion, because promotion moves every default-channel user to Rhai
  and the changelog must name the refusal.
- 2026-11-12: Chapter 3's "No release carries the book yet" is false: the tag
  `v1.1.0` carries the book and Pages serves `v1.1.0/`. Chapter 11 states the
  same limit, and chapter 3 links to it, so the draft corrects both.
- 2026-11-12: Chapter 11 rewritten around the two-step 2.0.0 delivery. New
  sections: "Choose a formula channel", "Package a skill source for the
  default directory", "Keep each old keg for its active readers", "Check the
  installation and record the evidence", "Promote a versioned release" with
  "See what promotion changes for an owner", and "Enable Pages for the book"
  (was "Set up Pages for the first release"). The recovery table has ten
  rows: release evidence and a stopped promotion are new. Every resume
  command names `--channel CHANNEL`.
- 2026-11-12: `docs/book/ste.md` refuses every form of "publish". Chapter 11
  says "deliver" for a channel or a release, and "put in the tap" for a
  formula. Script names keep their spelling.
- 2026-11-12: The complete example of the lone `policy.ts` passes
  `task docs:examples`. A copy that claimed status 0 failed. Ravel checks
  the entry before skill preparation (`choice.rs` lines 65 and 69), so an
  installation record cannot change its output (lesson L75).
- 2026-11-12: Release trials: doctor 3 s, `release:test` 5 s,
  `release:smoke` 53 s, `release:formula` 8 s passed. The first
  `release:lifecycle:test` failed at the mutant "install keeps cleanup",
  because the trial script exported `HOMEBREW_NO_INSTALL_CLEANUP=1`. The
  second run with `env -u` passed, 48 of 48 mutants killed. Chapter 11
  tells the reader to run the suite without the variable. The fixture gap
  is noted in ravel-main-landing-k312. The input digest did not change.
- 2026-11-12: Chapter 1 had the same stale limit as chapter 3 ("unchecked
  until the first release that contains the book"). The served
  `v1.1.0/03-a-complete-session.html` shows `ravel 1.1.0`, and the root page
  lists `v1.1.0`. Chapter 1 now says so. The contract's matching sentence
  went to proof under Handed forward.
- 2026-11-12: Lessons L80 (the environment of a check is an input) and L81
  (check each "not yet" statement) added to Ravel
  `docs/documentation-lessons.md`.
- 2026-11-12: Trial record `docs/evidence/develop-ravel-k308/` with its logs.
  Whole-book validation green: build, 18 figures, 27 examples, 22 excerpts,
  strict STE 32 files, breaks 11 files.
- 2026-11-12: The one in-session reviewer checks chapter 11's channel,
  promotion and recovery claims against the scripts.
- 2026-11-12: Review of chapter 11, 13 findings, all valid and actionable;
  none noise. Applied: the rows now end with "finish the later steps"
  (installation, evidence, `live`) and a fragment for them; resumes run
  after `jj new RELEASE_COMMIT`; restore a changed keg before a rerun;
  evidence needs the smoked archives in `target/dist`; rows for a promotion
  that stops before its tap write and for a new receipt; both formulae are
  rendered and tested; evidence reads local files too; the replacements are
  in the workspace `release.toml`; the `unversioned` row; `CHANNEL` only on
  the version-cut tasks; promotion takes no channel; preparation does not
  check the prior installation, evidence or `live`; the saved change after
  a rerun. The heading "Set up Pages for the first release" is restored,
  because three scripts print its anchor. The specification's matching
  overclaim went to ravel-main-landing-k312. Final book validation green
  after these edits.
