# 150-working-set — brief

**Kind:** node (build). Decomposed from a work leaf after a grilling pass settled
the design forks the original leaf flagged ("responsive defaults/breakpoints are
this leaf's design call"). The settled decisions are in **Decisions** below; no
new ADR — the *mechanism* is already ADR'd (ADR-0023 suppress/restore +
`replace_pane`; ADR-0022 constant nav + content region), and this node **applies
those primitives more broadly** (pair → working set) rather than inventing one. An
ADR is earned only if a surprising constraint surfaces in the build (as ADR-0023
itself was earned by the "surface can't ride a `ScreenInstruction`" discovery).

## Goal

Flesh out each grove's [[working set]] — harness + per-grove detail + a plain
terminal + yazi (files) + lazygit/lazyjj (vcs) — embedding the aux tools via the
[[trellis framework]]'s TUI-embedding (zellij's emulation; the same mechanism the
harness pane uses). Individual show/hide toggles, and a responsive layout that
packs everything on a large display (5K2K) and degrades to a MacBook Pro screen.
This is where ADR-0020's headline capability (embed *other* TUI apps) gets
exercised; 110 proved grove's *own* native surfaces.

## Decisions (settled in the grilling pass — carried into the build)

- **Working set = a variable-membership content region, not a fixed pair.** Today
  `ContentSwap` (screen.rs:1421) hard-wires a `primary_slot` (harness) + optional
  `secondary_slot` (detail). The headline framework change is generalising that to
  an **ordered working set** per key (harness + detail + terminal + yazi + vcs),
  parked/restored **as a unit** — the same suppress/restore + in-place
  `replace_pane` primitives (ADR-0023), applied to N members instead of 2.
- **Toggle = park alive (suppress/restore).** Hiding a working-set pane
  **suppresses** it (child keeps running, scrollback intact); showing restores it
  and re-tiles the visible members. Reuses `suppressed_panes` — the same primitive
  as the grove-switch park. *Not* close/respawn (loses state, re-pays yazi/lazygit
  startup) and *not* float (overlaps, different interaction model).
- **Responsive: two-tier.** Wide (5K2K/ultra-wide) packs **all** members; laptop
  (MacBook-class) defaults to **harness + detail** visible, with terminal / yazi /
  vcs hidden and toggled on demand. The default-visible set is chosen by terminal
  size at mount; the user can always toggle from there.
- **Wide arrangement: harness-dominant + side stack.** The harness takes the
  largest share; detail + terminal + yazi + vcs stack in a column (or columns)
  beside it — glanceable aux tools, harness is the focus of attention.
- **VCS pane is not hard-wired to git.** A single `vcs_tool()` indirection probes
  the worktree (jj repo → lazyjj, else lazygit); **default lazygit now**, the
  detection seam in place so lazyjj lands later as a one-point change. Aux panes
  all run in the grove's worktree cwd.
- **Aux panes are per-grove.** Each grove's working set carries its own
  terminal/yazi/vcs instances, parked alive as a unit on grove-switch (consistent
  with the harness+detail park already built).

## Done when (rollup for the subtree)

- A grove's content region shows harness + detail + terminal + yazi + vcs laid out
  per the wide/laptop tiers; the default-visible set adapts to terminal size.
- Each pane toggles individually from the nav (park-alive suppress/restore); aux
  panes run in the grove's worktree; the embedded tools behave exactly as they do
  bare (focus, input, copy, resize, cursor — the trellis embedding promise).
- lazygit works; the vcs pane routes through `vcs_tool()` (lazyjj later).
- Switching groves still parks the whole working set alive and restores it intact.
- `cargo build` / `cargo test` green (grove **and** the trellis `zellij-server`
  suite — the swap/editor/close-pane tests stay green).

## Decomposition (this node)

Framework generalisation first (independently reviewable, trellis-only), then
grove composes the real tools, then the toggle UX, then the responsive defaults —
each a focused commit, each depending on the prior.

```
010-working-set-mechanism  trellis: generalise ContentSwap pair → ordered
                           working set; suppress/restore + re-tile a single
                           member; HostDriver toggle verb. Trellis-only + tests.
020-aux-tool-panes         grove: spawn terminal + yazi + vcs (vcs_tool() probe,
                           default lazygit) as command panes in the grove's
                           worktree cwd; compose them into the working set beside
                           harness + detail. Proves "embed other TUI apps".
030-pane-toggles           per-pane show/hide driven from the nav; whichkey hints;
                           wire the toggle verb to keys; park-alive semantics.
040-responsive-layout      default-visible set + wide arrangement (harness-dominant
                           side stack); breakpoint on terminal size at mount.
```

## Pointers

- ADRs a session here must read (beyond the chain's): **ADR-0023**
  (content-swap = suppress/restore + in-place `replace_pane`; the primitive this
  node generalises), **ADR-0022** (constant nav + swapped content region),
  **ADR-0024** (embedded-tool *exit* slice — `HostDriver::open_editor` /
  `editor_exited`; the aux tools widen ADR-0020 §6 *on top of* this, not by
  re-deriving spawn/observe). **ADR-0020 §6** (first-class observability of wrapped
  tools — this node only needs them embedded + usable, not introspected; deeper
  observability stays later/lazy).
- Glossary terms in play: [[working set]], [[trellis framework]], host surface /
  host pane / host driver, content region (see CONTEXT.md).
- Key code: `src/trellis_host.rs` (layout KDL `GROVE_TUI_LAYOUT`, config),
  `src/tui.rs` `mod native` (DashboardSurface / DetailSurface / swap driving),
  trellis `zellij-server/src/{screen.rs,tab/mod.rs,panes/host_pane.rs}`
  (`ContentSwap`, `SwapContent`/`ContentSpawned`, `content_restore`,
  `mount_host_surface`, `suppress_pane_and_replace_with_pid`, `HostDriver`).

## Notes

- Keep it the smallest legible thing (constraint 4) — **not** a general
  tiling-config system. The responsive tiers are two breakpoints, not a layout DSL.
- The one-way crate seam holds (ADR-0020 §4): `zellij-server` defines the
  working-set/suppress/restore mechanism; grove drives it; trellis never names
  grove or a grove tool.
- Depends on 110/120/130/140 (host API; native nav; native detail; whichkey) — all
  done.
