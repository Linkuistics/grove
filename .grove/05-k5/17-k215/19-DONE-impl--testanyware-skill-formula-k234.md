# testanyware-skill-formula-k234

## Goal

Make TestAnyware's release archive and formula install `using-testanyware`
at `share/ravel/skills/testanyware`. Publish that release.

## Context

- The repository is `~/Development/AbstractingComputers/TestAnyware`. Read
  its own instructions and its release procedure first. Its Taskfile wraps
  `scripts/release-*.sh`.
- "Other formulae" of "The keg layout" in
  `docs/specs/codex-skill-provisioning.md`. The formula is
  `Formula/testanyware.rb` in `~/Development/Delivery/homebrew-taps`.
- The skill lies in `plugins/testanyware`, with the root `skills`.

## Done when

- Each release archive carries `plugins/testanyware` at
  `share/ravel/skills/testanyware`, with executable bits and the archive's
  own checks.
- The formula installs it in the keg, so Homebrew links it into the default
  skill directory.
- TestAnyware's README states that the formula installs the skill and Ravel
  delivers it. It gives no native marketplace step.
- TestAnyware's own checks pass. Its release procedure publishes the
  release and updates the tap. The installed keg source has the
  `resourceDigest` of a clean plain checkout of the release commit's
  `plugins/testanyware`.
- `docs/repository-moves-record.md` records the release, its commit and the
  digest.

## Notes

Use the release increment that TestAnyware's procedure states for a new
installed path. A minor release is the default.

If the procedure stops after a partial publication, resume its remaining
steps. Do not run the release again.

## Decisions (running log)

- The release is minor, `v2.8.0`. TestAnyware's procedure states no
  increment for a new installed path, so the default of this leaf applies.
- `release-build.sh` copies the skill with `git archive` of
  `<commit>:plugins/testanyware`, with Git's global and system
  configuration off. Git's export holds only tracked files, with their Git
  modes, so the copy equals a clean plain checkout.
- `release_candidate.py verify` checks each host archive. The files under
  `share/ravel/skills/testanyware` must equal the tree of
  `plugins/testanyware` at the commit of the manifest, by path, Git mode and
  blob ID. The build, the publication and `release:candidate-verify` all run
  that check.
- `plugins/testanyware/.claude-plugin/plugin.json` and `task plugin:check`
  stay. The README gives no native marketplace step.
  `native-marketplace-retirement-k235` owns Grove's marketplace entry.
- The live GUI checks do not run for this release. The commits after
  `v2.7.0` change no code of the app or the host CLI.
- `release-build.sh` packs each archive with `tar --no-mac-metadata`. The
  first run of the new archive check found AppleDouble members (`._<name>`)
  for each entry. macOS gives `com.apple.provenance` to each file that a
  build writes, and macOS tar archives that attribute as such a member. The
  2.7.0 macOS archive holds 35 of them among 70 members. macOS tar hides
  them, so `tar -tf` lists none, and the macOS keg holds none. GNU tar on a
  Linux host unpacks each one as a file into the keg.
- `release-publish.sh` takes a jj tap, as Grove's `release-publish.sh` does,
  and its default tap path is `~/Development/Delivery/homebrew-taps`. The
  former default does not exist. The tap clone is a colocated jj repository
  with a detached git `HEAD`. The former bare `git push` fails there, after
  the tag and the GitHub release. A plain git tap must now be on
  a branch with an upstream.
- Release commit `1809a49068a23d9d276913f3a90227dbefb98a82` is on
  `main@origin`, and the local annotated tag `v2.8.0` names it. The candidate
  `target/candidates/v2.8.0` in TestAnyware is built, notarised and
  verified. Its formula passed `task tap:formula:test` with `resourceDigest`
  `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6`. The
  harness's permission classifier refused `task release:publish`, so the
  publication waits for the human. Do not build the candidate again.
- The human ran `task --yes release:publish TAG=v2.8.0` in this session. It
  published the tag, the GitHub Release with four assets and tap commit
  `9ca2b5e37510`. `brew upgrade` installed 2.8.0, and the installed keg
  source has the `resourceDigest` of the plain checkout. The section "The
  TestAnyware formula" of `docs/repository-moves-record.md` records the
  results.
