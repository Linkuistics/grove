# grove-llm-book-k33

## Goal

Write the `grove-llm` book under `docs/walkthroughs/grove-llm/`: a complete,
source-exact walkthrough of the crate's 4 roots and 1,017 lines, in the shape the
human's structure brief settled, passing final validation.

## Context

- Inputs: this book's structure brief, the shared specification from
  `walkthrough-books-spec-k20`, and the pipeline `publishing-pipeline-k13`
  extracted.
- The corpus, exactly: every `crates/grove-llm/src/**/*.rs` plus
  `crates/grove-llm/Cargo.toml` — 4 roots, 1,017 lines. Every byte belongs to a
  fragment graph; `tests/` is evidence, not a root.
- `src/cli.rs` carries 944 of the 1,017 lines, and its verb surface is what
  `docs/USAGE.md` documents for the human. The book explains the same surface for
  a reader of the code; do not restate the guide.
- Scoped proof exists so a partial book is provable. Validate per slice as you
  go rather than discovering at the end that the graph does not close.

## Done when

- `docs/walkthroughs/grove-llm/` holds the book and final validation over it passes
  with no deferred holes.
- It is uniform with the other books' page conventions, navigation and prose
  contract, and is gated by `scripts/check.sh` through the book discovery rather
  than a hand-added line.
- `bash scripts/check.sh` passes.

## Notes

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If
they are not installed, stop and say so — authoring by hand wastes the pilot that
earned the pipeline.

**The corpus is frozen.** Do not edit `crates/grove-llm/`. A defect found while
documenting becomes its own leaf, and that leaf carries the source change, every
affected ledger and page, and a green validator run over every book it touched,
in one commit — or it is deferred behind the books it would invalidate.
