# baseline-report-k41


## Goal

Synthesize all frozen-rubric baseline outcomes into the authoring evidence for
the future walkthrough skill.

## Context

- Rubric and all three run directories under
  `docs/evaluations/writing-code-walkthroughs/baseline/`.
- Downstream producer: `writing-code-walkthroughs-k21`.

## Done when

- Aggregate scores, recurring omissions, and representative rationalizations
  are reported without discarding raw outcomes.
- Observed failures are separated from already-reliable behavior and from
  constraints better enforced mechanically.
- Every candidate skill rule maps to a repeated observed gap; behavior whose
  control did not fail is explicitly excluded from the authoring brief.
- The report supplies the unchanged cases and rubric to `skill-evaluation-k22`.

## Notes

This leaf interprets evidence; it does not write or scaffold the skill.

## Decisions (running log)

- The durable synthesis is `docs/evaluations/writing-code-walkthroughs/baseline/README.md`, beside the frozen rubric and the three case directories it summarizes.
- Only Case A can support behavioral classifications and candidate skill guidance: Cases B and C exhausted their replacement allowances without a final answer, so their infrastructure failures remain sample shortfalls rather than behavioral failures.
- The authoring brief groups the fifteen repeated Case A gaps into six rules while retaining the atomic criterion mapping. Mixed and present-in-sample behavior is explicitly excluded, and deterministic properties remain assigned to mechanical checks without being claimed as observed baseline gaps.
