# family-scope-guard-k20

**Integrates:** family-scope-guard-k19

## Goal

Triage and fix the three findings from the adversarial review of
`family-scope-guard-k18`. Preserve the useful registered-family assertion while
making its claimed failure path true and keeping it from imposing semantic
family intent on unrelated future scopes.

## Context

The producer added the family-scope guard at the foot of
`tests/session_kind_guidance.rs` and reconciled
`docs/specs/mandate-delivered-methodology.md`, `CHANGELOG.md`, and `CONTEXT.md`.
The review was inspection-only; no test, build, lint, or format command was run.

## Findings

1. **High — a kind omitted from `Kind::ALL` remains invisible to the new
   guard.** `tests/session_kind_guidance.rs:1365` derives every family's members
   by filtering `Kind::ALL`, but that array is still a hand-maintained
   `[Kind; 19]` at `src/leaf.rs:73`, and its count test still expects 19 at
   `src/leaf.rs:246`. Add a twentieth enum variant, update the exhaustive
   `label`, `is_producer`, and `family` matches so compilation resumes, but omit
   it from `Kind::ALL`: `Family::members()` never sees it, so the marker's old
   scope still equals the derived family and the promised marker-naming
   assertion stays green. This breaks `family-scope-guard-k18`'s second `Done
   when` and narrows the producer's recorded experiment to the path where the
   author also remembered to update `ALL`. Make enum membership and `Kind::ALL`
   coverage mechanically agree, then demonstrate both stages of the claimed
   failure path, including the omitted-`ALL` case.

2. **Medium — the inverse sweep guesses semantic intent from current set
   equality.** `tests/session_kind_guidance.rs:1487` treats any unregistered
   triggering unit whose current reach happens to equal a multi-member family as
   a declaration that every future member must receive it. Set equality cannot
   establish what a unit is about: a future unit may legitimately apply to the
   current members because of another shared property and intentionally exclude
   a later family member. There is no opt-out, so such a unit makes the suite red
   until the author either lies by registering it or changes the guard. Filtering
   out one-member families at `tests/session_kind_guidance.rs:1395` avoids the
   present `finish` and `combine-research` false positives but does not solve that
   inference. The inverse sweep was also outside `family-scope-guard-k18`'s four
   `Done when` bullets. Remove it and its current-state claims, or replace it only
   with an explicit authored distinction between family intent and coincidental
   shape; if that replacement needs design, externalize it rather than widening
   this integration.

3. **Medium — a wrong family decision defeats the exact omission guard, and the
   stated precedent does not share that residue.** The comment at
   `tests/session_kind_guidance.rs:1302` admits that a new `review-*` kind put in
   `Family::Producer` leaves every assertion green, then says this is the same
   accepted residue as `is_producer`. It is not: `src/leaf.rs:326` independently
   derives whether each kind has the two prefixed review-chain steps and compares
   that with `is_producer` at `src/leaf.rs:333`. The new family classifier has no
   corresponding taxonomy cross-check, so a mistaken arm silently preserves the
   narrow marker this guard exists to expose. Add an independent check of family
   classification against the kind taxonomy (without using it to derive the
   registered scope's expected membership), and add a negative control for a
   wrong-family decision; otherwise narrow the guarantee and remove the false
   comparison from the test, spec, changelog, and glossary.

## Done when

- All three findings are explicitly triaged; accepted trade-offs are recorded
  honestly, and real issues are fixed.
- The registered checks for `task-review-kinds`,
  `task-integrate-review-kinds`, and `task-research-pair` still compare reach
  through `Scope::admits`, not marker spelling or order.
- Fresh controls prove the guard fails for a missing family member, a renamed
  registered unit, a kind omitted from `Kind::ALL`, and any family
  misclassification the implementation claims to reject.
- The real embed passes, and the spec, changelog, glossary, and test comments
  describe only guarantees the code now provides.

## Notes

The review confirmed the remaining doubts:

- Reach rather than spelling is the correct contract. `Scope::admits` uses
  membership, so duplicate labels and order do not change which mandates receive
  the unit; canonical marker formatting would be a separate concern.
- The spec's test-seam placement and lack of a new `## Requirements` entry are
  the right grain for a maintenance guard, and the changelog entry is under the
  correct `Unreleased / Added` section. Their details must be reconciled with the
  fixes above.
- The existing synthetic controls exercise their stated branches: missing
  member and missing registered id for the forward check; universal,
  single-kind, one-member-family, complement, and procedural shapes for the
  inverse check. The combined quiet-shape fixture is evaluated independently per
  unit by the implementation.

The review re-derived the producer's twentieth-kind experiment statically. The
exhaustive `family` match does force classification before compilation, and a
correct `Family::Review` classification reaches the marker assertion only when
the new kind is also present in `Kind::ALL`; finding 1 is the uncovered branch.
