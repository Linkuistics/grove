# reviewer-spawns-skill-k13

**Kind:** work

## Goal
Execute plan Task 12: add references/harness-spawns.md to the
doubt-driven-development skill + the SKILL.md pointer; commit in
~/Development/skills.

## Context
- Plan Task 12: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Commit ONLY in ~/Development/skills — the plugins/marketplaces mirror is
  disposable and silently resets.

## Done when
Reference + pointer committed in the skills repo with the plan's message; the
spawn commands match the ids/profiles that 10-12 actually configured.

## Result
Committed in `~/Development/skills` at `86facad` (main). The spawn commands
(`pi -p --model kimi-coding/k3`, `codex exec --profile sol-xhigh`) matched
plan Task 12 verbatim — no deviation. One stale cross-reference did need
correcting: the reference's closing note pointed at `~/.codex/config.toml`,
but leaf 10 (codex-profiles-k10) moved the sol-xhigh/sol-high profiles into
per-name files (`CONFIG_PROFILE_V2`); updated to name
`~/.codex/sol-xhigh.config.toml` / `sol-high.config.toml` instead, folded
back into the plan file in this leaf's commit.
