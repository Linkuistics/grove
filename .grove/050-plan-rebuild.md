# 050-plan-rebuild

**Kind:** planning

## Goal

Continue planning the migration past the minimal-usable milestone. This is the
**tail planning leaf** (deliberately kept so the tree never falsely reads "done" before
the rebuild is actually scoped — ADR-0011). Grill each remaining area and grow its
leaves; decompose this leaf into a node as the areas firm up.

## Context

010-plan settled the cross-cutting engine (D1–D6) and the near-term leaves (020 rip-out,
030 engine, 040 history-ring). The areas below were enumerated by the founding seed but
deferred to their own grilling sessions rather than pre-decided here (lazy decomposition).
By the time this leaf is picked, the engine (030) exists, which will sharpen several of
these (focus model, pane/session model, layout primitives).

## Areas to grill + grow (each likely its own leaf)

- **Surfaces** — nav / per-grove detail / capture / whichkey rebuilt as plain ratatui
  widgets. Under rmux these stop being proxies/plugins/host-surfaces (the dissolved
  tower). What survives of the ADR-0019 "A′" UX (constant nav + swapped content)?
- **Working set** — multi-pane layout, aux panes (plain term / yazi / lazygit-lazyjj),
  park-alive (rmux splits/sessions keep panes alive off-screen natively — does this
  replace the suppress/restore + `replace_pane` machinery of ADR-0023?), responsive tiers
  (the ~220-col breakpoint).
- **Daemon lifecycle + bundling + launch** — vendoring/shipping the **forked** rmux daemon
  binary (D7: grove ships *our* build, not the published one) via `SDK_DAEMON_BINARY_ENV` /
  `connect_or_start`, session naming/persistence, how `grove tui` launches, fleet singleton
  + multi-repo (ADR-0025/0027) under rmux.
- **Detach + web path** — rmux session persistence/detach; rmux `web-share` (HTTP share
  URLs, opt-in daemon capability) vs grove's whole-UI-on-web goal. Likely research-first.
- **Teardown** — dissolve the ADR-0013–0028 tower (D4 mark-dissolved sweep + finalise the
  landmark/focused ADRs), retire the `bugs` grove, clean up `CONTEXT.md` (the superseded
  trellis/proxy/host-surface entries).

## Done when

The remaining areas are decomposed into leaves with briefs (or settled inline where
small), the tree reflects the full rebuild, and a teardown leaf exists. A PRD is written
only if an increment is a genuine human-facing agreement point.

## Notes
