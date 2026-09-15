# lifecycle-rows-k10


## Goal

Make every visible item's lifecycle readable before its name, with the agreed
leading tick/cross and independent cursor. This working increment builds on
full-width-view-k9 and is useful before runtime activity exists.



## Context

Implement Typed row data and Tree layout in `docs/specs/item-status.md`.
`crates/grove-tui/src/observation.rs` currently formats lifecycle into Row.label
and computes descendant totals in reverse pre-order. Move identity, handle,
kind/species, depth, lifecycle and counts into typed row data and format them
in `Viewer::render` in `crates/grove-tui/src/lib.rs`; never parse label strings
back into domain facts. Preserve whole-subtree counts and key-based state.

Use the public Viewer/TestBackend seam and the branch aggregate, navigation,
Markdown Unicode and live-transition cases in `crates/grove-tui/tests/browser.rs`.
Reuse the existing grapheme-width and control-text handling where appropriate;
verify APIs from the locked dependency source before choosing rendering helpers.

## Done when

- The 22-cell prefix reserves cursor (2), lifecycle marker (2), word (10) and
  activity (8) before indentation. Activity cells stay blank until a later
  increment supplies typed activity. Leading labels are ✓ DONE and ✗ ABANDONED;
  LIVE/EMPTY use no terminal marker. Words remain explicit.
- DONE marker, word and item text are green; ABANDONED is red; LIVE/EMPTY use
  normal foreground. Selection adds only the separate > gutter, with no
  row-wide reverse video or highlight modifier. Fold markers remain separate.
  Color-disabled output distinguishes lifecycle, cursor and folding by text.
- Root and branches aggregate LIVE, else DONE, else ABANDONED, else EMPTY.
  Their separate totals include all descendant leaves through folded branches.
- Deep trees, long slugs, Unicode and control characters at 60 × 10 cannot
  consume status cells. Of the 36 remaining inner cells, reserve at least 16
  for recognizable handle text and two for the fold indicator; bound indentation
  and mark compression with an ellipsis. Elide the slug before the key suffix,
  fit kind/counts afterwards, and never split graphemes.
- Tests assert typed behavior through rendered words and styles for selected
  and unselected lifecycle states, mixed and empty branches, depth/width limits,
  renaming, retirement and decomposition. Existing navigation and saved file
  positions still pass; viewing creates no filesystem state.
- Update the viewer usage and architecture sections to describe the shipped
  rows. Run `cargo test --locked -p grove-tui` and the root brief's principal
  checks after edits, including any affected source-derived documentation.

## Notes

The spec's bold-yellow RUNNING item/name treatment and bold normal NEXT word
belong to the activity consumers, which can use these typed fields without a
second row model. This leaf does not manufacture those statuses. A remaining
substantive rendering doubt earns a review-impl leaf with stem lifecycle-rows
once this artifact exists.
