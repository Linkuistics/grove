# 040-history-ring

**Kind:** planning

## Goal

Make open-in-editor *usable* (010-plan D1/D2): build a grove-owned per-pane
rendered-scrollback ring fed by `render_stream()`'s rmux-rendered snapshots, and wire
open-in-editor to dump the ring. **Opens with a validation spike** before committing —
the reconstruction is the risk, not the wiring.

## Context

rmux has no rendered-history capture (`snapshot`/`find_text` are visible-screen only;
`line_stream` is raw ANSI bytes — the spike's strip-ANSI dump was unusable, which is what
made this load-bearing). The chosen source (D2) is `render_stream()`: rmux-RENDERED
snapshots on each revision change. The ring reconstructs scrolled-off rendered lines by
diffing successive snapshots. Real standalone copy-mode/scrollback/search remain out of
scope (D1) — this leaf only delivers the editor stand-in over clean rendered history.

## Done when

The validation spike has answered whether snapshot-diffing reconstructs usable scrollback;
if yes, the per-pane ring + open-in-editor dump are built and tested headlessly; if the
heuristic proves unreliable, the fallback decision (escalate the upstream ask, or revisit
D2) is recorded. The upstream rmux rendered-history feature request is filed either way.
A focused ADR records the ring approach (D4).

## Open questions / spike targets

- Does diffing successive `render_stream` snapshots reliably recover scrolled-off lines
  across: a fast scroll, a partial scroll, a screen clear, `\r` in-place redraws, and a
  resize/reflow? (This is the empirical risk D2 flagged.)
- Alternate-screen apps (vim/htop) have no scrollback by design — confirm the ring simply
  reflects "current screen" for them and that that's acceptable for open-in-editor.
- Ring sizing/eviction policy; per-pane vs shared; memory bound.
- Exact open-in-editor flow under the grove-owned loop (suspend raw mode → `$EDITOR` →
  restore — the spike proved this is trivial when grove owns the loop).

## Notes

Upstream ask: rmux daemon already emulates and likely retains scrollback; the correct
long-term source is a `capture_region(start = -N)` / tmux `capture-pane -S` equivalent
that would let the ring retire. File against `Helvesec/rmux`.
