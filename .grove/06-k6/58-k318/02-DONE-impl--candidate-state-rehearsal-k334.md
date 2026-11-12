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

## Decisions (running log)

- The candidate label is the next minor after the workspace version, with the
  pre-release suffix `-rc.1`: `22.9.0-rc.1` while the workspace is 22.8.0.
  The owner's Homebrew 7.0.13 sorts it above 22.8.0 and below 22.9.0, so an
  upgrade sees it and the real 22.9.0 still supersedes it. A `-candidate`
  suffix sorts above 22.9.0 in Homebrew, because `candidate` is a plain
  string token. The label does not reserve 22.9.0, as
  `docs/specs/coordinated-release.md` requires. k335 keeps this label.
- The formula test asserts `grove --version` equals the formula version, so
  the binaries must carry the label. The instrument copies the files that a
  commit of the working copy would hold into the work directory. It sets the
  label in that copy's `Cargo.toml` and `Cargo.lock`, and runs
  `release-build.sh --unreleased` there for all three targets. The real
  working copy is not edited.
- State C is one commit on top of state B in the scratch tap. It replaces
  only `Formula/grove.rb` with the template rendered through `file://` URLs.
- Native setup and directory setup share one fixture HOME that holds both
  `policy.ts` and `config.rhai` and no `settings.json`. The spec says the two
  files coexist under one HOME. The fixture agent records its prompt and
  `RAVEL_SKILL_DIRECTORIES`, so the record shows which launch received the
  delivered skills.
- The first full run stopped in upgrade-a-c at `brew list --versions`. The
  upgrade keeps Grove 22.8.0 beside the candidate, because install cleanup
  is off. With two kegs in one rack, Homebrew 7.0.13 leaves the shell fast
  path of `Library/Homebrew/list.sh` for `cmd/list.rb`. That path also lists
  casks and fails on a private prefix without a `Caskroom`. The formula
  listing before the error was correct. The instrument now runs
  `brew list --formula --versions` and does not create a `Caskroom`. The
  measured run is the second, complete run.
- Both Ravel 1.1.0 and 2.0.0 print `stop attempt … Operation not permitted`
  in launches outside Homebrew's sandbox, and the launches succeed. That is
  the Darwin zombie-group `EPERM` that ravel-stop-eperm-k302 measured, kept
  as a report. The record's earlier observation tied it to Homebrew's
  sandbox. The record now states the measured cause.
