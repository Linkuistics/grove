# grove-always-starts-in-local-mode — brief

## Goal

Eliminate grove's legacy "local mode" so `grove tui` always runs the trellis
multiplexer. The grove's name is the bug it fixes: the shipped binary builds
with `trellis-seam` off and therefore always starts in the legacy single-repo
local dashboard, which ignores `~/.config/grove/fleet.toml`. Make trellis the
one, unconditional TUI and have it own its config (not the user's zellij config).

## Done when

- `grove tui` launches trellis unconditionally — no `--local`, no `tui::run`
  in-terminal loop, no `trellis-seam` feature gate.
- A default `cargo build` ships a trellis-capable binary (so `fleet.toml` is
  surfaced).
- The trellis path does not read `~/.config/zellij` / `$ZELLIJ_CONFIG_DIR`.
- Docs/CHANGELOG/glossary reflect the single-mode model.

## Decomposition

- `020-eliminate-local-mode` (work) — remove local dashboard + `trellis-seam`
  feature gate; make trellis unconditional; docs + ADR.
- `030-trellis-ignore-user-zellij-config` (work) — trellis config is
  grove-owned only.

See `010-plan.md` (planning leaf, retired after this session) for the full
grilling record and the turnaround that inverted the original premise.

## Pointers

- Dispatch / flags: `src/cli.rs:328-350` (tui), `:305-314` (`--server`
  intercept), `:48-57` (`TuiArgs`).
- Local path to delete: `src/tui.rs:55-66` (`tui::run`). Keep
  `tui::dashboard_surface` (`src/tui.rs:4496-4537`) — trellis host surface.
- Feature gate: `Cargo.toml:84-105` (`trellis-seam`).
- Trellis config base (zellij-config bug): `src/trellis_host.rs:171-179`,
  `GROVE_TUI_CONFIG` at `:241-256`.
- Fleet manifest read (works in trellis): `src/tui.rs:4516`, `src/fleet.rs`.

## Notes

Memory: the trellis fork owes upstream nothing — reshaping its config sourcing
to grove's use-case is exactly in-bounds.
