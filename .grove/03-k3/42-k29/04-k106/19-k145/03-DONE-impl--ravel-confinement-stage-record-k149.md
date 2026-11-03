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

## Decisions (running log)

- **The build commit is `d324668b86ca`, the book removal commit.** The
  working copy above it is empty. `jj git fetch` changed nothing, and `main`
  and `main@origin` are both `24a545236185`, so the line needs no rebase.
- **The build identity is `a4f55d04…`.** A script that applies the rule of
  `crates/ravel/build.rs` gives `a4f55d04b2d19a9948f2bcdcb3f305ee20ae80fae7f9011ec878f117d98e6519`.
  The same script gives the recorded `fd8dfdc2…` for `e9cc79993b46`, so the
  script is a control. It also gives `a4f55d04…` for `dc162664`: the book
  commit changed no source that the executable embeds.
- **The book tasks are those of the Linux floor repair record, plus
  `docs:figures` and `docs:examples:mutants`.** k147 changed the example
  checker, so its mutants run again.
- **All checks pass on `d324668b`.** `task check` ran 694 tests, all passed.
  `task test:cli` ran 14, all passed. `task harness:smoke:test`, ten book
  tasks and `task test:native-linux PLATFORM=arm64` (12 steps, 687 tests)
  exit 0. The working copy was empty before and after the run.
- **The installed `bin/ravel` SHA-256 is `26b487af…`.** The scratch prefix
  holds the identity twice. The runner's `check_ravel` passes this front and
  refuses the 1.1.0 keg.
- **Grove records the build in a new acceptance record section.** The
  runner's convention needs the build in `builds.json` and in the record
  before any result, so `source` names `#confinement-removal-build`.
- **The node closes.** Each Done when of ravel-confinement-removal-k145 holds.
  The parent brief now names the build for k146 and k125.
