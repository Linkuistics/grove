# grove-default-directory-checks-k228

## Goal

Move Grove's Ravel 2.0 lane to a build that reads the default skill
directory. Add the default-directory cases to Grove's caller checks.

## Context

- The build is Ravel's `skill-directory-delivery` line after
  `ravel-default-directory-k225`.
- `docs/ravel-target-inputs.md` and the `ravel:targets:*` tasks pin the 2.0
  build. `docs/grove-dual-ravel-stage-record.md` records the builds that
  Grove measured.
- "Test seams" of `docs/specs/codex-skill-provisioning.md` names the cases:
  the rows "Directory bootstrap", "Native route on 2.0" and "Settings
  isolation". The paragraph on `crates/grove/tests/directory_route.rs` states
  the fixture.
- "Grove's checks" in the running log of
  `ravel-default-skill-directory-k218`.

## Done when

- Grove's 2.0 lane names the new build by commit and build identity.
- Each check copies the 2.0 build into a private prefix and writes its
  installation record. The record names a fixture directory of
  Homebrew-shaped links. The resolution runs in a HOME without settings.
- `directory_route.rs` shows these cases:
  - With no `skills` key, a launch receives each source of the fixture
    default directory.
  - With `"sources": []` beside that directory, it receives none.
  - A copied build without a record delivers nothing.
  - A shared HOME without `skills` delivers the fixture sources on 2.0.
- `task ravel:directory` passes with the new build.
- The dual-Ravel stage record names the new build and its results.
- `bash scripts/check.sh` passes.

## Notes

The Linux floor targets can need a new build for this commit. Then record
the result of the general floor route. Otherwise record why that route keeps
its earlier build.

Guidance that says that the directory route needs `skills` changes in
`homebrew-delivery-guidance-k237`.
