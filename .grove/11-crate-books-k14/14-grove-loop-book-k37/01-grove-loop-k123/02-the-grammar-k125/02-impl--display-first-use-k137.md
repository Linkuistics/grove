# display-first-use-k137

## Goal

Adjudicate the disagreement between the early-use ledger's `Display` row and the
page that actually uses `Display` first, and make the manifest, the structure
brief, the ledger and the affected pages agree — whichever way it is settled.

## Context

- **The disagreement.** `docs/walkthroughs/grove-loop/walkthrough.toml`'s
  mandatory `[[early-use]]` row for `` `TaskName::compose`, `impl Display for
  TaskName` `` declares its first use at
  `03-kind-slug-handle.md#the-handle-is-the-identity`, and
  `docs/specs/grove-loop-book-structure.md`'s *Early uses the order forces* gives
  the reasoning: `every_positioned_name_ends_in_its_own_handle` is the handle's
  structural claim and can only be asserted over a rendered whole name.
- **What contradicts it.** Chapter 2's block `shape-refusal-tests` reproduces
  `crates/grove-loop/src/task_name.rs` lines 1,418 and 1,455, where
  `a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading` calls
  `to_string()` on a `TaskName` twice and asserts each name renders back to its
  own bytes. In page order the first use is chapter 2.
- **Nothing is red.** `check_early_uses` in
  `crates/book-validation/src/ledger.rs` matches a mandatory row byte-for-byte
  and checks only that the first-use chapter is strictly before the owner's; it
  never checks that a named first use is the earliest one. Chapter 2 is green
  with the row untouched.
- **`the-tokens-k134` left the row alone** and stated the rendering behaviour in
  prose at `02-the-tokens.md#refusals-inside-the-shape`, so the page is
  self-contained either way and this leaf is free to settle it in either
  direction.
- Precedent for the shape of the fix, if the brief is what moves:
  `structure-brief-dependency-count-k132`, which corrected the same brief and
  said what the gap was.

## Done when

- The question is answered in one of two directions, with the reason recorded:
  either the row's first use moves to chapter 2 — in which case the manifest row,
  the structure brief's *Early uses the order forces* table, the ledger row and
  chapter 2's anchor set all change together — or the row stands as a statement
  about the *load-bearing* first use, in which case the structure brief says so
  explicitly, so a later reviewer does not re-open it.
- If `TaskName::compose` and `Display` are split into two rows, both survive the
  byte-for-byte mandatory-row check.
- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-verdicts --check all` is still valid, and the resolved and deferred line
  counts are unchanged at 887 and 9,646.
- `bash scripts/check.sh` is red on `book-check` alone, as it is for every child
  of `the-grammar-k125` but the last.

## Notes

**This runs before chapter 3 deliberately.** `03-kind-slug-handle.md` is the page
that would otherwise create the anchor the disputed row promises, and settling
the question after that page exists means editing it rather than writing it.

**The corpus is frozen.** Nothing here touches `crates/grove-loop/`.
