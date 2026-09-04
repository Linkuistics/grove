# manifest-function-count-k82

## Goal

Correct the one count in `crates/grove/Cargo.toml` that the source does not bear
out — *the reason it is three functions long*, lines 31–32 — and land it as a
corpus change the book contract permits.

## Context

- Observed at `orientation-k77` while drafting the overview's chapter 1, which
  owns lines 27–33 of the manifest as the fragment `manifest-dependencies`.
  Outside the test module the crate defines two functions, `main` and `run`;
  counting the test helper `undescribed` makes three, and counting the two
  `#[test]` functions makes five. No reading a reader takes first yields three.
- The page states the structural fact beside the fragment — two production
  functions, one a one-line call to the other — and says the comment is
  reproduced as written. Once the comment changes, that paragraph in
  `docs/walkthroughs/overview/01-orientation.md` is wrong the other way and
  must be rewritten in the same commit.
- Prefer a wording with no count at all (*the reason there is nothing else in
  it*), on the rule in the grove spine's `references/execute.md`: a claim
  documented by a count of itself goes stale the moment the thing it counts
  changes.

## Done when

- The comment states something the source bears out, and the chapter-1
  paragraph that adjudicates it is rewritten to match.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, the affected fragment and page, and a green
  `book-check --final` over the overview book. A one-line rewording moves no
  line boundary; a rewording that changes the line count moves every boundary
  below it and must re-prove the whole `source-crate-manifest` root.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside `jj-docs-url-k64` and its
neighbours: editing a byte of a frozen root while a book that quotes it is being
written invalidates the ranges the freeze protects.
