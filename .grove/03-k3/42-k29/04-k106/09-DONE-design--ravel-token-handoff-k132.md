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

## Decisions (running log)

**Conflict with a recorded decision.** The decided route makes Ravel read the
owner's Claude Code sign-in and pass on its access token. Two Ravel records
forbid that: the decision record
`a-configuration-script-asks-a-model-through-a-harness-run.md` and the
"Credentials" section of the configuration specification. The section states:
"Ravel holds no model credential. It reads no API key and no sign-in, and it
stores and sends none." Their reason is Anthropic's terms.

<!-- tc: depart long-sentence because the sentence quotes Anthropic's terms verbatim -->
The current
[legal and compliance](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use)
page states: "developers may not collect, store, or intermediate Claude.ai
credentials or session tokens — sign-in to a Claude account must complete
through Anthropic's own flow."

**A sanctioned token exists.** The
[authentication](https://code.claude.com/docs/en/authentication#generate-a-long-lived-token)
page documents `claude setup-token` for scripts. It prints a one-year token
for `CLAUDE_CODE_OAUTH_TOKEN`. That token can make only model requests.

**Descriptor evidence.** In the 2.1.317 executable, Claude Code reads a
socket descriptor directly, and then it deletes the variable. It opens any
other descriptor as `/dev/fd/N`, and it reads directly only after `EACCES` or
`EPERM`. Under a copy of Ravel's profile, with dummy text, a read of
`/dev/fd/3` fails with `ENOENT` for a pipe and for a socket. A direct read of
descriptor 3 succeeds for both.

A handoff through a descriptor must therefore use a socket. Claude Code
writes its well-known token file only when `CLAUDE_CODE_REMOTE` is set.

**Scope question to the human.** Three routes remain:

- A. Ravel reads the owner's sign-in, as decided.
- B. Ravel hands an owner-minted `setup-token` through a socket. The token
  comes from a file that the settings name.
- C. The owner's wrapper reads a `--runtime-read` grant of that file. Ravel
  does not change.

The recommendation is B. The leaf stays live until the human answers.

**The human's answer: route B.** On 2026-11-03 the human answered "B, I
agree". The owner mints a token with `claude setup-token` and saves it in a
file that Ravel's settings name. Ravel hands that token to the confined
harness through a socket. Ravel never opens a harness's sign-in store.

The human mints and saves the token. No session reads it.

**The specification.** Ravel's `docs/specs/token-handoff.md` states the
handoff, in Ravel commit `7c5b6cdd3842` on the bookmark. The settings declare
the file, the descriptor variable and an `expires` date. The selection, or a
harness-run request, names the handoff. Ravel delivers it only under
`--confine`. Every declared token file is a protected path.

Ravel checks the file after the selection: the date, the file type, the
owner, the mode and the size. Then it reads the file once and sends the token
through a socket. A failed check refuses as `token_handoff_unavailable`, exit
3, and launches nothing. The decision record
`a-confined-harness-receives-an-owner-minted-token` holds the rejected routes.

**Review.** The specification is a landed contract for a credential, and
ravel-token-handoff-k133 builds on it. This leaf cut
`review-design` ravel-token-handoff-k138 before k133, with its doubts in the
body. The launch has no `GROVE_PROVIDER_EVIDENCE`, so the review has no
Creator line.
