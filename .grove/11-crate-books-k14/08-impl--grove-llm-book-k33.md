# grove-llm-book-k33

## Goal

Write the `grove-llm` book under `docs/walkthroughs/grove-llm/`: a complete,
source-exact walkthrough of the crate's 4 roots and 1,017 lines, in the shape the
human's structure brief settled, passing final validation.

## Context

- Inputs: **the structure brief at `docs/specs/grove-llm-book-structure.md`**
  (elicited at `grove-llm-structure-k32`, whose decision log carries every
  rejected alternative), the shared specification from
  `walkthrough-books-spec-k20`, and the pipeline `publishing-pipeline-k13`
  extracted. The brief states the reader and the outcome, the ordered chapter
  plan with each chapter's responsibilities, and what deserves emphasis and what
  the book does not cover — the three things `grove-draft` requires of a named
  artifact — and its chapter sequence and twenty-five ownership blocks are what
  `docs/walkthroughs/grove-llm/walkthrough.toml` records.
- **Two obligations outside the book, owed here before the book validates**
  (brief, *Outbound links* and *The book's row*): promote *Session epoch* and
  *Tree access lock* in `CONTEXT.md` from bold paragraphs to `###` headings with
  explicit anchors `session-epoch` and `tree-access-lock`, keeping each phrase
  as the heading text, as `overview-book-k30` did for `guaranteed-core`; and add
  the book's row to `docs/ARCHITECTURE.md`'s *Documentation ownership* table.
- **Three stale claims in the corpus are known in advance** (brief, *Known in
  advance*): the manifest's reachability claim beside the direct `jj-workspace`
  dependency, `lib.rs`'s *or `grove`*, and the `0.1.0` comment already leafed as
  `grove-llm-version-comment-k83`. Each page states the checkable fact beside
  the fragment; none is fixed from inside the book.
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
- Its links into `docs/USAGE.md` resolve, per the shared specification's
  guide-link contract, and `every_repository_markdown_reference_resolves` is what
  proves it.
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
