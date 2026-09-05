# grove-root-join-clauses-k148

## Goal

Correct `crates/grove-loop/src/lib.rs`'s two claims that `<worktree>/.grove` is
spelled in exactly one place, and add the adjudicating paragraph chapter 1 of the
`grove-loop` book owes beside the fragments that reproduce them.

## Context

- **The defect, in two places in one file.** `crates/grove-loop/src/lib.rs` line
  268 documents `fn grove_root` as ``/// `<worktree>/.grove`, spelled in exactly
  one place.``, and the module header at lines 28 to 30 says the entry points
  *take a **worktree**, not a grove root: `<worktree>/.grove` is the only
  spelling grove has ever opened, and putting the join here means no caller can
  spell it a second way*. Three production callers spell it a second way:
  `crates/grove-loop/src/tree_lifecycle.rs` line 76 (`transition_to_current`),
  the same file line 197 (`finish_commit`), and
  `crates/grove-loop/src/driver.rs` line 55 (`materialize_finish`). Enumerated
  with `grep -rn 'join("\.grove")' crates/grove-loop/src/` over non-test lines;
  the remaining hits are inside `#[cfg(test)]` modules.
- **This is the third clause of one class in one file, and chapter 1 found the
  first two.** `01-orientation.md#the-package` already adjudicates the
  `Cargo.toml` clauses about `libc` and `keyed-launch` under the heading *Two of
  those clauses describe the crate as it was before the driver arrived*, and
  `manifest-dependency-clauses-k133` holds their source fix. `materialize_finish`
  is `loop-crate-driver-k22`'s, so this clause belongs to the same class and was
  missed rather than judged. **Consider doing both files in one commit with
  k133**, which sits at position 25 of this node and runs first.
- **Found by `opening-k141`**, chapter 5's draft, whose own figure had repeated
  the claim as a parenthetical; the page was corrected to name `lib.rs::grove_root`
  and its two public openings instead. Chapter 5 owns none of these bytes, so it
  could stop repeating the claim but could not adjudicate it — adjudication sits
  beside the fragment that reproduces the claim, and both fragments are chapter
  1's.
- **The fix must stay inside the existing line counts.** `lib.rs` is one
  ownership block, `library-root`, declared `1-377`, and the root row in
  `walkthrough.toml` and `docs/walkthroughs/grove-loop/source-index.md` both carry
  377. Reword within the existing lines, or carry the manifest, the source index's
  root and ownership rows, the fragment index and chapter 1's affected fragments
  in the same commit.
- **What to say instead.** The true claim is narrower and still worth making: the
  join is in one place *for the crate's two public openings*, which is what makes
  `read` and `write` take a worktree rather than a grove root. What it is not is a
  crate-wide invariant, and nothing enforces it — there is no test over
  `join(".grove")` call sites, unlike
  `the_librarys_tree_lock_is_taken_from_exactly_one_module`, which is exactly the
  instrument this claim lacks. Whether to add one is this leaf's to decide and to
  record either way.

## Done when

- Both clauses in `crates/grove-loop/src/lib.rs` state something the code bears
  out, reworded inside the existing line counts or with every affected ledger row
  and fragment carried in the same commit.
- `docs/walkthroughs/grove-loop/01-orientation.md` adjudicates the claim beside
  the fragments that reproduce it, in the register its *Two of those clauses*
  paragraph already uses, and does not silently correct it.
- The four production sites are re-enumerated at the time of the fix rather than
  taken from this file, and the count in the page matches what that enumeration
  found.
- `bash scripts/check.sh` passes, including `book-check --final` over the
  `grove-loop` book. This leaf sits after `grove-loop-book-k37`, so the book is
  final by the time it runs and the root brief's one-commit rule can be met in
  full.

## Notes

**The corpus freeze does not license this leaf to break it either.** One commit
carries the source change, every affected ledger and page, and a green validator
run over every book it touched — or the leaf is deferred behind the books it
would invalidate and says so here.

## Decisions (running log)
