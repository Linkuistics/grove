# doubt-grove-design-integrate-k5

**Kind:** integrate-review-design

## Goal

Verify and integrate the design-review findings so the resulting spec and
decision set are ready to drive implementation.

## Context

Read the design artifact from `doubt-grove-design-k3` and findings in
`doubt-grove-design-review-k4`. Classify each finding as contract misread,
actionable issue, accepted visible trade-off, or noise before editing.

## Done when

- Every review finding is classified with evidence and each real issue is fixed.
- The spec/ADR set remains a minimum coherent current-state set.
- Root-brief pointers or terminology are reconciled if the design sharpened
  them.
- Implementation can proceed without inventing promotion, routing, or test
  semantics.

## Notes

If a finding changes a human-owned requirement rather than clarifying it, stop
and ask rather than silently rewriting the baseline.
