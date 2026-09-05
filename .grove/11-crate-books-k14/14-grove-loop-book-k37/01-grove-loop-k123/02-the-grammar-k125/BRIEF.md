# the-grammar-k125 — brief

## Goal

Draft Part I of the `grove-loop` book — chapters 2, 3 and 4, the whole of
`crates/grove-loop/src/task_name.rs` (1,714 lines) in nine ownership blocks — and
prove the prefix through slice `canonical-or-nothing`.

## Context

- Draft stage, child 2 of 7 of `grove-loop-k123`. The structure brief is
  `docs/specs/grove-loop-book-structure.md`; chapters 2, 3 and 4 are its
  *2 · The tokens, and the four verdicts*, *3 · Kind, slug, handle* and
  *4 · The name, and canonicity* sections, and the mapping is its *Top-level
  ownership blocks* and *`task_name.rs` three ways in nine blocks*.
- **It decomposed one child per chapter**, as its own leaf body anticipated:
  1,714 lines over three chapters — 451, 563 and 700 — with the third carrying
  the conformance kit and the canonicity argument. The blocks are fixed by the
  manifest, so the seam is the chapter boundary and nothing else.
- The blocks, in file order and with their owning chapter: `1-220` (2),
  `221-590` (3), `591-1020` (4), `1021-1177` (4), `1178-1199` (2), `1200-1312`
  (4), `1313-1521` (2), `1522-1550` (3), `1551-1714` (3). Production splits at
  the type boundaries; the inline test module's seven labelled sections
  distribute to the chapter whose concept each proves.
- **The prose obligation here is *supply the claim*.** 694 of these lines are the
  inline test module at 20% prose. For every reproduced test: the property it
  establishes, **and what would have to be true for it to pass while the property
  was broken**. The production halves are 42% prose and take the *do not
  restate* instruction instead.
- **The manifest's rows for this part close in chapter 4 and open before it.**
  They are the `TaskName`/`TaskNameError`/`Verdict` family with the support
  helpers (first use chapter 2, anchor `02-the-tokens.md#the-four-verdicts`),
  `impl Display for TaskName` (first use chapter 2, anchor
  `02-the-tokens.md#refusals-inside-the-shape`) and `TaskName::compose` (first
  use chapter 3, anchor `03-kind-slug-handle.md#the-handle-is-the-identity`) —
  three rows since `display-first-use-k137` split the last two apart, two before
  it. All are owned by chapter 4, and each anchor is fixed by the manifest and
  must exist on its page in explicit form. Chapter 1's cast rows owned
  by `four-verdicts`, `the-handle-not-the-position` and `canonical-or-nothing`
  move from `pending` to `explained` as each chapter lands.
- The carried example's steps for these chapters are the brief's *Worked
  examples* rows 2, 3 and 4: `01-requirements--plan-k1.md` from a directory
  listing to one of four verdicts; `plan-k1` peeled to a handle that carries slug
  and key and not the position; and the same filename parsed and rendered back
  byte-identical or refused as uncomputable.
- The spine for all three chapters is *the grammar is the thing that could not
  move*: the store would have accepted the entry, and it is grove's grammar that
  refuses it. `pick_refuses_a_species_mismatch_at_a_task_shaped_name` is chapter
  7's, but the refusal it exercises is defined here.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  canonical-or-nothing --check all` is valid: 13 files, 2,150 resolved lines,
  8,383 deferred, `final=false`.
- Chapters 2–4 exist, `README.md`'s contents and the navigation lines either side
  of them are updated, and the ownership ledger rows for the nine `task_name.rs`
  blocks read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decomposition

**Three children, one per chapter.** The blocks are fixed by the manifest, so the
seam is the chapter boundary and nothing else, and the order is forced:
`--through` proves a canonical prefix, so a child cannot prove its slice before
its predecessor's page exists.

| # | Child | Chapter | Slice | Owned lines | Cumulative resolved | Deferred |
|---:|---|---:|---|---:|---:|---:|
| 1 | `the-tokens-k134` | 2 | `four-verdicts` | 451 | 887 | 9,646 |
| 2 | `kind-slug-handle-k135` | 3 | `the-handle-not-the-position` | 563 | 1,450 | 9,083 |
| 3 | `the-name-k136` | 4 | `canonical-or-nothing` | 700 | 2,150 | 8,383 |

Each child's slug is its page id, which is the convention `orientation-k124` set.
Every child leaves `scripts/check.sh` red on `book-check` alone and says so; the
script runs `--final` over every book root by discovery, and a prefix
deliberately leaves later blocks deferred.

## Found while drafting

**The early-use ledger's `Display` row names chapter 3 as the first use, and
chapter 2 uses it.** The manifest's mandatory row for `` `TaskName::compose`,
`impl Display for TaskName` `` declares its first use at
`03-kind-slug-handle.md#the-handle-is-the-identity`, and the structure brief's
*Early uses the order forces* gives the same reasoning — the handle's structural
claim can only be asserted over a rendered whole name. But chapter 2's own block
`shape-refusal-tests` calls `TaskName`'s `Display` twice, at
`crates/grove-loop/src/task_name.rs` lines 1418 and 1455, where
`a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading` asserts that
each of two names renders back to its own bytes. The genuine first use in page
order is therefore chapter 2, not chapter 3.

The mandatory row is matched byte-for-byte by `check_early_uses` in
`crates/book-validation/src/ledger.rs`, and the validator never checks that a
named first use is the *earliest* one — so nothing is red, and chapter 2 is green
with the row untouched. `the-tokens-k134` left the row alone and stated the
rendering behaviour locally at `02-the-tokens.md#refusals-inside-the-shape`
instead, so the page is self-contained either way.
`display-first-use-k137` owned the adjudication and **ran before
`kind-slug-handle-k135`**, because chapter 3 is the page that would otherwise
write the anchor the row promises. **It settled that the manifest was wrong and
split the row in two**: `TaskName::compose` keeps chapter 3, and `impl Display
for TaskName` moved to `02-the-tokens.md#refusals-inside-the-shape`. The
*load-bearing first use* reading was rejected on the specification's own wording
— a row's fourth column is what *the earlier page* must state locally, so a row
naming a later page leaves that obligation unrecorded. The manifest, this brief's
parent structure brief, the ledger and chapter 2's four-name paragraph now agree.

**The criterion for owing a row is now in the structure brief, and chapter 2's
sweep is `floor-rows-chapter-two-k138`.** `display-first-use-k137` settled that a
reproduced block owes a row for a later-owned symbol it *names or exercises*
unless an existing row already covers it — `entry_path`'s mandatory row is the
precedent for *names* — and wrote that into *Early uses the order forces*. It
settled the criterion on one symbol without re-running it over chapter 2's
blocks, so k138 does that sweep before chapter 3; chapters 3 and 4 inherit both
the criterion and a worked example.

**The ledger is a floor, and chapter 2 added three rows to it.**
`TaskName::distinguished` (owner `canonical-or-nothing`), `Parts::leaf` (owner
`the-handle-not-the-position`) and the `a_kind` / `slug` test helpers (owner
`canonical-or-nothing`) are all called by bytes chapter 2 reproduces and defined
by blocks later chapters own. Chapters 3 and 4 owe the same enumeration over
their own blocks rather than treating the manifest's rows as the set.

**Five stale enumerations in `task_name.rs`'s comments, and `stale-enumerations-k139`
holds the fix.** Chapter 3 found four of them in its own blocks and adjudicates each
on the page: `Kind::is_finish`'s *three places that ask* against seven calls in six
functions; `Handle::parse`'s *only peel … outside `split_shape`* against three
callers of `peel_key`; `UnknownKind` at line 1,536, a `TaskNameError` variant
`open-kind-k20` deleted and nothing else in the repository mentions; and line
1,691's *three references* against a loop of four. **The fifth is chapter 4's**:
`peel_key`'s own doc at line 989 says *the two callers* where there are three, and
line 980 names two of the three, while line 1,004 — twenty-five lines below it —
states the relation correctly. `the-name-k136` owns adjudicating that one on
`04-the-name.md`, in the same register chapter 3 used, and does not fix it inline.
k139 runs after both pages exist, rewords all five inside their existing lines so
no block's line count moves, and rewrites both pages' adjudicating paragraphs in
the same commit.

**The reserved-word clause of `refuse_token` changes the message and not the
verdict.** `BRIEF`, `DONE` and `ABANDONED` are all uppercase, so the character-set
clause would refuse them anyway; the reserved clause runs first and is why an
operator reads *reserved* rather than *lowercase only*. Chapter 3's slug-rule test
cannot see the difference and says so. This is not a defect and needs no leaf — it
is a *what it would still pass under* observation chapter 4 may want when it reads
the same guard from the parse side.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

1. **Decomposed one child per chapter rather than drafting all three.** 1,714
   source lines is roughly four times the 436 that became chapter 1's 1,037
   markdown lines in one session, and the manifest's block owners already
   partition the file along exactly the chapter boundary. Any other cut would
   leave a child unable to prove itself with `--through`.
