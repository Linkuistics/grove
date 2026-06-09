# 010-pane-keying

**Kind:** work

## Goal

Refactor the pane addressing model from **grove-name** to a composite **(grove,
role)** key, so several panes can coexist per grove. Foundation for the side
column (020) and aux panes (030). No new panes or layout yet — this leaf only
generalises the addressing; behaviour is unchanged (one harness pane per grove,
role = harness).

## Context

Today `App.panes` is `HashMap<String, PaneEntry>` keyed by grove name (or
`"shell"`), `self.focused: String` names the displayed pane, and each
`PaneEntry.target: Option<CaptureTarget>` carries the grove for capture
(`src/tui/app.rs`). One pane per grove is baked into the key. The working set
needs N panes per grove, so the key must distinguish roles.

## Done when

- A `PaneRole` (or equivalent) distinguishes `Harness | Term | Yazi | Vcs` (and
  the bare-`shell` fallback). The pane map is keyed by a composite **(grove,
  role)** value; `self.focused` becomes that composite type, not a bare `String`.
- The capture target plumbing still resolves the **grove** from the focused key
  (capture/reject/move/detail all target the grove, regardless of which role pane
  is focused) — i.e. the grove is recoverable from the composite key, and the
  `"shell"` fallback (no grove) still works.
- `open_or_focus`, `rebuild_detail`, `submit_capture`, `reject_selected`,
  `begin_move`/`commit_move`, `select_initial_process`, and the render/resize
  paths are updated to the new key. The render channel's tag (`driver.rs`
  `spawn_render_task`) carries the composite key so snapshots route correctly.
- `Focus::Pane` is unchanged (it is already pane-agnostic — `self.focused` says
  which); no new focus variant.
- All existing TUI tests pass; the focus-table tests are untouched (pure, no key
  dependency). Add a unit test that two panes under the same grove (e.g. harness
  + a synthetic second role) coexist in the map and route their snapshots
  independently.

## Notes

- Keep it a pure refactor: do **not** spawn aux panes here (that is 030). A clean
  way to prove the keying without aux tools is a test-only second role, or simply
  the type-level change with the harness as the sole live role.
- Mind the borrow-shortening pattern already used (`focused_pane()` clones the
  cheap `Pane` handle to avoid holding a `self` borrow across `.await`).
