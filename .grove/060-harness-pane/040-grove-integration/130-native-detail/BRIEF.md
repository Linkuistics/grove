# 130-native-detail — brief

**Kind:** node (work)

## Goal

Render each grove's **detail** (task tree + inbox triage + capture + `$EDITOR`)
**natively, in-process, inside that grove's own [[workspace]] tab**, beside its
harness. The native realisation of the per-grove [[detail proxy]] (ADR-0019) —
without the dumb-terminal proxy, the socket, or the N-proxy lifecycle: just N
native [[host surface]]s the in-process host renders.

## Context

- **ADR-0020 + ADR-0019 (UX).** The per-grove detail UX stands: opening a grove
  shows *that grove's* task tree + inbox + capture next to its harness; the home
  tab is the nav (130 does not draw a grove list). The *mechanism* is the native
  [[host surface]] (110/030) bound to one grove, not a `grove __dash-proxy
  --grove <name>` dumb proxy over a seam.
- **Reuses the v1 `App` detail rendering** — the work is **scoping** (one fixed
  grove, detail-only) and **native lifecycle** (N independent surfaces). Capture
  and inbox triage already run in-process on the home dashboard surface
  (`DashboardSurface::process_action`, leaf 110/030) — the detail surface reuses
  those exact `grove-llm` shell-outs.

## Two forks settled at decompose (this session's grilling)

1. **The host-pane seam is single-tenant today and must widen to N.**
   `host_pane.rs` yields exactly *one* surface per session via a one-shot
   `take_host_surface()` consumed at first-layout (`screen.rs:3038`) — that one
   pane is the home nav. 130 needs a **detail host pane per grove tab**, so the
   injection widens from one-shot to **on-demand-per-tab** (leaf 010).

2. **The `$EDITOR` drop is NOT an "in-process tty hand-off" — that premise was
   stale.** It pre-dated ADR-0021, which established host surfaces render inside
   the **server daemon** (no tty; the thin foreground client owns the tty). So
   v1's `suspended()`→spawn-editor is structurally impossible. **Decision: run
   `$EDITOR` as a real trellis terminal pane** (vim/etc fully emulated natively),
   **observe its exit**, read the tempfile back, run `grove-llm inbox-edit`. This
   builds the first slice of ADR-0020 §6 **embedded-tool observability** — which
   `150-working-set` also needs — and is isolated in leaf 030.

## Done when (node acceptance — restates 130's, fork #2 applied)

- Opening a grove shows **that grove's** task tree + inbox + capture in a native
  detail pane beside its harness; triage / capture work in-process; `$EDITOR`
  (Ctrl-E) edits via a trellis editor pane.
- Two groves open at once each show their **own** detail, no cross-talk.
- Switching workspaces preserves each tab's detail (trellis keeps tabs alive).
- `cargo build`/`cargo test` green; grove core stays `ratatui`-free below the seam.

## Decomposition (this node)

```
010-multi-host-panes   trellis: widen the host-pane seam one-shot → on-demand;
                       a HostDriver verb opens a grove tab carrying a detail host
                       pane + the harness command pane. Framework foundation.
020-detail-surface     grove: a per-grove DetailSurface (scoped App, detail-only,
                       no grove list); nav's OpenHarness opens detail+harness.
                       Delivers the headline acceptance (minus $EDITOR).
030-native-editor      $EDITOR as a trellis terminal pane + embedded-tool exit
                       observability (first slice of ADR-0020 §6); wire
                       EditBody/EditObservation; seeds 150-working-set.
```

## Notes

- **Scope boundary with 150-working-set:** 150 owns the *responsive multi-pane*
  layout (harness + terminal + yazi + lazygit, pack-vs-degrade). 130 stays
  minimal — detail + harness in a simple split.
- Depends on **110** (host API) and **120** (nav opens the tab the detail lives in).
- An ADR for the embedded-tool-observability seam may earn its place at 030 (it is
  a reusable framework facility, not just grove glue) — raise it there if the shape
  stabilises, per `driving.md`.
