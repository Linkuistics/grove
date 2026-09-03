# two-documents-k110

## Goal

Draft chapter 3 of the `keyed-launch` book — *Two documents, neither one
assembled*, `03-two-documents.md`, slice `never-assembled` — owning
`src/templates.rs` lines 92–145 (54) and 276–414 (139): 193 lines.

## Context

- Draft stage, child 3 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *3 · Two documents, neither one assembled*.
- `Templates::load` as the crate's one entry point for a configuration, and its
  three promises: a key resolves only if the **primary** declares it, both
  documents are validated whole against the vocabulary, and the load is
  all-or-nothing in both halves. Why an unreadable, unparseable or invalid overlay
  fails the load rather than falling back to the very policy its owner was moving
  away from.
- `compile_vocabulary`'s duplicate-name refusal and its reason — a duplicated slot
  is counted twice against its own cardinality rule and takes whichever value
  arrived first, *a consumer bug that looks like a template bug for as long as it
  goes unnamed*; the `read_primary`/`read_overlay` asymmetry as the whole of what
  *the overlay is optional* means at the filesystem; and `validate_document`'s
  aggregation — every diagnostic in both documents, with locations, in one refusal.
- **This chapter carries the second ending**: a `.grove.kdl` declaring a key the
  personal file does not, refused by name, the refusal naming the file that must
  declare it. `a_key_only_the_overlay_declares_does_not_resolve` pins it.
- Prose obligation 3 is **supply the argument**. Chapter 5 owns the 146–275 that
  sits *between* this chapter's two blocks; say so where a reader would otherwise
  expect the file's own order.
- **This chapter is the first use of `validate_node` and `validate_template`,
  which chapter 4 owns.** The early-use ledger already carries the row; state the
  minimum locally at the anchor and leave the explanation to chapter 4.
- Required example anchor: `both-documents` — `load` with a primary and an
  overlay, ending with `impl` resolving from the overlay and `review-impl` from
  the primary, each naming its own file.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  never-assembled --check all` is valid: 524 resolved lines, 1,549 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

Two blocks of one root that are not adjacent: `templates-load` (92–145) and
`reading-and-whole-document-validation` (276–414). The ownership ledger is what
makes that legible; the page says why the concept order disagrees with the file's.

## Outcome

Chapter 3 landed. `03-two-documents.md` owns `src/templates.rs` 92–145 and
276–414 — 193 lines — in eleven literal fragments under two composites, and the
scoped validation is green at the planned figures:

    book-check --repo . --book docs/walkthroughs/keyed-launch \
      --through never-assembled --check all
    valid: 9 files, 524 resolved lines, 1549 deferred lines, final=false

`scripts/check.sh` is **red on `book-check` alone**, and that is this slice's
shape rather than a lapse: the script runs `--final` over every book root by
discovery, and a prefix deliberately leaves the remaining 1,549 lines deferred.
The other seven checks pass, and the four completed books stay valid.

## Decisions

**1 · Eleven fragments, cut at what `load` does and at function boundaries.**
`templates-load` (92–145) splits four ways — the doc comment and signature, the
primary, the overlay branch, the returned value — because the three promises the
comment states fail separately and each is pinned by a different test.
`reading-and-whole-document-validation` (276–414) splits seven ways: one per free
function, and `validate_document` further into its three passes, because the
nodes pass, the duplicate pass and the report pass are three separate arguments.
Rejected: one literal per block, which would have put 139 unbroken lines on the
page with no place to answer the five introduction questions.

**2 · A third early-use row, beyond the structure brief's two.** The brief's
*Early uses the order forces* names two rows. This chapter's reproduced bytes
also *call* three functions chapter 4 owns — `source_location` at line 330,
`format_location` at 382, `render_diagnostics` at 406 — and
`docs/specs/walkthrough-books.md` triggers a row when a page "reproduces source
bytes whose referent belongs to a later slice". The manifest's `[[early-use]]`
entries are the rows a book may not omit, not a ceiling: every precedent book's
ledger is longer than its manifest (`jj-workspace` 8 → 18, `grove-llm` 14 → 21).
One grouped row was added at `#parsed-then-validated`, matching the brief's own
grouping of the three as chapter 4's diagnostic machinery. No manifest edit.

**3 · The second ending is shown as output, not as a fragment.** The refusal's
wording lives in `unresolved`, inside chapter 5's block, so this chapter shows
the message in a `console` figure and names `overlay_only` — built on line 131,
which *is* in this chapter's block — as what selects that branch. Reproducing the
function here would have taken bytes chapter 5 owns.

**4 · The non-adjacency is stated up front, with a figure.** The task file asks
this chapter to say where chapter 5's 146–275 sits. It is a three-row table in
the opening section plus one paragraph noting that `impl Templates {` opens at
line 92 and the brace closing it is the last line of chapter 5's block at 275 —
a line this page never prints.
