# 020-detail-surface

**Kind:** work

## Goal

Build the **per-grove detail [[host surface]]** and wire the nav to open each
grove's [[workspace]] tab with **detail beside harness**. This is 130's headline
acceptance (minus `$EDITOR`, which is 030).

## Context

- **Depends on 010** (the host-pane seam now opens N panes on demand) and **120**
  (the nav's `Enter`/`OpenHarness` opens the grove tab).
- **Reuse the v1 `App` detail rendering**, scoped to one fixed grove, detail-only:
  no grove list, no master/detail drill-in. The relevant v1 pieces: `DetailState`,
  `render_grove_detail`, `render_inbox_pane`, the capture modal
  (`CaptureModal`/`render_capture_modal`), the disposition picker
  (`DispositionModal`), and the task-tree flatten (`flatten_for`/`FlatRow`).
- **Triage/capture already run in-process** on the home `DashboardSurface`
  (`process_action`: `shell_capture`, `shell_drain`). The detail surface reuses
  those exact shell-outs; the only deferral is `$EDITOR` (→ 030).

## Done when

- Opening grove `<name>` shows **that grove's** task tree + inbox + capture in a
  native detail pane beside its `grove do <name>` harness pane.
- Triage (`d` → disposition) and capture (`c` → `Ctrl-S` submit) work in-process,
  status-lined like the home surface. `Ctrl-E` ($EDITOR) shows a "lands in 030"
  pointer for now (same shape as the home surface does today).
- **Two groves open at once each show their own detail, no cross-talk** — each
  `DetailSurface` is constructed bound to its grove when its tab opens; fs-watch
  ticks refresh the right surface.
- **Switching workspaces preserves each tab's detail** — trellis keeps tabs (and
  their host panes) alive; the surface state survives.
- `cargo build`/`cargo test` green; grove core stays `ratatui`-free below the
  ADR-0013 seam (the detail surface is *above* it, like the dashboard surface).

## Notes

- **Scoping vs. forking `App`:** prefer reusing `App` in a "detail-locked" mode
  (constructed straight into `Screen::GroveDetail` for one grove, list/filter
  navigation suppressed) over copy-pasting the render fns — keep one detail
  renderer. Decide the minimal shape when the code is in front of you; the v1
  `render`/`handle_key` already branch on `Screen`.
- **Per-grove fs-watch:** the home surface watches the whole repo; a detail surface
  only needs *its* grove's `.grove/` + the shared `.grove-meta/inboxes/<name>`.
  Narrower watch = less churn (root brief's `.git/`-noise concern). Reuse the
  110/030 fs-watch-thread + `request_tick` pattern, scoped down.
- **Lifecycle:** the detail surface is created by the 010 driver verb when the nav
  opens the tab. Closing the tab drops the surface (its fs-watch thread exits when
  the `notify` channel closes, as the home surface's does).
- Layout split ratio (detail sidebar vs harness) is a minimal choice here; the
  *responsive* layout is 150's. Pick a sane fixed split and move on.
