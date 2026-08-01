# doubt-grove-implementation-review-k8

**Kind:** review-impl

## Goal

Adversarially review the implemented doubt/Grove composition for correctness,
safety, compatibility, and fidelity to the integrated design.

## Context

Review the diff and tests from `doubt-grove-implementation-k7` against the root
brief and integrated design. Produce findings only. Exercise public CLI behavior
at the highest available test seam; inspect both jj and git paths.

## Done when

- Atomicity, key preservation, tree ordering, current-leaf continuation, routing
  comparison, non-blocking warnings, and restart behavior are challenged.
- Every task-kind branch and the one-review budget are checked across canonical
  Grove and doubt skill text.
- Backward compatibility and standalone doubt behavior are checked.
- Findings are severity-ranked, reproducible, and recorded in this leaf for
  `doubt-grove-implementation-integrate-k9`.

## Notes

Assume passing tests are incomplete evidence; seek missing assertions and
contracts rather than summarizing the change.
