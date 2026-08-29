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

- A requirement-to-criterion trace derives every judged behavior from the
  parent acceptance contract and generic walkthrough method before the skill is
  consulted; a later coverage map may show where the skill addresses each row
  but cannot add or weaken a row.
- Exact same-case prompts, atomic criteria, primary target set, regression set,
  absolute enabled-arm thresholds, relative materiality threshold, and a gate
  for every required behavior family are stated before any control or enabled
  outcome exists. New control scores select no endpoint membership.
- Five assigned control/enabled pairs per case, a precommitted counterbalanced
  order, runtime identity, attempt ceiling, and pair-aware analysis are fixed.
- Replacement is limited to failures proven to precede prompt and treatment
  exposure. Post-exposure protocol violations and missing or truncated outputs
  receive a frozen outcome rather than replacement; exhausted pre-exposure
  replacements, empty sets, incomplete arms, and unblindable samples each have
  explicit non-vacuous verdict semantics.
- The design fixes two independent blind scorers per complete case, randomized
  normalized bundles, scorer prompt/runtime, atomic citation rules,
  disagreement measurement, and a blind tie-resolution procedure.
- The design separates deterministic access/digest/manifests from judged answer
  behavior and limits every conclusion to this bounded sample; it does not
  claim reader comprehension, population generality, or causal use of wording.
- No evaluated treatment or control context is launched in this leaf.

## Notes

The historical gaps identify protocol hazards and surfaces needing valid
evidence. They do not choose the new criterion set, threshold, or score.
