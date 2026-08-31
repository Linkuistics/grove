# release-ordinal-fs-tree-k114

## Goal

Deliver the completed ordinal-fs-tree walkthrough work: set the main bookmark
to its intended finished revision, push it, and make the requested minor
release using the repository's established release procedure.

## Context

The prior finish session externalised this work because branch publication and
release are outside grove teardown. The finish sentinel remains live after this
leaf and must not be torn down until this work is terminal.

## Done when

- The target revision, main bookmark, release scope, version, and release
  channel are verified from the repository and its current remote state.
- The main bookmark points at the intended revision and is pushed successfully.
- The minor release is prepared and published through the repository's approved
  procedure, or any missing authority or external failure is reported precisely
  without claiming a release.
- All resulting changes and evidence are committed with jj, and this leaf is
  retired.

## Notes

Do not use raw git for mutations. Treat publishing and any irreversible release
action as requiring the authority and safeguards documented by the repository.
