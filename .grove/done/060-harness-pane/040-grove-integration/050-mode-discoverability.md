# 050-mode-discoverability

**Kind:** planning

## Goal

Decide how the user discovers **which zellij mode they are in** and **what keys
are live**, once they unlock the substrate with `Ctrl-o`. Today, after unlocking,
there is *zero* on-screen feedback: `Ctrl-o` then `Ctrl-s` silently puts you in
scroll/search mode with no mode label and no key hints. Settle the approach
(grilling), record any ADR, and decompose into build work.

## Context

- Surfaced during 030's live verification (the `Ctrl-o` remap now works — the
  user immediately hit the discoverability gap). It is a **design tension**, not
  a 030 bug: 030's "Done when" deliberately hid all zellij chrome
  (`pane_frames false`, no status/tab bars, bars-free layout) so the composite
  "reads as grove, not a zellij session" (ADR-0015/0016). The status bar we
  removed is exactly what normally shows mode + key hints — so discoverability
  was the price of chrome-free.
- **The reconciliation question is real:** fully-chrome-free was an explicit
  030/ADR-0016 stance. Re-introducing any persistent indicator is a partial
  reversal and may warrant an ADR amendment.

## Design axes to grill

1. **Where does the indicator live?**
   - *zellij `compact-bar`* — a native one-line status bar that already shows the
     current mode and key hints. Cheapest, works globally (covers harness panes
     too), but reintroduces a sliver of zellij chrome and zellij's own styling
     (tension with "reads as grove").
   - *grove-rendered status line* — a line drawn by the controller in the
     dashboard's own ratatui (ADR-0016: grove renders, proxy is dumb). On-brand,
     fully grove-styled — **but only visible on the dashboard pane.** When focus
     is on a *harness* pane and you unlock, you are not looking at the dashboard,
     so this alone may not solve the global case.
   - *which-key style transient hint* — a popup/overlay of live bindings shown
     only while unlocked. Could be zellij-side (floating pane/plugin) or
     grove-side.
2. **What to show:** just the current mode? mode + a which-key of live bindings?
   only when unlocked (keep locked mode pristine)?
3. **Global vs per-pane:** the mode is a *global* zellij state; a dashboard-only
   line can't reflect it when a harness pane is focused. Does that force a
   zellij-level indicator (compact-bar / plugin) for the global case, with the
   grove status line reserved for dashboard-local info?
4. **1a WASM-plugin tie-in:** ADR-0015 recorded the `zellij_widgets` plugin
   dashboard (1a) as a future refinement; a status/which-key plugin is adjacent.
   Decide whether this leaf pulls any of that forward or stays 1b.

## Done when

- The approach is settled (grilling): indicator location, content, and
  global-vs-per-pane handling; the chrome-free-vs-discoverability tradeoff is
  resolved and, if it reverses an ADR-0015/0016 stance, an ADR amendment is
  raised.
- Decomposed into the concrete build leaf/leaves (or recorded as not-needed).

## Notes

- Keep the ADR-0013 boundary: any grove-rendered status line is controller-side
  ratatui above the seam; the proxy stays dumb.
- Independent of `040-harness-driving` (pane open/focus/close) — can sequence
  after it; discoverability matters *more* once multiple harness panes exist.
- Do not let this balloon into "build a full bar framework" — grove constraint 4
  (lazy/optional): the smallest thing that makes unlocked mode legible.
