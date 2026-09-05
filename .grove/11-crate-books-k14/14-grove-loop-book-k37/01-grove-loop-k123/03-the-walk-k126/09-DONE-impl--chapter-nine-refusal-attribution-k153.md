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

1. **The re-derivation contradicted the finding that commissioned this leaf, and
   the table needed correcting too.** Mutating `task_tree.rs` 942–945 and 952 one
   at a time in a workspace copy — each `bail!` replaced by a panicking sentinel,
   `cargo test --no-fail-fast -p grove-loop -p grove-llm -p grove` run against
   each, every failure set diffed against an unmutated control run of the same
   copy — attributes **four** tests to arm 3, not two:
   `add_refuses_a_parent_that_names_nothing_in_the_tree`
   (`crates/grove-loop/src/task_grow/tests.rs` 413, which asserts on the string
   *no entry matches*) and `insert_errors_when_target_missing` (same file, 1,159),
   alongside `add_under_nonexistent_parent_errors` and
   `insert_requires_an_existing_target` (`crates/grove-llm/tests/leaf.rs` 432 and
   526). Arm 4 is held by exactly one, `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys`
   (`task_grow/tests.rs` 268). The two failure sets are disjoint and each is
   non-empty against a control clean of both, which is what attributes them.
   So the page's arm-3 *row* was incomplete as well as its prose: this leaf
   corrects both, which is the measurement's consequence and not a widening.
2. **`growing-k146`'s diagnosis inherited the page's incomplete row.** It grepped
   for the two test names the table already carried and found them in another
   crate — true, and not the whole set. Locating the names a page carries says
   where *those* tests live and nothing about whether they are all of them; only
   the mutation enumerates. The page now states this in front of the reader.
3. **A panic-sentinel mutation is strictly stronger than a message swap, but it
   explains only one of the two missed observers.** Enumerating the four
   assertions rather than assuming them: three of arm 3's observers assert on the
   substring *no entry matches* — including
   `add_refuses_a_parent_that_names_nothing_in_the_tree`, one of the two the
   earlier row missed — and only `insert_errors_when_target_missing` asserts a
   bare `is_err()` and would survive a reworded `bail!`. So mutation strength
   accounts for that one test and nothing accounts for the other; the honest
   statement on the page is that starting from names a row already carries cannot
   produce a name it does not, and no reason beyond that is claimed. This
   correction was itself caught by enumerating a uniqueness claim the first draft
   of the page asserted without counting.
4. **Chapter 10's clause was false, not merely long.** It said the *no entry
   matches* arm is **not** pinned in the excluded file; two of its four observers
   are. Shortening it to a cross-reference, which is what this leaf's *Done when*
   asks for, removes the false adjudication rather than merely relocating it.
5. **The environmental baseline of a non-jj workspace copy is twelve failures
   here, not ten**: eleven in `crates/grove-loop/tests/prompt.rs` and
   `every_repository_markdown_reference_resolves` in `crates/grove/tests/`. The
   copy also needs `CONTEXT.md`, `CONTEXT-MAP.md`, `README.md`, `LICENSE` and
   `release.toml` beside `crates/`, or `composition_guidance.rs` fails to compile
   and the run reports nothing at all.
6. **Corrected the parent brief's own *Found while drafting* entry rather than
   leaving it.** It stated k146's diagnosis as settled, and every remaining child
   of `grove-loop-k123` reads it as guidance. The entry now carries the
   re-derived attribution and three lessons in place of one; the second — that
   locating the names a page carries cannot produce a name it does not — is the
   one k146's own method lacked.
