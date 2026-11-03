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

The specification is Ravel's `docs/specs/token-handoff.md`, first committed
as `7c5b6cdd3842`. Implement it as its review chain leaves it. The route uses
a token from `claude setup-token`, not the owner's Keychain sign-in. The human
mints that token and saves it with mode `0600`. No session runs
`claude setup-token` or reads the token file.

The book still states that Ravel holds no model credential, in
`01-what-ravel-is-for.md` and `06-reference.md`. Correct both with the
implementation. Add the owner's recipe and the reference of `tokenHandoffs`.
