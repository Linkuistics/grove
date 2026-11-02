# ravel-git-install-k74

## Goal

Settle and test the installed Git dependency. Decide whether the Homebrew
formula declares Git, and prove a cache miss with Git and a cache hit without
it in an installed layout.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: the transport
  paragraph of "Exact Git acquisition and offline reuse".
- Ravel `scripts/templates/ravel.rb.tmpl`, `scripts/release-formula.sh`,
  `scripts/release-formula.test.rb`, `scripts/installed-check.sh` and
  `scripts/install.sh`.
- The release preparation tasks in the Settled inputs of
  `ravel-pinned-delivery-k25`.

## Done when

- The decision about the formula is recorded with its evidence in the Ravel
  design evidence. It covers macOS, where `/usr/bin/git` can be a stub that
  asks to install developer tools, and Linux, where Git can be absent. The
  formula template and its test follow the decision.
- The installation documentation states that a cache miss needs Git on the
  `PATH` of the Ravel CLI and that a cache hit does not.
- The installed checks resolve a local Git fixture with Git, then resolve the
  warm cache with no Git on `PATH`, each with an isolated `HOME`. They also
  check the `skill_revision_unavailable` refusal of a cold pin without Git.
- `task install PREFIX=<temporary prefix>`, `task release:test`,
  `task release:formula`, `task release:archives` and `task release:smoke`
  pass where the platform admits them. The record names each platform that
  did not run.
- No shared installation, settings or tap changes, and nothing is published.
