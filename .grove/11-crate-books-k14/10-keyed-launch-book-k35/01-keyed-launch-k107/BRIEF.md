# keyed-launch-k107 — brief

## Goal

Draft the `keyed-launch` book to green final validation: a complete,
source-exact walkthrough of the crate's nine roots and 2,073 lines under
`docs/walkthroughs/keyed-launch/`, in the ten-page shape the structure brief
settled, uniform with the `overview`, `grove-llm`, `jj-workspace` and
`ordinal-fs-tree` books.

## Context

- **The structure brief is `docs/specs/keyed-launch-book-structure.md`**, and it
  is the precondition `grove-draft` requires. All three things it must state are
  in it, quotable: who the reader is and what they can do afterwards
  (*Audience and intended outcome* — a reader who knows Rust and Jujutsu and has
  driven a grove, whose outcome is the **pass-through test**); the ordered
  section plan (*Chapter sequence*, ten pages of which nine own source, and
  *Concept and seam responsibilities*, one subsection per page); and what
  deserves emphasis and what the book does not cover (*What each chapter's prose
  owes*, *The spine*, *What the book deliberately does not cover*). Its
  ownership mapping is the manifest's `[[page]]` and `[[block]]` groups; where
  the two disagree it is a defect in one of them, not a licence to prefer
  either.
- The corpus, exactly, per root: `Cargo.toml` (47), `src/lib.rs` (68),
  `src/vocabulary.rs` (44), `src/argv.rs` (48), `src/error.rs` (81),
  `src/conformance.rs` (104), `src/channel.rs` (404), `src/run.rs` (607),
  `src/templates.rs` (670) — 2,073 lines. `crates/keyed-launch/tests/` is
  evidence, not a root, and at 1,319 lines it is heavy evidence: the brief's
  intended outcome names six tests by name and each chapter owes the test that
  proves its claim.
- **`src/channel.rs` lines 272–404 are an inline `#[cfg(test)] mod tests` and
  are inside the corpus.** `docs/specs/walkthrough-books.md`'s corpus exception
  inventory carries no `keyed-launch` row, so those 133 lines are owned,
  reconstructed and explained like any other — by chapter 9, against chapter 6's
  fragments, which are behind it. Do not add an exception row to make them go
  away; that would be a specification edit the brief did not settle.
- The contract is `docs/specs/walkthrough-books.md`; the validator is
  `book-check`, run `--through <slice>` per child and `--final` by the last.
  Scoped proof exists so a partial book is provable — validate per slice as you
  go rather than discovering at the end that the graph does not close.
- Method: `linkuistics:writing-code-walkthroughs`. The draft owns structure and
  technical truth as obligations to discharge, not as things the brief handled:
  every claim about the crate is checked against the crate and its tests, not
  against memory, the decision records, or an earlier page of the same book.
- **The standing risk is the 13%/53% asymmetry**, and it is this book's own
  rather than a precedent's. 33% of the corpus is comment prose, unevenly:
  `src/run.rs` is 53% and argues its cases in situ, while `src/templates.rs` —
  the biggest root — is 13% and is where every rule the decision records state
  actually binds. So the third prose obligation is **directional**: chapters 3, 4
  and 5 supply the argument the source does not make; chapters 7 and 8 do not
  restate an argument the fragment graph has just quoted verbatim. A page that
  pads where the source is strong and thins where it is silent has failed the
  obligation in both directions.
- The crate never learns what a launch is *for*, and grove's mapping onto it is
  one line: a session kind is a key. Keep the book on the crate's side of that
  line — a book that explains grove's sessions has documented the wrong crate.
  `usage-session-lifecycle` is cited so that no chapter has to.

## Done when

- `docs/walkthroughs/keyed-launch/` holds the book and final validation over it
  passes with no deferred holes: 9 files, 2,073 resolved lines, `final=true`.
- `README.md` cites `docs/USAGE.md#usage-running-grove`, and the manifest's
  three declared anchors — `usage-running-grove`, `usage-session-lifecycle`,
  `loop-control-channel` — all exist in their targets in the explicit form
  today, so `book-check`'s `M201` is green from the first slice and **no
  glossary promotion is owed**.
- **The one obligation outside the book is discharged**: `docs/ARCHITECTURE.md`'s
  *Documentation ownership* table gains its `keyed-launch` row, in the wording
  the brief's *The book's row in the ownership table* carries, so
  `every_book_root_has_a_documentation_ownership_row` is green. It belongs with
  the first slice's scaffolding, as each precedent book's row was.
- `every_repository_markdown_reference_resolves` and the corpus-inventory tests
  pass; `bash scripts/check.sh` passes, the book gated by discovery rather than
  by a hand-added line.
- The last child's last act is `grove-llm leaf-add keyed-launch-book-k35
  keyed-launch --kind copy-edit`, unless a live later sibling under
  `keyed-launch-book-k35` already holds that stage.

## Decomposition

Ten children, one per slice, in canonical page order — the only order the scoped
validator accepts, since `--through` proves a prefix. Each child's figures are
cumulative resolved and deferred lines over the 2,073-line corpus.

1. `orientation-k108` — slice `understands-neither`, `01-orientation.md`, owns
   `manifest-three-dependencies` (47), `library-root` (68) and `two-opaque-errors`
   (81) — 196 lines. Carries the whole book's scaffolding: the complete manifest,
   `README.md`, both indexes, all nine source-root directives, the full ownership
   ledger with seventeen defers, every early-use row `pending`, and the one
   obligation outside the book. 196 resolved, 1,877 deferred.
2. `the-names-k109` — slice `rules-about-names`, `02-the-names.md`, owns
   `vocabulary` (44) and `template-shapes` (91) — 135. 331 resolved, 1,742
   deferred.
3. `two-documents-k110` — slice `never-assembled`, `03-two-documents.md`, owns
   `templates-load` (54) and `reading-and-whole-document-validation` (139) — 193.
   524 resolved, 1,549 deferred.
4. `template-law-k111` — slice `words-not-shell`, `04-template-law.md`, owns
   `node-and-template-rules` (120), `word-scanning` (83) and `diagnostics` (43) —
   246. 770 resolved, 1,303 deferred. The heaviest of the `templates.rs` chapters
   and the one that discharges *name what is wrong, name where, name what fixes
   it*.
5. `to-an-argv-k112` — slice `whole-word-or-nothing`, `05-to-an-argv.md`, owns
   `resolution-and-expansion` (130), `templates-keys` (10) and `argv` (48) — 188.
   958 resolved, 1,115 deferred.
6. `the-channel-k113` — slice `appearance-is-the-event`, `06-the-channel.md`, owns
   `channel-production` (271). 1,229 resolved, 844 deferred.
7. `the-job-k114` — slice `nothing-else-added`, `07-the-job.md`, owns
   `launch-shape` (123) and `terminal-and-spawn` (205) — 328. 1,557 resolved, 516
   deferred. The heaviest chapter in the book.
8. `the-escalation-k115` — slice `the-launchers-job`, `08-the-escalation.md`, owns
   `watch-and-launcher-signals` (120) and `supervise-and-escalate` (159) — 279.
   1,836 resolved, 237 deferred.
9. `how-checked-k116` — slice `checked-without-meaning`, `09-how-checked.md`, owns
   `channel-inline-tests` (133) and `conformance` (104) — 237. 2,073 resolved, 0
   deferred, still `final=false`.
10. `what-passes-through-k117` — slice `assembly`, `10-what-passes-through.md`.
    Owns no source and is final-only: it takes the book to green **final**
    validation and a green `bash scripts/check.sh`, and is the only child whose
    `Done when` carries either. Its last act is the pipeline's — `grove-llm
    leaf-add keyed-launch-book-k35 keyed-launch --kind copy-edit`, unless a live
    later sibling under that node already holds the stage.

**Every child but the last leaves `scripts/check.sh` red on `book-check`, and
that is the shape rather than a lapse.** The script runs `--final` over every book
root by discovery, so the book is inside the gate from the moment child 1 created
it, while a prefix deliberately leaves later blocks deferred. Each child proves
itself with `book-check --through <its slice> --check all` and the rest of the
script's checks, and says so.

## Notes

**This is the draft stage only.** Copy edit, art and proof are the later stages
under `keyed-launch-book-k35`, cut lazily, each as the last act of the stage
before it. Figures are drawn where the prose contract requires a relation to be
drawn, and left to `art` otherwise; a draft that has been polished leaves the
next stage's empty result unreadable.

**Expect this to decompose, one child per slice.** Nine source-owning chapters
over 2,073 lines, with chapter 7 at 328 lines and chapter 6 at 271, and a prose
contract that makes the prose-to-source ratio at least `grove-llm`'s. The order
is forced: `--through` proves a prefix, so children run in canonical page order,
and the closing `assembly` page owns no source and is final-only. That is
guidance from the shape of the corpus, not a decision already taken — the seam
is this session's to cut.

**Cite `docs/ARCHITECTURE.md`, `CONTEXT-MAP.md`, the specs and the four decision
records by backticked path, never by link**: the outbound-link contract closes a
book's local targets to its own pages, its own roots, the guide and the
glossary. The brief's *The records this crate is governed by are named and never
cited* says what each chapter owes instead — the lines that keep the record, and
which record they answer to.

**Nothing inside the corpus was found stale**, so unlike `grove-llm`'s chapters 1
and 2 no chapter here carries an adjudication obligation. The one known drift is
outside the corpus — `docs/specs/module-decomposition.md`'s decision 7 interface
sketch — and it is already `runner-sketch-drift-k106`'s. The book neither waits
on it nor contradicts it.

**The corpus is frozen.** Do not edit `crates/keyed-launch/`. A defect found here
becomes its own leaf under the root brief's cross-book rule, placed ahead of
`architecture-residue-k75`; one commit carries the source change, every affected
ledger and page, and a green validator run over every book it touched, or the
leaf is deferred behind the books it would invalidate and says so.

## Decisions (running log)

**1 · One child per slice, and this session did the first.** The corpus is 2,073
lines over nine roots; chapter 7 alone owns 328 and chapter 6 271, and the
structure brief's three prose obligations — adjudicate the claim, carry the
through-line, and carry the load where the source does not — make the
prose-to-source ratio at least `grove-llm`'s, whose 1,017 lines took seven
sessions. Ten sessions cost less than one long one that degrades by chapter 5.
The order is not this session's to choose: `--through` proves a canonical prefix,
so children run in page order, and the closing `assembly` page owns no source and
is final-only. Rejected: one session for the whole draft; and pairing chapters to
make six children, which would have put `templates.rs`'s four-way split across
two sessions that each had to hold the other's blocks in mind.

**2 · Slice IDs are the structure brief's, block IDs are this session's, and the
two namespaces stay separate.** The ten slices come from the brief's *Chapter
sequence* table unchanged (`understands-neither` … `assembly`); no slice token
equals any page ID, which is the property the separate slice domain exists for.
The twenty top-level block IDs are named for what the block *is* rather than for
its chapter or its line range, so a later refinement into descendants reads as a
narrowing of a named thing: `manifest-three-dependencies`, `library-root`,
`two-opaque-errors`, `vocabulary`, `template-shapes`, `templates-load`,
`resolution-and-expansion`, `reading-and-whole-document-validation`,
`node-and-template-rules`, `word-scanning`, `diagnostics`, `templates-keys`,
`argv`, `channel-production`, `channel-inline-tests`, `launch-shape`,
`watch-and-launcher-signals`, `terminal-and-spawn`, `supervise-and-escalate`,
`conformance`. Their roots, owners and ranges are the brief's *Top-level
ownership blocks* table verbatim; nothing here re-decides a boundary.

**3 · The nine roots are declared in the book's conceptual order, not the
filesystem's.** `source-crate-manifest`, `source-library-root`,
`source-error-types`, `source-vocabulary`, `source-templates`, `source-argv`,
`source-channel`, `source-run`, `source-conformance` — chapter order, which is
`src/lib.rs`'s own section order with the vocabulary moved ahead of the two
documents. Manifest root order is what the source index, the ownership table and
the fragment index all sort by, so declaring them alphabetically would have made
every derived table read in an order the book never uses. `src/argv.rs` sits
sixth because chapter 5 owns it, not because it sorts there.
