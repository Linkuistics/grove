# codex-profiles-k10

**Kind:** work

## Goal
Execute plan Task 9: append sol-xhigh and sol-high profiles to
~/.codex/config.toml and smoke-test both via codex exec.

## Context
- Plan Task 9: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Machine config, not repo: nothing to commit here beyond retiring this leaf.

## Done when
Both `codex exec --profile ... "Reply with exactly: profile-ok"` runs return
profile-ok on the subscription; any flag-spelling deviation recorded in this
leaf file before retiring.

## Result
Both smoke tests passed (`profile-ok`, sub-billed, no API-key errors).

**Deviation:** the installed codex build's `--profile <name>` is
`CONFIG_PROFILE_V2` — it layers `$CODEX_HOME/<name>.config.toml` on top of
the base config, and *rejects* the plan's assumed `[profiles.<name>]` table
in `config.toml` outright (`Error loading config.toml: --profile
'sol-high' cannot be used while ... contains legacy 'profile = "sol-high"'
or '[profiles.sol-high]' config`). Wrote `~/.codex/sol-xhigh.config.toml`
and `~/.codex/sol-high.config.toml` instead, each with `model =
"gpt-5.6-sol"` + `model_reasoning_effort`; `config.toml` untouched. Plan
Task 9 and the spec (line ~166) updated in place to match — see this
leaf's commit.
