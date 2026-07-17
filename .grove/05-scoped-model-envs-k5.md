# scoped-model-envs-k5

**Kind:** work

## Goal
Execute plan Task 4: GROVE_<HARNESS>_<KIND>_MODEL beats GROVE_<KIND>_MODEL;
selection helpers (KIND_SUFFIXES, env_suffix, model_for, any_model_env) land
for Task 5 to reuse.

## Context
- Plan Task 4: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md

## Done when
per_harness_model_env_beats_the_base_var passes (scoped beats base; another
harness's scoped var does not leak); `cargo test` green; one commit.
