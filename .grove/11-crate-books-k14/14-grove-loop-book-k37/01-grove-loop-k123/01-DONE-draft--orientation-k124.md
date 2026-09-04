# orientation-k124

## Goal

Create the `grove-loop` book and prove its first slice: `allowed-to-mean`,
`01-orientation.md`, owning the whole of `crates/grove-loop/Cargo.toml` (59) and
`crates/grove-loop/src/lib.rs` (377) — 436 lines.

## Context

- Draft stage, child 1 of 7 of `grove-loop-k123`. The structure brief is
  `docs/specs/grove-loop-book-structure.md`, and chapter 1's responsibilities are
  its *1 · Orientation — `allowed-to-mean`* section: the crate's own statement
  that it is the one library crate in the workspace allowed to be domain-bound;
  the four dependencies each read as the evidence, with the imposed set being
  exactly `libc`; from `lib.rs` the module list, `VERSION`, the *Opening mirrors
  the store's* and *Three shapes* theses, the one-error rule, and the
  `<worktree>/.grove` join that means no caller can spell the root a second way.
- **It carries three things no later chapter returns to:** how this book's spine
  differs from `jj-workspace`'s and `keyed-launch`'s; the *lock in the signature*
  claim, which is compiler-enforced and proved in chapters 5–13; and the
  `Key`/`Entry` collision `CONTEXT-MAP.md` keeps apart by hand.
- **It adjudicates the `version.workspace` claim.** `src/lib.rs` lines 67–68 say
  every member takes `version.workspace = true`; `crates/book-validation` does
  not. The paragraph is written to be uniform with the overview's chapter 2,
  which adjudicates the identical sentence at `crates/grove/src/cli.rs` line 12.
  `every-member-version-comment-k84` holds the fix and lands after this book.
- The first slice carries the book's scaffolding under
  `docs/specs/walkthrough-books.md`, *Authoring workflow and scoped proof*: the
  complete manifest, `README.md`, both lookup indexes, all thirteen source-root
  directives, the full ownership ledger with a defer for each of the
  thirty-seven later-owned blocks, and every early-use row `pending`.
- **The one obligation outside the book is this child's**: `docs/ARCHITECTURE.md`'s
  *Documentation ownership* table gains its `grove-loop` row, in the wording the
  brief's *The book's row in the ownership table* carries. No glossary promotion
  is owed — all twelve declared anchors already exist in explicit form, so `M201`
  is green from the first run.
- The carried example is one grove's whole life in the names the code itself
  writes, anchored on `01-requirements--plan-k1.md` — which `root_init` produces
  because `DEFAULT_ROOT_SLUG` is `"plan"` and `requirements` is a reserved kind.
  Chapter 1's step is the crate's own map: `Cargo.toml` and `lib.rs` in, and the
  twelve verbs, four dependencies and one error named out.
- **The prose obligation for this chapter is *do not restate*.** `Cargo.toml` is
  59% comment prose and `lib.rs` 58%; the comments already argue and the
  fragments quote them verbatim.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  allowed-to-mean --check all` is valid: 13 files, 436 resolved lines, 10,097
  deferred, `final=false`.
- `every_repository_markdown_reference_resolves`,
  `every_book_root_has_a_documentation_ownership_row` and the corpus-inventory
  tests pass with the new book root present.
- `scripts/check.sh` is red on `book-check`, which is the shape a prefix leaves,
  and on the two forward-commitment tests in
  `crates/grove/tests/corpus_exception_inventory.rs`, which creating the book
  directory emptied of subjects. `forward-commitment-tests-k131` holds the
  second, inserted ahead of the rest of the book; decision 10 records why it is
  not fixed here.

## Notes

The worked example fixes the values every later chapter reuses. They are the
crate's own: `01-requirements--plan-k1.md`, the slug `plan`, the key `1`, the
handle `plan-k1`, and `<worktree>/.grove` as the only spelling of the root.

## Decisions (running log)

**1 · This leaf opened two nodes before it drafted anything.**
`grove-loop-book-k37` became a node holding the draft stage
(`leaf-decompose … --kind draft`), because the four editorial kinds are installed
— `~/.config/grove/config.kdl` declares `draft`, `copy-edit`, `art` and `proof` —
and a book leaf authored by hand wastes the pilot that earned them. The draft
leaf `grove-loop-k123` then became a node of seven children, cut along the
structure brief's five parts with chapter 1 and the assembly page standing alone:
the brief's *Chapter sequence* says the parts *are* the draft stage's session
groupings and that this leaf opens the node and cuts its draft work along them.
Rejected: twenty-one chapter children cut here, which is the granularity the
sessions will end at but puts twenty-one bodies into the session that also has to
write the whole book's scaffolding, and settles each part's internal seam from
outside it; and one child per part with no separate orientation child, which
would have made the scaffolding a side-effect of a 2,150-line drafting session.
Each part is expected to decompose in its turn, one child per chapter — which is
what the four preceding books did at the chapter granularity directly.

**2 · The harness did not have `grove:grove-draft`.** The Claude Code plugin
cache this session loaded predates the four stage skills, as it did at
`orientation-k92` and `orientation-k108`. The skill was read from
`plugins/grove/skills/grove-draft/SKILL.md` and the editorial family file from
`plugins/grove/skills/grove/references/editorial.md` in the working tree, with
`grove:grove` and `grove:grove-impl` from the cache. Not escalated:
`overview-book-k30` decision 1 already flagged it to the human.

**3 · The manifest declares five dependencies, and the page says five.** The
structure brief says *the manifest's four dependencies each carry their reason in
situ* and names `anyhow`, `libc`, `keyed-launch` and `ordinal-fs-tree`; the
`[dependencies]` table has five entries, the fifth being `jj-workspace`
(`crates/grove-loop/Cargo.toml` line 28), which six modules of the crate reach
directly. The manifest's own comment is not wrong — it accounts for the table as
*the three modules it composes* plus `anyhow` and `libc`, which is five — so there
is no corpus defect and no adjudication is owed. The page states five and says
which four carry a clause of their own. Rejected: repeating the brief's four,
which would have made the page's own enumeration disagree with the fragment
directly beneath it. `structure-brief-dependency-count-k132` was cut and inserted
ahead of the rest of this node to correct the brief, because nothing goes red on
this and a later stage reading the brief against the page could otherwise correct
the page in the wrong direction.

**4 · Slice IDs are the structure brief's, block IDs are this session's, and the
two namespaces stay separate from the page IDs.** The twenty-one slices come from
the brief's *Chapter sequence* table unchanged, and the thirty-nine top-level
block IDs are named for what each block *is* rather than for its chapter or its
line range. Three were renamed off their first spelling to keep the fragment
namespace clear of the page namespace — `walk-selection`, `decompose-production`
and `loop-driver`, where `the-walk`, `leaf-to-node` and `the-loop` are page IDs.
Every block's root, owner and range is the brief's *Top-level ownership blocks*
table verbatim; nothing here re-decides a boundary. The arithmetic closes: nine
blocks over `task_name.rs`, ten over `task_tree.rs`, nine over
`tree_lifecycle.rs`, two over `driver_lease.rs` and one each over the remaining
nine roots, partitioning 10,533 lines.

**5 · The thirteen roots are declared in the book's conceptual order, not the
filesystem's.** Manifest root order is what the source index, the ownership table
and the fragment index all sort by, so the declared order is the order a reader
meets the crate in: the manifest and the library root, then the grammar, the
walk, the growing, the lifecycle, the surface, and the driver half. That is the
brief's *Top-level ownership blocks* table order.

**6 · Chapter 1's two roots are partitioned along the files' own blocks —
twenty-five literals under two composites.** `Cargo.toml` in six (package
identity 1–10; the four-dependency comment and table 11–30; the extracted tree
and the imposed-set claim 31–38; dev-dependencies 39–41; lints 42–44; release
45–59), split so that the `ordinal-fs-tree` line and its comment read as their own
decision rather than as a fifth row of the table above them. `lib.rs` in nineteen:
five for the module header's own paragraph breaks, then one per declaration group
— modules, `VERSION`, imports and exports, the two aliases, the two enums,
`TreeWrite`'s header and its `impl`, the two openings, the join, `Reference` and
its `Display`, `Selection`, and the error type and its four trait
implementations.

**7 · The early-use ledger is sixteen rows, thirteen of them this chapter's, and
the ledger is a floor rather than the manifest's list.** The structure brief
mandates three rows whose first use is on chapters 2, 3 and 5; those are in the
manifest and in the ledger unchanged. Chapter 1 additionally names the cast at low
resolution, and every name in that export list whose owner is a later chapter
takes a row of its own — thirteen rows grouped by owning slice, each with the
minimum statement the page carries verbatim at `#the-cast`. `TaskName` therefore
appears in two rows: this chapter's, for the export list, and the brief's, for
chapter 2's first substantive use. The alternative — leaving chapter 1's forward
references out because the brief did not enumerate them — would have made the
ledger a copy of the manifest rather than the book's own record, which is the
reading the specification rejects.

**8 · Two symbol families could not be spelled the way the brief words them.**
`valid_symbol_family` (`crates/book-validation/src/ledger.rs`) requires the
symbols cell to be a comma-and-space separated list of single backticked tokens
with no inner backticks, so the brief's *and the `verdict` / `entry` /
`malformed` support helpers* and *and `TaskName`'s `Display`* are `F009` as
written. They are recorded as the individual tokens instead — six for the first
family, two for the second. The families are unchanged; only the spelling is the
validator's. This is noted in the draft node's brief so chapters 2, 3 and 4 use
the same spelling.

**9 · The one obligation outside the book is discharged, and no glossary
promotion was owed.** `docs/ARCHITECTURE.md`'s *Documentation ownership* table
gains the `grove-loop` row in the wording the brief's *The book's row in the
ownership table* carries, placed after the `keyed-launch` row.
`every_book_root_has_a_documentation_ownership_row` passes. All twelve declared
anchors — four in `docs/USAGE.md`, eight in `CONTEXT.md` — already exist as
explicit `<a id="…"></a>` lines, so `M201` was green from the first run.

**10 · Creating the book directory turned two repository tests red, and the leaf
that settles them was inserted ahead of the rest of the book.**
`the_inventory_covers_books_that_have_not_been_written_yet` and
`the_subject_inventory_covers_books_that_have_not_been_written_yet`
(`crates/grove/tests/corpus_exception_inventory.rs`) each assert that some
inventory row names a book with no directory under `docs/walkthroughs/`.
`grove-loop` was the last one, so the set emptied the moment this slice created
the directory. Nothing is wrong: the property had a stated end and reached it,
and the specification's prose promises the same check. Not fixed here — the
choice between retiring the pair with the specification clause and narrowing them
is not this leaf's, and `crates/grove/tests/` is outside the corpus either way.
`forward-commitment-tests-k131` holds it, inserted ahead of the remaining
twenty draft sessions so none of them re-diagnoses the same two failures.

**11 · The leaf's one in-session reviewer was spent on the page's factual core,
and it paid.** `book-check` proves the bytes the page reproduces and proves
nothing about the claims around them, so a fresh context was given the five book
files, the two source roots, the structure brief and the shared contract with a
*find what is wrong* brief, explicitly barred from re-checking the fenced source
bytes. It returned eighteen findings, and every one was re-verified against the
source before anything was changed.

**Fourteen were valid and are fixed.** Five were counts or attributions that did
not survive recomputation: `root_init` writes a charter **and** a leaf, not one
file, and the page's own early-use row already said so; `verbs::root_init` takes
a slug unconditionally, so *with no slug override* named a mechanism that is
`default_root_slug`'s a level above it; the openings section called `Reading`,
`Writing` and `TreeWrite` type aliases when the file declares three aliases, two
enums and a struct; the module header carries five passages, and one sentence
still said four; and `crates/grove-loop/tests/` is six files and 3,457 lines, not
the five and 3,442 the structure brief and this book's own README carried — the
sixth is a fifteen-line support module. Two were claims about where a test lives:
`one_process_creating_and_reading_a_grove_never_waits_on_itself` is an **inline**
test at `src/tree_lifecycle.rs` line 1385, inside the block chapter 11 owns and
reproduces, and the page had filed it under `tests/` as evidence. Three were the
page contradicting itself or the brief: it called `stale_cross_refs` a verb two
sections after establishing that it is not; it claimed the book's chapter order
is the module header's order, which the table beneath it refutes; and it spoke of
*Part IV*, *Part V* and *the two Part boundaries* — the brief has five parts and
they are draft-session groupings that take no slice ID and appear in no contents
page, so the reader had never been shown the structure being named. Three were
scope: the export-list paragraph claimed every name in the block belongs to a
later chapter, when four belong to no chapter of this book at all, and the four
names it listed as exceptions are declared further down the file rather than in
that block; and the manifest comment's four reasons are carried by two comments
in two fragments, not one. One was the `walkthrough.toml` corpus note, where six
roots carry a `#[cfg(test)] mod tests` and five of those modules are inline.

**Two were valid and became a leaf rather than a fix**, because they are stale
claims inside the frozen corpus. `Cargo.toml`'s dependency comment says `libc` is
the contention probe in `task_tree` — it is also nineteen calls in
`driver_lease` and three in `loop_driver` — and that `keyed-launch` is reached by
exactly one verb, which is true of the verb surface and not of the crate, since
`driver_lease`, `loop_driver` and `session_config` all reach the runner. Both
clauses predate `loop-crate-driver-k22`. The page adjudicates them beside the
fragment that reproduces them and repeats neither;
`manifest-dependency-clauses-k133` holds the source fix, deferred behind this
book because the bytes are chapter 1's, and it also corrects the structure
brief's *No third stale claim was found*.

**One was a departure from the brief and is fixed by removal.** Chapter 1 cited
three declared anchors the brief assigns to other chapters —
`docs/USAGE.md#usage-tree-verbs` to chapter 15, `CONTEXT.md#task-tree-scheme` to
chapters 2–4 and 11–13, and `CONTEXT.md#tree-access-lock` to chapter 5. Nothing
was red, because every anchor is declared and exists. The three links are gone
and the prose that named the concepts stays, so chapter 1 cites no outbound
anchor at all — which is what `keyed-launch`'s chapter 1 does, and the required
guide citation is the `README.md`'s.

**One is a visible trade-off and is kept.** The reviewer found that several
early-use rows name `#the-cast` as their first use where the symbol appears
earlier on the page — in the vocabulary sentence, in the header fragments'
own bytes, and in the verb table. The clearest case is fixed: the opening
section now names grove's vocabulary as words rather than as types. The rest is
the shape all four preceding books took, and the reason is the reader's: the
ledger sends someone to the one anchor where every minimum statement is written
down, and the fragments that mention a name in passing state nothing to send them
to. The obligation the specification actually imposes — that the minimum be
stated at the anchor the row names — is met verbatim for all thirteen rows.

**Two were noise.** The manifest declares three `[[early-use]]` rows and the
ledger carries sixteen, which the reviewer read as a disagreement; the
specification says the opposite in as many words — *authors add further rows to
the book's own ledger; the manifest states the ones a book may not omit*. And the
prose-contract sweep flagged *the layer that stayed*, *their mirror* and *the
cast*: the first two are the structure brief's own words for this crate's
relation to `jj-workspace`'s and `keyed-launch`'s books, and *The cast* is the
section heading all four preceding orientation chapters use.

**Nothing the reviewer checked and passed was re-checked here.** It listed some
forty verified claims — every fragment range and their tiling of both roots, the
eleven-module and fourteen-`pub fn` counts, the nine occurrences of the carried
filename, the six-of-seven version inheritance, the 3,984 inline-test lines, the
39 ownership rows and the 10,533-line total, and the four link targets with their
explicit anchors — which is the part of the page the validator and the earlier
enumerations had already made cheap to get right.
