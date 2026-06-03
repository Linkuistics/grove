# 010-multi-host-panes

**Kind:** work

## Goal

Widen the trellis host-pane seam from **one host pane per session** to **N host
panes, created on demand** — the framework foundation 020 stacks the per-grove
detail surface on. Today (110/030) the home nav is the single host pane, injected
once at first-layout from a one-shot factory. 130 needs each grove tab to carry
its *own* detail host pane beside its harness command pane.

## Context

- **The single-tenant seam (today):** `register_host_surface(factory)` stores one
  `FnOnce`; `take_host_surface()` (`host_pane.rs:234`) yields it once;
  `screen.rs:3038` injects it into the first tab whose layout is applied via
  `Tab::inject_host_pane`. After that take it returns `None` forever.
- **What 020 needs:** when the nav opens grove `<name>`'s [[workspace]] tab, the
  tab must contain (a) a **detail host pane** rendering a grove-supplied surface
  *and* (b) the **harness command pane** (`grove do <name>`). Today
  `HostDriver::new_command_tab` opens a tab with *only* the command pane.

## Done when

- A `HostDriver` verb opens a **new tab** carrying a host pane built from a
  host-supplied surface **plus** the harness command pane, laid out as a simple
  split (detail + harness). The surface factory rides *in* the instruction (not the
  global one-shot slot), so N tabs each get their own.
- The existing one-shot home-nav injection still works unchanged (the home pane is
  not regressed).
- `HostSurfaceTick` / focus / resize / input route to the *correct* host pane when
  more than one exists (the pane-id addressing already in `HostPane` — verify it
  generalises; the tick handler at `screen.rs:5783` already finds the pane by id).
- `crates/trellis` builds; `HOST_API.md` updated to document the on-demand path
  beside the one-shot path. Unit test: the new driver verb posts the expected
  `ScreenInstruction` carrying the factory + command (mirror
  `host_driver_posts_the_expected_screen_instructions`).

## Notes

- **Crossing the re-exec / factory question:** the home pane's factory is set
  pre-`start_server` (it cannot pass a value across the spawn). The *detail* panes
  are opened *later*, from the already-running server's screen thread (the nav
  surface runs there) — so their surface *can* be passed by value in the
  instruction. The host surface is `Send`; confirm `ScreenInstruction` can carry a
  `Box<dyn HostSurface>` (it is an in-process channel, not serialized).
- Keep the one-way crate seam (ADR-0020 §4): `zellij-server` defines the verb;
  grove implements the surface. trellis still never names grove.
- The HOST_API.md "Multiple host panes / a host registry — one per session today"
  line is the explicit deferral this leaf lifts; update it.
- Don't build a general multi-host *registry* abstraction beyond what 020 needs
  (constraint 4) — on-demand per-tab creation is the requirement, not a query API.
