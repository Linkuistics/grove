# provision-sweep-k7

**Kind:** work

## Goal
Execute plan Task 6: provision the embedded skill for every installed harness
(primary unconditionally); symlink entries replaced as links (never deleted
through); foreign dirs refused; load_prompt reads the launching harness's copy.

## Context
- Plan Task 6: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- CONTEXT.md "Global skill provisioning" still describes the single-target
  ~/.claude model — reconcile that glossary entry in this same commit.

## Done when
The three new provisioning tests pass (symlink replacement proves the target
survives; foreign-dir bail; HOME sweep with pi's agent/ nesting); glossary
entry reconciled; `cargo test` green; one commit.
