# chapter-nine-refusal-attribution-k153

## Goal

Correct the attribution sentence in
`docs/walkthroughs/grove-loop/09-resolve.md`'s *What the refusals are worth,
measured*: it says both of `reference`'s operator-facing refusals are pinned by
`task_grow`'s tests, and one of the two is pinned from outside this book's
corpus entirely.

## Context

- **The defect.** The section's third bolded paragraph reads *arms 3 and 4 are
  `reference`'s, and `reference` is the mutating verbs' door, so its two refusals
  are pinned by `task_grow`'s tests — chapter 10's block, and the excluded
  `task_grow/tests.rs` at that, which this book cites by name and never
  reproduces.* The two tests the page's own table names for arm 3 are
  `add_under_nonexistent_parent_errors` and `insert_requires_an_existing_target`,
  and both are in `crates/grove-llm/tests/leaf.rs` — lines 432 and 526 — which is
  an integration target of a **different crate** and is not in this book's
  corpus at all. Only arm 4's `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys`
  is in `crates/grove-loop/src/task_grow/tests.rs`.
- **The table is right and the prose is wrong.** The rows name the tests that
  fail under mutation and say nothing about where they live; the sentence after
  the table is what places them. So this is a one-paragraph correction, not a
  re-measurement — but see *Done when*, because the measurement should be redone
  rather than trusted.
- **The same paragraph carries a second sentence that inherits the error**:
  *Chapter 9 owns the code and chapter 10 owns the evidence.* True of arm 4 only.
  The concept index carries it as an entry too —
  `docs/walkthroughs/grove-loop/concept-index.md`, *Chapter 9 owns the code and
  chapter 10 owns the evidence* — and that entry's label needs the same
  narrowing.
- **Chapter 10 already states the corrected division** and does not need editing:
  `10-growing.md#proof-outside-these-pages` says the ambiguity arm is pinned in
  the excluded file, the *no entry matches* arm from `crates/grove-llm/tests/leaf.rs`,
  and that chapter 9's page states it as pinned by `task_grow`'s tests, *which is
  true of one arm of two*. This leaf owes agreement in chapter 9, and the two
  pages must not both carry the adjudication — chapter 10's clause should shorten
  to a cross-reference once chapter 9 is right.
- **Found by `growing-k146`**, by grepping for the two test names rather than by
  reading the page, which is the general lesson its own body records: a page's
  citation of a test name is checked by locating the test, not by recognising the
  name.
- **The corpus is not touched.** The defect is in a book page.

## Done when

- `09-resolve.md`'s *What the refusals are worth, measured* places each of arms 3
  and 4 in the file its tests actually live in, and no sentence in the section
  claims that both of `reference`'s refusals are pinned by `task_grow`'s tests.
- The attribution is **re-derived, not edited from this file**: replace
  `crates/grove-loop/src/task_tree.rs` lines 942–945 and 952 one at a time in a
  copy of the workspace and run `cargo test --no-fail-fast -p grove-loop
  -p grove-llm -p grove` against each, diffing the failure set against an
  unmutated control run of the same copy. `growing-k146`'s *Decisions* records
  the result it obtained and the ten environmental failures a non-jj copy carries.
- The concept-index entry that repeats the claim is narrowed to what survives.
- `10-growing.md`'s clause is shortened to point at chapter 9 rather than
  re-adjudicating, and both pages are checked to agree.
- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  what-the-library-cannot-see --check all` is still valid at 13 files, 4,691
  resolved lines, 5,842 deferred, `final=false`.
- `bash scripts/check.sh` is unchanged — red on `book-check` alone, which is
  every child of `the-walk-k126`'s shape until chapter 21 lands.

## Notes

**Do not widen this into a re-audit of chapter 9.** Its own mutation study was
run and its table stands; what is wrong is one sentence of placement after it.

## Decisions (running log)
