# keyed-launch-book-k35 — brief

## Goal

Write the `keyed-launch` book under `docs/walkthroughs/keyed-launch/`: a complete,
source-exact walkthrough of the crate's 9 roots and 2,073 lines, in the shape the
human's structure brief settled, passing final validation.

## Context

- Inputs: **`docs/specs/keyed-launch-book-structure.md`** — this book's structure
  brief, settled at `keyed-launch-structure-k34` and named here by path because
  `grove-draft` stops without a named artifact — the shared specification from
  `walkthrough-books-spec-k20`, and the pipeline `publishing-pipeline-k13`
  extracted. The brief settles ten pages, nine owning source; the spine *the
  words are the words the file holds*; the pass-through test as the stated
  outcome; and the ownership mapping the manifest's `[[page]]` and `[[block]]`
  groups record.
- The corpus, exactly: every `crates/keyed-launch/src/**/*.rs` plus
  `crates/keyed-launch/Cargo.toml` — 9 roots, 2,073 lines. Every byte belongs to a
  fragment graph; `tests/` is evidence, not a root.
- The crate never learns what a launch is *for*, and grove's mapping onto it is
  one line: a session kind is a key. Keep the book on the crate's side of that
  line — a book that explains grove's sessions has documented the wrong crate.
- Scoped proof exists so a partial book is provable. Validate per slice as you
  go rather than discovering at the end that the graph does not close.
- **`src/channel.rs` lines 272–404 are an inline `#[cfg(test)] mod tests` and are
  inside the corpus.** `docs/specs/walkthrough-books.md`'s corpus exception
  inventory carries no `keyed-launch` row, so those 133 lines are owned,
  reconstructed and explained like any other — by chapter 9, not chapter 6. Do
  not add an exception row to make them go away; that would be a specification
  edit the brief did not settle.
- **One obligation falls outside the book, and only one**:
  `docs/ARCHITECTURE.md`'s *Documentation ownership* table has no `keyed-launch`
  row, and `every_book_root_has_a_documentation_ownership_row` is red until it
  does. The brief's *The book's row in the ownership table* carries the wording.
  No glossary promotion is owed — all three reserved anchors
  (`usage-running-grove`, `usage-session-lifecycle`, `loop-control-channel`)
  already exist in explicit form, so `book-check`'s `M201` is green from the
  first slice.
- `docs/ARCHITECTURE.md` line 1188's `residue(grove-loop, keyed-launch)` marker
  is made redundant by chapters 6–8, but the deletion is **joint** and is
  `architecture-residue-k75`'s, only once the `grove-loop` book has also landed.
  This leaf neither edits nor cites that document.

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

## Handed forward

- **`copy-edit` — `docs/walkthroughs/keyed-launch/07-the-job.md`.** That page
  writes its em and en dashes as the HTML entities `&mdash;` and `&ndash;` (49
  and 4 occurrences); chapters 1-6 and 8 use the literal characters and carry
  none. Nothing renders wrong and no validator sees it, so it is house-style
  consistency across the whole document rather than a defect in the page —
  `copy-edit`'s class, and invisible to a stage reading one chapter. Found at
  `the-escalation-k115`, which followed the majority convention.

## Notes

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If
they are not installed, stop and say so — authoring by hand wastes the pilot that
earned the pipeline.

**The corpus is frozen.** Do not edit `crates/keyed-launch/`. A defect found while
documenting becomes its own leaf, and that leaf carries the source change, every
affected ledger and page, and a green validator run over every book it touched,
in one commit — or it is deferred behind the books it would invalidate.
