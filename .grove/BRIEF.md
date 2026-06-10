# rmux-tui-polish — brief

## Goal

Post-parity polish of the `grove tui` nav UX, picking up the two items deferred
when the rmux-substrate grove deliberately shipped a minimal nav (030-engine
leaf 030): **sort/filter as a discrete mode** and **inspecting a grove without
launching its harness**.

## Done when

- Nav sort + filter is a discrete sub-modal mode with clear enter/exit keys,
  defined sort orders and filter dimensions, per-session (ephemeral) state.
- A grove's detail (task tree / brief chain / inbox) can be read from the nav
  without spawning its harness session.

## Decomposition

Grown by 010-plan (grilling 2026-06-10; verdicts in its running log, now in
done/). Ordered by dependency — data, then model, then engine, then wiring:

- 020-recency-field — core last-commit timestamp per grove (recency sort's datum)
- 030-nav-model — grouped headers + folds, seeds listed, scrolling
- 040-filter-engine — pure fuzzy-rank + sort/toggle projection
- 050-filter-mode — `/` mode wiring: keys, layered Esc, footer, criteria line
- 060-detail-preview — live preview; Tab/`l` into detail; Esc→Nav; seed preview
- 070-seed-start — Enter on seed: confirm modal + `grove do` launch

## Pointers

- Glossary: CONTEXT.md §TUI — Fleet (filter dimensions already defined there),
  Nav surface, Detail widget, Focus, Leader.
- ADR-0028 (rmux inversion — grove owns the draw loop; detail is a grove-drawn
  widget, decoupled from the harness pane).
- Code: `src/tui/nav.rs` (flat list, live-only, no filter/sort/scroll),
  `src/tui/focus.rs` (pure `arbitrate` transition table),
  `src/tui/app.rs` `rebuild_detail` (detail currently follows the *focused
  pane's* grove — the coupling peek-detail must break),
  `src/tui/detail.rs` (pure snapshot → Buffer widget).

## Notes

Seeded from the retired rmux-substrate `bugs` grove, leaves 070-sort-filter-mode
and 090-inspect-without-harness (both incorporated at this grove's first drain).
