# residue-map-one-pick-k178

## Goal

Correct one residue-map attribution in `docs/specs/grove-loop-book-structure.md`
that names two chapters, neither of which covers the marker's subject, while a
third covers it in full. Found by `what-could-not-move-k130` while checking the
map against the book that was actually written.

## Context

A correction made **in the brief**, not a leaf preferring its own wording — the
precedent is `structure-brief-dependency-count-k132`, `pick-test-count-k147`,
`structure-brief-lexical-pair-k150`, `structure-brief-chapter-attributions-k163`
and `structure-brief-test-list-k172`. It touches no file under `crates/`, so it
is nowhere near the frozen-corpus rule, and it changes no page of the book.

### The row

*What this book makes redundant* carries

    | 433, 451 | the walk and the finish-reservation rule; the one pick and what it serves | 7, 14 |

It is a **joint row over two markers**, and the chapter pair is right for the
first and wrong for the second.

- **Marker 433** — `the walk and the finish-reservation rule` — is
  `docs/ARCHITECTURE.md`'s account of the stateless depth-first pre-order walk,
  the first live leaf, and finish being *reserved, not blocking*. Chapter 7 owns
  `selected` and chapter 14 owns the finish leaf. `7, 14` is correct.
- **Marker 451** — `the one pick and what it serves` — sits under the heading
  *Authoritative selection and mandate* and its paragraph is entirely the
  **driver's** discipline: one authoritative pick per iteration after any
  required lifecycle mutation, one guarded read copying the selected leaf's path,
  stable handle and filename kind, the read guard released before the second
  configuration load and the spawn, and that single value serving readiness, the
  launch diagnostic line, template selection and the mandate **with no second
  tree read**, not recomputed immediately before spawn.

**Chapters 7 and 14 contain none of it.** Neither page carries *authoritative*,
*one pick*, *per iteration* or *second tree read* in any spelling, and neither
owns a line of `loop_driver.rs`.

**Chapter 20 covers it in full.** `20-the-loop.md`'s `#the-selection` section
reproduces the block that performs it and argues it: *The kind is passed, never
re-read* — `selection.kind` indexes `config.source`, indexes `config.expand`, and
is handed to `session_prompt`, so *one guarded selection reaches four consumers*
and the prompt and the command a session receives cannot disagree about what kind
it is. Chapter 19's closer states the same fact from the other side:
`loop_driver.rs` reads the selection once and does not recompute it before the
spawn.

### Why it is not cosmetic

The map **is** `architecture-residue-k75`'s coverage obligation — it is the
checklist that session reads to decide which passages a written book has made
redundant. `structure-brief-chapter-attributions-k163` recorded the cost exactly:
a marker attributed to a chapter that does not cover it is a passage k75 would
delete on the strength of a book page that never made it redundant, or would
leave standing because the named chapters plainly do not carry it. Here the
coverage genuinely holds, so the deletion is safe either way; what is wrong is
the checklist, and k75 has no other one.

### What was checked, and what was not

All thirty-one markers naming `grove-loop` were enumerated from
`docs/ARCHITECTURE.md` and checked by **subject** against the chapters the map
names — the brief's own instruction, since the line numbers move. The map's
line-number set and the document's marker set agree exactly today, and thirty of
the thirty-one attributions hold. This is the only one that does not.

## Done when

- The row is split, in `docs/specs/grove-loop-book-structure.md`'s *What this
  book makes redundant*, into

      | 433 | the walk and the finish-reservation rule | 7, 14 |
      | 451 | the one pick and what it serves | 20 |

  keeping the table's line-ordered shape, and with the marker's line numbers
  re-derived against `docs/ARCHITECTURE.md` as it stands at that moment rather
  than copied from here.
- The paragraph under the table that counts the markers still totals
  thirty-one, and any prose naming the joint row is reconciled.
- Chapter 21's `#what-the-book-made-redundant` section already records the
  coverage as delivered and the attribution as wrong; **reconcile it to the
  corrected brief** so the page does not go on describing a defect that has been
  fixed. The book page and the brief must agree afterwards.
- `bash scripts/check.sh` passes, and `book-check --repo . --book
  docs/walkthroughs/grove-loop --final --check all` is still valid at 13 files,
  10,533 resolved lines, `final=true`.

## Notes

**The corpus is frozen.** This leaf edits a specification and, if it touches the
book at all, one page's prose. It changes nothing under `crates/`.

**Run before `architecture-residue-k75`.** k75 is the last child of
`crate-books-k14` and reads this map as its checklist; this leaf sits under
`grove-loop-book-k37` and therefore precedes it in the walk.
