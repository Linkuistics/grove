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
- **Three early-use rows close in this part, and two of them open in it.** The
  manifest's rows for `TaskName`/`TaskNameError`/`Verdict` and the support
  helpers (first use chapter 2) and for `TaskName::compose` and `Display` (first
  use chapter 3) are owned by chapter 4, and their first-use anchors —
  `02-the-tokens.md#the-four-verdicts` and
  `03-kind-slug-handle.md#the-handle-is-the-identity` — are fixed by the manifest
  and must exist on those pages as explicit anchors. Chapter 1's cast rows owned
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
`display-first-use-k137` owns the adjudication and **runs before
`kind-slug-handle-k135`**, because chapter 3 is the page that would otherwise
write the anchor the row promises. Which of the two artifacts is wrong is that
leaf's to decide and is deliberately not decided here: the brief's reasoning is
about the *load-bearing* first use, and a reading on which chapter 2's round-trip
assertions are incidental and the row stands is a legitimate outcome.

**The ledger is a floor, and chapter 2 added three rows to it.**
`TaskName::distinguished` (owner `canonical-or-nothing`), `Parts::leaf` (owner
`the-handle-not-the-position`) and the `a_kind` / `slug` test helpers (owner
`canonical-or-nothing`) are all called by bytes chapter 2 reproduces and defined
by blocks later chapters own. Chapters 3 and 4 owe the same enumeration over
their own blocks rather than treating the manifest's rows as the set.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

1. **Decomposed one child per chapter rather than drafting all three.** 1,714
   source lines is roughly four times the 436 that became chapter 1's 1,037
   markdown lines in one session, and the manifest's block owners already
   partition the file along exactly the chapter boundary. Any other cut would
   leave a child unable to prove itself with `--through`.
