# ordinal-totals-table-k71

## Goal

Give `ordinal-fs-tree` the owned-source totals table its book contract requires
and its `source-index.md` does not have, and decide whether the validator should
be the thing that says so.

## Context

- The specification is unconditional: *`source-index.md` also carries an
  owned-source totals table — slice, page, and owned lines, with a total row
  naming the root count and the corpus line count*
  (`docs/specs/walkthrough-books.md`, *Source and ownership ledger*). It is a
  fifth table on that page, distinct from the four under fixed H2 headings.
- `docs/walkthroughs/jj-workspace/source-index.md` has one, under
  `## Owned source totals`. `docs/walkthroughs/ordinal-fs-tree/source-index.md`
  has none: its headings stop at `## Early uses` and the page ends with that
  table. The relocated book predates the clause and was never reconciled to it.
- **Nothing catches this.** `crates/book-validation/src/ledger.rs` requests
  exactly the four fixed headings — `Source roots`, `Ownership blocks`,
  `Fragment index`, `Early uses` — and recomputes those against the manifest. The
  totals table is required by prose and checked by nobody, which is why both
  books pass `book-check --final` today.
- The totals are derivable: ownership blocks carry per-block line counts and the
  manifest assigns each block's owner to a page, so a totals row per slice and the
  17-root, 8,720-line total are computable rather than authored guesses
  (`book-check docs/walkthroughs/ordinal-fs-tree` reports `17 files, 8720
  resolved lines`).
- The table is a figure and takes an adjacent statement of its role: it is
  outside the four-table `F009` carve-out, settled by `figure-contract-k70`
  (`docs/specs/walkthrough-books.md`, *Figures*).

## Done when

- `ordinal-fs-tree`'s `source-index.md` carries the totals table with a role
  statement, and its numbers reconcile with the manifest and the validator's own
  reported line count.
- A decision is recorded — in the commit or, if it earns one, an ADR — on whether
  the totals table joins the machine-reconciled set. Either the validator
  recomputes and compares it like the other four, or the specification says
  plainly that it is an editorial table the author maintains, so a reader is not
  left assuming a check that does not exist. Silence is the one outcome this leaf
  may not leave behind.
- No source root is touched, so the corpus freeze is not engaged.
- `bash scripts/check.sh` passes.

## Notes

Surfaced by `figure-contract-k70` while triaging finding `F3`, which asked
whether the fifth table earns the four-table exemption. It does not — but
answering that question is what showed one of the two books does not carry the
table at all. That divergence is a book defect rather than a figure-contract
question, so it is a leaf of its own rather than work absorbed into the
integration session.

Placed at the end of `crate-books-k14` beside the other measured defect leaves.
It blocks no book: a new book written to the contract carries the table from the
start, and this leaf closes the one that does not.
