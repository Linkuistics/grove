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
  `BLOCKS` (33 top-level ownership ranges) and `EARLY_USES` in
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

## Decisions (running log)

- **The derived corpus reaches the core as snapshot data, not as a core
  filesystem walk.** The specification's *The corpus rule and its witness* says
  the CLI enumerates and passes the derived set to the core alongside the
  manifest, so `BookSnapshot` gains a `derived_corpus` field and `check_inventory`
  compares it against the declared `[[root]]` paths. The core keeps performing no
  discovery of its own, which is the property `validator-structure-k21`
  established and this leaf must not spend.
- **Derivation itself is a library module, not CLI-private code.** `src/corpus.rs`
  owns pattern matching *and* the walk, because the test fixture must derive its
  own corpus from the real repository rather than restate the answer. A fixture
  that hand-wrote the derived set would make every core test agree with itself by
  construction — the exact self-witnessing this leaf exists to remove.
- **Manifest corpus types are named `SourceRoot`, `OwnershipBlock`, `EarlyUse`.**
  `parser::Root` and the removed `validator::Block` already hold the short names,
  and the two pairs mean different things: a `parser::Root` is a directive read
  off a page, a `manifest::SourceRoot` is the declaration it is checked against.
- **The exception inventory binds per book, and only for books that exist.** The
  table carries a `grove-loop` row for a book this campaign has not written yet,
  so an unconditional set equality fails on a forward commitment rather than on
  a disagreement. The comparison is therefore filtered to existing book roots,
  and a second test asserts the table still carries at least one row for a book
  that does not exist — without it, deleting the future rows along with the
  books that were to carry them would pass silently.
- **Both halves of the successor control were seen to fail before the bridge
  was deleted.** `check_derived_corpus` disabled turns
  `a_file_the_corpus_rule_reaches_and_the_manifest_does_not_declare_is_an_f006`
  and `a_declared_root_the_corpus_rule_does_not_reach_is_an_f006` red; an
  exclusion smuggled into the relocated book's manifest turns
  `every_books_corpus_exceptions_are_exactly_the_specifications_inventory` red,
  naming the smuggled path. The base-pattern, class and partition rules were
  each disabled in turn and their tests seen red too.
- **The second synthetic book moved to `--check all`.** `second_book.rs` said in
  its own module doc that it ran `--check markdown` only because the fragment
  path still read compiled constants. Leaving it there would have left the whole
  fragment half proved against exactly one book — the failure mode this leaf's
  brief names — so the widget fixture grew a source root, two ownership blocks,
  an excluded inline test module and literal fragments, and its control is a
  one-byte source mutation that must surface `F008`.
- **`scripts/check.sh` reports "no book roots found" as a failure.** The glob
  comes back literal when `docs/walkthroughs/` is empty, and a loop that skipped
  it would print a green umbrella having checked nothing. Both arms were
  observed: a `probe-book` root the script had never been told about failed the
  gate, and the function run in an empty tree returned 1.
- **The comparator's unit test no longer names a root.** Three literals survived
  the corpus deletion inside `#[cfg(test)]` — two root ids and a source path in
  a synthetic diagnostic. They are not a declared row, but they are the same
  book-specific knowledge in the same directory, and the comparator sorts by
  *position* in the declared root order, so the fixture only needs two roots
  apart. It now reads both off the loaded manifest.
- **Three accessors I wrote and never called were removed** — `Pattern::directory`,
  `Corpus::included` and `Manifest::subject`. A published accessor nothing needs
  is interface without a reason, and the corpus surface is small enough that the
  next consumer can add exactly the one it wants.
- **The specification's *Assurance* attribution was corrected.** It assigned all
  three discovery obligations to `book-assurance-surface-k39`; the first — the
  `check.sh` gate — is this leaf's, and leaving it there invited k39 either to
  redo it or to read it as already done and skip its own two.
- **`scripts/check.sh` enumerates book *directories*, not manifests.** The first
  version globbed `docs/walkthroughs/*/walkthrough.toml`, which would let a book
  whose manifest was deleted leave the gate without a word — the one failure a
  discovery loop exists to make impossible. A manifest-less book root was
  created and seen to fail the gate as `U002` before the loop was believed.
- **Finding, externalised as `corpus-subject-anchor-k50`.** The base-pattern rule
  fixes the corpus floor to `[book].subject`, but nothing fixes `subject`. A book
  can point it at a subdirectory of its own crate, drop every root outside it,
  and validate green while covering a fraction of what it claims. That is the
  remaining move against the completeness claim this leaf is the gate for; it
  needs the same declare-it-twice mechanism the exception inventory already uses,
  and it is sequenced ahead of the five unwritten books so they are authored
  against it rather than retrofitted.
