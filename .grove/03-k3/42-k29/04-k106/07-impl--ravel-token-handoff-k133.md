# ravel-token-handoff-k133

## Goal

Implement the sign-in handoff that ravel-token-handoff-k132 designs, on the
unpushed Ravel bookmark `skill-directory-delivery`.

## Context

- The specification that ravel-token-handoff-k132 writes.
- The `Ravel.skill-directory-delivery` workspace and the Ravel Taskfile.
- `scripts/harness-trials/builds.json` and the dual-Ravel stage record.

## Done when

- Ravel implements the handoff as specified, with its tests and the
  applicable Ravel checks passing.
- A check under Ravel's real profile, with no model call, shows
  `claude auth status` signed in with the handoff and not signed in without
  it. The human approves the check before it reads the owner's sign-in.
- The new build is in `scripts/harness-trials/builds.json` and in the record,
  by commit, build identity and `bin/ravel` SHA-256.

## Notes

Do not publish a release, install `ravel@2` or change the owner's settings.
loader-report-wording-k129 also moves the bookmark. Build on the bookmark as
it then stands.
