# landed-leaf-tally-chapter-21-k200

## Goal

Re-derive chapter 21's *four of that second group have since landed* tally from
the tree as it now stands, and rewrite the sentence and its per-chapter
attribution to match — or replace the count with a structural claim that cannot
go stale the next time a deferred defect leaf lands.

## Context

- **The claim.** `docs/walkthroughs/grove-loop/21-what-could-not-move.md`, lines
  463 to 465:

      Four of that second group have since landed, two of them chapter 1's, one
      chapters 5, 6 and 8's, and one spread across chapters 1, 7, 8 and 19.

  *That second group* is the *found while drafting* group enumerated at 456 to
  459 — stale enumerations, the miscounted helper list, the four comments naming
  a module this workspace does not have, the five unresolved intra-doc links, the
  parenthesised citation naming no anchor. The four it then details are
  `unresolved-doc-links-k151`, `manifest-dependency-clauses-k133`, and the two
  chapter 1 leaves.
- **It is stale by at least five, and the staleness is not one leaf's doing.**
  Every deferred defect leaf that has landed since chapter 21 was written is
  absent from the count and from the attribution:
  `grow-header-stale-helper-k154` (recorded at `10-growing.md:233`),
  `default-root-slug-two-spellings-k159`, `welded-grove-name-summary-k160`,
  `refused-grove-test-overclaims-k161` (`11-a-grove-begins.md:563`) and
  `stale-slug-precondition-comment-k162` (chapter 12). Confirm the list against
  `.grove/11-crate-books-k14/` rather than reading it from here — more may have
  landed by the time this leaf is picked, which is the whole defect.
- **No instrument reports it.** Chapter 21 is an assembly chapter: it owns no
  source roots, so `book-check` expands no fragment on the page and every
  sentence is an unchecked claim about other chapters. `bash scripts/check.sh`
  was green across all six books while the count was wrong.
- **A count of landed leaves is the wrong shape for a frozen page.** The sentence
  will be falsified again by the next deferred leaf that lands, and each one is a
  separate session that has no reason to read chapter 21. Prefer a structural
  claim — that each landed repair is recorded beside the fragment that judged it,
  and that the pages say which — over any tally, unless the tally is genuinely
  load-bearing for the chapter's argument.
- **Found by `stale-slug-precondition-comment-k162`'s in-session reviewer**,
  which was pointed at k162's own changed set and surfaced this as a
  pre-existing defect the change did not touch.

## Done when

- The sentence at `21-what-could-not-move.md:463-465` is true of the tree at the
  time this leaf runs, or has been rewritten so that it states something no later
  landing can falsify — and the per-chapter attribution that follows it agrees
  with the pages it names.
- Every landed defect leaf the sentence's group covers is accounted for, by
  enumeration from `.grove/11-crate-books-k14/` rather than from the list above.
- Any `concept-index.md` entry pointing at the rewritten passage still describes
  what the passage says.
- `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  is green, and `bash scripts/check.sh` is no worse than before this leaf.

## Notes

**This is a prose-only leaf; no source byte moves.** The freeze rule does not
bite and no ledger, fragment range or manifest count is in scope.

**Check whether the sibling assembly chapters carry the same shape.** Chapter 21
is `grove-loop`'s, but every book has a closing chapter that argues from
cross-chapter claims no validator expands. If one of the others tallies landed
leaves the same way, say so here rather than widening this leaf — a second book's
page is a second leaf.

## Decisions (running log)
