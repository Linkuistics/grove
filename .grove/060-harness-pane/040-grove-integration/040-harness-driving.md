# 040-harness-driving

**Kind:** work

## Goal

Let the user run grove harnesses from the dashboard: the controller drives zellij
via `zellij action` to **open** `grove do <name>` as a native pane, **focus/switch**
between live harnesses by stable pane ID, and **close** a harness cleanly — with the
dashboard staying the switch surface. This closes the 060 headline feature.

## Context

- ADR-0015/0016: driving zellij is the *controller's* job (it owns the decisions,
  fed by proxied input), using grove's shell-out idiom — `zellij action new-pane
  -- grove do <name>`, `zellij action focus-pane-id <id>`, `zellij action
  close-tab-by-id <id>` / `close-pane`. The 020 spike proved stable-ID addressing
  works headlessly.
- The dashboard already selects a grove (existing `App`/`handle_key`). This leaf
  adds: a key/action that opens the selected grove's harness, controller-side
  tracking of `{grove name → pane id}`, and switch/close actions — surfaced in the
  dashboard's keymap and status line.
- Harnesses are native zellij panes running `grove do <name>` (nesting grove
  deliberately — intended). grove does not emulate them.

## Done when

- From the dashboard, selecting a grove and invoking "open harness" makes the
  controller `zellij action new-pane -- grove do <name>` beside the dashboard; the
  user interacts with it normally (locked-mode passthrough).
- The controller tracks the created pane IDs (`{name → id}`) so a second invocation
  on the same grove **focuses** the existing pane rather than opening a duplicate.
- Switching focus between groves works from the dashboard (`focus-pane-id`); the
  dashboard remains reachable as the switch surface (a way back to it).
- Multiple harnesses can be alive at once; closing one is clean
  (`close-tab-by-id`/`close-pane`) and the controller forgets its id.
- The driving layer **does not hard-assume one repo** — it opens `grove do <name>`
  with an explicit repo/cwd so 070's cross-repo fleet reuses it unchanged.

## Notes

- Depends on 010/020/030 (seam, controller loop, live substrate).
- `pane id` discovery after `new-pane`: confirm the reliable way to learn the new
  pane's id (e.g. `--name` + `dump-layout`, or whatever the 020 spike's recipe
  was) and record it; this is the one mechanical unknown.
- Within-repo only here; 070 is the cross-repo fleet (it reuses this driving layer).
- Keep pane-decision state in the controller (single source of truth, ADR-0016),
  not in any proxy.
