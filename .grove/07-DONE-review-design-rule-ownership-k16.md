# rule-ownership-k16

**Reviews:** rule-ownership-k15

## Goal

A **third** adversarial read of the rule-ownership design, scoped to
`rule-ownership-k15`'s **delta** and not to the whole artifact. Two prior reads
found eight P1s between them, and the second repair again replaced the placement
function's input — so what the eight executing leaves would build on is, once more,
a design no reviewer has seen.

This is the last review this design gets before execution. Its exit is the chain's
own laziness: findings worth acting on earn an `integrate-review-design` leaf,
findings that are not do not. A P2-or-below outcome creates nothing.

## Why this leaf exists, and why it is narrow

`k12` reviewed the original design and found five defects, four P1. `k13` repaired
them by replacing `Bound`-alone with the `Bound`/`Occasion` pair. `k14` reviewed
*that* and found four more P1s and a P2. `k15` repaired those — and in doing so
changed the input model a second time, changed what "reachable" asserts, changed
the `own` set, added four rules, and wrote 24 canonical sentences that did not
exist before.

The pattern is what justifies another read: each repair has been structural rather
than local, and each structural repair has so far carried new defects. What does
**not** justify a full re-read is that `k12` and `k14`'s confirmed findings and
confirmed evidence still stand — those are not up for re-litigation.

## Scope — the delta, five surfaces

1. **`Occasion` as a set, with the earliest-step tie-break.** Is the domain now
   closed and single-valued over every inventory row? Does *earliest step wins*
   yield the recorded owner for every multi-step row, or does some row's owner only
   survive because nobody recomputed it? `escalation-names-the-tradeoff` is the
   worked case; look for the ones that were left single-valued and should not have
   been.
2. **The `context` occasion and rule 5.** It exists so the function derives
   `references/grove.md`, which it previously never did. Is `context` bounded, or is
   it an escape hatch — could a row that belongs to a loop step be filed here
   instead, and would anything catch it? Check the three rows that now carry it.
3. **Reachability as an asserted edge.** Four assertions plus two schema checks.
   Walk the whole edge set: does every non-static owner have an incoming edge, does
   every `@` file genuinely name its owner's path *given the rewrites the leaves are
   chartered to make*, and does any chain cycle? The previous test passed while
   `references/driver.md` had no incoming sentence; find the analogous survivor if
   there is one.
4. **The 24 canonical trigger sentences.** Each is claimed ≤25 words, one
   situation, one obligation, one path, no branch or enumeration. Re-measure them,
   and judge the grammar rather than the count: sentences 12, 14, 15 and 16 each
   cover two rows, and 1 covers two rows across two situations that were normalised
   into one. Is that normalisation honest, or is it the compound the class forbids
   wearing a shared `on(...)` string? Also: does the set cover every `trigger` row
   exactly once, and does the arithmetic (~613 words, 600–900 range) hold?
5. **The ADR split.** Two records now — `corpus-rules-have-one-owner` and
   `restatement-declares-its-class`. Is each one binding trade-off, are they
   genuinely independently reversible, and does either restate the other or the
   spec? Check that no citation anywhere still points at the merged record for the
   half it no longer carries.

## Context

- `docs/specs/corpus-rule-ownership.md` — the repaired design.
- `docs/adr/corpus-rules-have-one-owner.md`,
  `docs/adr/restatement-declares-its-class.md`.
- `.grove/06-DONE-integrate-review-design-rule-ownership-k15.md` — what was
  repaired and why; its findings section is `k14`'s output, not `k15`'s claim.
- `.grove/05-DONE-review-design-rule-ownership-k14.md` — the confirmed evidence to
  preserve, which this read does not reopen.
- `content/TASK-FORMAT.md`, `content/driving.md`, `content/references/finish.md` —
  the three files whose sentence-level audit produced the four new rows. If the
  audit is still incomplete, it is incomplete *here*.
- `.grove/BRIEF.md` and the eight executing leaf contracts, for whether the
  reconciliation actually landed everywhere the delta touches.

## Done when

- Each of the five surfaces has a stated verdict: defect, or confirmed with the
  evidence that confirms it.
- Every finding cites `path:line` and states the concrete failure a leaf would hit.
- The **inventory-wide** recomputations are checked rather than sampled: the
  arithmetic and the edge set are small closed sets, so partial coverage is a
  choice to declare, not a limit to accept.
- No artifact is edited and no test, build, lint or format command is run — this is
  inspection only, and the paired integration owns every fix.

## Notes

- `k15` verified two schema invariants mechanically (every `@ SKILL.md` row is
  `trigger`, and the converse) and re-measured all 24 sentences with a script,
  finding two of its own counts off by one. Those checks are evidence, not a reason
  to skip re-derivation — a script that encodes the same misunderstanding as the
  prose agrees with it.
- `content/SIGNAL.md`, `content/SIGNAL-FINISH.md` and `src/prompt.rs` are out of
  scope, unchanged, and not to be proposed for change.
- If the honest verdict is that the delta is sound, say so plainly and cut nothing.
  A third review that manufactures findings to justify itself is worse than no
  third review.
