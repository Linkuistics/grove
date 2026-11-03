# ravel-confinement-stage-record-k149

## Goal

Record the Ravel build after the confinement removal, with the checks of the
parent brief on that build commit.

## Context

- The "Linux floor repair record" and "Visible loader report record" of
  Ravel's `docs/skill-discovery-design-evidence.md`. They give the shape of a
  stage record.
- Grove's `scripts/harness-trials/builds.json`.

## Done when

- `task check`, `task test:cli`, `task harness:smoke:test`, the book tasks and
  `task test:native-linux PLATFORM=arm64` pass on the build commit.
- Ravel's design evidence holds a new stage record. It names the build commit,
  the build identity, the installed `bin/ravel` SHA-256 and each check result.
- Grove's `scripts/harness-trials/builds.json` names the new build.

## Notes

Do not publish a release, push the bookmark or install `ravel@2`. Do not
change the owner's settings or the 1.1.0 keg.
