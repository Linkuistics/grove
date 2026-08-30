# same-case-verdict-k67

## Goal

Apply the unchanged historical primary acceptance contract across all three
same-case records.

## Context

- Case records: `scope-adjudication-k62`,
  `source-fragment-adjudication-k64`, and `exposition-adjudication-k66`.
- Endpoint membership and all arithmetic come only from `campaign-freeze-k59`.

## Done when

- Every historical row and count is reproduced without a score, criterion,
  membership, or denominator change. The sole pre-skill baseline is the
  historical record; the new no-skill arm is the contemporaneous comparator.
- The acceptance verdict applies exactly `R`, `G`, the `2/5` materiality test,
  the `10/15` endpoint, and the historical incomplete/truncation exclusions. It
  contains no new absolute, per-family, mixed-row, or fail-closed gate.
- The verdict states that Cases B and C can contribute only descriptive row
  counts and terminal shortfall records because they lack historical baseline
  operands. Their materiality and regression classifications remain
  `undefined`, not zero, miss, or omitted, and neither case enters `R` or `G`.
- Any new absolute attainment, per-family gate, expanded regression set, pair-
  aware rule, or missing-data outcome is excluded and cannot rescue or weaken
  the verdict.
- The record carries the original failed enabled campaign beside the authorized
  replication and applies the exact retry authority settled by the requirements
  leaf; it never silently substitutes the new sample for the old one.
- Historical skill-manifest, discovery, invalid-attempt, truncation, and scorer-
  disagreement fields remain observations, never sample filters beyond the
  exact historical invalid-run rules.
- The record states only what this bounded execution sample establishes and
  does not claim reader comprehension, population generality, or that a visible
  skill read proves behavioral use.

## Notes

Transfer evidence is neither available nor needed here and cannot rescue or
weaken the same-case verdict.
