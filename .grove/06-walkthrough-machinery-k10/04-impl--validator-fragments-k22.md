# validator-fragments-k22

## Goal

Remove the validator's compiled-in knowledge of one book's *corpus*: source
roots, ownership blocks and early-use rows all come from the named book. Then
gate every book root under `docs/walkthroughs/` from `scripts/check.sh`.

This is the leaf that puts the fragment half of `plan-k1`'s decision 4 to bed —
the half whose scope widened when decision 1 reversed to complete source-exact
coverage, and the reason the validator sits on the campaign's critical path.

## Context

- The constants in scope: `ROOTS` (17 source paths with exact line counts),
  `BLOCKS` (34 top-level ownership ranges) and `EARLY_USES` in
  `crates/book-validation/src/`. Their per-book form is whatever
  `walkthrough-books-spec-k20` settled.
- `scripts/check.sh` today gates the book only through `cargo test --locked
  --workspace`, which runs `tests/corpus_validation.rs` against the one
  compiled-in book. "Gates every book" means enumerating the book roots that
  exist and running final validation over each, so that adding a sixth book adds
  a gate without editing the script.
- The line counts are exact and load-bearing: the root brief's freeze exists
  because a ledger holds a line count per source root, and an inline source fix
  shifts every line below it. Reading counts from data does not weaken that — it
  moves where the count is written down.

## Done when

- Final validation of the relocated book passes with no compiled-in root,
  block or early-use row anywhere in `crates/book-validation/src/`, established
  by enumerating and classifying every literal in that directory.
- `scripts/check.sh` runs final validation over every book root under
  `docs/walkthroughs/` and would pick up a new one with no edit to the script.
  Prove the discovery, not just the pass: a deliberately broken second book root
  must make the script fail, seen to fail, before a green run counts as evidence.
- `bash scripts/check.sh` passes.

## Notes

**This leaf is the campaign's gate.** Every book after it is proved by what lands
here, and a validator that is green over one book because it still knows that
book is a validator that will silently accept the next five. Spend the evidence
accordingly.

**Do not touch `crates/ordinal-fs-tree/`.** If a line count in the ledger
disagrees with the source, the ledger is what you have measured wrongly, or the
source has drifted and the campaign's freeze has already been broken — either way
it is a finding to report and, if it is a real source defect, its own leaf under
the root brief's cross-book rule.
