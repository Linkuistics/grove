# 040-grove-integration

**Kind:** work (reshaped by 020 — built on the [[zellij substrate]], ADR-0015,
not the in-process-pty embed)

## Goal

Wire grove onto its **owned zellij substrate** (Strategy 1b): a **head binary**
launches zellij with grove's bundled config + bars-free layout (presenting as a
single binary); the **dashboard** is a native zellij pane running grove's own
ratatui (the evolving v1 `grove tui`); and from the dashboard the user
**launches/attaches a grove's live harness** (`grove do <name>` → claude code /
codex) as a native zellij pane and **switches focus between groves**. This closes
the 060 "Done when" — now via zellij, not an embedded pty.

## Context

- **Substrate decided (020 → ADR-0015): grove-owned zellij.** Harnesses and the
  dashboard are native zellij panes; grove does *not* emulate terminals. The
  in-process-pty `harness-pane` crate is the shelved fallback, not consumed here.
- **Head-binary model.** `grove`/`grove tui` exec's `zellij --config <grove.kdl>
  --new-session-with-layout <grove-layout.kdl> --session <…>`. Decide where the
  bundled config/layout live (ship in the binary and write to a cache dir, or a
  known config path) and how the dashboard pane re-enters grove (an inner
  subcommand/flag vs `grove tui` detecting it is the dashboard pane).
- **Driving zellij from outside (grove's shell-out idiom).** The dashboard opens a
  harness with `zellij action new-pane -- grove do <name>`, focuses with
  `zellij action focus-pane-id <id>`, closes with `close-tab-by-id` — tracking the
  stable pane IDs it created. This is the same write-via-shell-out discipline grove
  uses for `grove` verbs; no plugin, no WASM (that is the recorded 1a future
  refinement, not this leaf).
- **Tamed config knobs (validated on zellij 0.44.3, from the 020 spike):**
  `default_mode "locked"` (key passthrough — claude/vim run as bare),
  `pane_frames false`, `simplified_ui true`, `show_release_notes false`,
  `show_startup_tips false`, `copy_on_select true`, `session_serialization true`;
  a custom top-level `layout { … }` with **no** tab/status-bar panes; command
  panes need `start_suspended false` so the dashboard auto-runs. The throwaway
  spike at `/tmp/grove-zellij-spike/` is the starting reference.
- **Boundary (ADR-0013) unchanged:** the dashboard ratatui is *above* the seam;
  `RepoView`/shell-out writes stay *below* and `ratatui`-free. zellij sits
  below/around the presentation.
- **Free from the substrate (do NOT rebuild):** per-pane copy mode, scrollback,
  search, floating panes, session persistence — all native zellij. (This is why
  030 was retired.)

## Done when

- `grove`/`grove tui` launches the zellij substrate with grove's bundled config +
  layout and **presents as a single binary** (no visible "you are in zellij":
  bars/frames/branding hidden; dashboard auto-runs in its pane).
- From the dashboard, selecting a grove **opens `grove do <name>` as a native
  zellij pane** beside the dashboard (`zellij action new-pane`), and the user can
  interact with it normally (locked-mode passthrough).
- **Switching focus** between groves works from the dashboard via stable pane-ID
  addressing (`focus-pane-id`), and the dashboard remains reachable as the switch
  surface.
- Multiple groves' harnesses can be alive at once; closing one is clean
  (`close-tab-by-id` / `close-pane`).
- The unlock key (zellij control seam) is set to a deliberate, documented,
  non-colliding key (default `Ctrl g` collides with nvim "show file info" — pick
  grove's leader consciously).

## Notes

- Scope is **within one repo** (the v1 dashboard already lists a repo's groves).
  **Cross-repo** fleet is 070 — but note it opens cross-repo harness panes via the
  same `zellij action` driving, so 040's driving layer should not hard-assume one
  repo.
- Launching `grove do <name>` inside a zellij pane nests grove deliberately (the
  harness *is* a grove session) — intended, not a problem.
- 1a (WASM plugin dashboard via `zellij_widgets` + pipe IPC) stays a recorded
  future refinement; do not pull it into this leaf (ADR-0015, grove constraint 4).
- Packaging (bundle/vendor zellij vs depend-on-installed) can start as
  depend-on-installed for the within-repo cut; a bundling decision can come later
  (flag if it needs its own leaf/ADR).
