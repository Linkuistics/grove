# chapter-twenty-drift-clause-k201

## Goal

Bring `docs/walkthroughs/grove-loop/20-the-loop.md`'s paragraph about decision 9
back into agreement with the record, which `decision-nine-loop-signature-k177`
has just repaired. The page asserts, in the present tense, a drift that no longer
exists.

## Context

- **The false sentences**, at `docs/walkthroughs/grove-loop/20-the-loop.md` lines
  718–724 (the paragraph opening *The header's citation of
  `docs/specs/module-decomposition.md`, decision 9, holds…*): *"The record's own
  code block **has since drifted** from the signature above it — it names the
  third parameter `&Templates` where the shipped one is `&TemplateSource`, and
  gives `LoopOutcome` two variants where the type above has three. The record is
  not corpus and the book does not correct it; `decision-nine-loop-signature-k177`
  holds it."* Every clause after the first sentence is now false: k177 repaired
  both, and it is retired.
- **The first sentence stays.** *The header's citation … holds for what the
  sentence leans on it for: the loop's shape, `exists? → create or find next →
  determine the command → run → finalise`, is that decision's* was true before
  k177 and is true after it. k177 changed no clause the loop's shape rests on.
  Re-read it against the page rather than trusting this quotation: a surviving
  clause inherits the scope of the false one beside it, and it is the clause this
  task file vouches for rather than one a check proved.
- **What k177 actually changed**, so the replacement paragraph is written against
  the record as it now stands rather than against this summary: `run`'s third
  parameter is now `&TemplateSource`; `LoopOutcome` now carries `Interrupted(i32)`
  beside `Finished` and `Stopped`; `Renumber` now carries `from_position` and
  `to_position`; and two new prose paragraphs follow the block — one recording
  that `Located`'s fourth field `outcome` is deliberately *not* listed (because
  `crates/grove-loop/src/task_tree.rs` justifies the field by reference to its
  absence from that listing), one stating that the block is a selective statement
  of the surface rather than an inventory.
- **This is prose only.** The paragraph is not inside a fragment, so no ledger row
  and no fragment range moves; `book-check docs/walkthroughs/grove-loop` must stay
  green, and it is the check that proves that.

## Done when

- The paragraph states the record as it is, not as it was. Whether the page
  mentions the former drift at all is the session's call — a walkthrough of the
  source owes no changelog of a specification — but it must not assert one that
  is gone, and it must not cite `decision-nine-loop-signature-k177` as a live
  holder.
- No `crates/` file is modified: the corpus freeze is not in play.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Check the rest of the page against the repaired record before editing, not just
this paragraph.** Chapter 20 was drafted while decision 9 was wrong, and one
paragraph is only what one chapter noticed; grep the whole book for `Templates`,
for `LoopOutcome` variant counts, and for `decision 9` before concluding this is
the only site.

**`.grove/11-crate-books-k14/14-grove-loop-book-k37/01-grove-loop-k123/BRIEF.md`
also states the drift in the present tense, and is deliberately left alone.** A
brief records what was true when it was written and directs the sessions under it;
grove does not rewrite task-tree files for staleness (constraint 1). Do not
"fix" it.

## Decisions (running log)

- **The former drift is not mentioned, and the paragraph does not become a
  changelog.** The task file left this to the session. A walkthrough of the
  source owes no history of a specification's repairs, and a sentence saying
  "the record used to disagree" would be the one clause a future reader has no
  way to check against anything in front of them.
- **The replacement says what the agreement is worth, not merely that it
  exists.** Recording only "the record's block matches" would waste the
  paragraph's slot; k177 also added two paragraphs bounding how far the block's
  authority runs — surface, not inventory, and one declaration deliberately
  short of the shipped type. That bound is the load-bearing fact for this book,
  because chapter 9 reads `Located`'s fourth field against exactly that
  omission. The paragraph now points at it.
- **The first sentence was re-read against the page rather than trusted from the
  task file's quotation, as the task file required.** `run`'s doc comment (the
  fragment immediately above, `crates/grove-loop/src/loop_driver.rs` 167–201)
  opens on the shape *exists? → create or find next → determine the command →
  run → finalise* and cites decision 9 for it; decision 9's prose carries the
  same shape. k177 changed `run`'s third parameter, `LoopOutcome`, `Renumber`
  and two paragraphs — no clause the loop's shape rests on. The sentence stands
  unedited.
- **The sweep the notes demanded found no second site.** `decision 9` occurs
  six times in the book: once here and five times in `19-the-core.md` (lines
  269, 453, 494, 680, 704), all about `Mandate`'s four fields and `compose` —
  untouched by k177. `k177` / `decision-nine-loop-signature` occurred only in
  the removed clause. Every `Templates` hit is in `18-which-files.md` and is
  `keyed_launch::Templates`, a different type from `grove-loop`'s
  `TemplateSource` — not the record's parameter. `LoopOutcome` is three variants
  everywhere it appears (`01-orientation.md` 574/590/592/628,
  `20-the-loop.md` 602/786/1082/1098, `source-index.md` 684); no page counts two.
  `Renumber`'s four fields in `10-growing.md` 602–620 already include
  `from_position` and `to_position`. `09-resolve.md` 195–202 says `outcome` is
  "the one field the design record did not have" and is "added back against the
  listing" — still true after k177, which recorded the omission rather than
  closing it.
- **Prose only, and the validator says so.**
  `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  → `valid: 13 files, 10557 resolved lines, 0 deferred lines, final=true`. No
  `crates/` file touched; `jj status` shows the one page modified.
