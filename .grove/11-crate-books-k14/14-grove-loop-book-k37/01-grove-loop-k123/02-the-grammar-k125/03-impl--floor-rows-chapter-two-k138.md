# floor-rows-chapter-two-k138

## Goal

Apply the early-use criterion `display-first-use-k137` wrote into
`docs/specs/grove-loop-book-structure.md` across the whole of chapter 2's
reproduced blocks, and add every floor row chapter 2 owes but does not carry.

## Context

- **The criterion, now stated.** `docs/specs/grove-loop-book-structure.md`,
  *Early uses the order forces*, says a reproduced block owes a row for a
  later-owned symbol it **names or exercises**, unless an existing row already
  states the minimum a reader needs there. `entry_path`'s mandatory row is the
  precedent for *names* — it is anchored on the module header that names it, not
  on the chapter that reads the function.
- **This leaf exists because k137 settled the criterion on one symbol and did not
  re-run it over the block.** That leaf's goal was the disputed `Display` /
  `compose` row; a partial enumeration is the exact failure the *enumerate, then
  classify* rule warns about, so the whole sweep is here rather than split across
  two sessions.
- **Four candidates, found by an adversarial read of k137 and not yet
  verified by enumeration.** Each is used by bytes chapter 2 reproduces and
  defined by a block a later chapter owns:
  - `impl Display for TaskNameError` — defined at `task_name.rs` 727, block
    `591-1020` (chapter 4). Chapter 2 calls it at source 1,344, 1,386 and 1,472,
    all in block `1313-1521`, and its prose analyses the rendered advice text.
    This is the **sharpest** one: it is the same shape as the row k137 added for
    `TaskName`'s `Display`, in the same block, and k137 added one and not the
    other. Note the bytewise sort — `` `impl Display for TaskNameError` ``
    sorts **before** `` `impl Display for TaskName` `` (`E` is 0x45, `` ` `` is
    0x60), so the row goes above it.
  - `peel_key` — `task_name.rs` 1,012, block `591-1020` (chapter 4). Named in
    chapter 2's reproduced header at source 68 and load-bearing in the page's
    prose. A private free function, so no cast row can cover it.
  - `split_shape` — `task_name.rs` 967, block `591-1020` (chapter 4). Named in
    the same reproduced header and used in the `#classified` analysis. Also
    private.
  - `Handle::render` — `task_name.rs` 517, block `221-590` (chapter 3). Named in
    the reproduced header and analysed in the prose beside it. The chapter-1 cast
    row covers `Handle`, not this associated function — and the book already
    decided that question in the affirmative for `Parts::leaf`.
  Lower confidence, same shape and to be classified rather than assumed:
  `Kind::new` and `Slug::new`, named in the reproduced header.
- **Do not trust that list.** It is a starting set from one reviewer, not an
  enumeration. Extract every candidate token from chapter 2's three blocks —
  `1-220`, `1178-1199`, `1313-1521` — and classify each one, which is complete by
  construction; the list above is what to expect to find, not what to look for.
- The row k137 added is `` `impl Display for TaskName` `` at
  `02-the-tokens.md#refusals-inside-the-shape`; its statement and placement are
  the model for these.

## Done when

- Every later-owned symbol chapter 2's blocks name or exercise either carries a
  row in `docs/walkthroughs/grove-loop/source-index.md`'s *Early uses* table or
  is recorded here as classified out, with the reason.
- Each added row has its minimum local statement on the page, at the anchor the
  row names — the *Refusals inside the shape* paragraph that currently opens
  *Four names this section uses before the chapter that explains them* is where
  most of them belong, and its count moves with them.
- No manifest change: these are floor rows, and `[[early-use]]` carries only the
  rows a book may not omit.
- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-verdicts --check all` is still valid, with resolved and deferred line
  counts unchanged at 887 and 9,646.
- `bash scripts/check.sh` is red on `book-check` alone.

## Notes

**This runs before chapter 3 deliberately**, for the same reason k137 did:
`kind-slug-handle-k135` and `the-name-k136` owe the same enumeration over their
own blocks, and they should inherit a settled criterion and a worked example
rather than discovering both.

**The corpus is frozen.** Nothing here touches `crates/grove-loop/`.

**The criterion may belong one level up, and that is a finding to record rather
than a change to make.** `docs/specs/walkthrough-books.md`'s *Early-use ledger*
states the two-clause trigger — *first uses* or *reproduces source bytes whose
referent belongs to a later slice* — but carries no *unless an existing row
already covers it* exception, and `display-first-use-k137` wrote that exception
into the `grove-loop` structure brief because that is where it was needed. It is
book-general, so it arguably belongs in the specification; but that specification
governs five books, four of them already written and proved `final=true`, so
changing it is a cross-book claim that has to be checked against those four
ledgers. Note it here, do not act on it.

**Whether `entry_path`'s anchor is right is not this leaf's.** The criterion
above was written to explain the row as it stands; if the sweep turns up evidence
that it cannot, that is a finding to record, not to act on — chapter 5 does not
exist yet.
