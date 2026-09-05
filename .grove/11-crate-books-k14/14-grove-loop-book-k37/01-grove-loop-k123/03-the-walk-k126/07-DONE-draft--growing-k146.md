# growing-k146

## Goal

Draft chapter 10 of the `grove-loop` book — *Growing: `leaf-add` and
`leaf-insert`*, `docs/walkthroughs/grove-loop/10-growing.md` — and prove the
prefix through slice `what-the-library-cannot-see`, which completes Part II.

## Context

- The last of `the-walk-k126`'s six chapter children. Its **one** ownership block is
  `growing-the-tree` (`task_grow.rs` 1–518), the whole file, 518 lines.
- The structure brief's section is *10 · Growing: `leaf-add` and `leaf-insert`*.
  The rule is **the preconditions the library cannot see, checked against the
  same snapshot the operation then plans from**. The brief calls this the spine
  at its most explicit, because the module header is a list of exactly it — the
  reference grammar, the preconditions, the task-file template, and the
  cross-reference lint, each with its own paragraph saying why it could not move.
- **The chapter carries key prediction.** Because the template's bytes embed the
  key, grove predicts the allocation and checks it against the store's report.
  That is the carried example's row 10 and its observable end: a sibling at the
  next position, its key predicted and checked.
- **This chapter's proof is entirely outside its own pages, and it is the only
  chapter of the book of which that is true — and the chapter says so.**
  `src/task_grow/tests.rs` (1,680 lines) is the book's one declared
  `[[corpus.exclude]]`, so its tests are cited by name and never reproduced. The
  four the brief names are
  `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys`,
  `add_preserves_a_gap_a_hand_edit_left_rather_than_filling_it`,
  `a_refused_run_does_not_consume_positions_or_keys` and
  `insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved`.
  Verify each name against the file before citing it; a cited test that has been
  renamed is a claim about the subject that the subject does not bear out.
- **The block is wholly production at 49% prose and takes *do not restate*.**
  There is no inline test to supply a claim for — the two-line `#[cfg(test)] mod`
  declaration names the external file instead.
- **The early-use ledger is a floor**: enumerate this chapter's reproduced bytes
  for later-owned symbols named or exercised, and add the rows they owe.
- `grove-does-not-stage-its-own-renames` and `entries-are-never-removed` are
  among the records the structure brief names for the chapters that keep them.
  Under the link contract they are **named in prose and never cited**.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  what-the-library-cannot-see --check all` is valid: 13 files, 4,691 resolved
  lines, 5,842 deferred, `final=false`. That is `the-walk-k126`'s own
  `Done when` and this leaf discharges it.
- Chapter 10 exists, `README.md`'s contents entry and chapter 9's two navigation
  lines are updated, and the `growing-the-tree` ownership row reads `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

**This leaf's last act is not a `leaf-add`.** The pipeline's next stage is cut by
`grove-loop-k123`'s last child, `what-could-not-move-k130`, under the node
brief's own `Done when`; this leaf closes a part, not the document.

## Decisions (running log)

1. **Ran the mutation study over all eleven of the block's control-stopping arms
   rather than reasoning about them.** The parent brief names chapters 9 and 10 as
   owning functions of the same shape and says a coverage claim is worth exactly
   the re-run. Result: four observed, seven not, and the seven split by *who could
   produce the condition* — one unreachable through the verb by the CLI's arity,
   five saying *the library broke its own contract*, one a silent skip. That split
   is a fact about this block (classify, then call) and not a rule; `leaf_entry`'s
   five unobserved arms were ordinary operator-facing refusals. **Environmental
   baseline:** a workspace copy that is not a jj repository fails ten
   `crates/grove-loop/tests/prompt.rs` tests before any mutation, so every arm was
   diffed against an unmutated control run of the same copy — 42 targets, 616
   passed, 10 failed. Copy included `.cargo/` (its `GROVE_SIGNAL_FILE` guard),
   `testing/`, `plugins/` and `CHANGELOG.md`. One target was interrupted during
   arm 11's run and was re-run against that arm alone; it passes, so all eleven
   arms are measured over all forty-two targets.
2. **Cut `chapter-nine-refusal-attribution-k153` rather than editing chapter 9
   inline.** `09-resolve.md` says both of `reference`'s operator-facing refusals
   are pinned by `task_grow`'s tests; `add_under_nonexistent_parent_errors` and
   `insert_requires_an_existing_target` are in `crates/grove-llm/tests/leaf.rs`,
   outside this book's corpus. Chapter 10 states the corrected division and k153
   shortens it to a cross-reference once chapter 9 is right. Found by grepping for
   the two names rather than by reading the page.
3. **Cut `grow-header-stale-helper-k154` under `crate-books-k14`, inserted before
   `architecture-residue-k75`, and deferred it behind the book.** Two defects in
   the module header: `leaf_slug`, a shared helper that has never existed, and a
   `#` heading with no body whose paragraphs sit under the next heading. Both are
   adjudicated on the page under the freeze; the fix must stay inside the file's
   518 lines or every fragment range in chapter 10 moves.
4. **Enumerated the early-use ledger rather than trusting this file's Context.**
   One row owned by this slice closed (`task_grow::allocated` → `explained`) and
   three floor rows were added — `verbs::leaf_add`, `verbs::leaf_insert` (both
   named in the reproduced doc comments' hyphenated spelling, which is chapter 8's
   precedent) and `tree_lifecycle::initialize_grove`, the only later-owned Rust
   path the block names. Both spellings were swept, as the parent brief requires.
5. **Cut no leaf for the structure brief's prose percentage.** Its table says
   `task_grow.rs` is 49%; the count that reproduces every other cell of that table
   exactly — leading `//`, `///` and `//!` lines over total lines — gives 251/518
   = 48%. The obligation band is 41–73% either way, so no page claim changes and
   nothing is reconciled. Recorded here rather than adjudicated on the page,
   because no page states a prose percentage.
6. **Verified `NoOccupantAtOrdinal`'s *all three of its messages* by counting the
   library's `Display` arms**, not by reading the source comment: three match arms
   after a shared preamble, so the count stands. Note for whoever owns
   `ordinal-fs-tree`: that crate's own `ops.rs` comment says *one refusal and two
   messages* and is stale by one. Out of this book's corpus, so not adjudicated
   here and no leaf cut for it from this session.
7. **Adjudicated the fourth `llm_cli` reference as *not* the same defect.** The
   parent brief asked this chapter for the same one-clause adjudication chapter 6
   made. The occurrence in the excluded test file is a dated provenance note, and
   at `loop-crate-verbs-k21`'s parent `src/llm_cli.rs` did exist and did carry a
   `mod tests` holding the read-count assertions that now sit at the end of that
   file. Three occurrences name a module nothing wears now; this one names a
   module by the name it wore on the date it gives.
8. **Also corrected a citation in `docs/specs/grove-loop-book-structure.md` on the
   page rather than in the brief.** `a_refused_run_does_not_consume_positions_or_keys`
   refuses at the slug boundary before `leaf_add` is entered, so it pins the front
   door's ordering rather than the atomic run; the run's claim is held by five
   other tests in the same section. The page names both. No leaf cut: the brief's
   sentence is true of the test, and what is wrong is the inference the outcome
   table draws from it — which the page now states.

