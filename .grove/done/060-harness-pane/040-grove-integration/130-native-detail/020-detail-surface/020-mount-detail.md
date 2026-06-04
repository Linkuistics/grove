# 020-mount-detail

**Kind:** work

## Goal

Mount each grove's **detail** (task tree + inbox + capture) beside its harness in
the content region, on top of the `010` swap substrate — closing 130's headline
acceptance (minus `$EDITOR`, which is 030). The content slot becomes a **split**
(harness + detail), and the swap parks/restores the harness+detail **pair** as a
unit, two groves with no cross-talk.

## Context

- **Builds on `010`** (the constant-nav + content-slot layout, the `MountHostSurface`
  registry seam, the `HostDriver` swap verb, harness-into-slot park/restore proven on
  a single pane). `010` left the content slot holding the harness only; this leaf
  adds the per-grove detail host pane beside it.
- **Depends on 120** (nav surface + leader) — both done.
- **Reuse the v1 `App` detail rendering**, scoped to one fixed grove, detail-only:
  no grove list, no master/detail drill-in. The relevant v1 pieces: `DetailState`,
  `render_grove_detail`, `render_inbox_pane`, the capture modal
  (`CaptureModal`/`render_capture_modal`), the disposition picker (`DispositionModal`),
  and the task-tree flatten (`flatten_for`/`FlatRow`). Prefer reusing `App` in a
  "detail-locked" mode (constructed straight into `Screen::GroveDetail` for one grove,
  list/filter navigation suppressed) over copy-pasting render fns — keep one detail
  renderer.
- **Triage/capture already run in-process** on the home `DashboardSurface`
  (`process_action`: `shell_capture`, `shell_drain`). The detail surface reuses those
  exact shell-outs; the only deferral is `$EDITOR` (→ 030).

## Build

1. **`DetailSurface`** — a `HostSurface` wrapping a detail-locked `App` bound to one
   grove. Registered per-grove in the `010` keyed host-surface registry; mounted into
   the content region via `MountHostSurface { key }` when its grove is first selected.
2. **Content region = split.** The slot now holds harness (terminal) + detail (host)
   in a simple fixed split. The `010` swap operates on the grove's **content subtree**:
   one `replace_pane`+park per pane in the pair. Park/restore the pair together so
   switching away parks both alive and switching back restores both.
3. **Per-grove fs-watch.** Each `DetailSurface` watches *its* grove's `.grove/` + the
   shared `.grove-meta/inboxes/<name>` (narrower than the home surface's repo-wide
   watch — less `.git/` churn). Reuse the 110/030 fs-watch-thread + `request_tick`
   pattern, scoped down; ticks must refresh the **right** surface (no cross-talk).
4. **Triage + capture in-process.** `d` → disposition picker → `shell_drain`; `c` →
   capture modal → `Ctrl-S` → `shell_capture`; status-lined like the home surface.
   `Ctrl-E` ($EDITOR) shows the "lands in 030" pointer (same shape as today).

## Done when

- Selecting grove `<name>` in the constant nav mounts **that grove's** task tree +
  inbox + capture beside its `grove do <name>` harness in the content region.
- Triage (`d`) and capture (`c` → `Ctrl-S`) work in-process, status-lined; `Ctrl-E`
  shows the "lands in 030" pointer.
- **Two groves selectable in turn each show their own detail, no cross-talk** — each
  `DetailSurface` is bound to its grove when first opened, parked (alive) when another
  grove is selected, and fs-watch ticks refresh the right surface.
- **Switching back restores** a grove's detail + harness (the parked pair survives).
- `cargo build`/`cargo test` green; grove core stays `ratatui`-free below the
  ADR-0013 seam (the detail surface is *above* it, like the dashboard surface).

## Notes

- **Lifecycle:** a detail surface is created when the nav first opens a grove and
  parked, not dropped, when another grove is selected (harness + scrollback survive).
  Dropping happens only on explicit close.
- The detail/harness split here is a minimal fixed choice; the *responsive*
  multi-pane content region (terminal/yazi/lazygit, pack-vs-degrade, toggles) is
  150-working-set's.
- After this lands, update `HOST_API.md`'s "one host pane per session" deferral
  (ADR-0023 Consequences) to reflect the realised N-surface registry.
