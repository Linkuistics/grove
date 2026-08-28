# baseline-rubric-k43

**Reviews:** baseline-rubric-k38

## Goal

Adversarially verify that the frozen baseline rubric can support a fair,
auditable same-case comparison without overstating isolation, reproducibility,
or generalization.

## Context

- Producer artifact: `baseline-rubric-k38` and its commit.
- Primary artifact:
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Frozen fixture under that rubric's `fixtures/external-check-floor/` directory.

## Done when

- The review checks prompt/criterion alignment, atomic scoring, invalid-run
  selection, treatment/control equivalence, access evidence, fixture portability,
  runtime drift controls, and the stated limit on same-case conclusions.
- Findings cite the producer commit and exact artifact locations.
- The review does not run a baseline scenario or edit the producer artifact.

## Notes

The producer already spent its one in-session doubt pass and made substantive
changes. This tree-level review is the independent re-read before any baseline
run. If it finds actionable issues, insert an `integrate-review-impl` leaf ahead
of the first run leaf.
