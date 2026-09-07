# template-source-read-count-k86

## Goal

Correct the one claim in `crates/grove-loop/src/session_config.rs` that the
loop does not bear out — the `TemplateSource` type's documentation saying *the
loop re-reads the configuration once per iteration* — and land it as a corpus
change the book contract permits.

## Context

- Observed at `the-surface-k78` and verified at `three-steps-k79` while drafting
  the overview, whose chapter 3 owns the early-use row for `TemplateSource`.
  `crates/grove-loop/src/loop_driver.rs` calls `templates.load(&delta_roots)`
  twice inside the loop body: once before `transition_to_current`, so the
  just-in-time presence rule for the finish leaf is asked against the document
  as it stood before the tree was mutated, and once after the leaf is selected,
  so the launch expands the selected kind's template from the document as it
  stands. The type's own doc comment names both reads in the same sentence
  that says *once*, so the count is stale rather than the design.
- The overview does not repeat the stale count. Its ledger row, its manifest
  `[[early-use]]` statement and the structure brief's table were amended at
  `three-steps-k79` to *twice, before and after the tree transition*, and
  chapter 3 states the two reads and what each is for. Nothing in the overview
  needs to change when the comment does.
- This file is a root of the `grove-loop` book (`grove-loop-book-k37`), which
  will own the `TemplateSource` documentation and must reconstruct it. A
  rewording that keeps the line count moves no boundary; one that changes it
  moves every range below it in that root.

## Done when

- The comment states what the loop does: two reads per iteration, and the
  reason each exists — the presence rule against the pre-transition document,
  the launch against the current one — with no count the code contradicts.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the
  `grove-loop` book, and a green `book-check --final` over that book.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside
`manifest-function-count-k82`, `grove-llm-version-comment-k83` and
`every-member-version-comment-k84`: editing a byte of a frozen root while the
book that quotes it is being written invalidates the ranges the freeze
protects. If `grove-loop-book-k37` lands first and quotes the comment as
written, its page adjudicates the stale count the way the overview's chapter 1
adjudicates the function count, and this leaf rewrites that paragraph in the
same commit.

## Decisions (running log)

1. **The rewording holds the comment's seven-line span exactly** (lines 88–94 of
   `crates/grove-loop/src/session_config.rs`, file still 358 lines), so no
   ownership range, manifest `lines` value or fragment fence moved and no other
   book's `check.sh` transcript was falsified. The sentence now reads *reads the
   configuration twice per iteration* and attaches each of its two clauses to
   the read it belongs to.
2. **Seven surfaces changed, only one of which `book-check` reads.** The source
   comment; `18-which-files.md`'s fragment `«config-template-source»` (the
   validated one) and its adjudicating section; `20-the-loop.md`'s two-reads
   section *and* a present-tense restatement 1,100 lines below it, in the
   chapter's preconditions summary, which shares none of that section's wording;
   `21-what-could-not-move.md`'s corpus-claims tally; three `concept-index.md`
   rows; and four places in `docs/specs/grove-loop-book-structure.md` — the
   summary bullet, the *Known in advance* preamble, item 2, and chapter 18's
   pointer.
3. **The tagged `CHANGELOG.md` entry was deliberately left alone.** v20.0.0's
   entry carries the same stale rationale (*because the loop re-reads it once
   per iteration*, line 265). Amending a released section falsifies the record
   of what shipped, and `every-member-version-comment-k84` set the precedent
   exactly here: its own claim survives verbatim at line 279 and k84 did not
   touch it. Neither leaf added an `Unreleased` entry either.
4. **The false uniqueness claim these pages carried was corrected, not
   re-asserted.** Three passages said this was *the only adjudication in the
   campaign whose claim and refutation are both inside one book's corpus*, and
   `20-the-loop.md` added *every other stale comment in this book is adjudicated
   against evidence the reader has to take on trust*. Both are false: chapter 5
   reproduces `task_tree.rs`'s header claiming canonicalisation appears in one
   place and chapter 6 reproduces `target`, the second site, from the same file
   (`canonicalisation-sites-k149`); and the manifest's `libc` clause was refuted
   by three production modules of this same crate. The quantifier was dropped in
   favour of the property that is true and is what the paragraphs were for — the
   reader can count the calls without leaving the book — and the spec's summary
   bullet was corrected from *one of them* to *two of them* refuted in-corpus.
5. **Two broken citations in the same file were externalised, not absorbed.**
   `18-which-files.md` nominated this leaf to carry `session_config.rs` line
   271's *requirement 6* (a record that does not exist) and line 331's Markdown
   link to `../docs/adr/untracked-configuration-delta.md` (an address that
   resolves from nowhere), because it was already editing this file's comments
   within the frozen line counts. Neither serves this leaf's stated goal, so
   both went to `requirement-six-citation-k189` and both chapter sections were
   re-pointed at it. The second had been assigned to this leaf by
   `which-files-k166` and would otherwise have been orphaned.
