# release-k4

**Kind:** work

## Goal

Cut the major release (v11.0.0 — `do` losing its name argument is a breaking
CLI change) with a changelog entry, and verify the **live** brew-installed
binary drives the new scheme end-to-end.

## Context

Releases are cut manually via `scripts/release-{doctor,build,publish}.sh`.
Watch-out (v7 lesson): a version bump + green build ≠ feature wired — verify
the live binary's behaviour, not just that it builds.

## Done when

Brew-installed grove v11.0.0: in a scratch repo with a hand-created worktree
(`git worktree add`), argument-less `grove do` opens a bootstrap session
(`--no-launch` for the scripted check); `grove retire <node-path>` resolves
in-worktree; `grove do --help` shows no name argument and no `--start-point`;
changelog cut; tap updated.

## Notes

