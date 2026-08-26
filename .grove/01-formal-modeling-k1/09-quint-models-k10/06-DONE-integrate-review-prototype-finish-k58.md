# finish-k58

**Integrates:** finish-k57

## Goal

Integrate the adversarial review of the Quint finish/recovery prototype so its
green run states only what its instruments establish.

## Context

Integrates review `finish-k57` of producer `finish-k12`. Read the review's B1–B6
findings and cited lines before editing. The affected artifacts are
`crates/grove-finish/models/finish.qnt`,
`crates/grove-finish/models/finish-controls.qnt`, the Quint section of
`crates/grove-finish/models/README.md`, and entry 045 of
`docs/formalism-findings.md`; catalogue corrections remain owned by
`formal-synthesis-k16` unless a finding is specifically about faithfully
representing the frozen catalogue.

The independence barrier still applies: do not open any `.als` file, any Alloy
section of a model-directory `README.md`, or entries 026–043.

## Done when

- `FN-25.b` compares the diagnosis carried by every `Blocked` outcome with the
  independent state classifier, and its witness no longer calls one reached
  block an exhaustive sweep.
- The `FN-06` root-swap outcome is reconciled with the frozen catalogue: either
  model it as the required refusal or record the catalogue conflict without
  using the case as `FN-25` partition evidence. Entry 045 finding 6 is reframed
  accordingly.
- Rootless completion proof is attempt-bound; an alien ticket sharing the finish
  handle cannot satisfy `FN-03` or `FN-28`.
- `Current(Live)` is either represented in stable classification and exercised,
  or the narrowing is declared and no affected obligation is claimed over it.
- `FN-14`, `FN-26` and `FN-30` gain falsifying controls, or their evidence claims
  are narrowed to construction facts.
- Q4-105 through Q4-107 are recorded as the bundled `relax_EN_03` candidate they
  actually test, unless independent artifact-removal controls are added.
- The eight existing model mutations retain their causal kills; the two wide
  controls (`mutant_short_preflight`, `mutant_unproven_ownership`) are described
  as bundle controls.
- All post-fix Quint checks and runner coverage required by `finish-k12` pass,
  and the README plus entry 045 record the new evidence and limits.

## Notes

The review classified all 129 witness commands as protocol-established; do not
replace that result with a claim that every paired property is non-vacuous. It
also found that the explicit `Step` / `persistentEffect` / `DECLARED_STEPS`
encoding genuinely answers `FN-24.b` and should be preserved.
