# structure-brief-lexical-pair-k150

## Goal

Correct the one remaining Part II claim in
`docs/specs/grove-loop-book-structure.md` that enumeration refutes: the
discriminating pair the chapter 7 section names for
`pick_orders_numerically_not_lexically`, which is `100` against `99` in the
block and cannot be `10` against `9` under the canonical grammar.

## Context

- **The defect.** The brief's *7 · The walk: pick and select — `first-live-leaf`*
  section says the test *passes under a lexical sort too until there are ten
  leaves, and it is `10` against `9` that makes it a test.* Both halves are
  false under the grammar the book documents.
- **Why `10` against `9` discriminates nothing.** A position is zero-padded to
  **at least two digits and carries no other leading zero** —
  `crates/grove-loop/src/task_name.rs`'s `NotCanonical` message states it, and
  `docs/adr/task-names-are-canonical.md` is the record. So 9 renders `09` and 10
  renders `10`, and `"09" < "10"` bytewise as well as numerically: a lexical sort
  and a numeric one agree on that pair. A lexical sort first disagrees where the
  digit count changes, which under a two-digit minimum is 99 against 100.
- **What the block actually holds.** `crates/grove-loop/src/task_tree.rs` 1,142
  to 1,155 builds `100-impl--b-k2.md` and `99-impl--a-k1.md` and expects
  `99-impl--a-k1.md`. The test's own comment says why in the same words: the old
  spelling made the point with an unpadded `2-…`, *which the canonical grammar
  now refuses*, and *the zero-padding is a minimum width, so a three-digit
  ordinal is the discriminating case that survives*.
- **It was wrong when written, not outrun.** `task_tree.rs` is unchanged since
  `grove-loop-structure-k36` wrote the brief — the same provenance
  `pick-test-count-k147` established for the count in the same sentence — so the
  brief was describing the pre-canonical spelling of a test that had already
  moved on.
- **The page is already right and is not edited for this.**
  `docs/walkthroughs/grove-loop/07-the-walk.md` states `100` against `99`, gives
  the padding rule as the reason, and cites the comment. This leaf owes agreement
  between the brief and that page, in the brief. `the-walk-k143` decision 2 holds
  the finding.
- **The corpus is not touched.** The defect is in the brief that describes the
  block, exactly as `structure-brief-dependency-count-k132`'s and
  `pick-test-count-k147`'s were.

## Done when

- The brief's chapter 7 section names `100` against `99` as the discriminating
  pair and gives the digit-count reason, with no surviving claim that a lexical
  sort agrees up to ten leaves.
- The padding rule is verified against `crates/grove-loop/src/task_name.rs`
  rather than trusted from this file, and the fixture is verified by reading
  `crates/grove-loop/src/task_tree.rs` 1,141 to 1,155.
- Chapter 7's page is checked to agree with the corrected brief rather than
  rewritten; if the two disagree, the bytes decide.
- No other statement of the same pair survives elsewhere in the brief.
- `bash scripts/check.sh` is unchanged — red on `book-check` alone, which is
  every child of `the-walk-k126`'s shape until chapter 21 lands.

## Notes

**Do not widen this into a sweep of the whole brief.** `pick-test-count-k147`
already checked Part II's other counts and recorded which stand; this is the one
claim it did not reach, because it was reading the sentence for its number rather
than for its example.

## Decisions (running log)

1. **Corrected the brief in the brief and left `07-the-walk.md` untouched.** The
   two facts were re-derived from the bytes rather than from this file:
   `crates/grove-loop/src/task_name.rs` 732–736 states the padding rule in the
   `NotCanonical` message — *a position is zero-padded to at least two digits and
   carries no other leading zero, so `05` and `100` are names and `5` and `005`
   are not* — and `task_tree.rs` 1,141–1,155 builds `100-impl--b-k2.md` against
   `99-impl--a-k1.md` and expects the latter. The page already states the pair,
   the digit-count reason and the padding rule at lines 475–488, so there was
   nothing to reconcile; the disagreement was one-sided and the brief was the
   wrong side.
2. **Gave the reason rather than only the pair.** The replaced sentence was
   wrong in two independent ways — the pair, and the claim that a lexical sort
   agrees below ten leaves — so naming `100`/`99` alone would have left a reader
   able to re-derive `10`/`9` from the same false premise. The new text states
   the premise that kills it: 9 renders `09`, so `"09" < "10"` bytewise as well
   as numerically and any pair below the 99/100 boundary discriminates nothing.
3. **Did not sweep the rest of the brief.** `pick-test-count-k147` covered
   Part II's counts and the parent brief records which stand; a grep for
   `lexical` and *ten leaves* across `docs/specs/grove-loop-book-structure.md`
   returns only the corrected sentence, which is the *no other statement
   survives* obligation and not a licence to widen.
