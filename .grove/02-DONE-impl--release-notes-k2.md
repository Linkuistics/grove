# release-notes-k2

## Goal

Deliver `task release:notes` with the behavior and agreed test seams in the root
brief. Share its writing instructions and generation path with full-release
preparation.

## Context

- `scripts/check.sh` enumerates shellcheck inputs and the release-preparation
  integration suite; include any new script and test entry points there.
- `plugins/grove/conformance.sh` discovers `grove-*` skill directories as
  methodology kinds. Account for that when naming and installing the standalone
  writing skill: it operates on staged artifacts, without a task-tree lifecycle.
- `release-notes-runner-k3` owns actual personal configuration and live execution.
  This leaf establishes and documents the command's required runtime interface.

## Done when

- The Taskfile exposes the standalone command and its help describes refreshing
  Unreleased from all changes since the last release.
- Evidence collection, staged writing instructions, output validation and
  changelog updating meet the root brief. The selected revision is fixed for a
  run; all unreleased ancestors contribute to the evidence.
- Existing Unreleased text reaches the writer, the result replaces that section,
  and versioned notes and unrelated content remain intact.
- Standalone generation preserves the workspace's jj parents and bookmarks.
  Failure and invalid output leave the changelog unchanged.
- Full-release preparation reuses generation while preserving its existing
  `main`, fill-only, commit and publication responsibilities.
- The deterministic integration suite exercises multiple unreleased commits,
  refreshed notes, baseline/no-change diagnostics, malformed or missing output,
  runner failure and existing full-release behavior. Test the Taskfile entry as
  well as the underlying script interface.
- The writing skill has a single source used by both paths, is explicitly staged
  or supplied as instructions to the standalone invocation, and follows that
  invocation's completion instructions.
- Usage, setup requirements and the shipped change are documented; relevant
  shell checks and integration tests pass, followed by `bash scripts/check.sh`.

## Decisions (running log)

- One script, `scripts/release-notes.sh [--to <revset>]`, owns evidence,
  invocation, validation and the changelog write. The task uses the default `@`;
  `release-prepare.sh` keeps fetch, `main`, fill-only and commit, and calls it
  with `--to main`. An option beat a sourced library: one interface, one test
  seam, and prepare's working copy already sits on `main`.
- The endpoint is resolved to a commit id once. The Unreleased input is read
  from that commit, and the write is refused when the working copy's
  `CHANGELOG.md` no longer matches it, so an edit made mid-run is never
  overwritten.
- The baseline is `tags(exact:"v<version>") & ::<endpoint>` through jj rather
  than `git rev-parse`, because the task must work in a non-colocated workspace
  such as this grove's.
- The skill is `scripts/release-notes/SKILL.md`, outside `plugins/`. Under
  `plugins/grove/skills` a non-kind directory would sit in the tree
  `conformance.sh` reads as the methodology, and `install.sh` would link it into
  every harness; neither is wanted for a repo-local writer that is always staged
  as an `--input`. The prompt only points at the staged file.
- `current-unreleased.md` is always staged, empty or not, so both paths make the
  same five-input invocation and k3 configures one interface.
- Under `set -e` a bare `! grep` asserts nothing (shellcheck SC2251). Negative
  assertions in the suite are `if … then fail`; five mutations of the script
  (latest-only descriptions, latest-only diff, wrong previous changelog, lost
  spacing, withheld Unreleased text) were each seen to fail the suite.

## Notes

Keep this increment independently verifiable using the fake runner. The later
runner leaf makes the user's real headless configuration operational and runs
the live smoke test. Do not turn absence of that local setup into an excuse to
skip deterministic command coverage.
