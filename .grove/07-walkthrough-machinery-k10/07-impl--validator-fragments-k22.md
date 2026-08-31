# validator-fragments-k22

## Goal

Remove the validator's compiled-in knowledge of one book's *corpus*: source
roots, ownership blocks and early-use rows all come from the named book. Then
gate every book root under `docs/walkthroughs/` from `scripts/check.sh`.

This is the leaf that puts the fragment half of `plan-k1`'s decision 4 to bed —
the half whose scope widened when decision 1 reversed to complete source-exact
coverage, and the reason the validator sits on the campaign's critical path.

## Context

- The constants in scope: `ROOTS` (17 source paths with exact line counts),
  `BLOCKS` (34 top-level ownership ranges) and `EARLY_USES` in
  `crates/book-validation/src/`. Their per-book form is whatever
  `walkthrough-books-spec-k20` settled.
- `validator-structure-k21` has already moved the structure metadata —
  `SLICE_ORDER`, `PAGE_BY_OWNER`, `SOURCE_INDEX` — to per-book data, *including*
  its readers inside `src/ledger.rs`. The line between the two leaves is which
  constants each owns, not `book-check`'s `--check` modes: both sets are read on
  the fragment path, and both leaves must leave `--check all` green.
- `scripts/check.sh` today gates the book only through `cargo test --locked
  --workspace`, which runs `tests/corpus_validation.rs` against the one
  compiled-in book. "Gates every book" means enumerating the book roots that
  exist and running final validation over each, so that adding a sixth book adds
  a gate without editing the script.
- The line counts are exact and load-bearing: the root brief's freeze exists
  because a ledger holds a line count per source root, and an inline source fix
  shifts every line below it. Reading counts from data does not weaken that — it
  moves where the count is written down.

## Done when

- Final validation of the relocated book passes with no compiled-in root,
  block or early-use row anywhere in `crates/book-validation/src/`, established
  by enumerating and classifying every literal in that directory.
- `scripts/check.sh` runs final validation over every book root under
  `docs/walkthroughs/` and would pick up a new one with no edit to the script.
  Prove the discovery, not just the pass: a deliberately broken second book root
  must make the script fail, seen to fail, before a green run counts as evidence.
- The corpus control has a **successor**, not just a deletion. The specification's
  *The corpus rule and its witness* now carries a normative corpus exception
  inventory and requires a repository test comparing it against the
  `[[corpus.add]]` and `[[corpus.exclude]]` entries of every manifest under
  `docs/walkthroughs/`, per book, path for path and class for class. That test
  exists and has been **seen to fail** — against a manifest carrying an exception
  the table does not — before `compiled_corpus_copy_matches_the_book_ledger_tables`
  is deleted. So has the corpus-derivation check, against a manifest missing a
  real root, which is what that bridge's doc comment already promises.
- The two schema rules that make derivation an external witness are implemented
  and each seen to fail: `[corpus] include` must contain both base patterns
  derived from `[book].subject`, and every `add`/`exclude` `class` is one of the
  closed values, with an `inline-test-module` path whose file name is not
  `tests.rs` refused. Both are `U002`.
- `bash scripts/check.sh` passes.

## Notes

**Deleting the bridge is the whole reason this leaf is dangerous.**
`walkthrough-books-spec-k47` accepted a review finding that filesystem
derivation alone is *not* an external corpus witness — it proves the declared
patterns matched, never that the author declared the right ones, and it cannot
judge an exception. The successor control is the pair above, and a session that
deletes the bridge without both leaves the campaign's completeness claim resting
on each book's own account of itself.

**This leaf is the campaign's gate.** Every book after it is proved by what lands
here, and a validator that is green over one book because it still knows that
book is a validator that will silently accept the next five. Spend the evidence
accordingly.

**Do not touch `crates/ordinal-fs-tree/`.** If a line count in the ledger
disagrees with the source, the ledger is what you have measured wrongly, or the
source has drifted and the campaign's freeze has already been broken — either way
it is a finding to report and, if it is a real source defect, its own leaf under
the root brief's cross-book rule.
