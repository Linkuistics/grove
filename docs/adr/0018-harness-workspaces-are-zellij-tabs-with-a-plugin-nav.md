# 18. Harness workspaces are zellij tabs, navigated by a plugin nav

- Status: **superseded by [ADR-0028](0028-rmux-substrate.md)** (rmux substrate,
  2026-06-10, 070-teardown D4) — no zellij tabs and no WASM plugin nav: grove
  draws a native ratatui nav surface and owns its layout directly. Premise +
  mechanism gone.
- Date: 2026-06-02
- Deciders: Antony Blakey (with grove 060 design)
- Amends: ADR-0016 (dashboard surfaces are dumb-terminal proxies — no longer
  *uniformly* so; see Decision)
- Refines: ADR-0015 (pulls the recorded-future Strategy 1a WASM plugin forward
  for the nav; switching moves from `focus-pane-id` to tabs)

## Context

The `040-harness-driving` build delivered the ADR-0015/0016 model: the dashboard
is one pane, harnesses are sibling panes, and you switch by the controller
issuing `focus-pane-id` driven from the dashboard. Live use exposed a dead-end.
In `default_mode "locked"` every key passes through to the focused app, so the
only way to reach grove's control is the unlock seam — but `Ctrl-o` unlocks to
zellij's `Normal` mode, and grove's bundled config (leaf 030) deliberately
stripped **all** pane-navigation bindings (the plan was "grove drives panes
externally, the user never navigates zellij by hand"). So once focus is in a
harness there is **no bound key that moves focus back to the dashboard** — the
"dashboard is the switch surface" model has no usable way back to the surface.

Reframing with the user changed the product, not just the bug:

- Show **one grove's working set at a time**, not many coexisting harness panes:
  harness (claude/codex) + a plain **terminal** + **yazi** + **lazygit** (→ lazyjj),
  switched as a unit.
- The panes are **individually toggleable** and the layout is **responsive** —
  pack everything on a 5K2K display, degrade gracefully to a MacBook Pro screen.
- The dashboard/nav shrinks to a small **persistent header**, and is just one
  more switchable workspace ("home").

Two facts settled the realisation. First, zellij's `GoToTab`/`GoToNextTab` are
bindable in locked mode (verified against the 0.44.3 default config) — so a
**tab per grove** gives native, reliable, state-preserving workspace switching
with no nav round-trip. Second, the leader-key problem has exactly one clean
native answer: a zellij keybind **cannot** focus a tiled pane by id (no such
bindable action), but `LaunchOrFocusPlugin "<name>"` focuses a **plugin** pane by
name, and a focused plugin receives every keypress. So the leader-reachable
control surface must be a plugin (the user ruled out a floating palette). The
zellij plugin API was verified against the official docs (see Notes).

## Decision

**Each grove is a zellij tab** (a *workspace*); the home dashboard is a tab too.
zellij keeps every tab's panes alive across switches. Switching is **native
`GoToTab` keybinds** for the hot path, plus the nav for rich/fuzzy selection.

**Each grove tab holds the grove's working set** — harness + terminal + yazi +
lazygit — individually toggleable and responsively laid out (defaults are a
build detail).

**The nav is a WASM plugin** — Strategy 1a, pulled forward from ADR-0015's
"recorded future refinement." The leader (`Ctrl-o`, unchanged from ADR-0016, but
now bound to `LaunchOrFocusPlugin "grove-nav" { move_to_focused_tab true }`
instead of `SwitchToMode "Normal"`) focuses it; the focused plugin receives keys
and is the command surface — switch grove, toggle panes, jump home — **and** the
answer to mode/key discoverability.

This **amends ADR-0016**: dashboard surfaces are no longer *uniformly* dumb
proxies. The model is now hybrid — the **home dashboard stays a controller-
rendered dumb proxy** (reusing the 020/030 seam), while the **nav is a smart,
self-rendering plugin**. ADR-0016's core invariant is preserved: the controller
remains the single source of truth for **grove state** (`RepoView`/`MultiRepoView`,
fs-watch, shell-out writes). The plugin holds only **zellij-layout** logic and no
grove state — it renders the grove list the controller pipes to it.

**Driving is split by concern:**

- The **plugin** issues pure-zellij navigation that needs no grove data —
  `go_to_tab`, focus a pane, toggle a pane — directly via the plugin API
  (snappy, no round-trip).
- The **controller** owns grove state, pipes it to the nav, and performs the
  grove-data action: **first-time create** of a grove's tab + working set, reusing
  `040`'s tested `zellij action` driver. The nav signals "open grove X" intent
  back to the controller.

Communication is zellij's **pipe** mechanism: controller → plugin (grove state,
via the `zellij pipe` CLI) and plugin → controller (open-grove intent).

## Consequences

- **The locked-mode dead-end is fixed.** The leader reliably focuses a real
  surface that receives every key — no reliance on zellij's nav-less `Normal`
  mode.
- **`040-harness-driving` is partially superseded, not wasted.** Its `zellij
  action new-pane`/`close-pane` primitives and the `HarnessPanes` tracker survive
  as the controller's first-open substrate. Its `focus-pane-id`-between-dashboard-
  panes switching and the dashboard `o`/`x` keys are superseded by tabs + nav.
  The leaf retires as "primitives landed, switching model superseded here," not
  "headline feature complete."
- **`050-mode-discoverability` is subsumed.** The plugin nav *is* the mode/key
  surface; its concern folds into the nav-plugin build rather than a standalone
  leaf.
- **Strategy 1a is pulled forward** from ADR-0015's "revisit only if CLI-driving
  chafes." It is justified by requirements (a persistent, leader-focusable nav;
  one-grove-at-a-time workspaces), not gold-plating — CLI-driving *did* chafe, at
  exactly the spot ADR-0015 flagged. New cost: a Rust→WASM plugin artifact with
  its own build/bundle, plus the controller↔plugin pipe protocol.
- **ADR-0013 boundary holds.** Grove data + writes stay in the controller; the
  plugin carries zellij-layout logic only. A future web client is still a proxy
  to the controller (ADR-0016's web axis is unaffected).
- **Cost accepted:** a second rendering surface (plugin alongside the proxy) and
  layout logic split across the controller (grove-data) and the plugin
  (pure-zellij), in exchange for a control model that actually works in locked
  mode and the one-grove-workspace UX.

## Notes

- **Verified plugin-API facts (zellij 0.44.x, official docs) — the evidence this
  rests on:** `Event::Key` fires for *every* keypress while the plugin pane is
  focused (no permission); `Event::Visible(bool)` fires on tab show/hide;
  `LaunchOrFocusPlugin` is bindable in locked mode and focuses a running instance
  by name (with `move_to_focused_tab`); `zellij pipe` carries CLI↔plugin messages
  (bidirectional) and `pipe_message_to_plugin` carries plugin↔plugin;
  `open_command_pane_near_plugin` opens a command pane in the plugin's tab and
  returns its `PaneId` (needs `RunCommands`). A keybind **cannot** focus a pane by
  id — only `LaunchOrFocusPlugin` (by name) or `ToggleFloatingPanes` can land
  focus on a specific surface; the floating option was rejected by the user.
- **The leader stays `Ctrl-o`** (ADR-0016/030 decision); only its *action*
  changes (`SwitchToMode "Normal"` → `LaunchOrFocusPlugin`).
- **Scroll/copy/search free-wins** (zellij `Scroll` mode, ADR-0015) remain
  reachable; whether entry is a retained binding or a nav command is a build
  detail.
- Responsive defaults (5K2K ↔ MacBook Pro) and the exact pipe message shapes are
  build details of the respective leaves, not fixed here.
