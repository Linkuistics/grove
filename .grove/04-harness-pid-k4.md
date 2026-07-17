# harness-pid-k4

**Kind:** work

## Goal
Execute plan Task 3: rename the session PID handle to GROVE_HARNESS_PID with a
one-release co-export + read-fallback of GROVE_CLAUDE_PID.

## Context
- Plan Task 3: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md

## Done when
The new resolve_opts fallback test passes; wrapper co-exports both names; doc
text in complete.rs/llm_cli.rs updated; `cargo test` green; one commit.
