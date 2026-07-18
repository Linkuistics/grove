# changelog-verify-k9

**Kind:** work

## Goal
Execute plan Task 8: the v12.0.0 changelog entry (breaking: codex --profile,
GROVE_HARNESS_PID, multi-harness provisioning; added: pi harness, routing +
scoped envs; fixed: explicit stamp), then full verification.

## Context
- Plan Task 8: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md

## Done when
`cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`
all clean; changelog committed.
