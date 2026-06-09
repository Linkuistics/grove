# 020-side-column-layout

**Kind:** work

## Goal

Generalise `composed_layout` from "harness + single detail column + footer" to
**harness (dominant left) + a side column stacking N visible members + footer**,
with the ~220-col breakpoint governing side-column geometry. Pure ratatui `Rect`
math, headless-tested. Renders detail + (eventually) aux-pane slots; the aux panes
themselves spawn in 030.

## Context

`composed_layout` (`src/tui/app.rs`) today returns `{ pane, detail, footer }` and
`detail_column_width` caps the detail column at 48 (34% of content). The side
column must instead hold a *stack* of members: detail (top, always) + zero-or-more
aux slots. The breakpoint (Q4) sets column width + per-member min-height; it does
**not** decide membership (that is user-toggle, 030).

## Done when

- A layout fn returns the harness `Rect` + an ordered list of side-column member
  `Rect`s (detail first, then a slot per visible aux member) + the footer `Rect`.
  Members stack vertically with equal (or weighted) shares; "let it tile" on
  overflow (no scroll machinery — Q4).
- The ~220-col breakpoint sets side-column **width** (wider/larger cap when
  ≥~220; ≈today's narrower column below) and a **per-member min-height**. Exact
  numbers are tuning — settle "harness keeps the dominant share, column wide
  enough for one foreign TUI".
- `render_surface` (the `Focus::Pane | Detail` and `Focus::Modal` arms) draws the
  harness into its `Rect`, then iterates the side-column members rendering detail
  via `Detail::render` and each aux slot via `render_pane` (aux panes are empty
  until 030 — render whatever is in the map, or nothing).
- `pane_viewport` / the resize paths size the **harness** to its (now smaller)
  dominant `Rect`, and size each visible aux pane to **its** slot `Rect`
  (resize-on-show — a hidden member has no slot this frame). Detail (a widget) is
  not resized.
- Headless tests cover: stack order (detail top), member count → slice count,
  the breakpoint's two width regimes, dominant-harness invariant, and
  degrade-without-panic on a tiny area (extend the existing `composed_layout`
  tests).

## Notes

- Detail is always the top member and never toggles off (Q5) — the stable anchor.
- This leaf can land with zero aux panes visible (the side column = detail only,
  i.e. today's behaviour re-expressed through the general stack) and still be a
  complete, testable increment. 030 adds real aux members on top.
- Builds on 010's composite keying to address which aux panes are present, but the
  layout math itself is key-agnostic (it takes a count/ordered list of members).
