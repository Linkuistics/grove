# stamp-explicit-k3

**Kind:** work

## Goal
Execute plan Task 2: an explicit --harness always persists to
.grove-stamps/<name>; single-harness auto-detection still writes nothing.

## Context
- Plan Task 2: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md

## Done when
tests/harness_stamp.rs (new) passes; maybe_stamp takes explicit: bool and
launch.rs passes args.harness.is_some(); `cargo test` green; one commit.
