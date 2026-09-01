# glossary-anchors-k62

## Goal

Give every term entry in `CONTEXT.md` an explicit anchor, so the glossary is
uniformly addressable, and remove the four-of-sixty-one inconsistency
`jj-workspace-book-k25` left behind.

## Context

- **The problem, found rather than predicted.**
  [`docs/specs/walkthrough-books.md`](../../docs/specs/walkthrough-books.md),
  *Anchors are declared, and the declaration is what is checked*, requires a cited
  anchor to exist in its target as an explicit `<a id="…"></a>` line
  **immediately preceding a heading**, and `book-validation`'s `explicit_anchors`
  (`crates/book-validation/src/markdown.rs`) implements exactly that: an anchor
  line whose next line is not a heading is not an anchor. `CONTEXT.md`'s 61 term
  entries are bold paragraph lead-ins (`**Term**:`), not headings.
- `orientation-k55` therefore converted the four terms its book reserves —
  `task-commit-boundary`, `driver-lease`, `stated-vcs`, `loop-control-channel` —
  into anchored `###` headings, and deliberately went no further: a document-wide
  reformat is outside the draft stage's allowlist in
  [`preregistration.md`](../../docs/evaluations/editorial-pipeline-pilot/preregistration.md),
  and the draft's diff is the pilot's baseline.
- Placed **after** every crate book so the full reserved-anchor set is known when
  the generalisation is made, rather than guessed at ahead of it. Nothing waits on
  this leaf: a book reserving a new term simply anchors that term as
  `orientation-k55` did.
- Two structural facts to respect: `## Language` is the glossary's only body H2,
  and `### Task-tree scheme …` and `### Root and lifecycle semantics …` are
  **grouping** headings introducing runs of terms rather than terms themselves.
  A uniform scheme has to place term headings against both.

## Done when

- Every term entry in `CONTEXT.md` carries an explicit `<a id="…"></a>` line
  immediately preceding a heading, under one stated scheme — anchor slug derived
  from the term, heading level consistent with the two grouping headings.
- The four anchors the books already reserve keep their exact slugs. A renamed
  slug silently breaks the book that reserved it, so treat those four as fixed.
- Every book under `docs/walkthroughs/` still validates:
  `bash scripts/check.sh` passes.
- `CONTEXT.md`'s own `[[wikilink]]` cross-references still read correctly against
  the new headings.

## Notes

**This is presentation, not vocabulary.** No term's definition, `_Avoid_` line or
retirement note changes; if a definition looks wrong while doing this, that is a
separate leaf.
