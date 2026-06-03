# 130-native-detail

**Kind:** work

## Goal

Render each grove's **detail** (task tree + inbox triage + capture) **natively,
in-process, inside that grove's own [[workspace]] tab**, beside its harness. This is
the native realisation of the per-grove [[detail proxy]] (ADR-0019) — without the
dumb-terminal proxy, the socket, or the N-proxy lifecycle: it is just N native
surfaces the in-process host renders.

## Context

- **ADR-0020 + ADR-0019 (UX).** The per-grove detail UX stands: opening a grove
  shows *that grove's* task tree + inbox + capture next to its harness; the home tab
  is the nav (130 does not draw a grove list). The *mechanism* changes — the old
  `grove __dash-proxy --grove <name>` dumb proxy over a seam is replaced by a native
  detail surface bound to one grove, rendered in-process via the 110 host API.
- **Port the v1 detail views:** task tree, inbox list + disposition, capture modal,
  and the **`$EDITOR` drop** — the v1 flows port across the ADR-0013 seam. The
  `$EDITOR` drop is now an **in-process** tty hand-off (the old `RunEditor` seam
  frame is unnecessary — the host owns the tty).
- **Per-grove binding** is trivial now: each native detail surface is constructed
  for its grove when the nav opens the tab (120); no `--grove` connect handshake,
  no cross-talk risk.

## Done when

- Opening a grove shows **that grove's** task tree + inbox + capture in a native
  detail pane beside its harness; triage / capture / `$EDITOR` work in-process.
- Two groves open at once each show their **own** detail, no cross-talk.
- Switching workspaces preserves each tab's detail (trellis keeps tabs alive).
- `cargo build`/`cargo test` green; grove core stays `ratatui`-free below the seam.

## Notes

- Reuses the v1 `App` detail rendering; the work is **scoping** (one fixed grove,
  detail-only) and **native lifecycle** (N independent surfaces), now far simpler
  without the proxy seam.
- Depends on **110/120** (host API; nav opens the tab the detail lives in).
