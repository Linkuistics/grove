# grove-loop-book-k37 — brief

## Goal

Write the `grove-loop` book under `docs/walkthroughs/grove-loop/`: a complete,
source-exact walkthrough of the crate's 13 roots and 10,533 lines, in the shape the
human's structure brief settled, passing final validation.

## Context

- Inputs: **the structure brief at
  [`docs/specs/grove-loop-book-structure.md`](../../../docs/specs/grove-loop-book-structure.md)**
  — `grove-draft` stops without a named artifact — the shared specification from
  `walkthrough-books-spec-k20`, and the pipeline `publishing-pipeline-k13`
  extracted. The brief settles the spine, twenty-one pages with exact block
  ranges, the carried example, the stated outcome, the per-chapter prose
  obligation, twelve declared anchors and the slice IDs. No second interview is
  owed.
- The corpus, exactly: every `crates/grove-loop/src/**/*.rs` plus
  `crates/grove-loop/Cargo.toml` — 13 roots, 10,533 lines. Every byte belongs to a
  fragment graph; `tests/` is evidence, not a root.
- This is the campaign's largest book by a wide margin and it will almost
  certainly become a node. Decompose along the chapter boundaries the structure
  brief names, doing only the first child. The brief's **five parts** — 436;
  1,714; 2,541; 2,725; 516; 2,601 lines — are the intended draft groupings.
- **Thirty-eight per cent of the corpus is inline test code and none of it is
  excluded.** Five roots carry a `#[cfg(test)] mod tests`: `tree_lifecycle.rs`
  1,649 lines, `task_tree.rs` 1,008, `task_name.rs` 694, `driver_lease.rs` 564,
  `loop_driver.rs` 69 — 3,984 in all. The manifest declares exactly **one**
  `[[corpus.exclude]]`, `crates/grove-loop/src/task_grow/tests.rs` class
  `inline-test-module`, matching the specification's inventory; declaring any
  other turns
  `every_books_corpus_exceptions_are_exactly_the_specifications_inventory` red.
- **Two claims in the corpus are known false and are adjudicated on the page**,
  not repeated and not corrected: `src/lib.rs` line 68 (*every member takes
  `version.workspace = true`* — `book-validation` does not; fixed later by
  `every-member-version-comment-k84`) and `src/session_config.rs` line 89 (*the
  loop re-reads the configuration once per iteration* — `src/loop_driver.rs`
  lines 241 and 260 load twice; fixed later by
  `template-source-read-count-k86`). Both fixing leaves sit after this one, so
  each will rewrite the adjudicating paragraph in the same commit as the comment.
- **One edit outside the book is owed and no glossary promotion is:** the
  `grove-loop` row in `docs/ARCHITECTURE.md`'s *Documentation ownership* table,
  whose wording the brief gives. All twelve declared anchors exist today, so
  `M201` is green from the first slice.
- **This is the last book, and it unblocks `architecture-residue-k75`
  entirely.** Thirty-one of `docs/ARCHITECTURE.md`'s forty-one residue markers
  name this crate, including the two joint ones whose other books are written
  (line 1189 with `keyed-launch`, line 1324 with `jj-workspace`). The brief maps
  every marker to a chapter; that map is the coverage obligation.
- The `ordinal-fs-tree` glossary and grove's collide on *leaf* and *node* and
  differ on *ordinal* / *position* and *key* / *permanent key*
  (`CONTEXT-MAP.md`). A page speaking of both must say which tree it means,
  sentence by sentence.
- Scoped proof exists so a partial book is provable. Validate per slice as you
  go rather than discovering at the end that the graph does not close.

## Done when

- `docs/walkthroughs/grove-loop/` holds the book and final validation over it passes
  with no deferred holes.
- It is uniform with the other books' page conventions, navigation and prose
  contract, and is gated by `scripts/check.sh` through the book discovery rather
  than a hand-added line.
- Its links into `docs/USAGE.md` resolve, per the shared specification's
  guide-link contract, and `every_repository_markdown_reference_resolves` is what
  proves it.
- `bash scripts/check.sh` passes.

## Decomposition

The four editorial stages, cut lazily in pipeline order, each as the last act of
the stage before it. Only the first exists today.

1. `grove-loop-k123` — the **draft**, itself a node of seven children cut along
   the structure brief's five parts, with chapter 1 and the assembly page
   standing alone. Its brief carries the figures and the per-part obligations.
2. `copy-edit`, then `art`, then `proof` — each cut by the stage before it under
   `plugins/grove/skills/grove/references/editorial.md`, each carrying
   `grove-loop` as its whole slug, and none skipped on a judgement that it would
   find nothing.

A defect an earlier stage owns becomes a contiguous run of re-run leaves from
that stage through `proof`, in place of the ordinary last act; there is no
integrate step and no backward edge.

## Carried forward from the draft's first slice

- **The structure brief says four dependencies; the manifest declares five**, and
  chapter 1 says five. `structure-brief-dependency-count-k132` corrects the
  brief. A later stage that reconciles the page to the brief would introduce the
  error rather than remove it.
- **The book's row in `docs/ARCHITECTURE.md`'s *Documentation ownership* table
  is in**, in the wording the structure brief carries. Nothing else outside the
  book is owed, and no glossary promotion was owed: all twelve declared anchors
  already exist in explicit form.

## Notes

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If
they are not installed, stop and say so — authoring by hand wastes the pilot that
earned the pipeline.

**The corpus is frozen.** Do not edit `crates/grove-loop/`. A defect found while
documenting becomes its own leaf, and that leaf carries the source change, every
affected ledger and page, and a green validator run over every book it touched,
in one commit — or it is deferred behind the books it would invalidate.
