# ravel-confinement-removal-k145

## Goal

Remove OS confinement from Ravel 2.0.0 on the bookmark
`skill-directory-delivery`, as the integrated design of
confinement-withdrawal-k144 states.

## Context

- The integrated design of confinement-withdrawal-k142 and
  confinement-withdrawal-k144.
- `Ravel.skill-directory-delivery`. Follow that repository's VCS discipline
  and Taskfile.
- The "Linux floor repair record" and "Visible loader report record" of
  Ravel's `docs/skill-discovery-design-evidence.md`, which name the present
  builds.

## Done when

- Ravel has no confinement code, flag or backend. A removed flag gives the
  refusal and the remedy of the design.
- Ravel's specifications, decision records and book agree with the code.
- `task check`, `task test:cli`, `task harness:smoke:test`, the book tasks and
  `task test:native-linux PLATFORM=arm64` pass on the build commit.
- Ravel's design evidence holds a new stage record. It names the build commit,
  the build identity, the installed `bin/ravel` SHA-256 and each check result.
- Grove's `scripts/harness-trials/builds.json` names the new build.

## Notes

Do not publish a release, push the bookmark or install `ravel@2`. Do not
change the owner's settings or the 1.1.0 keg.
