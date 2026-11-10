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
