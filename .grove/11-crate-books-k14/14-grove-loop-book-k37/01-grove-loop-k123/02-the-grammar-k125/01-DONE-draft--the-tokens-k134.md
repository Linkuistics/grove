# the-tokens-k134

## Goal

Draft chapter 2 of the `grove-loop` book — *The tokens, and the four verdicts*,
`docs/walkthroughs/grove-loop/02-the-tokens.md` — and prove the prefix through
slice `four-verdicts`.

## Context

- The first of three children of `the-grammar-k125`, one per chapter. Its three
  ownership blocks are `tokens-and-verdicts` (`task_name.rs` 1–220),
  `classification-verdict-tests` (1,178–1,199) and `shape-refusal-tests`
  (1,313–1,521) — 451 lines.
- The structure brief's section is *2 · The tokens, and the four verdicts*. The
  rule is that a task-shaped name that is wrong is Malformed, never Foreign,
  because skipping it is lost work — and skipping a task-shaped *directory* takes
  its whole subtree out of the walk.
- The production block is 42% comment prose and takes the *do not restate*
  instruction. The two test blocks take *supply the claim*: per reproduced test,
  the property it establishes **and** what would have to be true for it to pass
  while the property was broken.
- The manifest's early-use row `` `TaskName`, `TaskNameError`, `Verdict`,
  `verdict`, `entry`, `malformed` `` fixes the anchor
  `02-the-tokens.md#the-four-verdicts`, which must exist explicitly on the page.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-verdicts --check all` is valid: 13 files, 887 resolved lines, 9,646
  deferred, `final=false`.
- Chapter 2 exists, `README.md`'s contents entry and chapter 1's two navigation
  lines are updated, and the three `task_name.rs` ownership rows this chapter
  owns read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

1. **Sixteen fragments over the 220-line production block, eight of them the
   module header.** The header makes eight separable claims and three of them are
   proved in later chapters, so a fragment per claim is what lets the prose say
   which chapter proves each without restating any of them.
2. **The `Outcome`, `TokenError` early-use row moved from `pending` to
   `explained`.** Its owner is `four-verdicts`, which this slice completes;
   `owner_is_complete` in `crates/book-validation/src/ledger.rs` computes the
   expected status from the scope, so leaving it `pending` is `F009`.
3. **Three early-use rows added beyond the manifest's.**
   `TaskName::distinguished`, `Parts::leaf`, and the `a_kind` / `slug` helpers
   are all called by reproduced bytes and defined by later-owned blocks. The
   ledger is a floor, not the set.
4. **The `Display` first-use divergence was externalised, not fixed here.**
   `display-first-use-k137` owns it; see the node brief's *Found while drafting*.
5. **`scripts/check.sh` is red on `book-check --final` over this book**, which is
   the shape every child but the last leaves: the script proves each book root
   `--final` by discovery, and a prefix deliberately defers later blocks. Every
   other check in the script is green.
6. **The leaf's one in-session reviewer was spent on the *supply the claim*
   half**, which is the part of this chapter nothing mechanical checks: the
   validator proves reconstruction and never whether a "what it would still pass
   under" statement is true. Sixteen findings came back; fifteen were valid and
   are fixed, one was noise (it read only `walkthrough.toml` and so missed the
   three early-use rows added to the book's own ledger, which is where authors
   add them). No second reviewer was materialised and no `review-*` leaf was
   cut — every fix was a local prose correction against source already read, not
   a redesign, and this family takes no `review-*` leaf.

   The three worth recording, because each was a claim that read as true:

   - **The `moved` fixture does not discriminate first-`--` from last-`--`.**
     Both fixtures of `a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading`
     contain exactly one separator, so the two grammars agree on both. The only
     fixture on this page that separates them is `01-impl--a--b-k1.md`, two tests
     below, and that is now what the page says. Verified by enumerating every
     string literal in the chapter's three blocks: it is the only one with two.
   - **The round-trip test's negative assertion rules out `Verdict::Entry` only**,
     so it is satisfied by `Foreign` — the test would pass while a separator-less
     migration name was silently skipped, which is the exact failure this chapter
     is named for. That gap was missing from the page and is now stated.
   - **`peel_key` is chapter 4's, not chapter 3's.** It sits at lines 1,012–1,019,
     inside block `the-task-name` (591–1,020). Two other attributions were wrong
     in the same way and are corrected.

   Two false uniqueness or count claims were also caught — "every one of them is
   refused" over a block two of whose fixtures parse, and "the only test on this
   page whose fixtures parse" when the charter test parses too. Both are the
   class the node brief's predecessors keep hitting: count before writing one.

