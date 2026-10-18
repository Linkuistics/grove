# codex-always-on-k12

## Goal

Make the skill always on in Codex as far as Codex allows, as the design maps it:
the instruction, the reminder, the document gate and the reply log wherever Codex
has a hook for them, and the skill's own instruction to run the checker where it
has none. The one switch turns it all off.

## Context

- Read the design `docs/specs/technical-communication.md` and `harness-hooks-a.md`
  for Codex's hook events, configuration and pitfalls, and reuse the shared
  scripts and the switch from `claude-code-always-on-k11` through the thin
  adapter pattern it set.
- Skills reach Codex through Grove's startup provisioning, which embeds a
  snapshot of `plugins/` in the Grove binary (`docs/specs/codex-skill-provisioning.md`),
  and through `plugins/install.sh`. Executable flags on supporting scripts are
  preserved. Check that anything this leaf adds under `plugins/` reaches Codex by
  both routes.
- The route is the spec's: a hooks plugin whose root is the skill directory
  (`.codex-plugin/plugin.json`, with its entries in `harnesses/codex/hooks.json`
  and a Codex adapter in `tc_hook.py`). The human adds and trusts it once by
  hand. Never write trust into the human's configuration. For the live run, use
  a scratch `CODEX_HOME`, where trust through the TUI or
  `--dangerously-bypass-hook-trust` is acceptable for the test only. Check that
  the entries still reach the scripts if Codex copies the plugin.
- This repository never writes Codex's global `AGENTS.md`
  (`skill-design-k5`). `publication-k16` offers the human an optional line.
- A project memory records that Codex's sandbox blocks writes to the git
  directory. Check what it does to a hook that writes the reply log.

## Done when

- In a real Codex session, show by running it which layers work and which fall
  back to the model running the checker, and that the switch turns them off.
  Put the evidence in the running log, with the exact commands the human runs to
  add and trust the plugin, for `publication-k16`'s user document.
- `bash scripts/check.sh` is green, and the provisioning path still passes its
  checks.

## Notes

- Until the human trusts the plugin, Codex has the skill and the fallback.
  Say so plainly.
