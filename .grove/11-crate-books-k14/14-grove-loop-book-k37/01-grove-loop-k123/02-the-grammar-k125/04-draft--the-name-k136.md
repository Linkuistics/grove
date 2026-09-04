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
- **Three early-use rows close here and none open.** The manifest's rows owned by
  `canonical-or-nothing` — the `TaskName` / `TaskNameError` / `Verdict` family
  with the `verdict` / `entry` / `malformed` helpers, and `TaskName::compose`
  with `Display` — plus the rows chapter 2 added for `TaskName::distinguished`
  and for the `a_kind` / `slug` helpers, all move from `pending` to `explained`
  when this slice lands. So does chapter 1's `TaskName` cast row. Check every row
  in the ledger whose owner is this slice.
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
