# lifecycle-cutover-k39

**Kind:** impl

## Goal

Cut the human lifecycle over to one bare config-driven `grove` loop for live,
fresh, and legacy trees, with one authoritative selected leaf and a
mandate-bearing direct process launch.

## Context

- Depends on `session-config-integrate-k21`,
  `session-kind-migration-integrate-k29`, and
  `session-epoch-integrate-k37`.
- Binding design: the Solution, Validation, Authoritative selection,
  Single-command lifecycle, and Existing live tree sections of
  `docs/specs/config-driven-sessions.md`.
- Primary code surfaces: `src/main.rs`, `src/cli.rs`, `src/launch.rs`,
  `src/loop_driver.rs`, `src/provision.rs`, `src/tree_read.rs`, `src/herdr.rs`,
  plus isolated process-level driver tests with real config files and fake
  commands.
- Retain obsolete commands/routing only as dead compatibility code until
  `legacy-launch-removal-k46`; do not let it participate in the bare path.

## Done when

- Bare `grove` provisions independently, reports Herdr working, acquires the
  lease, resolves/version-checks its sibling `grove-llm`, fully validates
  config, performs at most one required root/migration/live transition, picks
  once, reloads config, and launches the selected filename kind.
- The exact absolute `grove-llm` path that passed the sibling version check is
  also embedded in generated Herdr turn-hook JSON; hook execution cannot drift
  to a different PATH-resolved helper.
- The selected value contains path, handle, and kind from one guarded read;
  `${prompt}` carries the embedded continue launcher plus that handle as an
  explicit mandate, and no routing forecast, second pick, target environment,
  or task-body metadata participates.
- Template expansion is executed directly with the worktree cwd. The driver
  injects no harness/model/naming/grant/hook/agent arguments or environment
  beyond its own loop-control channel; configured wrappers remain opaque.
- Pre-mutation invalid config or sibling-tool skew leaves rootless, legacy,
  current, empty, and pending-migration trees byte-identical. A config edit
  after a completed transition prevents launch while preserving resumable tree
  state; config reload between loop iterations is observable.
- Fake-command acceptance tests record exact argv, cwd, environment, prompt
  mandate, exit status, elapsed time, and signals for paths with spaces,
  non-final prompt, optional Herdr splice, spawn failure, launch-window insert,
  no-signal/nonzero exits, and sibling-versus-PATH hook resolution.
- `--help` and `--version` remain metadata-only. Empty-tree finish allocation is
  intentionally delegated to `finish-lifecycle-k43`; the current no-live path
  remains resumable until that next slice.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

This is the cutover point. Later contraction may delete old code only after the
new black-box seam is green.
