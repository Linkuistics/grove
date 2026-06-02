# 060-workspace-tabs

**Kind:** work

## Goal

Make each grove a zellij **tab** (a [[workspace]]) and the dashboard the "home"
tab: from the home dashboard, opening a grove creates its tab with the harness
running inside; `GoToTab` keybinds switch between home and grove tabs; the
controller tracks `{grove → tab}` so a second open focuses the existing tab
rather than duplicating. The tabs spine of ADR-0018 — switching that actually
works in locked mode.

## Context

- ADR-0018: grove = zellij tab; switch via `GoToTab`/`GoToNextTab` (bindable in
  locked mode, verified on 0.44.3), not the superseded `focus-pane-id`-between-
  dashboard-panes model. zellij keeps every tab's panes alive across switches.
- Reuses the **040** `zellij action` driver + `HarnessPanes` tracker (now in
  `done/`, code still in `src/harness_drive.rs`): first-open spawns the harness;
  `new-tab`/`new-pane --tab-id` place it; switching is `go-to-tab`.
- The home dashboard proxy (020/030) becomes the "home" tab. Pressing the open
  key on a selected grove in the home dashboard flows to the controller (it owns
  the proxy), which creates the tab — no plugin needed for this leaf.
- Config (leaf 030's `CONFIG_KDL`): add locked-mode `GoToTab`/`GoToNextTab`/
  `GoToPreviousTab` binds (e.g. `Alt 1..9`, `Ctrl ]`/`Ctrl [`). The leader rebind
  to `LaunchOrFocusPlugin` is **070**'s concern; until then `Ctrl-o` may stay or
  be a no-op.

## Done when

- From the home dashboard, opening the selected grove creates a zellij tab named
  for the grove with `grove do <name>` running in it; the controller tracks
  `{grove name → tab}`.
- A second open of the same grove **focuses** its existing tab (`go-to-tab`), no
  duplicate.
- `GoToTab` keybinds switch between home and grove tabs; panes stay alive across
  switches; closing a grove's tab is clean and the controller forgets it.
- Opens with an explicit repo/cwd so 070-fleet-view reuses the path cross-repo.

## Notes

- Depends on 010/020/030 (done) and reuses 040 (done). Working set here is just
  the harness; terminal/yazi/lazygit + responsive layout is **090**.
- The [[nav plugin]] (**070**) is the in-tab control surface; until it lands,
  switching is the `GoToTab` keybinds + the home dashboard.
- Keep all tab/pane decision state in the controller (ADR-0016 single source of
  truth).
