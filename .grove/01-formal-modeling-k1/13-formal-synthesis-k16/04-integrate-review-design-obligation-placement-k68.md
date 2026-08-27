# obligation-placement-k68

**Integrates:** obligation-placement-k67

## Goal

Integrate the three findings from `obligation-placement-k67` before
`catalogue-disposition-k64` applies the placement rule to the rest of the claim
catalogue.

## Context

Read the review's `## Findings` verbatim; its `path:line` citations are the
handoff. The producer is `obligation-placement-k63`, and its committed change is
the review baseline.

The findings are coupled at one seam:

1. `TT-24.a` makes the rule ambiguous because *action* spans task-tree, finish,
   lifecycle and environment scopes. If the quantifier is universal, the `TT-`
   placement and its coverage are wrong; if it is prefix-local, Q4-6 cannot use
   task-tree coverage as evidence for a finish reaper mutation.
2. `FN-32` names both the witness slot and cleanup marker, but mutation 63 and
   `mutant_unproven_ownership` do not independently falsify the cleanup-marker
   half, and the Alloy witness pairs the marker with a `Discard` step that cannot
   mutate it.
3. The contested-cell report calls a property-only, no-witness cell “answered,”
   and its new extractor and condition have no durable runner controls.

## Done when

- The ADR's direction/observation/joint rule is mechanically readable from the
  obligation text, including terms such as *action* that span groups. `TT-24.a`
  and Q4-6 are re-decided under the corrected rule, and the row cites evidence
  executed against the action set its mutation changes.
- Each of the original six placements is rechecked after that correction. Any
  changed placement is reconciled across the semantic contract, both model
  families, scope READMEs, Q4 rows and the runner manifest.
- Every artifact `FN-32` names has an independently reached antecedent and an
  isolating falsifier in both families, or the claim is narrowed to exactly what
  the existing controls establish. A witness beside an unrelated framed field
  does not count.
- The contested report distinguishes complete answers from property-only cells,
  preserves the chosen fatal/nonfatal policy without overstating its evidence,
  and `models/run-controls.sh` carries positive and negative controls for
  `control_ob` plus the contested-cell condition.
- All post-fix verification required by the producer's touched scopes and runner
  is rerun against final files and recorded durably. The integration session owns
  every fix and every verification command; the review ran none.

## Notes

This leaf was inserted at the first live sibling after the review so no catalogue
edit can stale its citations. Do not let `catalogue-disposition-k64` absorb these
fixes: it consumes the placement rule and must see the integrated version.
