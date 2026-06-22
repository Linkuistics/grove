# 22. Harness UX is a constant nav + a swapped content region (no tab-per-grove)

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record. (Prior status: accepted —
  **mechanism superseded by [ADR-0028](0028-rmux-substrate.md); UX intent survives**,
  rmux substrate, 2026-06-10, 070-teardown D4; per the 050-plan-rebuild/050-working-set
  verdict.)
- Date: 2026-06-03
- Deciders: Antony Blakey (with grove 060/040/130 decompose)
- Supersedes: the **tab-per-grove realisation** of ADR-0018 (grove = zellij tab,
  `GoToTab` switching) and the **per-workspace-tab** realisation of ADR-0019
  (detail lives in each grove's tab). The *UX they settled* — one grove's working
  set at a time, per-grove detail beside the harness, a leader-focused nav — is
  **carried forward**; only "a grove is a tab, switched by `GoToTab`" is replaced.
- Builds on: ADR-0020 (native fork — grove renders in-process) and ADR-0021
  (library-you-link host API; the host pane is grove's third pane kind).

> **rmux-substrate verdict (070-teardown, D4; per 050-plan-rebuild/050-working-set).**
> The *UX* survives: a harness-dominant layout, an aux side column, and a width
> breakpoint (now a single ~220-col breakpoint governing column **geometry only**).
> Two *mechanisms* dissolve: the constant-nav **region** pin (already retired in
> 010-surfaces — nav is reachable via the leader, not pinned on screen) and
> **tier-as-mount-time-membership** (membership is now user-driven lazy-toggle plus
> always-on detail, not chosen by tier at mount). Annotated, not blanked.

## Context

ADR-0018 made each grove a zellij **tab** and the nav a **home tab** you switch
*away from*; ADR-0019 put each grove's detail inside that grove's tab. The native
fork (ADR-0020/0021) then dissolved the WASM/proxy mechanism those ADRs assumed,
but left the **tab** model standing. Reviewing it against the in-process reality
surfaced two problems:

1. **The nav is not constant.** As a home *tab*, the nav is invisible while a
   grove is focused — you leave it to reach a grove and `GoToTab`/leader back to
   it. The user wants the nav **always on screen**: a persistent switcher beside
   the live grove, not a place you travel to and from.

2. **"Pin a tiled pane across all tabs" is not a native zellij capability.**
   `set_pinned`/`toggle_pinned` are no-ops for tiled panes; pinning exists only
   for *floating* panes (`floating_panes.has_pinned_panes()`). So a constant nav
   *sidebar* cannot be had by pinning a tiled nav across tabs — the tab model
   actively fights the constant-nav requirement.

A third fact removes the only reason tabs were load-bearing: **a pty stays alive
and keeps capturing scrollback whether or not it is displayed.** trellis runs one
reader thread per terminal pane, pumping child output into that pane's vt100 grid
+ scrollback continuously, independent of layout. zellij relied on this so
non-active *tabs* survive; the aliveness is a property of the **pane/pty**, not of
the tab. Tabs were the mechanism for "keep N harnesses alive and switch between
them" — but that aliveness does not require tabs.

## Decision

**The harness UX is one persistent layout: a constant nav surface + a content
region the nav swaps the selected grove's working set into.**

- The **nav is always on screen** (a native [[host surface]] — ADR-0021 — not a
  home tab). It is the switcher: selecting a grove swaps that grove's harness +
  detail into the content region. The leader (`Ctrl-o`) focuses the nav from the
  content region; there is no "home tab" to travel to.
- **A grove's working set lives in the content region**, not a tab — its detail
  ([[host surface]]) beside its harness (terminal pane). Selecting another grove
  **parks** the current grove's panes (alive, off-screen) and **mounts** the
  selected grove's.
- **Non-selected harness ptys keep running and capturing scrollback** — the free
  consequence of per-pane pty threads. Switching groves never suspends a harness.
- **`GoToTab`/`Alt-1..9` tab switching is retired** as the grove switcher (the
  `GROVE_TUI_CONFIG` binds go); switching is a nav-driven content swap.

**The mechanism for park/mount is deferred to a build-discovery spike** (leaf
`130-native-detail/010`): zellij's native `suppressed_panes` primitive vs a
grove-managed pane pool. That spike lands its own ADR; this ADR fixes only the
*model* (constant nav + swapped content, no tab-per-grove), not the realisation.

## Consequences

- **ADR-0018/0019 mechanism is superseded; their UX survives.** One-grove-at-a-
  time, per-grove detail beside the harness, leader-focused nav, scroll/copy/search
  free-wins — all carried forward. "Grove = tab, `GoToTab`" is replaced by
  "grove = a swappable working set in the content region."
- **130-native-detail reshapes.** Its `010` shifts from "N host panes in N tabs"
  to "the constant-nav + content-swap substrate" (the park/mount spike); `020`'s
  detail surface mounts into the content region; `030`'s `$EDITOR` pane is unchanged
  in intent.
- **120-native-nav (done) is partially reframed, not un-retired.** Its surface +
  leader + `RepoView` piping survive; its "Enter opens a workspace tab via
  `new_command_tab`" behaviour becomes "Enter swaps the grove's working set into
  the content region." The delta lands in the reshaped 130 leaves, not by
  re-opening 120.
- **150-working-set is unaffected in spirit:** the working set (harness + terminal
  + yazi + lazygit, toggleable + responsive) is now what fills the content region
  for the selected grove, rather than a tab's contents.
- **Simpler model, more framework ownership.** No tab bar, no `GoToTab` semantics,
  no home-tab round-trip — at the cost of building (or adopting) a park/mount
  mechanism trellis does not expose as a one-liner. The spike (010) sizes that.

## Notes

- The glossary's [[Workspace]] (grove = tab) and [[Working set]] entries are
  updated to the constant-nav + content-region model; the term *workspace* is kept
  for "one grove's swappable working set" but no longer means a zellij tab.
- This is the third reshaping of the harness UX in this node's history
  (ADR-0015 → 0016 → 0018 → 0019 → 0020 → here). Each was forced by a fact the
  previous model assumed away; the constant-nav requirement + the pinning/aliveness
  facts are this one's. The artifacts-over-state model (root brief) means none of
  the superseded code is load-bearing for resilience — switching realisations is a
  presentation change, not a data-model one.
