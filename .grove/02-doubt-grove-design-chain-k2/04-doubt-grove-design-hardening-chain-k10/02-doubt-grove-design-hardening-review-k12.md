# doubt-grove-design-hardening-review-k12

**Kind:** review-design

## Goal

Adversarially review the hardened doubt/Grove design and try to disprove its
concurrency, interruption, receipt-freshness, and warning-payload guarantees.

## Context

Review the artifact produced by `doubt-grove-design-hardening-k11` against the
root brief and the seven source findings recorded there. Pay special attention
to concurrent promotion, process interruption versus power loss, VCS index
state, launch-window divergence, retries that cannot write a receipt, nullable
model identity, and missing relationships.

## Done when

- Every source finding is traced to a falsifiable behavior or remains open.
- Counterexamples cover two concurrent promoters, interruption at every
  mutation, producer restart under a different target, and every nullable-model
  pairing.
- Findings are severity-ranked and recorded in this leaf for
  `doubt-grove-design-hardening-integrate-k13`; do not fix the design.

## Notes

Assume the hardening author closed the obvious example while leaving a nearby
state transition undefined. Report issues only.
