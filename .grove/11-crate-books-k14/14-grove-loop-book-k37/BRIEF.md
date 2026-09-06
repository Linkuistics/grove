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
the stage before it. The first is complete and the second is cut.

1. `grove-loop-k123` — the **draft**, a node of seven children cut along the
   structure brief's five parts, with chapter 1 and the assembly page standing
   alone. **Complete**: all seven children retired, the book green at
   `final=true`, the node closed. Its brief carries the figures, the per-part
   obligations and the findings promoted here.
2. Two `impl` leaves the draft's last child cut, both prose-only and both ahead
   of the editorial stages in the walk — `residue-map-one-pick-k178` (the
   structure brief's residue map attributes one marker to two chapters that do
   not cover it) and `core-fallible-construct-counts-k179` (two figures in
   chapter 19 that no counting rule reproduces).
3. `copy-edit--grove-loop-k180`, then `art`, then `proof` — each cut by the stage
   before it under `plugins/grove/skills/grove/references/editorial.md`, each
   carrying `grove-loop` as its whole slug, and none skipped on a judgement that
   it would find nothing.

A defect an earlier stage owns becomes a contiguous run of re-run leaves from
that stage through `proof`, in place of the ordinary last act; there is no
integrate step and no backward edge.

## Carried forward from the draft, which is complete

**The draft is done and green.** `grove-loop-k123` retired with the book at 13
files, **10,533 resolved lines, 0 deferred, `final=true`**, all thirty-nine
ownership rows `resolved` and all fifty early-use rows `explained`; `bash
scripts/check.sh` passes all eight principal checks, which it had not done at any
earlier point in this book's drafting. Twenty-one pages, two lookup surfaces, one
contents page. Read the whole book as the draft left it and change only what your
own stage's charter owns.

**The charter split, restated because the next three stages depend on it.** The
draft owned **structure** — conceptual order, what each chapter is for, whether
the book delivers its stated outcome — and **technical truth** — whether what the
book says about the crate is true against the crate. Both were discharged. A fix
that reorders a chapter, re-attributes a test, or re-adjudicates a claim about
the source is a **correction run**, not a copy edit, an art pass or a proof: cut
the contiguous run the family file describes, from the owning stage through
`proof`, in place of your ordinary last act.

**What is frozen and what is not.** The literal fragments are exact source bytes
and may not be touched — `book-check` compares them byte for byte and `F008`
reports a single character. Everything outside a four-backtick fence is prose the
stages own. `crates/grove-loop/` is frozen outright.

**The two known-false corpus claims are adjudicated on the page and must stay
that way.** `src/lib.rs` line 68's *every member takes `version.workspace =
true`* (chapter 1) and `src/session_config.rs` line 89's *the loop re-reads the
configuration once per iteration* (chapters 18 and 20, refuted by
`src/loop_driver.rs` lines 241 and 260). `every-member-version-comment-k84` and
`template-source-read-count-k86` hold the fixes and both sit after this book;
each will rewrite its adjudicating paragraph in the same commit as the comment.
**Do not "tidy" an adjudication into a correction, and do not delete one.**

**Many more comment defects are adjudicated and deliberately unfixed**, each with
its own leaf under `crate-books-k14` — stale enumerations, a helper list naming a
function that never existed, four comments addressing `grove-llm`'s CLI by a
module name this workspace does not have, five unresolved intra-doc links, a
parenthesised anchor citation naming nothing in the repository, and a decision
record whose stated signature has drifted from the shipped one. A stage that
reads one of these as an error in the *book* has misread it: the book is
reporting, not asserting.

**Manifest data that is not prose.** Every chapter's H1 must equal its
`walkthrough.toml` `title`, the navigation labels reuse it, and every anchor named
by an early-use row is a first-use target. A heading or anchor change on one of
those is a manifest change, not an edit.

**The link contract, which reads as an omission if you do not know it.** A book's
local targets are closed to its own pages, its own roots, `docs/USAGE.md` and
`CONTEXT.md`. Decision records, `docs/specs/*`, `docs/CONFIGURATION.md`,
`docs/ARCHITECTURE.md` and `CONTEXT-MAP.md` are **named in prose by backticked
path and never linked** — a linkified one turns `M201` red. This book also
**cannot link to another book**: where `grove-loop` calls `ordinal-fs-tree`,
`keyed-launch` or `jj-workspace`, the page names the crate in prose and stops.

**One outstanding defect, with its own leaf.** The structure brief's residue map
attributes `docs/ARCHITECTURE.md`'s *the one pick and what it serves* marker to
chapters 7 and 14; neither covers it and chapter 20 covers it in full.
`what-could-not-move-k130` enumerated all thirty-one markers naming this crate by
subject and found this the only wrong attribution; the coverage itself holds, so
`architecture-residue-k75` is not blocked, but its checklist is wrong until the
leaf beside this one lands. Chapter 21 records the coverage as delivered, so
**the page and the brief will disagree until that leaf reconciles them** — that
is known and owned, not a defect for a later stage to fix.

**The dependency count is settled at five, in the brief and on the page**
(`structure-brief-dependency-count-k132`). The structure brief said four where the
manifest declares five; it now states five in both places, names the four that
carry a reason in situ, and says what `jj-workspace` — the fifth, which carries
none — is reached for. A later stage reconciling the page to the brief does not
risk reintroducing the error.

**Nothing outside the book is owed.** The `grove-loop` row in
`docs/ARCHITECTURE.md`'s *Documentation ownership* table is in, in the wording the
structure brief carries; no glossary promotion was owed, because all twelve
declared anchors already existed in explicit form.

## Handed forward

- **`art` — chapter 21 draws two tables and no figure.** The twenty-by-three
  application of the three questions and the four-row enforcement ladder are both
  relations the prose contract required to be drawn. Whether the three questions
  themselves want a figure — they are the book's stated outcome and are drawn
  nowhere in twenty-one pages — is `art`'s call and not the draft's, and the
  draft deliberately left it rather than pre-empting the stage.

## Notes

**It was authored through the pipeline, and the remaining stages are the rest of
it.** This leaf became a node at `grove-loop-k123` under `--kind draft`; that
stage is complete and `copy-edit` is cut. `art` and `proof` follow, each as the
last act of the stage before it, and neither is skipped on a judgement that it
would find nothing.

**The corpus is frozen.** Do not edit `crates/grove-loop/`. A defect found while
documenting becomes its own leaf, and that leaf carries the source change, every
affected ledger and page, and a green validator run over every book it touched,
in one commit — or it is deferred behind the books it would invalidate.
