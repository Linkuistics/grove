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

## Decisions (running log)

**The build (2026-11-06).** The new 2.0 build is Ravel
`ff14cba826b02ac492fcde0996d662c5378ef923`, the k226 commit at the tip of
the bookmark `skill-directory-delivery`, built from an empty working copy
above it. Its build identity is
`58d479ed058f1094b8754c72eecf9f0cec5a60b3d22b76baecdb827364abd217`. The
`output` files of Ravel's build script give that identity for each of the
three release targets. `task release:archives` built all three archives at
that commit. The lane names one identity for every target, so the Linux
floor targets take the new build too.

**The scope of the pins (2026-11-06).** Only the 2.0 lane of
`scripts/ravel_target_inputs.py` and the current tables of
`docs/ravel-target-inputs.md` change. The dated records keep the builds
that they measured. `scripts/harness-trials/builds.json` pins the build of
the real harness trials, and `candidate-keg-k238` owns those trials, so it
stays.

**The fixture (2026-11-06).** `Homebrew::plant` in `directory_route.rs`
copies the 2.0 build into `Cellar/ravel@2/2.0.0` of a private prefix. It
writes the record there, which names the prefix's `share/ravel/skills`. That
directory holds relative links into two source kegs: `engineering` (roots
`skills` and `craft`) and `grove-core` in the `grove` keg, and the craft-only
`personal` in its own keg. No settings name a mandatory file on that route,
so the fake agent's new `discover` finds the kind and then the spine by name.
The shared copy of `testing/support.rs` holds no record, so it is the
record-less build of the third case.

**Controls (2026-11-06).** Against the earlier build `23e50ce90037`, three
of the four new cases failed: the two that expect default sources, and the
case with `"sources": []`, which that build refused for the missing
`mandatory`. The record-less case passed there too, as it should. A mutation
that gave the record-less case the copy with a record failed it at the
delivery check: the prompt carried Ravel's skill-directory preamble. The file
was restored to the digest of the measured run.

**First measured run (2026-11-06).** `task ravel:directory` passed 11 of
11, and `task release:smoke` passed all three targets. Both gates failed
only `cargo clippy`: `type_complexity` on the source table of
`Homebrew::plant`. A named `KegSource` struct replaced the tuple. The target
controls did not run, because the script gave no `ARCHIVES`. Every check
then ran again on the final files.
