# plan-k1

## Goal

Establish what should be built, in the human's own words: a user guide and code
walkthroughs for all crates. Decide the depth, the set, the layout, the
assurance and the process, and either cut the tree or hand it to planning.

## Context

The grove was scaffolded with an empty brief and an empty task file — the only
input was the workstream name. Everything below was settled in one interview.

## Done when

Settled and recorded; the root brief written; the tree grown; the leaf retired
and committed.

## Notes

The interview ran the full grilling procedure: the open questions were far more
than three and their answers interdependent. Four decisions reversed mid-way,
which is the interview working — the questions triggered the human's own
reflection, and each reversal is recorded below in place rather than tidied
away.

The eight-field intake the `writing-code-walkthroughs` skill requires is
answered by decisions 1, 3, 6, 7, 9, 10 and 17 below. Downstream sessions read
it here rather than re-eliciting it.

## Decisions (running log)

**1 · Depth — complete source-exact.** First settled as orientation-level, with
no fragment graph; **reversed** to complete source-exact coverage, uniform with
the existing book, on the human's ground of uniformity. Every production byte of
the corpus belongs to a fragment graph and final verification proves
reconstruction.

**2 · The user guide is one document, expanded in place.** `docs/USAGE.md`
already owns the *Human workflow and commands* row; a second document on that
subject would violate the ownership table. The singular "user guide" against
plural "walkthroughs" in the goal statement reads the same way.

**3 · One root, and the existing book moves into it.** `docs/walkthroughs/`
holds every book. The relocation of `docs/ordinal-fs-tree/book/` costs a wide
mechanical rename — the validator's path constants, its test suite, the crate
README's invocations, and the context map's citation — and was chosen over two
roots anyway, for uniformity.

**4 · The fragment validator is generalised in place.** Its corpus and its
ledger become per-book configuration rather than compiled-in constants. It stays
under `crates/` for now; moving it to the walkthrough skill, so it works over
any repository, is deferred to its own workstream because `plugins/` has never
shipped a compiled artifact and that has no precedent to follow. Scope widened
when decision 1 reversed: the fragment half must be generalised too, not only
the structural half, which puts it on the critical path ahead of every book.

**5 · The overview absorbs the architecture document's descriptive half.**
Runtime flow, command surfaces and module seams move into the overview
walkthrough; the architecture document is left as decisions, constraints and
measurement records. Its anchors are cited from Rust source and tests, so the
move is guarded by a link-integrity suite and a compile-time content
assertion — the citations are re-pointed by turning a red test green, not by a
hopeful sweep.

**6 · The set is four new books, an overview, and the relocated one.**
`grove-llm`, `jj-workspace`, `keyed-launch`, `grove-loop`. `crates/grove` is
covered inside the overview, which absorbs command surfaces anyway.
`book-validation` is excluded: it is not part of the grove system and its home
is with the walkthrough skill.

**7 · The audience knows Rust and jj and has driven a grove.** Grove's
vocabulary is linked to the glossary, never re-taught. Entry point is the user
guide.

**8 · Assurance is machine-held wherever it can be.** The repository-wide link
sweep already covers new books for free. The validator is gated in the check
script across every book root; the book roots join the curated user-surface
list; each book earns a tested ownership-table row.

**9 · The corpus is frozen and nothing is fixed inline.** First settled as
"fix trivia inline, externalise the rest"; **reversed** once decision 1 flipped,
because the ledger holds an exact line count per source root and an inline fix
silently breaks a page an earlier session already proved. Every defect found
while documenting becomes its own leaf.

**10 · The book spec is split in place.** The existing book's spec becomes a
shared walkthrough-books spec — page conventions, navigation, prose contract,
audience, assurance, and the fragment language, which every book now uses — with
per-book ledgers as data rather than specification. Editing in place rather than
writing a second overlapping document is what the records doctrine prescribes.

**11 · The process is a publishing house, not a book-per-leaf loop.** Draft,
developmental edit, technical edit, copy edit, art, proof. The existing book has
no diagrams of any kind, so the art phase is new ground: a figure format, an
asset convention, and validator support that does not exist.

**12 · Pilot first, then extract, then scale.** `jj-workspace` — four roots,
698 lines, and a real external boundary — is authored with the stages run by
hand and recorded. The kinds and the loop are derived from what that measured.
Only then are the remaining books authored through the shipped pipeline.

**13 · The loop is modelled in grove, and its representation is the filesystem
view.** First settled as skills-only with no code; **reversed** — the construct
is modelled in grove because it has a representation to expose. The minimal UI
is the filesystem view and determining that shape comes first; a render verb or
a terminal surface is future work, not now. This reopens *one task is one
session*, *entries are never removed*, and the outcome-partition records.

**14 · The in-house prior art is harvested.** `Writegood`, `grove.gh-issue-12`
and `TheGreatExplainer` are all on disk and all on this problem. The last
specifies this exact pipeline; the first argues the ordering constraint that
measurement precedes machinery.

**15 · A human-authored structure brief is an input to every book.** The source
does not contain enough to structure a walkthrough. The manual form is available
today; how to capture and persist such input generally is research.

**16 · Capture and formalism are two layers.** Formal specifications are poor
human artifacts and good LLM artifacts, so the stakeholder-shareable capture
layer and the formal tier a coding phase consumes are separate. A notation
attempting both fails at both. This is the constraint the research pair is
commissioned against, alongside the doubt that the glossary works as a shareable
artifact at all.

**17 · Four products, three arms, and the documentation is never blocked.** The
books proceed on the manual structure brief while the research pair and the loop
construct run as their own arms and retrofit as they land.

**18 · This brief earns an adversarial read.** Seventeen decisions, four
reversed, executed by a long chain of sessions with no human present. A
`review-requirements` leaf is cut ahead of planning; it creates nothing if it
finds nothing.

## Amendments (`plan-k8`)

`plan-k2` read this log adversarially and reported nine findings; `plan-k8`
triaged them. **No decision above is reversed or rewritten** — the human's words
stand as recorded. What changed is the summary layer that was inconsistent with
them, and it changed in `.grove/BRIEF.md`. For a downstream session the brief is
now authoritative wherever it is more specific than a decision here.

- **Decision 6** stands: `book-validation` is excluded. The root *goal* said
  "every crate" and contradicted it; the goal now says *every Grove runtime
  crate* and records why the validator is outside the boundary.
- **Decisions 1, 6, 9 and 12** depended on a corpus nobody had written down.
  The 33 roots / 14,525 lines are now enumerated per deliverable in the brief's
  *Pointers*, with both exclusions named. The manifest field of the
  `writing-code-walkthroughs` intake is answered there rather than here.
- **Decision 5** claimed the architecture move is guarded because red tests
  re-point the citations. The repository-wide resolver reads Markdown only, and
  the one `.rs` check matches the `ADR <slug>` form — not the twenty-six raw
  `docs/ARCHITECTURE.md#<anchor>` citations in Rust sources. The move is
  mechanical *after* the resolver is widened; the brief now requires that first.
- **Decision 9** said a defect becomes its own leaf and left open what such a
  leaf may commit. The brief now carries the cross-book transition, taken from
  the contract the existing book already holds.
- **Decision 12** said "the kinds and the loop are derived from what that
  measured", which reads as gating scale-out on P3. The brief now gates it on
  P2 only, which is what decision 17's *retrofit as they land* already implied.
  Its measure is preregistered rather than reported, so a stage can fail.
- **Decision 2** settled the file, not the content. "Complete" is now a stated
  coverage inventory written before the guide is edited.
- **Decision 16** stands as the human's position, and is now marked in the brief
  as the arm's hypothesis rather than its premise — matching what k4/k5/k6 were
  already chartered to do. The brief's *Done when* gains P4's deliverable, which
  it lacked entirely.
- **Decision 18** is the eighteenth; the brief called this an adversarial read
  of seventeen. Corrected.

Two of `plan-k2`'s findings were narrowed rather than accepted whole. F5 argued
the research was commissioned against a fact it was supposed to test — but
`specification-capture-k4` already says *test that expectation rather than
assuming it* and k6 already owes a verdict, so only the root brief's wording and
its missing acceptance criterion were at fault. F2 argued the tree order cannot
deliver the scale-out without blocking documentation; against the actual
depth-first pre-order walk the current order is sound once scale-out is gated on
P2, so the ambiguity was fixed and **no leaf was moved**.
