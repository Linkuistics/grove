# grove-loop-k180

## Goal

Copy-edit the `grove-loop` book — `docs/walkthroughs/grove-loop/`, twenty-one
chapters, two lookup surfaces and a contents page over 10,533 lines of frozen
corpus — and leave it at a green `book-check --final --check all` and a green
`bash scripts/check.sh`.

## Context

- **The draft is complete and green.** `grove-loop-k123` retired with the book at
  13 files, 10,533 resolved lines, 0 deferred, `final=true`, all thirty-nine
  ownership rows `resolved` and all fifty early-use rows `explained`. Read the
  whole book as the draft left it and change only what this stage's charter owns.
- **This is the largest book in the campaign by a wide margin** — roughly 30,900
  lines of page against `keyed-launch`'s and `grove-llm`'s. Budget accordingly;
  it is one session's work only if it is read as prose rather than re-verified.
  The draft owned technical truth and discharged it, including an adversarial
  pass over the assembly chapter.
- **The charter is prose, and the draft owned structure and technical truth.**
  Conceptual order, what each chapter is for, and whether a claim about the crate
  is true were the draft's obligations. A fix here that reorders a chapter,
  re-attributes a test, re-adjudicates a comment or corrects a measurement is a
  **correction run**, not a copy edit. Where you find one, cut the contiguous run
  the family file describes — from the owning stage through `proof` — rather than
  absorbing it.
- **What is frozen and what is not.** The literal fragments are exact source
  bytes and may not be touched: `book-check` compares them byte for byte and
  `F008` reports a single character. Everything outside a four-backtick fence is
  prose you own. `crates/grove-loop/` is frozen outright.
- **Manifest data, not headings you may reword.** Every chapter's H1 must equal
  its `walkthrough.toml` `title`, the navigation labels reuse it, and every anchor
  named by an early-use row in the source index is a first-use target. A heading
  or anchor change on one of those is a manifest change, and the ledger goes red.
- **The link contract reads as an omission if you do not know it.** A book's
  local targets are closed to its own pages, its own roots, `docs/USAGE.md` and
  `CONTEXT.md`. Decision records, `docs/specs/*`, `docs/CONFIGURATION.md`,
  `docs/ARCHITECTURE.md` and `CONTEXT-MAP.md` are named in prose by backticked
  path and **never linked** — a linkified one turns `M201` red. The book also
  cannot link to another book. Twelve anchors are declared and all twelve exist.

## What the draft hands to the copy edit

- **Two spellings of one vocabulary run through every page, deliberately.** The
  `ordinal-fs-tree` glossary and grove's collide on *leaf* and *node* and differ
  on *ordinal* / *position* and *key* / *permanent key*. Chapters 12, 13, 14 and
  15 open a paragraph with **Say which tree** and then hold the distinction
  sentence by sentence. That is a technical obligation, not a verbal tic: do not
  smooth away a *which tree* clause, and do not unify the two vocabularies.
- **Adjudications are reports, not assertions, and there are many.** Two corpus
  claims were known false before drafting and are adjudicated on the page
  (`lib.rs` line 68, `session_config.rs` line 89); a dozen more were found while
  drafting — stale enumerations, a helper list naming a function that never
  existed, four comments addressing `grove-llm`'s CLI by a module name this
  workspace does not have, five unresolved intra-doc links, a parenthesised
  anchor citation naming nothing in the repository. Each is stated beside the
  fragment that reproduces it because the corpus is frozen. **A stage that reads
  one of these as an error in the book has misread it**, and tidying an
  adjudication into a correction is the one edit here that would do real damage.
- **Chapter 21 is the page where this stage's ordinary instincts are most likely
  to be wrong.** It owns no source, so nothing in it is anchored by a fragment,
  and every sentence is a claim about another chapter. Its cadence — the
  twenty-by-three table, then one section per question reading down a column — is
  structural rather than stylistic. Its counts and its one ordinal were
  enumerated rather than estimated, and several were corrected mid-draft after an
  adversarial read; **do not adjust a number**, and if one looks wrong, that is a
  correction run.
- **Two open leaves sit ahead of this one and will edit pages you are reading.**
  `residue-map-one-pick-k178` corrects the structure brief's residue map and
  reconciles chapter 21's `#what-the-book-made-redundant` section to it;
  `core-fallible-construct-counts-k179` re-derives two figures in chapter 19's
  closing section. Both run before this stage in the walk, so the pages should
  already carry their result — but if either is still live when you start, say so
  rather than editing around it.
- **Nothing is outstanding in the node brief's `## Handed forward`** except the
  one entry for `art`, which is that stage's and not this one's.

## Done when

- Every page reads as one voice, with the book's prose contract held and the four
  classes above left alone.
- `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  is valid at 13 files, 10,533 resolved lines, 0 deferred, `final=true`.
- `bash scripts/check.sh` passes — all eight principal checks.
- Any defect an earlier stage owns has become a contiguous re-run of leaves from
  that stage through `proof`, in place of the ordinary last act.
- **Last act:** `grove-llm leaf-add grove-loop-book-k37 grove-loop --kind art`,
  unless a live later sibling under `grove-loop-book-k37` already holds that
  stage — in which case cut nothing.

## Notes

**The corpus is frozen.** A defect found here becomes its own leaf under the root
brief's cross-book rule; one commit carries the source change, every affected
ledger and page, and a green validator run over every book it touched, or the
leaf is deferred behind the books it would invalidate and says so.
