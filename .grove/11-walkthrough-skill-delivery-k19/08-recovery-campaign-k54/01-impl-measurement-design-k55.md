# measurement-design-k55

## Goal

Write the complete recovery-cycle measurement design independently of the
deployed skill's wording and of new campaign outcomes.

## Context

- Parent contract: `walkthrough-skill-delivery-k19` and `recovery-campaign-k54`.
- Hazards only, not a replacement rubric:
  `docs/evaluations/writing-code-walkthroughs/README.md`.
- Preserve `baseline/rubric.md` byte-for-byte.

## Done when

- A role-separated criterion author receives only the parent acceptance
  contract, generic walkthrough method, and declared case/fixture metadata — no
  skill bytes, historical outcomes, new outcomes, or prior score tables. Exact
  inputs, raw outputs, runtime identity, chronology, and digests are preserved.
- The requirement-to-criterion trace is frozen before a separate coverage
  mapper sees the deployed skill. That later map may annotate where the skill
  addresses each row but cannot add, delete, reclassify, or weaken a row.
- A retained / reworded / dropped map compares the frozen new rows with
  `baseline/rubric.md`, gives a reason for every drop, and serves only as
  non-weakening audit evidence rather than a source of criteria.
- Exact same-case prompts, atomic criteria, primary target set, regression set,
  absolute enabled-arm thresholds, relative materiality threshold, and a gate
  for every required behavior family are stated before any control or enabled
  outcome exists. Every requirement-derived row outside the target set is in
  the regression set, including mixed-control rows; a frozen paired rule says
  how each mixed-control row is judged. New control scores select no endpoint
  membership or judging rule.
- The source/fragment prompt retains the historical Case B fixture,
  `targets/ocaml/check_floor.ml`, at its frozen external-source digest. The
  measurement design states explicitly that this is an external, non-Rust
  same-case surface.
- Five assigned control/enabled pairs per case, a precommitted counterbalanced
  order, back-to-back arms within each pair, per-arm start/end timestamps,
  runtime identity, and pair-aware analysis are fixed.
- Replacement legality is a deterministic function of the runner-recorded
  exposure phase. Proven pre-exposure failures consume a frozen wall-clock
  resource budget rather than an attempt ceiling. The freeze fixes the number,
  duration, and automatic start condition of execution windows, with no
  operator-triggered replenishment or extension after outcomes exist.
- A window starts a pair only when its frozen pair reserve remains. Once either
  arm is exposed, the runner completes both arms back-to-back within that
  reserve; exhaustion makes the pair terminally unavailable rather than
  resuming one arm later. Between pairs, the next window resumes the earliest
  incomplete pair in frozen schedule order without inspecting arm or outcome.
  The first post-exposure attempt is retained and ends its arm's assignment.
- The outcome contract marks an illegally replaced attempt as a protocol failure
  for the affected case. Missing, unavailable, protocol-failed, and unblindable
  data never contribute to attainment and leave the parent conjunct open.
- The design fixes two independent blind scorers for every scored bundle,
  randomized normalized bundles, scorer prompt/runtime, atomic citation rules,
  disagreement measurement, a blind tie-resolution procedure, and a forced arm
  guess per scorer and bundle whose accuracy is reported as a limitation.
- The external transfer verdict is a required, separately reported conjunct of
  the parent brief's cross-codebase and cross-language applicability clause; it
  cannot rescue the same-case endpoint or drive skill wording.
- The design separates deterministic access/digest/manifests from judged answer
  behavior and limits every conclusion to this bounded sample; it does not
  claim reader comprehension, population generality, or causal use of wording.
- No evaluated treatment or control context is launched in this leaf.

## Notes

The historical gaps identify protocol hazards and surfaces needing valid
evidence. They do not choose the new criterion set, threshold, or score.
