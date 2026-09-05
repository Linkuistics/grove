# a-grove-begins-k155

## Goal

Draft chapter 11 of the `grove-loop` book, *A grove begins*, over
`tree_lifecycle.rs`'s three `never-mistaken-for-finished` blocks — `332-489`,
`1013-1076` and `1077-1466`, 612 lines — and prove the prefix through slice
`never-mistaken-for-finished`.

## Context

- Child 1 of 4 of `no-word-for-k127`. The chapter's section in
  `docs/specs/grove-loop-book-structure.md` is *11 · A grove begins*; the rule is
  **a fresh grove starts with one live leaf, so it is never mistaken for
  finished**.
- The three blocks are `grove-beginning` (`332-489`, production: `root_init`,
  `default_root_slug`, `initialize_grove`, `RootShape`, `root_shape`),
  `body-helpers` (`1013-1076`: `grove_name`, `root_brief_body`,
  `append_brief_suffix_in_file`) and `root-init-tests` (`1077-1466`), which is the
  test module's opening **plus** the `root-init` section — the shared test support
  the next two chapters' tests use, then thirteen `#[test]` functions.
- **The prose obligation is *supply the claim*** over the whole of `1077-1466`,
  at 15% prose: per reproduced test, the property it establishes **and what would
  have to be true for it to pass while the property was broken**.
- Two early-use rows are owned by this slice and both are `pending`:
  `verbs::root_init` (first use `01-orientation.md#the-cast`) and
  `tree_lifecycle::initialize_grove` (first use
  `10-growing.md#the-prediction-held-to-account`). Both move to `explained`.
  **The manifest's rows are a floor** — enumerate this chapter's own bytes for
  symbols a later chapter owns, in both the Rust-path and the hyphenated-verb
  spelling.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  never-mistaken-for-finished --check all` is valid: 13 files, 5,303 resolved
  lines, 5,230 deferred, `final=false`.
- `11-a-grove-begins.md` exists, `README.md` and chapter 10's navigation are
  updated, and the three `never-mistaken-for-finished` ownership rows read
  `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf.

## Decisions (running log)

1. **Decomposed `no-word-for-k127` one child per chapter and landed chapter 11**,
   which is the shape both `the-grammar-k125` and `the-walk-k126` took; this node's
   brief carries the table and the reasoning.
2. **Adjudicated the `llm_cli` stale address on the page rather than cutting a
   leaf.** Chapter 6 set that precedent for the same defect, and the behaviour the
   comment claims is correct — only the address is stale. Line 42 remains chapter
   14's to adjudicate.
3. **Cut two source-fix leaves rather than adjudicating alone**, because both are
   defects a page cannot repair by describing: `default-root-slug-two-spellings-k159`
   (two independent `"plan"` literals, agreement unheld) and
   `welded-grove-name-summary-k160` (two summaries in one doc paragraph). Both are
   inserted ahead of `architecture-residue-k75` under `crate-books-k14` and both are
   deferred behind the book, with `k151`, `k152` and `k154`, because these bytes are
   now reproduced on a finished page.
4. **Added three early-use floor rows the manifest does not carry** —
   `DEFAULT_ROOT_SLUG`, `transition_to_current` and `CurrentTransition`, all owned by
   `the-tree-deletes-itself`. Found by enumerating this chapter's reproduced bytes
   against every symbol the crate defines, mapped to its owning block, rather than by
   reading the manifest's rows; the manifest's two rows for this slice both moved to
   `explained`.
5. **Stated that `a_refused_grove_leaves_no_root_behind` does not hold the
   unwinding**, against the structure brief's pairing of it with
   `root_init_creates_the_whole_grove_through_one_store_operation`. It is
   mechanically the same test as `root_init_rejects_a_bad_slug_without_leaving_a_grove_behind`:
   the helper validates the slug before opening the vacancy, so no store operation
   is entered. The property is still true — it is `TreeVacancy::initialize`'s
   contract, in another crate — and the page says so on the store's word rather than
   claiming a proof this block does not contain. **The structure brief is not
   corrected here**: its pairing is a statement about which tests carry the
   chapter's rule, and the page's job was to say what each actually pins.
6. **Spent the leaf's one in-session reviewer** on the chapter's factual claims —
   counts, call-site enumerations, line citations and the mutation table — because
   the chapter makes an unusual number of claims the validator cannot check.
7. **Cut a third source leaf, `refused-grove-test-overclaims-k161`, as a decision
   rather than a fix.** The two outcomes — earn the unwinding claim with a real
   test, or narrow the comment and reconcile the structure brief — differ in what
   the crate ends up proving, so folding it into either doc-comment leaf beside it
   would have pre-decided it.
8. **Integrated the reviewer's fifteen findings as fourteen corrections and one
   piece of noise.** Every one was re-derived against the source before being
   applied, because the page had been edited nine times while the reviewer read it
   — and that caught the noise: its finding about *`root_init`'s only caller* cited
   a sentence I had already replaced. The rest were valid: one structural
   mis-attribution that produced three wrong sentences, two inverted claims, four
   undercounts, and six loose phrasings. **No second reviewer was materialised** —
   the fixes are factual corrections each independently checked against the crate,
   not a substantive redesign, so the allowance is spent and the doubt is routed
   into this node's brief for chapters 12 to 14 instead.

