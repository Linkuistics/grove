# 040-grove-integration — brief

**Kind:** node (build). **Substrate reshaped by ADR-0020 ([[trellis framework]]):**
grove is now a **deep hard fork** of zellij — grove's TUI logic compiled in
**natively, in-process**, with the non-grove parts modularised as a publishable
TUI-embedding framework (grove-first, extract-later). This **supersedes ADR-0015**
(unmodified installed zellij) and **largely supersedes ADR-0016/0018/0019**: the
[[dashboard proxy]] seam, the WASM [[nav plugin]], and the `cli_pipe_output`
back-channel all evaporate — they were workarounds for grove being *outside*
zellij. The **UX model** they settled mostly survives ([[working set]], home = nav,
[[whichkey bar]], detail per-grove); only the plugin/proxy/back-channel
**realisations** are replaced by native code. **Then ADR-0022 reshaped one piece
of that UX:** grove is **no longer a [[workspace]] tab** — the nav is **constant**
and a **content region** swaps the selected grove's working set in (non-selected
harnesses stay alive off-screen). So "grove = tab / `GoToTab` switching" is
superseded; one-grove-at-a-time + per-grove detail beside the harness survive.

*(History: substrate per ADR-0015 → dashboard as [[controlling process]] +
[[dashboard proxy]] per ADR-0016 → harness UX as [[workspace]] tabs + WASM nav per
ADR-0018 → nav-opens-itself + per-grove [[detail proxy]] per ADR-0019 → native
fork per ADR-0020 → constant-nav + swapped content, no tab-per-grove, ADR-0022.)*

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

**Superseded by the fork (ADR-0020) — kept for history, not the live path:**
render-over-socket (`CrosstermBackend` over a socket writer, 010), the hand-rolled
input decoder (010), bundled-config-to-cache-dir + depend-on-installed-zellij
(030), the N-proxy protocol, and **split driving** (ADR-0018: WASM nav issues
pure-zellij nav, controller first-opens via `zellij action`). The native fork
renders **in-process** and drives panes/tabs/focus by **direct calls**, so none of
the socket/plugin/`zellij action` machinery survives.

**Carried forward:** Leader = `Ctrl-o` (its *binding* is now a native focus call,
not `LaunchOrFocusPlugin`). The v1 `RepoView` data layer, fs-watch, shell-out
writes, and the v1 dashboard/detail **ratatui views** all port to the native path.
ADR-0013's boundary holds (ratatui above; `RepoView`/writes below) and is promoted
to the framework↔grove crate seam.

## Done when (acceptance for the whole node — fork ADR-0020 + UX ADR-0022)

- `grove tui` launches the **forked [[trellis framework]]** as a single binary; the
  grove dashboard renders **natively in-process** (no proxy socket).
- The **nav is constant** (always on screen, native — full-height grove list); the
  leader (`Ctrl-o`) focuses it from the content region by a **direct in-process
  call** (no WASM, no `LaunchOrFocusPlugin`).
- Selecting a grove **swaps its [[working set]] into the content region
  in-process** (ADR-0022 — no tab-per-grove, no `GoToTab`); the previously-selected
  grove's harness stays alive off-screen.
- The content region shows the selected grove's [[working set]] — harness + native
  per-grove detail (its task tree / inbox / capture) + terminal + yazi + lazygit,
  the aux tools embedded via trellis's TUI-embedding — laid out responsively, each
  pane toggleable.
- One grove-owned native **whichkey** spans the bottom; no other surface draws
  hints.
- Within one repo; the nav opens with explicit repo/cwd so 070-fleet-view reuses
  the driving cross-repo.

*(The framework's own publish-grade surface — GraphQL/network, full observability,
per-platform pipeline, extraction — is explicitly out of this node's acceptance;
those are the later/lazy leaves.)*

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

  090-zellij-fork-framework  (planning, GATING) [done → ADR-0020] DECIDED: deep
                             hard fork — grove becomes a based-on-zellij codebase,
                             native in-process; non-grove parts modularised as the
                             publishable [[trellis framework]]; grove-first,
                             extract-later. Supersedes ADR-0015; largely supersedes
                             ADR-0016/0018/0019 (mechanism, not UX).

live: (the fork path — ADR-0020. Native in-process replaces proxy/WASM/back-channel.)
  100-framework-fork-bringup hard-fork zellij into the cargo workspace as the
                             `trellis` crate; one-way framework↔grove seam; minimal
                             rebrand + MIT attribution; builds on the dev platform.
                             Foundation; spike-then-build, decompose if needed.
  110-native-host-api        framework MVP: render a native ratatui surface as an
                             in-process pane; native tab/pane/layout/focus +
                             input/event delivery; port grove's v1 dashboard to
                             render natively. Subsumes the 010/020 proxy seam +
                             040 zellij-action driving. May decompose.
  120-native-nav             home nav surface, native (no WASM); opens/switches
                             grove [[workspace]] tabs in-process. Supersedes 070
                             (plugin) / 080 (back-channel) / old 100 (self-opens).
  130-native-detail          per-grove detail (task tree + inbox + capture)
                             rendered natively in each grove tab; $EDITOR drop
                             in-process. Supersedes old 110 (dumb detail proxy).
                             [decomposed → node, then reshaped by ADR-0022
                             (constant nav + swapped content, no tab-per-grove):
                             010 content-swap-spike (suppressed_panes vs pool →
                             mechanism ADR-0023), 020 detail-surface (mount detail
                             into the content region), 030 native-editor ($EDITOR
                             as a trellis pane + §6 exit observability). $EDITOR-as-
                             in-process-tty premise corrected: surface renders
                             server-side w/ no tty (ADR-0021).]
  140-native-whichkey        grove-owned full-width bottom hint bar, native (sigils);
                             single hint owner. Supersedes old 120 (WASM bar).
  150-working-set            harness + terminal + yazi + lazygit as embedded TUI
                             apps via trellis's embedding; per-pane toggles;
                             responsive layout (5K2K ↔ MacBook Pro). Was old 130.

later/lazy (added when earned — ADR-0020 §7):
  per-platform build pipeline (CI matrix; CVE-watch); GraphQL/network surface +
  full observability API; **extraction** of `trellis` into its own repo + grove.
```

## Notes

- Scope is within one repo; the cross-repo fleet is 070-fleet-view (it reuses
  this driving layer, opened cross-repo).
- Launching `grove do <name>` inside a tab nests grove deliberately — intended.
- ADR-0019 supersedes 080's back-channel and the "home dashboard lists groves"
  framing; the nav is the home + the navigator, detail is per-grove.
- The shelved [[harness-pane crate]] embed (ADR-0014) stays a recoverable
  fallback, not on this path.
