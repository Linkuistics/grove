# external-pack-contract-k164

**Integrates:** external-pack-contract-k163

## Goal

Triage the findings in external-pack-contract-k163's committed review, and
apply the real design corrections before pack-conformance-runner-k156 uses
the contract.

## Context

- The review's commit, identified by external-pack-contract-k163. Read its
  findings there, rather than treating this body as a list of required fixes.
- The reviewed artifact is `docs/specs/external-pack-contract.md`, with the
  ADRs, reconciled sections and views named by the producer
  external-pack-contract-k155. The review identifies its exact base.

## Done when

- Each finding has a source-supported disposition under the integration
  procedure. Apply accepted corrections to the design and its affected
  handoffs, and record accepted trade-offs or rejected findings with reasons.
- The contract and its cited records are coherent with the brief chain.
  Dependent leaves consume the settled design, and applicable documentation
  verification is recorded.
- Externalise substantial redesign as a producer/review chain before the
  dependent runner work, as the integration procedure directs.

## Notes

This step immediately precedes pack-conformance-runner-k156. Keep design
integration within its charter; implementation and its tests belong to their
own leaves.
