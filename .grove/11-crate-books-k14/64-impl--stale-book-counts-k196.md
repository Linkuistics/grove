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
