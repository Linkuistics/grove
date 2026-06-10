# 23. Content-swap is in-place `replace_pane` into a `suppressed_panes` slot

- Status: **superseded by [ADR-0028](0028-rmux-substrate.md)** (rmux substrate,
  2026-06-10, 070-teardown D4; mechanism-dissolved) — the `suppressed_panes` +
  in-place `replace_pane` park machinery has **no analogue**: under rmux a pane in
  a detached window stays alive in the daemon whether or not grove draws it, so
  "park" is just "don't draw it this frame". The mechanism evaporates wholesale.
- Date: 2026-06-03
- Deciders: Antony Blakey (with grove 060/040/130/010 spike)
- Builds on: ADR-0022 (constant nav + swapped content region — this ADR fixes the
  *mechanism* that ADR-0022 deferred to a build-discovery spike), ADR-0021
  (library-you-link host API; the host pane / `HostSurface` / `HostDriver` seam),
  ADR-0020 (deep native fork; the one-way `zellij-server`→grove crate seam).

## Context

ADR-0022 settled the harness UX *model* — a **constant nav** always on screen
beside a **content region** the nav swaps the selected grove's working set
(harness + detail) into, with non-selected harness ptys kept alive off-screen —
but explicitly **deferred the park/mount mechanism** to this build-discovery
spike (leaf `130-native-detail/010`). Two candidates:

- **A — native `suppressed_panes`.** zellij already hides panes alive via a
  per-tab `suppressed_panes` map (it is how the built-in scrollback-editor hides
  the edited pane). Park = suppress; mount = bring back into the content slot.
- **B — grove-managed pane pool.** grove holds every open grove's harness +
  detail panes in a pool *outside* the displayed layout and mounts one set into
  the content region on selection.

The spike's residual question (per ADR-0021 precedent — decide what the code
forces by tracing it, demo only what the code cannot settle) was the **layout
axis**: whether a parked pane can be mounted into a *fixed content slot* beside a
constant nav and resize the child correctly. The **keep-alive axis** was settled
by code trace before any demo.

## Evidence

1. **Keep-alive is structurally A's for free (code trace).** `Tab::handle_pty_bytes`
   (`zellij-server/src/tab/mod.rs:2723`) routes a child's output to the terminal
   pane found in `tiled_panes` → `floating_panes` → **`suppressed_panes`**
   (2735-2740). A *suppressed* terminal pane keeps receiving pty bytes and updating
   its vt100 grid + scrollback. The same `.or_else(suppressed_panes)` fallback
   recurs at ~15 sites (`get_pane_with_id`, `set_pane_color`, the resize paths, …):
   suppression is a first-class **alive-but-hidden** state — exactly the *park*
   primitive. **This is why B loses:** a pool *outside* the tab sits off every one
   of those routing sites, so B would re-implement that wiring (or the parked
   harness goes deaf) for no compensating upside.

2. **In-place `replace_pane` mounts into a fixed slot (code trace).** Stock
   `suppress_pane`/`unsuppress_pane` route an un-suppressed pane back through
   `add_tiled_pane`, which *re-tiles* — the pane lands wherever the layout solver
   puts it, not a designated slot. But `TiledPanes::replace_pane`
   (`panes/tiled_panes/mod.rs:145`) swaps **in place**: the incoming pane adopts
   the removed pane's `PaneGeom` (`with_pane.set_geom(removed_pane_geom)`), focus
   moves to it, and the removed pane is returned for the caller to park. zellij's
   own scrollback-editor uses exactly this (`close_pane_and_replace_with_other_pane`
   → displaced pane into `suppressed_panes`). So "constant nav + content slot" is
   two stable tiled panes, and **swap = `replace_pane` on the content slot** — the
   nav (a sibling tiled pane) is never touched.

3. **The layout/resize residual was proven by throwaway (demo, green first run).**
   Throwaway test `grove_content_swap_spike` built the ADR-0022 shape — a fixed
   30-col constant nav beside a content slot, vertically split — and asserted the
   three deltas the existing editor tests do not cover:
   - **sibling untouched:** the nav's `position_and_size()` is byte-for-byte
     identical before and after a content swap (the editor tests fullscreen, which
     hides every sibling, so they could not show this);
   - **park keeps the pty alive + scrollback:** bytes fed to grove A *before* the
     park and *while parked* are **both** present in its grid after restore
     (`dump_screen(true)`);
   - **resize correctness:** after a tab resize the nav stays fixed at 30 cols and
     the mounted pane's `get_content_columns()` grows with the slot — and that
     content geometry **is** the child winsize, since `resize_pty!`
     (`tab/mod.rs:73`) forwards `get_content_columns()/get_content_rows()` verbatim
     onto the pty-writer channel.
   The throwaway is **not retained** (spike discipline / ADR-0021 precedent — the
   decision is the artifact); the build leaf (`020-detail-surface`) writes the real
   swap verb and its own tests.

4. **The N-host-surface widening cannot ride a surface in a `ScreenInstruction`
   (code trace — corrects the leaf's premise).** The leaf assumed a detail surface
   could "ride by value in a `ScreenInstruction` (in-process, not serialised;
   `HostSurface: Send`)." `Send` is necessary but **not sufficient**:
   `ScreenInstruction` is `#[derive(Debug, Clone)]` (`screen.rs:319`) and a
   `Box<dyn HostSurface>` is neither, so a variant carrying the boxed surface
   breaks the derive for the whole enum. The proven one-shot path already sidesteps
   this — the surface lives in a `static Mutex<Option<factory>>`
   (`HOST_SURFACE_FACTORY`, `panes/host_pane.rs:219`) and `take_host_surface()`
   pops it at first-layout while the instruction carries nothing.

## Decision

**Park/mount is Candidate A — native `suppressed_panes` — realised as in-place
`TiledPanes::replace_pane` into a stable content slot.**

- The persistent layout is **one tab**: a constant nav (a host `Pane`, fixed
  width) beside a **content slot** (also a tiled pane). The nav and the slot are
  siblings; the nav is never the target of a swap.
- **Mount grove X** = `replace_pane(content_slot_pane_id, grove_X_pane)`: the
  incoming pane inherits the slot's exact geometry; the displaced pane (the
  previously-selected grove's) is **parked** into the tab's `suppressed_panes`,
  where the existing pty/resize routing keeps it alive and capturing scrollback.
- **Switch back to a parked grove** = `replace_pane(current_slot_pane_id,
  parked_pane)`, pulling the parked pane out of `suppressed_panes`. Symmetric; no
  stock `suppress_pane`/`unsuppress_pane` (they re-tile).
- The content region holds the grove's **working set** — in 130's scope, the
  harness (a terminal pane) **and** the per-grove detail (a host pane) in a simple
  split (`150-working-set` owns the responsive multi-pane region). The swap
  therefore operates on the grove's **content subtree**, realised as one
  `replace_pane`+park per pane in that subtree.

**The host-pane seam widens from one-shot to N via a keyed registry + an id-only
mount instruction**, not a surface-carrying instruction:

- A keyed host-surface **registry** (e.g. `Mutex<HashMap<GroveKey, factory>>`)
  generalises the proven `HOST_SURFACE_FACTORY` static from one to N.
- A lightweight `ScreenInstruction::MountHostSurface { key, … }` carries only ids
  (`Clone + Debug`-friendly). The screen thread pops the factory and builds the
  `HostPane` in the target slot — `inject_host_pane` generalised from "first-layout,
  the placeholder slot" to "on demand, the content slot." **The channel carries the
  key, never the surface.**

**`HostDriver` gains a fire-and-forget swap verb** (working name
`swap_content(grove_key)`) that posts the id-keyed mount/park instruction. It is
the nav-driven switcher; the tab verbs (`new_command_tab`/`focus_tab`/`close_tab`)
are superseded as the switcher, and the `GROVE_TUI_CONFIG` `GoToTab`/`Alt-1..9`
binds **retire** (they were the tab-switcher). `Ctrl-o` (leader → focus nav)
stays.

## Consequences

- **130-native-detail builds on A.** `020-detail-surface` mounts the per-grove
  detail host pane (and the harness) via the registry + swap verb above;
  `030-native-editor` is unchanged in intent. The node BRIEF and `020` are
  re-grounded onto this verdict.
- **The host-pane seam stops being one-per-session.** `HOST_API.md`'s deferral
  ("Multiple host panes / a host registry — one per session today") is resolved by
  this decision; the doc is updated **when the build leaves land** (020/030), not by
  the spike (per the leaf's note).
- **No new framework concept.** The mechanism reuses `suppressed_panes`,
  `replace_pane`, and the existing host-surface-factory pattern — all proven. The
  one-way crate seam (ADR-0020 §4) holds: `zellij-server` defines `replace_pane`,
  `suppressed_panes`, the registry, and `MountHostSurface`; grove drives them.
  trellis never names grove.
- **Resilience unchanged.** Per the root brief's artifacts-over-state model, a
  parked harness surviving in `suppressed_panes` is a *convenience*; the durable
  state is the grove's files, not the live pane.

## Notes

- "Constant nav + content slot" is two stable tiled panes in one tab — **not** a
  pinned pane across tabs (which ADR-0022 established is not native for tiled
  panes) and **not** a tab-per-grove (superseded by ADR-0022).
- The throwaway proved the single-slot primitive; composing the harness+detail
  *pair* is N applications of the same primitive (one `replace_pane`+park per pane
  in the grove's content subtree) — a build detail for `020`, not a new mechanism.
