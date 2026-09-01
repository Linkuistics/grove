# jj-workspace-book-k25 — brief

## Goal

Draft the `jj-workspace` book to green final validation: a complete,
source-exact walkthrough of the crate's four roots and 698 lines under
`docs/walkthroughs/jj-workspace/`, uniform with the relocated `ordinal-fs-tree`
book.

## Context

- Inputs, both committed before this session and both binding: the human's
  structure brief from `jj-workspace-structure-k17`, and the preregistration from
  `pilot-preregistration-k24`. The preregistration's attribution rule tells you
  what this session owes the record about the draft stage; satisfy it as you go
  rather than reconstructing it afterwards.
- The corpus, exactly, from the root brief: `crates/jj-workspace/src/lib.rs`
  (343), `src/refusal.rs` (230), `src/jj.rs` (81), and `crates/jj-workspace/Cargo.toml`
  (44). `tests/` is evidence, not a root. Every byte of those four files belongs
  to a fragment graph.
- The contract is the shared specification `walkthrough-books-spec-k20` landed,
  and the per-book corpus format `validator-fragments-k22` implemented. The
  validator takes the book directory; there is no compiled-in ledger to edit.
- Method: `linkuistics:writing-code-walkthroughs`. Scoped proof exists so a
  partial book is provable — use `--through` per slice as you go rather than
  discovering at the end that the graph does not close.

## Done when

- `docs/walkthroughs/jj-workspace/` holds the book, and final validation over it
  passes with no deferred holes.
- Its `README.md` reader contract cites `docs/USAGE.md` **by a declared anchor**,
  that anchor is listed in the book's `[guide] anchors`, and it exists in the
  guide as an explicit `<a id="…"></a>`. An anchorless file link does not
  discharge this: the specification makes the anchor citation the one place the
  guide contract binds, because it is the only thing the guide-before-books
  ordering actually buys.
- This is the **first book to cite the glossary**, so it owns the other half of
  that contract: every `CONTEXT.md` anchor it reserves is added to `CONTEXT.md`
  as an explicit anchor in this commit. `CONTEXT.md` carries none today; adding
  them is additive and breaks nothing.
- `every_repository_markdown_reference_resolves` passes, and the book's own
  `M201` checks pass — the sweep accepts a heading slug and is not evidence for
  either obligation above on its own.
- `bash scripts/check.sh` passes, with the new book gated by the discovery
  `validator-fragments-k22` built rather than by a hand-added line.
- The draft stage's record exists in the form the preregistration's attribution
  rule requires.

## Decomposition

Seven children, one per slice of the book's own sequence, in the canonical page
order the structure brief fixed. The ordering is the book's reading order and it
is also the only order the scoped validator accepts: `--through` proves a
canonical *prefix*, so a slice cannot be drafted before the slice ahead of it
exists.

1. `orientation-k55` — slice `no-dependencies`, `01-orientation.md`. Carries the
   whole book's scaffolding as well as its own chapter, because
   [`walkthrough-books.md`](../../../docs/specs/walkthrough-books.md)'s
   *Authoring workflow and scoped proof* requires it: the manifest is complete
   from the start, and the first slice creates `README.md`, both lookup indexes,
   every source-root directive, the complete top-level ownership ledger, its own
   fragment definitions, and a defer for every later-owned block. It also
   discharges the three obligations that live outside the book — the four
   `CONTEXT.md` anchors, the `docs/ARCHITECTURE.md` ownership row, and the draft
   stage record's `## Baseline`.
2. `the-gate-k56` — slice `one-lane`, `02-the-gate.md`.
3. `subprocess-seam-k57` — slice `nothing-ambient`, `03-subprocess-seam.md`.
4. `namespace-k58` — slice `no-consumer-vocabulary`, `04-namespace.md`.
5. `scope-and-commit-k59` — slice `no-transactions`, `05-scope-and-commit.md`.
6. `refusal-k60` — slice `no-remedy-of-its-own`, `06-refusal.md`.
7. `what-jj-owns-k61` — slice `assembly`, `07-what-jj-owns.md`. Owns no
   production source and is final-only: it is the child that takes the book to
   green **final** validation and to a green `bash scripts/check.sh`, and it is
   therefore the only child whose `Done when` carries either.

**Every child but the last leaves `scripts/check.sh` red, and that is the shape
rather than a lapse.** The script runs `book-check --final --check all` over every
book root under `docs/walkthroughs/` by discovery, so the book directory is inside
the gate from the moment `orientation-k55` creates it, while a prefix deliberately
leaves later blocks deferred. Each child therefore proves itself with
`book-check --through <its slice> --check all` and with the rest of the script's
checks, and says so; `what-jj-owns-k61` closes the gate. The preregistration
already admits this: the draft is the one stage permitted a commit range, its
baseline is the commit before the first of them, and its validity is judged on
final validation being green **at the end of the stage**.

## Notes

**This is the draft stage only.** Developmental edit, technical edit, copy edit,
art and proof are the `pilot-measure-k26` node's — one leaf and one commit each. A draft that has been
quietly polished destroys the attribution rule's ability to credit anything,
because there is then no unedited baseline for the later stages to be measured
against.

**The corpus is frozen.** Do not edit `crates/jj-workspace/`. A defect found here
becomes its own leaf under the root brief's cross-book rule.

**If this proves bigger than one session, decompose it** — one child per slice of
the book's own sequence, doing only the first. That is cheaper than a long
session, and the scoped validator was built to make a prefix provable.

## Decisions (running log)

**1 · This leaf is decomposed into one child per slice, and this session does the
first.** The task file pre-authorised it and the preregistration accommodates it
(*Part 3*, the draft's one exception: the draft stage is every commit from the
first drafting commit to the last inclusive, its baseline is the commit before the
first of them, and the draft is not scored, so a range costs the measurement
nothing). Taken on measurement rather than on feel. The precedent book is 12,575
Markdown lines over 8,720 source lines — a ratio of 1.44 — and this book is
required by decision 5 of `jj-workspace-structure-k17` to carry *more* prose per
source line than the precedent, not less: every argued claim must name the jj
behaviour it rests on, the test that proves it, and the alternative rejected. So
698 source lines is a book of roughly 2,500 to 3,000 Markdown lines across seven
chapters, `README.md`, two lookup surfaces and a manifest, every byte of it
line-aligned to a fragment graph. Beside that, two of this leaf's obligations are
design work rather than transcription — the form the `CONTEXT.md` anchors take
(decision 2) and the fragment partition inside each block, which the structure
brief explicitly leaves to the owning slice. Running that long in one session
degrades the later chapters, and the later chapters are where `refusal.rs` (230
lines, the largest block) and the closing synthesis live. Rejected: one session
with scripted fragment extraction — extraction is the cheap half, and the
expensive half is the adjudicating prose the pilot is measuring.

**2 · The four reserved `CONTEXT.md` anchors are taken minimally, as four
anchored `###` term headings, and generalising the glossary is cut as its own
leaf.** Found while checking the obligation rather than assumed: the contract's
"explicit anchor" is not merely an `<a id="…"></a>` line. `book-validation`'s
`explicit_anchors` accepts an anchor line **only when the next line is a
heading** (`crates/book-validation/src/markdown.rs`), which is the specification's
*immediately preceding a heading* read literally. `CONTEXT.md` has 61 term
entries and every one of them is a bold paragraph lead-in (`**Term**:`), not a
heading, so the task file's "adding them is additive and breaks nothing" does not
hold as written — an anchor before a bold paragraph is a label for nothing and is
reported as `M201` against the manifest. All four reserved terms sit above the
first `###` grouping heading, so `###` is the level that introduces no skip.
Rejected: converting all 61 terms to headings in this commit — it is a
document-wide reformatting rather than the "`CONTEXT.md` anchors" the
preregistration's allowlist admits for the draft, and the draft's diff is the
pilot's baseline. Rejected: citing `CONTEXT.md` without anchors, which the
specification does permit everywhere except the `README.md` guide citation —
decision 7 of `jj-workspace-structure-k17` reserved these four by name and the
task file's `Done when` requires them, so it is not this session's to reopen. The
inconsistency four-of-sixty-one leaves is real and is `glossary-anchors-k62`'s.
