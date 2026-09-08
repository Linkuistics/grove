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
