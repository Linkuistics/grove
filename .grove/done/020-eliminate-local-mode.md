# 020-eliminate-local-mode

**Kind:** work

## Goal

Make trellis the one, unconditional TUI: delete the legacy local dashboard and
remove the `trellis-seam` feature gate so a default build is trellis-capable.

## Context

The shipped binary builds with `trellis-seam` off (no `default = [...]` in
`Cargo.toml`; `release.toml` uses `cargo release` with default features; no
script passes `--features trellis-seam`). So `grove tui` takes the
`#[cfg(not(feature = "trellis-seam"))]` branch → `tui::run` → the single-repo
local dashboard that ignores `fleet.toml`. That accidental local-only ship is
the bug this grove fixes. See `010-plan.md` for the full rationale.

`grove tui` is convenience-only (real work is `grove do`; non-interactive view
is `grove status`), so no in-terminal fallback is owed.

## Done when

- `Cargo.toml`: `trellis-seam` feature removed; `trellis`, `trellis-client`,
  `trellis-server` are unconditional (non-optional) deps.
- No `#[cfg(feature = "trellis-seam")]` / `#[cfg(not(feature = "trellis-seam"))]`
  anywhere (grep is clean). Notably:
  - `src/cli.rs:305-314` — the `--server` re-exec intercept becomes
    unconditional.
  - `src/cli.rs:328-350` — the tui dispatch reduces to
    `crate::trellis_host::run_client(&repo_args)`.
- `src/cli.rs:48-57` — `TuiArgs.local` field removed; `--local` is gone (unknown
  flag if used). Update the `Tui` command doc-comment (`:39-45`).
- `src/tui.rs:55-66` — `pub fn run` (the in-terminal event loop) deleted.
  **Keep** `tui::dashboard_surface` (`:4496-4537`) and the `App`/ratatui
  rendering — trellis uses them as a host surface. Delete only what becomes dead
  once `tui::run` is gone (e.g. the standalone `ratatui::init`/`restore`
  event-loop wiring `run` used, if not shared).
- Build/release config carries no `--features trellis-seam` (none currently
  does — verify).
- Docs: `README.md`, `CONTEXT.md` (glossary entries mention the `--local`
  dashboard — e.g. "Whichkey bar", "Owned zellij substrate"), `content/`,
  `docs/workflows/`, `CHANGELOG.md` updated to the single-mode model.
- An ADR is written (see below).
- `cargo build` (default features) succeeds and `cargo test` passes.

## ADR to write

"trellis is the only TUI — local dashboard and the `trellis-seam` feature gate
removed." Amends ADR-0020/0021 (trellis as the supported path) by removing the
`--local` escape hatch and the build-feature gate. Rationale: the gate-off-by-
default shipped a local-only, fleet-blind binary; `grove tui` is convenience-
only so no fallback is owed; trade-off is build time / binary size (always
compile the forked zellij) for one coherent always-trellis binary.

## Notes

Coordinate with `030` (trellis config). Order doesn't strictly matter; this leaf
is the larger structural change.
