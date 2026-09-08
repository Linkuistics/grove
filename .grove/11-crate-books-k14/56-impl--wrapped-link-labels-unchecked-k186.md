# wrapped-link-labels-unchecked-k186

## Goal

Make `book-check` see the Markdown links it currently skips: a link whose **label
is hard-wrapped across a line** is dropped by the scanner without a diagnostic, so
it is never link-checked in any book.

## Context

- `crates/book-validation/src/markdown.rs:686-689`: after finding `](`,
  `scan_links` does `if markdown[index..separator].contains('\n') { index =
  label_start; continue; }`. The link is neither scanned nor reported — it simply
  leaves the corpus the checker sees.
- The guard is defensible in intent (it stops a `[` in one paragraph pairing with
  a `](` in the next), but its effect is silent under-coverage, and hard-wrapped
  prose is this repository's house style — so the excluded case is the *common*
  case, not an edge one.
- Two live examples, both in a book that passes `--final --check all`:
  `docs/walkthroughs/jj-workspace/06-refusal.md:230` and `:463` — cited at `:216`
  and `:445` when this leaf was cut, and moved down by
  `jj-workspace-method-counts-k184`, which rewrote prose above both. Both are
  same-page `#anchor` links with wrapped labels. They are unchecked, and — see
  below — the resolver would reject them if it saw them.
- **A second, separable defect the first one hides.** `resolve_local`
  (`markdown.rs:835-860`) pops the filename off the source path and, for an empty
  path component, returns the *directory*. So a bare `#anchor` link resolves to
  `docs/walkthroughs/<book>`, which is in none of the three file maps, and is
  reported as `M201 … resolves to missing repository file`. Measured directly:
  the same link written on one line fails `book-check`; wrapped across two, it
  passes. Whether bare `#anchor` should be legal at all is the decision this leaf
  owes — the two examples above assume it is.
- Found while correcting book prose in `env-selector-coverage-k68`, whose
  one-line same-page link went red where the book's existing wrapped ones do not.

## Done when

- A wrapped-label link is scanned like any other, or is reported — silence is the
  one outcome ruled out. If the newline guard is kept in some form, it is narrowed
  to the pairing hazard it was written for (a blank line between `[` and `](`)
  rather than to any newline.
- Bare `#anchor` links have a stated answer: either `resolve_local` returns the
  source file for an empty path and they are legal, or they are rejected with a
  message that names the remedy. Whichever is chosen, the two occurrences in
  `06-refusal.md` end up conforming rather than exempt.
- The gap has a test that has been **seen to fail**: a fixture whose only link is
  a wrapped one pointing at a target that does not exist must go red. A test
  asserting the current behaviour proves nothing about the change.
- `bash scripts/check.sh` passes over all six books.

## Notes

**This is a validator change, so expect it to find things.** Every book's prose
becomes visible to the link checker for the first time, and a wrapped link with a
stale destination is exactly what has been able to hide. Re-running `book-check`
across all six books before writing any prose fix is the cheapest way to size it —
and if the fan-out is larger than one session, that sizing is the decomposition,
not a reason to narrow the fix.

**`book-validation` is deliberately outside the documented corpus** (root brief,
*P1*): it is the authoring tool, not the system being documented, so no book root,
fragment or ledger is at stake here and the freeze rule does not bind. Only the
prose fixes the change forces would touch a book.
