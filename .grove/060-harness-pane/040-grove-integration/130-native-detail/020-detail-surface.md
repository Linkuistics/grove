# 020-detail-surface

**Kind:** work

## Goal

Build the **per-grove detail [[host surface]]** and wire the nav so selecting a
grove **mounts** that grove's detail beside its harness in the **content region**
(ADR-0022). This is 130's headline acceptance (minus `$EDITOR`, which is 030).

## Context

- **Depends on 010** (the content-swap substrate + park/mount mechanism + the
  `HostDriver` swap verb, per ADR-0023) and **120** (the nav surface + leader).
  **Concrete mechanism (ADR-0023):** swap = in-place `TiledPanes::replace_pane` on
  the content slot (incoming pane inherits the slot geom; displaced pane parked in
  the tab's `suppressed_panes`, kept alive by the existing pty/resize routing).
  The detail host pane is created on demand via a keyed **host-surface registry +
  an id-only `MountHostSurface` instruction** — *not* a surface carried in a
  `ScreenInstruction` (the enum is `Clone+Debug`). `HostDriver` gains the swap
  verb. The `GoToTab`/`Alt-1..9` binds in `GROVE_TUI_CONFIG` retire here.
- **Reuse the v1 `App` detail rendering**, scoped to one fixed grove, detail-only:
  no grove list, no master/detail drill-in. The relevant v1 pieces: `DetailState`,
  `render_grove_detail`, `render_inbox_pane`, the capture modal
  (`CaptureModal`/`render_capture_modal`), the disposition picker
  (`DispositionModal`), and the task-tree flatten (`flatten_for`/`FlatRow`).
- **Triage/capture already run in-process** on the home `DashboardSurface`
  (`process_action`: `shell_capture`, `shell_drain`). The detail surface reuses
  those exact shell-outs; the only deferral is `$EDITOR` (→ 030).

## Done when

- Selecting grove `<name>` in the constant nav mounts **that grove's** task tree +
  inbox + capture beside its `grove do <name>` harness in the content region.
- Triage (`d` → disposition) and capture (`c` → `Ctrl-S` submit) work in-process,
  status-lined like the home surface. `Ctrl-E` ($EDITOR) shows a "lands in 030"
  pointer for now (same shape as the home surface does today).
- **Two groves selectable in turn each show their own detail, no cross-talk** —
  each `DetailSurface` is constructed bound to its grove when first opened; parked
  (alive) when another grove is selected; fs-watch ticks refresh the right surface.
- **Switching back restores** a grove's detail + harness (the parked state
  survives, per the 010 mechanism).
- `cargo build`/`cargo test` green; grove core stays `ratatui`-free below the
  ADR-0013 seam (the detail surface is *above* it, like the dashboard surface).

## Notes

- **Scoping vs. forking `App`:** prefer reusing `App` in a "detail-locked" mode
  (constructed straight into `Screen::GroveDetail` for one grove, list/filter
  navigation suppressed) over copy-pasting the render fns — keep one detail
  renderer. The v1 `render`/`handle_key` already branch on `Screen`.
- **Per-grove fs-watch:** a detail surface only needs *its* grove's `.grove/` + the
  shared `.grove-meta/inboxes/<name>`. Narrower watch = less churn (root brief's
  `.git/`-noise concern). Reuse the 110/030 fs-watch-thread + `request_tick`
  pattern, scoped down.
- **Lifecycle:** the detail surface is created when the nav first opens a grove
  (via the 010 swap verb) and parked, not dropped, when another grove is selected —
  so its harness + scrollback survive. Dropping happens only on explicit close.
- The detail/harness split *within* the content region is a minimal fixed choice
  here; the *responsive* content layout (and the terminal/yazi/lazygit panes) is
  150's.
