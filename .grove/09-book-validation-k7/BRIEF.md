# book-validation-k7 — brief

## Goal

Deliver deterministic checks that make each book increment verifiable and make
the final completeness claim mechanical.

## Context

`book-system-k6` owns the input formats and validator contracts. This subtree
implements those contracts without changing the book design to suit an
implementation shortcut.

## Done when

- Exact fragment expansion and source coverage are tested through success and
  failure fixtures.
- Markdown structure and every local page, file, and heading link are checked
  deterministically without fetching external URLs.
- The commands are documented, non-interactive, stable in their diagnostic
  ordering, and usable both on a scoped authoring increment and on the complete
  book.
- The repository's normal verification exercises the validators or an explicit
  book-verification command runs both with one documented invocation.

## Decomposition

- `fragment-validation-k8` proves the literate-programming seam and exact source
  reconstruction.
- `markdown-validation-k9` proves book shape and local navigation independently
  of fragment expansion.

## Pointers

- Design contract: `book-system-k6`.
- Exact source corpus: `ordinal-fs-tree-book-k10`'s brief.
- CLI conventions for any command surface: the `cli-tool-design` skill.

## Notes

Validators fail closed on ambiguous input and report all deterministic findings
that can be collected safely in one run. They do not assess prose quality or
technical truth; the later independent reviews own those judgments.
