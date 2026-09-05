# grove-loop-k123 — brief

## Goal

Draft the `grove-loop` book to green final validation: a complete, source-exact
walkthrough of the crate's thirteen roots and 10,533 lines under
`docs/walkthroughs/grove-loop/`, in the twenty-one-page shape the structure brief
settled, uniform with the `overview`, `grove-llm`, `jj-workspace`,
`keyed-launch` and `ordinal-fs-tree` books.

## Context

- **The structure brief is `docs/specs/grove-loop-book-structure.md`**, and it is
  the precondition `grove-draft` requires. All three things it must state are in
  it, quotable: who the reader is and what they can do afterwards (*Audience and
  intended outcome* — a reader who knows Rust and jj and has driven a grove,
  whose outcome is **the what-could-not-move test**, three questions with a cost
  and a named test each); the ordered section plan (*Chapter sequence*, twenty-one
  pages of which twenty own source, and *Concept and seam responsibilities*, one
  subsection per page); and what deserves emphasis and what the book does not
  cover (*What each chapter's prose owes*, *The spine*, *What the book
  deliberately does not cover*). Its ownership mapping is the manifest's
  `[[page]]` and `[[block]]` groups; where the two disagree it is a defect in one
  of them, not a licence to prefer either.
- The corpus, exactly, per root: `Cargo.toml` (59), `src/lib.rs` (377),
  `src/task_name.rs` (1,714), `src/task_tree.rs` (2,023), `src/task_grow.rs`
  (518), `src/tree_lifecycle.rs` (2,725), `src/verbs.rs` (363), `src/driver.rs`
  (57), `src/complete.rs` (96), `src/driver_lease.rs` (1,383),
  `src/session_config.rs` (358), `src/prompt.rs` (245), `src/loop_driver.rs`
  (615) — 10,533 lines over thirteen roots. `crates/grove-loop/tests/` is 3,442
  lines over five files and is evidence, not roots.
- **Thirty-eight per cent of the corpus is inline test code and none of it is
  excluded.** Five roots carry a `#[cfg(test)] mod tests`: `tree_lifecycle.rs`
  1,649 lines, `task_tree.rs` 1,008, `task_name.rs` 694, `driver_lease.rs` 564,
  `loop_driver.rs` 69 — 3,984 in all. The manifest declares exactly **one**
  `[[corpus.exclude]]`, `crates/grove-loop/src/task_grow/tests.rs` class
  `inline-test-module`, matching the specification's inventory; declaring any
  other turns
  `every_books_corpus_exceptions_are_exactly_the_specifications_inventory` red.
- The contract is `docs/specs/walkthrough-books.md`; the validator is
  `book-check`, run `--through <slice>` per child and `--final` by the last.
  Scoped proof exists so a partial book is provable — validate per slice as you
  go rather than discovering at the end that the graph does not close.
- Method: `linkuistics:writing-code-walkthroughs`. The draft owns structure and
  technical truth as obligations to discharge, not as things the brief handled:
  every claim about the crate is checked against the crate and its tests, not
  against memory, the decision records, or an earlier page of the same book.
- **The prose obligation splits three ways and it is this book's own**, set by
  measurement in the brief's *What each chapter's prose owes*: **supply the
  claim** at every inline test block (chapters 2, 3, 4, 6, 7, 8, 9, 11, 12, 13,
  14, 17, 20 — 3,984 lines at 14% prose), stating for each reproduced test both
  the property it establishes **and what would have to be true for it to pass
  while the property was broken**; **supply the argument** over
  `driver_lease.rs` 1–819 (chapter 16, 12% prose), naming per mechanism the line
  that enforces it, the failure it prevents and the record clause it keeps; and
  **do not restate** across the production blocks at 41–73%, where the comments
  already argue and the fragment graph quotes them verbatim.
- **Two claims inside the corpus are known false and are adjudicated on the
  page**, never repeated as true and never silently corrected: `src/lib.rs` line
  68 (*every member takes `version.workspace = true`*) at chapter 1, and
  `src/session_config.rs` line 89 (*the loop re-reads the configuration once per
  iteration*) at chapters 18 and 20, refuted by `src/loop_driver.rs` lines 241
  and 260 inside this same corpus. `every-member-version-comment-k84` and
  `template-source-read-count-k86` hold the fixes and both sit after this book.
- The `ordinal-fs-tree` glossary and grove's collide on *leaf* and *node* and
  differ on *ordinal* / *position* and *key* / *permanent key* (`CONTEXT-MAP.md`).
  A page speaking of both must say which tree it means, sentence by sentence.

## Done when

- `docs/walkthroughs/grove-loop/` holds the book and final validation over it
  passes with no deferred holes: 13 files, 10,533 resolved lines, `final=true`.
- `README.md` cites `docs/USAGE.md#usage-task-tree`, and all twelve declared
  anchors — four in the guide, eight in `CONTEXT.md` — exist in their targets in
  the explicit form today, so `book-check`'s `M201` is green from the first slice
  and **no glossary promotion is owed**.
- **The one obligation outside the book is discharged**: `docs/ARCHITECTURE.md`'s
  *Documentation ownership* table gains its `grove-loop` row, in the wording the
  brief's *The book's row in the ownership table* carries, so
  `every_book_root_has_a_documentation_ownership_row` is green. It belongs with
  the first slice's scaffolding, as each precedent book's row did.
- `every_repository_markdown_reference_resolves` and the corpus-inventory tests
  pass; `bash scripts/check.sh` passes, the book gated by discovery rather than
  by a hand-added line.
- The last child's last act is `grove-llm leaf-add grove-loop-book-k37 grove-loop
  --kind copy-edit`, unless a live later sibling under `grove-loop-book-k37`
  already holds that stage.

## Decomposition

**Seven children, cut along the structure brief's five parts, with chapter 1 and
the assembly page standing alone.** The order is forced: `--through` proves a
canonical prefix, so children run in page order. Each child covers a contiguous
run of chapters and is expected to become a node of its own, one child per
chapter, cut by the session that picks it — the brief's *Where the draft's own
node boundaries fall* leaves that to each part rather than settling it here.

| # | Child | Chapters | Lines | Cumulative resolved | Deferred |
|---:|---|---|---:|---:|---:|
| 1 | `orientation-k124` | 1 | 436 | 436 | 10,097 |
| 2 | `the-grammar-k125` | 2–4 | 1,714 | 2,150 | 8,383 |
| 3 | `the-walk-k126` | 5–10 | 2,541 | 4,691 | 5,842 |
| 4 | `no-word-for-k127` | 11–14 | 2,725 | 7,416 | 3,117 |
| 5 | `the-surface-k128` | 15 | 516 | 7,932 | 2,601 |
| 6 | `what-a-runner-cannot-k129` | 16–20 | 2,601 | 10,533 | 0 |
| 7 | `what-could-not-move-k130` | 21 | 0 | 10,533 | 0 |

Child 1 additionally carries the whole book's scaffolding — the complete
manifest, `README.md`, both lookup indexes, all thirteen source-root directives,
the full top-level ownership ledger with a defer for every later-owned block, and
the one obligation outside the book. Child 7 owns no source and is **final-only**:
it adds the synthesis chapter 21 is, closes the indexes, and takes the book to
green `--final` validation and a green `bash scripts/check.sh`.

**Every child but the last leaves `scripts/check.sh` red on `book-check`, and
that is the shape rather than a lapse.** The script runs `--final` over every book
root by discovery, so the book is inside the gate from the moment child 1 created
it, while a prefix deliberately leaves later blocks deferred. Each child proves
itself with `book-check --through <its last slice> --check all` and the rest of
the script's checks, and says so.

## Found while drafting

**The structure brief undercounts the dependencies, and the book says five.**
`crates/grove-loop/Cargo.toml` declares five — `anyhow`, `jj-workspace`,
`keyed-launch`, `libc` and `ordinal-fs-tree` — and four of them carry a reason in
situ; `jj-workspace` is covered only by the comment's grouping of *the three
modules it composes*. The brief says four in two places: its chapter 1 section
and its worked-example table. Chapter 1 states five, which is what the manifest
bears out. `structure-brief-dependency-count-k132` holds the correction and runs
before the rest of this node; until it lands, do not reconcile a page down to
four.

**Two symbol families the ledger's syntax will not carry as the brief words
them.** An early-use symbols cell is a comma-and-space separated list of single
backticked tokens, checked by `valid_symbol_family` in
`crates/book-validation/src/ledger.rs`, so prose inside the cell is `F009`. The
brief's *`TaskName`, `TaskNameError`, `Verdict`, and the verdict / entry /
malformed support helpers* is recorded in the manifest as those six tokens, with
the helpers named individually. Its *`TaskName::compose`, and `TaskName`'s
Display* was recorded as two tokens in one row until `display-first-use-k137`
split it into two rows of one token each — `` `TaskName::compose` `` at chapter 3
and `` `impl Display for TaskName` `` at chapter 2 — because they are two symbols
with two first uses rather than one family, and the structure brief now carries
them as separate rows too. **A family is the brief's and the spelling is the
validator's, but a family whose members have different first uses is not a
family** — splitting one is a correction to the brief, made in the brief, and not
a leaf preferring its own wording. Chapters 2, 3 and 4 must use the manifest's
spelling in their own ledger rows.

**An early-use row is owed by *naming or exercising*, and the manifest's rows are
a floor.** Promoted from `the-grammar-k125`, which settled it in
`display-first-use-k137` and wrote it into the structure brief's *Early uses the
order forces*: a reproduced block owes a row for a later-owned symbol it names or
exercises, unless an existing row already covers it (`entry_path`'s mandatory row
is the precedent for *names*). `floor-rows-chapter-two-k138` swept chapter 2 under
that criterion and added three rows the manifest did not carry. **Every remaining
child owes the same enumeration over its own blocks** rather than treating the
manifest's rows as the set — the manifest is the minimum a page must declare, not
the maximum.

**A family whose members have different first uses is not a family.** k137 split
the manifest's `` `TaskName::compose`, `impl Display for TaskName` `` row into two
rows of one token each, in the manifest, the structure brief and the ledger
together. That is a correction made in the brief, not a leaf preferring its own
wording; the syntax constraint is `valid_symbol_family` in
`crates/book-validation/src/ledger.rs`, which refuses prose inside the cell.

**This crate's comments miscount, misattribute, and outlive what they measured —
count and re-run rather than reproduce.** `the-grammar-k125` found three classes
of defect in `task_name.rs` alone. **Stale enumerations**: six of them, corrected
by `stale-enumerations-k139` — miscounted call sites and references, and a
`TaskNameError` variant `open-kind-k20` deleted that a comment still names.
**A fixture that had stopped posing its question**: the conformance kit's
`5-impl-domain-k29.md` became `Malformed(MissingSeparator)` when
`grammar-separator-k15` changed the grammar under it, so it was refused before
canonicity was asked and the comment's *measured, not reasoned* claim was no
longer true of the fixture it described — while the kit still reported both
*conforming* and *obligation exercised*. **A doc comment welded to the next
item**: a missing blank line between two `///` runs, so rustdoc rendered
`peel_key`'s paragraphs as `terminal_key`'s and left `peel_key`'s docblock empty.
`kit-fixture-and-peel-doc-k140` fixed the last two.

**A stale module name in four comments, and no leaf holds it.** Found by
`paths-k142`: `crates/grove-loop/src/task_tree.rs` line 1,070 says *production
never wants it: `llm_cli` holds one tree across `pick` and the ancestor walk*.
There is no `llm_cli` in this workspace; the code is
`crates/grove-llm/src/cli.rs`, whose `cmd_brief_chain` opens one tree and hands
the same guard to `leaf_in` and `verbs::brief_chain`. **The behaviour the comment
claims is correct and only the address is stale**, which is why chapter 6
adjudicated it on the page rather than cutting a leaf for it. Three other
comments spell it the same way and each belongs to a chapter still to be written:
`tree_lifecycle.rs` line 42 (chapter 14's block) and line 1,147 (chapter 11's),
and one in the excluded `task_grow/tests.rs` that chapter 10 may only cite. Each
of those chapters owes the same one-clause adjudication; if a session decides the
four are worth a source fix, `canonicalisation-sites-k149` is already touching
`task_tree.rs`'s comments inside the frozen line counts and can carry them.

**Three instruments, and none of them is reading the bytes.** A comment's *n
places / the two callers / the only X* is checked by enumerating the calls. A
comment claiming *disabling X leaves this green and that red* is checked by
performing the mutation in a copy of the workspace and watching the arm without
the subject stay green — a claim about a measurement is worth exactly the re-run.
A comment's *attachment* is checked from `cargo doc --no-deps
--document-private-items` and the rendered docblock, never from reading the
source, because nothing warns. Each defect is adjudicated on the page, fixed by
its own leaf, and — because the ledger holds an exact line count per block —
fixed **inside the existing line counts**, or every later block of the file
shifts and finished pages break.

**A fourth thing `cargo doc` sees, and a fifth instrument nobody runs.**
Promoted from `kind-and-briefs-k144`. Alongside a doc comment's *attachment*,
`cargo doc --no-deps --document-private-items` reports **unresolved intra-doc
links**, and this crate has five: `task_tree.rs` 580, 586 and 638 link to
`pick`, `select` and `kind`, which are `crate::verbs` functions the module does
not import; `prompt.rs` 28 links to `crate::methodology`; `lib.rs` 283 to
`Resolution::Ambiguous`. Nothing in `scripts/check.sh` runs `cargo doc`, so none
of this fails anything. **The prose around each link is true and only the link is
broken**, so the pages adjudicate and `unresolved-doc-links-k151` holds the fix,
deferred behind the book because one of the five is in an unwritten chapter's
block. Every chapter reproducing a doc comment should run the command over its
own block rather than reading the links.

**A refusal test names a clause; the assertion names which clause actually
fired, and only a mutation confirms it.** Promoted from `kind-and-briefs-k144`,
which found three of chapter 8's four refusal tests refusing somewhere other
than their names say — one at `is_file` twenty-nine lines before the clause it is
named for, one at the grammar arm rather than the containment check, one in
chapter 5's opening without reaching the function at all. The procedure that
settles it: read the `bail!` texts, match each assertion's substring to exactly
one of them, then replace each refusal in a copy of the workspace and watch which
tests fail — **including a control mutation of the ones you believe are covered**,
because that is what rules out a silent second observer and attributes each
failure to one test. In `leaf_entry` the result was five of seven refusals
unobserved by all 245 inline tests and all twenty-five `grove-llm` targets.
**Chapters 9 and 11 to 14 all own private helpers with many `bail!` arms**, and a
coverage sentence about any of them is worth exactly the same re-run.

**Count the opens before believing a test labelled *together*.** The file's
closing block is *pick + brief-chain together* and opens the tree twice, which is
the race both `pick_in`'s and `brief_chain_at`'s doc comments exist to warn
against. Any later chapter reproducing a test that composes two verbs owes the
same check.

## Notes

**This is the draft stage only.** Copy edit, art and proof are the later stages
under `grove-loop-book-k37`, cut lazily, each as the last act of the stage before
it. Figures are drawn where the prose contract requires a relation to be drawn,
and left to `art` otherwise; a draft that has been polished leaves the next
stage's empty result unreadable.

**Cite `docs/ARCHITECTURE.md`, `CONTEXT-MAP.md`, the specs and the decision
records by backticked path, never by link**: the outbound-link contract closes a
book's local targets to its own pages, its own roots, the guide and the glossary.
The brief's *The decision records themselves* names the records each chapter
keeps; a linkified one turns `M201` red.

**This book cannot link to another book.** Where `grove-loop` calls
`ordinal-fs-tree`, `keyed-launch` or `jj-workspace`, the page says what grove
asked for and what came back, names the crate in prose, and stops.

**The corpus is frozen.** Do not edit `crates/grove-loop/`. A defect found here
becomes its own leaf under the root brief's cross-book rule, placed ahead of
`architecture-residue-k75`; one commit carries the source change, every affected
ledger and page, and a green validator run over every book it touched, or the
leaf is deferred behind the books it would invalidate and says so.
