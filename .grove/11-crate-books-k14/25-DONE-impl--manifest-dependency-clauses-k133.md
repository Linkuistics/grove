# manifest-dependency-clauses-k133

## Goal

Bring two clauses of `crates/grove-loop/Cargo.toml`'s dependency comment up to
the crate as it is since `loop-crate-driver-k22`, and rewrite the paragraph in
`docs/walkthroughs/grove-loop/01-orientation.md` that adjudicates them, in one
commit with a green validator run over the book.

## Context

- **The defect.** The comment at `crates/grove-loop/Cargo.toml` lines 17–25
  describes the crate's dependencies as they stood before the driver moved into
  this crate, and two of its clauses now stop short of the code:
  - *`libc` is the lock-contention probe in `task_tree`, which needs `flock(2)`
    non-blocking before it announces a wait.* `libc::` is reached from three
    modules, not one — eight times in `task_tree.rs` (the probe), **nineteen in
    `driver_lease.rs`** for the lease's locking and its close-on-exec
    descriptors, and three in `loop_driver.rs` for `isatty`, `tcgetpgrp` and
    `signal`. The largest user is the one the comment does not name.
  - *`keyed-launch` is the runner, reached by exactly one verb: `complete` …*
    True of the **verbs**, and `complete` is still the only one. It is not true
    of the crate: `driver_lease.rs` discards an abandoned channel,
    `loop_driver.rs` spawns and reaps through the runner, and
    `session_config.rs` compiles grove's templates against the runner's own
    types.
- **Found at `orientation-k124`, while drafting chapter 1**, and it is a **third**
  stale claim in this corpus — `docs/specs/grove-loop-book-structure.md`'s *No
  third stale claim was found* is wrong, and that sentence is part of the fix.
- **The book already adjudicates it and does not repeat it.**
  `01-orientation.md`, under *The package*, states which clause is still true and
  which is not, beside the fragment that reproduces both. When this leaf lands,
  that paragraph is rewritten in the same commit as the comment, because the
  adjudication exists only for as long as the claim does.
- **This is a corpus change and is deferred behind the whole `grove-loop` book,**
  which is why it sits here rather than ahead of it. The comment is inside
  `manifest-domain-bound`, lines 1–59, owned by chapter 1: adding a line moves
  every line below it and invalidates that block's fragments and the book's
  ledger. `crates/grove-loop/Cargo.toml` belongs to no other book, so the blast
  radius is one book — but that book must be complete before the bytes move.
- Its neighbours are the same shape and were placed for the same reason:
  `every-member-version-comment-k84` and `template-source-read-count-k86`.

## Done when

- The two clauses describe the crate as it is, naming `driver_lease` and
  `loop_driver` for `libc` and distinguishing *the verb surface reaches the
  runner once* from *the crate reaches it in four places*.
- `01-orientation.md`'s adjudicating paragraph is rewritten to match, and the
  book's fragments and ledger are updated for whatever line movement the edit
  causes.
- One commit carries the source change, the affected pages and ledger, and a
  green `book-check --final --check all` over `docs/walkthroughs/grove-loop`.
- `docs/specs/grove-loop-book-structure.md`'s *Known in advance* section names
  this as the third adjudicated claim rather than asserting there is no third.
- `bash scripts/check.sh` passes.

## Notes

**Do not fix this by deleting the clauses.** The dependency argument is the
chapter's evidence for what the crate imposes; what is wrong is its scope, not
its existence.

## Decisions (running log)

1. **Both of the task file's counts were wrong, in opposite directions, and
   re-deriving them is what shaped the fix.** The leaf was written at
   `orientation-k124` and its figures had not been re-measured since.
   - *`libc`: "nineteen in `driver_lease.rs`" is a whole-file line count that
     includes the inline test module.* `driver_lease.rs`'s `mod tests` opens at
     line 821; six of those nineteen lines are inside it (952, 955, 1048, 1137,
     1193, 1329). The production figure is **thirteen**. `task_tree.rs`'s eight
     and `loop_driver.rs`'s three are both wholly production (`mod tests` at
     1017 and 548). Twenty-four production lines, and `driver_lease` is still
     the largest user either way, so the leaf's conclusion survives its
     arithmetic. The comment says *three production modules* because
     `task_grow`'s inline tests are a fourth site — and they are the 1,680 lines
     the book's corpus excludes, so "three modules" without the qualifier would
     have been false about the crate while true about the book.
   - *`keyed-launch`: "four places" is five.* The task file names `complete`,
     `driver_lease`, `loop_driver` and `session_config`. It misses
     `src/lib.rs` line 87, `pub use keyed_launch::reraise;` — the only one of
     the five that is not a call but a **re-export**, putting a runner item into
     this crate's own public surface. Writing "four" would have replanted a
     count defect while fixing one. The comment and the page say *the verb
     surface once, the crate in four more places*, which is five sites and
     satisfies the *Done when* clause without the error.

2. **The wording splits the two clauses rather than treating them alike, because
   they were wrong in different ways.** `libc`'s clause was simply incomplete —
   it named one user of three. `keyed-launch`'s clause was **true of the verbs
   and false of the crate**: `complete` is still the only one of the twelve verbs
   chapter 15 counts that reaches the runner, so the defect was not the claim but
   the domain it was read over. Correcting it by widening the number would have
   thrown away a true and load-bearing fact; the fix is the distinction. This is
   the same move `every-member-version-comment-k84` made — change the
   quantifier's domain, not the set — and the page now explains it as such.

3. **Deleting the clauses was ruled out by the task file and the code agrees.**
   Chapter 1's whole argument is that the manifest exposes the cost of the
   crate's permission to be domain-bound; a dependency table with no reasons
   proves nothing. What was wrong was scope, not existence.

4. **Superlative caught before it shipped.** A draft sentence called the
   `reraise` re-export *the only place this crate puts a dependency's item into
   its own public surface*. `lib.rs` has three such `pub use` lines covering four
   names — `jj_workspace::{Commit, Workspace}`, `keyed_launch::reraise`,
   `ordinal_fs_tree::Sought` — and the chapter already says so 400 lines later
   (*Four names in that block belong to no chapter of this book*). Corrected to
   *one of four names* before the page was written, by enumeration rather than
   by review.

5. **The fan-out was ten surfaces, and `book-check` reads two of them.** The
   manifest grew 59 → 68, so chapter 1's owned lines went 436 → 445 and the
   corpus 10,533 → 10,542. Inside the book: the six fragment fences and the
   reproduced fragment body (validator-checked); `walkthrough.toml`'s root and
   block rows; `source-index.md`'s root table, ownership-blocks table, fragment
   index, per-slice table and prose; `01-orientation.md`'s *all fifty-nine
   lines*, its lead-in paragraph describing the old comment, and the
   adjudicating paragraph; `README.md` (three), `10-growing.md`,
   `14-finishing.md`, and `21-what-could-not-move.md` (seven, including an
   explicit addition chain). Outside it: `docs/walkthroughs/jj-workspace/`'s
   reproduced `scripts/check.sh` transcript, and the spec.

6. **The cross-book surface is one book, not four, and only enumeration shows
   it.** `lossy-path-rendering-k66` recorded that a moved line count falsifies
   the `check.sh` transcript in *each of the other four books*. That is not true
   here: `grove-llm`, `keyed-launch` and `overview` reproduce transcripts from
   before this book existed — they list four, five and three books and never
   mention `grove-loop`. Only `jj-workspace/07-what-jj-owns.md` was later
   refreshed to the six-book run and carries `10533`. The lesson holds; its
   count did not.

7. **Two surfaces that no token sweep would have found.** `05-opening.md`'s *On
   `libc`* paragraph and `06-paths.md`'s *The class is familiar* paragraph both
   argue **from** this defect in prose, naming neither `59` nor `10,533`. They
   were found by grepping the book for `adjudicat` — the concept — rather than
   for the tokens the edit changed. `06-paths.md` groups this claim with two
   others as a class of *uniqueness claims written from the shape of the design
   rather than from an enumeration of the code*; the class survives the fix and
   the passage now says this member has landed.

8. **Frozen measurement records were left stale, on precedent.**
   `.grove/BRIEF.md`'s corpus table, `.grove/11-crate-books-k14/BRIEF.md`, the
   `k37` subtree's task files, `docs/evaluations/**` and
   `docs/adr/a-book-carries-no-asset.md`'s incidental *10,533* all keep the old
   numbers. This is `lossy-path-rendering-k66` decision 10 applied unchanged,
   and it is observably the established practice rather than an inference:
   `jj-workspace` is **752** lines on disk against the root brief's **698**, and
   no leaf has touched it. The brief says *counts are of the corpus as frozen*,
   and rewriting it would make it false about what it asserts.

9. **The structure brief is the exception, because `## Known in advance` is a
   current-state set under `SPEC-FORMAT.md`.** Four things there moved, as k83
   and k84 established: the summary bullet near the top (*a second such claim* →
   *a second and a third*), the section's intro (two claims → three, and where
   the third was found), chapter 1's per-chapter pointer, and chapter 5's. The
   sentence **No third stale claim was found** — which the task file names as
   part of the defect — is replaced by item 3 itself, which records why the
   brief could say it: the claim was found at `orientation-k124`, after the
   brief was written. The brief's *tables* are left as written, with one added
   note saying the corpus has moved and naming `walkthrough.toml` and the source
   index as authoritative — the same treatment k66 gave the `jj-workspace`
   brief.

10. **Chapter 21's tally was checked and is not falsified.** *Two were known
    false before drafting began* is a claim about what was known **before**
    drafting, and this defect was found during it, so it belongs to the *many
    more were found while drafting* set the same paragraph describes. Its
    closing sentence — *where a page judged a source change worth making, a leaf
    carries it* — is what this leaf discharges. Left unedited deliberately: the
    assembly tally counts a different set from the spec's.

11. **The leaf's one in-session reviewer was spent on the changed prose, and it
    paid.** The claim put to it: *every count, uniqueness claim, superlative,
    line range and cross-reference in the changed files holds against the source
    as it now stands*, with an instruction to count for itself. Six findings,
    each re-derived here before being acted on. Five stand, one is rejected.

    - **Valid — my own sweep missed it (two).**
      `docs/specs/grove-loop-book-structure.md`'s chapter-1 description still
      quoted both clauses in their pre-fix wording (*`libc` for the `flock(2)`
      contention probe alone; `keyed-launch` reached by exactly one verb*), 840
      lines above the *Known in advance* item calling them corrected — an
      internal contradiction inside a file I had already edited. And
      `05-opening.md`'s section lead still said the probe is *the* use the
      manifest names, a definite singular, 116 lines above the paragraph I had
      corrected to *first among three*. Both were invisible to a token sweep:
      neither carries a number.
    - **Valid — a count I introduced myself (one).** I wrote *eight of the
      twenty-four production lines*. `task_tree.rs` line 219 is a `///` doc
      comment, so its eight lines are seven of code; `driver_lease`'s thirteen
      and `loop_driver`'s three carry no doc comments at all. The rule was
      applied inconsistently and the total was 23, not 24. Fixed by deleting the
      total rather than repairing it: the page now says `driver_lease` reaches
      `libc` on more lines than the other two together (13 > 7 + 3), which is
      the fact the sentence was for and does not turn on how a comment is
      counted. Same fix in the spec's item 3.
    - **Valid — a uniqueness claim I introduced myself (one).** *A fourth site
      exists in test code* named `task_grow` and stopped there. `driver_lease`'s
      own `#[cfg(test)]` module reaches `libc` on six more lines — four locking,
      two asserting `FD_CLOEXEC` — and those are **inside the corpus**, owned by
      chapter 17. `task_grow` is a fourth *module* but not the only further
      *site*, and the reason the clause says *production* is stronger stated
      properly. Exactly the class this book records as its most common defect,
      committed while writing the paragraph that fixes an instance of it.
    - **Valid, and reversing decision 10 (one).** Chapter 21's tally survives
      literally — *two known false before drafting began* is a claim about prior
      knowledge — but the sentence after it says *each* of the many found while
      drafting *was adjudicated beside its fragment*, and that is no longer true
      of this one. One sentence added, naming what landed and pointing at the
      brief's count of three; nothing decremented.
    - **Rejected (one).** `docs/adr/a-book-carries-no-asset.md` line 117's
      *thirteen roots and 10,533 lines* is the survivor the reviewer wanted
      moved. It is the second half of a comparison whose first half is
      *`jj-workspace` is four roots and 698 lines* — a figure
      `lossy-path-rendering-k66` left stale **by name in this same file**,
      because the sentence describes what the pilot measured rather than what
      the corpus is. `jj-workspace` is 752 lines today and that 698 still
      stands. Moving one half of a measured comparison and not the other would
      be worse than moving neither, and moving both would falsify what the
      paragraph asserts. Left as written.

12. **`scripts/check.sh` went red once on the way, and it was
    `driver-lease-fixture-timing-k85`, not this change.** The first full run
    failed `a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver`
    and `a_sigtermed_driver_stops_and_reaps_its_child` in
    `crates/grove/tests/loop_driver.rs`, then wedged in
    `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal` — the
    exact fixture, the exact failure mode and the exact wedge k85 records, with
    the first of those two failures named verbatim in that leaf. Attributed by
    measurement rather than by reading the leaf: `cargo test --locked -p grove
    --test loop_driver` alone passes **all eleven in 42.20s**, against k85's
    predicted ~40s, and the two failures do not reproduce. The change under test
    is a comment in a `Cargo.toml` and Markdown, which cannot reach driver
    timing. A second full `scripts/check.sh` on a quieter machine printed
    **check: all 8 principal checks pass**, exit 0 — the *Done when* run. k85
    stays live and unamended: it already records that a green run is not
    evidence the deadline is adequate, and this session adds no hypothesis it
    does not have.
