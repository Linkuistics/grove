# confined-harness-trials-k128

## Goal

Start Claude Code and Codex under Ravel's confinement, and measure
`discovery-confined` and `missing-kind-confined` in both harnesses.

## Context

- The record sections `discovery-confined`, `discovery-confined in Codex`,
  `missing-kind-confined`, `missing-kind-confined in Codex` and "Missing kind:
  Codex and Claude Code".
- The triage section of `docs/common-loading-acceptance-record.md`.
- `scripts/harness_trial.py` and `scripts/harness-trials/`.
- Ravel's Seatbelt profile in `crates/keyed-launch/src/confinement.rs` of the
  `Ravel.skill-directory-delivery` workspace.
- `grove run --help`. It grants read-only runtime files and no writable
  location outside the invocation directory.

## Done when

- Claude Code: the trial wrapper sets `CLAUDE_CODE_TMPDIR` to the private run
  directory that Ravel names in `TMPDIR`. The record states whether the
  claude.ai sign-in works under a profile with no Keychain Mach lookup.
- Codex: `CODEX_HOME` names a writable directory inside the grants, and the
  sign-in arrives through a `--runtime-read` grant. The record states the
  Codex sandbox mode inside Ravel confinement.
- For `missing-kind-confined` in Codex, the record states how the native
  copies stay outside the grants while the native roots lie in `CODEX_HOME`.
- `discovery-confined` and `missing-kind-confined` run once in each harness,
  with the observations of their original leaves. Each harness starts under
  confinement, or the record names the exact blocker that no runner change
  can remove.
- The record states where a durable fix belongs: the owner's policy, Grove's
  `grove run`, or Ravel's profile.

## Notes

The original runs suggest that a real harness cannot start under a confined
`grove run` at all. If so, the confined standalone route fails for real
harnesses in production, not only in trials. State that consequence plainly.

A blocker that needs a change to Ravel's profile, such as a Keychain Mach
lookup, becomes a scope question to the human. Do not change the profile here.
An unconfined retry gives no credit to a confined row. Do not change the
owner's settings.
