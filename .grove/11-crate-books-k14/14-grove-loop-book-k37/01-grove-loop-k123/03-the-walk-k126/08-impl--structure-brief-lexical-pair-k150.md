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
