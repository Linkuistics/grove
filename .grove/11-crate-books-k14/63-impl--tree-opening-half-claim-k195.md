# tree-opening-half-claim-k195

## Goal

Correct the `grove-loop` book's claim that a public verb is the *tree-opening
half* of the `task_tree` function it calls, in the six places that survive
`unresolved-doc-links-k151`, and settle it against the one verb that really does
open the tree.

## Context

- **Why it is false.** No verb in `crates/grove-loop/src/verbs.rs` opens a tree
  except `finish_commit`, and chapter 15 says so twice — its own reproduced doc
  comment at line 289 of `verbs.rs` reads *It opens the tree itself, unlike every
  other verb here*, and the page restates it. The read verbs take a `&Tree`
  (`pick` line 70, `kind` line 83, `brief_chain` line 95, `resolve` line 108); the
  write verbs take a `&TreeWrite`. `crates/grove-loop/src/task_tree.rs`'s own
  comment, which chapter 6 reproduces, dates the rule: *every verb takes an
  already-open tree since* `loop-crate-verbs-k21`.
- **The surviving surfaces, enumerated.**
  - `docs/walkthroughs/grove-loop/15-the-verbs.md:325` — *Two verbs chapter 8 read
    from the other side. Each is the tree-opening half of a `task_tree` function.*
    Those two are `verbs::kind` and `verbs::brief_chain`, and it is false of both.
    This one also contradicts the same chapter's own line 90.
  - Five early-use rows in `docs/walkthroughs/grove-loop/source-index.md` —
    `verbs::leaf_add`, `verbs::leaf_insert`, `verbs::leaf_decompose`,
    `verbs::leaf_retire`, `verbs::leaf_prune`. Two of the five soften it in the
    same breath (*it takes a `TreeWrite` and hands this function the guard*),
    which is what a corrected wording probably looks like, but *tree-opening
    half* is still the phrase. Decide what a `TreeWrite` opening — `lib.rs` calls
    it *the reopening a `TreeWrite` performs when it no longer holds one* —
    entitles the page to say, and say that instead.
- **Two surfaces are already gone.** `unresolved-doc-links-k151` rewrote
  `08-kind-and-briefs.md:138` and the `verbs::kind` early-use row while correcting
  the doc link they were built around, because that leaf's own change falsified
  the *links to* clause in the same sentence. Both now say the verb takes an
  already-open `&Tree`. That leaf found the class and left the rest here.
- **Enumerate rather than sweep.** The phrase is hard-wrapped, so a line grep
  misses instances split across two lines. Enumerate the functions in `verbs.rs`
  by first-argument type — `&Tree`, `&TreeWrite`, `&Workspace`, none — and
  classify every sentence in the book that assigns one of them a half, rather
  than searching for one phrase.

## Done when

- No page of `docs/walkthroughs/grove-loop/` says or implies that a verb other
  than `finish_commit` opens a tree, and chapter 15's sentence about the pair
  chapter 8 reads is true of both verbs it covers.
- Each of the five write-verb early-use rows either states what a `TreeWrite`
  actually does or drops the claim, consistently across the five.
- The result does not contradict `15-the-verbs.md`'s *unlike every other verb
  here*, which is the sentence the corrected wording has to agree with.
- `book-check --final --check all` is green over `docs/walkthroughs/grove-loop`,
  and `bash scripts/check.sh` is no worse than before.

## Notes

**No source changes, so the freeze is not in play.** Every byte of this fix is
book prose; no fragment, ownership range or line count moves, and the leaf is
deferred behind nothing.

## Decisions (running log)
