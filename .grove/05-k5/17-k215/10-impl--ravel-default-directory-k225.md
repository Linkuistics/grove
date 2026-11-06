# ravel-default-directory-k225

## Goal

Implement Ravel's default skill directory in Ravel's code, on the
`skill-directory-delivery` line, before the 2.0.0 cut.

## Context

- Work in the Ravel workspace
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`. Read
  its own instructions first.
- The contract is the section "Default skill directory" of Ravel's
  `docs/specs/skill-discovery.md`, at Ravel commit `78d833d24364`. The
  decision record is "Installed skill sources are the default".
- The running log of `ravel-default-skill-directory-k218` holds the reasons
  and the rejected options.

## Done when

- `skills.sources` and `skills.mandatory` are optional. Without `sources`,
  the default directory supplies the sources. `"sources": []` disables the
  default. A mandatory reference names a source of the effective set.
- Ravel reads `share/ravel/installation.json` two levels above its canonical
  executable, only when the settings name no `sources`. An absent record
  gives no default directory. A malformed record refuses as
  `skill_directory_invalid`.
- Ravel lists the directory at each resolution, as the entry table states.
  Each refusing row refuses as `skill_directory_invalid`, stage `skills`,
  exit 3, and names the entry. A default source has the roots `skills` and
  `craft`, where each exists, in that order. Default sources follow the byte
  order of their IDs.
- The routing notice's `skills` object holds `defaultDirectory`.
- The specification's test-seam row "Default skill directory" passes, with a
  copied executable and a fixture directory of Homebrew-shaped links. A
  control for each refusing row has been seen to fail.
- Ravel's own checks pass. The commit is on the `skill-directory-delivery`
  bookmark.

## Notes

The formula channels that write the record, their installed tests and the
book chapters belong to `ravel-installation-record-k226`. This leaf's tests
write the record themselves.

The contract is a public 2.0 surface. If an executable seam cannot establish
a property, judge at the boundary whether a `review-impl` leaf is earned.
