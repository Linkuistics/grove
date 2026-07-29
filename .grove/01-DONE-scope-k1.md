# scope-k1

**Kind:** requirements

## Goal

Absorb the already-committed spec and plan into this grove, check the
decomposition with fresh eyes, and cut the implementation leaves.

**There is nothing to grill the human about up front.** Requirements, design and
planning were completed and committed in the session that created this grove.
Read them first; only bring questions to the human if reading turns some up.

## Context

Read in this order:

1. `.grove/BRIEF.md` — goal, done-when, and the verified CLI contract.
2. `docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md`
   — why a shell path exists at all, and what the skill must cover.
3. `docs/superpowers/plans/2026-07-29-using-codebase-memory-skill.md` — three
   proposed tasks with runnable steps and expected output.

The plan proposes three leaves: the CLI contract and frontmatter; the
composition patterns and the `min_degree` correction; the manifest and install
verification. **Treat that as a proposal, not a mandate** — it was written by the
same session that wrote the spec, so this leaf is its first independent check.
Confirm, merge, split or reorder as the reading warrants, and say what you
changed and why.

## Done when

- `.grove/BRIEF.md`'s Decomposition section records the decomposition actually
  chosen (and any divergence from the plan, with reasoning).
- The implementation leaves are cut with `grove-llm leaf-add` (or
  `leaf-add-chain` if any artifact here proves load-bearing enough to warrant a
  review chain — a single skill file probably does not).
- This leaf is committed and retired.

## Notes

Judgement call left open for this session: whether the skill's own content
warrants a `review-impl` step, or whether the plan's per-task verification (run
every documented command, compare against the claim) is sufficient. The plan
argues the latter.
