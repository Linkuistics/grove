# 040-responsive-layout

**Kind:** work (the responsive defaults — closes the node)

## Goal

Choose the **default-visible working-set members by terminal size** at mount, and
the **wide arrangement** (harness-dominant + side stack), so the working set packs
on a large display (5K2K) and degrades gracefully to a MacBook Pro screen. Closes
the 150 node and the 060 headline feature.

## Context

- 010 (variable membership + toggle), 020 (the real aux tools), 030 (toggle UX)
  are in place. This leaf decides *what shows by default* at each size and *how the
  visible members are arranged* — the responsive policy on top of the mechanism.
- **Two-tier breakpoint** (brief decision): wide (ultra-wide / 5K2K) shows **all**
  members; laptop (MacBook-class) shows **harness + detail**, with terminal / yazi
  / vcs hidden (toggle-on via 030). The tier is chosen from the content region's
  size at mount; the user can always toggle from the default.
- **Wide arrangement** (brief decision): harness takes the largest share; detail +
  terminal + yazi + vcs stack in a column (or columns) beside it.

## Done when

- On a wide content region the default working set is all members in the
  harness-dominant + side-stack arrangement; on a laptop-sized region the default is
  harness + detail, the rest hidden but toggle-able (030).
- The tier is decided from terminal/content size at mount (a clear, documented
  breakpoint — two tiers, not a continuum); resizing across the breakpoint behaves
  sensibly (re-tile is fine; no requirement to auto-flip the default set on live
  resize unless it falls out for free).
- The full 150 "Done when" rollup is met: harness + detail + terminal + yazi + vcs,
  per-pane toggles, responsive defaults, vcs via `vcs_tool()`, switching parks the
  set alive; grove + trellis suites green.

## Notes

- The breakpoint is grove's policy (a column-count threshold), passed into the
  mount; trellis stores/applies membership + visibility but does not own the
  responsive policy (one-way seam).
- Keep it two tiers — **not** a layout DSL or per-user config (constraint 4). A
  later leaf can add config if a real need surfaces; do not pre-build it.
- On node retirement, promote the settled responsive tiers + wide arrangement into
  the [[working set]] glossary entry (it currently describes the UX in the
  abstract) and check whether the working-set generalisation earned an ADR.
