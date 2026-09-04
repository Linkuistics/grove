# grove-loop-structure-k36

## Goal

Elicit, from the human, the structure brief for the `grove-loop` book: audience
refinements, conceptual order, what deserves emphasis, and what the reader must
be able to do at the end. Commit it as an input artifact.

## Context

- Decision 15 of `plan-k1`: the source does not carry audience, conceptual order
  or emphasis, so every book takes a human-authored structure brief. **Do not
  invent one.** This leaf exists so that no session has to.
- Already settled, and not to be re-elicited: the audience (decision 7 — knows
  Rust and jj, has driven a grove, grove vocabulary linked to `CONTEXT.md` and
  never re-taught), the depth (decision 1 — complete source-exact, every byte in
  a fragment graph), the method (`linkuistics:writing-code-walkthroughs`, whose
  eight-field intake `plan-k1` already answered), and the corpus.
- The corpus, exactly, from the root brief: 13 roots, 10,533 lines — every
  `crates/grove-loop/src/**/*.rs` plus `crates/grove-loop/Cargo.toml`. `tests/` is
  evidence, not a root.
- This is 72% of the remaining corpus, and the largest structural conversation in
  the campaign. `src/tree_lifecycle.rs` is 2,725 lines, `src/task_tree.rs` 2,023,
  `src/task_name.rs` 1,714 and `src/driver_lease.rs` 1,383.
  `src/task_grow/tests.rs` (1,680) is excluded by the root brief as an inline test
  module rather than production source.
- `CONTEXT-MAP.md` records that `grove-loop` **is** the grove context: kind,
  handle, brief chain, outcome, selection and finishing all live here, and
  `CONTEXT.md` is already their glossary. It takes the store's `Key` and `Entry`
  from `ordinal-fs-tree` and adds `Kind`, `Handle` and `Outcome` beside them — a
  collision the context map keeps apart by hand, and one the book must not blur.
- Expect this book to need many chapters. Its structure brief should say where
  the chapter boundaries fall, because the authoring leaf will decompose along
  them.
- The books already written are the shape to react against, and the pilot's
  `jj-workspace` book is the closest precedent for a new one.

## Done when

- A committed structure brief states, in the human's own words: the chapter
  sequence and what each chapter is for, the concepts that carry the book, what
  to emphasise and what to pass over, and the reader-facing outcome.
- It is specific enough that the authoring leaf needs no second interview.
- `bash scripts/check.sh` passes.

## Notes

**HITL: the loop stalls here by design.** An agent answering its own structure
questions has broken the distinction this leaf exists for. If the human is not
available, stop and say so.

**Structure, not prose.** The deliverable is the book's shape and emphasis, not
draft text; drafting is the authoring leaf's, through the pipeline.

## Decisions (running log)

The deliverable is
[`docs/specs/grove-loop-book-structure.md`](../../docs/specs/grove-loop-book-structure.md),
1,018 lines, which carries each decision below in its settled form. This log
records what was put to the human, what was chosen, and what was rejected with
why. Seven questions, one per prompt, in the order the four preceding
structure-brief interviews used; every fact below was measured or read before its
question was framed, and none of them was asked.

**0 · The corpus was closed arithmetically before anything else.** 12,154 lines
of `crates/grove-loop/src/**/*.rs`, less the 1,680 of the excluded
`src/task_grow/tests.rs`, plus `Cargo.toml`'s 59 = **10,533** over 13 roots,
matching the root brief exactly. Three further measurements framed the whole
interview: **3,984 lines (38%) are inline `#[cfg(test)] mod tests` inside five
roots**, and the specification's exception inventory carries one `grove-loop`
row, so all of them are corpus; comment prose is 35% overall but splits into
production halves at 41–51% and test halves at 3–20%, with `driver_lease.rs`'s
production at **12%**; and **thirty-one** of `docs/ARCHITECTURE.md`'s forty-one
residue markers name this crate.

**1 · The spine is what is left here is grove's own.** Every chapter opens on
what this module kept when the domain-free crates took the rest, and why it could
not move. Recovered from eight places, not one: `lib.rs` lines 4–10 (*the one
library crate in the workspace that is allowed to be domain-bound … none of them
has a word for a* kind, *a* brief chain, *an* outcome, *a* handle *or*
finishing), `Cargo.toml` lines 12–16, and the opening paragraph of every large
module — `task_name` (*the whole seam*), `task_tree` (*What changes is who owns
the walk*), `task_grow` (*What grove still owns, and why each piece could not
move*), `tree_lifecycle` (*what happens to a grove that the store has no word
for*), `session_config` (*What is left here is the part that is grove's alone*),
`verbs` (*none of which the store has a word for*), `driver` (*the reason they
are not in `crate::verbs`*), `driver_lease` (the seam owns *where*, grove owns
*whose*), `loop_driver` (***All of that is `crates/keyed-launch`'s, not this
module's***). Pinned by `the_library_imposes_only_libc` and
`every_consumer_takes_the_library_with_default_features_off`. **Checked against
the other books' spines, as k34's lesson requires:** three of the four existing
books are refusal-shaped, and this one is their deliberate mirror — they say what
they refuse, this says what nobody else could take, because meaning is the one
thing a domain-free crate cannot hold. Chapter 1 states that difference in a
sentence and no later chapter returns to it. Rejected: **the lock a verb needs is
visible in its signature** (`lib.rs` 17–41), the crate's sharpest and only
compiler-enforced claim, but reaching 5,629 lines and missing the entire driver
half, which has no locks and no `Tree` in its signatures — it is chapter 1's
stated claim and chapters 5–13's proof; **the tree's shape is the only state**,
reaching 76% but making `driver_lease.rs` — 1,383 lines, the deliberate
counterexample — open by contradicting the spine, so it is chapters 13 and 16's
thesis instead; and **a library of verbs and, since `loop-crate-driver-k22`, a
driver**, the crate's true structural fact but one boundary, giving two rules
rather than a rule per chapter — the ground on which `keyed-launch` rejected *the
two halves meet only at `Argv`*. It is stated in chapter 1 and is the Part
IV/Part V boundary.

**2 · Twenty-one pages, cut by concept, with each root's inline tests travelling
with the concept they prove.** Twenty own source and a final-only assembly page
owns none. The order is the seam order: orientation, the grammar (3), the walk
(6), what the store has no word for (4), the surface (1), the driver (5).
Every block boundary was verified with `sed -n` on the exact lines and moved back
where necessary to the first line of an item's own doc comment, and the mapping
closes: 436 + 451 + 563 + 700 + 290 + 370 + 322 + 428 + 613 + 518 + 612 + 775 +
808 + 530 + 516 + 819 + 564 + 358 + 245 + 615 + 0 = 10,533. Four roots split
across chapters — `task_name` three ways in nine blocks, `task_tree` five ways in
ten, `tree_lifecycle` four ways in nine, `driver_lease` two ways in two — the
first three because the files are ordered by Rust convention and the book by
concept, the fourth because that root's two halves need opposite prose treatments
and a chapter cannot carry both instructions. **Two blocks were placed while the
brief was written rather than during the interview, and both removed an early use
rather than creating one:** `task_tree.rs` 1997–2023 to chapter 8 (it exercises
`brief_chain`), and `tree_lifecycle.rs` 1467–1665 to chapter 14 (they are the
finish tests). Rejected: **fifteen pages, production split from proof per root**,
which gives every chapter one contiguous block but makes four chapters *the tests
for the previous chapter* — a cut by file half, with chapter 8 at 1,649 lines and
no rule of its own; **eleven pages, whole files**, leaving a 2,725-line chapter;
**twenty-one pages in the reader's order**, which is `docs/USAGE.md`'s order and
the way a grove runs, but has every early chapter using vocabulary the book has
not defined, and a seam is only visible once both sides are named. **Two books**
were rejected on a fact: the specification's subject inventory carries one row
for this crate, both halves live side by side in `src/`, so no subject path
separates them, and the inventory test's own narrowing attack case exists to
reject exactly that.

**3 · The carried example is one grove's whole life, in the names the code itself
writes.** Anchored at `01-requirements--plan-k1.md` — which `DEFAULT_ROOT_SLUG`
and the reserved `requirements` kind make `root_init`'s exact output, which
appears nine times in the inline tests, and which is the first entry of the tree
this book was written inside. The brief tabulates the anchor, start and
observable end for all twenty-one pages, and every lifecycle step has a named
test. The observable end is the grove ceasing to exist, which is also why the
book does not point at `.grove/` and tell the reader to look. The second ending
is **a name grove refuses** — a task-*shaped* name the store would have accepted,
rejected by the grammar that did not move
(`pick_refuses_a_species_mismatch_at_a_task_shaped_name`,
`a_session_kind_that_is_not_a_token_is_malformed`,
`transition_does_not_scaffold_over_a_name_grove_refuses`). Rejected: **the
crate's own test fixtures verbatim**, where every step would be a named passing
test but the fixtures are ad-hoc per test — `a`/`b`, `design`/`build`, `add` —
built and discarded inside each test because each isolates one property, so no
thread runs through them (`keyed-launch` rejected the same shape); **one
session's life, driver-side**, which is the crate's reason and carries chapters
15–20 better than anything, but reaches none of the 7,436 lines before them, so
it is chapter 20's thesis; and **this repository's own `.grove/` quoted as of a
commit**, disqualified by chapter 14 — `finish-commit` deletes the grove, so
twenty chapters of illustration would point at nothing in the repository a reader
clones.

**4 · The stated outcome is the what-could-not-move test.** The reader can take
any layer split out of a system into a domain-free library and ask *what stayed
behind, and why could it not go?* — answered in three parts, each with a cost and
named tests, all provable inside the corpus: on the way in, the **names** (a
grammar you own must be canonical, and canonicity costs a conformance kit —
chapters 2–5); on the way through, the **preconditions** (checked against the
same snapshot the operation then plans from, and a refused run consumes nothing —
chapters 6–13); on the way out, the **policy** (stated where a reader can find
it, and restating nothing the layer above owns — chapters 14–20). Coverage is all
twenty source-owning chapters with no gaps, and chapter 21 is the application.
Rejected: **the re-derivation test**, sharp and explaining the crate's strangest
fact — that a crate whose thesis is *the tree is the only state* holds 1,383
lines of untracked-file locking — but reaching roughly 3,700 lines only by
restatement; it is chapters 13 and 16's thesis and their contrast is one sentence
in chapter 1. **The one-place-that-means test**, whose evidence lives largely
outside the corpus, in `CONTEXT-MAP.md` and the other five crates, and which no
test in this crate's suite holds — the `Key`/`Entry` collision it names is still
handled, as chapter 1's and chapter 6's obligation. **The maintainer outcome**,
which follows free from source-exactness, is not transferable and leaves chapter
21 nothing to argue; all four preceding briefs rejected the equivalent.

**5 · The third prose obligation is to say what each reproduced test establishes,
and what it would still pass under.** Measured first, and the measurement is why:
production halves argue for themselves at 41–51% everywhere except
`driver_lease.rs` at **12%**, while the test halves run 3–20% — and under decision
2 those 3,984 lines, 38% of the corpus, land on the page beside the concept they
prove. A `#[test]` body shown without prose teaches the mechanics and not the
claim, and a test *name* is a label rather than an argument. So the instruction
is stated per **block** and differs three ways: **supply the claim** for every
inline test block, including what would have to be true for the test to pass
while the property was broken; **supply the argument** for `driver_lease.rs`
1–819, per mechanism naming the enforcing line, the failure prevented and the
record clause kept — the same instruction `keyed-launch`'s chapters 3–5 carry for
`templates.rs` at 13%; and **do not restate** across the production blocks at
41–73%, where the comments already argue and the graph quotes them verbatim. The
two shared obligations — adjudicate the claim, carry the through-line — are
carried unchanged. Rejected: **name the owning side at every fragment**, which
the spine already does once per chapter and the outcome once per part, is
repetition at fragment granularity, is nearly content-free across chapters 16–20
where there is no seam underneath, and says nothing about the corpus's dominant
measured fact; and **hold the records to the code**, which would make k75's
deletion a check rather than a judgement but is roughly half-discharged by
comments that cite records already, and which the book **cannot cite** under the
link contract, so it would be held in prose no link-checker can verify.

**6 · Twelve anchors, all of which exist today, and no promotion.** `[guide]`
`docs/USAGE.md` with `usage-task-tree`, `usage-tree-verbs`, `usage-driver-lease`
and `usage-finish`; `[[glossary]]` `CONTEXT.md` with **all eight** anchors the
document carries. The required `README.md` guide citation is `usage-task-tree` —
*The task tree and its filename grammar* — which states this crate's subject in
the reader's own terms and is reserved by no other book. All eight glossary
anchors are over vocabulary `CONTEXT.md`'s own ownership table assigns this
crate, which is twelve terms; `grove-llm`'s book already declares seven of the
eight on weaker grounds. `M201` is green from the first slice and nothing outside
the book is owed on their account. **No promotion, and the reason is a fact:**
`task-tree-scheme` is a **grouping** heading (`CONTEXT.md` 769–770) introducing a
run of term entries — `glossary-anchors-k62`'s own context names it as one of two
such groupings — so citing it already lands the reader in the passage defining
*Node directory*, *Session kind*, *Work-item handle* and the rest, and chapters
2–4 need no target that does not exist. Rejected: **promoting `session-kind` and
`work-item-handle`** into anchored `###` headings, which `glossary-anchors-k62`
expressly permits (*a book reserving a new term simply anchors that term as
`orientation-k55` did*) but which buys granularity the book does not need at the
price of a `CONTEXT.md` restructuring on a book leaf already carrying 21 chapters
— and k62 sits after every book precisely so it can do it uniformly against the
full reserved set; **a two-anchor guide set**, which under-declares against
chapters 14 and 19 and would have the crate `CONTEXT.md` names as owner of twelve
terms reserving fewer glossary anchors than `grove-llm`, which the same table
says adds no word; and **`usage-running-grove` and `usage-session-lifecycle`**,
which are `keyed-launch`'s and the overview's and describe the operator's
experience of the loop rather than anything this crate owns.

**7 · The test seams, put to the human and not objected to; slice IDs and
omissions applied from precedent.** Seams, all existing: `book-check --final
--check all` over `docs/walkthroughs/grove-loop/` by discovery from
`scripts/check.sh` line 78;
`every_books_subject_is_exactly_the_specifications_inventory` and
`every_books_corpus_exceptions_are_exactly_the_specifications_inventory`, **both
green with no specification edit** — the subject row exists and the book declares
exactly the one exclusion the inventory carries, which is what puts the other five
inline `mod tests` inside the corpus;
`every_book_root_has_a_documentation_ownership_row`,
`user_documentation_references_resolve` and
`every_repository_markdown_reference_resolves`, the last of which also sweeps this
brief's own links; and `M201` against the manifest. **One edit outside the book
is owed** — the ownership row, which is `grove-loop-book-k37`'s as each precedent
row was its book leaf's. Slice IDs are named for the rule each chapter carries —
`allowed-to-mean`, `four-verdicts`, `the-handle-not-the-position`,
`canonical-or-nothing`, `one-spelling-of-grove`, `paths-are-built-here`,
`first-live-leaf`, `root-to-leaf`, `wider-than-a-key`,
`what-the-library-cannot-see`, `never-mistaken-for-finished`, `the-key-survives`,
`marked-in-place`, `the-tree-deletes-itself`, `twelve-not-fourteen`,
`one-per-working-tree`, `which-calls-are-admitted`, `whose-file-and-whether`,
`too-late-to-say-later`, `four-things-a-runner-cannot-choose`, `assembly` — none
equal to a page ID and none carrying a task key, applying
`jj-workspace-structure-k17`'s decision 8 rather than re-eliciting it. Stated
limit: nothing mechanical checks the spine, a worked example's start and end, or
the per-chapter prose obligation, and the crate's 3,442-line integration suite
plus 3,984 lines of inline tests are the book's evidence rather than its gate.
Rejected: **subject-named slices**, which map one-to-one onto modules but make the
book's index look like the crate's module list — the shape sequence 2 was rejected
for — and would make this the only book whose slices read differently from the
convention k17 settled for all of them.

**Counts were enumerated rather than asserted.** *Twelve verbs* is `lib.rs`'s
claim and `verbs.rs` declares **fourteen** `pub fn`; the two that are not verbs
are `stale_cross_refs`, a lint that follows an insert, and `signal_channel`, a
question a caller must ask *before* the write rather than learn from it — each
says so in its own doc comment, and `driver.rs` holds two further tree operations
that are not verbs and states that counting them *would say the surface has
fourteen verbs, which it does not*. Chapter 15's slice is named for that triple.
Likewise the residue count: thirty markers are spelled `residue(grove-loop…)` and
a thirty-first, at line 1324, is spelled `residue(jj-workspace, grove-loop)`, out
of forty-one in the document.

**Consequential edits, made this session.** `grove-loop-book-k37`'s task file now
names the brief by path — `grove-draft` stops without a named artifact — and
carries the five things it would otherwise have to rediscover: the five draft
groupings and their line counts; that 3,984 lines of inline `mod tests` are
corpus and the manifest declares exactly one exclusion; the two known-false claims
it must adjudicate rather than repeat or correct, with the leaves that fix them
sitting after it; that the ownership row is its one obligation outside the book
and no glossary promotion is owed; and that it is the last condition on
`architecture-residue-k75`.

**Externalised: nothing.** No defect was found that a leaf does not already hold.
The two stale claims in this corpus belong to `every-member-version-comment-k84`
and `template-source-read-count-k86`, both already cut and both correctly placed
after the book they would invalidate; `grove-llm-dependency-comments-k102` reaches
`crates/grove-loop/src/lib.rs` line 81 only as evidence and changes nothing here.
