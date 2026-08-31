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

## Findings

### F1 · High — The ADR's clause-by-clause split contradicts both the old rejection and the new spec

At the producer's parent revision, the rejected sidecar duplicated “parents,
children, ownership, and ranges” already visible in Markdown and the source
index retained directives plus reconciled tables for that reason
(`docs/specs/ordinal-fs-tree-book.md@d2d839bb^-:1059-1065`). The new ADR says all
of those clauses concern only the fragment graph and that none reaches the
obligation (`docs/adr/a-book-cannot-witness-its-own-corpus.md:53-71`), while the
new spec says the ownership plan was never in Markdown
(`docs/specs/walkthrough-books.md:1265-1268`). That is false for source roots and
top-level ownership blocks: `walkthrough.toml` carries their IDs, paths, owners
and ranges (`docs/specs/walkthrough-books.md:218-226`), and the same spec
explicitly says the Markdown Source roots and Ownership blocks tables restate
those manifest rows (`docs/specs/walkthrough-books.md:588-599`).

The parents/children/bytes half of the split can still stand, but the current ADR
is not a coherent current-state account of the trade-off: adopting the manifest
*does* reverse the old rejection for top-level ownership and ranges, under a new
trust order. Rework the ADR and the spec to admit and justify that reversal (or
remove the duplicate); do not preserve the verdict by redefining the historical
clause more narrowly than it was written.

### F2 · High — The filesystem derivation is not an external corpus witness

The campaign's corpus is fixed independently as each subject crate's own
`Cargo.toml` plus every `src/**/*.rs`, with one named campaign exclusion
(`.grove/BRIEF.md:113-134`). The manifest schema does not encode that rule. It
accepts any non-empty ordered set of exact paths or recursive patterns and any
number of authored additions and exclusions
(`docs/specs/walkthrough-books.md:191-208`); `[book].subject` is only diagnostic
context, not a constraint tying those patterns to the subject. A manifest can
therefore include only `<subject>/Cargo.toml`, declare that one root, and pass the
proposed set-equality check. It can also exclude a real production file with a
plausible reason: reasons are never validated, and the spec expressly permits an
exclusion outside the observed test/test-support class so long as the manifest
describes it (`docs/specs/walkthrough-books.md:267-298`).

The real directory proves that declared patterns matched; it does not prove that
the author chose the right patterns or exceptions. The old control required two
independently authored corpus statements, whereas the replacement lets one
manifest edit change both the asserted rule and the roots checked against it.
Consequently property 4's claim that a book cannot choose the corpus it proves
(`docs/specs/walkthrough-books.md:68-79`), the ADR's “stronger” claim
(`docs/adr/a-book-cannot-witness-its-own-corpus.md:36-51`), and the bridge's
promise that derivation restores the external witness
(`crates/book-validation/src/validator.rs:1513-1520`) do not hold.
`validator-fragments-k22` must not delete the independent control until the
design supplies a corpus boundary the manifest cannot narrow itself — for
example, required base patterns derived from `subject` plus a closed, checkable
exception policy, or another independent inventory.

### F3 · Medium — A slice does not uniquely identify its chapter

The manifest derives its ordered `--through` domain from chapter `slice` values
and lets blocks name those values as owners
(`docs/specs/walkthrough-books.md:210-226,238-253`), but nowhere requires one
slice value per chapter. A schema-valid-looking manifest can therefore give two
chapters the same slice. The accepted-value list then contains a duplicate, a
block owner does not identify one required page, and `F010` cannot say which
chapter owns a fragment. The current one-book implementation makes the missing
invariant visible: `PAGE_BY_OWNER` is one-to-one and fragment placement takes the
first owner match (`crates/book-validation/src/ledger.rs:9-38,699-707`).

This is exactly the design decision `validator-structure-k21` is forbidden to
invent (`.grove/07-walkthrough-machinery-k10/05-impl--validator-structure-k21.md:11-18`).
Specify that chapter slice IDs are unique, make duplication a `U002` schema
failure, and state the corresponding uniqueness requirements for the page fields
used as identities before that consumer runs.

### F4 · Medium — The guide-first anchor contract remains optional and its writes are unowned

The spec permits a guide citation with no anchor unconditionally
(`docs/specs/walkthrough-books.md:692-705`), does not require a non-empty
`[guide].anchors`, and exempts the only book that exists today from a guide link
(`docs/specs/walkthrough-books.md:742-754`). The future book leaves require only
that links to `docs/USAGE.md` resolve — an anchorless file link satisfies that
wording (`.grove/09-pilot-k12/03-impl--jj-workspace-book-k25.md:28-36`;
`.grove/11-crate-books-k14/03-impl--overview-book-k30.md:27-37`). Meanwhile
`usage-guide-k23` requires coverage, examples and existing link checks, but no
explicit anchor set (`.grove/08-user-guide-k11/02-impl--usage-guide-k23.md:26-32`).
The spec itself records that both `docs/USAGE.md` and `CONTEXT.md` currently have
zero explicit anchors, then assigns their creation only in prose
(`docs/specs/walkthrough-books.md:707-719`).

Thus the campaign can satisfy every live leaf with anchorless guide links and no
glossary anchors, while `user-guide-k11` still claims its costly placement is
earned by anchor stability (`.grove/08-user-guide-k11/BRIEF.md:50-64`). Either
require and predeclare at least the stable anchors runtime books must cite and
place both target-document edits on live leaves, or keep file-only links and
remove the anchor-stability argument for guide-first ordering. Merely noting that
a future book may add anchors does not assign the already-visible work.

### F5 · Medium — The relocated book's human structure contract was deleted without a replacement

The old spec carried page-by-page concept responsibilities and exact worked
example anchors/boundaries (`docs/specs/ordinal-fs-tree-book.md@d2d839bb^-:141-253,1011-1033`).
The new spec says those decisions belong exclusively to a per-book structure
brief — including concept/seam responsibility, example boundary, early uses and
deliberate exclusions — and admits that the manifest preserves only the chapter
sequence and ownership mapping (`docs/specs/walkthrough-books.md:756-778`). The
tree contains structure-requirements leaves for the five new deliverables, but
none for `ordinal-fs-tree`; `validator-structure-k21` authors the manifest and
does not restore the missing human contract.

The finished Markdown shows what was written, not which responsibilities were
binding, so it cannot witness the design it is meant to be reviewed against.
Recover the deleted ordinal responsibilities and worked-example table into the
per-book structure-brief form before `validator-structure-k21`, and give that
input a lifetime appropriate to any later edit or review instead of relying on
VCS archaeology.
