# 19. The nav opens workspaces itself; grove detail lives per-workspace

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record. (Prior status: accepted —
  **mechanism superseded by [ADR-0028](0028-rmux-substrate.md); UX intent survives**,
  rmux substrate, 2026-06-10, 070-teardown D4; per the 050-plan-rebuild/010-surfaces
  verdict.)
- Date: 2026-06-02
- Deciders: Antony Blakey (with grove 060 design)
- Supersedes: ADR-0018's **split-driving back-channel** (the nav no longer
  "signals open-intent back to the controller") and its **dashboard-as-home-list**
  framing
- Amends: ADR-0016 (the controller now renders **N per-grove detail proxies**, one
  per open workspace, not a single home dashboard)

> **rmux-substrate verdict (070-teardown, D4; per 050-plan-rebuild/010-surfaces).**
> The *UX* survives: the nav stays always-reachable and per-grove detail stays
> scoped to its grove. The *realisations dissolve* — the N dumb `grove __dash-proxy`
> detail proxies, the controller socket seam, and the `RunEditor` frame become a
> single ratatui detail **widget** grove draws from `RepoView` (030-detail-widget);
> the "nav opens the workspace itself via the plugin API" becomes the leader-dispatch
> gate (`leader → g`); the constant-nav *region* pin gives way to "reachable via the
> leader" (020-leader-dispatch). Annotated, not blanked.

## Context

Leaf `080-controller-plugin-pipe` built ADR-0018's split-driving seam: the nav
plugin signals "open grove X" back to the controller over a `zellij pipe`
back-channel (`cli_pipe_output`), and the controller first-opens the tab. Live
testing on zellij 0.44.3 (instrumented nav + headless probes + a human
reproduction) established what actually holds:

**Confirmed working** — the ADR-0018 spine is sound:
- **Keys reach a focused plugin in locked mode.** The nav's key counter climbed on
  every `j`/`Enter` while focused under `default_mode "locked"`. The leader→nav
  model is not the problem.
- **Controller → nav forward pipe works** (the `grove-state` push; the list
  renders).
- **The nav can act on zellij directly**, including **opening a tab itself**:
  `open_command_pane_in_new_tab(CommandToRun, ctx) -> (tab_id, pane_id)` and
  `new_tabs_with_layout(layout) -> tab_ids` are plugin-API calls.

**The dead end** — why "Enter doesn't go":
1. `cli_pipe_output` / `unblock_cli_pipe_input` silently **no-op without the
   `ReadCliPipes` permission** ("Control command line pipes and output"), which the
   nav never requested. (Granting it made the channel emit — root cause #1.)
2. More fundamentally, `cli_pipe_output` is a **reply primitive**: it only delivers
   to the CLI invocation whose message the plugin is **currently handling** (the
   `tail -f | zellij pipe | wc -l` request/response model). It **cannot** push to a
   *stored* pipe id from a later, unrelated event. Verified directly: emitting to a
   stored id from a different message context delivered nothing to the reader.

ADR-0018's design triggers the open from a **keypress** (`Enter`) and emits to a
**stored** channel id — exactly the unsupported pattern. So "the nav signals intent
back to the controller" is unworkable, independent of the permission.

Reframing with the user also resolved two UX findings from the same session: the
nav and the home dashboard **both listed groves** (redundant), and the dashboard
**drew its own hint line** (should be one grove-owned surface).

## Decision (the "A′" model)

**The nav is self-sufficient — it opens *and* switches workspaces itself.** On a
selection: an already-open grove → `switch_tab_to`; a closed grove → the nav
first-opens its tab via the plugin API (`new_tabs_with_layout`). No back-channel.

**The controller pushes everything the nav needs, forward only.** The `grove-state`
pipe carries, per grove: name, inbox-pending count, **cwd**, and the **exact
`grove do <name>` command** (composed in Rust). Command composition stays in Rust;
it is transmitted in the one direction that works.

**Grove detail lives in each grove's own workspace tab.** A grove's tab holds its
harness pane **and** a [[detail proxy]] — a controller-rendered, grove-scoped
dumb proxy (`grove __dash-proxy --grove <name>`) showing *that grove's* task tree,
inbox, and capture. This **amends ADR-0016**: the controller renders **one detail
proxy per open grove** (N proxies), each fixed to its grove, rather than a single
home dashboard that switches selection. No nav→controller signalling is needed —
each proxy is self-describing (it connects with its grove name).

**The home tab is the nav, full-height** — the grove list *is* the home surface.
Global capture (`c`) is a nav key. The home dashboard's own grove-list is dropped,
removing the nav/dashboard redundancy.

**The whichkey is a grove-owned, full-width bottom-bar plugin** (its own leaf),
rendering context-sensitive keys with sigils; the dashboard and harness stop
drawing their own hint lines.

**The 080 back-channel is deleted** — `IntentReader`, the `nav_pipe` return path,
`cli_pipe_output`, and the `ReadCliPipes` request — recoverable from git history.

## Consequences

- **No fragile reply-channel.** The nav is the navigator; the only controller→nav
  traffic is the forward `grove-state` push, which works reliably.
- **N-proxy rendering is now load-bearing.** ADR-0016's "supports N proxies" moves
  from latent to exercised: the controller renders a detail proxy per open grove.
- **The nav needs a tab-opening permission** (`OpenTerminalsOrPlugins` /
  `RunCommands`). To avoid a first-run permission prompt inside grove's chrome-less
  locked config, **grove pre-writes the grant into zellij's `permissions.kdl` at
  launch** (it owns its bundled plugin path). Proven out in the first leaf.
- **ADR-0018 mostly holds.** Leader→nav, tabs-as-workspaces, `GoToTab` switching,
  and locked-mode key delivery are all confirmed live. Only the split-driving
  back-channel and the dashboard-as-home-list are superseded.
- **080's code is removed**, its leaf retained in `done/` as history. The work was
  not wasted: it is what surfaced the `cli_pipe_output` constraint empirically.

## Notes

- **The load-bearing fact, recorded so no future session re-burns it:**
  `cli_pipe_output` is request/response (reply to the message you're handling), not
  a server-push channel. A plugin cannot push to the controller from a keypress; if
  a plugin must *act*, it uses the plugin API directly (open/switch tabs), and the
  controller feeds it forward via `zellij pipe`.
- `ReadCliPipes` is required for `cli_pipe_output` to do anything at all — found the
  hard way (silent no-op); moot once the back-channel is deleted.
- Plugin-API facts verified this session (live, not just docs): `Event::Key` fires
  for a focused plugin in `locked` mode; `open_command_pane_in_new_tab` /
  `new_tabs_with_layout` open tabs from a plugin; `switch_tab_to` switches.
