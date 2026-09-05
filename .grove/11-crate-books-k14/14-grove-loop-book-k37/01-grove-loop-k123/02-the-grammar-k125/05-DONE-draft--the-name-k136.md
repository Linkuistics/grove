# the-name-k136

## Goal

Draft chapter 4 of the `grove-loop` book — *The name, and canonicity*,
`docs/walkthroughs/grove-loop/04-the-name.md` — prove the prefix through slice
`canonical-or-nothing`, and close `the-grammar-k125`.

## Context

- The last chapter of `the-grammar-k125`, the largest of the three at 700 lines,
  and the one that closes `task_name.rs`: `the-task-name` (`task_name.rs`
  591–1,020), `name-test-support-and-kit` (1,021–1,177) and
  `grammar-and-canonicity-tests` (1,200–1,312). After it, all nine of the file's
  blocks read `resolved`.
- The structure brief's section is *4 · The name, and canonicity*. The rule is
  `format(parse(f)) == f`, **or one entity occupies two files**. The types are
  `TaskName`, `TaskNameError`, the `EntryName` implementation that is the whole
  seam, and the helpers under it.
- **This chapter carries the conformance kit** — the library's own, run against
  grove's grammar — and the canonicity argument, whose stakes the module header
  states and which chapters 2 and 3 have already pointed here for: the withdrawn
  model was lenient on padding, accepted a hand-typed `5` and rendered `05`, so
  one entry could occupy two files sharing a key and a position.
- **The kit's fixture is the load-bearing part.** `listings()`'s last two entries
  — `5-impl-domain-k29.md` and `07-DONE-grove-flip-k28` — are the near-misses,
  and the comment above it records that this was measured rather than reasoned:
  disabling the domain's canonicity check leaves the kit green without them and
  red with them.
- **Every row owned by `canonical-or-nothing` closes here, and none opens.**
  Enumerate them in the ledger rather than trusting a count: as of
  `display-first-use-k137` they are six — chapter 1's `TaskName` cast row; the
  `TaskName` / `TaskNameError` / `Verdict` family with the `verdict` / `entry` /
  `malformed` helpers, and `TaskName::distinguished`, both at
  `02-the-tokens.md#the-four-verdicts`; the `a_kind` / `slug` helpers and
  `` `impl Display for TaskName` ``, both at
  `02-the-tokens.md#refusals-inside-the-shape`; and `` `TaskName::compose` `` at
  `03-kind-slug-handle.md#the-handle-is-the-identity`. `compose` and `Display`
  were one row until k137 split them, and chapter 3 may add more, so re-read the
  ledger. All move from `pending` to `explained` when this slice lands.
- **Prose obligation.** 430 production lines at 42% take *do not restate*; the
  270 lines of inline tests — the support block, the conformance kit, *the
  grammar* and *question 2: the grammar is canonical* — take *supply the claim*.
- The carried example's step is `01-requirements--plan-k1.md` parsed and rendered
  back byte-identical, or refused as uncomputable.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  canonical-or-nothing --check all` is valid: 13 files, 2,150 resolved lines,
  8,383 deferred, `final=false`.
- Chapter 4 exists, `README.md`'s contents entry and the navigation lines either
  side of it are updated, and **all nine** `task_name.rs` ownership rows read
  `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.
- `the-grammar-k125` has no live leaf left, and is closed as
  `references/retire.md` directs.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

**Do not cut a `copy-edit` leaf here.** This node is one part of seven under
`grove-loop-k123`; the pipeline's next stage is cut by the *last* child of the
last part, which is `what-could-not-move-k130`.

## Decisions (running log)

1. **The structure brief precondition is met, and the artifact is
   `docs/specs/grove-loop-book-structure.md`.** This leaf's parent brief names it
   by path, and all three things `grove-draft` requires are quotable in it: who
   the reader is and what they can do afterwards (*Audience and intended
   outcome*, and *The stated outcome: the what-could-not-move test*); the ordered
   section plan (*Chapter sequence*, twenty-one rows, and *Concept and seam
   responsibilities*, one subsection per page — chapter 4's is *4 · The name, and
   canonicity*); and what deserves emphasis and what is excluded (*What each
   chapter's prose owes*, *The spine*, *What the book deliberately does not
   cover*). No `BRIEF.md` was relied on for any of the three.

2. **Twelve early-use rows are owned by `canonical-or-nothing`, not the six this
   leaf's body predicted.** Enumerated from `source-index.md` rather than
   counted from the body, which said so itself: `TaskName` (chapter 1's cast
   row); `TaskName::distinguished`; the `TaskName` / `TaskNameError` / `Verdict`
   family with `verdict` / `entry` / `malformed`; `peel_key`; `split_shape`; the
   `a_kind` / `slug` helpers; `impl Display for TaskNameError`; `impl Display for
   TaskName`; `TaskName::Brief`, `TaskName::Positioned`; `TaskName::parse`;
   `terminal_key`; and `TaskName::compose`. `floor-rows-chapter-two-k138` and
   `kind-slug-handle-k135` added six of them after the body was written. All
   twelve move to `explained`.

3. **No new early-use row is owed by this chapter, and that was enumerated
   rather than assumed.** Every backticked identifier and intra-doc link in the
   three owned blocks was extracted and classified. The only later-owned referent
   named in the reproduced bytes is `resolve` (lines 996 and 1,000), and row
   `verbs::resolve`, `Resolution` at `01-orientation.md#the-cast` already states
   its minimum — the exception `display-first-use-k137` wrote into the structure
   brief, and the treatment chapter 3 gave the same clause. Everything else is
   either this chapter's own, an earlier chapter's, or `ordinal-fs-tree`'s, and
   the ledger is a ledger of *this book's* later slices.

4. **`the-grammar-k125` does not close at this leaf, and its `Done when`'s
   fourth bullet is stale.** `06-impl--stale-enumerations-k139.md` is live and
   sits after this leaf; `kind-slug-handle-k135` cut it after this body was
   written. No close cascade runs here.

5. **The conformance kit's fixture no longer holds the property its comment
   says was measured, and the page adjudicates it rather than repeating it.**
   `listings()`'s comment (lines 1,061–1,074) claims *disabling this domain's
   canonicity check leaves the kit green without these two entries and red with
   them*, citing `docs/formalism-findings.md` entry 020. Measured in a scratch
   copy of the workspace, not reasoned:

   | Arm | `listings()` | Mutation | `the_task_tree_domain_conforms` |
   |---|---|---|---|
   | baseline | all 8 | none | green |
   | A | all 8 | canonicity lines (882-888) removed | **green** |
   | B | first 6 | canonicity lines (882-888) removed | green |
   | control | all 8 + `5-impl--a-k1.md` | canonicity lines (882-888) removed | **red**, `TheGrammarIsCanonical` |

   The control is the one that makes the clean read evidence: the instrument can
   go dirty, and it does, on a fixture that reaches the check. The two near-misses
   do not reach it. `5-impl-domain-k29.md` carries a single `-` where the grammar
   has required `--` since `grammar-separator-k15`, so it is
   `Malformed(MissingSeparator)` before canonicity is asked; entry 020 was
   written against the pre-separator grammar, where that same name parsed and
   rendered `05-impl-domain-k29.md`. `07-DONE-grove-flip-k28` is
   `Malformed(NodeWearsOutcome)`, and stays Malformed under the canonicity
   mutation too — a separate mutation removing the node-outcome guard turns it
   into `Malformed(NotCanonical)`, which is canonicity working as the backstop
   *inside the domain* and still invisible to the kit, because the kit's listings
   check only inspects names that reach `Verdict::Entry`.

6. **Lines 977–1,005 are one doc comment on `terminal_key`, and `peel_key` has
   none.** The `///` run is contiguous — no blank line at 992/993 — so its first
   two paragraphs, which describe a function returning `(&str, &str)`, are
   rendered as `terminal_key`'s documentation. Confirmed from rustdoc's own
   output rather than from reading the bytes: `fn.terminal_key.html`'s docblock
   opens *Peel a terminal `-k<digits>` into what precedes it and the digit run*,
   and `fn.peel_key.html`'s docblock is empty. This is why
   `stale-enumerations-k139`'s row 3 calls line 989 *`peel_key`'s own doc*: it
   was written to be, and is not attached to it.

7. **One leaf carries both, cut after `stale-enumerations-k139`.**
   `kit-fixture-and-peel-doc-k140`. They are one commit's worth of the same
   thing — `task_name.rs`'s account of itself corrected, with chapter 4's
   adjudicating paragraphs rewritten in the same commit under the corpus-freeze
   rule — and splitting them would put two commits through the same file, each
   having to re-prove the book. It runs after k139 rather than before because
   k139 rewords the two-callers count *inside* the paragraphs this leaf then
   moves, and reading the file as it then stands has no staleness to manage.

8. **The kind-shape comment names two shapes the fixture does not carry, and
   that row went to `stale-enumerations-k139` rather than to a new leaf.** Line
   1,120 says the sample is over *one word, two words, four words, a pair where
   one token is a proper prefix of another, digits, and a single character*. The
   eight tokens' longest is three words, twice, and no pair among them is a
   proper prefix of another — `integrate-review-impl` and
   `integrate-review-prototype` share the two-word prefix `integrate-review` and
   neither is a prefix of the other. It is a miscount inside a comment and its
   fix is a reword inside existing lines, which is exactly k139's charter and
   method, so it is that leaf's row 6 rather than a seventh leaf. k139's context
   already says it takes its defects from chapters 3 and 4, so adding the row
   only puts on its checklist what it was already told to read.

9. **The `parse` block is fourteen flat fragments rather than a nested
   composite.** `parse` is 111 lines and wants four reader-sized pieces, and the
   grammar allows a composite under a composite; no fragment in this book nests
   one, so the four pieces are direct children of `the-task-name` alongside the
   other ten. The partition is gapless either way and the line-count credit is
   the top-level block's either way, so nesting would buy an extra level of
   indirection in the fragment index for nothing a reader gets.

10. **One in-session reviewer spent, on technical truth rather than on prose.**
    The leaf's allowance is one, and the draft kind owns technical truth as an
    obligation to discharge. The reviewer was given the page, the source and the
    store's `conformance.rs` and `name.rs` with an adversarial *find what is
    wrong* prompt biased at counts, line numbers and *what it would still pass
    under* claims — the three classes this page carries most of.

11. **`scripts/check.sh` is red on `book-check` alone, as the shape requires.**
    `check: FAILED — 1 of 8`, and the one is `book-check`, failing only with
    `M101` for the thirteen `grove-loop` pages chapters 5 to 21 have not written
    yet; the other five books validate `final=true`. The seven other checks pass.
    That is the same result every child of `grove-loop-k123` leaves until the
    last one, and the parent brief says so.

12. **Three of this page's own claims were wrong on a first pass and were
    corrected against the source rather than left.** They are recorded because
    each names a class worth checking again on the pages after this one.
    - *The kit cannot generate the species case.* It can: the kit offers every
      sample name under all three `Found` values, so a fixture of agreeing pairs
      still poses *parse refuses what `found` contradicts*. Canonicity is the one
      obligation whose reach really is the fixture's, and the page now says which
      of the five are generated and which is not.
    - *A key has no canonicity of its own.* It has. The key renders as plain
      decimal, so `01-impl--a-k007.md` is `NotCanonical` with the canonical form
      `01-impl--a-k7.md` — measured, not read off. That makes chapter 3's
      `parse_is_lenient_on_the_key_and_strict_on_the_slug` the other half of a
      deliberate asymmetry rather than an isolated leniency, and the page now
      draws it.
    - *Five free functions, thirty-one lines, four lines of canonicity, the
      middle line of `peel_key`.* Four counts, none of them enumerated when
      written: the file holds six free functions below the impl, the rest of the
      trait is thirty-two lines, the canonicity check is seven lines, and the
      terminality rule is line 1,013. This is the class the memory note calls the
      highest-yield defect class, and it caught nothing that the validator could
      have caught.

13. **The reviewer returned twenty findings; every one was classified and none
    was noise.** Seventeen were valid and actionable and are applied; two were
    contracts this page had stated unclearly rather than wrongly (*the only place
    in the module that names it*, true of the test module and not the module;
    *both arms end in `Handle::render`*, which repeats the source comment's own
    looseness where the leaf arm ends in `.md`); one — the missed adjudication —
    was a defect in the corpus this page had walked past. No finding was a
    trade-off and none was rejected.

14. **The largest finding is a property of the crate, and the page had it
    backwards.** Five *what it would still pass under* paragraphs claimed a test
    was blind to a misread ordinal, kind, slug or infix. It is not: `parse`
    returns `Entry` only after `parsed.to_string() == name`, so the `entry`
    helper is itself a round-trip assertion, and a test that binds one field
    still fails when `Display` writes any other one wrongly. Measured — adding
    one to the ordinal inside `Display` turns **twelve of the file's
    twenty-five** tests red, including all three the page called blind to it, and
    widening the node arm's position format to three digits turns
    `a_node_directory_parses` red. The page now states the property once, in the
    canonicity section, and each of the five paragraphs says what is genuinely
    absent instead — a fixture no test carries, or a coordinated change to
    `Display` and `parse` together.

15. **A fourth defect in `listings()`' comment, found by the reviewer and added
    to `kit-fixture-and-peel-doc-k140`.** Its first paragraph enumerates the
    fixture as ending in *both transaction sentinels*. There are none, and
    `ordinal-fs-tree` names none anywhere today; the last two entries are the
    near-misses. It reads current because it lists eight items against eight
    entries — `docs/formalism-findings.md` entry 020 records the ten-listing
    fixture it was written for, with `FORMAT` and *the three* sentinels — so the
    count was brought down and the contents were not. This is the execute
    procedure's *never document a claim with a count of itself* seen from the
    other side: the count is what made the sentence look maintained.

16. **No second review, and nothing handed forward.** The family file has no
    escalation kind for a second reviewer, and every finding here was a point
    correction verifiable against the source rather than a redesign — seventeen
    of them demonstrated by a named mutation or a `sed -n`. The corrections are
    this stage's own charter, technical truth, so none belongs to `copy-edit`,
    `art` or `proof`, and the node brief's `## Handed forward` stays empty.
