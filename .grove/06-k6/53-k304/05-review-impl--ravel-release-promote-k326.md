# ravel-release-promote-k326

**Reviews:** `ravel-release-promote-k306`

## Goal

Read Ravel's promotion task adversarially before its first real use. Finish
runs it after `finish-commit`, so no leaf can repair it then. That use pushes
to the public tap and moves the machine's default `ravel` command to 2.0.0.
Produce findings only.

## Context

- The artifact is the Ravel commit "Promote a complete ravel@2 release to
  unversioned ravel (ravel-release-promote-k306)" on the bookmark
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`. Its
  main file is `scripts/release-promote.sh`. It also changes
  `release-channel.sh`, `release-formula.sh`, `release-formula.test.rb`,
  `release-install.sh`, the Taskfile and `release-channel.test.sh`.
- The contract is Ravel `docs/specs/release-channels.md`, sections
  "Promotion without another release", "Promotion interface", "Consumer
  readiness receipt" and "Recovery and existing processes". Grove
  `docs/specs/coordinated-release.md` owns state D and the row "Promotion
  partial".
- The producer's decisions are in the running log of
  `ravel-release-promote-k306`.

## Done when

- Each doubt below has an answer that cites the source, a fixture or a
  reproduction.
- Each finding names its file and line, a concrete failure scenario and its
  severity.

## Notes

The fixtures replace gh and brew, and a stub replaces the Homebrew formula
test. `task release:formula` runs the real formula test. These doubts remain:

- The jj tap states. Look for a state that the classification admits but
  should refuse. Examples are a conflicted `main` after the fetch, a working
  copy away from `main`, and a remote that moves between the fetch and the
  push. Check that no refusal leaves a half-written tap.
- The evidence and the receipt. Decide whether the tag commit, the asset
  digests, the book commit and the consumer formula's version are enough
  identity. Find a receipt that the jq checks accept and that should refuse.
- The deprecation date is the tag commit's UTC date. Homebrew compares it
  with `Date.today` in local time. Find a window where `ravel@2` renders as
  not yet deprecated.
- `release-install.sh --version` runs from `main`, not from the tag. Check
  that its working-tree `installed-record.sh` and its Homebrew calls cannot
  accept the wrong keg. Compare the new retained-keg rule with Homebrew's
  real directory layout.
- The order. Check that every verification precedes the first tap write and
  the first Homebrew mutation. Check that a failed installation after the
  push leaves a state that a rerun completes.

The real upgrade rehearsal with Homebrew belongs to
`promotion-upgrade-rehearsal-k318`. Book chapter 11 belongs to the book
pipeline after this review.
