# plan-k1

## Goal

Establish what `grove view` must do, in the human's own words. The grove's name,
`grove-view-keep-current-leaf-vertically-centered`, was the only statement of
the request before this session. During the session the human added more
requirements, so the request now has five parts:

1. Follow the loop's leaf and keep it vertically centred in the tree.
2. Vim-style movement keys for the tree list.
3. The kind in bold.
4. One coloured Unicode character per marker, instead of the word.
5. A depth column at the far left of the tree.

## Context

- `crates/grove-tui/src/lib.rs`: the tree pane renders through the stateful
  Ratatui `List`. The list moves its viewport only as far as it must to keep the
  `>` selection visible. At startup the selection is the root row. The function
  `tree_item` draws each row.
- `crates/grove-tui/src/terminal.rs`: `Action::from_key` decodes one key event
  into one action. Ctrl-u and Ctrl-d move a full page, and only in File view.
- `docs/USAGE.md#usage-viewing-tree` and `docs/specs/item-status.md`, sections
  *Tree layout* and *View switching and chrome*: the current row, viewport and
  cursor contract. The spec says that the cursor stays independent of RUNNING.
  It also says that every marker stays distinguishable with colour disabled.
- The retired herdr pane, `herdr-grove-plugin-k5` (change `pmowxyyvwllr`, file
  `herdr-plugin/grove_tree.py`): the precedent. Its function `window` centred
  the window on the first live leaf in walk order and clamped the window at the
  ends of the tree. Any scroll key froze the window, and `r` resumed centring.
  Its function `render_leaf` marked rows with `✓`, `✗`, `○` and, for the
  current leaf, `▶`.

## Done when

- The human confirms a shared understanding of the behaviour.
- The settled requirements and the agreed test seams are recorded where the
  next session reads them.

## Notes

## Decisions (running log)

**"Current leaf" names the loop's leaf, not the cursor.** The human chose the
leaf that the loop works on. The tree view follows the loop as it advances, as
the retired herdr pane did. The idle case is still open.

**The tree list gains vim-style movement keys.** The human asked:

<!-- tc: off because the block quotes the human's own words -->
> I also want to add gg/G/f/b/u/d/zz (or whatever the standard vim bindings
> are) for moving around the list
<!-- tc: on -->

Vim moves a page with Ctrl-f and Ctrl-b, and half a page with Ctrl-u and
Ctrl-d. `gg` and `G` go to the first and last line, and `zz` centres the cursor
line. Still open: whether the bare letters also work, and whether File view
takes the same keys.

**Rows change their presentation in three ways.** The human asked:

<!-- tc: off because the block quotes the human's own words -->
> Also the kind should be in bold, and the DONE/ABANDONED/LIVE/CURRENT markers
> should use a single unicode char (with the appropriate colour) rather than
> the word. Finally I want a tree depth column in the far left
<!-- tc: on -->

The glossary has no CURRENT, so the meaning of that marker is still open. The
glyph set and the form of the depth column are also open.

**The cursor follows the current leaf only while it is on it.** The human said:

<!-- tc: off because the block quotes the human's own words -->
> The cursor should auto-move if it was on the current leaf when the current
> leaf changes.
<!-- tc: on -->

So a move of the cursor away from the current leaf pauses following, and a move
back onto it resumes following. A cursor elsewhere stays where the human left
it. No separate follow mode is needed.

**NEXT stands in while idle.** The human chose NEXT as the current leaf when no
session runs. So the current leaf is the RUNNING item while a session runs, and
NEXT otherwise. It never disappears between sessions, so a resting cursor moves
straight on to the next leaf. `CONTEXT.md` now defines **Current leaf**.

**CURRENT is the human's word for RUNNING.** The human chose two activity
glyphs, one for RUNNING and one for NEXT. So the current leaf always carries one
of them, and a row still shows whether a session runs. Each row keeps two marker
slots, because a leaf can be DONE and still RUNNING until its session ends.

**The vim keys include the bare letters and `zt` and `zb`.** The human chose
the vim set plus two extras. So `gg`, `G`, `zz`, `zt` and `zb` work. Ctrl-f or
`f` moves a page down, and Ctrl-b or `b` a page up. Ctrl-d or `d` moves half a
page down, and Ctrl-u or `u` half a page up. `zz`, `zt` and `zb` move the view
and leave the cursor where it is.

**The glyph set is "play and next".** The human chose `►` for RUNNING and `»`
for NEXT, with `◦` LIVE, `✓` DONE, `✗` ABANDONED and `∅` EMPTY. Each glyph has
neutral East Asian width, so it takes one cell in every terminal locale. Each
shape also differs with colour off, as the spec requires. The human chose from
a mockup that showed this row order: depth column, cursor, lifecycle glyph,
activity glyph, then the item. The mockup showed the root at depth 0, bold
kinds, today's colours, and branch counts as glyphs such as `◦1 ✓2 ✗0`.

**The test seams are the existing ones.** The human agreed that the application
seam proves all five parts: `Viewer::new`, `act`, `tick` and `render` into the
Ratatui `TestBackend`, driven by real key events through `Action::from_key`.
One scenario uses the real-launch fixture in `tests/witnessed.rs` to prove
RUNNING as the current leaf. The work adds no new seam. The viewer itself holds
a pending `g` or `z`, so the key sequences stay testable at the application
seam.

**A key takes the cursor to the current leaf.** The human asked:

<!-- tc: off because the block quotes the human's own words -->
> We should have a key binding to take the cursor to the running (or next if
> there is no running) leaf
<!-- tc: on -->

That target is the current leaf. By the cursor rule, the key also resumes
following. The human chose `c`, for "current". The key also centres the
current leaf.

**The depth column is always two digits wide.** When the derived points were
put to the human, the human amended one:

<!-- tc: off because the block quotes the human's own words -->
> depth should be two digits - many trees are more than 9
<!-- tc: on -->

So the column reserves two cells and right-aligns the depth. A depth above 99
widens the column rather than lose digits.

**The human confirmed the shared understanding.** The human accepted the
derived points, with the depth amendment above, and chose three `impl` leaves.
The root brief now holds the settled design and the agreed seams. The leaves
are flat siblings under the root, because they share that brief's context and
need no scope of their own. In order, they are `glyph-rows-k2`, `vim-keys-k3`
and `follow-current-leaf-k4`.
