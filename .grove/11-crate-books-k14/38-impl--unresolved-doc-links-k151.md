# unresolved-doc-links-k151

## Goal

Repair the five unresolved rustdoc intra-doc links in `crates/grove-loop/src/`,
reconcile every book page that reproduces the changed bytes, and add the
early-use rows those links owe.

## Context

- **The defect, enumerated.** `cargo doc --no-deps --document-private-items -p
  grove-loop` reports five `unresolved link` warnings. Three are in
  `src/task_tree.rs`: line 580 `[`pick`]`, line 586 `[`select`]`, line 638
  `[`kind`]`. The other two are `src/prompt.rs` line 28 `[`crate::methodology`]`
  and `src/lib.rs` line 283 `[`Resolution::Ambiguous`]`. Found by
  `kind-and-briefs-k144`.
- **The cause for the `task_tree.rs` three.** `pick`, `select` and `kind` are
  public functions of `crate::verbs`, and `task_tree` does not import `verbs` —
  its only `crate::` imports are `task_name::{…}` and `Reference`. So rustdoc
  resolves none of the three, renders the text as literal `[kind]`, and warns.
  **The prose around each link is true**; only the link is broken, which is why
  chapters 7 and 8 adjudicated rather than corrected.
- **Nothing fails today.** A broken intra-doc link is a rustdoc warning, not a
  build or test failure, and `scripts/check.sh` does not run `cargo doc`. The
  instrument is the `cargo doc` invocation above and the rendered docblock —
  never reading the source, because nothing warns.
- **The likely fix is a qualified path** — `[`verbs::kind`]` or
  `[`crate::verbs::kind`]` — which changes bytes on one line and no line counts.
  Check the other two against their own modules before assuming the same shape;
  `crate::methodology` may name something that no longer exists, in which case
  the repair is to the sentence rather than to the path.
- **Two book pages reproduce the changed bytes** and must change in the same
  commit: `docs/walkthroughs/grove-loop/07-the-walk.md` (fragments
  `«walk-pick-in»` 580-585 and `«walk-select-in»` 586-595) and
  `08-kind-and-briefs.md` (fragment `«kind-in»` 638-656). `01-orientation.md`
  reproduces `lib.rs` 283. `prompt.rs` line 28 is chapter 19's, `«the-prompt-core»`,
  which is **not yet written** — see *Notes*.
- **Two early-use rows are owed and are not in the ledger.** Chapter 7's block
  reproduces `[`select`]`, whose referent `verbs::select` has no row anywhere;
  `pick` has one (owner `first-live-leaf`) but it is about the walk rather than
  about the public verb. `kind-and-briefs-k144` added the two rows its own block
  owed — `verbs::kind` and `verbs::brief_chain` — and chapter 7's equivalent
  sweep did not run. Adding a row requires the earlier page to state the minimum
  locally, so this leaf edits chapter 7's prose as well as the ledger.

## Done when

- `cargo doc --no-deps --document-private-items -p grove-loop` reports no
  `unresolved link` warning.
- Every book page reproducing a changed line reproduces the new bytes, and
  `book-check` is green over every book the commit touched, at whatever slice
  each is proved at when this runs.
- The `verbs::select` row exists in the early-use ledger with chapter 7 stating
  its minimum locally, and chapters 7 and 8's adjudicating paragraphs are
  rewritten to describe the repaired links rather than the broken ones.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the `grove-loop` book and says so.** `prompt.rs`
line 28 belongs to chapter 19, which `what-a-runner-cannot-k129` has not written;
fixing it now would put the corrected bytes in the source before the page that
must reproduce them exists. Run this after `grove-loop-k123`'s last child has
taken the book to green `--final` validation. Until then the three adjudications
on the pages stand as the record.

**The freeze holds.** One commit carries the source change, every affected ledger
and page, and a green validator run over every book it touched — or the leaf is
deferred behind the books it would invalidate, which is what the paragraph above
does. None of the fixes changes a line count, so no ownership range moves.

## Decisions (running log)
