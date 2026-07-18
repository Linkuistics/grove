# release-k4

**Kind:** work

## Goal

Cut the release carrying driver-side-kill-k2 so the trial machines' brew
binary has the fix.

## Context

Releases are cut manually via `scripts/release-{doctor,build,publish}.sh`.
Removing `GROVE_HARNESS_PID`/`GROVE_CLAUDE_PID` is a breaking change to the
loop↔agent contract (the embedded skill content updates in lockstep).

## Done when

Release published and installed (`brew upgrade`); the **live installed
binary** demonstrates the new behaviour (a version bump + green build is not
enough — verify the wired feature, not the build).

## Notes
