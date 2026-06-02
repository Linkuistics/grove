# 080-controller-plugin-pipe

**Kind:** work

## Goal

Implement the **controller ↔ `grove-nav` pipe protocol** (the split-driving seam
of ADR-0018): the controller pushes the live grove/workspace list to the plugin
via `zellij pipe`; the plugin signals "open grove X" intent back; the controller
first-opens that grove's tab + working set via the 040 `zellij action` driver.

## Context

- ADR-0018; `zellij pipe` verified as the CLI↔plugin channel (bidirectional via
  the invocation's stdout). The controller already owns `RepoView` + fs-watch.
- **Controller → plugin:** on startup and on every fs-watch settle, the
  controller pipes the grove list (+ per-grove state, and the exact `grove do
  <name>` command + cwd) to `grove-nav` so the nav renders real, live data.
- **Plugin → controller:** the plugin handles already-open switching itself
  (`go_to_tab`); only *first-open* needs the controller (so `grove do` command
  composition stays in Rust). Mechanism: the controller holds a long-lived
  `zellij pipe --name grove-intent` reader (or equivalent) for intents the plugin
  emits.

## Done when

- `grove-nav` renders the real grove list piped from the controller and updates
  on fs-watch changes.
- Selecting a not-yet-open grove in the nav causes the controller to create its
  tab + working set (via the 040 driver); already-open groves switch via the
  plugin's own `go_to_tab`.
- The channel survives the plugin (re)launching (e.g. first `LaunchOrFocusPlugin`).

## Notes

- Depends on **060** (tabs) and **070** (the plugin). Wire format is this leaf's
  call — keep it lean (no `serde` unless it earns its place; mirror the
  hand-rolled [[seam frame]] codec discipline). Pins down the message shapes
  ADR-0018 left open.
