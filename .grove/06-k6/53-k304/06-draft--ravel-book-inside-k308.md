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
