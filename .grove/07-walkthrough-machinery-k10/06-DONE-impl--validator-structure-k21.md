# validator-structure-k21

## Goal

Remove the validator's compiled-in knowledge of one book's *structure*: page
inventory, page-to-slice mapping, canonical slice order and the `--through`
token set all come from the book directory named by `--book`.

## Context

- The design this implements is `walkthrough-books-spec-k20`'s. Read that spec
  and its ADR first; if what you find there does not answer a question this leaf
  hits, that is a gap in the spec, not a licence to invent an answer.
- The constants in scope: `SLICE_ORDER` and `PAGE_BY_OWNER`, plus
  `SOURCE_INDEX`; and the hard-coded slice list in `src/cli.rs`'s `--through`
  `value_parser`, which today rejects any token outside the ordinal book's seven
  authorable slices. `src/cli.rs`'s `about` and `after_help` name one book by
  path and must stop doing so.
- Out of scope, and left exactly as they are for the next leaf: `ROOTS`,
  `BLOCKS`, `EARLY_USES`.
- **The split is by data, and `walkthroughs-k3` decision 4 is wrong about
  where.** It claimed the compiled-in ledger divides along `book-check`'s own
  `--check markdown` / `--check fragments` seam. It does not:
  `crate::ledger::check` runs only under `Fragments` or `All`
  (`crates/book-validation/src/validator.rs`), and it is `src/ledger.rs` that
  defines `PAGE_BY_OWNER` and `SOURCE_INDEX` and imports `SLICE_ORDER`;
  `src/markdown.rs` reads `SLICE_ORDER` as well. Every constant this leaf touches
  is read on the fragment path too. The real seam is **shared structure
  metadata** — this leaf — against **per-book corpus data** —
  `validator-fragments-k22`. What lets each land independently is that the two
  constant sets have disjoint *definitions*, not disjoint readers.
- **So `--check fragments` is this leaf's problem as much as `--check
  markdown`.** Re-derive the consumer set rather than trusting this note; a leaf
  that proved only the Markdown mode has proved half of what it changed.

## Done when

- `book-check --book docs/walkthroughs/ordinal-fs-tree --final --check all` is
  green, and so is every scoped `--through` invocation the crate's tests make.
- No page filename, page identifier, slice token or slice ordering appears as a
  literal in `crates/book-validation/src/`, established by enumerating every
  candidate token in that directory and classifying each — not by sweeping a
  pattern list.
- `--through` accepts the slices the named book declares and refuses tokens it
  does not, with a diagnostic that names the book.
- `bash scripts/check.sh` passes.

## Notes

**The diagnostic contract is a tested surface.** `tests/diagnostic_contract.rs`
and the `F0nn` / `U0nn` codes in the book spec are part of what the crate
promises. Codes that stop having a subject, or that gain one, are a change to
that contract and belong in the commit that makes it — not left for a later
session to notice.

**A test fixture that hard-codes the ordinal book is the same defect as a
constant.** The generalisation is not done if the suite can only be run against
one book; a second, synthetic book fixture is the cheapest proof that it is.

## Decisions (running log)

- **The manifest is authored complete, and only its structure half is
  consumed.** `docs/walkthroughs/ordinal-fs-tree/walkthrough.toml` carries every
  group `docs/specs/walkthrough-books.md` requires — `[book]`, `[corpus]`,
  `[[page]]`, `[[root]]`, `[[block]]`, `[[early-use]]`, `[guide]` — because a
  manifest missing required groups is a non-conformant artifact, not a
  half-finished one. The loader validates the structure groups and reads
  `[[block]] owner`; the corpus groups are parsed for shape and dropped, with
  `validator-fragments-k22` named at the definition site as their reader.
  `[[block]]` had to be present in this leaf regardless: the specification
  derives the scoped domain from *which slices own blocks*, so a manifest
  without blocks could not answer `--through` at all.
- **The manifest was generated from the compiled constants, not transcribed.** A
  throwaway test emitted `[[root]]`, `[[block]]` and `[[early-use]]` from
  `ROOTS`, `BLOCKS` and `EARLY_USES` and the page rows from `markdown::CHAPTERS`
  / `FIXED`, so the first run of the new reader compared data that provably
  started identical. Any diff was then a bug in the reader rather than a typo in
  the data.
- **Two interim bridge tests defend the interval.** For as long as the corpus is
  stated twice — compiled and in the manifest — nothing else compares the two
  statements, so `the_manifest_restates_the_compiled_corpus_exactly`
  (`src/validator.rs`) and `the_manifest_restates_the_compiled_early_uses_exactly`
  (`src/ledger.rs`) do. Both say in their own doc comments that
  `validator-fragments-k22` deletes them with the constants they defend.
- **`SOURCE_PATHS` is gone, and that is a scope call.** The leaf names `ROOTS`,
  `BLOCKS` and `EARLY_USES` as out of scope and does not mention `SOURCE_PATHS`
  — a fourth compiled copy of one book's source paths, in `src/lib.rs`, read by
  the CLI's loader. It had to go: the loader read it unconditionally, so *no*
  second book could be loaded at all, and the leaf's own note requires a second
  book fixture. The CLI now loads exactly the manifest's `[[root]] path` values.
  `ROOTS`, `BLOCKS` and `EARLY_USES` are untouched and still the fragment path's
  authority.
- **Spec vocabulary stays in `src/`; book data does not.** `README.md`, the three
  roles, and the `source-index` / `concept-index` lookup identities are fixed by
  the specification for *every* book and are declared once in `src/manifest.rs`
  with that stated. What is book-specific — chapter file names, page ids, titles,
  slice tokens, slice order, the book directory — is read from the manifest and
  appears nowhere in `src/` outside the three corpus constants k22 owns. Both
  sides of that line are exercised: the second-book fixture shares the
  vocabulary and differs in every datum.
- **`--help` names no book and no slice, and its examples take placeholders.**
  The specification asks for help that is both runnable and names no particular
  book; static help cannot be both once the accepted `--through` values come
  from the manifest. The examples read `--book docs/walkthroughs/<book>
  --through <slice>` and the help says where the accepted values come from. This
  is a deviation from the letter of *Fragment validator contract* and is flagged
  here rather than silently taken.
- **`ScopedSlice` is a resolved index, not free text.** It carries the chapter
  index the prefix is computed from plus the token for reporting, and its only
  constructor is `Manifest::resolve_scoped`, so the core cannot represent an
  unknown scoped value or a final-only slice. An unrecognised `--through` is a
  `U001` after the manifest load, naming the book and listing its accepted
  values in order.
- **The second book proves the structure half, under `--check markdown`.** The
  fragment path still reads the compiled corpus constants, so a second book
  cannot pass `--check fragments` until k22 — by design, since that is the seam
  the two leaves split on. `tests/second_book.rs` proves the inventory,
  identities, titles, navigation, contents links and the `--through` domain are
  all the named book's, and says in its own header what k22 turns it into.
- **Scoped runs against the finished `ordinal-fs-tree` book fail, and always
  have.** The finished book records every ownership block `resolved`, which is
  what final mode requires and what a prefix forbids; `fixed_ownership_rows`
  compared the same way before this leaf. The scoped invocations that must be
  green are the crate's own, over the fixtures, and they are.
- **Spent the leaf's one in-session reviewer, on the whole change.** Claim under
  doubt: that the relocation is behaviour-preserving for the ordinal book, that
  the derived `--through` domain is the specification's, and that the one-book
  literals left in `src/` are exactly the three constants k22 owns. Twelve
  findings came back and every one is classified below; eight are fixed here,
  each with a test, so no re-review and no `review-impl` leaf. The reviewer
  cleared the off-by-one question, the navigation forms, the `M101` inventory,
  `scan_markdown_links`, and the manifest against the compiled constants.
  - *Fixed.* A chapter's `NN` prefix is now required to equal its position, so a
    reordered manifest cannot carry two orderings; the contents page's `id` is
    pinned to `contents` as the specification's identity line fixes it; a
    `[[page]] file` must be a plain `.md` name in the book directory, so a
    malformed one is `U002` rather than a confusing "page is missing";
    `[[root]] id` and `[[block]] id` are held to the fragment-ID grammar; the
    manifest is held to the LF-only, ends-in-LF rule every page is held to;
    `page_id_for_path` was silently widened from chapters to all pages and is
    back to chapters; both interim bridges now compare **order**, not just
    membership; and `page_key` no longer allocates inside a sort comparator.
  - *Fixed — lost coverage.* Nothing exercised rejection of a slice the manifest
    *declares* but no block owns, which is the case the derivation exists to get
    right. The synthetic book grew a third chapter that owns no source, and
    `a_declared_slice_that_owns_no_source_is_final_only` covers it.
  - *Externalised.* `[guide]` and `[[glossary]]` are schema-checked and dropped,
    and the *Markdown and link validator contract* obligations that consume them
    are unimplemented — Markdown-contract work, so k22 is the wrong leaf for it.
    Cut as `book-outbound-links-k49`, with the three rules and their evidence
    written into its body.
  - *Accepted trade-off, restated.* `--help` no longer carries a runnable scoped
    invocation. The specification asks for help that is both runnable and names
    no particular book, and static help cannot be both once the accepted values
    come from the manifest. Where the two conflict the node brief wins —
    "`book-check` carries no compiled-in knowledge of any particular book" —
    so the examples take `<book>` and `<slice>` placeholders and the help says
    where the real values come from.
  - *Noise.* A page linking `walkthrough.toml` is `M201`. That is correct: *The
    link contract* names the permitted local targets exactly, and the manifest is
    not among them.
  - *Stated, not a defect.* The page inventory now has no cross-check outside the
    book — it is data the book declares, verified against the book's own
    Markdown. That is the specification's choice, argued in its *Deriving the
    whole contract from the book directory* rejected alternative; the corpus is
    where the external witness lives, and k22 owns restoring it.
  - *Corrected in a sibling.* `validator-fragments-k22`'s *Context* said `BLOCKS`
    holds 34 top-level ownership ranges; it holds 33, and a session hunting a
    thirty-fourth would be hunting nothing. Corrected in place, as a measured
    fact rather than a scope change.
- **Evidence for the no-literals claim, with a control seen to fail.** Every
  string literal in `crates/book-validation/src/` was enumerated from the
  production code (test modules stripped) — 507 distinct — and each of the 58
  matching any book-data shape was classified. The instrument was proved by
  planting `"09-a-page-that-does-not-exist.md"` in a scratch file in that
  directory and watching the enumeration report it, then removing it and
  watching the report go clean. The structural result, stated without a count of
  itself: **every remaining book-data literal in production code falls inside
  `ROOTS`, `BLOCKS` or `EARLY_USES`**, checked by line span rather than by eye.
  Both interim bridges were also seen to fail, by swapping two `[[root]]`
  stanzas and two `[[early-use]]` stanzas in the manifest and confirming the
  restored file was byte-identical.
