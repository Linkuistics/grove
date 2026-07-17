# neutral-text-k8

**Kind:** work

## Goal
Execute plan Task 7: harness-neutral wording in cli.rs MODEL_ENV_HELP (now
documenting scoped + routing vars and per-harness launch flags), SKILL.md, and
the two rustdoc headers.

## Context
- Plan Task 7: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Clean-cutover prose: current scheme on its own terms, no "formerly claude".

## Done when
The two greps in Task 7 step 4 come back clean (only the sanctioned compat
survivors); `cargo test` green; one commit.
