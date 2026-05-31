# 030-zellij-launch

**Kind:** work

## Goal

Make `grove`/`grove tui` the **head binary** that launches the zellij substrate
and presents as a single binary: embed grove's tamed config + bars-free layout,
write them to a cache dir at launch, start zellij **as a child** (the controller
persists alongside it — does not `exec` and vanish, ADR-0016), and have the layout
place the dashboard pane as `grove __dash-proxy --socket <path>` so it auto-runs.
Composite reads as grove, not "a zellij session."

## Context

- ADR-0015 + the 020 spike give the validated knobs: `default_mode "locked"`,
  `pane_frames false`, `simplified_ui true`, `show_release_notes false`,
  `show_startup_tips false`, `copy_on_select true`, `session_serialization true`;
  a custom top-level `layout { … }` with **no** tab/status-bar panes; command
  panes need `start_suspended false`. Spike reference: `/tmp/grove-zellij-spike/`
  (`config.kdl`, `layout.kdl`, `run.sh`).
- Node-brief decisions: **embed config+layout in the binary, write to a cache dir**
  (`$XDG_CACHE_HOME/grove/zellij/` → `~/.cache/grove/zellij/`) at launch; **depend
  on an installed zellij** for this cut (bundling later); **unlock key = `Ctrl-o`**
  (remap zellij's locked-mode unlock; document it).
- The controller (020) is already a persistent process owning the socket; this leaf
  wires the launch sequence: create socket path → start the controller's listener →
  spawn zellij child with `--config <cache>/config.kdl
  --new-session-with-layout <cache>/layout.kdl --session <name>` whose layout's
  dashboard pane runs `grove __dash-proxy --socket <path>` → wait on the zellij
  child → tear down (remove socket, optionally delete-session) on exit.

## Done when

- `grove`/`grove tui` launches into the zellij substrate with no visible bars,
  frames, or zellij branding; the dashboard auto-runs in its pane (no "press ENTER
  to run" suspend prompt).
- Config + layout are embedded assets written to the cache dir at launch (idempotent
  overwrite so a grove upgrade refreshes them); paths resolved via XDG with a
  `~/.cache` fallback.
- zellij runs as a child of the controller; quitting zellij returns control and the
  controller tears down cleanly (socket removed, no leaked session/threads).
- The unlock key is `Ctrl-o`, set in the bundled config, and documented (README /
  workflow doc as appropriate).
- Launching from a repo with no live groves still comes up cleanly (empty dashboard
  list), and from inside an existing grove worktree behaves sanely.

## Notes

- This leaf depends on 010 (the proxy exists) and 020 (the controller serves it).
- Keep the launch logic out of the data layer (ADR-0013): it's controller/transport
  wiring, not `RepoView`.
- Packaging is depend-on-installed zellij; if a missing/old zellij needs friendly
  handling, do the minimum (clear error) and flag a bundling leaf/ADR rather than
  solving vendoring here.
- Harness panes are 040 — this leaf only stands up the substrate + dashboard pane.
