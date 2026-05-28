# 060-changelog-and-release-v4

**Kind:** work

## Goal

Land the v4.0.0 release for this workstream: consolidate the breaking-
change CHANGELOG entries from the prior leaves, bump the crate version,
and verify the release scripts cope with the bumped major.

## Context

- `CHANGELOG.md` — should already contain breaking-change entries from
  [[030-remove-list-and-version-and-adr-0007]] and
  [[040-merge-install-and-update-and-adr-0008]]. This leaf consolidates
  them under a v4.0.0 release header.
- `Cargo.toml` — the crate version (currently `3.0.1`-ish).
- `Cargo.lock` — refresh after the bump.
- `release.toml` — the cargo-release config; verify the major bump still
  fires correctly.
- `scripts/release-doctor.sh`, `scripts/release-build.sh`,
  `scripts/release-publish.sh` — releases are cut manually via these
  (per memory `release-process.md`); walk through `release-doctor.sh`
  pre-flight at minimum.
- Decisions from [[010-shape-the-feature]] driving this leaf:
  - Three CLI breaking changes bundle into a single major-version bump.
  - No deprecated aliases for any of the removed verbs.

## Done when

- `CHANGELOG.md` has a `## [4.0.0] - YYYY-MM-DD` (or unreleased — match
  this repo's convention) section listing the breaking changes:
  - `grove list` removed.
  - `grove version` removed (use `grove --version` for the CLI version,
    `grove status` for the full picture).
  - `grove update` removed; `grove install` is now idempotent and
    always prints a per-harness outcome line.
- `Cargo.toml` `version = "4.0.0"`; `Cargo.lock` refreshed.
- `scripts/release-doctor.sh` runs clean (or its diagnostics are
  understood and recorded if not).
- A migration note in `CHANGELOG.md` explains how to translate prior
  invocations (one-liners for each).

## Notes

- This leaf does **not** cut the release tarball — that's the manual
  `release-build.sh` + `release-publish.sh` step run by the maintainer
  after merge.
- The grove finishes with this leaf's commit. `grove finish` cycle
  (delete `.grove/` and merge) follows separately.
