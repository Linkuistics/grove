# vim-keys-k3

## Goal

Add the vim-style movement keys that the root brief's *Keys* section settles,
in Tree view and in File view. List them in the help overlay. The `c` key
belongs to `follow-current-leaf`, not to this leaf.

## Context

- `Action` and `Action::from_key` in `crates/grove-tui/src/terminal.rs` and
  `crates/grove-tui/src/lib.rs`. Today Ctrl-u and Ctrl-d move a full page in
  File view only, and Tree view ignores the page keys.
- `docs/specs/item-status.md`, section *View switching and chrome*, and the key
  table in `docs/USAGE.md`.

## Done when

- Every key in the brief's *Keys* section works as stated, except `c`.
- `zz`, `zt` and `zb` move the tree view and leave the cursor in place. The view
  then holds until the cursor leaves the pane.
- After `g` or `z`, any other key cancels the prefix and then acts as usual.
- The help overlay lists every key and a glyph legend, and it fits at 60 × 10.
- Application-seam tests drive each key through `Action::from_key`, including
  the two-key sequences.
- The key table in the usage guide and the spec describe the new keys.
- `bash scripts/check.sh` passes.

## Notes

- The viewer holds the pending prefix, not the key decoder, so the application
  seam can test the sequences.
- The tree view needs viewport control of its own for `zz`, `zt` and `zb`.
  `follow-current-leaf` reuses that control to centre the current leaf.

## Decisions (running log)

**The `b` key has its own action.** Alone, `b` moves a page up. After `z`, it
ends `zb`. If the decoder mapped `b` to `PageUp`, the viewer could not tell
`zb` from `z` and Ctrl-b. So `Action::B` reaches the viewer, and the viewer
resolves it. The same holds for `g`, `z` and `t`.

**A key with no binding reaches the viewer as `Action::Unbound`.** The brief
says that any other key cancels a prefix. A key that the decoder drops would
leave the prefix waiting. So `from_key` returns `None` only for a key release.

**A Ctrl-modified `g`, `z`, `t` or `G` is unbound.** In raw mode, Ctrl-z
arrives as a key, not as a stop signal. A sequence must not start from it.

**In File view, `z` still waits for its second key.** The completed `zz`, `zt`
or `zb` then does nothing. So `zb` in File view never acts as a page up, and
the placement does not wait for Tree view.

**A prefix is not held while help or the resize notice shows.** Navigation
pauses there, and a prefix is part of navigation. Refresh, Escape and quit
still cancel a prefix.

**A placement waits for the next tree render.** `act` stores the wanted place,
and `render` applies it with the pane height and the visible rows of that
frame. `follow-current-leaf` can store `Place::Middle` at each centring moment,
including startup, before the first render knows the pane height.

**Every placement clamps at both ends of the tree.** The brief states the clamp
for centring, and the three placements share one control. So `zt` near the end
of the tree puts the cursor row lower, without blank rows. This differs from
vim, which shows blank rows below the last line.

**An even pane height puts `zz` on the upper of its two middle rows.** With
eight rows, three rows show above the cursor and four below.

**At an end of the tree, a page key moves the cursor alone.** The view stops at
the first or the last page, and the cursor moves on by the same step. This
matches vim's Ctrl-d at the end of a buffer.

**The help overlay has eight lines, which fill the 60 × 10 minimum.** The
half-page line has 36 cells, so `follow-current-leaf` can add ` | c: current
leaf` to it and stay within the 58 inner columns.
