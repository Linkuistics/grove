# 070-async-revisit

**Kind:** planning (likely a short re-evaluation; may resolve to "no change")

## Goal

Re-evaluate concern 4 (the sync→async refactor) **after** the tmux-owner
decision deflated its original justification. Decide whether the event loop
needs async at all, needs a minimal slice, or stays sync.

## Context

- 010-plan D2 rationale: the async case rested on "juggling subprocess output
  for harness panes." Under the tmux-owner model the TUI never holds those
  ptys — tmux does — so that pressure is gone. The remaining candidate driver is
  watching N repos (050), which v1 already handles synchronously with
  `notify` + 200ms debounce.
- Inputs: whatever 040/050 surface about event-loop pressure in practice.

## Done when

- A recorded decision: keep sync / add minimal async / full async refactor —
  with the concrete pressure (if any) that justifies it. If "keep sync," this
  leaf retires with a one-line rationale and an ADR/note so the deferred concern
  is formally closed rather than left dangling.

## Notes

Deliberately last and deliberately small. If 040 and 050 land cleanly on the
sync loop, the honest outcome is "async not needed" — record it and move on
rather than refactoring for its own sake.
