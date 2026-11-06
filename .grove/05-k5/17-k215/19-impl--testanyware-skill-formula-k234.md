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
