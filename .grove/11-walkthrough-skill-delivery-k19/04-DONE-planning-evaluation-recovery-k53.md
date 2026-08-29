# evaluation-recovery-k53

## Goal

Plan the smallest new predeclared evaluation cycle that can validly test the
deployed `writing-code-walkthroughs` skill against the parent brief's unchanged
acceptance requirement.

## Context

- The durable report is
  `docs/evaluations/writing-code-walkthroughs/README.md`.
- The prior campaign did not establish its rubric. Its no-tool prompts made
  observable file-based skill loading an invalidating event, selecting retained
  enabled samples against visible skill use, and nearly all judged evidence used
  a superseded skill digest.
- Preserve the completed campaign as historical evidence. A recovery cycle must
  be separately predeclared; it must not rewrite the frozen rubric or rescore old
  outputs.

## Done when

- The tree contains dependency-ordered vertical slices for a new campaign whose
  prompts, access rules, skill-loading evidence, sample sizes, replacement
  limits, controls, scoring, and contamination checks are fixed before execution.
- The plan tests the deployed skill bytes across the behaviors needed by the
  parent acceptance requirement and keeps transfer evidence separate.
- The plan names how every arm can complete without treating required skill
  discovery as a protocol breach, and how deterministic guarantees remain
  distinct from judged behavior.

## Notes

Use the existing report's unresolved gaps as inputs, not as a new rubric. Keep
the skill generic; do not shape the recovery cycle around `ordinal-fs-tree`-only
wording.

## Decisions (running log)

The recovery is a new contemporaneous paired campaign, not a continuation of
the frozen historical sample. Its criteria trace to the parent acceptance
contract independently of the treatment bytes; the deployed skill is mapped to
those criteria only after the requirement set is fixed. Exact primary and
regression sets, absolute attainment thresholds, per-behavior-family gates,
empty-set semantics, and incomplete-campaign semantics are frozen before any
arm runs. No set is selected from the new control outcomes, and transfer remains
a separate verdict.

Each case assigns five control/enabled pairs under a precommitted,
counterbalanced order. A run may be replaced only for a failure proven to occur
before prompt or treatment exposure. Once exposure occurs, discovery failure,
access-rule violation, timeout, refusal, truncation, or missing final output is
retained under the frozen outcome rule rather than selected away. Exact skill
delivery, a visible read of the pinned `SKILL.md`, announcement, and behavioral
adherence are four different observations; none substitutes for another and no
sample is filtered on any of them.

The campaign apparatus is built before it is frozen. Measurement design,
runner, access auditor and scoring-bundle builder, and the separately blinded
transfer fixture are completed first; one joint freeze then pins every prompt,
criterion, schedule, fixture, template, digest, outcome rule, scorer instrument,
and deterministic test before treatment execution. The access contract uses a
narrow auditable command grammar over the active skill directory and declared
fixture, not a claim that the host filesystem was unreadable. Full events go to
the access audit; treatment-neutral normalized bundles go to blind behavioral
scorers.

Same-case generation and adjudication are separate focused leaves for each of
scope elicitation, source/fragments, and exposition/assurance. A separate
transfer node runs against its pre-frozen external target, and a dedicated
deployment-verification leaf reruns plugin and repository checks at the pinned
digest before the final report. Every run or adjudication leaf can retire with
a complete result, a retained protocol-failure result, or a predeclared
unavailable result; none depends on obtaining a favorable or fully populated
sample.

The first adversarial pass found substantive defects in the initial campaign
shape. The corrected decomposition therefore earns a scheduled
`review-planning` step before any campaign child runs. That reviewer re-derives
the plan from this committed artifact and the parent contract; any integration
leaf remains lazy and is cut only if the review records actionable findings.
