# testanyware-keg-projection-k296

## Goal

Make TestAnyware's release template and the tap's generated `testanyware`
formula install the projected keg layout together. A later TestAnyware
release must render the same layout.

## Context

- TestAnyware's repository is `~/Development/AbstractingComputers/TestAnyware`.
  Its `scripts/templates/testanyware.rb.tmpl` renders the tap formula at
  each release, through `scripts/release-build.sh` and
  `scripts/release_candidate.py`. `scripts/tests/release-pipeline-test.sh`
  tests that rendering.
- The tap's `Formula/testanyware.rb` installs
  `share/ravel/skills/testanyware` as a real directory. Its release archive
  carries the source at that path, and the archive may keep it.
- "The keg layout" of `docs/specs/codex-skill-provisioning.md` owns the
  layout. Finding L6 of the evidence section "Directory-link layout review"
  explains why a tap-only edit does not last.
- tap-skill-keg-projection-k295 adapts `tap:formula:test` to the layout.

## Done when

- The template moves the archive's source into
  `libexec/skill-sources/testanyware/` and creates the relative alias. The
  executables, `pkgshare` and the Linux library and venv steps are
  unchanged.
- TestAnyware's release pipeline test asserts the projected install method
  in the rendered formula. TestAnyware's applicable checks pass, and its
  change is committed and pushed in its own repository.
- The tap's `testanyware.rb` carries the same install method and a formula
  `revision` at version 2.8.0. It is committed in the tap without a push,
  beside the commit of tap-skill-keg-projection-k295.
- `task tap:formula:test` passes for the revised formula. The run records
  its full package version.

## Notes

The template renders no `revision`. A later TestAnyware release raises the
version, which resets the revision. Record that in TestAnyware's release
documentation if its process would otherwise copy the tap's revision.

This leaf does not cut a TestAnyware release. skill-formula-publication-k298
publishes the tap revision.

shared-directory-seam-k294 delivered the seam. Change the formula's `layout`
in `scripts/skill-formulae.json` to `projected` with its revision. The seam
refuses a keg that differs from its declaration. `task tap:formula:test` now
also needs `RAVEL_FORMULA` and `RAVEL_ARCHIVE`, and its seam reads the
published formula from the tap's `origin/main` as the migration predecessor.
The revision needs a new package version, or the seam refuses the pair.

## Decisions (running log)

- TestAnyware's `release-publish.sh` copies the rendered template over the
  tap's `Formula/testanyware.rb`. It never reads the tap's formula, so a
  release cannot carry the tap's revision forward. The template states in a
  comment that it renders no revision. No other release document needs the
  note.
- The release pipeline test asserts the install lines and the alias
  assertion of the test method. It also asserts that no line installs a real
  directory in `share/ravel/skills` and that no `revision` renders. The
  control was the template of TestAnyware's `main`. With that template, the four layout
  checks failed. A rendered formula with `revision 1` failed the revision
  check.
- The tap formula is the template rendered at 2.8.0 with the release's three
  `sha256` values, plus a comment and `revision 1` after `license`. The old
  template rendered the same way is byte-equal to the published tap
  formula. The only difference between the two renderings is therefore the
  layout.
- The tap test keeps its `direct` branch. No tap formula declares that
  layout now, but the seam still uses `direct` for published predecessors.
  `skill-formula-publication-k298` changes those declarations.
- TestAnyware commit `cd2bcf5f35de` is on its pushed `main`, and tap commit
  `12b53f8c449d` lies above `c0e61c1f3309` with no push.
  `task tap:formula:test` passed for `testanyware 2.8.0_1`. The section
  "TestAnyware keg projection" of `docs/homebrew-skill-delivery-evidence.md`
  records the run and the controls.
