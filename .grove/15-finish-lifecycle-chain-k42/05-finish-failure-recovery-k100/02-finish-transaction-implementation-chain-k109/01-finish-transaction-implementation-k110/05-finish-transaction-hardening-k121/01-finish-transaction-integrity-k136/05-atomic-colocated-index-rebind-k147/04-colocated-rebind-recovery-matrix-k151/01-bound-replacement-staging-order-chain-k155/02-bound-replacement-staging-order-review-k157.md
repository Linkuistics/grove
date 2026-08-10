# bound-replacement-staging-order-review-k157

**Kind:** review-impl
**Reviews:** bound-replacement-staging-order-k156

## Goal

Adversarially review `bound-replacement-staging-order-k156` and record concrete
findings for its integration step.

## Context

- Inspection-only by default: read the producer commit and reason about it.
  Running the existing suite is fine; if disproving a claim needs a temporary
  in-tree probe, disclose it under a `## Verification run` heading and leave the
  working tree byte-identical to the reviewed producer commit.
- Record findings only. `bound-replacement-staging-order-integrate-k158` owns
  every fix and all post-fix verification.
- The claim under test is that the inverted publication order removes the
  unowned-entry state entirely rather than merely narrowing it. Attack that
  directly: enumerate every interruption point in the new sequence and ask what
  is on disk with no live document describing it, and what a later recovery,
  disposal, activation, or same-attempt retry then does with it.
- Also attack the new staging namespace as an authority: a forged or substituted
  state document names both the staging entry and the deterministic replacement
  name, so ask what the strongest reachable redirection is now.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The working tree is left byte-identical to the reviewed producer commit.
