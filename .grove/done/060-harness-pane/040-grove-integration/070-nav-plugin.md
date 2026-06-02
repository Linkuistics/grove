# 070-nav-plugin

**Kind:** work

## Goal

Build the **`grove-nav` zellij WASM plugin**: the [[leader]]-focused command
surface. It renders the grove/workspace list, receives keys while focused, and
drives pure-zellij navigation (switch workspace via `go_to_tab`, focus a pane,
toggle a pane). Wire the leader: `Ctrl-o` → `LaunchOrFocusPlugin "grove-nav"
{ move_to_focused_tab true }`. Subsumes the old `050-mode-discoverability` — the
plugin *is* the live mode/key surface.

## Context

- ADR-0018 (Strategy 1a pulled forward). Verified plugin API: `Event::Key` fires
  for every key while the plugin pane is focused (no permission); `Event::Visible`
  on tab show/hide; `LaunchOrFocusPlugin` focuses a running instance by name.
- New artifact: a Rust→WASM crate (`wasm32-wasi*`) using `zellij-tile`, built and
  **bundled** like the config/layout — embed + write to the cache dir at launch
  (mirror `src/zellij.rs`'s `CONFIG_KDL`/`LAYOUT_TEMPLATE` handling), referenced
  from the layout / `LaunchOrFocusPlugin` by a stable path.
- The plugin holds **no grove state**. It renders what the controller pipes it
  (the live grove list); the pipe protocol is **080** — this leaf may start with
  a placeholder/static list and integrate the pipe in 080.

## Done when

- The plugin builds to wasm and loads in grove's substrate; `Ctrl-o` from any
  pane focuses it (replacing the dead-end `Ctrl-o`→`Normal` binding).
- It lists workspaces (home + groves), lets you switch (`go_to_tab`) and jump
  home, and shows **live key hints / current mode** — closing the 050 gap.
- Selecting/acting returns focus to the prior pane/tab; locked mode is otherwise
  untouched (keys still pass to the focused app).

## Notes

- Depends on **060** (tabs exist to switch between). The controller↔plugin pipe
  is **080**; toggling working-set panes becomes meaningful once **090** adds
  them.
- Permissions: `RunCommands` only if the plugin itself spawns panes; per ADR-0018
  the controller does grove-data first-open, so the plugin mostly *navigates*.
- Build/bundle choreography mirrors the embedded config/layout (lazy: smallest
  plugin that renders the list + handles keys + drives `go_to_tab`).
