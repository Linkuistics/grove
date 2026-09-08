# refused-grove-test-overclaims-k161

## Goal

Decide what `a_refused_grove_leaves_no_root_behind` should be, given that it is
mechanically the same test as `root_init_rejects_a_bad_slug_without_leaving_a_grove_behind`
and its doc comment claims a property neither of them exercises — and reconcile
the page that reproduces it.

## Context

- **The duplication.** `crates/grove-loop/src/tree_lifecycle.rs` 1318–1326 and
  1430–1440 call `root_init_at(&wt, "Bad Slug")`, assert `is_err()`, and assert
  `!wt.join(".grove").exists()`. Only the assertion messages differ —
  *".grove must not be created on a bad slug"* against *"a refused root-init must
  leave no root"*.
- **The overclaim.** The second test's doc comment (1424–1428) says *the store
  creates the root, places the charter and the first leaf, and takes the root back
  down if any of it fails — so the partial shape `root-init` used to leave between
  its two phases is not one grove can produce any more.* Its own inline comment
  concedes the truth: *A slug the grammar refuses, checked before the lock is
  taken.* `root_init_at` runs `Slug::new` on line 1211 and
  `task_tree::write_or_vacancy` on 1213, so a refused slug never enters
  `initialize_grove` and no store operation is begun. What the test holds is that
  grove's front door creates no directory.
- **Nothing in this workspace observes the unwinding.** Establishing it would need
  `TreeVacancy::initialize` to fail *after* the root directory exists, and no
  fixture constructs that. Confirmed by mutation while drafting chapter 11: panics
  placed on `initialize_grove`'s `map_err`, on both its `bail!` arms and on the
  `allocated` guard each leave all 558 `grove-loop` and `grove-llm` tests green
  against an 11-failure control. The property is true — it is `initialize`'s contract,
  with `ordinal-fs-tree`'s own tests behind it — but it is true on that crate's
  word, not on this block's evidence.
- **The structure brief pairs it with the rule.**
  `docs/specs/grove-loop-book-structure.md`'s chapter 11 section says
  `root_init_creates_the_whole_grove_through_one_store_operation` holds the rule
  *and `a_refused_grove_leaves_no_root_behind` is its negative*. That reading is
  what this leaf either earns or corrects.
- **Found by `a-grove-begins-k155`**, which adjudicates it at
  `11-a-grove-begins.md#what-the-tests-establish` rather than editing the frozen
  corpus.

## Done when

One of these is true, and the leaf says which and why:

- **Earn the claim** — a test makes `initialize` fail after the root exists and
  asserts no root survives, so the doc comment becomes true of something. This is
  the outcome that would make the structure brief's pairing correct as written.
- **Narrow the claim** — the doc comment is rewritten to say what the assertions
  hold (grove's front door creates no directory, checked before the lock), and the
  duplicate is either removed or given a distinct fixture. Removing it shifts every
  later line of a 2,725-line root, so removal carries the ledger and the page.

And in either case:

- If the wording is narrowed, `docs/specs/grove-loop-book-structure.md`'s chapter
  11 section is reconciled in the same commit, as `pick-test-count-k147` and
  `structure-brief-lexical-pair-k150` did for chapter 7.
- `11-a-grove-begins.md` reproduces the new bytes and its adjudication is rewritten;
  any `concept-index.md` entry naming the defect follows.
- If nothing is added or removed, **`tree_lifecycle.rs` is still exactly 2,725
  lines**; if the count moves, every ownership range, manifest `lines` value and
  fragment range for the root moves with it in the same commit.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green at
  whatever slice the book is proved at when this runs.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Deferred behind the `grove-loop` book**, with `unresolved-doc-links-k151`,
`unreachable-root-clause-k152`, `grow-header-stale-helper-k154`,
`default-root-slug-two-spellings-k159` and `welded-grove-name-summary-k160`.

**This is a decision leaf, not a mechanical fix.** The two outcomes differ in what
the crate ends up proving, not just in wording, which is why it is not folded into
one of the two doc-comment leaves beside it.

## Decisions (running log)

1. **Earn the claim, don't narrow it.** `a_refused_grove_leaves_no_root_behind`
   keeps its name and its doc comment's substance, and takes a fixture that
   reaches the store: `root_init_at(&wt, &"a".repeat(300))`. The slug is one the
   grammar accepts — `refuse_token` bounds the character set and the ends, not
   the length — so `Slug::new` passes, `write_or_vacancy` hands out the vacancy,
   `initialize` creates the root, writes `BRIEF.md`, and fails placing
   `01-requirements--aaa…-k1.md` with `ENAMETOOLONG`. The store unwinds its own
   effects and `remove_dir`s the root. Measured, not read: the error is
   *"creating the leaf …: File name too long (os error 63). Nothing was changed —
   every effect this operation had applied was undone."*
2. **The reason is asserted, not left to the fixture.** The test now asserts the
   error contains `creating the leaf`. Without it a later length bound on `Slug`
   would silently restore exactly the defect this leaf fixes — the assertions
   would still pass and the doc comment would overclaim again. That assertion is
   what makes the fixture's *lateness* a tested property rather than a comment.
3. **The duplication is gone by divergence, not by deletion.**
   `root_init_rejects_a_bad_slug_without_leaving_a_grove_behind` keeps
   `"Bad Slug"` and holds the pre-lock refusal; this one holds the unwind. Two
   tests, two properties, no removal — so `tree_lifecycle.rs` is still exactly
   **2,725 lines** and no ownership range, manifest `lines` value or fragment
   range moves.
4. **Mutation is the evidence, with its control.** A `panic!` on
   `initialize_grove`'s `map_err` (line 391) — one of the four the leaf's Context
   records as surviving all 558 tests — now **fails** this test, and **passes**
   when the same test is reverted to the `"Bad Slug"` fixture under the same
   mutant. Green-here plus red-there is what rules out a broken instrument; the
   old fixture reaching nothing was measured, not inferred.
5. **The structure brief needs no edit, and that is the point of earning it.**
   `docs/specs/grove-loop-book-structure.md`'s chapter 11 section pairs
   `root_init_creates_the_whole_grove_through_one_store_operation` with
   `a_refused_grove_leaves_no_root_behind` as its negative. Under outcome A that
   pairing is now true as written — the positive holds the operation succeeding
   whole, the negative holds it failing and leaving nothing — so the *Done when*
   clause requiring reconciliation (which is conditioned on narrowing) does not
   fire. Reconciling k147's and k150's chapter 7 had a brief that had gone wrong;
   this one had a brief that was ahead of the code.
6. **The page's fan-out was five passages, not one.** Beyond the fragment bytes
   and the adjudication the leaf names, three more claims on the same page were
   falsified by the change and none of them sits under
   `#what-the-tests-establish`: the helper paragraph's *four of the chapter's
   thirteen tests … three of them* (now three and two, since the test is no
   longer decided in `root_init_at`); the front-door paragraph's *what they do
   not establish is that anything unwinds*, which now needs the forward pointer
   to the test that does; and the measurement section's row 2, its *seven of ten
   arms are held by nothing*, its *rows 2, 3, 4 and 5 are the class chapter 10
   named*, and its *three arms observed by four tests*. Two `concept-index.md`
   entries followed — *the same test twice, under two names and two claims* and
   *ten arms measured, seven held by nothing*.
7. **The measurement table was re-derived, not reasoned about.** All ten arms
   were re-mutated against the changed test in a workspace copy. Exactly one row
   moves: row 2 (line 391) goes from **nothing** to this test; rows 1, 3, 4, 5,
   6, 7, 8, 9 and 10 all survive it. That delta is complete because exactly one
   test body changed and no non-test source did, so no row's observer set can
   move except by this test entering or leaving it — which is why a ten-mutant
   run against one test replaces a ten-mutant run against 558. The 558 total and
   the 11-failure control are untouched: no test was added or removed.
8. **The first classifier reported the one row that mattered as unmeasurable.**
   It tested for `^error` before testing for a failed test, and `cargo test`
   prints `error: test failed` on a red test — so row 2, the only killed mutant,
   came back `COMPILE-FAIL` while the nine survivors read correctly. A classifier
   that cannot distinguish *did not build* from *built and failed* is exactly the
   broken instrument `execute.md` warns about, and it failed silently in the one
   direction that would have left the table unchanged.
9. **The retired records are left as they are.**
   `…/04-no-word-for-k127/BRIEF.md` and its `01-DONE-draft--a-grove-begins-k155.md`
   describe the duplication as it stood when the draft ran, and that description
   was accurate then. They are a record of a session, not a statement of current
   state, so they are not rewritten; this log is where the correction lives.
10. **The leaf's one in-session reviewer was spent on the page, and it paid.** It
    was given the test, the three edited passages and the contract — that the test
    must fail *after* the root exists, that every count must be re-derivable, and
    that the prose must not contradict the rest of the book. It confirmed contract
    1 independently (re-running the row-2 mutation over both crates, reddening
    exactly this test) and contract 4 (fragment byte-identical to 1424–1441), and
    returned nine findings. Classified:
    - **Valid, mine, fixed here (5).** Three were the *same* stale sentence in
      three places: I wrote that the two tests *make the same two assertions*,
      which was true of the old body and is not true of the new one — the first
      assertion is now the error-text check. It survived into the helper paragraph,
      the front-door paragraph and a `concept-index.md` entry I rewrote, because I
      carried the old adjudication's framing forward instead of re-reading the test
      I had just written. Also: *the two `bail!`s beside it* dropped row 5, which
      is the same class and which my own text thirty lines later counts correctly;
      and my summary of `refuse_token` omitted the `--` separator and empty checks
      the page enumerates precisely elsewhere.
    - **Valid, not this leaf's, externalised (4).** The 558-test baseline is 560
      (`stale-mutation-suite-baseline-k198`, with the derived *reddens 41* → 43),
      and two prose claims in the measurement section — an unreferenced *two of the
      ten arms* and *as a literal* for a call site that binds
      (`ch11-measurement-prose-claims-k199`).
    - **Noise: none.**
11. **The stale-assertion cluster is the finding worth keeping.** All three
    instances shared one cause — editing the prose *around* a rewritten test while
    still holding the old test in mind — and each instance was in a different
    section, so none of them was reachable from the others by a local read. A page
    that adjudicates a test has to be re-read against the test's new bytes as a
    whole, not patched where the fragment sits.
12. **The 558 baseline was not falsified by this leaf and was already wrong.**
    Decision 7's *the 558 total and the 11-failure control are untouched* is
    correct about this change — no test was added or removed — but it should not be
    read as confirming 558. Two earlier leaves' tests had already moved it to 560;
    `k198` owns it.
13. **The gate is green, and the second run's stall is the machine rather than
    this leaf.** `bash scripts/check.sh` passed whole — *all 8 principal checks
    pass*, six books, 0 failing — after the source change and the bulk of the page
    edits. A confirming re-run after the last prose-only edits stalled in the
    driver suite on
    `spawn_failure_names_the_kind_executable_and_config_without_retiring_the_leaf`,
    which had passed in the first run with no slow warning. The cause is outside
    the repository: **24 orphaned `configured-command.sh` processes** are spinning
    on this machine, together holding some 690 CPU-minutes, the oldest with 346
    minutes of its own — they predate this session. They are not killed here,
    because grove launches a session through exactly that template and one of the
    24 is this session's own driver.
    The prose edits made after the green run are covered by the only two checks
    that read Markdown, both re-run and green: `book-check --final --check all`
    over `docs/walkthroughs/grove-loop`, and all 13 tests of
    `crates/grove/tests/reference_navigation.rs`. No link target changed — the two
    `concept-index.md` edits are label text.
