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
