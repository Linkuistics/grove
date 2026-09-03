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

## Carried forward from the art stage

`keyed-launch-k121` is closed. What `proof` needs from it:

- **The figures are settled, and so are the declines.** Sixty-two figures were
  enumerated across the thirteen pages and every non-exempt one carries an
  adjacent role statement; four relations that ran only in prose were drawn
  (chapter 8's `Watch`-against-`End` table, chapter 9's five-decoy table, and
  chapter 10's three-arms table and its split-roots diagram). Five more were
  declined **on editorial judgement**, each for a reason recorded in
  `03-DONE-art--keyed-launch-k121.md`; none was declined because the medium could
  not carry it, so nothing here reopens the contract's *Figures* medium.
- **The four exempt tables are `source-index.md`'s reconciled four**, and its
  fifth — the owned-source totals — is not exempt and states its role. `F009`
  forbids a lead-in above the four; do not add one.
- **The two chapter-10 figures replaced prose that carried the same relations.**
  The sentences that remain beside them carry the *reasons*, not the ranges, and
  are not stray fragments of a deleted paragraph.

## Notes

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If
they are not installed, stop and say so — authoring by hand wastes the pilot that
earned the pipeline.

**The corpus is frozen.** Do not edit `crates/keyed-launch/`. A defect found while
documenting becomes its own leaf, and that leaf carries the source change, every
affected ledger and page, and a green validator run over every book it touched,
in one commit — or it is deferred behind the books it would invalidate.
