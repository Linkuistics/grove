# stale-book-counts-k196

## Goal

Correct two counts the `grove-loop` book states that measurement contradicts —
the early-use ledger's row count in chapter 21, and chapter 20's tally of
`loop_driver.rs`'s intra-doc links — and check the ledger row count against the
condition that let it go stale.

## Context

- **Chapter 21 says fifty early-use rows; there are fifty-two.**
  `docs/walkthroughs/grove-loop/21-what-could-not-move.md`, *The closed ledgers*:
  *Fifty rows, every one `explained`. Thirteen of them have their first use at*
  The cast *… the other thirty-seven*. Counted by enumerating the table between
  `## Early uses` and `<a id="owned-source-totals">` in `source-index.md`: **52**
  rows, 13 at *The cast* and **39** elsewhere, every one `explained`. The
  thirteen and the `explained` status both still hold; only the totals are wrong.
- **When it went stale, and why that matters.** Walking the history and counting
  the table at each commit: it was 50 from `what-could-not-move-k130`, where
  chapter 21 was written, all the way to `grove-root-join-clauses-k148`, and it
  became 52 at `canonicalisation-sites-k149` — which added the `target` and
  `existing_path` rows and did not update the chapter that counts them. So this is
  not drafting drift: it is the assembly chapter's standing exposure, that a
  later leaf can add a ledger row and nothing reports the roll-up it falsifies.
  Consider whether that condition is worth a check rather than a correction, since
  the next defect leaf to add a row will do the same thing again.
- **Chapter 20 says `loop_driver.rs` has eight intra-doc links that resolve; it
  has seven.** `20-the-loop.md`: *names this file in **none** of them: all eight
  of its intra-doc links resolve.* The file carries eight `` [`…`] `` tokens, at
  lines 24, 118, 122, 130, 163, 349, 527 and 537 — but **line 24 is inside the
  54-line plain `//` header**, and the same paragraph, four sentences earlier,
  establishes that `cargo doc` never reads it. So seven are intra-doc links whose
  resolution the clean run is evidence about, and the eighth is a bracketed token
  in a comment rustdoc does not parse. The sentence uses the chapter's own blind
  spot as though it were not there.
- **Both were found by `unresolved-doc-links-k151`**, which moved the warning
  counts on the same two pages and re-derived every number it touched. Neither is
  caused by that leaf and neither is repaired by it: the ledger count is the same
  before and after its commit, and the eight-links clause sits beside a number it
  corrected rather than inside one.

## Done when

- Chapter 21 states the ledger's row count, its *at The cast* split and its
  remainder as the table actually reads, each counted by enumerating the table
  rather than by reading a previous statement of it.
- Chapter 20 distinguishes the seven intra-doc links the clean run is evidence
  about from the eighth token, which sits where the chapter has already said the
  instrument cannot look — and says so rather than dropping the count.
- Whether the ledger roll-up should be checked mechanically has been decided
  either way, with the reason recorded; a `leaf-add` if the answer is yes.
- `book-check --final --check all` is green over `docs/walkthroughs/grove-loop`,
  and `bash scripts/check.sh` is no worse than before.

## Notes

**No source changes, so the freeze is not in play**, and the leaf is deferred
behind nothing.

## Decisions (running log)

**The ledger's totals, re-derived by enumeration.** Counting the rows of the
table between `## Early uses` and `<a id="owned-source-totals">` in
`source-index.md` field by field rather than by eye: **52** rows, **13** with
their first use at `01-orientation.md#the-cast`, **39** elsewhere, and the
status column is `explained` on all 52 with no other value present. Chapter 21's
*Fifty rows* and *the other thirty-seven* are corrected to *Fifty-two* and *the
other thirty-nine*; the thirteen, the `explained` status and the *twelve later
chapters* clause beside them are untouched, because the two rows added at
`canonicalisation-sites-k149` (`target` at `05-opening.md` and `existing_path` at
`06-paths.md`) are both outside *The cast*.

**The correction is anchored on the paragraph, not on the phrase.** *the other
thirty-seven* occurs **twice** in `21-what-could-not-move.md` — once in the
*Early use* paragraph, wrong, and once in the *Ownership* paragraph directly
above it, where 39 top-level blocks less chapter 1's own 2 makes it right. A
substitution over the file would have broken the correct one. Both edits on this
leaf were made by replacing a whole anchored passage with an asserted
single-occurrence match instead.

**Chapter 20's eight is seven, and the eighth is named rather than dropped.**
Enumerating every comment line in `crates/grove-loop/src/loop_driver.rs` that
contains a `[` — not only the `` [`…`] `` form — gives nine: the eight bracketed
tokens the task file lists (lines 24, 118, 122, 130, 163, 349, 527 and 537) plus
line 53, a `[ -n "$disposition" ]` shell test inside the header's sketch, which
is not a token at all. The header runs lines 1–54 and `#[cfg(test)]` opens at
547, so line 24's token is inside the plain `//` region and all seven `///`
tokens are inside the 492 lines the instrument reads. The sentence now says
eight comment lines carry the bracketed form, that line 24's is not an intra-doc
link because rustdoc never parses the header, and that the other seven resolve.
That agrees with Control C in the table three lines below it, which plants a
broken link in the plain `//` header and still reports 26 warnings — the old
sentence contradicted its own control.

**The ledger roll-up should be checked mechanically, and the check is its own
leaf.** Deciding *yes* on the evidence: the number went stale at
`canonicalisation-sites-k149`, `bash scripts/check.sh` was green across all six
books the whole time, and this node still holds nine other live defect leaves as
direct children, any of which may add a row and repeat it. Chapter 21 is an
assembly chapter — it owns no source roots, so `book-check` expands no fragment on the page and every sentence
on it is unchecked — and this is the *second* stale roll-up found on that one
page, the other being `landed-leaf-tally-chapter-21-k200`'s. Two independent
instances of one condition is what makes it a condition rather than a slip. Not
done here: the roll-up is spelled in English number words inside running prose,
so a checker needs a locator, and every book has an assembly chapter, so the
mechanism is cross-book. That is a validator or test-suite change with a
manifest-facing design question in it, not a paragraph edit. Cut as
`ledger-rollup-check-k207`, last in this node.
