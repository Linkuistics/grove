# paths-k142

## Goal

Draft chapter 6 of the `grove-loop` book — *Paths, and addressing*,
`docs/walkthroughs/grove-loop/06-paths.md` — and prove the prefix through slice
`paths-are-built-here`.

## Context

- The second of `the-walk-k126`'s six chapter children. Its two ownership blocks are
  `paths-and-addressing` (`task_tree.rs` 291–570) and `path-composition-tests`
  (1,016–1,105) — 370 lines.
- The structure brief's section is *6 · Paths, and addressing*. The rule is that
  **the library returns no paths, so grove builds them — in exactly one place**:
  `entry_path` and why it is safe without a check; `Target`, `target` and
  `unreachable_by_any_walk`; `addressable_key` and `interrupted_promotion`;
  `next_key`, `live_leaf`, `entry_outcome`.
- **The chapter carries why nothing canonicalises for output.** On macOS `/var`
  and `/private/var` name the same inode, so canonicalising would make the mere
  presence of a lock rewrite every path grove prints. Chapter 5 reproduced the
  header paragraph that says so and pointed here; this is where the account sits,
  beside the function.
- The test block is the one the source itself labels *the path-taking
  compositions, which are the tests' alone* and takes *supply the claim*: per
  reproduced test, the property it establishes **and** what would have to be true
  for it to pass while the property was broken. The production half is 42% prose
  and takes *do not restate*.
- **This chapter closes the manifest's `entry_path` early-use row**, whose owner
  is `paths-are-built-here`: it moves from `pending` to `explained` with this
  slice, because `owner_is_complete` in `crates/book-validation/src/ledger.rs`
  computes the expected status from the scope and leaving it `pending` is `F009`.
  Chapter 1's cast row owned by `one-spelling-of-grove` moved to `explained` in
  chapter 5 for the same reason.
- **The early-use ledger is a floor.** Enumerate this chapter's own reproduced
  bytes for symbols a later chapter owns — named *or* exercised — and add a row
  per symbol not already covered, rather than treating the manifest's rows as the
  set. `floor-rows-chapter-two-k138` is the worked precedent.
- The chapter's carried-example row is the brief's row 6: from an entry of a
  snapshot to one absolute path, built in exactly one place.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  paths-are-built-here --check all` is valid: 13 files, 2,810 resolved lines,
  7,723 deferred, `final=false`.
- Chapter 6 exists, `README.md`'s contents entry and chapter 5's two navigation
  lines are updated, and both `paths-are-built-here` ownership rows read
  `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

1. **Sixteen fragments over the two blocks: nine over the 280-line production
   run and five over the 90-line test-module opening, under two composites.**
   The production block is one fragment per item — `entry_path`, `Target`,
   `target`, `unreachable_by_any_walk`, `addressable_key`,
   `interrupted_promotion`, `next_key`, `live_leaf`, `entry_outcome` — because
   every item is a separable decision with its own doc-comment argument, unlike
   chapter 5's module header, which needed splitting by claim. The test block
   splits at the module opening, the compositions, `a_kind` with the two
   mid-module `use` lines, `brief_chain_at`, and the four fixtures.
2. **`path-composition-tests` carries no `#[test]`, and the page says so rather
   than discharging the per-test obligation silently.** Enumerated across all
   thirty-nine ownership blocks: sixteen carry `test` in their id, and this is
   the only one of the sixteen with zero tests — the other fifteen hold between
   one and thirty-two. The structure brief lists chapter 6 under *supply the
   claim — every inline test block*; the arithmetic is right, since those ninety
   lines are part of the 3,984, but the instruction has no instances here and a
   reviewer checking the list cannot tell **vacuous** from **omitted**. Recorded
   as a third Part II defect on `pick-test-count-k147`, which already holds the
   brief's other two and runs next.
3. **A source defect externalised to `canonicalisation-sites-k149`**, inserted
   under `crate-books-k14` at position 36, ahead of `architecture-residue-k75`
   and beside `grove-root-join-clauses-k148`. The module header's
   *Canonicalisation appears once, in `leaf_entry`* is false: six production
   `canonicalize` calls sit in this file, three in `target` and three in
   `leaf_entry`, and `target`'s own doc comment says *exactly as `leaf_entry`
   does*. Chapter 6 owns the counterexample, so the adjudication sits here; the
   fix is a later leaf's because the corpus is frozen. The `leaf_entry` early-use
   row's minimum statement carried the same false claim and was corrected in this
   commit — it is a floor row rather than a manifest row, so no contract moved.
4. **Four early-use rows beyond the manifest's, and two of them came from the
   reviewer.** `task_grow::allocated` (chapter 10) is named at line 524;
   `pick_in` and `select_in` (chapter 7) are exercised by the compositions.
   `tree_lifecycle::leaf_decompose` and `tree_lifecycle::leaf_retire` were
   missed by the author's sweep and found by the reviewer, because the bytes name
   them **hyphenated** — `leaf-retire`, `leaf-decompose` — not as Rust paths.
   Chapter 5's `leaf_prune` row was written from an underscore spelling, so the
   precedent did not point at the gap. Written into the node brief for every
   remaining child: enumerate both spellings.
5. **The leaf's one in-session reviewer was spent on the page's factual claims
   and the early-use floor**, the two things `book-check` cannot reach — it
   proves reconstruction and ledger structure and never whether a sentence about
   the source is true. Twenty findings came back. **Every one was re-verified
   against the source here before it was acted on, and every one that survived
   was valid**; none was noise, an unclear contract, or a visible trade-off. Six
   had already been found and fixed by the author's own pass while the reviewer
   ran, and the reviewer said so. No second reviewer was materialised and no
   `review-*` leaf was cut: every fix was a local prose correction or a ledger
   row against source re-read in this session.

   The ones worth recording are the five that were not merely arithmetic:

   - **The `Target` collision was attributed to the wrong crate.** The page said
     `task_grow` declares a `Target` of its own with variants `Root` and `Key`.
     It does not: `task_grow.rs` line 54 imports `ordinal_fs_tree::Target`, and
     so does `tree_lifecycle.rs` at line 50. Three consequences followed — the
     collision is with the library rather than within the crate, chapter 10 owns
     no such type, and the page had implied an early-use row that is not owed.
   - **`chapter 5 declined to repeat `lib.rs`'s one-spelling claim` was false in
     both halves.** Chapter 5 never mentions the claim; chapter 1 does, at
     `01-orientation.md` line 307, and **states it as true**. The page now says
     that, which is also what `grove-root-join-clauses-k148` is for.
   - **A counterfactual that could not happen.** The page said
     `decomposing_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed`
     would still pass if `promote` reached the live twin first. It would not: the
     test calls `unwrap_err()`, so a successful promotion panics. The correct
     reading is that the message assertion separates Grove's sentence from the
     library's and the tree assertion separates a refusal from a partial
     mutation.
   - **The sentence's fifth assertion, in the excluded file.** *Two entries in
     this tree carry key N* is asserted by five tests, not four:
     `insert_refuses_a_target_whose_key_names_two_entries` is `leaf-insert`'s, in
     `src/task_grow/tests.rs`, which chapter 10 may cite and not reproduce. The
     page had enumerated only `tree_lifecycle.rs`.
   - **The wrong model file, repeated in the page's own voice.**
     `addressable_key`'s comment attributes `by_key`'s unmodelled tie-break to
     `structure.als`; the library's own `by_key` doc says the tie-break is *the
     one reading behaviour no model checks* and that `operations.qnt` picks the
     least internal id, and `operations.qnt` is where the note lives —
     `structure.als`'s duplicate-key content is an admitted witness. The
     substance holds and the attribution does not, so the page now attributes it
     to the comment instead of asserting it.

   Four of the remaining findings were counts written from the shape of the
   argument — *nineteen lines*, *four clauses*, *eleven lines each*, *every test
   uses them* — which is the class the parent brief warns of, met four more
   times in one page. Each was replaced by an enumeration.
6. **`scripts/check.sh` is red on `book-check --final` over this book alone**,
   which is every child of `the-walk-k126`'s shape until chapter 21 lands. The
   failure is `M101` for chapters 7 to 21; the other seven checks are green and
   the other five books are green. Chapter 6 has left the `M101` list.
