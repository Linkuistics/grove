# legacy-command-surface-removal-k77

**Kind:** impl

## Goal

Remove the obsolete human command and persisted harness-stamp surfaces while
leaving the bare configured driver as a complete, working CLI.

## Context

- Depends on `finish-lifecycle-integrate-k45`.
- Primary surfaces: `src/cli.rs`, `src/harness_stamp.rs`, human-command help and
  process fixtures, and `.grove-stamps/` ignore rules.
- Preserve dead routing internals except where compilation requires a narrow
  adjustment; `routing-policy-removal-k82` owns their behavioral contraction.

## Done when

- Human `do` / `migrate` / `retire`, `--harness`, `--no-launch`, and dry-run
  routing are absent; bare `grove`, `--help`, and `--version` are the complete
  human CLI.
- Harness-stamp creation and lookup, stamp fixtures, and `.grove-stamps/`
  repository rules are removed.
- Bare-driver process tests, `cargo fmt --check`, and `cargo test --locked`
  pass.

## Notes

This is independently useful product behavior: users see one lifecycle entry
without waiting for internal routing cleanup.
