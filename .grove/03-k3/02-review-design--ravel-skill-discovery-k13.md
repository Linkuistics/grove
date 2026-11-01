# ravel-skill-discovery-k13

**Reviews:** ravel-skill-discovery-k7

## Goal

Adversarially review Ravel's generic skill-discovery contract before Grove's
adoption design or implementation planning treats it as settled.

## Context

Read the producer's committed artifacts in
`/Users/antony/Development/AgenticOrchestration/Ravel`, following that repository's
instructions and VCS discipline. The producer task records its owning-repository
commit. The behavioral owners are `docs/specs/skill-discovery.md` and the updated
`docs/specs/session-execution.md`; the companion source/coverage evidence is
`docs/skill-discovery-design-evidence.md`, with editable views under
`docs/design/skill-discovery/`.

Producer's Ravel commit: `b68af6e492316494dfce7ec2c0ee77a1a996e60b`
(jj change `kqtnkxrunqnmuypxwtmsqnmvmvuxxukk`).

The human approved the presented direction, not a claim that it is implemented.
Use the root and common-skill-loading briefs as requirements. Product code remains
Ravel 1.1, so distinguish intended interface changes from current source facts.

## Done when

- Read the complete contract against accepted requirements and the real launch,
  policy-worker and confinement seams, refreshing graph coverage/source evidence.
- Challenge immutable capture/publication, exact Git identity/offline behavior,
  concurrent readers, untracked delegate lifetime and manual cleanup. Look for
  failure cases that the acceptance table cannot discriminate.
- Challenge whether delegate read access and removal of enclosing ending
  authority are feasible through real harness adapters; separate actual Ravel
  guarantees from instructed behavior and explicitly unsupported nested launches.
- Challenge the no-source/bootstrap behavior, duplicate/native handling,
  original/prepared prompt byte contract, request/context compatibility and
  schema/version migration. Check for contradictory ownership across the specs.
- Produce findings without fixes. Cut an integrate-review-design step immediately
  before the dependent Grove adoption leaf only if actionable findings earn it;
  its charter points to this review rather than restating the findings.

## Notes

This is the load-bearing contract for every later external pack. No additional
in-session reviewer was used. The finishing producer had no
`GROVE_PROVIDER_EVIDENCE` in its environment, so no Creator line was inferred.
If the owner policy requires provider evidence, its owner-declaration remedy
applies; do not reconstruct a receipt from model names or historical sessions.
