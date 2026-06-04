# 010-swap-substrate

**Kind:** work

## Goal

Build the **content-swap substrate** (ADR-0023) end-to-end for the *harness* pane:
a **constant nav** beside a **content slot** in one tab, and a `HostDriver` swap
verb that mounts the selected grove's `grove do <name>` harness into the slot,
parking the previously-selected grove's harness alive off-screen. This replaces
the tab model (`Enter` → `new_command_tab`) with the ADR-0022 constant-nav +
content-swap model. **Detail is not built here** (that is `020-mount-detail`) — the
content slot holds the harness only, so swap/park/restore is proven on a single
pane before `020` adds the second.

## Context

- **Current state is the tab model.** `120-native-nav` built the nav as a *home
  tab*; `DashboardSurface::handle_key` intercepts `Enter` → `OpenHarness` →
  `HostDriver::new_command_tab` (a new [[workspace]] tab per grove). grove uses
  trellis's **default layout** (`trellis_host.rs` leaves `opts.layout` unset), so
  the nav host pane is injected as the first tab's sole pane via the one-shot
  `take_host_surface()` at `apply_layout`.
- **The mechanism is decided (ADR-0023)** and its primitives largely exist in
  `crates/trellis/zellij-server/src`:
  - `tab/mod.rs::suppress_pane_and_replace_with_other_pane` (~2367) — replace a
    tiled pane with another, parking the displaced one into `suppressed_panes`.
  - `panes/tiled_panes/mod.rs::replace_pane` (~145) — in-place swap; incoming pane
    inherits the removed pane's `PaneGeom` via `set_geom`, focus follows.
  - `screen.rs::ScreenInstruction::NewPane { placement: Tiled }` (~330) — open a
    command pane *into an existing tab* (not a new tab).
  - The one-shot host seam: `HOST_SURFACE_FACTORY: Mutex<Option<factory>>`,
    `take_host_surface()`, `next_host_pane_id()`, `inject_host_pane`
    (`panes/host_pane.rs` ~217–243, `tab/mod.rs` ~5578).

## Build

1. **Two-pane layout.** Give grove a session layout (programmatic
   `TiledPaneLayout` or KDL) of one tab: a **fixed-width nav pane** (e.g. 30 cols)
   beside a **content slot** pane. Inject the nav `DashboardSurface` into the nav
   pane; leave the content slot as an addressable placeholder (a stable
   `PaneId`/position the swap verb targets). Replace the "first tiled pane" target
   in `inject_host_pane` with the nav-pane target.
2. **Keyed registry + `MountHostSurface`.** Generalise `HOST_SURFACE_FACTORY` to a
   keyed `Mutex<HashMap<GroveKey, factory>>` (GroveKey = grove name). Add
   `ScreenInstruction::MountHostSurface { key, … }` (ids only — `Clone+Debug`); the
   screen thread pops the factory by key and builds the `HostPane` into the content
   slot. *(For the harness, which is a terminal pane not a host surface, the mount is
   a `NewPane`/command-pane into the slot — see step 3; the registry/MountHostSurface
   is the seam `020-mount-detail` reuses for the detail host pane. Build whichever of
   the two the harness-only acceptance actually needs; the detail-host registry can
   wait for `020` if the harness swap does not exercise it.)*
3. **`HostDriver::swap_content(grove_key)`.** Fire-and-forget verb the nav posts.
   First selection of a grove: open `grove do <name>` as a terminal pane **into the
   content slot** (`replace_pane` the placeholder/previous occupant; park the
   displaced harness into `suppressed_panes`). Re-selection of a parked grove:
   `replace_pane(content_slot, parked_harness)` pulling it back out of
   `suppressed_panes`. Track grove→harness `PaneId` mapping in the screen/host layer.
4. **Wire the nav.** `nav_enter_target()` → `swap_content(name)` instead of
   `OpenHarness`/`new_command_tab`. Keep `Ctrl-o` (leader → focus nav).
5. **Retire the tab switcher.** Remove the `GoToTab`/`Alt-1..9`/`Alt-]`/`Alt-[`
   binds from `GROVE_TUI_CONFIG` (they were the tab switcher).

## Done when

- One tab: a constant ~30-col nav pane + a content slot. The nav renders the v1
  grove list; `Ctrl-o` focuses it.
- Selecting grove **A** mounts `grove do A` into the content slot beside the nav;
  selecting grove **B** swaps `grove do B` in and **parks A's harness alive**
  (its pty keeps running, scrollback intact); selecting **A** again **restores**
  A's harness (scrollback present).
- No `GoToTab`/`Alt-N` tab switching remains; no per-grove tabs are created.
- `cargo build`/`cargo test` green (incl. a trellis-side test of the
  swap/park/restore primitive, in the spirit of the retired throwaway's three
  deltas: sibling-nav-untouched, parked-pty-alive, resize-correct). grove core
  stays `ratatui`-free below the ADR-0013 seam.

## Notes

- The grove→harness `PaneId` bookkeeping (which grove's harness is in the slot,
  which are parked) is new state; keep it in the host/screen layer, not in `App`.
- `020-mount-detail` turns the content slot into a **split** (harness + detail) and
  handles parking/restoring the **pair**; design the slot addressing so that
  widening from one pane to a sub-split is additive, not a rewrite.
- Deciding programmatic-layout vs bundled-KDL is a build call; whichever keeps the
  nav-pane + content-slot addressing simplest. Record it inline (glossary/ADR) only
  if it turns out to be a load-bearing, surprising choice.
