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
