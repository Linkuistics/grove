# 030-engine

**Kind:** planning

## Goal

Productionise the spike (`~/Development/rmux-spike/src/interactive.rs`) into a minimal
but *usable* rmux-backed `grove tui` — the milestone that brings the TUI back after the
020 rip-out. Grill the engine design, then build: the event-driven draw loop (010-plan
D3: `render_stream` + `PaneDriver` + tokio `select!` over render-updates/input/fs-watch),
`Rmux::connect_or_start`, one harness pane rendered via `ratatui-rmux` `PaneWidget` with
the hardware cursor placed from `snapshot.cursor`, crossterm→tmux-token input + a focus
model, plus a minimal nav + the centered capture modal (the proof-point for the bug that
motivated the whole migration).

## Context

This is where the inversion becomes real grove code, so it likely **decomposes** (draw
loop / daemon-session model / input + focus / minimal nav / capture modal / `grove tui`
entry). Below the presentation boundary (ADR-0013) the `RepoView`/`MultiRepoView` core is
unchanged — this leaf is the new presentation over it. Drafts the **landmark "rmux
substrate" ADR** (D4) as the architecture settles.

## Done when

`grove tui` runs on rmux: a harness pane renders + takes input, a centered capture modal
works over the live pane, and a minimal nav exists. Build/headless tests green (the
testability win — the spike's probes ran headless). Landmark ADR drafted.

## Open questions to grill

- Crate structure: a `grove-tui` presentation crate vs. in the existing binary? Where does
  the rmux glue (PaneDriver-per-pane, render-stream tasks, ring tap) live?
- Session/pane model: one rmux session per `grove tui` process? Pane addressing (slot vs
  `PaneId`). How the harness `grove do <name>` process maps to an rmux pane.
- Focus model: which pane/surface receives input; modal focus capture; the leader key
  (`Ctrl-o` historically) under a grove-owned loop where there's no zellij locked-mode.
- Input coverage beyond the spike's `forward_key`: mouse routing (automation-style
  `mouse().click/move_to`), bracketed paste, when grove handles a key as UI vs forwards it.

## Notes
