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
- Also attack the new staging namespace as an authority. The exact-name pin the
  previous review won for `staged_artifact_name` became a **shape** pin, so ask
  what the strongest reachable redirection is now, and whether the producer's
  argument for that weakening — that a forged document can only name entries its
  author created — actually holds on every path that reads it.
- Two adjacent questions the producer saw and deliberately left alone, both fair
  game: `staged_name`, the staged *marker*, still has no namespace pin at all and
  is gated only by parsing as a Grove marker; and `dispose` on a struct built
  before a mid-settle failure may act on a stale marker snapshot.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The working tree is left byte-identical to the reviewed producer commit.
