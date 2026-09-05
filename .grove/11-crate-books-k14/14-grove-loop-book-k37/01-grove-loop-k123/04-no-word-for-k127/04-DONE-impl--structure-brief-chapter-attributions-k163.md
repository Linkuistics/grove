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

1. **The transition-table row reads `11, 14`, not `14` alone.** The marker's
   subject is the four-row table, and the rows split across two chapters'
   blocks: the dispatch is `transition_to_current` (`tree_lifecycle.rs` 75) and
   `materialize_finish` (113), both inside `finish-transition` (`1-331`, owner
   `the-tree-deletes-itself`), while row 1's *create the root brief and the
   first leaf* is `initialize_grove` (381) and row 2's refusal text is
   `root_shape`'s classification (455), both inside `grove-beginning`
   (`332-489`, owner `never-mistaken-for-finished`). Block owners read from
   `docs/walkthroughs/grove-loop/walkthrough.toml`, not from the verb names.
   Taking the scaffold half as covered at chapter 14 would have made this row
   the only one of the four `tree_lifecycle` rows not to name both, against
   `1080, 1176, 1217` beside it.

2. **All thirty-one marker lines corrected by enumerating both documents, not
   by sampling.** `grep -n 'residue(' docs/ARCHITECTURE.md` yields exactly
   thirty-one `grove-loop` markers — 311, 323, 333, 340, 350, 363, 371, 379,
   433, 451, 461, 472, 706, 739, 779, 857, 873, 884, 905, 969, 1007, 1053,
   1080, 1163, 1176, 1190, 1217, 1242, 1265, 1325, 1343 — and each is exactly
   one more than the number the table carried, checked pairwise across all
   thirty-one rather than on the three the finding named. The prose below the
   table (`1189`, `1324`, `471`) carried the same offset and was corrected with
   it.

3. **Added a standing instruction rather than only the numbers.** *Cite the
   marker's subject, not its line* now sits under the table, because the
   numbers move whenever anything above them in `docs/ARCHITECTURE.md` does —
   `architecture-residue-k75` will move all of them — and the subject column is
   the stable identifier. This is the brief's copy of what
   `no-word-for-k127`'s brief already tells the remaining chapters.

4. **The 13/16 contrast correction is three places, not one.** *The stated
   outcome*'s clause was the reported one, but the same false claim about
   chapter 1 stood in the chapter 16 section (*chapter 1 has already said so in
   a sentence*) — the summary-layer leak `references/execute.md` warns about,
   found by sweeping the whole brief for the claim rather than fixing the
   sentence the finding quoted. Both now name chapter 13, and chapter 13's own
   section states positively that it carries the sentence for both, so a later
   stage reconciling chapter 16 to the brief is told to inherit rather than
   re-argue. Verified against the pages: `01-orientation.md`'s only occurrences
   of `16` are the Part IV/V boundary (286), a fragment directive (310), a
   table cell (385) and a cast row (569) — none a thesis contrast — while
   `13-outcomes.md` 30–36 states it under `#marked-in-place`.

5. **`bash scripts/check.sh` is red on `book-check` alone**, as the part's shape
   requires: seven of eight green, and `book-check` failing `M101` on the eight
   pages 14–21 that `finishing-k158` and the later children still owe. Nothing
   in the workspace reads this specification file — `grep` over `crates/`,
   `scripts/` and `plugins/` finds it only in two prose sentences of
   `docs/walkthroughs/grove-loop/10-growing.md` — so `cargo test`, including
   `every_repository_markdown_reference_resolves`, is green and no page needed
   reconciling to these corrections.
