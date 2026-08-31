# validator-structure-k21

## Goal

Remove the validator's compiled-in knowledge of one book's *structure*: page
inventory, page-to-slice mapping, canonical slice order and the `--through`
token set all come from the book directory named by `--book`.

## Context

- The design this implements is `walkthrough-books-spec-k20`'s. Read that spec
  and its ADR first; if what you find there does not answer a question this leaf
  hits, that is a gap in the spec, not a licence to invent an answer.
- The constants in scope: `SLICE_ORDER` and `PAGE_BY_OWNER`, plus
  `SOURCE_INDEX`; and the hard-coded slice list in `src/cli.rs`'s `--through`
  `value_parser`, which today rejects any token outside the ordinal book's seven
  authorable slices. `src/cli.rs`'s `about` and `after_help` name one book by
  path and must stop doing so.
- Out of scope, and left exactly as they are for the next leaf: `ROOTS`,
  `BLOCKS`, `EARLY_USES`.
- The `--check markdown` / `--check fragments` seam is the split. This leaf owns
  the Markdown side; `validator-fragments-k22` owns the other.

## Done when

- `book-check --book docs/walkthroughs/ordinal-fs-tree --final --check all` is
  green, and so is every scoped `--through` invocation the crate's tests make.
- No page filename, page identifier, slice token or slice ordering appears as a
  literal in `crates/book-validation/src/`, established by enumerating every
  candidate token in that directory and classifying each — not by sweeping a
  pattern list.
- `--through` accepts the slices the named book declares and refuses tokens it
  does not, with a diagnostic that names the book.
- `bash scripts/check.sh` passes.

## Notes

**The diagnostic contract is a tested surface.** `tests/diagnostic_contract.rs`
and the `F0nn` / `U0nn` codes in the book spec are part of what the crate
promises. Codes that stop having a subject, or that gain one, are a change to
that contract and belong in the commit that makes it — not left for a later
session to notice.

**A test fixture that hard-codes the ordinal book is the same defect as a
constant.** The generalisation is not done if the suite can only be run against
one book; a second, synthetic book fixture is the cheapest proof that it is.
