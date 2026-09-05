# opening-k141

## Goal

Draft chapter 5 of the `grove-loop` book — *Opening, contention and refusal*,
`docs/walkthroughs/grove-loop/05-opening.md` — and prove the prefix through slice
`one-spelling-of-grove`.

## Context

- The first of `the-walk-k126`'s six chapter children, one per chapter. Its **one**
  ownership block is `tree-opening` (`task_tree.rs` 1–290), 290 lines, and it is
  the smallest of the six.
- The structure brief's section is *5 · Opening, contention and refusal*. The
  rule is that **a guard is proof the tree was there when it was opened, and no
  more than that**. The chapter covers `Tree`, `Guard`, `Opening`, `TreeVacancy`,
  the four openings, the three error paths — `absent_tree`, `raised`, `restate` —
  and `announce_contention`, which is the whole reason `libc` is a dependency.
- **The block is wholly production and takes *do not restate*.** There is no
  inline test in 1–290; the `#[cfg(test)]` items inside it are the `READ_COUNT`
  thread-local and its accessors, not tests. Prose connects arguments across
  items, names the test that holds each claim from `crates/grove-loop/tests/`
  or from a later chapter's block, and stops.
- **The `entry_path` early-use row is this page's obligation.** The manifest
  anchors it on `05-opening.md#one-spelling-of-the-root`, so that
  `<a id="one-spelling-of-the-root"></a>` must exist explicitly and the section
  must state the minimum locally: the one place an entry's absolute path is
  built, because the store returns no paths. The row's owner is
  `paths-are-built-here`, so it stays `pending` after this slice.
- The chapter's carried-example row is the brief's row 5: from `<worktree>/.grove`
  and a caller with a worktree, to a `Tree` — or a refusal naming what is absent.
- `CONTEXT.md#tree-access-lock` is the one glossary anchor the brief assigns to
  this chapter.
- The module header states two things this chapter reproduces and later chapters
  read: that path construction lives in `entry_path` alone (chapter 6), and that
  canonicalisation appears once, in `leaf_entry`, and only to compare (chapter 8).
  Both are named forward, not explained here.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  one-spelling-of-grove --check all` is valid: 13 files, 2,440 resolved lines,
  8,093 deferred, `final=false`.
- Chapter 5 exists, `README.md`'s contents entry and chapter 4's two navigation
  lines are updated, and the `tree-opening` ownership row reads `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

1. **Decomposed `the-walk-k126` one child per chapter and landed chapter 5.**
   2,541 source lines over six chapters, against the 700 that became chapter 4's
   1,727 markdown lines in one session. The manifest's block owners already
   partition `task_tree.rs` and `task_grow.rs` along exactly the chapter
   boundary, and `--through` proves a canonical prefix, so any other cut would
   leave a child unable to prove itself. Five sibling leaves and the node brief
   were written here; the node brief's decision 2 records the `the-walk` slug
   collision the page-id convention forces.
2. **Seventeen fragments over the 290-line production block.** The module header
   makes four separable claims and each is proved in a different later chapter,
   so a fragment per claim is what lets the prose say which chapter proves each
   without restating any of them. The remaining thirteen are one per item.
3. **Chapter 1's cast row `Tree`, `Vacancy`, `task_tree::Guard`,
   `task_tree::write` moved from `pending` to `explained`.** Its owner is
   `one-spelling-of-grove`, which this slice completes; `owner_is_complete` in
   `crates/book-validation/src/ledger.rs` computes the expected status from the
   scope, so leaving it `pending` is `F009`.
4. **Three early-use rows added beyond the manifest's.** `leaf_entry` (owner
   `root-to-leaf`) and `tree_lifecycle::leaf_prune` (owner `marked-in-place`)
   are both named in reproduced doc comments; `brief_chain`, `kind_in` (owner
   `root-to-leaf`) are named in the module header's first sentence, which lists
   the five reading verbs. The third was found by this leaf's reviewer, not by
   the author's own sweep — the ledger is a floor and an under-enumerated floor
   is **silent**, since `check_early_uses` verifies the manifest's mandatory rows
   are present and never that no row is missing.
5. **Two structure-brief defects externalised to `pick-test-count-k147`**, cut
   ahead of `the-walk-k143`. The brief says chapter 7 has fifteen tests and
   `pick-tests` has nineteen; and it calls `announce_contention` *the whole
   reason `libc` is a dependency*, which chapter 1 already adjudicated against
   the identical `Cargo.toml` clause. `task_tree.rs` is unchanged since
   `grove-loop-structure-k36`, so the count was wrong when written rather than
   outrun.
6. **One source defect externalised to `grove-root-join-clauses-k148`**, placed
   ahead of `architecture-residue-k75` with this node's other source fixes.
   `lib.rs` claims twice that `<worktree>/.grove` is spelled in exactly one
   place; three production callers spell it a second way. Chapter 5 owns none of
   those bytes, so it stopped repeating the claim and left the adjudication to
   the chapter that reproduces it.
7. **`scripts/check.sh` is red on `book-check --final` over this book**, which is
   the shape every child but the last leaves: the script proves each book root
   `--final` by discovery, and a prefix deliberately defers later blocks. The
   failure is `M101` for chapters 6 to 21; the other seven checks are green and
   the other five books are green.
8. **The leaf's one in-session reviewer was spent on the two things nothing
   mechanical checks**: the early-use floor's completeness, and the page's own
   factual claims. `book-check` proves reconstruction and ledger *structure*, and
   never whether a sentence about the source is true. Ten findings came back and
   **all ten were valid** — none was noise, an unclear contract, or a visible
   trade-off. Every one was re-verified against the source here before it was
   acted on. No second reviewer was materialised and no `review-*` leaf was cut:
   each fix was a local prose correction against source re-read in this session,
   not a redesign, and this family takes no `review-*` leaf.

   Five had already been found by the author's own pass and were staged before
   the report arrived — the `write_or_vacancy` uniqueness overclaim, *twenty are
   argument*, the `TreeVacancy` uniqueness claim, `write` described as an
   acquisition, and six restated doc comments. The five that were not are the
   ones worth recording, because each read as true:

   - **The cited test does not discriminate.**
     `mutator_waits_for_a_shared_worktree_reader_before_allocating_a_leaf` was
     cited as holding `reopen_write`'s *announce once per command, not once per
     guard* rule. The test exists and asserts exactly what the page said. But it
     runs `leaf-add`, which takes **one** guard, so its
     `assert_eq!(…count(), 1)` passes identically under a per-guard policy. Only
     `apply_prune`'s loop at `tree_lifecycle.rs` line 953 takes *N*, and no test
     watches stderr through a contended bulk prune — so the rule is argued in its
     comment and pinned nowhere. The page now says that. **A test whose name
     matches the prose is not evidence that it discriminates.**
   - **`READ_COUNT` reaches eight chapters, not three, through nine
     assertions** — one in chapter 7, six in `tree_lifecycle.rs` across the
     blocks chapters 11 to 14 own, and two in the file chapter 10 may only cite.
     The claim was offered as the reason the counter is worth following, so the
     error carried the paragraph's whole point.
   - **Six of `task_tree.rs`'s ten blocks carry no `#[test]`, not one.** The five
     production blocks, and `path-composition-tests` despite its name; the file's
     first `#[test]` is at line 1,108. The true claim is one level up — chapter 5
     is the only one of the file's five chapters owning no part of the inline test
     module. The author's own correction of this claim had itself said *four*.
   - **`lib.rs`'s one-spelling claim is false**, and the page had repeated it in
     the chapter's opening figure. `grove-root-join-clauses-k148` holds it.
   - **The "every test that pins it" enumeration was half the set** — eight
     assertions, not four, plus a third production spelling of the sentence in
     `grove-llm`. The conclusion survived; the enumeration licensing *every* did
     not.

   The pattern across four of those five is one class: **a count or uniqueness
   claim written from the shape of the argument rather than from an
   enumeration**, which is what the parent node's brief warns of and what
   `pick-test-count-k147` found in the structure brief on the same day.
