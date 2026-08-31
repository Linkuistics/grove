# walkthrough-books-spec-k46

**Reviews:** walkthrough-books-spec-k20

## Goal

An adversarial read of the campaign's agreement point: `docs/specs/walkthrough-books.md`,
`docs/adr/a-book-cannot-witness-its-own-corpus.md`, and the reasoning recorded in
`walkthrough-books-spec-k20`'s running log. Four books and an overview are
authored against whatever this settles, and no session after k20 had a human
present. Findings only — no fixes.

## Context

Read the producer's commit from its handle, then the current tree. The specific
doubts, in the order they would hurt most:

- **The rejection was split rather than upheld.** k20's decision 1 claims the
  `docs/specs/ordinal-fs-tree-book.md` sidecar rejection was two rejections
  fused — one about the fragment graph (kept) and one about the authoring
  contract (never actually made). Re-read that rejection in the VCS history and
  test the split clause by clause yourself. If any clause reaches the corpus
  boundary, the page inventory or the slice order, the verdict is wrong and four
  books are about to be built on it.
- **Property 3 is the walk-away property and it was amended.** The spec now says
  expansion reads only Markdown and source, and that the manifest "contributes
  zero bytes". Attack it: is there any field in *The manifest* that a correct
  implementation would have to consult to reconstruct a byte? The root `lines`
  count, the block ranges and the derived scoped-slice domain are the candidates.
- **The corpus witness is the whole argument for the design.** k20's decision 4
  claims the filesystem-derivation check is *stronger* than the two-hand-written-
  lists cross-check it deletes. Test that: what class of error did the old
  spec-versus-constants comparison catch that derivation does not? An
  `[[corpus.add]]` with a plausible reason is an unchecked assertion; so is an
  `[[corpus.exclude]]`. Is the exclusion class ("inline test modules and
  test-support modules") actually closed, or is it five observations wearing a
  rule's clothes?
- **Implementability without a second design conversation.** The spec claims
  `validator-structure-k21` and `validator-fragments-k22` can implement against
  it directly. Read *The manifest*, *The corpus rule and its witness* and
  *Fragment validator contract* as if you were k21: is any field's type,
  cardinality, ordering or failure code undetermined? The two accepted glob forms,
  the `--through` domain derivation, and the CLI's new post-manifest validation of
  `--through` are where under-specification would hide.
- **The interim corpus control.** `compiled_corpus_copy_matches_the_book_ledger_tables`
  (`crates/book-validation/src/validator.rs`) is a deliberate bridge that k20
  admits is weaker than what it replaced. Check the admission is honest and that
  the successor obligation is actually placed on a live leaf rather than only
  described.
- **The guide-link contract.** k20's decision 8 puts one guide link per book on
  `README.md` and gates anchors through the manifest; decision 9 exempts the
  relocated `ordinal-fs-tree` book entirely. Both are judgement calls a reviewer
  should contest. Does the exemption leave `walkthroughs-k3` decision 5's
  guide-before-books ordering asserting anything at all for the one book that
  exists today? And are the two anchor obligations — `docs/USAGE.md` and
  `CONTEXT.md` each carry zero explicit anchors — placed on a leaf, or merely
  noted?
- **What the spec deleted.** The old document's ~110 lines of per-page concept
  responsibilities and its worked-examples table were moved out to a per-book
  *structure brief*. Check nothing load-bearing was lost with them: the relocated
  book's page responsibilities are now recorded nowhere, and no structure brief
  for it exists.

## Done when

Findings are written down with enough specificity that an integrating session can
act on each without re-deriving it: what is wrong, where, and why it matters to a
downstream leaf. A review that finds nothing creates nothing and simply retires —
that is a normal outcome and the right one if the design holds.

## Notes

**Place any `integrate-review-design` you cut ahead of its consumers, not at this
directory's end.** `validator-structure-k21` and `validator-fragments-k22` both
implement against this spec and both sit after this leaf, so a plain `leaf-add`
would put the integration behind the work it exists to redirect. Cut it as
`grove-llm leaf-insert validator-structure-k21 walkthrough-books-spec --kind integrate-review-design`.

**The corpus is frozen and this leaf changes nothing.** A defect you find in
`crates/` is a finding, and the leaf that fixes it lands under the root brief's
cross-book rule.
