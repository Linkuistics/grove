# 030-pane-toggles

**Kind:** work (the toggle UX — keys + hints driving the 010 verb)

## Goal

Wire **per-pane show/hide toggles** for the working set, driven from grove's nav
(and whichkey hints), onto the 010 toggle verb. Each aux/detail member of the
selected grove's working set can be individually shown or hidden (park-alive
suppress/restore); the harness is always present.

## Context

- 010 built the framework verb (toggle one member: suppress alive + re-tile / show
  + re-tile). This leaf binds it to keys and surfaces the hints — the *grove-side*
  UX, no new framework mechanism.
- The nav is grove's leader-focused command surface; the whichkey bar is the single
  hint owner (140). Toggle keys are bound in `GROVE_TUI_CONFIG`'s locked mode (like
  the `Ctrl-o` leader) and/or handled by the focused nav surface, then drive the
  `HostDriver` toggle verb for the currently-mounted working set.
- Park-alive semantics (brief decision): a hidden member's child keeps running and
  capturing scrollback; re-showing restores it instantly with state intact.

## Done when

- Each working-set member (detail, terminal, yazi, vcs) has a key/affordance that
  toggles its visibility for the **currently-selected** grove; the harness stays.
- Hiding a member re-tiles the remaining visible members; showing restores it,
  child + scrollback intact (park-alive). The nav and whichkey are untouched.
- The whichkey bar shows the toggle hints when the relevant surface is focused;
  hints follow the single-owner model (140) — no second hint surface.
- Toggling is per-grove: a member hidden in grove A's set has no effect on grove B's
  set; switching groves restores each set's own visibility.
- `cargo build` / `cargo test` green.

## Notes

- Pick a small, legible keymap (e.g. a toggle leader + a per-member letter, or a
  numbered toggle) — **not** a tiling-config DSL (constraint 4). Exact keys are a
  build call; document them in the whichkey hints.
- Default-visible set on first mount is 040's concern; here a member just needs to
  toggle correctly from whatever its starting visibility is.
