# rule-ownership-k12

**Reviews:** rule-ownership-k2

## Goal

Adversarially read `docs/specs/corpus-rule-ownership.md` and ADR
`corpus-rules-have-one-owner`. Every remaining leaf in this grove executes
against them, and they are cut ahead of those leaves precisely so a fault is
found before eight rewrites are built on it.

The `review-design` reads apply: does the design satisfy the requirements, is the
ADR set a minimum coherent set, are the seams at the right height and count.
Below are the specific claims worth attacking first — the ones the producing
session could not establish from inside its own frame.

## Context

- `docs/specs/corpus-rule-ownership.md` — the artifact under review.
- `docs/adr/corpus-rules-have-one-owner.md` — the decision it rests on.
- `.grove/BRIEF.md` — the requirements it must satisfy, and its work order.
- `.grove/01-DONE-requirements-plan-k1.md` — the requirements as originally
  stated.
- `src/prompt.rs` — the fixed runtime the design must not require changing.

## The claims to attack

1. **Is the placement function actually a function?** It claims one answer per
   rule. Find a real rule in `content/` whose `Bound(R)` makes *two* files
   equally narrow, or where "narrowest file every bound session opens" is
   ambiguous. The producing session did not find one and did not prove there is
   none — a single counterexample means the design needs a tie-break rule it
   currently lacks.

2. **Is `Bound(R)` decidable without judgement?** The whole claim of being
   better than filing-by-topic is that the axis is a runtime fact rather than an
   opinion. Test it on the hardest rows: `spine-suggested-shape` (does it bind a
   session, or only a reader?), `records-are-current-state` (all nineteen, or
   only kinds that write records?), `hitl-afk-mark-predicts`. If Bound needs a
   judgement call, the function has moved the judgement rather than removed it.

3. **Is the inventory complete?** The spec claims completeness is checkable per
   owner file. Pick two `content/` files and check that **every** normative
   sentence resolves to a row. `references/driver.md` and `content/grilling.md`
   are the likeliest to have unlisted rules.

4. **Does `driving.md` really have `Bound(R)` = ∅ once the listed rules leave?**
   This is the design's most consequential single output — a 5,817-word file
   deleted from the embed. Read what remains after the tabulated rules are
   removed and find anything still normative. A rule left behind there is a rule
   deleted, not rehomed, because `docs/` is unreachable.

5. **Does the two-register rule survive contact with `SKILL.md`?** The design
   says most `SKILL.md` content is per-kind and loses its mirror. Sample the
   current `SKILL.md` and check the claim holds at the ~700–900-word target the
   brief sets. If the surviving conditions do not fit, either the target or the
   register rule is wrong.

6. **Is the B/S test split sound?** The design forbids handing single-source
   assertions to `behavior-evals-k3` because they cannot be green before the
   rewrite. Check no **B** row is secretly structural, and that the **B** set
   really does cover the nine invariants the brief's *Done when* names.

7. **Is the deferral policy per-deferral, or a blanket rule with 14 rows?** The
   requirements forbid a blanket answer. The design supplies a generating
   question (*does absence change what a session writes, or how well?*) and 14
   answers. Decide whether that is a decided-per-row policy or one rule in a
   table's clothing — and if the latter, whether that is actually wrong.

8. **Does the ADR clear its own AND test, and is it separable from
   `skill-delivers-the-methodology`?** The producing session argued separability
   from differing reopen conditions. Attack that: if the two records would always
   be revisited together, the set is not minimal and they should be one.

## Done when

- Each claim above is confirmed or refuted, with the evidence.
- Findings are recorded as findings only — this session runs no fix and no
  rewrite. Anything worth acting on goes into an `integrate-review-design` leaf,
  cut as this session's last act, carrying the findings verbatim. A review that
  finds nothing worth acting on creates nothing and simply retires.

## Notes

- **Inspection only.** Read the committed artifacts, the requirements and the
  corpus. Do not edit `content/`, `docs/`, or the tree beyond cutting the
  integration leaf.
- The producing session's own repo-wide claims were verified with normalised,
  controlled sweeps (emphasis stripped, whitespace collapsed) — the counts of 14
  deferrals across 9 files, six statements of the placement rule, three of the
  working-increments rule. **Re-run them rather than trusting them**; an
  unnormalised sweep reads clean on a wrapped or bolded match, which happened
  twice during the producing session.
- The producer spent no in-session reviewer, so nothing here has been read
  adversarially yet.
