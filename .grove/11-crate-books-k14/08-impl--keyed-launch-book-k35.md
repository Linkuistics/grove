# keyed-launch-book-k35

## Goal

Write the `keyed-launch` book under `docs/walkthroughs/keyed-launch/`: a complete,
source-exact walkthrough of the crate's 9 roots and 2,073 lines, in the shape the
human's structure brief settled, passing final validation.

## Context

- Inputs: this book's structure brief, the shared specification from
  `walkthrough-books-spec-k20`, and the pipeline `publishing-pipeline-k13`
  extracted.
- The corpus, exactly: every `crates/keyed-launch/src/**/*.rs` plus
  `crates/keyed-launch/Cargo.toml` — 9 roots, 2,073 lines. Every byte belongs to a
  fragment graph; `tests/` is evidence, not a root.
- The crate never learns what a launch is *for*, and grove's mapping onto it is
  one line: a session kind is a key. Keep the book on the crate's side of that
  line — a book that explains grove's sessions has documented the wrong crate.
- Scoped proof exists so a partial book is provable. Validate per slice as you
  go rather than discovering at the end that the graph does not close.

## Done when

- `docs/walkthroughs/keyed-launch/` holds the book and final validation over it passes
  with no deferred holes.
- It is uniform with the other books' page conventions, navigation and prose
  contract, and is gated by `scripts/check.sh` through the book discovery rather
  than a hand-added line.
- Its links into `docs/USAGE.md` resolve, per the shared specification's
  guide-link contract, and `every_repository_markdown_reference_resolves` is what
  proves it.
- `bash scripts/check.sh` passes.

## Notes

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If
they are not installed, stop and say so — authoring by hand wastes the pilot that
earned the pipeline.

**The corpus is frozen.** Do not edit `crates/keyed-launch/`. A defect found while
documenting becomes its own leaf, and that leaf carries the source change, every
affected ledger and page, and a green validator run over every book it touched,
in one commit — or it is deferred behind the books it would invalidate.
