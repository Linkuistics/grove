# the-surface-k128

## Goal

Draft Part IV of the `grove-loop` book — chapter 15, owning
`crates/grove-loop/src/verbs.rs` (363), `src/driver.rs` (57) and
`src/complete.rs` (96), 516 lines in three blocks — and prove the prefix through
slice `twelve-not-fourteen`.

## Context

- Draft stage, child 5 of 7 of `grove-loop-k123`, and the only part that is one
  chapter. The structure brief is `docs/specs/grove-loop-book-structure.md`, and
  this chapter is its *15 · The twelve verbs, and the two that are not*.
- **One session, no decomposition expected.** 516 lines over three roots, each
  owned whole, and the three are 57%, 73% and 65% comment prose — the most
  argued-in-situ material in the corpus. The obligation here is *do not restate*:
  the comments make the argument and the fragment graph quotes them verbatim.
- **The rule: the surface is twelve verbs, and everything else that touches the
  tree has to say why it is not one.** `verbs.rs` declares fourteen `pub fn`;
  `stale_cross_refs` and `signal_channel` each say in their own doc comment why
  they are not verbs. `driver.rs` holds two more tree operations that are not
  verbs and states that putting them beside the twelve *would say the surface has
  fourteen verbs, which it does not*. `complete.rs` is the twelfth verb and the
  loop's child-side half.
- **Count before writing the count.** *Twelve*, *fourteen* and *two* are the
  chapter's whole argument and each is a claim about the file in front of you:
  enumerate the `pub fn` declarations in `verbs.rs` and the operations in
  `driver.rs` rather than repeating this paragraph.
- Chapter 1's cast rows owned by `twelve-not-fourteen` — `verbs::stale_cross_refs`
  and `verbs::finish_commit`, and `interpret` with `Disposition` — move to
  `explained` when this chapter lands.
- The chapter cites the guide anchor `usage-tree-verbs` and the glossary anchors
  `loop-control-channel` and `task-commit-boundary`; all three exist today.
- `report_insert` is named by `verbs.rs`'s own comment as the owner of a decision
  this crate declines to make. It belongs to the binaries above, and the brief
  puts it outside this book: name the boundary, do not cross it.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  twelve-not-fourteen --check all` is valid: 13 files, 7,932 resolved lines,
  2,601 deferred, `final=false`.
- Chapter 15 exists, contents and navigation are updated, and the three
  ownership rows for `verbs.rs`, `driver.rs` and `complete.rs` read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

**The last act is this part's only chain obligation and it is nothing**: the next
stage is cut by child 7, not here.

## Decisions (running log)

1. **The ledger was enumerated rather than read off this body, and this body was
   wrong.** The `Context` above names two chapter-1 cast rows — `stale_cross_refs`
   and `verbs::finish_commit` — plus `interpret`/`Disposition`. The ledger
   actually carries **nine** rows owned by `twelve-not-fourteen`, and the
   chapter-1 row is `` `verbs`, `verbs::stale_cross_refs`, `verbs::signal_channel` ``
   — not `finish_commit`. The other seven are first used in chapters 8, 10, 12
   and 13. All nine moved to `explained`. This is the node brief's *a count in a
   leaf body is not a count in the tree* landing exactly as predicted.
2. **The early-use floor sweep found no new rows owed.** The only later-owned
   symbols the three blocks name are `DriverLease`, `compose` and `run`, all at
   `driver.rs` lines 10–11, and chapter 1's cast already carries a row for each
   with an earlier first use. Swept mechanically over the later chapters' symbol
   sets, not by eye.
3. **Three adjudications on the page, no source leaf cut.** `driver.rs`'s
   public-module justification covers one of its two subjects; `complete.rs`'s
   *(after commit + retire)* reverses the order `task-commit-boundary` makes
   load-bearing; and `tests/verbs.rs:345` calls `stale_cross_refs` a verb eleven
   lines after its subject denies being one. In all three the behaviour and the
   decision are right and only the wording over-reaches — the `llm_cli`
   precedent — so the page settles them in front of the reader and the frozen
   bytes stand.
4. **Two coverage claims were measured, with a control.** Deleting
   `signal_channel`'s `GROVE_SIGNAL_FILE` fallback, and deleting `interpret`'s
   `.trim()`, each leave the suite identical to an unmutated control of the same
   workspace copy: 558 tests, 547 passed, 11 failed (all `prompt.rs`, the copy
   not being a jj repository). Deleting the emptiness filter instead turns
   exactly three tests red at the same total, which is what makes the two zeros
   readable. Promoted to the node brief for chapters 16, 17 and 20, which own the
   same variable.
5. **Nine of my own numeric claims were wrong before landing and were corrected
   against the source**, including *eleven names come in from the crate root*
   where the import list carries fifteen. Counts in this chapter are enumerated,
   and the comment-share figures are given as counts because my counting rule and
   the structure brief's disagree by one point on two of three roots.
6. **Last act: nothing.** Child 5 of 7; `copy-edit` is cut by child 7 when
   `grove-loop-k123` closes, per the node brief's `Done when`.

## Result

`book-check --repo . --book docs/walkthroughs/grove-loop --through
twelve-not-fourteen --check all` reports **valid: 13 files, 7932 resolved lines,
2601 deferred lines, final=false** — the three figures this leaf asked for.
Chapter 15 exists as `docs/walkthroughs/grove-loop/15-the-verbs.md`, contents and
both nav lines on chapters 14 and 15 are updated, the concept index carries
twenty-six entries for it, and the `verbs.rs`, `driver.rs` and `complete.rs`
ownership rows read `resolved`.

**`bash scripts/check.sh` is red on `book-check` alone** — `FAILED — 1 of 8`,
with the other seven green. Every one of its twenty-three `--final` complaints
names a page from chapter 16 on, a block or early-use row owned by a later slice,
or the missing `Next` link into the unwritten chapter 16. That is the prefix
shape the node brief predicts for every child but the last, not a lapse.
