# intake-adjudication-k88

## Goal

Blindly adjudicate the frozen intake record and publish its independent paired
surface verdict.

## Context

- Frozen instrument: `campaign-freeze-k84`.
- Sealed generation barrier: `exposition-assurance-generation-k87`.
- Raw intake record: `intake-generation-k85`.

## Done when

- Manifest identity, the three-surface generation barrier, access validity, and
  deterministic replacement legality are reverified over the full attempt
  history before behavioral scoring.
- Treatment-neutral randomized bundles contain the evidence required by the
  frozen intake rows and no explicit assignment metadata. Every scored bundle,
  including an irregular or incomplete one, receives two independent blind
  scores with minimal citations.
- Each scorer records a forced arm guess only after scoring. Guess accuracy is
  revealed and reported as a supplemental limitation after score records are
  sealed.
- Disagreements are resolved by the manifest-owned blind procedure, with both
  original scores, citations, resolver input, and resolver output preserved.
- Only frozen acceptance rows feed the intake calculation. The report publishes
  all per-row arm counts and says pass only when the comparative materiality
  threshold and absolute enabled floor both pass.
- Missing, unavailable, protocol-failed, unblindable, or otherwise incomplete
  required evidence yields the exact frozen fail-closed result and cannot be
  dropped from the endpoint.

## Notes

No supplemental observation may change sample membership, row scores, or the
surface verdict.
