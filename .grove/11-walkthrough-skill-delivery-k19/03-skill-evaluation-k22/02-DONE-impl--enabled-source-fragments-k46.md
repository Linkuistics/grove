# enabled-source-fragments-k46


## Goal

Run and score the frozen external-source inventory and fragment-completeness
scenario in fresh contexts with the unchanged skill enabled.

## Context

- Frozen contract: `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Baseline comparison: `docs/evaluations/writing-code-walkthroughs/baseline/source-fragments/`.
- Preserve outcomes under `docs/evaluations/writing-code-walkthroughs/enabled/source-fragments/`.

## Done when

- The frozen sample size, prompt, fixtures, access boundary, contamination
  controls, replacement rule, and interleaved five-enabled/five-control schedule
  are applied unchanged.
- Every valid run preserves its skill revision, access manifest, raw answer,
  atomic scores, and adjudication notes.
- Sample shortfalls and infrastructure failures remain distinct from behavior;
  the skill is not refined in this slice.

## Notes

Use the same pre-refinement skill revision measured by
`enabled-scope-elicitation-k45`.

## Decisions (running log)

- Preserve the frozen prompt's canonical terminal LF in the `codex exec`
  argument by appending a sentinel during command substitution and removing
  only that sentinel. Verify the resulting argument digest before every
  attempt. This closes the protocol breach observed in the preceding enabled
  slice without changing the prompt bytes.
- Treat MCP resource discovery as an access-boundary violation in Case B. It
  accesses plugin catalog material rather than the sole declared fixture, so a
  run containing it is invalid and consumes a replacement even when the call is
  read-only and the process returns a normal final answer.
- Stop the scenario after enabled repetition 1 exhausts its third attempt, as
  the frozen replacement rule requires. Preserve the preceding control sample
  and all three invalid enabled attempts, but do not run the remaining eight
  schedule positions. Report those positions as unexecuted after the sample
  shortfall, not as behavioral failures or additional invalid attempts.
- Retain control repetition 1 as the selected refusal under the exhaustive
  replacement predicates, while marking it access-audit protocol-breached: its
  answer quotes blocked shell results that the JSONL stream does not expose as
  direct tool events. Score it descriptively, not as comparison evidence.
- Resolve the blind primary's `B01` and `B19` awards to zero under the exact
  conjunctive wording. An intention to establish an inventory is not an
  inventory, and read-only source inspection is not explicitly read-only,
  non-regenerating validation. The resolved descriptive score is `1/27`, with
  only absence-shaped `B18` succeeding.
- Apply both fresh-context doubt findings as bounded report corrections: all
  three enabled attempts share the unstructured access-audit breach around
  intended skill reads, while only attempts 2 and 3 name
  `writing-code-walkthroughs` exactly. Do not infer an outside-boundary shell
  operand from a call event the JSONL does not contain.
