# walkthrough-books-spec-k20

## Goal

Settle where a per-book corpus lives and what governs it: reopen the
sidecar-manifest rejection now that there are six books, record the verdict as an
ADR, and rewrite `docs/specs/ordinal-fs-tree-book.md` into a specification of the
book *system* whose per-book ledgers are data.

## Context

- Decision 10 of `plan-k1`: the book spec is split **in place** — a shared
  walkthrough-books spec covering page conventions, navigation, prose contract,
  audience, assurance and the fragment language, with per-book ledgers as data
  rather than specification. Editing in place rather than writing a second
  overlapping document is what the records doctrine prescribes.
- The rejection being reopened is `docs/specs/ordinal-fs-tree-book.md`,
  *Rejected alternatives and limits* → *A TOML sidecar manifest*. Its argument is
  that a sidecar duplicates parents, children, ownership and ranges already
  visible in Markdown, that drift would force a choice about which
  representation to trust, and that raw Markdown must remain sufficient to
  reconstruct the code. Read it before you decide anything: the argument was
  made for one book and the question is whether any clause of it depends on that.
- The same document's *Intended outcome* property 3 states the same commitment
  positively — raw Markdown exposes every fragment's parent, children, source
  range and owning slice **without requiring an author-maintained sidecar
  manifest**. A verdict that admits a sidecar has to amend that property too.
- What the validator currently compiles in, and what therefore has to come from
  somewhere: `SLICE_ORDER` and `PAGE_BY_OWNER` (canonical slice order, page
  filenames, page-to-slice mapping), `ROOTS` (source path and exact line count
  per root), `BLOCKS` (34 ownership ranges), `EARLY_USES`, and `SOURCE_INDEX`
  (the book's own source-index path). `src/cli.rs` also hard-codes the slice
  tokens as a `--through` `value_parser` list.
- The corpus each book must carry is enumerated in the root brief's *Pointers*:
  33 roots and 14,525 lines across five deliverables, plus the relocated book's
  own unchanged seventeen-file corpus.

## Done when

- An ADR under `docs/adr/` records the per-book-corpus decision with its
  trade-off — whether the ledger stays derived from the book's own Markdown, or
  becomes a sidecar, or splits — and states what would reopen it. If the original
  rejection survives contact with six books, an ADR recording *that*, with the
  clause-by-clause reasoning, is a successful outcome.
- `docs/specs/` holds one specification of the book system, with per-book data
  described as data. Whether that is a rewritten
  `docs/specs/ordinal-fs-tree-book.md` under a new slug or a rename is yours to
  decide; `CONTEXT-MAP.md`'s record-ownership list names the current slug and
  must agree with whatever you leave behind.
- The specification states the per-book corpus format precisely enough that
  `validator-structure-k21` and `validator-fragments-k22` can implement against
  it without a second design conversation.
- Every artifact that links into the old spec still resolves, and
  `bash scripts/check.sh` passes.

## Notes

**This is the campaign's agreement point and it earns an adversarial read.** Four
books and an overview are authored against whatever this settles, and no session
after this one has a human present. Cut `review-design` as your last act if the
decision is anything other than "the rejection stands unchanged".

**Do not specify the art phase here.** Figures, assets and what the validator
should know about them are deliberately deferred until the pilot has run the art
stage by hand and its measure has said whether art paid — see
`figure-contract-k18`. A specification that leaves a hole for assets is correct;
one that fills the hole now is machinery ordered ahead of its measurement.

**The prose contract, audience and assurance sections are shared as they stand.**
Decision 7 of `plan-k1` fixes the audience — a reader who knows Rust and jj and
has driven a grove, with grove vocabulary linked to `CONTEXT.md` and never
re-taught — and it applies to every book, not just this one.
