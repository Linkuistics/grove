# 060-detail-preview

**Kind:** work

## Goal

The [[live preview]]: while Nav has focus, the detail widget follows the
*highlighted* grove (no harness spawn); Tab (or `l` on a grove row) moves
focus into detail; Esc there returns to Nav; seeds preview their pending
observations.

## Context

- `src/tui/app.rs` `rebuild_detail` keys detail off the focused *pane* —
  this leaf breaks that coupling while Nav (or Nav-originated Detail focus)
  is active, and restores it when focus returns to a pane.
- `src/tui/focus.rs` — Detail's Esc currently always goes to Pane; entering
  detail from Nav must return to Nav (track the entry origin in the focus
  state, keeping `arbitrate` pure).
- `src/tui/detail.rs` already renders from `GroveDetail`; a seed has
  `task_tree: None` — render the inbox-only state cleanly (it may already;
  verify with a seed fixture).
- `l` is fold-expand on header rows (030) and enter-detail on grove rows —
  row-kind-dependent dispatch.

## Done when

- Moving the nav cursor re-points detail live (preserving detail scroll only
  when the grove is unchanged, as `Detail::show` already does).
- Tab and `l` (grove row) focus detail; Esc returns to Nav; the pane-entered
  path still Esc's to Pane. Headless transition tests for both origins.
- A highlighted seed previews its observations; inbox grooming keys (`x`,
  `m`) work on a previewed grove without any pane open.
- Leaving Nav to a pane restores pane-coupled detail (today's behavior).

## Notes

Watch for fs-watch refresh racing the preview re-point: `refresh_fleet`
calls `rebuild_detail`, which must respect the Nav-preview override.
