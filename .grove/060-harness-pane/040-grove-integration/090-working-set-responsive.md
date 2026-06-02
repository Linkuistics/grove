# 090-working-set-responsive

**Kind:** work

## Goal

Flesh out each grove tab's [[working set]] — harness + a plain terminal + yazi
(files) + lazygit (→ lazyjj) — with **individual show/hide toggles** (driven from
the [[nav plugin]]) and a **responsive layout** that packs everything on a large
display (5K2K) and degrades gracefully to a MacBook Pro screen.

## Context

- ADR-0018. Leaf 060 stood up the harness-only tab; this adds the terminal +
  yazi + lazygit panes, the per-pane toggles, and the responsive behaviour.
- Each aux pane runs in the grove's worktree cwd. Toggling a *specific* pane
  isn't a native zellij concept (zellij's float-toggle is all-or-nothing), so a
  toggle is open/close (or float/embed) of that one pane — driven from the nav
  (it has the API + the data).
- Responsive: define sensible default-visible sets per terminal size here (e.g.
  harness+terminal+yazi+git tiled on a large screen; harness + on-demand aux on a
  laptop).

## Done when

- A grove tab shows harness + terminal + yazi + lazygit laid out sensibly for the
  current screen size; the default visible set adapts to terminal size.
- Each pane toggles individually from the nav; panes run in the grove's worktree.
- lazygit works now; the vcs pane is not hard-wired to git (lazyjj later).

## Notes

- Depends on **060/070/080**. Keep it the smallest thing that's legible (grove
  constraint 4) — not a general tiling-config system. The responsive
  defaults/breakpoints are this leaf's design call.
