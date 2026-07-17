# release-stamp-trial-k15

**Kind:** work

## Goal
Execute plan Task 13: merge to main, release v12.0.0 via
scripts/release-{doctor,build,publish}.sh, verify the LIVE brew binary, stamp
one grove per side, observe the review reroute live, start the trial clock.

## Context
- Plan Task 13: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Watch-out (v7 lesson): a version bump + green build is not a wired feature —
  verify the live binary's behaviour.
- Before merging: `git ls-files .grove` in this worktree must be empty on the
  merge commit — the finish cycle deletes .grove/ first; use `git rm -rf` if a
  staged retire rename trips it.

## Done when
Live binary verified per Task 13 step 3; both stamps written and full cycles
observed (including the pi reroute in a codex grove); the human has been
prompted to cancel Anthropic and the trial end date (~one month out) is
recorded in the release notes or trial log.
