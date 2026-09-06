# grove-loop-k182

## Goal

Proof the `grove-loop` book — `docs/walkthroughs/grove-loop/`, twenty-one
chapters, two lookup surfaces and a contents page — and leave it at a green
`book-check --final --check all` and a green `bash scripts/check.sh`. This is the
last stage of the pipeline and the last leaf under this node; `proof` cuts
nothing after itself.

## Context

- **Three stages have run and all three are green.** The draft closed the
  fragment graph at 13 files, 10,533 resolved lines, 0 deferred, `final=true`;
  the copy edit read the whole document for prose; the art stage
  (`grove-loop-k181`) read it for figures. The node brief carries what each
  handed on, and its `## Handed forward` list is **empty** — the draft's one
  entry was `art`'s and `art` closed it.
- **Read the whole document and change only what proof owns.** A defect an
  earlier stage owns is a **correction run**, not a proof fix: cut the contiguous
  run of leaves from the owning stage through `proof`, in place of the ordinary
  last act, per `plugins/grove/skills/grove/references/editorial.md`. Structure
  and technical truth are the draft's; prose is the copy edit's; figures and
  their role statements are the art stage's.
- **What is frozen and what is not.** The literal fragments are exact source
  bytes and may not be touched — `book-check` compares them byte for byte and
  `F008` reports a single character. Everything outside a four-backtick fence is
  prose. `crates/grove-loop/` is frozen outright.
- **Manifest data, not text you may correct.** Every chapter's H1 must equal its
  `walkthrough.toml` `title`, the navigation labels reuse it, and every anchor
  named by an early-use row is a first-use target. A heading or anchor change on
  one of those is a manifest change and the ledger goes red.
- **The link contract reads as an omission if you do not know it.** A book's
  local targets are closed to its own pages, its own roots, `docs/USAGE.md` and
  `CONTEXT.md`. Decision records, `docs/specs/*`, `docs/CONFIGURATION.md`,
  `docs/ARCHITECTURE.md` and `CONTEXT-MAP.md` are named in prose by backticked
  path and **never linked** — a linkified one turns `M201` red. The book also
  cannot link to another book. Twelve anchors are declared and all twelve exist.
- **Adjudications are reports, not assertions.** Two corpus claims are known
  false and adjudicated on the page (`lib.rs` line 68, `session_config.rs`
  line 89), and a dozen more were found while drafting. `every-member-version-comment-k84`
  and `template-source-read-count-k86` hold the two fixes and both sit after this
  book. **Do not tidy an adjudication into a correction, and do not delete one.**
- **One known disagreement, owned.** The structure brief's residue map and
  chapter 21 disagreed on the *one pick and what it serves* marker;
  `residue-map-one-pick-k178` reconciled them. Chapter 21 records the coverage as
  delivered.

## What the art stage hands to the proof

- **Two tables are new and both are transcriptions.** Chapter 2's four-verdict
  partition and chapter 21's three-question table were built cell by cell from
  sentences already standing on their own pages; no number, count or claim was
  derived, adjusted or introduced. **A figure disagreeing with its page is a
  transcription error in the figure** — proof's to fix — rather than a claim to
  re-verify, which would be a correction run.
- **Two role statements are new**, in chapter 15 (after the carried-example
  figure) and chapter 20 (between the first and second mutation tables). Both are
  prose about a figure and neither restates a measurement.
- **Chapter 21 is where this stage's ordinary instincts are most likely to be
  wrong.** It owns no source, so nothing in it is anchored by a fragment and
  every sentence is a claim about another chapter. Its counts and its one ordinal
  were enumerated rather than estimated and several were corrected mid-draft;
  **do not adjust a number**, and if one looks wrong, that is a correction run.
- **Two spellings of one vocabulary run through every page, deliberately.** The
  `ordinal-fs-tree` glossary and grove's collide on *leaf* and *node* and differ
  on *ordinal* / *position* and *key* / *permanent key*. Chapters 12 to 15 hold
  the distinction sentence by sentence. Do not smooth away a *which tree* clause
  and do not unify the two vocabularies.

## Done when

- The whole document has been read for this stage's charter and what it owns is
  fixed.
- `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  is valid at 13 files, 10,533 resolved lines, 0 deferred, `final=true`.
- `bash scripts/check.sh` passes — all eight principal checks.
- Any defect an earlier stage owns has become a contiguous re-run of leaves from
  that stage through `proof`, in place of the ordinary last act.
- **Last act:** none. `proof` closes the chain; this node has no live leaf after
  it and the retirement procedure closes the node.

## Notes

**The corpus is frozen.** A defect found here becomes its own leaf under the root
brief's cross-book rule; one commit carries the source change, every affected
ledger and page, and a green validator run over every book it touched, or the
leaf is deferred behind the books it would invalidate and says so.
