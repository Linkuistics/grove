# lease-size-ranking-k171

## Goal

Correct the two places that rank `crates/grove-loop/src/driver_lease.rs` by size
and get the rank wrong — the structure brief's chapter 16 section and
`docs/walkthroughs/grove-loop/13-outcomes.md` — so that no later chapter inherits
either number.

## Context

- **The enumeration.** The corpus's thirteen roots by line count are
  `tree_lifecycle.rs` 2,725, `task_tree.rs` 2,023, `task_name.rs` 1,714,
  **`driver_lease.rs` 1,383**, `loop_driver.rs` 615, `task_grow.rs` 518,
  `lib.rs` 377, `verbs.rs` 363, `session_config.rs` 358, `prompt.rs` 245,
  `complete.rs` 96, `Cargo.toml` 59, `driver.rs` 57. `driver_lease.rs` is the
  **fourth** largest. Separately, the twenty owned blocks by line count begin
  819 (chapter 16), 808 (13), 775 (12), 700 (4) — so chapter 16's block is the
  **largest single owned block in the book**.
- **Defect 1 — `docs/specs/grove-loop-book-structure.md`, *16 · One live driver
  per working tree*.** *At 12% comment prose over 819 lines it is the
  thinnest-argued root in the corpus and the third largest.* The first clause is
  true — 103 comment lines in 819 is 12.6% and is the minimum across the corpus.
  The second is wrong under either reading: the **root** is fourth, and the
  **block** is first.
- **Defect 2 — `13-outcomes.md`, the *second thesis* paragraph.** *The second
  largest block in the corpus is 1,383 lines of locking whose whole purpose is
  to hold state the tree must not hold.* 1,383 is the whole root rather than a
  block — the root splits at its `#[cfg(test)]` line into 819 and 564, and
  neither is 1,383 — and as a root it is fourth rather than second. **The
  argument the sentence makes is correct and only the ranking is wrong**, which
  is why chapter 16 inherits the argument and states the sizes itself rather
  than repeating the number.
- **The phrase is invisible to a one-line grep** in the structure brief, where it
  wraps as `the third\nlargest`. Enumerating the roots is what finds this class;
  searching for the sentence is not.
- **Nothing in the frozen corpus changes**, so this leaf is not deferred behind
  the book: both defects are in a specification and a book page, and no
  ownership range, manifest value or fragment range moves.

## Done when

- The structure brief's chapter 16 section states the rank that enumeration
  gives, or drops the ranking clause; its *thinnest-argued* clause is left alone,
  because it is true.
- `13-outcomes.md`'s sentence keeps its argument and states a size a reader can
  check — the root's 1,383 lines without a rank, or chapter 16's 819-line block
  as the largest — and chapter 16's own *The rule* section still agrees with it.
- No other page or brief repeats either ranking. Check by **enumerating the
  roots**, not by grepping a phrase.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs, and
  `bash scripts/check.sh` is no worse than before.

## Notes

**Placed before chapter 17** so the remaining Part V chapters, and chapter 21's
assembly page, read a corrected brief. That is the same placement
`pick-test-count-k147` and `structure-brief-chapter-attributions-k163` took for
the same reason.

**Do not widen this into a sweep of every size claim in the book.** Chapter 13's
own *third largest inline-test block in the book behind chapter 12's 569 and
chapter 14's …* was not checked here and is not this leaf's; a chapter that
carries a ranking checks its own by enumeration.

## Decisions (running log)

1. **The enumeration reproduces, independently.** `wc -l` over
   `crates/grove-loop/Cargo.toml` and every `src/**/*.rs` gives 12,213 across
   fourteen files; less the excluded `src/task_grow/tests.rs` (1,680) that is
   **10,533 over thirteen roots**, matching the brief exactly. Ordered: 2,725 /
   2,023 / 1,714 / **1,383** / 615 / 518 / 377 / 363 / 358 / 245 / 96 / 59 / 57.
   `driver_lease.rs` is **fourth**. The leaf body's figures are confirmed rather
   than assumed.
2. **"Owned block" has two readings and only one reproduces the body's
   numbers.** `walkthrough.toml` holds **39** `[[block]]` records, whose largest
   are 819 / 615 / 569 / 564 — not the body's 819 / 808 / 775 / 700. Aggregating
   block lines by **owning page** gives exactly **twenty** pages owning source
   and 819 / 808 / 775 / 700 at the top, which is the body's list. So the unit
   behind *largest single owned block in the book* is the page's total, chapter
   16's `one-per-working-tree` at 819 is first, and the correction says which
   unit it means rather than leaving it implicit. That implicitness is the whole
   defect: a rank with no unit cannot be checked.
3. **Three live sites, not two — the leaf body named two.**
   `docs/specs/grove-loop-book-structure.md` line 154 (the rejected-spines
   paragraph) calls `driver_lease.rs` *the second-largest owned block*, which is
   the same error as Defect 2 — a root ranked as a block — in a different section
   of the same brief. It is in scope under *No other page or brief repeats either
   ranking*, and it is exactly the failure mode `execute.md` names: a finding
   against one section does not reach the other layers of the same document.
   Corrected with the other two.
4. **The instrument was controlled before it was believed.** A line grep for
   `third largest` over `docs/` returns only `13-outcomes.md:765` — chapter 13's
   own, unrelated claim — and **misses the structure brief entirely**, because
   the phrase wraps as `the third\nlargest`. The whitespace-normalised sweep
   finds it (positive control) and returns `False` for `ninth largest` in the
   same file (**seen to fail**), so a clean result from it is a reading rather
   than a broken instrument.
5. **Enumerated the subject, not a phrase list.** Two sweeps over every `.md`,
   `.rs` and `.toml` in the repository with whitespace normalised: one for
   `largest|smallest|biggest` near a lease subject (40 windows), one for every
   `driver_lease|1,383` near any rank word (50 windows). Every window was
   classified. Beyond the three corrected sites the live hits are chapter 16's
   own enumeration, chapter 12's *largest single inline-test block* (569, ahead
   of 564 and 491 — checked, correct) and chapter 13's *second largest of
   `tree_lifecycle.rs`'s five production blocks* (317 behind 331 — checked,
   correct). Both are the file-scoped rankings the *Notes* put out of scope, and
   both hold. No fourth site ranks this root under any other wording.
6. **The corrections state the unit, not just a corrected number.** The brief's
   chapter 16 section now reads *the **fourth** largest root at 1,383 lines … and
   its 819-line production half is the **largest single owned block in the
   book***; the *thinnest-argued* clause beside it is untouched, because it is
   true and, checked separately, is not load-bearing on the clause removed.
   Line 154 now names the production half as the largest owned block instead of
   miscalling the root a block. `13-outcomes.md` **drops the rank** and keeps
   *1,383 lines of locking whose whole purpose is to hold state the tree must not
   hold* — the argument the leaf body says is correct, and a size a reader can
   check — which is the first of the two options the *Done when* offers.
7. **Chapter 16 needed a consequential edit, and leaving it would have been a
   new defect.** `16-the-lease.md` asserted in the present tense that *two
   sentences in this book get it wrong*, named which two, and said
   `lease-size-ranking-k171` holds the correction. Landing the fix makes all
   three clauses false. The paragraph keeps its enumeration — the one place a
   later page should take a size from — and now explains *why* the rank is easy
   to get wrong (root and owned block are two different units, only one of which
   is 1,383 lines) instead of citing pages that no longer err. Its *The rule*
   section agrees with the corrected `13-outcomes.md`, as the *Done when*
   requires.
8. **One live brief amended, retired task files left as history** — the k132
   precedent, decision 6. `what-a-runner-cannot-k129`'s `BRIEF.md` is read by
   chapters 17 to 21: it quoted `13-outcomes.md`'s old wording as the source
   chapter 16 inherits, and carried *Until it lands, do not reconcile a page to
   either rank*, both of which would misdirect the next stage. Both rewritten as
   settled, recording that the class stays open even though this root's rank is
   closed. The remaining `.grove/` hits are retired task files and decision logs
   recording what the pages said at the time; those are correct as written.
9. **The measurement was restarted once, deliberately.** A first
   `scripts/check.sh` was launched and then killed rather than read: a wrap fix
   to `docs/specs/grove-loop-book-structure.md` landed while it was running, and
   that file is one of the run's subjects. An instrument adjusted mid-reading has
   not read anything, so the run was discarded and re-launched over a digested,
   frozen tree (spec `4feacd7c…`, `13-outcomes.md` `4a422664…`, `16-the-lease.md`
   `e431a6ea…`).
10. **`book-check` is green at the slice and `scripts/check.sh` is no worse.**
    `--through one-per-working-tree --check all` → **valid: 13 files, 8,751
    resolved lines, 1,782 deferred, final=false** — byte-identical to the state
    `the-lease-k164` left, so the fragment graph is untouched by these prose
    edits. `scripts/check.sh` → **FAILED — 1 of 8**, red on `book-check` alone;
    `cargo test` passed, with none of the `driver_lease.rs` fixture flakiness the
    brief warns about. The other five books all report `final=true`.
11. **The red is prefix-shaped, and provably not this leaf's.** Every failing row
    names an unwritten chapter: `M101` for pages 17 to 21; `M103` for `README.md`
    and chapter 16's `Next` link to a page that does not exist; `F003` for the
    four blocks (`lease-tests`, `whose-file`, `the-prompt-core`, `loop-driver`)
    deferred to chapters 17 to 20; `F009` at `source-index.md` 131 and 594–596
    for the same deferred block and the three cast rows still `pending`. The cast
    row owned by `one-per-working-tree` at line 593 reads `explained` and is not
    among them. All of it is in `source-index.md`, **a file this leaf does not
    modify** — the diff is five markdown files and no source, so the corpus
    freeze is intact and this leaf was never deferred behind the book.
12. **No in-session reviewer was spent.** The allowance is leaf-wide and this
    leaf's claims are arithmetic over line counts, each reproduced twice from the
    bytes and controlled against a mutation of the instrument. There is no
    hard-to-reverse decision and no claim the enumeration cannot settle, so the
    allowance is left unspent rather than used for a second opinion.
