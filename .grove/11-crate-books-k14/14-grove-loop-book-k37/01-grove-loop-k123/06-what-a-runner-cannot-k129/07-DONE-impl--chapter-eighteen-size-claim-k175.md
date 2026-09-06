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

- **The defect reproduces, measured rather than inherited.** `wc -l` over the
  four Part V roots gives `driver_lease.rs` 1,383, `loop_driver.rs` 615,
  `session_config.rs` 358, `prompt.rs` 245 — `prompt.rs` is the smallest and
  chapter 18's root is second. The same `wc` over the whole corpus gives 12,213
  lines across 14 files, and 12,213 − 1,680 (`src/task_grow/tests.rs`, the one
  declared `[[corpus.exclude]]`) is exactly the 10,533 the briefs state: a free
  positive control that the instrument is reading the book's own corpus.

- **Neither site takes a ranking; both take a characterisation.** The `## What
  could not move` opener is an idiom across this part — chapter 16 asks the
  question *of the one chapter whose answer is not about meaning*, chapter 17
  *of a block that is entirely evidence* — and chapter 18 alone reached for a
  size where its siblings reached for a subject. So line 1039 became *asked of a
  root whose whole subject is a choice rather than state*, reusing the framing
  the chapter's own opening paragraph already established, and line 9 became *a
  much smaller root*, a comparison against the file chapters 16 and 17 read and
  not a rank. **`second-smallest` was rejected**: it would leave a second Part V
  ranking standing on a page in the part that has now got that class wrong
  twice, and chapter 19's *What could not move* already states the enumeration
  and owns it. Line 22's *more than its 358 lines suggest* is untouched and now
  carries the page's only size statement in that opening.

- **The enumeration is bounded and the sweep is wider than the word.**
  `grep -rn 'smallest'` over the book returned thirteen hits before and eleven
  after, the two lost being exactly these; chapter 19's own enumeration and the
  ten unrelated uses in chapters 1, 4 (×3), 7, 10, 11 (×2), 13 and 15 are
  untouched. Swept chapter 18 for the whole class rather than the one word —
  `largest`, `biggest`, `shortest`, `longest`, `second-…`, `…er than`, `fewest`,
  `least` — and it holds no other size or rank claim. `Part V` now appears once
  on the page, at line 1070, as a subject claim rather than a size one, which is
  a live control that the grep can still see this file.

- **No structure-brief edit was owed, confirmed rather than assumed.**
  `docs/specs/grove-loop-book-structure.md` contains `smallest` nowhere; its four
  ranking hits are `largest`, all chapter 16's and all already corrected by
  `lease-size-ranking-k171`. This is the difference from k171, where the brief
  carried two of three sites.

- **Verified.** `book-check --repo . --book docs/walkthroughs/grove-loop
  --through too-late-to-say-later --check all` → `valid: 13 files, 9918 resolved
  lines, 615 deferred lines, final=false`, matching k167's landing figures
  exactly; no fragment, ledger row or line count moved, as a prose-only change
  should not.

- **Left alone deliberately.** `18-which-files.md:1070` — *Every other chapter in
  Part V is about coordination* — is an impressionistic subject claim, not a size
  or count one, and its three illustrations are illustrative rather than an
  enumeration. It is outside this leaf's goal and was not grown into a leaf.
