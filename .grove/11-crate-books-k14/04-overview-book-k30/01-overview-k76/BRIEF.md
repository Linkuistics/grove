# overview-k76 — brief

## Goal

Draft the system overview to green final validation: a complete, source-exact
walkthrough of `crates/grove`'s three roots and 204 lines under
`docs/walkthroughs/overview/`, in the five-chapter shape the structure brief
settled, uniform with the `jj-workspace` and `ordinal-fs-tree` books.

## Context

- **The structure brief is `docs/specs/overview-book-structure.md`**, and it is
  the precondition `grove-draft` requires: it states who the reader is and the
  thin-entry-point outcome (*Audience and intended outcome*), the ordered
  section plan (*Chapter sequence* and *Concept and seam responsibilities*), and
  what deserves emphasis and what the book does not cover (*What each chapter's
  prose owes*, *What the book deliberately does not cover*). Its ownership
  mapping is the manifest's `[[block]]` table; where the two disagree it is a
  defect in one of them.
- The corpus, exactly: `crates/grove/Cargo.toml` (54), `crates/grove/src/main.rs`
  (13), `crates/grove/src/cli.rs` (137). `cli.rs`'s `#[cfg(test)] mod tests`
  is inside a root and is reconstructed. `crates/grove/tests/` is evidence.
- The contract is `docs/specs/walkthrough-books.md`; the validator is
  `book-check`, run through `--through <slice>` per child and `--final` by the
  last.
- Method: `linkuistics:writing-code-walkthroughs`. The draft owns structure and
  technical truth as obligations: every claim about the binary is checked
  against the binary, not against memory or an earlier page.

## Done when

- `docs/walkthroughs/overview/` holds the book and final validation over it
  passes with no deferred holes: 3 files, 204 resolved lines, `final=true`.
- `README.md` cites `docs/USAGE.md#usage-running-grove`, and every anchor the
  manifest declares exists in its target in the explicit form.
- `every_repository_markdown_reference_resolves`,
  `every_book_root_has_a_documentation_ownership_row` and the corpus-inventory
  tests pass; `bash scripts/check.sh` passes, the book gated by discovery.
- The last child's last act is `grove-llm leaf-add overview-book-k30 overview
  --kind copy-edit`, unless a live later sibling under `overview-book-k30`
  already holds that stage.

## Decomposition

Five children, one per slice, in canonical page order — the only order the
scoped validator accepts, since `--through` proves a prefix.

1. `orientation-k77` — slice `compiler-held`, `01-orientation.md`, owns
   `manifest-thin-by-construction` (`Cargo.toml` 1–54). Carries the whole
   book's scaffolding: the complete manifest, `README.md`, both indexes, every
   source-root directive, the full ownership ledger with four defers, and the
   two obligations outside the book (the `CONTEXT.md` anchors and the
   ownership row).
2. `the-surface-k78` — slice `no-arguments`, `02-the-surface.md`, owns
   `surface-grammar` (`cli.rs` 1–19).
3. `three-steps-k79` — slice `one-call`, `03-three-steps.md`, owns
   `entry-point-three-steps` (`main.rs` 1–13) and `surface-resolve-lease-run`
   (`cli.rs` 20–53).
4. `proving-a-negative-k80` — slice `closure-proved`, `04-proving-a-negative.md`,
   owns `surface-closure-tests` (`cli.rs` 54–137).
5. `what-the-call-reaches-k81` — slice `assembly`, `05-what-the-call-reaches.md`.
   Owns no source and is final-only: it takes the book to green **final**
   validation and a green `bash scripts/check.sh`, and is the only child whose
   `Done when` carries either.

**Every child but the last leaves `scripts/check.sh` red on `book-check`, and
that is the shape rather than a lapse.** The script runs `--final` over every
book root by discovery, so the book is inside the gate from the moment child 1
created it, while a prefix deliberately leaves later blocks deferred. Each child
proves itself with `book-check --through <its slice> --check all` and the rest
of the script's checks, and says so.

## Notes

**This is the draft stage only.** Copy edit, art and proof are the later stages
under `overview-book-k30`, cut lazily; a draft that has been polished leaves the
next stage's empty result unreadable. Figures are drawn where the prose contract
requires a relation to be drawn, and left to `art` otherwise.

**Do not move `docs/ARCHITECTURE.md`'s prose.** That is `architecture-move-k31`'s,
after this node. Leave each chapter shaped for what it will receive — the node
brief's *Pointers* say which section receives which — and write nothing that
would duplicate the moved sentences when they arrive. Cite `docs/ARCHITECTURE.md`
by backticked path, never by link: the outbound-link contract permits only the
guide and the glossary.

**The corpus is frozen.** Do not edit `crates/grove/`. A defect found here
becomes its own leaf under the root brief's cross-book rule; the first is
`manifest-function-count-k82`.

## Decisions (running log)

**1 · One child per slice, and this session did the first.** The corpus is
204 lines, but the structure brief requires every chapter to adjudicate — name
the behaviour, the test and the rejected alternative for every argued claim — and
to carry the through-line across three roots, so the prose-to-source ratio is
at least `jj-workspace`'s, and chapter 1 alone is 390 Markdown lines over 54
source lines. Five sessions cost less than one long one that degrades by chapter
4, where 84 lines of tests carry the book's thesis. Rejected: one session for
the whole draft.

**2 · Page titles are the short form** — *Orientation*, *The surface*, *Three
steps*, *Proving a negative*, *What the call reaches* — and each chapter's
subtitle from the structure brief is its opening section's heading. The H1 must
equal the manifest `title` and the navigation labels reuse it, so a long title
would repeat on every page twice.

**3 · Chapter 1 partitions the manifest into eight literals along its own
comment blocks**: the package block, the three theses (the human's binary, a
crate not a `[[bin]]`, no `[lib]` plus the target), the dependencies, the
tests-live-here comment, the dev-dependencies and the lints. Blank lines lead
the fragment that follows them, as in `jj-workspace`'s manifest chapter.

**4 · The early-use anchors are fixed now, one of them on a page that does not
exist.** `grove_loop::run`'s first use is `01-orientation.md#the-binary`; the
four `grove-loop` types on `cli.rs` line 2 have their first use at
`02-the-surface.md#the-imports`, so `the-surface-k78` must carry an explicit
anchor of exactly that name on the section that reads lines 1–2. The manifest
is complete from the first slice and the ledger carries all five rows
`pending`.

**5 · The two new glossary anchors are taken minimally, as `jj-workspace-book-k25`
took its four**: `guaranteed-core` promotes the bold term to a `###` heading
with the anchor line before it, and `task-tree-scheme` adds only the anchor line
because the heading exists. `book-validation` accepts an anchor only when the
next line is a heading, so an anchor before a bold paragraph would be `M201`.
Rejected: promoting more terms, which is `glossary-anchors-k62`'s.

**6 · A count the source does not bear out is recorded on the page and cut as a
leaf, not fixed.** The manifest's *three functions long* matches no natural
count of the crate's functions. The page states the structural fact beside the
fragment and reproduces the comment as written; `manifest-function-count-k82`
is cut beside the other frozen-root follow-ups, after every crate book, because
the fix must land in one commit with the page that quotes it.

**7 · `driver-lease` is cited at its first use, in chapter 1's worked example,
as well as in chapter 3 where the structure brief places it.** The glossary
contract permits a citation from any page at the first use of a term the book
does not own; a reader meets the lease in the low-resolution trace two chapters
before the page that owns `DriverLease::acquire`.

**8 · Chapter 1 names `Workspace` only as *the type that `cli.rs` calls
`Workspace`*, and names no other `grove-loop` type**, so that no early-use row
has its first use moved back to chapter 1. `LoopOutcome` in particular is
avoided: the trace says *run returns `Ok(())`* and *the loop reports why it
stopped* without naming the enum.

**9 · Chapter 2 partitions its block into four literals along the file's own
structure**: the two imports (1–2), the doc comment with its leading blank line
(3–7), the derive and `#[command]` attributes (8–18), and the empty struct
(19). The composite `surface-grammar` carries them in file order. Blank lines
lead the fragment that follows them, as in chapter 1.

**10 · The early-use anchor `the-imports` carries a table, not four
sentences.** Four types with one statement each is a mapping the prose
contract's *Figures* rule says to draw, and the table is also where the ledger's
four `pending` statements are stated verbatim, so a reader and the validator
read the same words.

**11 · The worked example is measured, not rendered from convention**, and the
measurement corrected one claim: `name = "grove"` fixes the `--version` line,
not the usage line, which takes argv[0]. The page says which transcripts the
guide also carries and which one it does not.

**12 · A stale claim in the `grove-llm` corpus is adjudicated on the page and
cut as a leaf.** The matching version attribute in `crates/grove-llm/src/cli.rs`
says the package carries its own `0.1.0`; its manifest inherits the workspace
version. Chapter 2 states the two mechanisms that actually hold the numbers
equal and does not repeat the stale one. `grove-llm-version-comment-k83` sits
beside `manifest-function-count-k82`, after every crate book, under the root
brief's cross-book rule.

**13 · The twelve-verb catalogue names and groups; it does not argue.** The
reasons the agent surface has the members it has — *twelve, not thirteen*, the
two retirements — are the residue `architecture-move-k31` leaves in
`docs/ARCHITECTURE.md`, and the page names that document by path and stops.

**14 · A second stale version claim sits in this book's own corpus, and the
draft adjudicates it rather than repeating it.** `cli.rs` line 12 says *every
member takes `version.workspace = true`*; `book-validation` does not. Chapter 2
states the narrower invariant that holds — every crate on the path from `grove`
to `grove-loop` inherits one version — beside the fragment, and chapter 1's
sentence was narrowed to match in the same session, which is the draft's own
technical-truth charter and not a later stage's. `every-member-version-comment-k84`
is the leaf, beside k82 and k83.

**15 · For `three-steps-k79`: the `TemplateSource` early-use statement says the
loop re-reads it *once* per iteration; `crates/grove-loop/src/loop_driver.rs`
loads it twice inside the loop body, once before the transition and once
after.** The structure brief and the manifest carry the *once* wording, and
chapter 2 uses a count-neutral form at the anchor. Chapter 3 owns the rows and
the type: verify the count against the source, and if *twice* is right, amend
the manifest statement, the ledger row and the brief's table in one commit.
