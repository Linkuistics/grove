# 040-grove-integration — brief

**Kind:** node (build). Substrate per ADR-0015 ([[zellij substrate]]); dashboard
architecture per ADR-0016 ([[controlling process]] + [[dashboard proxy]]);
**harness UX reshaped by ADR-0018** — groves are [[workspace]] tabs navigated by
a [[nav plugin]] — then **the integration model reshaped again by ADR-0019** after
live-testing 080: `cli_pipe_output` is reply-only, so the nav opens workspaces
**itself** (no back-channel) and grove **detail lives per-workspace** as a
[[detail proxy]]; the home tab is the nav, full-height.

## Goal

Wire grove onto its owned [[zellij substrate]] under the controlling-process
model: a persistent [[controlling process]] launches zellij and owns all state +
rendering; the **home dashboard** is a [[dashboard proxy]] it renders into; and
**each grove is a [[workspace]] tab** holding that grove's [[working set]]
(harness + terminal + yazi + lazygit), navigated by a leader-focused [[nav
plugin]] (ADR-0018). Closes the 060 headline feature — live harness sessions, one
grove's working set at a time, switched as workspaces.

## Context

- **Substrate (ADR-0015):** harness panes are native zellij panes; grove drives
  via `zellij action`. Copy/scrollback/search/persistence are native (why
  030-scrollback-copy was retired).
- **Controller + proxy (ADR-0016):** the controller renders the dashboard and
  ships frames to a dumb `grove __dash-proxy`; input up, display down. Tamed
  config knobs validated on zellij 0.44.3 (locked mode, no bars, command panes
  `start_suspended false`).
- **Harness UX reshaped (ADR-0018):** the original model — dashboard is the
  switch surface, harness panes beside it, switched by `focus-pane-id` — hit a
  locked-mode dead-end (no bound key returns focus to the dashboard; grove's
  config stripped pane-nav binds). Replaced by: grove = zellij tab; switch via
  `GoToTab` + the [[nav plugin]] (a WASM plugin, Strategy 1a pulled forward);
  split driving (plugin does pure-zellij nav, controller does grove-data
  first-open).

## Decisions carried into the build

- Render-over-socket = `CrosstermBackend` over a socket writer (010, done).
  Hand-rolled input decoder, no new dep (010, done). Bundled config/layout
  embedded + written to a cache dir (030, done). Depend on an installed zellij.
  Leader = `Ctrl-o` (now bound to `LaunchOrFocusPlugin`, ADR-0018). Protocol
  supports N proxies.
- **Split driving (ADR-0018):** the [[nav plugin]] issues pure-zellij nav
  (`go_to_tab`/focus/toggle) directly; the controller owns state, pipes it to the
  nav, and does first-open of a grove's tab + working set via the 040 `zellij
  action` driver.

## Done when (acceptance for the whole node)

- `grove tui` launches the zellij substrate as a single binary. **[done —
  010/020/030]**
- The **home tab is the [[nav plugin]]** (full-height grove list); the leader
  (`Ctrl-o`) focuses it from any pane (ADR-0019).
- Selecting a grove in the nav **opens or switches its [[workspace]] tab itself**
  (no controller back-channel); `GoToTab` keybinds also switch.
- Each grove tab shows its [[working set]] — harness + a per-grove [[detail
  proxy]] (its task tree / inbox / capture) + terminal + yazi + lazygit — laid out
  responsively, each pane toggleable.
- One grove-owned [[whichkey bar]] spans the bottom of every tab; no other surface
  draws hints.
- Within one repo; the nav opens with explicit repo/cwd so 070-fleet-view reuses
  the driving cross-repo.

## Decomposition (this node)

```
done/
  010-proxy-protocol       IPC seam + dumb proxy + input decoder + render backend
  020-controller-loop      dashboard event loop in the controller, over the seam
  030-zellij-launch        head binary: embed config+layout, launch zellij, Ctrl-o
  040-harness-driving      zellij-action open/close + HarnessPanes tracker
                           [primitives landed; focus-pane-id switcher superseded]
  050-mode-discoverability [subsumed by the nav plugin — see 070]
  060-workspace-tabs       grove = zellij tab; home tab; GoToTab switching
  070-nav-plugin           grove-nav WASM plugin: render piped state, keys when
                           focused, pure-zellij nav; leader→LaunchOrFocusPlugin
  080-controller-plugin-pipe controller↔plugin pipe [built, then SUPERSEDED by
                           ADR-0019: cli_pipe_output is reply-only; back-channel
                           deleted. The work surfaced that constraint empirically.]

live (ADR-0019 "A′" model):
  090-nav-self-opens         nav opens/switches tabs ITSELF (new_tabs_with_layout);
                             grove-state carries cmd+cwd; delete the 080 channel;
                             pre-seed permissions.kdl; sigil hints
  100-detail-proxy-per-grove controller renders per-grove detail into each grove
                             tab (N proxies); home tab = nav full-height
  110-whichkey-bar           grove-owned full-width bottom-bar plugin (sigils);
                             dashboard/harness stop drawing hints
  120-working-set-responsive +terminal +yazi +lazygit; per-pane toggles;
                             responsive layout (5K2K ↔ MacBook Pro)
```

## Notes

- Scope is within one repo; the cross-repo fleet is 070-fleet-view (it reuses
  this driving layer, opened cross-repo).
- Launching `grove do <name>` inside a tab nests grove deliberately — intended.
- ADR-0019 supersedes 080's back-channel and the "home dashboard lists groves"
  framing; the nav is the home + the navigator, detail is per-grove.
- The shelved [[harness-pane crate]] embed (ADR-0014) stays a recoverable
  fallback, not on this path.
