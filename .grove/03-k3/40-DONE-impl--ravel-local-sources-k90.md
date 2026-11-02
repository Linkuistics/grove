# ravel-local-sources-k90

## Goal

Make the Ravel feature build match the local-source contract of
ravel-local-sources-k78. Replace the temporary Git refusal with the permanent
`type` refusal, and remove the remaining Git code, fixtures and documentation.

## Context

- Work in the workspace `Ravel.skill-directory-delivery`. The unpushed bookmark
  `skill-directory-delivery` is at `7a894c782037`, the k78 narrowing. Ravel
  `main` was `24a545236185` on 2026-11-02.
- The section "Implementation handoff" of Ravel's
  `docs/skill-discovery-design-evidence.md` lists the nine changes. It is the
  work list of this leaf.
- The specification `docs/specs/skill-discovery.md` at `7a894c78` states the
  refusal and the acceptance case "Local sources only".
- The section "Local stage record" of the same evidence document holds the
  build inputs and the installed-pair smoke of the local stage.

## Done when

- A `type` value other than `local` refuses as `settings_invalid`, stage
  `cli`, exit 2, at `skills.sources[N].type`, with the checkout remedy. The
  check comes before the other fields of the record, at both public
  boundaries, with no cache object and no worker.
- No Git record parser, Git origin, Git branch, `refuse_git`,
  `skill_git_unsupported` or pinned-stage note remains in code, tests, the
  book or `CHANGELOG.md`.
- The "Local sources only" fixtures pass at `skills resolve` and at
  `start-session`. A spy `git` first on `PATH` sees no call. The public tamper
  test checks the remedy of `skill_cache_corrupt`.
- The Ravel tasks and book tasks that the change touches pass. The macOS
  installed-pair smoke passes against the Homebrew 1.1.0 release.
- The evidence document records the stage base, the output commit, the build
  commit and the build identity. Record them here and in the Grove commit.

## Notes

At entry, rebase the line onto the then-current Ravel `main`. Reconcile any
upstream change, rerun the affected checks and record the exact base. Keep
the work on `skill-directory-delivery`. Do not push it or land it on `main`.
Rollout owns those steps.

grove-dual-ravel-k27 consumes the build of this leaf as its directory build.
The build still reports `ravel 1.1.0`, so identify it by commit and build
identity.

Earn a `review-impl` only if the change is more than a removal and the
settled refusal.

## Decisions (running log)

- Base at entry, 2026-11-03: `jj git fetch` in `Ravel.skill-directory-delivery`
  reported no change. Local `main` and `main@origin` are both
  `24a545236185`, so the line above `7a894c782037` needed no rebase and no
  reconciliation.
- The `Origin` enum had one variant left, so `SourceSetting` now holds the
  `path` directly. The parser's `Problem` carries an optional remedy, and only
  the `type` problem sets it (`TYPE_REMEDY`). `settings.rs` uses it in place of
  the generic settings remedy. Every other `settings_invalid` keeps that remedy.
- The `type` check runs right after the record is found to be an object. A
  non-object record still refuses at `skills.sources[N]`, because it has no
  `type` to read.
- The "Local sources only" fixtures use a real Git checkout, made by the
  caller's own `git` with no global or system configuration. A spy `git` is
  first on Ravel's `PATH`. The resolve fixture's `PATH` gained its private
  `bin` for this.
- Controls seen failing: `type` checked after the unknown-field rule failed the
  unit test and the resolve fixture. The generic remedy in place of
  `TYPE_REMEDY` failed both public fixtures. The spy script recorded a call
  when it ran.
- No `review-impl` leaf: the change is the removal, the settled refusal and
  their fixtures, which the leaf's Notes exempt.
- `task docs:breaks -- --strict docs/book/src/06-reference.md` reports two
  runs over 450 words. Both are at the base `7a894c78` with the same or more
  words, so this change does not own them. The stage's book task is
  `docs:breaks:test`.
- Build commit `78332fcc367d7f28d04c3de10361b687bbf89319`, the child of
  `7a894c782037`. Checks run on it with an empty working copy.
- Checks on the build commit, each exit 0: `task check` (728 of 728 run, 48
  skipped), `task test:cli` (14 of 14 run, 1 skipped),
  `task harness:smoke:test`, the seven book tasks (24 of 24 excerpt mutants
  killed), `task test:native-linux PLATFORM=arm64` (721 of 721 run, 48
  skipped, `failed=0`, unchanged subjects), `task install` and
  `task smoke:pair` against Homebrew 1.1.0. A first Linux invocation without
  `OUT` refused before any test ran, and the rerun above supplied it.
- Stage record, in the Ravel section "Local-sources stage record": base
  `main` `24a545236185b0910ba6117fe8f9e8b2e2c8a748`; build commit
  `78332fcc367d7f28d04c3de10361b687bbf89319`; output commit
  `4f831feb4d0f288f9699b1a4b02fc24cda9843c5`, the tip of the unpushed
  bookmark `skill-directory-delivery`; build identity
  `9feb86b7982a3920f738b96ffd2d6e6bc1744d6345b87451196e9299721fb234`;
  installed `bin/ravel` SHA-256
  `151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`. The
  build reports `ravel 1.1.0`. Nothing was pushed or landed on `main`.
- Brief promotion: the node brief and the grove-dual-ravel-k27 brief name
  the output commit, the build commit and the build identity.
