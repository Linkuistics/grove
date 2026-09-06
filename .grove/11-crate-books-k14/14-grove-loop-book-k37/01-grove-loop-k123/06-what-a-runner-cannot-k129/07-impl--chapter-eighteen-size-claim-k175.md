# chapter-eighteen-size-claim-k175

## Goal

Correct the two sites in `docs/walkthroughs/grove-loop/18-which-files.md` that
call `session_config.rs` *the smallest root in Part V*, which `prompt.rs` is, and
leave chapter 19 as the one place a later page takes a Part V size from.

## Context

- **The defect.** `18-which-files.md` line 9 opens *This one reads the smallest
  root in Part V*, and line 1039 repeats *The book's question, asked of the
  smallest root in Part V*. Part V's roots are `driver_lease.rs` 1,383 (chapters
  16 and 17), `loop_driver.rs` 615 (chapter 20), `session_config.rs` 358 (chapter
  18) and `prompt.rs` **245** (chapter 19). `session_config.rs` is the
  second-smallest of the four, not the smallest.
- **Nothing turns on the claim except the claim.** Neither sentence's argument
  depends on the ranking — line 9 goes on to contrast the chapter's *subject*
  with chapters 16 and 17, and line 1039 introduces the three questions. The
  minimal repair is *second-smallest*, or dropping the superlative; the sizes
  themselves are correct everywhere else on the page, which states 358 lines
  repeatedly.
- **This is `lease-size-ranking-k171`'s class, met a second time in the same
  part.** That leaf corrected *the third largest* and *the second-largest owned
  block* after chapter 16 enumerated the roots and the owned blocks, and its
  finding — **the size is settled and the unit is the trap** — was promoted with
  the instruction that *chapters 17 to 21 still owe the enumeration for their own
  size and count claims*. Chapter 18's two sites predate that enumeration
  reaching this part's page-level prose.
- **Enumerated, and the enumeration is bounded.** `grep -rn 'smallest'` over
  `docs/walkthroughs/grove-loop/` returns thirteen hits; ten are unrelated uses in
  chapters 1 (×1), 4 (×3), 7 (×1), 10 (×1), 11 (×2), 13 (×1) and 15 (×1), one is
  chapter 19's own enumeration, and the remaining two are these. `docs/specs/grove-loop-book-structure.md` contains
  the word nowhere, so **no structure-brief edit is owed** — unlike k171, where
  the brief carried two of the three sites.
- **Chapter 19 states the enumeration and is the site to defer to**, in its *What
  could not move* section: Part V's roots run 1,383 / 615 / 358 / 245 and its
  owned blocks 819 / 615 / 564 / 358 / 245. Point the corrected sentences at
  nothing — the book cannot link chapter to chapter for a fact like this without
  clutter — but do not restate the list on chapter 18.

## Done when

- Neither sentence in `18-which-files.md` calls `session_config.rs` the smallest
  root in Part V, and no other page acquires a Part V size ranking.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs — this touches prose
  only, so no fragment, ledger row or line count moves.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This runs before `the-loop-k168` deliberately.** Chapter 20 is the last
source-owning chapter in the part and carries size claims of its own; it should
read a corrected chapter 18, for the same reason `lease-size-ranking-k171` was
placed ahead of the rest of Part V rather than after it.

**Not deferred behind the book.** It changes no source byte, so the freeze is not
in play and there is nothing to reconcile in a later commit.

## Decisions (running log)
