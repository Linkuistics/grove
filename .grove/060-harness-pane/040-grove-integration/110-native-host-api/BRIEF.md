# 110-native-host-api — brief

**Kind:** node (build — framework MVP). **Decomposed when picked** into three
strictly-stacked leaves (boot → render+input → port+drive); the carve is
mechanical (architecture settled by ADR-0020/0021), not a re-grilling.

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

## Decomposition (this node)

Three leaves, each building strictly on the prior, each a focused commit. The
seam they realise is ADR-0021's three points (grove owns `main` + role-dispatch;
substitute `ClientOsApi`/`ServerOsApi`; native `Pane` impl as a third pane kind).

```
010-boot-role-dispatch   grove owns `main`; link zellij-client/-server/-utils;
                         dispatch client vs `--server`; bring up a live trellis
                         session FROM the grove binary (stock terminal pane proves
                         the `current_exe --server` re-exec). Foundation — nothing
                         renders without it.
020-native-pane-render   the third `PaneId`/`Pane` kind: a host ratatui buffer →
                         `CharacterChunk`s server-side, composited as a real pane;
                         in-process key/mouse/resize/focus delivery (crossterm
                         model, replacing the 010 hand-rolled socket decoder).
                         The headline render+input proof.
030-port-dashboard-drive port the v1 `App`/`RepoView` dashboard to render through
                         the native pane; create/close/focus tab+pane by direct
                         call (subsumes the proxy seam + `zellij action` driving);
                         document the host-API seam. Acceptance: the v1 dashboard
                         runs natively, no proxy socket anywhere.
```

The node's original "Done when" (below) is the acceptance for the *whole* node,
met when 030 retires.

## Notes

- Resist gold-plating the API — it is **discovered by a real consumer** (grove);
  add surface only as 120–150 demand it (constraint 4). GraphQL/network exposure is
  explicitly out of scope (ADR-0020 §5).
- Depends on **100** (the fork builds; hosting-API direction chosen — ADR-0021).
