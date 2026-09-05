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
- **`display-first-use-k137` has settled the disputed row: it is now two.**
  `` `TaskName::compose` `` keeps
  `03-kind-slug-handle.md#the-handle-is-the-identity` as its first use, so that
  anchor must still exist on this page in explicit form; `` `impl Display for
  TaskName` `` moved to `02-the-tokens.md#refusals-inside-the-shape`, which is
  chapter 2's and is already there. The row this page owes an anchor for is
  therefore `compose` alone, and its statement is about composition — the
  handle's structural claim asserted over names the test *builds* rather than
  parses — with the rendering half no longer part of it.
- **The early-use ledger is a floor, and the criterion is now written down.**
  `docs/specs/grove-loop-book-structure.md`, *Early uses the order forces*, says
  a reproduced block owes a row for a later-owned symbol it **names or
  exercises**, unless an existing row already states the minimum a reader needs
  there. Enumerate every candidate token in this chapter's blocks and classify
  each; do not sweep a list. `floor-rows-chapter-two-k138` runs immediately
  before this leaf and works the same sweep over chapter 2, so read its outcome
  for the worked example and the row wording.

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

1. **Spent the leaf's one in-session reviewer on three enumeration claims inside
   the corpus, not on the page.** `references/execute.md` allows one reviewer per
   picked leaf and one reviewer asked to inspect several named axes spends one.
   The claims a fresh context could settle and the compiler could not were
   `Kind::is_finish`'s *three places that ask*, `Handle::parse`'s *only peel …
   outside `split_shape`*, and `Kind`'s *no third kind literal exists in the
   machinery* — all three in bytes this chapter reproduces, all three the class
   the memory of this workstream flags as highest-yield. The conclusion was
   stripped from the prompt, as step 2 requires.

2. **Three of the reviewer's findings are adjudicated on the page; three are
   not.** Valid and actionable: `is_finish`'s *three places* (seven calls in six
   functions, three of them outside all three named categories);
   `Handle::parse`'s *only peel* (`terminal_key` is a third, and the consequence
   clause survives because all three route through `peel_key`); and line 1,691's
   *three references* against a loop of four. Rejected: the test comment at
   1,618–1,619 (*`split_shape` and `Handle::parse` now share a single peel* is
   true, merely not exclusive — not a defect); the reviewer's own verification of
   *nineteen variants*, which counted shipped `grove-<kind>` skills and got the
   count wrong — there are twenty-three, and the comment is a historical claim
   about a deleted enum that this corpus cannot bear on either way; and
   `grove-llm`'s second spelling of the default root slug, which is real but
   belongs to another crate's frozen corpus and another book.

3. **Line 221's *the skill a session is told to load* is stated precisely rather
   than adjudicated.** `prompt.rs` line 63 renders `format!("{PLUGIN}-{}",
   kind.label())`, so the skill is `grove-<kind>` and the kind is its stem. That
   is a compression rather than a claim the code refutes, and the page discharges
   it by naming where the skill name is actually composed. `label`'s own doc at
   293–294 already words it correctly.

4. **Four floor rows, from an enumeration of every symbol token in the three
   owned blocks rather than from a list.** `TaskName::Brief` / `TaskName::
   Positioned`, `TaskName::parse`, `terminal_key` and `parse_ref` are each named
   or exercised by bytes this chapter reproduces, owned by a later slice, and
   covered by no existing row. Not rowed: `verbs::resolve` and `TaskNameError`
   (chapter 1's and chapter 2's rows state the minimum needed here), `peel_key`
   and `split_shape` (chapter 2's rows), `a_kind` / `slug` / `entry` and `impl
   Display for TaskName` (chapter 2's rows), and `finish-commit`, which is named
   as a command a human types rather than as `verbs::finish_commit` and is stated
   locally instead — the same treatment chapter 2 gave `leaf-add`.

5. **This leaf cuts no `copy-edit` leaf.** `grove-loop-k123`'s *Done when* assigns
   that act to the last child, `what-could-not-move-k130`, and the book is two
   chapters into twenty-one. The editorial family's last-act rule is discharged
   there, not here.

6. **`scripts/check.sh` is red on `book-check` alone, as this node's brief says
   every child but the last will be.** Measured on the committed bytes: `check:
   FAILED — 1 of 8`, the one failure being `book-check`, and every diagnostic
   under it a consequence of chapters 4 to 21 not existing yet — eighteen `M101`
   *required book page … is missing*, plus the two `M103`s that follow from them,
   the README's contents list and this chapter's navigation both lacking a link
   to `04-the-name.md`. The other five books are `final=true` and valid. The
   prefix this leaf owes is proved separately and is green: `book-check --repo .
   --book docs/walkthroughs/grove-loop --through the-handle-not-the-position
   --check all` reports 13 files, 1,450 resolved lines, 9,083 deferred,
   `final=false`.

7. **The green scoped run was checked with two deliberate failures before it was
   credited.** A validator that reads clean everywhere is not evidence, so one
   byte inside a reproduced fragment was changed and one mandatory early-use
   anchor renamed: the first produced `F008` naming the source byte and the
   fragment path, the second two `F009`s against the ledger. Both were reverted
   and the run is green again. The same control was run over the string-literal
   extractor behind decision 1's enumeration — a scratch file with two production
   literals and one inside a `#[cfg(test)]` module, where it reported the two and
   skipped the third.
