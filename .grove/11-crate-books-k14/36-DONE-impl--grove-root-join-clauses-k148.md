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

1. **The enumeration was wider than this file recorded, and the extra sites are
   in another crate.** The task file named three second-spellings, all inside
   `grove-loop`. Re-run at the time of the fix, `grep -rn 'join(".grove")'
   crates/*/src/` gives thirty hits; twenty-three are inside test modules
   (nineteen in `tree_lifecycle.rs`, whose `mod tests` opens at 1078; one in
   `task_tree.rs`, `mod tests` at 1017; three in `task_grow/tests.rs`, whose
   whole file is the module `task_grow.rs` declares under `#[cfg(test)]` at line
   517). That leaves **seven** production sites, not four: `lib.rs` 270,
   `tree_lifecycle.rs` 76 and 197, `driver.rs` 55 — and
   `crates/grove-llm/src/cli.rs` 505, 570 and 881. The last three are the
   strongest refutation of *no caller can spell it a second way*, because they
   are a **consumer outside the crate**, and the original clause was written
   from an enumeration that never left `grove-loop`. Checked for alternative
   spellings too (`format!`, `push`, a constant): there are none.

2. **The correction splits openings from renderings, because the two groups
   exist for different reasons.** The three inside `grove-loop` are openings that
   cannot route through `read` or `write`: `transition_to_current` and
   `materialize_finish` are the driver's two tree operations and run before it
   has an opening to give them, and `finish_commit` is a session verb that opens
   the tree itself. The three in `grove-llm` open nothing — they spell the root
   to **name** it in output, because the library returns no path for a root it
   did not open, and `cli.rs`'s own comment above line 570 says exactly that.
   Counted as openings rather than as sites the crate figure is five, which is
   the same five `TreeWrite`'s header already names as the calls that must not
   be made while a guard is in hand. Same move as
   `every-member-version-comment-k84` and k133's `keyed-launch` clause: the
   count was not the defect, the domain was.

3. **Both rewordings stayed inside their own line counts, and that is what kept
   the fan-out to one book.** `lib.rs` is still 377 lines, so the `library-root`
   block's `1-377`, the `walkthrough.toml` root row, `source-index.md` and every
   fragment range are untouched, and `book-check --final` still reports
   **10,542 resolved lines** — the control. k133's comparable fix grew a
   manifest by nine lines and reached ten surfaces across two books plus the
   spec. Surfaces here: `lib.rs` (two comments), `01-orientation.md` (two
   fragments, the section heading, three prose sites and the new passage),
   `06-paths.md`'s class paragraph, `21-what-could-not-move.md`'s tally,
   `concept-index.md`, `docs/specs/grove-loop-book-structure.md`, and one comment
   in `crates/grove-loop/tests/verbs.rs` (decision 5).

4. **No test was added, and the reason is the disanalogy rather than the cost.**
   `the_librarys_tree_lock_is_taken_from_exactly_one_module` pins a **safety**
   property — a second module taking the store's lock is the deadlock
   `collapse-tree-access-k13` deleted a layer to remove. A count of
   `join(".grove")` call sites pins tidiness. A sixth opening that takes a
   worktree would need a join of its own and would violate nothing, so a test
   over the count would go red on a change that is correct, which is worse than
   no instrument at all. The absence is recorded on the page and in the spec's
   entry 4 instead. This decision binds this claim only;
   `canonicalisation-sites-k149` judges the same question for its own, and
   chapter 6 now says so rather than leaving *that test is what the class lacks*
   reading as a standing recommendation.

5. **A third copy of the retracted claim was fixed in the same commit, and two
   more were left.** `crates/grove-loop/tests/verbs.rs` line 132 carried *so no
   consumer can spell the grove root a second way* — the same sentence, in the
   same crate, and false for the same reason. It is in `tests/`, which is
   evidence rather than a book root, and no page cites the file by line, so the
   fix has no fan-out; shipping the retraction while leaving a verbatim copy
   asserting the opposite would have been worse than doing either alone. Two
   survivors were **not** touched and are findings rather than omissions:
   `crates/grove-llm/src/cli.rs` line 865 (*no caller here can spell the grove
   root a second way*, sixteen lines above one of the three spellings) is a
   `grove-llm` corpus root whose own book already adjudicates it at
   `02-the-grammar.md`, and `tree_lifecycle.rs` line 1195 (*the one
   `<worktree>/.grove` is joined in*, inside the test module) is in this book's
   corpus, so changing it would move fragment ranges. Neither is this leaf's.

6. **The one in-session reviewer paid, and every finding it raised was
   re-derived here before being acted on.** Fifteen findings, thirteen valid.
   The two that mattered most were factual errors I had asserted from a module
   header rather than from a signature: `finish_commit` takes a `&Workspace` and
   reads the worktree off it, so *each take a worktree* was false; and
   `finish_commit` is a session verb, not one of the driver's two tree
   operations, so the driver's explanation did not cover all three sites. A
   third was self-falsifying: *a source fix under the corpus freeze reached no
   page but this one*, in a commit that changes three other pages of the same
   book. Also valid: the new `lib.rs` line 268 said *Three other sites spell it*
   where the book uses **sites** for the seven-element set — the defect class
   this leaf exists to remove, re-committed in the fix for it, and now reworded
   to make no count at all; the concept-index entry dropped the qualifier that
   made *six* true; an inserted sentence split the antecedent of *This one* in
   chapter 1's class paragraph; *the end of this chapter* named a section with
   two sections and 190 lines after it; *line 570's own comment* is at 566–568;
   *exactly these five* overstated a match that holds only after a delegation
   hop; and the true half of the old clause — *the only spelling grove has ever
   opened* — was deleted with the false half and unremarked, though it still
   stands at `tree_lifecycle.rs` line 1020 and chapter 11 reads it there. The
   page now names that deletion. Two findings were rejected as scope rather than
   defect: the spec's *Known in advance* has never been the book's whole ledger
   of judged claims, and chapter 21's sentence about it inherits that; rather
   than renumber a set the section was never counting, both now say what the
   list is over, which is claims a leaf has corrected **at source**.

7. **The gate was green first time, unlike k133's run.** `bash scripts/check.sh`
   printed **check: all 8 principal checks pass**, exit 0, with all six books
   valid and `grove-loop` at 10,542 lines — the same figure as before the change,
   which is the control for decision 3. Two prose-only edits landed after that
   run began, so the two checks that read the changed Markdown were re-run
   against the final bytes: `cargo test --locked -p grove --test
   reference_navigation` (13 passed) and `book-check --final --check all` over
   `grove-loop` (valid). The driver-timing flakes k133 attributed to
   `driver-lease-fixture-timing-k85` did not appear, despite an unrelated cargo
   build competing for the machine throughout.

