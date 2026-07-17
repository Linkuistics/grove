# harness-registry-k2

**Kind:** work

## Goal
Execute plan Task 1: add the pi harness row, switch codex model_args to
--profile, add Harness.skills_dir, derive known_names() from the registry.

## Context
- Plan Task 1 (follow steps + code verbatim, TDD order included):
  docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Plan Task 0 first if the branch state is unclear: this worktree IS the
  codex-pi-harness branch — no new branch needed, work here.

## Done when
Task 1 steps all checked off in the plan file; tests/harness.rs additions and
the reworked codex argv assertion pass; `cargo test` green; one commit in the
plan's message style.
