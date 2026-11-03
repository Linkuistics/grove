# ravel-confinement-removal-k145 — brief

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

## Decomposition

ravel-confinement-removal-k145 was too big for one session, so it became this
node. The design handoff is the "Implementation handoff" of Ravel's
`docs/skill-discovery-design-evidence.md`, at commit `2df9dd055824`. Its
sections divide the work. The children run in this order:

1. `ravel-confinement-code-removal-k147`: the sections "Source", "Tests" and
   "Scripts" of the handoff.
2. `ravel-confinement-book-removal-k148`: the sections "Book" and
   "Specifications and records that change with the code", except the stage
   record.
3. `ravel-confinement-stage-record-k149`: the full checks on the build
   commit, the stage record and the build in Grove's `builds.json`.

`task check` does not build the book, so the first child can pass its checks
alone. `task docs:examples` fails until the second child removes the
confined example of chapter 5.
