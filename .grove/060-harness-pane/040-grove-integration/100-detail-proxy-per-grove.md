# 100-detail-proxy-per-grove

**Kind:** work

## Goal

Put a grove's **detail** (task tree + inbox triage + capture) into that grove's
**own [[workspace]] tab**, beside its harness, as a per-grove [[detail proxy]] the
[[controlling process]] renders — and make the **home tab the [[nav plugin]],
full-height** (ADR-0019). This dissolves the old home master/detail dashboard: the
list is the nav, the detail is per-grove.

## Context

- **ADR-0019.** The controller renders **one [[detail proxy]] per open grove** (N
  proxies — ADR-0016's latent "supports N proxies" becomes load-bearing). Each
  proxy is `grove __dash-proxy --grove <name>` and is **fixed to its grove for
  life** (set when the nav opens the tab in 090), so there is **no nav→controller
  selection signalling** (which is what makes A′ buildable).
- **Nav opens the tab with both panes (extends 090):** the first-open layout grows
  from harness-only to `harness + detail-proxy`. The proxy command carries the
  grove name + the controller socket (piped to the nav in `grove-state`, or
  embedded in the layout the nav builds).
- **Controller side:** a **detail-only render mode** — reuse the v1 detail view
  (task tree, inbox list + disposition, capture modal, `$EDITOR` drop via
  `RunEditor`) but scoped to one fixed grove, with no grove-list/master pane.
  Multiple such proxies render independently (per-proxy render target, already in
  the seam design).
- **Home tab:** drop the dashboard's grove-list entirely; the home tab is the nav
  full-height. Global capture (`c`) is offered by the nav (routes a capture to a
  controller verb) — or deferred if it complicates this leaf; decide here.

## Done when

- Opening a grove shows **that grove's** task tree + inbox + capture in a detail
  pane beside its harness; triage/capture/`$EDITOR` work there (reusing the v1
  flows over the seam).
- Two groves open at once each show their **own** detail (N proxies, no
  cross-talk).
- The **home tab is the nav full-height**; the redundant home grove-list is gone.
- Switching workspaces preserves each tab's detail pane (zellij keeps tabs alive).

## Notes

- Reuses the v1 `App` detail rendering; the work is the **scoping** (one fixed
  grove, detail-only) and **N-proxy** lifecycle (accept/track/render multiple
  proxies, each tagged with its grove).
- Keep the seam codec unchanged where possible; a proxy may pass its `--grove`
  identity at connect so the controller binds the right render target.
- Depends on **090** (nav self-opens the tab; the layout it builds gains the proxy
  pane).
