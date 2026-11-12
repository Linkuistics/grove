# ravel-release-notes-upgrade-order-k339

## Goal

Put the Grove upgrade remedy into the release notes of Ravel 2.0.0 before
finish promotes Ravel. The remedy is "upgrade Grove with Ravel, or first".

## Context

- `docs/specs/coordinated-release.md`, the paragraphs after the table of
  "Compatible tap states". The spec requires the remedy in the Ravel 2.0.0
  release notes before promotion.
- `docs/promotion-upgrade-rehearsal-record.md`, sections "Homebrew output of
  the Ravel-only upgrade" and "Grove 22.8.0 beside Ravel 2.0.0".
- promotion-upgrade-remedy-docs-k338 corrected Grove's own pages. Use its
  wording in Grove's caveats and `docs/USAGE.md`, section "Promotion".
- Ravel's `CHANGELOG.md`, section "v2.0.0", and the GitHub Release of
  `v2.0.0`. The Release body is a copy of that section. Neither mentions
  Grove's upgrade order or the place of `"sources": []`.

## Done when

- The notes that a user reads for Ravel 2.0.0 say that a lone
  `brew upgrade ravel` leaves Grove 22.8.0 or earlier. Homebrew upgrades a
  dependent only when the dependent and each of its dependencies are
  bottled, and neither formula is. That Grove refuses Ravel 2.0.0, and its
  message names `brew upgrade ravel`. The remedy is `brew upgrade` with no
  formula name, or `brew upgrade grove`.
- The same notes give the off value in its place,
  `{"skills": {"schemaVersion": 1, "sources": []}}`, if they state it.
- The chosen place of the notes and the reason for it are in the running
  log.
- Ravel's checks pass, and the change is published through Ravel's own
  procedure.

## Notes

The book of v2.0.0 is final: `release-promote.sh` refuses promotion when the
`gh-pages` commit of that book differs from the cut's. Do not change the
published book. The `v2.0.0` section of `CHANGELOG.md` is tagged, so an edit
there does not change the released notes. The GitHub Release body is
outward-facing. Ask the human to approve the edit before you make it.
Promotion makes no cut, so no later Ravel release carries the notes before
finish.
