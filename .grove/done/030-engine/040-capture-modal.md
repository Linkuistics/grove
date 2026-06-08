# 040-capture-modal

**Kind:** work

## Goal

Build the **centered capture modal over the live harness pane** — the proof point for the
bug that motivated the whole migration. A native ratatui focus-overlay (E3/E4) that
renders centered over the *whole* window, over the live pane behind it, captures all keys
while up, and on submit writes an observation to a grove inbox. Under the zellij fork this
shape was impossible; under grove's owned loop it is `Clear` + a centered `Rect` + an
overlay widget (the spike's F2 modal, productionised).

Build:
- `tui::capture`: the modal widget (centered `Rect`, `Clear`, bordered block, a text
  input buffer with cursor) drawn *after* the pane each frame when `Focus = Modal`.
- Wire it into the E4 focus machine's `Modal` state (replacing 020's stub): opened from
  Nav (or a leader sub-key); captures all keys (text into the buffer, bracketed paste
  literal per E5); **Esc cancels**, **Enter submits**, both restoring the prior focus.
- On submit, perform grove's **capture write** below the seam (E1): shell out to
  `grove-llm inbox-add --to=<name> --body=…` (sync call from async context). Surface
  success/failure briefly. The write idiom and target-grove selection follow the existing
  capture/inbox model — don't invent a new one.

## Context

Depends on 010 (loop/pane render order — the modal must paint *over* the `PaneWidget`),
020 (Modal focus capture + paste), and 030 (Nav, the natural place to invoke capture from).
This is the leaf that **finalizes the landmark "rmux substrate" ADR** (D4): with the modal
working, the inversion thesis is fully demonstrated (grove owns the draw loop → centered
modals over live foreign panes are trivial → the capture-popup bug is structurally fixed),
so the ADR can record consequences + supersede pointers and reach `accepted`.

## Done when

- Pressing the capture key over a live harness shows a centered modal over the whole
  window; typing/paste fills it; Esc cancels; Enter writes the observation and restores
  focus to the harness.
- The modal renders correctly over the live pane (the motivating bug is demonstrably
  fixed) — verifiable headlessly (overlay-over-pane render into a `Buffer`).
- The landmark ADR is `accepted` (inversion thesis + E1–E6 + supersede pointers framing;
  the full ADR-tower dissolution sweep remains 050's teardown job per D4).
- `cargo build`/`cargo test` green. `grove tui` now meets the node's "done when":
  harness renders + takes input, capture modal works, minimal nav exists.

## Notes

The landmark ADR only needs to *exist and supersede at the thesis level* here — marking
the 0013–0028 tower Superseded one-by-one is **050**'s teardown (D4). Don't do the full
sweep in this leaf; draft the landmark + its supersede list and leave the per-ADR edits
to teardown. Capture's deeper UX (target-grove picker, templates) is out of scope —
minimal write is enough for the proof point.
</content>
