# ravel-release-channel-k325

**Reviews:** `ravel-release-channel-k305`

## Goal

Read Ravel's release-channel tooling adversarially before its first public
use. That use publishes `ravel@2` to GitHub, Pages and the tap, which cannot
be undone. Produce findings only.

## Context

- The artifact is the Ravel commit "Carry one explicit release channel
  through the release tasks (ravel-release-channel-k305)" on the bookmark
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
- The contract is Ravel `docs/specs/release-channels.md`, sections "One
  release, two formula channels", "Release interface" and "Recovery and
  existing processes".
- The producer's decisions are in the running log of
  `ravel-release-channel-k305`.

## Done when

- Each doubt below has an answer that cites the source, a fixture or a
  reproduction.
- Each finding names its file and line, a concrete failure scenario and its
  severity.

## Notes

The producer's own fixtures replace gh and brew. They cannot settle these
doubts:

- Real Homebrew 7.0.13 behaviour for a first `brew install
  linkuistics/taps/ravel@2` beside a linked unversioned `ravel` 1.1.0. Does
  anything relink, upgrade or clean up the 1.1.0 keg that the running driver
  uses? Does `brew --prefix ravel@2` resolve to the keg before and after the
  opt link exists? Does `brew info --json=v2` report
  `keg_only_reason.reason` as `":versioned_formula"`?
- Task 3.54 reads environment values as variables. An exported `CHANNEL` in
  the operator's shell therefore satisfies the required variable. Decide
  whether that defeats "omission never publishes unversioned `ravel`".
- `release-prepare.sh` predicts the target with `next_version`. The
  admission then runs after `jj new main`. Check that every refusal leaves the
  owner's work recoverable and that no refusal comes after a mutation of
  `main`.
- Publication checks admission against the tap at `check_tap`'s fetch. Look
  for a window between that fetch and the tap commit, and for a resume that a
  stale `target/dist` misleads.
- The formula test now writes `"sources": []` into its `HOME`. Check that
  the real `brew test ravel@2` still exercises the default skill directory
  through the relocated copy.

The recovery prose of book chapter 11 is out of scope. The book pipeline
after this review owns it.
