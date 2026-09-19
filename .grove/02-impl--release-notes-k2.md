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

## Notes

Keep this increment independently verifiable using the fake runner. The later
runner leaf makes the user's real headless configuration operational and runs
the live smoke test. Do not turn absence of that local setup into an excuse to
skip deterministic command coverage.
