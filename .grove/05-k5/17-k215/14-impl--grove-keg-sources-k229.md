# grove-keg-sources-k229

## Goal

Make Grove's release archive and formula carry core and the five domain packs
at `share/ravel/skills/<source-id>`, from Grove's record of pack commits.

## Context

- "The keg layout" and "The pack record and the release" of
  `docs/specs/codex-skill-provisioning.md`. The row "Installed distribution"
  of its "Test seams".
- `scripts/release-build.sh`, `scripts/release-common.sh` (the archive
  manifest), `scripts/templates/grove.rb.tmpl`, `scripts/release-smoke.sh`,
  `scripts/release-formula.test.sh` and `scripts/release.test.sh`.
- The record and its checkouts come from `grove-pack-record-k227`. The 2.0
  build comes from `grove-default-directory-checks-k228`.

## Done when

- The archive build copies each pack's source directory from the plain
  checkout of its `sha` that `check:packs` uses. It copies `plugins/grove` of
  the tagged tree to `share/ravel/skills/grove-core/`. Each copy keeps
  executable bits and links.
- The archive manifest lists each copied file with its Git mode, and each
  archive passes the manifest check. A control with a changed mode, and one
  with a missing pack checkout, has been seen to fail.
- The formula installs `share/ravel/skills/` beside the executables and
  `libexec/`.
- `task release:smoke` and `task release:formula:test` copy the 2.0 build
  into a private prefix, with a record that names the keg's
  `share/ravel/skills`. With no settings, `ravel skills resolve --json`
  reports exactly the six sources. Each `originPath` lies inside the keg.
- `release:formula:test` compares each source's `resourceDigest` with that of
  the plain checkout of the same commit.
- Fresh install and upgrade pass at each tap state A to D of the coordinated
  release, with an admitted Ravel major at each state.
- `bash scripts/check.sh` passes.

## Notes

The two marketplace files at the archive root belong to
`grove-hook-adapters-k230`.

A cut changes only `CHANGELOG.md`, so the `grove-core` bytes of a release
equal those of its candidate. Keep that true.
