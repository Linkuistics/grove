# what-could-not-move-k130

## Goal

Draft chapter 21 of the `grove-loop` book — *What could not move*, slice
`assembly` — close both indexes, and take the book to green `--final` validation
and a green `bash scripts/check.sh`.

## Context

- Draft stage, child 7 of 7 of `grove-loop-k123`, and the only one that owns no
  source. The structure brief is `docs/specs/grove-loop-book-structure.md`, and
  this chapter is its *21 · What could not move*.
- **It applies the stated outcome's three questions to each of the twenty
  source-owning chapters in turn, and answers them for the crate as a whole.**
  The questions, with the cost each carries and the test that pins it, are the
  brief's *The stated outcome: the what-could-not-move test*: the names on the
  way in, the preconditions on the way through, and the policy on the way out.
- **Every sentence here is a claim about another page, and no validator reads
  one.** `book-check` proves that this page owns no source and that its links
  resolve; it cannot see whether chapter 13 says what this chapter says it says.
  That is why the brief calls this the page a reviewer should be spent on, and
  why the leaf-wide in-session review allowance belongs here rather than earlier.
  Check each claim against the chapter it names, not against this leaf's body.
- The three parts of the outcome are *meaning*, *timing* and *choice*, and each
  has a named test inside the corpus:
  `pick_refuses_a_species_mismatch_at_a_task_shaped_name` and the conformance
  kit for the first; `prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address`,
  `a_refused_run_does_not_consume_positions_or_keys` and
  `one_process_creating_and_reading_a_grove_never_waits_on_itself` for the
  second; `the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one`,
  `the_runtime_facts_restate_no_rule_the_skill_owns` and
  `the_library_imposes_only_libc` for the third.
- **The coverage obligation is the thirty-one residue markers.** The brief's
  *What this book makes redundant* maps every `docs/ARCHITECTURE.md` marker
  naming this crate to a chapter. This chapter is where that map is checked
  against the book that was actually written: a marker whose chapter did not
  cover it is a finding now, not `architecture-residue-k75`'s surprise later. The
  book neither cites nor edits that document.
- The concept index is curated across all twenty-one pages here, and the source
  index's early-use ledger must show every row `explained`.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  is valid: 13 files, 10,533 resolved lines, 0 deferred, `final=true`.
- `bash scripts/check.sh` passes — the whole script, not one check — and this is
  the first child of which that is true.
- `every_repository_markdown_reference_resolves`,
  `every_book_root_has_a_documentation_ownership_row`,
  `every_books_subject_is_exactly_the_specifications_inventory` and
  `every_books_corpus_exceptions_are_exactly_the_specifications_inventory` pass.
- **Last act:** `grove-llm leaf-add grove-loop-book-k37 grove-loop --kind
  copy-edit`, unless a live later sibling under `grove-loop-book-k37` already
  holds that stage — in which case cut nothing.

## Notes

**The corpus is frozen.** A defect found here becomes its own leaf.

## Decisions (running log)
