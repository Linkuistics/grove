# grove.grove-view-keep-current-leaf-vertically-centered — brief

## Goal

Make `grove view` keep the loop's position in sight and make the tree quicker
to move around. The tree view follows the current leaf and keeps it vertically
centred. It gains vim-style movement keys and a key that goes to the current
leaf. Its rows become compact: a depth column, one glyph per marker and bold
kinds.

## Done when

- The behaviour matches *The settled design* below in every point.
- The viewer section of `docs/USAGE.md` and `docs/specs/item-status.md`
  describe the new behaviour in place. No statement of the old words, keys,
  cursor rule or viewport rule remains.
- Tests at the agreed seams cover every point of the settled design.
- `bash scripts/check.sh` passes.

## Decomposition

The requirements session `plan-k1` settled the whole design with the human, so
no planning leaf follows. Three `impl` leaves are flat siblings under the root,
because they share this brief's context and need no scope of their own. Each
leaf is a vertical slice that a session can show and test alone. The order
encodes one dependency: following reuses the viewport control that `zz`, `zt`
and `zb` add.

1. `glyph-rows`: the row presentation.
2. `vim-keys`: the movement keys and the help overlay.
3. `follow-current-leaf`: following, centring, the startup position and `c`.

## Pointers

- Glossary terms in play: **Current leaf**, **RUNNING**, **NEXT** and **Tree
  lifetime** (see `CONTEXT.md`).
- Spec for this area: `docs/specs/item-status.md`. It owns the row layout, the
  interaction contract and the acceptance table. Each leaf updates the sections
  that its slice changes.
- Usage guide: `docs/USAGE.md`, section *Viewing a tree*.
- No ADR applies. No decision here passes the ADR test, because each one is
  easy to reverse.
- Test seams that the leaves share: *Test seams* below.

## The settled design

### Rows

- The columns come in this order: depth, cursor, lifecycle glyph, activity
  glyph, then the item. The item is the indentation, the fold indicator, the
  handle, and the kind or the branch counts.
- The depth column shows the row's tree depth. The root is at 0, and a child of
  the root is at 1. The column is always two digits wide and right-aligned. A
  depth above 99 widens the column rather than lose digits.
- The lifecycle glyphs are `◦` LIVE, `✓` DONE, `✗` ABANDONED and `∅` EMPTY. The
  activity glyphs are `►` RUNNING and `»` NEXT. They replace the words in rows.
- Each glyph has neutral East Asian width, so it takes one cell in every
  terminal locale. Each shape differs with colour off, as the spec requires.
- Colours stay as today. DONE rows are green, ABANDONED rows are red, the
  RUNNING row is bold yellow and the NEXT glyph is bold. LIVE and EMPTY use the
  normal foreground.
- The kind is bold.
- Branch counts use the glyphs, for example `◦1 ✓2 ✗0`, instead of
  `branch [LIVE 1 DONE 2 ABANDONED 0]`.
- The RUNNING and NEXT summary lines keep their words and gain `►` and `»`, as
  a built-in legend.
- The fixed area before the item shrinks from 22 cells to about 9.

### Keys

- `gg` and `G` go to the first and the last row, as Home and End do.
- Ctrl-f or `f` moves a page down, and Ctrl-b or `b` moves a page up. In Tree
  view, PageDown and PageUp do the same.
- Ctrl-d or `d` moves half a page down, and Ctrl-u or `u` moves half a page up.
- In Tree view, the page keys move the cursor and the view together, as in vim.
  So the cursor keeps its place on the screen.
- `zz`, `zt` and `zb` scroll the tree so that the cursor row sits in the
  middle, at the top or at the bottom of the pane. They do not move the cursor.
- `c` takes the cursor to the current leaf and centres it there.
- File view takes the same keys for scrolling, except `zz`, `zt` and `zb`,
  because File view has no cursor. Ctrl-u and Ctrl-d there change from a full
  page to half a page.
- The viewer holds a pending `g` or `z`. After a prefix, any other key cancels
  the prefix and then acts as usual.
- The help overlay lists every key and a glyph legend, and it still fits at
  60 × 10.

### Following and centring

- The current leaf is the RUNNING item while a session runs, otherwise NEXT.
- The cursor follows the current leaf only while it rests on it. When the
  current leaf changes, a cursor on the old one moves to the new one. A cursor
  elsewhere stays where the human left it.
- At startup the cursor starts on the current leaf, centred. With no current
  leaf, it starts on the root, as today.
- The viewer centres the current leaf at three moments: at startup, when
  following moves the cursor, and when the human presses `c`. Between those
  moments the view scrolls only as far as it must to keep the cursor visible.
  So `zz`, `zt` and `zb` hold until the current leaf next changes.
- A cursor away from the current leaf never moves by itself.
- While activity or the tree cannot be observed, the viewer keeps the last
  current leaf that it saw. It does the same while NEXT is none. A resting
  cursor follows when a different leaf next becomes current. So a brief WAITING
  never breaks following.
- A tree replacement clears the remembered current leaf with the other saved
  state. The viewer then behaves as at startup.
- Following unfolds a branch to reveal the new current leaf, as a selection
  move does today.
- Following works in both views, because they share one selection. In File
  view the file changes to the new current leaf's file. Each file keeps its own
  reading position.
- Near the top or the bottom of the tree, the view clamps. The current leaf
  then sits off-centre rather than leave blank rows.
- With no current leaf, `c` shows a notice and moves nothing.

### Test seams

The application seam proves all three slices: `Viewer::new`, `act`, `tick` and
`render`, over temporary trees, drawn into the Ratatui `TestBackend`. Tests
drive it with real key events through `Action::from_key`, as the existing tests
do. A retired leaf moves NEXT, which changes the current leaf without a live
session. One scenario uses the fixture in
`crates/grove-tui/tests/witnessed.rs` that runs a real driver and session. It
proves RUNNING as the current leaf and the handoff to NEXT when the session
ends. The work adds no new seam.

## Notes

- The retired herdr pane, `herdr-grove-plugin-k5` (change `pmowxyyvwllr`), is
  the precedent for centring and for glyphs. Its `○` and `▶` were rejected,
  because they have ambiguous East Asian width.
- Today the `List` widget draws the cursor gutter as its highlight symbol,
  which always comes first in a row. A depth column to its left therefore needs
  each row to draw its own cursor cell.
- Two spec rules change. The cursor stays independent of RUNNING only until the
  human rests it on the current leaf. The viewport no longer moves only as far
  as needed, because of the centring moments above.
