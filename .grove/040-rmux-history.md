# 040-rmux-history

**Kind:** planning

## Goal

Make open-in-editor *usable* (010-plan D1) by adding **native rendered-history capture to
rmux via a grove fork** (D7), then wiring open-in-editor to dump that clean rendered
history. **Opens with a scoping spike** before committing — the size of the rmux change is
the risk, not the wiring.

## Context

rmux has no rendered-history capture (`snapshot`/`find_text` are visible-screen only;
`line_stream` is raw ANSI bytes — the spike's strip-ANSI dump was unusable, which is what
made this load-bearing). D7 supersedes the earlier grove-ring idea (D2): instead of
reconstructing history by diffing `render_stream` snapshots (heuristic, fragile across
clears / partial scrolls / resize-reflow), grove forks rmux and exposes the daemon
emulator's *actual* rendered scrollback. No grove ring. The same capability is what the
deferred real copy-mode/scrollback/search (D1) will later build on — this is an
investment, not a stopgap.

## Done when

- The scoping spike has determined whether the daemon emulator already retains a rendered
  scrollback buffer (expose it) or must gain one (add it), and the rmux change is made on
  the fork accordingly: a `capture_region(start = -N)` / `capture-pane -S` equivalent that
  returns rendered cells (not raw bytes) from history.
- grove's dependency points at the fork (SDK + daemon); open-in-editor dumps the captured
  rendered history and is genuinely readable (the bar D1 set — not the spike's raw dump).
- Tested headlessly. A focused ADR records the fork + capture-API decision (D4/D7).
- An upstream PR is opened against `Helvesec/rmux` (fire-and-forget; don't block on merge).

## Open questions / spike targets

- **Sizing (first):** does rmux's daemon emulator (rmux-core/rmux-pty/rmux-server) already
  keep a scrollback grid (then: expose via a new proto verb — small) or only the visible
  grid + a raw byte backlog (then: add scrollback to the emulator — medium)?
- API shape: how `capture_region(-N)` returns rendered cells over rmux-proto; how it
  reaches the SDK (`Pane::capture_history(...)` or similar) and `ratatui-rmux` if relevant.
- Alternate-screen apps (vim/htop) have no scrollback by design — confirm history capture
  simply reflects "current screen" for them and that that's acceptable for open-in-editor.
- Fork mechanics: where the fork lives, how grove's Cargo deps point at it (git vs vendored
  path), and how the **daemon binary** built from the fork is found/bundled (overlaps the
  050 daemon-bundling area; `SDK_DAEMON_BINARY_ENV`).
- Open-in-editor flow under the grove-owned loop (suspend raw mode → `$EDITOR` → restore —
  the spike proved this is trivial when grove owns the loop).

## Notes

The fork tracks upstream: rebase the one rendered-history patch onto new rmux releases
until the PR merges, then drop back to the published crate. This is the bounded ongoing
cost D7 accepted (contrast the deliberately-frozen zellij fork).
