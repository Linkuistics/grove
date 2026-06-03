# 010-content-swap-spike

**Kind:** planning (build-discovery spike → mechanism ADR)

## Goal

Decide **empirically** how to realise the ADR-0022 model — a **constant nav** + a
**content region** the nav swaps the selected grove's working set (harness +
detail) into, with non-selected harness ptys kept alive off-screen. Output: a
working throwaway proving the **park/mount** mechanism, and a **mechanism ADR**
(ADR-0023) the 020/030 build leaves stand on.

## The fork to resolve

How does a grove's harness pane + detail [[host surface]] get **parked** (alive,
off-screen) and **mounted** into the content region on selection?

- **Candidate A — `suppressed_panes` (native).** zellij already hides panes alive
  via `suppressed_panes` (it's how the built-in `$EDITOR`-over-a-pane works). Park
  = suppress; mount = un-suppress into the content slot. Reuses battle-tested
  machinery; the question is whether suppress/un-suppress addresses *arbitrary*
  panes into a *fixed slot* cleanly, and whether scrollback/resize survive it.
- **Candidate B — grove-managed pane pool.** grove holds the harness `TerminalPane`
  + detail `HostPane` for every open grove in a pool *outside* the displayed
  layout, and mounts one pair into the content region on selection. Full control,
  but more framework surgery (the layout engine assumes panes live in tabs).

## What the spike must establish

- The chosen mechanism keeps a parked harness's **pty alive and scrollback
  intact** while another grove is displayed (the ADR-0022 premise — verify, don't
  assume, against the real layout/resize paths).
- **Resize correctness:** a parked pane mounted into the content region resizes to
  the slot; the harness child sees the right `winsize`.
- **The host-pane seam widens from one-shot to N.** Today `take_host_surface()`
  (`host_pane.rs:234`) yields one surface at first-layout (`screen.rs:3038`); the
  detail surfaces are created later, from the running server's screen thread (where
  the nav surface lives) — so a surface *can* ride by value in a `ScreenInstruction`
  (in-process channel, not serialised; `HostSurface: Send`). Confirm.
- **Switching is nav-driven**, not `GoToTab`: the `HostDriver` gains a swap verb
  (mount grove X's pair, park the current). The `Alt-1..9`/`GoToTab` binds in
  `GROVE_TUI_CONFIG` retire.

## Done when

- A throwaway proves: constant nav pane + content region; selecting a grove mounts
  its harness (a live pty) into the content region and parks the previous one
  alive; switching back restores scrollback.
- **ADR-0023** records the verdict (A vs B), the `HostDriver` swap verb shape, and
  what the host-pane seam becomes (one-shot home nav + on-demand swapped content).
- The node BRIEF + 020/030 are re-grounded on the verdict if it shifts them.
- Decompose 010 into a build node if the chosen mechanism is more than one focused
  session (likely if B wins).

## Notes

- **Spike discipline (driving.md / ADR-0021 precedent):** throwaway code to decide;
  the *decision* is the artifact. Keep only what the ADR needs as evidence; don't
  carry scaffolding into the build leaves.
- Keep the one-way crate seam (ADR-0020 §4): `zellij-server` defines any new verb;
  grove implements the surface. trellis never names grove.
- The HOST_API.md "Multiple host panes / a host registry — one per session today"
  deferral is what this resolves; update the doc when the build leaves land, not in
  the throwaway.
- **Read first:** `suppressed_panes` usage in `tab/mod.rs`; `inject_host_pane` +
  `take_host_surface`; `HostDriver` (`host_pane.rs`); the `NewTab` path the current
  `new_command_tab` uses (the swap verb replaces tab-creation as the switcher).

## Decisions (running log)

### Keep-alive axis is structurally forced toward Candidate A (suppressed_panes)

Code trace (not assumption): `Tab::handle_pty_bytes` (tab/mod.rs:2723) routes a
child's output to the terminal pane found in `tiled_panes` → `floating_panes` →
**`suppressed_panes`** (2735-2740). A *suppressed* terminal pane therefore keeps
receiving pty bytes and updating its vt100 grid + scrollback — the ADR-0022
"parked pty stays alive" premise, confirmed. The same `.or_else(suppressed_panes)`
fallback recurs at ~15 sites (`get_pane_with_id`, `set_pane_color`, resize, …):
suppression is a first-class *alive-but-hidden* state — exactly the **park**
primitive. Candidate B (a grove pool outside the tab) sits off this routing path,
so it would have to re-plumb every one of those sites or the parked harness goes
deaf. This is the "more framework surgery for B" the brief anticipated.

### Q1 — Spike route: throwaway for the residual layout/resize risk only *(settled)*

The keep-alive axis is decided by the trace above; the residual risk is the
**layout axis** — whether the unsuppress path can mount an arbitrary parked pane
into a *fixed content slot* beside a constant nav and resize the child correctly
(`unsuppress_pane` today calls `add_tiled_pane`, which lets the layout engine
place the pane, not a designated slot). **Decision:** build a *minimal* throwaway
that exercises exactly that residual risk (mount a live-pty pane into a fixed slot
beside a constant-width nav, switch, resize, confirm scrollback survives), take
keep-alive as proven by the trace, then write ADR-0023 from trace + demo. Not a
full A-vs-B bake-off (the routing evidence already makes B costly with no upside).

### Throwaway verdict: A wins, realised via in-place `replace_pane` *(settled, demo green)*

Throwaway test `grove_content_swap_spike` (tab/unit/tab_tests.rs — **to be
deleted**; 020 writes the real verb + tests) **passes on first run**. It builds the
ADR-0022 shape — a fixed-30-col constant nav beside a content slot, vertically
split — and proves the three deltas the existing scrollback-editor tests don't:

- **Geometry / fixed slot:** the swap uses the **in-place `replace_pane`**
  primitive (via the scrollback-editor gesture `replace_active_pane_with_editor_pane`),
  *not* stock `suppress_pane`/`unsuppress_pane` (which re-tile through
  `add_tiled_pane`). The incoming pane inherits the slot's exact `PaneGeom`
  (`tiled_panes::replace_pane`: `with_pane.set_geom(removed_pane_geom)`), and the
  **sibling nav pane is byte-for-byte untouched** by the swap (asserted).
- **Park keeps the pty alive + scrollback:** bytes fed to grove A *before* the park
  and *while parked* (`handle_pty_bytes(1, …)` routing to `suppressed_panes`) are
  **both** present in its grid after restore (`dump_screen(true)` contains both).
- **Resize correctness:** after a tab resize the constant nav stays fixed at 30
  cols and the mounted pane's `get_content_columns()` grows with the slot — and
  that content geometry **is** the child winsize, since `resize_pty!` forwards
  `get_content_columns()/get_content_rows()` verbatim (tab/mod.rs:73).

**Verdict for ADR-0023:** Candidate **A** (native `suppressed_panes`), realised as
**in-place `replace_pane` into a stable content slot**, with the displaced pane
parked in `suppressed_panes`. Candidate B (grove pool outside the tab) is rejected:
it sits off the ~15-site pty/resize routing that already keeps a suppressed pane
alive, so it would re-implement that wiring for no upside.

### Host-pane seam widening (one-shot → N): registry + id-only mount instruction

The brief's premise — "a surface can ride by value in a `ScreenInstruction`
(in-process, not serialised; `HostSurface: Send`)" — **does not hold directly**:
`ScreenInstruction` is `#[derive(Debug, Clone)]` and `Box<dyn HostSurface>` is
neither, so a variant carrying the boxed surface breaks the derive for the whole
enum. `Send` is necessary but not sufficient. The proven one-shot path already
sidesteps this: the surface lives in a `static Mutex<Option<factory>>`
(`HOST_SURFACE_FACTORY`) and the instruction carries nothing. **The N-surface
widening is the same shape**: a keyed registry (e.g. `Mutex<HashMap<GroveKey,
factory>>`) plus a lightweight `ScreenInstruction::MountHostSurface { key, … }`
that carries only ids (`Clone + Debug`-friendly). The screen thread pops the
factory and builds the `HostPane` in the target slot — `inject_host_pane`
generalised from "first-layout, the placeholder slot" to "on demand, the content
slot." The channel carries the *key*, never the surface.

### Swap verb + binds

- `HostDriver` gains a **swap verb** (working name `swap_content(grove_key)`):
  fire-and-forget, posts the id-keyed mount/park instruction. The current
  `new_command_tab`/`focus_tab`/`close_tab` tab verbs are superseded as the
  switcher (the harness is now a content-slot pane, not a tab).
- The `GROVE_TUI_CONFIG` `GoToTab`/`Alt-1..9` binds **retire** (they were the
  tab-switcher); `Ctrl-o` (leader → focus nav) stays.
