# guardrail-plugin-trial-k213

## Goal

Prove the second plugin of PersonalSkills in real Claude Code sessions, in an
isolated Claude Code home. The `guardrail` plugin alone lists `guardrail` and
no other skill. The `/guardrail` command and its hook work from that plugin.
A native installation of the ordinary plugin alone installs `guardrail` as
its dependency. Repair the pack layout if a reading fails.

## Context

- `docs/specs/pack-membership.md`: the paragraph on `personal-pack-k183`
  below "The acceptance of each move".
- `docs/specs/owner-skill-delivery.md`: "Native residue".
- writing-hooks-trial-k210 and its section of the moves record, as the model
  of a native trial with `claude -p`, stream JSON and hook events.
- The published pack commit of personal-pack-repository-k212, with its
  plugin names and its dependency decision.
- The section "How it works" of `guardrail/SKILL.md` and the cases of
  `guardrail-hook.test.sh`, for a destructive command that the hook gates.

## Done when

Each run uses a new Claude Code home that links only the sign-in store. A
scratch marketplace in that home lists both plugins from the published pack
commit.

- **The `guardrail` plugin alone.** The plugin listing shows `guardrail`
  alone. A session that runs `/guardrail` and then attempts a gated command
  shows the hook's `ask` decision in the hook events. The gated command is
  harmless in the scratch directory, for example `rm -rf` of a scratch file.
- **The control.** The same session without `/guardrail` shows no decision
  from the hook for the same command. The skill is user-invoked, so the
  hook must not run before the command.
- **The ordinary plugin alone.** An installation of the ordinary plugin
  installs the `guardrail` plugin as its dependency. The skill listing shows
  the three craft skills and one `guardrail`.
- If a reading fails, repair the layout in the pack, publish the repair and
  run the trial again. Record each repair and its reason.
- The moves record names the Claude Code version, the pack commit, each
  reading and each named gap with its disposition. A gap is never a pass.
- Grove's `bash scripts/check.sh` passes after the record changes.

## Notes

The node brief asks for the command and the hook. The membership decision
asks only for the listing, and leaves the command to the staging trials.
This leaf follows the node brief, which is stricter.

If personal-pack-repository-k212 recorded no dependency, drop the third
reading and record the reason.

The membership decision asks for no Codex trial. The skill declares
`harnesses: [claude-code]`, and its hook is inert elsewhere.

A real run spends subscription usage. Do not install, update or remove a
plugin in the owner's own Claude Code home. A nested session that the
auto-mode classifier refuses in `bypassPermissions` can run in `acceptEdits`
with a narrow tool set, as k210 recorded.

personal-removal-k214 pins the commit that this leaf accepts.

## Decisions (running log)

- The trial reuses the k210 recipe: `claude -p` 2.1.318 in an `env -i`
  home that links only `Library/Keychains`, with stream JSON and hook events.
  A scratch marketplace `scratch` has two `git-subdir` entries at pack
  commit `ac98636608b0`. The scripts are in the session scratchpad.
- One session must run `/guardrail` and then the gated command, so the
  session reads two user messages through `--input-format stream-json`.
  A resumed second process would not show that the hook lives for the
  session.
- The gated command is `rm -rf` of a scratch directory in the home's work
  directory. The runs allow `Bash` and `Skill` and nothing else.
- The run of the `guardrail` plugin alone passed at the first attempt. In
  print mode Claude Code turns the hook's `ask` into a `permission_denied`
  event with the reason type `hook`. The typed `/guardrail` resolves to
  `/guardrail:guardrail`.
- `claude plugin details` estimates the skill at about 236 always-on
  tokens. The session's model-facing skill listing omits `guardrail`.
  The record names the estimate as an observation, not a gap of the pack.
- The control passed: no hook start, and the allowed command ran. The run
  of `personal` alone passed: one dependency `guardrail`, the three craft
  skills and one `guardrail`, and the same `ask` from the dependency.
- Every reading passed at `ac98636608b0`. The pack needs no repair, and this
  leaf accepts that commit for `personal-removal-k214`.
