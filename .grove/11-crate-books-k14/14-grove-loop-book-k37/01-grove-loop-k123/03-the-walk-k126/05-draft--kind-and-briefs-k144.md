# kind-and-briefs-k144

## Goal

Draft chapter 8 of the `grove-loop` book — *Kind, and the brief chain*,
`docs/walkthroughs/grove-loop/08-kind-and-briefs.md` — and prove the prefix
through slice `root-to-leaf`.

## Context

- The fourth of `the-walk-k126`'s six chapter children. Its **three** ownership blocks
  are `kind-and-brief-chain` (`task_tree.rs` 638–746),
  `brief-chain-and-kind-tests` (1,361–1,652) and `pick-with-brief-chain-tests`
  (1,997–2,023) — 428 lines. The third is the file's closing block and it is this
  chapter's rather than chapter 7's **because it exercises `brief_chain`**; the
  structure brief records that placement as one of two made while the brief was
  written.
- The structure brief's section is *8 · Kind, and the brief chain*. The rule is
  **ancestor briefs, root to leaf, and a brief is not a leaf**: `kind_in`,
  `brief_chain` and `leaf_entry`.
- **`leaf_entry` is where canonicalisation appears — once, and only to compare.**
  Chapter 5 reproduced the module header's statement of that and pointed here.
  The chapter says what is compared against what, and why comparing is not the
  same as canonicalising for output.
- **Three test blocks, all taking *supply the claim*.** The source labels two of
  them `brief-chain` and `kind` and the third `pick + brief-chain together`; per
  reproduced test, the property it establishes **and** what would have to be true
  for it to pass while the property was broken. The production half is 42% prose
  and takes *do not restate*.
- `a_session_kind_that_is_not_a_token_is_malformed` is one of the three forms of
  the book's second ending — a task-shaped name grove refuses that the store
  would have accepted.
- **The early-use ledger is a floor**: enumerate this chapter's reproduced bytes
  for later-owned symbols named or exercised, and add the rows they owe.
- The chapter's carried-example row is the brief's row 8: from a leaf inside a
  node, to its ancestors' briefs, root to leaf.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through root-to-leaf
  --check all` is valid: 13 files, 3,560 resolved lines, 6,973 deferred,
  `final=false`.
- Chapter 8 exists, `README.md`'s contents entry and chapter 7's two navigation
  lines are updated, and all three `root-to-leaf` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)
