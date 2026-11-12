# candidate-state-rehearsal-k334

## Goal

Add tap state C to the promotion rehearsal instrument. Rehearse fresh C and
the upgrades from A and from B to C in the private Homebrew.

## Context

- The node brief, `_promotion-upgrade-rehearsal.md`, and the instrument and
  record that scratch-homebrew-rehearsal-k333 made.
- `docs/specs/coordinated-release.md`, "Compatible tap states", row C.
- `scripts/templates/grove.rb.tmpl`, `task release:archives` and
  `scripts/release-formula.test.sh` for the candidate formula.

## Done when

- The instrument renders a candidate Grove formula from the template with
  unreleased archives of the working result, through `file://` URLs. It
  commits that formula to the scratch tap and pushes it to the scratch
  `origin`. It never changes the real tap.
- Fresh C and the upgrades from A and from B to C give a usable pair. Native
  setup works by default with unversioned Ravel 1.1.0. Directory setup
  explicitly selects `ravel@2` and reads Grove's keg sources with no
  settings.
- The old 1.1.0 front reports 1.1.0 and refuses `end-session` outside a
  session after each package mutation.
- The record gains the state C section: commands, results, Homebrew output
  for each upgrade, and the version label of the candidate keg.

## Notes

The candidate version must sort above 22.8.0 for Homebrew to see an upgrade.
State which version label the candidate uses and why, and keep it for k335.
