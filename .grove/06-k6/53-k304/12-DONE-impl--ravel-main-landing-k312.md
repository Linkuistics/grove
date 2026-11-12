# ravel-main-landing-k312

## Goal

Land the release line on Ravel `main` with every check green, ready for one
2.0.0 cut.

## Context

- Grove `docs/specs/coordinated-release.md`, step 3 of "Publication ownership
  and order".
- Ravel `CHANGELOG.md`, section `Unreleased`, and the paused leaf
  `Ravel.gh-issue-2/.grove/21-impl--release-ravel-2-0-0-k37.md` for the
  changelog content it expected (read only).
- Ravel specifications whose status still describes unreleased work, such as
  `docs/specs/release-channels.md` and `docs/specs/skill-discovery.md`.

## Done when

- A fresh fetch shows Ravel `main` on the remote. The line is rebased onto it.
  Each conflict is resolved and recorded.
- The `Unreleased` changelog opens with the 2.0.0 summary. It names the
  configuration-format change, the `config_format_retired` refusal with exit
  3 and book chapter 4. It names the new settings keys, the host functions,
  schema 4, the removed Bun worker and the withdrawal of confinement.
- Ravel's specifications, `CONTEXT.md` and `README.md` describe the state
  that the release publishes. Historical records stay history.
- From the default workspace, `task check`, `task test`, `task release:smoke`,
  `task release:lifecycle:test`, `task release:formula` and every book check
  pass on the rebased commit.
- `main` points to that commit and is pushed through jj.
- `task release:doctor` passes with `RAVEL_TAP_DIR` set to the tap clone. No
  version is cut.

## Notes

If `main` moved since 2026-11-11, reinspect manifests, tags, releases, book and
tap before the rebase. If any 2.0.0 cut or public tag exists, stop. Reconcile
it under the recovery table, and do not cut again.

`task release:lifecycle:test` fails at the mutant “install keeps cleanup”
when the shell exports `HOMEBREW_NO_INSTALL_CLEANUP=1`. The fake `brew` of
`scripts/release-channel.test.sh` logs that variable from the inherited
environment, so the mutant of `scripts/release-install.sh` passes every
control. The draft ravel-book-inside-k308 saw it fail with the variable and
pass with `env -u` and no other change, on 2026-11-12. The operator notes of
this node ask for the variable on Homebrew commands. Run the suite without
it. A scrub of the variable inside the test's fixture would close the gap.
Chapter 11 of the book now tells the reader to run the suite without it.

The draft's review of chapter 11 also found two statements of the
specification `docs/specs/release-channels.md`, "Release interface", that
the scripts do not hold. "Every script that writes or reads a formula takes
`--channel`" omits `release-promote.sh`, which takes none, and the optional
channel of `release-formula.sh`. The chapter now states the exceptions.
Make the specification agree when it moves to the released state.

## Decisions (running log)

- 2026-11-12: A fresh `jj git fetch` in `Ravel.skill-directory-delivery`
  changed nothing. `main`, `main@git` and `main@origin` are all
  `24a545236185`, the commit of planning. The remote holds the tags v0.1.0,
  v1.0.0 and v1.1.0 only. GitHub lists releases up to v1.1.0. The tap holds
  `ravel.rb` and `ravel-lite.rb`, and no `ravel@2.rb`. Thus no 2.0.0 cut or
  public tag exists, and `main` did not move.
- 2026-11-12: The line `skill-directory-delivery` (`424af53d3f0c`) descends
  from `main`. A rebase onto `main` is therefore a no-op, and no conflict
  occurs. The landing moves `main` forward to the line's tip.
- 2026-11-12: `release-formula.sh` takes its channel as an optional second
  positional argument, not as `--channel`. Without it, the script keeps both
  formulae. `release-promote.sh` takes no channel. It reads `ravel@2` and
  writes both formulae. The specification states these exceptions.
- 2026-11-12: The `Unreleased` notes open with a two-paragraph 2.0.0
  summary. New entries describe the Rhai format change with
  `config_format_retired`, the settings keys `configEnv`, `harnesses` and
  `budgets`, the twelve host functions and the release channels. The
  existing entries keep schema 4, the Bun removal, confinement and skill
  delivery. The published sections stay as published, under a `tc: off`
  marker placed after `## v1.1.0`, so it never enters the 2.0.0 notes.
- 2026-11-12: `skill-discovery.md`, "Status and ownership", now states that
  2.0.0 implements the whole contract. It points to Grove's
  `docs/common-loading-acceptance-record.md` for the actual harness trials,
  which ran staged builds before the cut. `release-channels.md` lists the
  three scripts that take no `--channel` argument. `README.md` names
  `ravel skills resolve`, the keg-only `ravel@2` and the published book.
  `CONTEXT.md`, `configuration-evaluation.md`, `session-execution.md` and
  `result-job-completion.md` already describe 2.0.0, so they stay unchanged.
  The design evidence and the decision records stay as history.
- 2026-11-12: Ravel commit `935f0dbc8f7a` holds these changes, on the
  bookmark `skill-directory-delivery`.
- 2026-11-12: Moving the default workspace from `b93bb927` to `935f0dbc`
  exposed `crates/ravel/worker/node_modules`, 1,040 files that the retired
  worker's ignore rule once hid. jj snapshotted them as additions. The
  session scratchpad now keeps them, and the working copy is empty again.
- 2026-11-12: The checks run without `RAVEL_EXIT_FILE`, `GROVE_LAUNCH_DIR`,
  `GROVE_RELEASE_RUNTIME_READ` and `HOMEBREW_NO_INSTALL_CLEANUP`, so no test
  inherits this session's authority. The book checks are those of the proof
  leaf, plus `docs:breaks`, each `docs:*:test`, `docs:examples:mutants` and
  both tool integrity checks.
- 2026-11-12: The first run on `935f0dbc` passed 19 of 20 steps. The strict
  `docs:ste` refused one README word: the book's word list admits no
  "publishes" for a book. The README now says "puts its book on GitHub
  Pages". The amended commit is `12df84bebf4a`, and the whole set runs again
  on it.
- 2026-11-12: The second run on `12df84bebf4a` passed all 20 steps in the
  default workspace. The commit and the empty working copy were the same
  before and after the run. `release:smoke` passed for
  aarch64-apple-darwin, aarch64-unknown-linux-gnu and
  x86_64-unknown-linux-gnu with each floor control. The archives report
  1.1.0, because no version is cut.
- 2026-11-12: `main` moved to `12df84bebf4a`, and `jj git push` pushed it
  with `skill-directory-delivery`. A fresh fetch and `git ls-remote` show
  `refs/heads/main` at that commit.
- 2026-11-12: `task release:doctor` with
  `RAVEL_TAP_DIR=/Users/antony/Development/Delivery/homebrew-taps` reports
  "all prerequisites met". The remote holds no v2 tag, and the tap clone has
  no changes. No version was cut.
