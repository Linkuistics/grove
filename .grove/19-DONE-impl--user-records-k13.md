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

## Decisions (running log)

- Each record landed as two changes: a conversion to controlled English, then the
  edit. `docs/RELEASING.md` and `docs/USAGE.md` are large, so each converted only
  the touched sections and carries `pending` markers for the rest. `driver.md`,
  `configure-grove/SKILL.md` and `references/policy.md` are small and converted in
  full.
- Front matter of the two skill files: the checker reads only the `description`
  value of front matter, so no marker is needed. The one failing sentence of
  `configure-grove/SKILL.md` ("Use when installing, ... or remedying a refused
  launch", 29 words) became two sentences with the same triggers. `policy.md` has
  no front matter. A marker above front matter was never needed.
- `docs/RELEASING.md`: six product packages (`grove`, `grove-llm`, `grove-loop`,
  `grove-tui`, `jj-workspace`, `ordinal-fs-tree`) inherit the workspace version.
  `book-validation` is the seventh member and carries its own version. The
  anchor `one-release-eight-packages-one-tag` stays. Its heading and the three
  prose and comment counts say six.
- `docs/USAGE.md`: the launch policy section states "stable Ravel 1.x from
  1.1.0" and "Grove refuses Ravel 1.0.x". The standalone section checks the
  cancellation and a successful Ravel status. The stopping section sends the
  forwarded signal to the `ravel` process. The ending section drops the
  surviving supervisor group. All anchors and the coverage map stay, so
  `user_guide_coverage.rs` needs no change from this leaf.
- `driver.md` carries no reference to Grove's own documents, so the ADR on what
  binds without dependencies needs nothing here. Its post-reap order loses the
  surviving supervisor group and the gone group. The seven conformance anchors
  for the file still hold (`bash plugins/grove/conformance.sh`).
- The Ravel links to `v1.0.0/docs/session-cli.md` stay. Ravel 1.1.0 ships no such
  document, as `ravel-floor-k6` found.
- `configure-grove` (SKILL.md and policy.md), `CONTEXT-MAP.md` and
  `docs/ARCHITECTURE.md` now state "stable Ravel 1.x from 1.1.0". Both sections of
  the last two were already converted by `design-records-k12`.
- Search of the five owned records for `keyed-launch`, `keyed_launch`, `survivor`,
  `surviving`, "gone group", "group is gone", "supervisor group", "process group",
  `seven`, `1.0.0` and "stable 1.x". A seeded `docs/zz-seed-control.md` returned
  all 7 of its lines, then I removed it. The remaining matches are the three links
  to the Ravel v1.0.0 CLI document and two plain-English "surviving old ancestor"
  phrases in the viewer text of `docs/USAGE.md`, which are unrelated to process
  groups.
