# user-records-k13

## Goal

Correct the records that the owner and the sessions read as instructions:
the usage guide, the release guide and the driver reference of the
methodology.

## Context

- The root brief: *Records to rework*, items for `docs/RELEASING.md`,
  `docs/USAGE.md` and `plugins/grove/skills/grove/references/driver.md`.
- `crates/grove/tests/user_guide_coverage.rs` checks the usage guide against
  `docs/specs/user-guide-coverage.md`. That spec is edited in `book-specs-k14`,
  so read the test before an edit to the guide.

## Done when

- `docs/RELEASING.md` says six product packages and keeps its anchor.
- `docs/USAGE.md`: the standalone section no longer names the survivor group
  of `grove run`. The lifecycle section no longer names a surviving
  supervisor group. The section on stopping the loop says that Grove sends the
  forwarded signal to the `ravel` process.
- The post-reap order in `driver.md` no longer names a surviving supervisor
  group or a gone group.
- The plugin and the methodology stay consistent. The conformance checks pass.
- Every edited record passes the technical-communication checker. Convert each
  record as its own change before the edit.
- `bash scripts/check.sh` passes.

## Notes

- `driver.md` is part of the plugin that other projects install. State what
  binds without Grove's own documents, as
  `docs/adr/a-skill-states-what-binds-without-its-dependencies.md` requires.
- Do not edit the released entries of `CHANGELOG.md`.
- `ravel-floor-k6` moved the floor to Ravel 1.1.0. These records still state the
  old floor or a bare "stable 1.x": `plugins/grove/skills/configure-grove/SKILL.md`
  ("starting at 1.0.0"), its `references/policy.md` ("currently accepts stable
  1.x"), `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md` and `docs/USAGE.md`. The front
  matter of the two skill files fails the checker, and a marker cannot sit above
  it. Decide how to convert them before the edit.
