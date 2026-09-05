# structure-brief-chapter-attributions-k163

## Goal

Correct two chapter attributions in `docs/specs/grove-loop-book-structure.md`
that name a chapter which does not and will not carry the thing attributed to it.
Found by `outcomes-k157` while drafting chapter 13; **placed before
`finishing-k158` because the first of the two is chapter 14's coverage
obligation** and that session reads the brief's map as its checklist.

## Context

Both are corrections made **in the brief**, not a leaf preferring its own
wording — the precedent is `structure-brief-dependency-count-k132`,
`pick-test-count-k147` and `structure-brief-lexical-pair-k150`. Neither touches
`crates/`, so neither is near the frozen-corpus rule.

### 1 · The residue map sends the transition table to chapter 13

*What this book makes redundant* has the row

    | 1162 | the transition table | 13 |

The marker is `docs/ARCHITECTURE.md`'s `<!-- residue(grove-loop): the transition
table -->`, and the table under it has four rows: no `.grove/` creates the root
brief and `01-requirements--plan-k1.md`; a root short of a whole grove is
refused; live leaves means no transition; no live leaf appends or reuses the
driver-owned finish leaf. **Every one of those is `transition_to_current`'s or
`materialize_finish`'s**, which are the `finish-transition` block (`1-331`) —
chapter 14's — with the scaffold half chapter 11 owns. Chapter 13 owns
`leaf_retire`, `leaf_prune` and their seven helpers, and nothing it owns
describes a lifecycle transition. The row should read `11, 14`, matching the
three rows beside it (`1079, 1175, 1216`), or `14` alone if the scaffold half is
taken as already covered there.

This matters rather than being cosmetic: the map **is**
`architecture-residue-k75`'s coverage obligation, so a marker attributed to a
chapter that does not cover it is a passage k75 would delete on the strength of a
book page that never made it redundant.

### 2 · The 13/16 contrast is not in chapter 1

*The stated outcome* says of the rejected re-derivation test: *It survives as the
thesis of chapters 13 and 16, and the contrast between those two is stated once
in chapter 1.*

**Chapter 1 does not state it.** `01-orientation.md` carries the spine, the
`jj-workspace` and `keyed-launch` spine difference, the Part IV/V boundary
(*chapters 16 to 20 are this paragraph*), and one forward pointer to chapter 13
alone (*chapter 13 is where a subtree prune spends* N *guards for* N *marks*).
There is no sentence contrasting the two chapters' thesis. The brief's own
chapter-1 section is consistent with the page rather than with this clause: its
*It carries three things no later chapter returns to* list does not include the
contrast, so chapter 1 discharged what it was actually asked for.

`outcomes-k157` wrote the contrast into chapter 13 — the earlier of the two
pages, so the book states it before either chapter needs it — under
`#marked-in-place`, citing `driver_lease.rs` line 25's own *untracked
coordination directory*. So the book is coherent and only the brief is wrong
about where.

**Also fix the offset while here.** Every line number in *What this book makes
redundant*'s table is one less than the marker it names — 310/311, 332/333,
968/969, 1162/1163, 1342/1343, and so on for all thirty-one. It is a systematic
off-by-one rather than thirty-one separate errors, and it is worth correcting
once rather than having each remaining chapter rediscover it.

## Done when

- The residue row for the transition-table marker names the chapter or chapters
  that actually cover it, and the reasoning above is reflected rather than
  restated.
- *The stated outcome*'s clause names chapter 13 rather than chapter 1, or is
  reworded so it does not assert a sentence chapter 1 has not written.
- The table's marker line numbers agree with `docs/ARCHITECTURE.md`.
- `bash scripts/check.sh` is no more red than it was — `book-check` alone, on the
  incomplete `grove-loop` book.

## Notes

**The corpus is frozen**, and nothing here needs to touch it: all three items are
in one specification file.

## Decisions (running log)
