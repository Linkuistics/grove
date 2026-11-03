# ravel-token-handoff-k132

## Goal

Design how Ravel gives a confined Claude Code its claude.ai sign-in without a
Keychain grant. Ravel reads the owner's sign-in before confinement. It hands
only the access token to the harness.

## Context

- The record sections "Diagnostics under a copy of the profile",
  "discovery-confined, repaired" and "Where a durable repair belongs" of
  `docs/common-loading-acceptance-record.md`.
- Ravel's `crates/ravel/src/confinement.rs` (`Confined::environment`) and
  `crates/keyed-launch/src/confinement.rs`, in the
  `Ravel.skill-directory-delivery` workspace.
- Ravel's `docs/specs/skill-discovery.md` and its session-execution and
  configuration specifications.
- Claude Code 2.1.317 reads `CLAUDE_CODE_OAUTH_TOKEN` and
  `CLAUDE_CODE_OAUTH_TOKEN_FILE_DESCRIPTOR`. Its Keychain item is the generic
  password `Claude Code-credentials`. On Linux it keeps `.credentials.json`.

## Done when

- A Ravel specification states the handoff. It names who declares it: the
  owner's settings or the configuration script. It states what Ravel reads,
  when it reads it, and how the harness receives it. The human chose this as
  Ravel work in this workstream, during confined-harness-trials-k128.
- The design hands the access token only, never the refresh token, so a
  confined refresh cannot rotate the owner's sign-in. It states the
  consequence: a run cannot outlast the token's lifetime.
- The design prefers a file descriptor to an environment variable, or states
  why not. It states what the agent can read: its own access token, and no
  other Keychain item. The profile gains no Keychain grant.
- The design states the macOS and Linux behaviour. It states a refusal with
  a remedy when the item is absent or expired. No unconfined fallback exists.
- The test seams name a check under the real profile that needs no model
  call, such as `claude auth status`, and its control without the handoff.

## Notes

The human proposed this: "Can ravel read the keychain etc and expose it to the
claude code instance?" The alternative profile change would let a confined
agent request any Keychain item that `security` can read.

This session could not read the owner's Keychain item: the permission layer
refused it as credential materialisation. The design leaf must not read the
secret either. The implementation's check runs with the human's approval.
