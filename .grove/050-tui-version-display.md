# 050-tui-version-display

**Kind:** work

## Goal

Add cli + repo versions to the TUI in a top header bar, and append
per-row `worktree=` (with drift marker) to each grove row. Match the
`grove status` rendering rule so the same data tells the same story
across surfaces.

## Context

- `src/tui.rs:710` — `render`; current Rect layout has body + footer.
  Will gain a header Rect above body.
- `src/tui.rs:785` — `render_grove_list`; renders the list of grove
  rows.
- `src/tui.rs:808` — `grove_row`; the per-row line builder. Adds
  `worktree=…` (+ trailing `⚠ repo=…` on drift).
- `src/tui.rs:830` — `render_grove_detail`; the split-pane detail
  screen. The header (rendered by `render`) is visible here too.
- `src/tui.rs:965` — `render_footer`; not touched (footer keeps the
  keyhint role).
- `crate::status` (post-[[020-extend-status-with-worktree-versions]]) —
  reuse the per-grove worktree-version helper added there.
- Decisions from [[010-shape-the-feature]] driving this leaf:
  - TUI placement: option I (header bar).
  - Per-row `worktree=…`; drift styled (e.g. yellow/red) and trailing
    `⚠ repo=…`.
  - Same string-equality drift rule; same `(unknown)` for missing
    `VERSION.md`.

## Done when

- `render` (`src/tui.rs:710`) allocates a header Rect above the body,
  drawn on both `Screen::GroveList` and `Screen::GroveDetail`.
- Header shows `cli=X.Y.Z · repo=A.B.C` per installed harness; on cli
  vs repo drift, the affected token is styled to draw the eye.
- `grove_row` (`src/tui.rs:808`) appends `worktree=W.X.Y`; on drift,
  appends `⚠ repo=A.B.C` in the same drift-style colour.
- Manual TUI run (against a worktree with a mismatched `VERSION.md`)
  shows the drift in both the header and the row, as appropriate.
- A worktree with no `VERSION.md` renders as `worktree=(unknown)` —
  not styled as drift.
- No new dependency; reuse `ratatui` primitives already in use.

## Notes

- Multi-harness repos: the header shows one row per harness (or a
  compact `cli=… · repo[claude-code]=… · repo[codex]=…` form — judgment
  call during implementation, prefer whichever stays under typical
  terminal width).
- Tall vs short terminals: the header costs one row. On a 24-row
  terminal that's a real tax on the grove list; acceptable per the
  Q9 grilling.
- No ADR — placement is a UI choice, easy to revise.
