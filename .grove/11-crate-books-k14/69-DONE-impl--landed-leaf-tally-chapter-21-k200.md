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

**The sibling assembly chapters were checked, and one carries the same shape —
but it is true today.** `docs/walkthroughs/grove-llm/07-what-order-holds.md:379`
reads *All three found while drafting have since landed*, with a *Five were
judged worth a source change, and four leaves carried them* tally above it. The
same VCS enumeration over `crates/grove-llm/src` and `crates/grove-llm/Cargo.toml`
shows no corpus commit after `grove-llm-dependency-comments-k102`, so all three
tallies hold. No leaf cut: the defect there is latent, not live, and the first
`grove-llm` corpus repair to land is what would make it real. The other three
closing chapters — `jj-workspace/07-what-jj-owns.md`,
`keyed-launch/10-what-passes-through.md`,
`ordinal-fs-tree/08-invariants-and-trade-offs.md` — carry no landed-leaf tally at
all.

## Decisions (running log)

**1 · The tally is replaced by a structural claim rather than re-derived.** The
task file offered both, and the enumeration settles it. The sentence entered the
page at `manifest-dependency-clauses-k133` — not at `what-could-not-move-k130`,
which wrote the chapter without it — and has carried **five** different numbers
since: *One* at k133, *Two* at `grove-root-join-clauses-k148`, *Three* at
`canonicalisation-sites-k149`, *Four* at `unresolved-doc-links-k151`, *Five* at
`requirement-six-citation-k189`. **None of the five was true on the day it was
written** (the true figures on those days were 3, 4, 5, 6 and 16). Two landings
in between — `unreachable-root-clause-k152` and
`default-root-slug-two-spellings-k159` — rewrote parts of this page and left the
number where they found it, which is the failure mode the shape invites. Enumerated from the VCS, not from the task file:

    jj log -r '::@ & files("crates/grove-loop/src" | "crates/grove-loop/Cargo.toml")' \
      --reversed --no-graph -T 'change_id.short() ++ "|" ++ description.first_line() ++ "\n"'

which gives every commit that has touched the frozen corpus, in ancestry order.
Sixteen members of the second group have landed — `stale-enumerations-k139`,
`kit-fixture-and-peel-doc-k140`, `manifest-dependency-clauses-k133`,
`grove-root-join-clauses-k148`, `canonicalisation-sites-k149`,
`unresolved-doc-links-k151`, `unreachable-root-clause-k152`,
`grow-header-stale-helper-k154`, `default-root-slug-two-spellings-k159`,
`welded-grove-name-summary-k160`, `refused-grove-test-overclaims-k161`,
`stale-slug-precondition-comment-k162`, `lease-stale-reader-name-k169`,
`prompt-rule-id-prefix-k174`, `loop-driver-kill-anchor-k176` and
`requirement-six-citation-k189` — against the five the page named. A count that
has been wrong at every value it has ever held is not a count worth re-deriving.

**2 · Two of the sixteen landed *before* chapter 21 was written**, which is why
the original *Four* was already false at `what-could-not-move-k130`.
`stale-enumerations-k139` and `kit-fixture-and-peel-doc-k140` (both
`task_name.rs`, both found while drafting chapters 3 and 4) sit at positions
1429 and 1430 of `::@` against the chapter's own 1454. So the defect is not that
later leaves outran a true sentence; the sentence was never true.

**3 · The group's boundary, measured.** Twenty commits have touched the corpus
since drafting began. Two are the *first* group (`every-member-version-comment-k84`,
`template-source-read-count-k86`, known false before drafting). Two more are
**outside both groups** and stay unmentioned: `library-root-function-count-k187`
(found at `manifest-function-count-k82`, and asserted in chapter 1's own voice
*unadjudicated*, so it fails the group's *each was adjudicated beside its
fragment*) and `release-cut-member-comments-k188` (found by the enumeration k84
owed). The remaining sixteen are the second group's.

**4 · The structural claim is the freeze rule, and it is the one thing on this
page an instrument does hold.** A repair travels in one commit with every page
that reproduces the changed bytes, so no page reproduces a byte a later commit
changed without changing beside it — and `book-check`'s reconstruction is what
refuses to let that be false. Deliberately *not* claimed: that every landed
repair *names its leaf on the page*. Measured false — `k139`, `k140` and `k161`
appear nowhere in the book by name, because a repair that lands while the
chapter is still being drafted leaves a page that simply explains the corrected
bytes. Nor is the claim about fragment `lines="A-B"` ranges, which
`fragment-range-unchecked-when-final-k206` is opening precisely because nothing
reads them when a book is final.

**5 · The structure brief's *Known in advance* is untouched, and correctly so.**
Chapter 21's sentence about it (*counts five claims in total … none of the five
now stands in the corpus*) is a claim about what the brief counts, and the brief
still counts five. That section is scoped to claims known **before** drafting and
takes false claims only, never broken addresses — the precedent is explicit at
`unresolved-doc-links-k151` and `loop-driver-kill-anchor-k176`, neither of which
was added. So the sixteen do not belong there and the brief is not stale.

**6 · Verification.** `book-check --repo . --book docs/walkthroughs/grove-loop
--final --check all` → *valid: 13 files, 10557 resolved lines, 0 deferred,
final=true*. `bash scripts/check.sh` → *all 8 principal checks pass*, 6 books
checked, 0 failing. No `concept-index.md` row describes the tally: the two rows
pointing at `#the-closed-ledgers` are the ownership ledger and the early-use
floor, both untouched.
