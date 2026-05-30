# 080-async-revisit

**Kind:** planning (likely a short re-evaluation; may resolve to "no change")

## Goal

Re-evaluate concern 4 (the sync→async refactor) once the harness backend is
settled. Decide whether the event loop needs async at all, needs a minimal slice,
or stays sync.

## Context

- **The answer hinges on D2 (the 050 spike).** The original async case rested on
  "juggling subprocess output for harness panes." If 050 picks **tmux-owner**, the
  TUI never holds those ptys — tmux does — so that pressure stays gone, and the
  only candidate driver is watching N repos (the 070 fleet view), which v1 handles
  synchronously with `notify` + 200ms debounce → likely "keep sync." If 050 picks
  **in-process pty**, grove *does* hold N ptys and must pump their output in the
  loop — the original async pressure **returns**, and this leaf becomes a real
  refactor decision rather than a formality. The 050 spike explicitly answers
  "can N embedded ptys be pumped in the sync loop?" — read its findings first.
- Inputs: whatever 050 (spike) and 060 (harness pane) surface about event-loop
  pressure in practice.

## Done when

- A recorded decision: keep sync / add minimal async / full async refactor —
  with the concrete pressure (if any) that justifies it. If "keep sync," this
  leaf retires with a one-line rationale and an ADR/note so the deferred concern
  is formally closed rather than left dangling.

## Notes

Deliberately last. Its size now depends on D2: a formality if tmux wins (record
"async not needed" and close the deferred concern with a one-line ADR/note), a
genuine refactor decision if in-process pty wins. Either way, don't refactor for
its own sake — let the concrete pressure 050/060 surface justify it.
