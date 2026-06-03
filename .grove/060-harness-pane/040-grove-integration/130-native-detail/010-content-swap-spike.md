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
