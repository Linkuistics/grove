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
