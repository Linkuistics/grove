# ravel-local-handoff-k61

## Goal

Close the local stage with a reproducible matched temporary installation, a
reconciled documentation set and recorded commits. Pinned delivery and Grove's
dual-Ravel stage then use one known build as their base.

## Context

- The `## Integration handoff` and `## Settled inputs` sections of
  `ravel-local-delivery-k23`.
- Ravel `docs/skill-discovery-design-evidence.md`: "Feature line and stage
  handoff" and "Consumer handoff".
- Ravel `scripts/install.sh`, `scripts/installed-check.sh` and
  `scripts/installed-smoke.sh`.

## Done when

- The session fetches Ravel. When `main` moved, it rebases the line, reconciles
  upstream changes and reruns the affected checks. It records the exact base.
- The documentation set describes the delivered local subset and nothing more.
  That set is the specifications, `CONTEXT.md`, the book, the examples and
  `CHANGELOG.md`. It names the temporary Git refusal and the unfinished pinned
  stage. All book tasks pass, with `task book:integrity`.
- `task check`, `task test:cli`, `task harness:smoke:test` and
  `task test:native-linux` pass on the final commit.
- `task install PREFIX=<temporary prefix>` installs the feature build. The
  installed checks pass with an isolated `HOME`. The build reports
  `ravel 1.1.0`. The record names its commit, build identity, toolchain and
  platform.
- The temporary installed-pair smoke runs the installed 1.1.0 release and the
  feature build at the same time, with fake managed programs. The 1.1.0
  release runs a native-only launch under one temporary `HOME`. The feature
  build runs a directory launch under a second temporary `HOME`. A settings
  file with `skills` under the first `HOME` refuses in 1.1.0, exit 2. Neither
  run reads or writes the owner's real `HOME`. The installed 1.1 resources,
  settings and ending executables do not change.
- The Ravel evidence document records the base, the output commit, the build
  inputs and the platform facts of this stage. The Grove brief of
  `ravel-pinned-delivery-k25` and the running log name the same commit.
- Nobody pushes the line, and no release consumes it.

## Notes

When a producer of this stage cuts a `review-impl` leaf, it inserts that leaf
before this one with `grove-llm leaf-insert`. The review and its integration
then precede the handoff.
