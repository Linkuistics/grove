# doubt-grove-design-hardening-integrate-k13

**Kind:** integrate-review-design

## Goal

Classify and integrate the hardening review so the spec and ADR set are safe to
drive implementation.

## Context

Read the artifact from `doubt-grove-design-hardening-k11` and findings in
`doubt-grove-design-hardening-review-k12`. Classify each as contract misread,
actionable issue, accepted visible trade-off, or noise before editing.

## Done when

- Every review finding is classified with evidence and each real issue is fixed.
- Promotion, receipts, nullable-model comparison, warning payloads, and VCS
  semantics require no implementation-time design invention.
- The spec/ADR set remains a minimum coherent current-state set and the root
  brief/glossary are reconciled if the contract sharpened.

## Notes

If a finding changes a human-owned requirement rather than clarifying it, stop
and ask rather than silently rewriting the baseline.
