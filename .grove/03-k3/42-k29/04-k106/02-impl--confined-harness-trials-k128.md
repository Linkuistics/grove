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

## Decisions (running log)

**Runner seam.** Ravel scrubs a confined harness's environment, so the
wrapper sets the names inside the launch. `--tmp-env NAME SUB` and
`--tmp-link SUB TARGET` act under Ravel's `TMPDIR`, which lies in the private
run directory that Ravel removes after the run. The authority control admits
`CLAUDE_CODE_TMPDIR` and `CODEX_HOME` only with a value inside the child's
own `TMPDIR`. Runner commit `e4dda1043365`.

**Claude Code sign-in: a profile blocker.** A diagnostic under a copy of
Ravel's profile, with no model call, showed the claude.ai sign-in only when
the profile allows both the Mach services `com.apple.SecurityServer` and
`com.apple.securityd.xpc` and reads of `~/Library/Keychains`. Either one
alone fails. The same home unconfined is signed in. This is a scope question
to the human. The profile stays unchanged.

**Codex sandbox inside Ravel: `danger-full-access`.** The diagnostic showed
`sandbox-exec: sandbox_apply: Operation not permitted` for Codex's own
Seatbelt inside Ravel's. Under `workspace-write`, every command would fail.
Ravel's confinement is then the only sandbox of a confined Codex launch.

**Codex home inside the grants.** `CODEX_HOME` is `$TMPDIR/codex-home`, with
`auth.json` a link to the owner's sign-in. The `--runtime-read` grant of the
sandbox link admits that file read-only. `codex login status` reported the
ChatGPT sign-in under the profile copy. Codex warns that it does not make its
helper links under `TMPDIR`, and continues.

**Codex TLS roots: a runner repair.** The first repaired Codex launch,
`discovery-confined-20261103T062127Z`, started and then failed every request.
The diagnostic showed that Codex loads its TLS roots through the Security
framework, which needs `com.apple.SecurityServer`. `CODEX_CA_CERTIFICATE`
with the system bundle removes that need under the unchanged profile. Runner
commit `e5d57a7409d2`. Grove's staged release-notes helper already makes the
same three Codex choices.

**Results.** `discovery-confined` in Codex passes the confined observation.
`missing-kind-confined` in Codex is a gap with a new cause: Grove's standalone
prompt holds no kind load and no missing-guidance rule, so the agent completed
and Grove published. Both Claude Code runs start and stop at "Not logged in".

**Scope questions to the human.** The Claude Code sign-in under confinement,
and the missing-guidance rule of a standalone run. Each needs a decision
outside this leaf. The leaf stays live until the human answers.
