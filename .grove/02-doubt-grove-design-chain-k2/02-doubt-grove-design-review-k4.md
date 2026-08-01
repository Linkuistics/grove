# doubt-grove-design-review-k4

**Kind:** review-design

## Goal

Adversarially review the doubt/Grove composition design and try to disprove that
it satisfies the confirmed requirements without violating Grove's spine.

## Context

Review the artifact produced by `doubt-grove-design-k3` against the root brief.
Pay special attention to stable-key preservation, current-leaf identity after a
move, atomic failure, jj/git symmetry, chain-node semantics, stateless routing,
warning accuracy after configuration changes, and recursive-review escape
hatches. Produce findings only; do not fix the design.

## Done when

- Every root requirement is traced to a falsifiable design behavior or flagged
  missing.
- Counterexamples cover interrupted promotion, first/last sibling placement,
  already-chained leaves, wrong task kinds, same-target routing, restart, and
  integration/research exclusions.
- Findings are specific, severity-ranked, and recorded in this leaf for
  `doubt-grove-design-integrate-k5`.

## Notes

Assume the design author was overconfident. Do not validate or summarize.
