# overview-structure-k29

## Goal

Elicit, from the human, the structure brief for the system overview: what the
overview is *for*, its conceptual order, how much of `docs/ARCHITECTURE.md`'s
descriptive account it absorbs and in what shape, and what a reader should be
able to do at the end.

## Context

- Decision 15 of `plan-k1`: every book takes a human-authored structure brief;
  the source does not carry audience, conceptual order or emphasis. Do not invent
  one.
- The overview is the odd deliverable of the set. Its source corpus is the
  smallest — `crates/grove`, 3 roots and 204 lines — but it also absorbs the
  descriptive half of a 1,700-line architecture document (decision 5 of
  `plan-k1`): runtime flow, command surfaces and module seams. The proportion is
  what the human has to settle: a source-exact walkthrough of 204 lines with a
  large descriptive account around it, or a descriptive system account with the
  binary's own source as one chapter.
- What is already settled and must not be re-elicited: the audience (decision 7),
  the depth (decision 1 — complete source-exact over the corpus), the corpus (the
  root brief's table), and the method.
- The other books will link into this one, and the user guide is the reader's
  entry point ahead of it. Where the overview sits between the guide and the
  per-crate books is a structure question, not an authoring one.

## Done when

- A committed structure brief states, in the human's own words: the overview's
  purpose and reader outcome, its chapter sequence, which architecture material
  it takes and how it is reshaped, and what it deliberately leaves in
  `docs/ARCHITECTURE.md`.
- It is specific enough that `architecture-move-k31` can decide clause by clause
  what moves without a second interview.
- `bash scripts/check.sh` passes.

## Notes

**HITL: the loop stalls here by design.** If the human is not available, stop and
say so.

**The ownership table is the constraint on the answer.** `docs/ARCHITECTURE.md`'s
*Documentation ownership* section fixes one canonical source per subject and
bounds what may sit directly under `docs/`. An overview that duplicates a subject
another row owns is not a structure choice the human can make; say so if the
conversation heads there.

## Decisions (running log)

**1 · The spine is the call, and the moved description hangs off the call site
that provokes it.** Each chapter opens at one of the binary's own steps and the
absorbed architecture material is the body of the chapter whose step reaches it.
Chosen over a descriptive system account with the source as one chapter, and over
a two-part book (source-exact part, then source-less description). Both
alternatives put the moved prose in chapters owning no source, and a slice owning
no `[[block]]` is not a `--through` value — so the whole descriptive half would
have had no scoped proof checkpoint and would be proved, if at all, only by the
final run. The spine is also recovered rather than imposed: 97 of the corpus's
204 lines are comment prose, and every thesis in them is about what the binary
declines to do.

**2 · The overview takes system-level description only; crate-internal
description stays where it is and leaves with its own book.** `docs/ARCHITECTURE.md`'s
*Task-tree data model* (374 lines), *Task kinds and composition* (447), *Lifecycle
and resumption*, *Human authority and completion*, *Version-control seam* and *How
the methodology reaches a session* describe `grove-loop`'s and `jj-workspace`'s
internals, whose canonical sources are those crates' own books. Moving them into
the overview would make the overview canonical for a subject another book is
about, which the *Documentation ownership* rule does not allow, and the link
contract forbids the later book citing the overview to repair it. Rejected:
moving everything now (~1,480 lines, and the ownership collision is real, not
notional) and deleting what a later book will re-derive (leaves the repository
with no prose account of the task tree between this leaf and
`grove-loop-book-k37`).

**The cost is accepted explicitly and is not this leaf's to repair.** Both the
root brief's and `crate-books-k14`'s *Done when* say `docs/ARCHITECTURE.md`
carries decisions, constraints and measurement records **only**, and attribute it
to `architecture-move-k31`. Under this decision k31 cannot deliver that alone: the
end state is reached when the last crate book lands. See decision 9.

**3 · The clause test is a positive move test, and the default is to stay.** A
clause moves iff it states what the system does, is shaped like, or reaches —
something checkable against the code as it stands. Everything else stays,
*including anything the session cannot classify confidently*. A sentence that
describes and justifies at once is split where the halves stand alone and stays
whole where they do not. The ground is asymmetric reversal cost, not taxonomy: a
clause moved into the overview acquires a page ID and an ownership block in a
fragment-validated book, and pulling it back means editing a proved book and its
ledger; a clause left in `docs/ARCHITECTURE.md` moves later for the price of an
edit. Rejected: the mirror-image rule (stay iff it names a rejected alternative,
a ground, a constraint or a measurement — default moves), which reaches
decisions-only faster but puts every uncertain clause in the expensive document;
and paragraph granularity, which is cheaper to review but wrong in one direction
for every mixed paragraph, and this document's paragraphs are long and often
mixed.

**k31's own task file overstates its case and the brief corrects it.** It calls
*Runtime flow*, *Command surfaces* and *Main module seams* "descriptive
throughout". Under decision 3 each keeps a real residue: *Runtime flow* keeps the
two-advisory-steps record; *Command surfaces* keeps "**Twelve**, not thirteen",
"Two entries are gone rather than renamed", and the discoverable-API ground;
*Main module seams* keeps the visibility constraint, the no-service-layer ground
and the `loop-crate-verbs-k21` record.

**4 · Anchors stay with the residue, and each stripped section opens with one
forward pointer.** No citation is re-pointed, in Rust or in Markdown. This was
settled by measurement rather than by preference: every real design citation of
an architecture anchor points at an argument, and arguments stay. Of the 39
`docs/ARCHITECTURE.md#…` citations in Rust sources, `library-refusals` (12) and
`tree-access-lock` (8) are in sections decision 2 leaves entirely;
`task-kind-taxonomy` (2), `symmetric-vcs-rule` (2), `cli-binary-split`,
`no-migration` and `pruning` all sit above surviving argument —
`cli-binary-split`'s single citation in `crates/grove-llm/src/cli.rs` cites the
audience-split *decision*. The two that appear to cite description,
`#runtime-flow` and one `#task-kind-taxonomy`, are positive controls inside
`crates/grove/tests/reference_navigation.rs` asserting the resolver works.
Rejected: anchors following the description into the book — it would re-point
citations that are not pointing at description, and would move them into a
namespace no check resolves, since `architecture-anchors-k19`'s Rust scan reads
`docs/ARCHITECTURE.md#` only and the repository sweep is Markdown-only. Also
rejected: anchors stay with no pointer, which leaves a reader arriving from a
source comment holding an argument with its subject removed.

**5 · Five chapters, and each moved section has exactly one home.**

| # | Page | Owns | Lines | Absorbs |
|---|---|---|---:|---|
| 1 | Orientation — the binary that selects nothing | `Cargo.toml` 1–54 | 54 | six packages; the two products |
| 2 | The surface — two binaries, one of them empty | `cli.rs` 1–19 | 19 | *Command surfaces* |
| 3 | Three steps — resolve, lease, run | `cli.rs` 20–53, `main.rs` 1–13 | 47 | *Runtime flow* |
| 4 | Proving a negative — the closure tests | `cli.rs` 54–137 | 84 | — |
| 5 | What the call reaches — the module map | *(none; final-only)* | 0 | *Main module seams* |

Total owned 204, partitioning all three roots exactly. Chapter 4 exists because
the test module is 41% of the corpus and carries the book's thesis — a closure
property rather than a list of rejected flags — so it is a chapter rather than a
subsection. Rejected: four chapters folding the surface into the call (chapter 2
would carry ~55 lines of moved prose from two different sections, losing the
one-section-one-home property), and six chapters splitting the tests across two
chapters to sit beside what each constrains (chapter 2 would own two
non-adjacent blocks and send a reader backwards through the file once).

**6 · The stated outcome is the thin-entry-point test.** At the end the reader
can take an entry point in their own code, ask what is left for an argument to
select, and tell a binary that is thin from one that merely looks thin — naming
which of three mechanisms holds it: a package boundary the compiler enforces, a
closure property a test asserts, or a convention nobody checks. All three are
provable inside the 204 lines: `Cargo.toml`'s crate-not-a-`[[bin]]` argument,
`the_human_command_surface_has_nothing_left_to_select`, and
`the_human_facing_binary_describes_every_option_it_lists`. Uniform with
`jj-workspace`'s transferable-test outcome and with its refusal of the
reference-card one. The locate-and-route outcome — name the package and module
owning any behaviour — follows for free from chapter 5's module map and from
source-exactness, and is therefore not the *stated* goal. *Read one run end to
end* was rejected by fact rather than preference: decision 2 leaves the
description of what happens inside `grove_loop::run` in `docs/ARCHITECTURE.md`
until the `grove-loop` book lands, and the link contract forbids the overview
citing it, so the promise would exceed what the book can deliver.

**7 · One invocation, two endings.** Bare `grove` in a grove working tree,
carried through four chapters; the fifth owns no source and does synthesis, as
`jj-workspace`'s seventh does.

| Chapter | Required anchor | Start and observable end |
|---|---|---|
| `01-orientation.md` | `one-invocation` | The carried invocation at low resolution: `grove` at a workspace root, through lease and launch, to one session started and the loop continuing. |
| `02-the-surface.md` | `worked-argv` | The same invocation's argv: `grove` alone; `--help` and `--version` stopping before the flow; `grove --harness claude` refused, with the text clap renders. The twelve-verb catalogue follows it. |
| `03-three-steps.md` | `worked-run` | The same invocation at full resolution: `current_dir`, `Workspace::resolve`, `DriverLease::acquire`, `TemplateSource::from_env`, `grove_loop::run`, and both endings — `Finished`/`Stopped` to `Ok(())`, and `Interrupted(SIGTERM)` through `reraise` to a wait status of `128 + 15`. The two error endings `run()`'s own doc comment names are the same trace stopping earlier. |
| `04-proving-a-negative.md` | `worked-assertion` | The two tests run against a `Cli` that has grown a flag: what `undescribed` collects and what the assertion prints. |

The second ending is the reason for the choice: it is the most argued and least
obvious claim in the corpus (`cli.rs` lines 29–36) and a reader would not guess
it. Rejected: two refusals as the carried trace — the happy path is the whole
point of the binary and would become the aside, and chapter 4 would break the
carry because neither closure test is about an error path; and the
self-referential run against this grove's own tree, whose named task file is
retired and renamed within days of the book landing, so the book would carry a
value that goes stale by design.

**The catalogue rule bites twice.** No section earlier than a chapter's worked
example may primarily enumerate three or more public verbs, operations or
obligations. Chapter 2's twelve `grove-llm` verbs and chapter 5's twelve-module
table are both catalogues and both follow their chapter's example — chapter 5
having none, its table may open the page.

**8 · Four guide anchors, five glossary anchors, two of them new.**

    [guide] docs/USAGE.md
      usage-running-grove       README.md reader contract — the required citation
      usage-tree-verbs          ch.2, beside the twelve verbs
      usage-driver-lease        ch.3, beside DriverLease::acquire
      usage-session-lifecycle   ch.3, beside grove_loop::run

    [[glossary]] CONTEXT.md
      driver-lease              exists (line 408)
      stated-vcs                exists (line 718)
      loop-control-channel      exists (line 357)
      guaranteed-core           NEW — promote the bold term at line 77 to a
                                `###` heading and anchor it
      task-tree-scheme          NEW — line 765 is already a `###` heading and
                                needs only the anchor line

`usage-running-grove` is the required `README.md` citation because it is exactly
this book's subject; the contract requires a book whose subject is a Grove
runtime crate to link the guide from that one place, with an anchor. The two new
glossary anchors are work **outside** the book and are
`overview-book-k30`'s: a declared anchor the target does not carry is `M201`
against the manifest whether or not a page cites it, so both must land before the
book validates. `CONTEXT.md` carries its terms as bold paragraphs and only four
have been promoted to `###` headings — the four `jj-workspace-book-k25` needed —
so a new anchor means promoting a term, not just adding a line. Rejected: the
zero-external-work set (the two terms the book most depends on would be the two
it navigates to worst — a bare link into a 1,046-line glossary), and the wide set
adding `usage-workspace-layouts`, `usage-finish`, `session-epoch` and
`session-kind`, which buys navigation the five chapters do not need at the price
of three more promotions.

**9 · The residue is one leaf, cut now, positioned last.** `architecture-residue-k75`
is appended at the end of `crate-books-k14` with `--kind impl`: after every book
has landed, it deletes the crate-internal description each book has made
redundant and reaches the decisions-only end state both briefs state. The
mechanism is deletion rather than a second move, because a crate book re-derives
its crate's account from source and does not absorb this document's prose. Cut
now rather than later because the question is precisely stateable now, which is
`decompose.md`'s test; positioned last because `pick` is a pre-order walk and
position is the only schedule there is. Rejected: correcting the task files and
cutting nothing (the obligation would live in prose the walk never reaches — the
same failure `walkthroughs-k9` found once already, a *Done when* assigned
nowhere); and folding the deletion into each book's `proof` stage (widens four
extracted kind charters to fix one document, against the editorial ADR's rule
that a stage's last act is cutting the next stage).

**Consequential edits, made this session.** `crate-books-k14`'s *Done when*
reattributes the end state to k75; its *Pointers* bullet on the descriptive
sections now names the three in play and points at the brief's rule; and
`architecture-move-k31`'s *Context* and *Done when* are corrected so it executes
the settled rule rather than eliciting one and no longer demands an end state it
cannot reach.

**10 · The books do not link into the overview, and the brief's third ground is
struck.** `walkthrough-books.md`'s link contract closes a book's permitted local
targets to its own pages, its own source roots, the `[guide]` path and each
`[[glossary]]` path; any other repository artifact is `M201`. So
`crate-books-k14`'s "because the other books link into it" cannot be true, and it
is removed from the brief, leaving the two grounds that hold. The overview is
designed as a book, not a hub; `docs/ARCHITECTURE.md`'s ownership table is the
hub, every book root is already owed a row in it, and a test holds that. Rejected:
amending the specification to admit a third citable document class (a spec, a
schema and a validator change, and it weakens the closed set that makes a book's
outbound coupling countable); and borrowing the `[[glossary]]` slot, which
`check_glossary` at `crates/book-validation/src/manifest.rs:1014` would accept
today — a manifest that satisfies the checker while contradicting the document
the checker enforces is the narrowing failure the corpus rule's three
anti-narrowing rules were written against.

**11 · The module map names and explains nowhere here.** Chapter 5 keeps
*Main module seams*' three columns — package, module, responsibility — in that
document's own words, and states once, not per row, that each row's *how* is its
crate's own book, three of which are not written; exactly one row is a module
this book explained. Rejected: names only (absorbing a section down to its column
headings is close to not absorbing it, and it costs the locate-and-route outcome
decision 6 said follows for free); and a book path per row (three of five do not
exist, inline code is opaque to the link scanner, so the result is forward
references nothing reports when they stay wrong — the unchecked-obligation shape
the outbound-link contract was written to remove).

**12 · Slice IDs are named for the mechanism.** `compiler-held`, `no-arguments`,
`one-call`, `closure-proved`, `assembly` — each the mechanism its chapter
establishes, so the slice list reads as the spine and as the three-way proof of
decision 6's outcome, and no token equals a page ID (`orientation`,
`the-surface`, `three-steps`, `proving-a-negative`, `what-the-call-reaches`).
The rule — no Grove task key, differing from page IDs by construction — is
`jj-workspace-structure-k17`'s decision 8 and is applied rather than re-elicited.
Rejected: naming for the corpus region (`manifest`, `grammar`, `entry`,
`assertions`), which stops the slice list carrying the spine and reuses two words
`CONTEXT.md`'s collision table already keeps apart.

**The test seams, put to the human and confirmed.** `book-check --final --check
all` by discovery from `scripts/check.sh`;
`every_books_subject_is_exactly_the_specifications_inventory` and
`every_books_corpus_exceptions_are_exactly_the_specifications_inventory`
(`crates/grove/tests/corpus_exception_inventory.rs`);
`every_book_root_has_a_documentation_ownership_row`,
`user_documentation_references_resolve`,
`every_repository_markdown_reference_resolves` and
`every_architecture_anchor_citation_in_a_source_resolves`
(`crates/grove/tests/reference_navigation.rs`); `book-check`'s `M201` against the
manifest for the two new glossary anchors; and the `include_str!` reads in
`crates/grove-llm/tests/composition_guidance.rs`, which go red on k31's move by
design. One gap stated rather than blurred: nothing mechanically checks decision
3, and the brief's *Stated limits* says so.
