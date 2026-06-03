# 110-native-host-api

**Kind:** work (framework MVP — **may decompose** when picked)

## Goal

Build the **minimal native-host API** of the [[trellis framework]] and prove it by
**rendering grove's v1 dashboard natively, in-process, as a trellis pane** — no
proxy socket, no WASM. This is the framework MVP that the rest of grove's UX
(120–150) is built on, and the concrete replacement for the superseded proxy seam
+ `zellij action` driving.

## Context

- **ADR-0020.** Rendering is in-process; the [[controlling process]], the
  [[dashboard proxy]] seam (010/020), and external `zellij action` driving (040)
  all dissolve into direct in-process calls. This leaf builds the API those direct
  calls go through.
- **Minimal API surface** (the smallest that unblocks 120–150):
  - render a native **ratatui** surface as an in-process pane (a host-app draws,
    trellis composites it as a real pane);
  - native **layout/pane/tab** control (create/close/focus tab; create/close/focus
    pane) by direct call — no `zellij action`, no protobuf;
  - native **input + event** delivery to the focused host surface (the v1
    crossterm `KeyCode`/mouse model, now in-process, replacing the 010 hand-rolled
    socket decoder).
- **Port, don't rewrite:** the v1 `App` dashboard rendering and the `RepoView` data
  layer port across the ADR-0013 seam unchanged (ratatui above; `RepoView`/writes
  below). Only the *transport* changes (socket frames → in-process draw).
- **Embedded-tool plumbing is foreshadowed but not built here** — that is trellis's
  headline (150 exercises it); 110 only needs grove's *own* native surface working.

## Done when

- A host app (grove) renders a native ratatui surface as a trellis pane, receives
  input/events in-process, and drives pane/tab/focus by direct call — the v1
  dashboard runs this way with no proxy socket anywhere.
- The minimal host API is documented at the seam (it is the contract 120–150 build
  on, and the first cut of what a future extracted `trellis` publishes).
- `cargo build`/`cargo test` green; grove core stays `ratatui`-free below the seam.

## Notes

- Resist gold-plating the API — it is **discovered by a real consumer** (grove);
  add surface only as 120–150 demand it (constraint 4). GraphQL/network exposure is
  explicitly out of scope (ADR-0020 §5).
- If the host-render path and the input/event path each want a session, decompose.
- Depends on **100** (the fork builds; hosting-API direction chosen).
