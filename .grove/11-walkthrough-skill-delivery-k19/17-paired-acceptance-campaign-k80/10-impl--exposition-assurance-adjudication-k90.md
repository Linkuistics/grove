# exposition-assurance-adjudication-k90

## Goal

Blindly adjudicate the frozen exposition/assurance record and publish its
independent paired surface verdict.

## Context

- Frozen instrument: `campaign-freeze-k84`.
- Sealed generation barrier: `exposition-assurance-generation-k87`.
- Raw exposition/assurance record: `exposition-assurance-generation-k87`.

## Done when

- Manifest identity, the generation barrier, access validity, and deterministic
  replacement legality are reverified over the full attempt history before
  scoring.
- Treatment-neutral randomized bundles retain all evidence needed by the frozen
  exposition/assurance rows and no explicit assignment metadata. Every scored
  bundle, including an irregular or incomplete one, receives two independent
  blind scores with minimal citations in fresh scorer and resolver contexts for
  this surface.
- Each scorer records a forced arm guess after scoring. Only after this
  surface's score records and resolver output are sealed are labels revealed
  for all three surfaces and guess accuracy reported as a supplemental blinding
  limitation.
- The frozen blind resolver preserves both scorers' decisions and citations and
  cannot alter, add, or drop a row.
- All frozen primary and no-regression rows are published. The surface passes
  only when its comparative materiality threshold and absolute enabled floor
  both pass under the predeclared mixed-control rule.
- Missing, unavailable, protocol-failed, unblindable, or incomplete required
  evidence follows the frozen fail-closed result and cannot be compensated by a
  different surface.

## Notes

Adjudication measures returned artifacts and assurance plans, not reader
comprehension.
