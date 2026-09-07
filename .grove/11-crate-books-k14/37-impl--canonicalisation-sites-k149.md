# canonicalisation-sites-k149

## Goal

Correct `crates/grove-loop/src/task_tree.rs`'s module-header claim that
canonicalisation appears in exactly one place, and reconcile the two pages of the
`grove-loop` book that reproduce or restate it.

## Context

- **The defect, in the module header.** `crates/grove-loop/src/task_tree.rs`
  lines 27 to 29 read *Canonicalisation appears once, in [`leaf_entry`], and only
  to `compare` a caller's spelling of a leaf against the tree's*. It appears in
  two functions of the same file. `target` (lines 335 to 374) canonicalises the
  candidate path, the grove root, and each candidate entry's built path;
  `leaf_entry` (lines 665 onwards) does the same three things. Enumerated with
  `grep -rn 'canonicalize' crates/grove-loop/src/`: six production call sites,
  lines 346, 349 and 369 in `target` and 712, 715 and 734 in `leaf_entry`, plus
  two inside `driver_lease.rs`'s `#[cfg(test)]` module at 1237 and 1241.
- **It is refuted from inside the same file**, which is what makes it the same
  class as the two claims the structure brief already lists. `target`'s own doc
  comment at line 332 says *Canonicalised to **compare** and never to report,
  exactly as `leaf_entry` does* — an explicit statement that there are two.
- **The second half of the header's sentence is true and must survive.**
  Canonicalisation is only ever used to compare, never to produce a path grove
  hands back, in both functions. The false half is *once, in `leaf_entry`*. The
  true claim is narrower and still worth making: canonicalisation appears only in
  the two resolvers, and neither returns a canonicalised path.
- **Found by `paths-k142`**, chapter 6's draft, which owns `target` and therefore
  owns the counterexample. Chapter 6 adjudicates the claim on the page beside the
  fragment that reproduces `target`'s doc comment
  (`06-paths.md#canonicalise-to-compare`) and does not repeat it as true.
- **Two book pages restate the claim and one of them asserts it.**
  `docs/walkthroughs/grove-loop/05-opening.md`, at
  `#one-spelling-of-the-root`, reproduces the header fragment
  `«tree-header-no-canonicalising»` and then says *The minimum is the exception
  the passage states: it is the one place canonicalisation appears*. That
  sentence was corrected by `paths-k142` to point forward to chapter 6's
  adjudication rather than to assert the claim; check it reads correctly against
  the reworded source, and correct the fragment's bytes if the wording changes.
  The `leaf_entry` row of the early-use ledger in `source-index.md` carries the
  same statement and was corrected in the same pass.
- **Chapter 8 owns `leaf_entry` and has not been written yet.** If it lands
  before this leaf, check it does not reintroduce the claim.
- **The fix must stay inside the existing line counts.** `task_tree.rs` is
  declared 2,023 lines in `walkthrough.toml` and in the source index's root row,
  and its ten ownership blocks carry exact ranges. Reword inside lines 24 to 30 —
  the exact bytes of the `«tree-header-no-canonicalising»` fragment, chapter 5's
  — or carry the manifest, the source index's root and ownership rows, the
  fragment index and every affected fragment on both pages in the same commit.

## Done when

- The header clause in `crates/grove-loop/src/task_tree.rs` states something the
  file bears out, reworded inside the existing line counts or with every affected
  ledger row and fragment carried in the same commit.
- The six production call sites are re-enumerated at the time of the fix rather
  than taken from this file, and any count on a page matches that enumeration.
- `05-opening.md` and `06-paths.md` agree with the corrected comment, and the
  `leaf_entry` early-use row agrees with both.
- `bash scripts/check.sh` passes, including `book-check --final` over the
  `grove-loop` book. This leaf sits after `grove-loop-book-k37`, so the book is
  final by the time it runs and the root brief's one-commit rule can be met in
  full.

## Notes

**This is the third claim of one class in this crate, and the class is worth
naming in the fix.** `lib.rs`'s *`<worktree>/.grove`, spelled in exactly one
place* (`grove-root-join-clauses-k148`, the leaf before this one) and the
manifest's `libc` clause (`manifest-dependency-clauses-k133`) are the other two.
All three are uniqueness claims written from the shape of the design rather than
from an enumeration of the code, and none of the three has a test over its call
sites — unlike `the_librarys_tree_lock_is_taken_from_exactly_one_module`, which
is exactly the instrument they lack. Whether to add one for canonicalisation is
this leaf's to decide and to record either way; **consider doing all three files
in one commit with k148**, which sits immediately before this leaf.

**The corpus freeze does not license this leaf to break it either.** One commit
carries the source change, every affected ledger and page, and a green validator
run over every book it touched — or the leaf is deferred behind the books it
would invalidate and says so here.

## Decisions (running log)
