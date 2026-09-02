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
