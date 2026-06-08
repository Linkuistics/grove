# 010-surfaces

**Kind:** planning

## Goal

Settle and grow the **full surface set** as plain ratatui widgets grove draws:
nav (landed 030), capture (landed 030), **per-grove detail** (new), and
**whichkey** (new). Decide what of the ADR-0019 "A′" UX (constant nav + swapped
per-grove content) survives the inversion, and how detail + whichkey slot into the
`Harness | Nav | Modal` focus model. Grow work leaves for the new surfaces.

## Context

030 established the pattern: grove's own surfaces are pure
`snapshot/state → Buffer` widgets (headless-tested), and focus is the leader-gated
`Harness | Nav | Modal` state machine (ADR-0028 E4, `src/tui/focus.rs`). Two
surfaces from the trellis-era design are not yet built and change character under
the inversion:

- **Per-grove detail** (task tree / inbox triage / capture entry-point) was a
  *dumb terminal proxy* (`grove __dash-proxy --grove`) rendered controller-side
  over a socket seam (ADR-0016/0019). Under rmux it is **a widget grove draws**
  from `RepoView` — no proxy, no seam, no `RunEditor` frame.
- **Whichkey** was an injected non-focusable host pane that other surfaces
  *published* hint lines to (ADR-0019, leaf 140). Under rmux it is **a line/bar
  grove draws directly** from the focused surface's state — grove already owns a
  `footer_line` concept in the capture work (ADR-0028).

## Areas to grill (questions, not answers)

- **A′ survival.** ADR-0019's "constant nav + per-grove detail in that grove's own
  region" was a *tab/proxy* realisation. Does the UX intent (nav always reachable,
  detail scoped per grove) survive as a layout (constant nav region + swapped
  content rect), or does owning the draw loop suggest a different shape?
- **Detail as its own focus state?** Does `Focus` gain a `Detail` variant, or is
  detail a non-focusable panel beside the harness? How do nav → detail → harness
  key flows compose?
- **Detail interactivity.** Read-only (task tree + briefs + inbox view) or
  interactive (inbox triage actions, capture launch)? Triage actions are
  shell-outs below the seam (the ADR-0028 E1 idiom) — which belong in detail vs
  deferred?
- **Whichkey: surface or footer?** Is whichkey a distinct surface concept or just
  the `App` drawing the focused surface's `footer_line`? What survives of the
  "single hint-owner" rule (ADR-0019) when there is no longer a separate pane to
  own it?
- **Relationship to the working set (020).** Detail is one panel; the working set
  (020) is the multi-pane layout it lives in. Settle detail's *content + focus*
  here; defer its *placement among aux panes* to 020 (flag the seam).

## Done when

The surface set, the focus-model extension (detail/whichkey), and the A′-survival
question are settled; work leaves for per-grove detail and whichkey are grown;
the ADR-0019 "what survives" verdict is recorded (feeds the 040 teardown).

## Notes
