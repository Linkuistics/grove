# writing-hooks-trial-k210

## Goal

Prove the native residue in real Claude Code sessions, in an isolated
Claude Code home. The skill-free hooks plugin runs the hooks and lists no
native `technical-communication` skill. A native installation of the
ordinary writing plugin alone installs the hooks plugin and runs the hooks.
Repair the pack layout if a reading fails.

## Context

- `docs/specs/pack-membership.md`, the row "A second plugin in one
  repository" of "The acceptance of each move".
- `docs/specs/owner-skill-delivery.md`: "Native residue".
- `docs/adr/the-off-file-is-the-only-switch.md`, or the moved record that
  writing-craft-repository-k208 names.
- design-native-namespace-k200 and its section "Design native namespace" of
  the moves record, as the model of a native trial in an isolated home.
- The published pack commit of writing-hooks-plugins-k209, with its plugin
  names and its hook count.

## Done when

Each run uses a new Claude Code home that links only the sign-in store. A
scratch marketplace in that home lists both writing plugins from the
published pack commit.

- **The hooks plugin alone.** The plugin listing shows the hooks plugin and
  no `technical-communication` skill. A session shows the effect of each
  hook that can run without a write. Examples are the session-start text
  and a reply record in the log.
- **The ordinary plugin alone.** An installation of the writing plugin
  installs the hooks plugin as its dependency. A session shows the same hook
  effects once each. The skill listing shows one `technical-communication`
  skill, from `craft/`.
- **The off file.** The same session with the off file present shows no
  hook effect. This is the control that the hook effects come from the
  hooks.
- If a reading fails, repair the layout in the pack, publish the repair and
  run the trial again. Record each repair and its reason.
- The moves record names the Claude Code version, the pack commit, each
  reading and each named gap with its disposition. A gap is never a pass.
- Grove's `bash scripts/check.sh` passes after the record changes.

## Notes

The membership decision asks for no Codex trial. The Codex hooks plugin is
absent from the owner's setup, and the switch keeps it absent.

A real run spends subscription usage. Do not install, update or remove a
plugin in the owner's own Claude Code home. The always-on layer of this
session comes from that home's plugin cache.

The removal leaf, writing-craft-removal-k211, pins the commit that this
leaf accepts.

## Decisions (running log)

- The trial drives `claude -p` directly, with `--output-format stream-json`,
  `--verbose` and `--include-hook-events`. The readings are hook events and
  the reply log, not a Grove task, so `scripts/harness_trial.py` does not
  apply. The scripts are in the session scratchpad, and the record restates
  their recipe.
- The auto-mode classifier refused a nested session in `bypassPermissions`.
  The runs use `acceptEdits`, the tools `Agent` and `Write` only, and
  `--add-dir` for one gate directory.
- The write gate skips the temporary roots, and the scratchpad is under
  `/private/tmp`. The gate file is therefore in Grove's ignored `target/`.
- The scratch marketplace takes both writing plugins from GitHub at pack
  commit `4cafb0fd8e72`, and `grove` from GitHub at `GROVE_COMMIT`
  `31ec9117c9da`, which the hook count also reads.
- The off-file run reuses the home of the writing-plugin run, so that the
  off file is the only difference.
- Every reading passed at `4cafb0fd8e72`. The pack needs no repair, and this
  leaf accepts that commit for `writing-craft-removal-k211`.
- The owner's `known_marketplaces.json` changed during the runs. A
  model-free probe showed that the scratch plugin commands do not write it.
  The record names the change as an observation with its disposition.
