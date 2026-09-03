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

- **`copy-edit` — `docs/walkthroughs/keyed-launch/concept-index.md`.** Two
  entries carry the byte-identical label *The vocabulary is an input to `load`,
  not to `expand`* and point at different targets —
  `01-orientation.md#the-map`, where the library root's own section list names the
  rule, and `02-the-names.md#an-input-to-load`, where the chapter argues it. Both
  entries earn their place; a reader scanning 332 curated entries cannot tell them
  apart, and one label needs rewording to say which is the map and which is the
  argument. Whole-document label consistency, invisible to a chapter-local read
  and found only by enumerating the index. Found at `what-passes-through-k117`,
  which owns neither chapter's wording.

## Carried forward from the draft

`keyed-launch-k107` is closed; the book exists and validates. What its brief held
that the remaining three stages still need:

- **The prose obligation is directional, and a whole-document stage is exactly
  where it gets flattened.** 33% of the corpus is comment prose, unevenly:
  `src/run.rs` is 53% and argues its cases in situ, `src/templates.rs` is 13% and
  is where every rule the records state actually binds. So chapters 3, 4 and 5
  **supply** an argument the source does not make, while chapters 7 and 8 **do not
  restate** one the fragment graph has just quoted verbatim. A stage that evens
  the chapters out — padding where the source is strong, trimming where it is
  silent — has broken the obligation in both directions while making the book look
  more uniform.
- **Chapter 8 must not attribute the child's group leadership to `setpgid` on both
  sides of the fork.** `src/run.rs` lines 591–592 say that and are wrong;
  `command.process_group(0)` is the whole of it, measured thirty times with
  controls at `the-job-k114`. Chapter 7's `#the-latch-and-the-child-away` carries
  the measurement. No page is wrong today, and `spawn-ordering-comments-k119` owns
  the source fix — a later stage that "corrects" a page against those two comment
  lines would introduce the defect the draft avoided.
- **Nothing inside the corpus was found stale**, so no chapter carries an
  adjudication obligation and none should acquire one. The single known drift is
  outside the corpus and is `runner-sketch-drift-k106`'s.
- **`docs/ARCHITECTURE.md`, `CONTEXT-MAP.md`, the specs and the four decision
  records are cited by backticked path and never by link.** The outbound-link
  contract closes a book's local targets to its own pages, its own roots, the
  guide and the glossary, so linkifying one of those turns `M201` red.
- **Chapter 10 says *seven tests*, and seven is right.** The structure brief's
  *Audience and intended outcome* names seven — two in, three through, two out.
  `keyed-launch-k107`'s brief and `what-passes-through-k117`'s task file both said
  six; the miscount was theirs, not the brief's. The page separately notes that it
  names nine tests in all. Do not reconcile either number downward.

## Notes

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If
they are not installed, stop and say so — authoring by hand wastes the pilot that
earned the pipeline.

**The corpus is frozen.** Do not edit `crates/keyed-launch/`. A defect found while
documenting becomes its own leaf, and that leaf carries the source change, every
affected ledger and page, and a green validator run over every book it touched,
in one commit — or it is deferred behind the books it would invalidate.
