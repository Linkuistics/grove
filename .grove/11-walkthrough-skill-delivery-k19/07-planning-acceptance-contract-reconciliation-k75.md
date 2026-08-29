# acceptance-contract-reconciliation-k75

## Goal

Reconcile the recovery campaign with the parent brief's requirement for a
pre-skill control followed by the same scenarios and unchanged rubric, so the
campaign report cannot substitute a new contemporaneous comparison for the
accepted experiment.

## Context

- Parent contract: `walkthrough-skill-delivery-k19` and the root brief.
- Existing recovery plan: `evaluation-recovery-k53`,
  `evaluation-recovery-k73`, `evaluation-recovery-k74`, and the
  `recovery-campaign-k54` subtree.
- Frozen historical evidence and instrument:
  `docs/evaluations/writing-code-walkthroughs/README.md` and
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.

## Done when

- A clause-by-clause trace states exactly which parent acceptance claims the
  historical pre-skill records, unchanged historical rubric, recovery
  contemporaneous controls, and new enabled runs may each discharge.
- The plan fixes whether every recovery prompt, fixture, atomic row, threshold,
  endpoint, and regression rule must remain byte-identical to the historical
  instrument. Any new requirement-derived row is explicitly supplemental and
  cannot replace an unchanged-rubric comparison without a separately authorized
  requirements change.
- The recovery subtree is reshaped so `campaign-report-k72` cannot call a new
  control arm a pre-skill baseline, cannot claim an altered instrument met the
  unchanged rubric, and cannot silently drop a parent-required comparison.
- Historical rubric bytes and records remain untouched. If the parent contract
  is impossible to discharge from valid evidence, the plan names the precise
  requirements decision needed and externalizes it rather than weakening the
  acceptance claim.
- No evaluated treatment or control context runs. `measurement-design-k55`
  remains sequenced behind this reconciliation and its review.
- As the final planning action, commission a lazy `review-planning` sibling with
  this leaf's bare stem and a charter to try to disprove the reconciled
  clause-to-evidence trace before campaign execution.

## Notes

This is the substantial redesign surfaced by the integration leaf's one
fresh-context doubt review. It is separate from applying F1–F14: those repairs
remain valid constraints on whatever recovery shape survives reconciliation.
