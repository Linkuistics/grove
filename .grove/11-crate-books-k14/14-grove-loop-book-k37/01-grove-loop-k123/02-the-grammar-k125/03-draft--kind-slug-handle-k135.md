# kind-slug-handle-k135

## Goal

Draft chapter 3 of the `grove-loop` book — *Kind, slug, handle*,
`docs/walkthroughs/grove-loop/03-kind-slug-handle.md` — and prove the prefix
through slice `the-handle-not-the-position`.

## Context

- The second chapter of `the-grammar-k125`, and 563 lines in three blocks:
  `kind-slug-and-handle` (`task_name.rs` 221–590), `slug-rule-tests`
  (1,522–1,550) and `handle-grammar-tests` (1,551–1,714).
- The structure brief's section is *3 · Kind, slug, handle*. The rule is that
  **the handle is the identity and the position is not in it**. The types are
  `Kind` with its two reserved labels, `Slug`, `HandleError`, `Handle` and
  `Parts`.
- **The chapter's structural claim is the one
  `every_positioned_name_ends_in_its_own_handle` asserts**: every positioned
  name's rendering ends in its own handle's rendering — a node's exactly, a
  leaf's followed only by the `.md` suffix. That test's own comment says why it
  is asserted rather than reviewed for: *drift is not expressible* has to be held
  by something, or it is a promise. The other named test is
  `the_slug_rule_is_the_one_grove_already_had`.
- Chapter 2 has already reproduced the module header's two handle passages
  (`task_name.rs` 57–76) and said that this chapter proves them. Do not restate
  the header's argument; discharge it.
- **Prose obligation splits by block.** 370 production lines at 42% take *do not
  restate*; the 193 lines of inline tests take *supply the claim* — per
  reproduced test, the property and what it would still pass under.
- The carried example's step is `plan-k1`: the name peeled to a handle that
  carries the slug and the key and not the position.
- **Read `display-first-use-k137`'s outcome before writing the page.** That leaf
  runs immediately before this one and settles whether the early-use row for
  `` `TaskName::compose`, `impl Display for TaskName` `` still names
  `03-kind-slug-handle.md#the-handle-is-the-identity` as its first use. If it
  does, that anchor must exist on this page in explicit form.
- **The early-use ledger is a floor.** Enumerate what this chapter's reproduced
  bytes call that chapters 4 and later own, and add a row for each; chapter 2
  added three beyond the manifest's.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  the-handle-not-the-position --check all` is valid: 13 files, 1,450 resolved
  lines, 9,083 deferred, `final=false`.
- Chapter 3 exists, `README.md`'s contents entry and the navigation lines either
  side of it are updated, and the three `task_name.rs` ownership rows this
  chapter owns read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)
