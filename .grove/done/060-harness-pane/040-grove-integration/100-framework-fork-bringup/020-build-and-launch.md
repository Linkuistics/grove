# 020-build-and-launch

**Kind:** work (foundation)

## Goal

Get `cargo build` to produce a **working forked-mux binary that launches on the
dev platform** (Darwin arm64) — a bare shell pane is enough — and do the
**minimal rebrand**: strip zellij branding from the chrome/startup we control,
set grove's defaults **in source** (no bars, locked-friendly) rather than via the
old bundled-config `clear-defaults`/bars gymnastics (ADR-0020 makes those
unnecessary).

## Context

- Depends on **010-vendor-and-seam** (the `trellis` crate(s) compiling in the
  workspace). This leaf makes the vendored mux actually *run*.
- "Launchable" bar is low and deliberate (constraint 4): launch the forked
  multiplexer, get a shell pane. Native grove surfaces (dashboard, nav, detail,
  whichkey, working set) are **110+**, not here.
- **Minimal rebrand only:** chrome/startup branding + grove defaults in source.
  Full rebrand / public naming is deferred to the extraction leaf (ADR-0020 §7,
  later/lazy).
- Earlier leaves validated tamed config knobs on zellij 0.44.3 (locked mode, no
  bars, command panes `start_suspended false`) — now expressed **in source**.

## Done when

- `cargo build` produces a working binary that launches the forked mux on the
  dev platform; a bare shell pane renders and is interactive.
- zellij branding is stripped from the startup/chrome grove controls; grove's
  no-bars / locked-friendly defaults are set in source (not a bundled kdl).

## Notes

- Per-platform build pipeline, GraphQL, observability API are **later/lazy**
  (ADR-0020 §7) — do not pull forward.
- Keep changes minimal and surgical; this is a launch checkpoint, not a
  feature pass.
