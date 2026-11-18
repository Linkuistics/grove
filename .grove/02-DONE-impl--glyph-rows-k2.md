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

## Decisions (running log)

**The fixed area is nine cells.** A row starts with four parts, and a space
follows each part: the depth, the cursor, the lifecycle glyph and the activity
glyph. For example, a selected retired leaf at depth 12 that still runs starts
`12 > ✓ ► `. The spaces keep each marker in its own column with colour off.
At 60 columns the item then has 49 of the 58 inner cells.

**The depth column takes its width from the deepest row of the whole tree.**
Folded rows count too. So folding never shifts the columns, and every row
widens together when one row is deeper than 99.

**The summary glyphs are plain text.** The brief keeps today's colours, and
today the summary lines have none. So `►` and `»` in the summary lines are a
legend of shapes, and a yellow `►` never appears beside `RUNNING: none (idle)`.

**The count glyphs take the row's colour.** Today the whole item text takes the
row's colour, and the counts are part of the item.

**The gate failures come from the toolchain, so a sibling owns them.** The
final `bash scripts/check.sh` fails 2 of its 18 checks, `cargo nextest` and the
Ravel provenance check. All 38 failing tests report that a sibling binary was
not built or that `CARGO_BIN_EXE_grove` is unset. The base revision `89eba142`
fails the same way, so this leaf did not cause them.

The viewer, formatting
and every `grove-tui` and `view_terminal` test pass. This leaf therefore
retires against *Done when* narrowed by those inherited failures only.
`cargo-build-layout-k5`, inserted next, owns "the gate passes" for the grove.
