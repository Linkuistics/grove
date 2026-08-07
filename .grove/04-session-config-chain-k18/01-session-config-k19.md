# session-config-k19

**Kind:** impl

## Goal

Add the deep configuration module that loads and validates the complete
nineteen-kind KDL document and expands one selected command template into
direct-exec argv, without changing the active driver yet.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` sections
  "Configuration file" and "Module interfaces", plus
  `docs/adr/complete-session-configuration.md`.
- Primary code surfaces: `Cargo.toml`, `src/lib.rs`, and a focused new
  configuration module; keep harness-specific launch policy out of its
  interface.
- Add focused configuration tests rather than extending the already-large
  legacy driver fixtures. Preserve Rust 1.74 compatibility.
- This is the expand step. `lifecycle-cutover-k39` becomes the first production
  caller; legacy environment and harness routing remain untouched until then.

## Done when

- One load operation reads `~/.config/grove/config.kdl` into a complete map with
  every exact session kind once, and aggregate diagnostics cover missing,
  unknown, duplicate, malformed, and source-spanned KDL entries.
- Template validation implements POSIX shell-word parsing without a shell,
  literal non-empty word zero, required-once `${prompt}`, optional-at-most-once
  scalar substitutions, and whole-word zero-or-two `${herdr_settings}`.
- Expansion returns argv values with no reparsing and proves spaces and shell
  metacharacters cannot change argument boundaries; executable resolution and
  spawn remain caller responsibilities.
- Tests cover the specification's successful and failing examples, including
  prompt in a non-final position, words after the Herdr splice, literal `env`
  word zero, aggregate failures, and absence of shell evaluation.
- `cargo fmt --check` and the full `cargo test --locked` suite pass while the
  existing CLI behavior remains unchanged.

## Notes

Use the configuration interface itself as the test seam. Parser or source-span
types are implementation details and should not leak to driver callers.
