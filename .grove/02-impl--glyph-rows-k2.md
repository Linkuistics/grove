# glyph-rows-k2

## Goal

Make each tree row compact, as the root brief's *Rows* section settles. Add a
two-digit depth column at the far left. Use one glyph for each lifecycle and
activity marker. Make kinds bold, and show branch counts as glyphs. Add `►` and
`»` to the RUNNING and NEXT summary lines.

## Context

- The row renderer and the chrome in `crates/grove-tui/src/lib.rs`.
- `docs/specs/item-status.md`, section *Tree layout*, and its acceptance rows
  on lifecycle cues, colour and the 60 × 10 minimum.
- `docs/USAGE.md`, section *Viewing a tree*: the paragraphs on status words,
  colours and narrow widths.

## Done when

- Rows render as the brief's *Rows* section states, at sizes down to 60 × 10,
  with colour on and with colour off.
- Application-seam tests read the depth column, each glyph, the bold kind, the
  colours and the branch counts from the rendered buffer. Existing tests that
  read the old words now read the glyphs.
- The spec's *Tree layout* and the usage guide describe the new rows. No
  statement of the old status words remains.
- `bash scripts/check.sh` passes.

## Notes

- Keep the cursor gutter. The `List` highlight symbol always comes first in a
  row, so a depth column on its left needs each row to draw its own cursor cell.
- Keep a deep row's handle and permanent key recognisable at 60 columns, as the
  spec requires today.
