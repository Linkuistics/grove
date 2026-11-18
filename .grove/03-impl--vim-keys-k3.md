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
