# 030-grove-integration

**Kind:** work

## Goal

Make grove **consume** `crates/harness-pane`: from the dashboard, launch/attach
a grove's live harness (`grove do <name>` → claude code / codex) in a pane
beside the dashboard, render it live, type into it, and **switch focus between
groves**. This is where the crate becomes the headline feature; it closes the
060 "Done when".

## Context

- **v1 TUI** is `src/tui.rs` (sync event loop, `ratatui 0.29` + crossterm,
  master/detail dashboard). This leaf adds harness-pane consumption *above* the
  ADR-0013 seam — `tui.rs` already sits above it; grove **core** modules stay
  `ratatui`-free.
- **Sync loop integration (050 evidence):** drain each `PtySession`'s output via
  `try_recv` between `event::poll` ticks; the startup-burst max backlog was 4
  chunks/tick, so the existing sync loop absorbs it. No async (that's 080).
- **Dashboard-as-switcher (040 layout intent):** the dashboard is the
  navigation surface; the user picks the focused grove *there*, not via a
  multiplexer. Native ratatui splits — no tmux join/break choreography.
- **The one cross-seam coupling (ADR-0014):** focus → mouse capture. grove owns
  the real `execute!(EnableMouseCapture/DisableMouseCapture)`; the crate only
  reports `wants_mouse()`. grove re-evaluates this on **every focus change**.
- Depends on **010** (the crate). Benefits from **020** (scrollback/copy) but
  can land a first cut before it.

## Done when

- From the dashboard, selecting a grove **launches `grove do <name>`** in a
  `PtySession` and renders its `TerminalEmulator` in a pane beside the
  dashboard (native ratatui layout).
- Keystrokes route to the **focused** harness; its output drains live in the
  sync loop; multiple groves' harnesses can be alive at once (parallel work).
- **Switching focus** between groves works from the dashboard, and **every
  focus change re-evaluates `EnableMouseCapture`** from the focused pane's
  `wants_mouse()`.
- The **native hardware cursor** is positioned from the focused pane's cursor
  (`f.set_cursor_position`), with tui-term's drawn cursor hidden — one cursor,
  the app's colors.
- The dashboard remains reachable as the switch surface (not "zoom a harness
  forever").

## Notes

- Scope is **within one repo** — the v1 dashboard already lists a repo's groves,
  so "switch between groves" is in scope here. **Cross-repo** fleet is 070.
- Keep all pty/vt100 below the seam: grove calls *into* the crate, never the
  reverse; the only data-up/command-down flow is the mouse-capture toggle.
- Launching `grove do <name>` inside a pane nests grove deliberately (the
  harness *is* a grove session) — that's the intended model, not a problem.
