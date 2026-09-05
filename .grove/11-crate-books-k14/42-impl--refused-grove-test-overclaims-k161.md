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
