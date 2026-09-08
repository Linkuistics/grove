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

1. **The enumeration found eight surfaces, not six.** Classifying `verbs.rs` by
   first-argument type and then reading every sentence that assigns a verb an
   opening role — rather than grepping the phrase — turned up two the task file
   had not listed, both saying the same false thing in different words:
   `10-growing.md:338` (*`verbs::leaf_add` is the tree-opening **wrapper** a
   caller reaches*) and `08-kind-and-briefs.md:236` (*`verbs::brief_chain`
   **opens the tree**, calls this, and hands back the paths*). A line grep for
   *tree-opening* finds the first and never the second. The task file's *enumerate
   rather than sweep* was aimed at hard wrapping; the wider yield was the
   paraphrase.

2. **The nearby true claims were left alone, and checking them was the point.**
   `10-growing.md:872` says the verb wrapper beside `leaf_insert` *performs that
   second opening, and it gives up the write guard first* — that is
   `verbs::stale_cross_refs`, and it is true: the body calls `tree.relinquish()`
   and then `task_tree::read(tree.root())` (`verbs.rs:196–210`). So does
   `15-the-verbs.md:531`'s *`finish_commit` also opens the tree*, and
   `12-leaf-to-node.md:273`'s claim about `grove-llm`'s own CLI ordering.
   Correcting by pattern rather than by evidence would have falsified a true
   sentence about the one verb that really is an exception besides
   `finish_commit`.

3. **What a `TreeWrite` opening entitles a page to say: nothing about the verb
   opening anything.** `TreeWrite::guard` hands over the guard `write` opened
   with, or reopens through `task_tree::write` when that one is spent
   (`lib.rs:218–232`) — so a *reopen* is real, but it belongs to the `TreeWrite`
   the caller already opened, not to the verb, and the verb's signature is
   `&TreeWrite` either way. The wording adopted across all five write-verb rows
   is therefore the one two of them already used: **the public half — it takes a
   `TreeWrite` and hands this function the guard**. Uniform across five, and it
   does not smuggle the reopen in as something the verb does.

4. **The read pair got the wording `unresolved-doc-links-k151` had already
   settled.** `15-the-verbs.md:325` now reads *the public half of a `task_tree`
   function — taking the already-open `&Tree` a caller hands it, since no verb
   here but `finish_commit` opens a tree*, which agrees with the chapter's own
   line 90 (*unlike every other verb here*) and with the `verbs::kind` early-use
   row k151 rewrote (*it takes an already-open `&Tree`*). `08-kind-and-briefs.md`
   now says the same of `brief_chain` as line 140 already said of `kind`.

5. **Verified green.** `book-check --repo . --book docs/walkthroughs/grove-loop
   --final --check all` → `valid: 13 files, 10557 resolved lines, 0 deferred
   lines, final=true`; `bash scripts/check.sh` → `check: all 8 principal checks
   pass`, with all six books green. No source byte changed, so no ledger,
   ownership range or line count moved.
