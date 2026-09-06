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

## Decisions (running log)

1. **The divergence is one book of six, not one of two.** The task file was
   written when two books existed. Enumerating the H2 headings of all six
   `source-index.md` pages: `grove-llm`, `grove-loop`, `jj-workspace`,
   `keyed-launch` and `overview` all carry `## Owned source totals`;
   `ordinal-fs-tree` alone does not. The finding stands and is stronger.

2. **The totals table is exactly derivable, and that is measured rather than
   argued.** A derivation from `walkthrough.toml` alone — chapters in manifest
   order for the slice and page columns, the sum of each owner's `[[block]]`
   ranges for owned lines, and `[[root]]` for the total row — reproduces all
   five existing tables **byte-for-byte**. Positive control: five of five
   identical. Negative control: flipping one `[[block]]`'s owner in
   `grove-llm`'s manifest moved two rows (101→70, 100→131), so the derivation
   reads the manifest rather than the page it is compared against. The
   manifest loader already guarantees the two invariants this needs — every
   chapter declares a slice, and every block owner is some chapter's slice.

3. **The validator recomputes the totals table.** Of the two outcomes the
   `Done when` allows, this is the one this leaf's own discovery argues for:
   the table went missing from a book for the whole life of its relocation and
   every `book-check --final` run stayed green, because a heading nobody asks
   for cannot be reported missing. The specification's own justification for
   the four restatements — *safe precisely because it is checked in both
   directions rather than trusted* — applies to a fifth table that is equally
   derived. Making it editorial-and-saying-so would leave the hole open for a
   seventh book.

4. **The figure treatment survives; only the reconciliation claim changes.**
   The clause `figure-contract-k70` settled makes three claims: the validator
   requests exactly four headings, nothing reconciles the totals, and `F009`'s
   no-lead-in rule names only the four. Only the third is a *figure* fact. The
   totals table stays outside the no-lead-in exemption — it still takes its
   adjacent role statement, and the new check tolerates that lead-in without
   inspecting it, because the validator checks no figure's role statement
   anywhere and this leaf does not start.

5. **No ADR.** The `ADR-FORMAT.md` test is an AND of three, and this decision
   clears at most two: there is a real rejected alternative (leave it editorial
   and say so), but the check is cheap to remove and the specification now
   states plainly what binds, so it is neither hard to reverse nor surprising
   in context. `docs/adr/a-book-carries-no-asset.md` needs no rework either —
   its claim is that the validator is ignorant of *figures*, and that still
   holds: the totals table's role statement is permitted and unread, and only
   its rows are reconciled. The durable record is the specification; this log
   and the commit carry the reasoning.

6. **The in-session review allowance is unspent.** The correctness at issue is
   established by executable seams rather than by argument: the derivation
   reproduces five books byte-for-byte, the new tests go red against a wrong
   cell, a wrong total row and a broken fixture, and `scripts/check.sh` gates
   all six books. Nothing here is a narrow unexpected claim the compiler and
   the suite cannot reach, so the leaf-wide reviewer is left for a leaf that
   needs it.
