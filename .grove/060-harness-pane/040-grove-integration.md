# 040-grove-integration

**Kind:** work (reshaped by 020 — built on the [[zellij substrate]], ADR-0015;
dashboard architecture per ADR-0016)

## Goal

Wire grove onto its **owned zellij substrate** (Strategy 1b) under the
**controlling-process / dumb-proxy** model (ADR-0016): a persistent **controlling
process** launches zellij (as a child) with grove's bundled config + bars-free
layout (presenting as a single binary) and owns *all* logic, state, and rendering;
the **dashboard** appears in a zellij pane as a [[dashboard proxy]] (dumb terminal)
that the controller renders into; and from the dashboard the user
**launches/attaches a grove's live harness** (`grove do <name>` → claude code /
codex) as a native zellij pane and **switches focus between groves**. This closes
the 060 "Done when" — via zellij, with logic centralised in the controller.

## Context

- **Substrate decided (020 → ADR-0015): grove-owned zellij.** Harness panes are
  native zellij panes; grove does *not* emulate terminals. The in-process-pty
  `harness-pane` crate is the shelved fallback, not consumed here.
- **Dashboard architecture (ADR-0016): controlling process + dumb proxies.** The
  controller renders every dashboard surface and ships ratatui diffs over a local
  IPC channel (unix socket assumed) to a thin `grove __dash-proxy` running in the
  pane; the proxy only reports its size (+ SIGWINCH), blits controller output to
  stdout, and forwards stdin up. The proxy holds **no** state/logic/ratatui. Build:
  the controller (logic + per-proxy ratatui render target + zellij-action driving),
  the dumb proxy client, and the protocol (size up, frames down, input up).
- **Controlling-process model.** `grove`/`grove tui` starts zellij as a **child**
  and persists for its lifetime (not `exec`-and-vanish — the controller must
  outlive the launch to serve proxies). Decide where the bundled config/layout
  live (ship in the binary → cache dir, or a known config path) and how the
  dashboard pane is launched as the proxy (`grove __dash-proxy --socket <path>`,
  placed by the layout and by `zellij action new-pane` for extra surfaces).
- **Driving zellij is the controller's job (grove's shell-out idiom).** The
  *controller* (which owns the decisions, fed by proxied input) opens a harness
  with `zellij action new-pane -- grove do <name>`, focuses with
  `zellij action focus-pane-id <id>`, closes with `close-tab-by-id` — tracking the
  stable pane IDs it created. Same write-via-shell-out discipline grove uses for
  `grove` verbs; no plugin, no WASM (the recorded 1a future refinement, not here).
- **Tamed config knobs (validated on zellij 0.44.3, from the 020 spike):**
  `default_mode "locked"` (key passthrough — claude/vim run as bare),
  `pane_frames false`, `simplified_ui true`, `show_release_notes false`,
  `show_startup_tips false`, `copy_on_select true`, `session_serialization true`;
  a custom top-level `layout { … }` with **no** tab/status-bar panes; command
  panes need `start_suspended false` so the dashboard auto-runs. The throwaway
  spike at `/tmp/grove-zellij-spike/` is the starting reference.
- **Boundary (ADR-0013) reinforced (ADR-0016):** all of grove — data layer,
  writes, *and* dashboard ratatui rendering — lives in the controlling process;
  the dashboard proxy is pure presentation transport. The seam becomes a runtime
  client/server split, and a future web client is just another proxy.
- **Free from the substrate (do NOT rebuild):** per-pane copy mode, scrollback,
  search, floating panes, session persistence — all native zellij. (This is why
  030 was retired.)

## Done when

- `grove`/`grove tui` launches the zellij substrate with grove's bundled config +
  layout and **presents as a single binary** (no visible "you are in zellij":
  bars/frames/branding hidden; dashboard auto-runs in its pane).
- **The dashboard pane is a dumb proxy** (`grove __dash-proxy`): the controlling
  process renders the dashboard and ships frames to it; the proxy carries no grove
  state/logic/ratatui. Resize (SIGWINCH) and input round-trip correctly.
- From the dashboard, selecting a grove makes the **controller open `grove do
  <name>` as a native zellij pane** beside the dashboard (`zellij action
  new-pane`), and the user can interact with it normally (locked-mode passthrough).
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
