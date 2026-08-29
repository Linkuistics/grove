# scope-adjudication-k62

## Goal

Audit, blindly score, and publish the frozen scope-elicitation case result.

## Context

- Assignment records: `scope-runs-k61`.
- Auditor, scorer instruments, criteria, and outcome rules: joint campaign
  freeze from `campaign-freeze-k59`.

## Done when

- Before scoring, the deterministic auditor replays replacement legality over
  the complete preserved attempt history and classifies every access, exposure
  phase, retained outcome, and treatment-delivery observation. Any illegal
  replacement marks the case protocol-failed and excludes it from attainment.
- Randomized normalized bundles are scored by the two frozen independent blind
  contexts regardless of case completeness; each scorer records a forced arm
  guess per bundle, and disagreements receive the predeclared blind resolution
  with atomic citations and no arm-aware reinterpretation.
- The case record reports all assigned outcomes, per-row and pair-aware counts,
  absolute attainment, material deltas, regression rows, scorer disagreement,
  arm-guess accuracy as a limitation, and discovery/read/announcement/adherence
  separately.
- Complete, protocol-failed, and unavailable case endings are rendered exactly
  as frozen; missing, unavailable, protocol-failed, and unblindable data never
  contribute to attainment, and no cross-case acceptance claim is made.

## Notes

Full raw events remain in the audit record and outside the behavioral scoring
bundle except for normalized case evidence the frozen criteria require.
