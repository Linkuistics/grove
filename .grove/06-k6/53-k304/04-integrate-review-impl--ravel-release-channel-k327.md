# ravel-release-channel-k327

**Integrates:** `ravel-release-channel-k325`

## Goal

Triage the review's findings against Ravel's release-channel contract, apply
the actionable corrections and verify the resulting release tooling before
its first publication.

## Context

- Read the findings and source-backed doubt dispositions in
  `ravel-release-channel-k325` from its committed review.
- The producer is `ravel-release-channel-k305`. The current artifact is on
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
- Ravel `docs/specs/release-channels.md` owns the channel mechanics. Grove
  `docs/specs/coordinated-release.md` owns the publication order.

## Done when

- Every finding has an evidence-backed disposition, and the accepted
  corrections have appropriate verification recorded in the owning repo.
- No release, tag, tap change or installation is published by this leaf.

## Notes

Keep the review's source coordinates anchored to its reviewed commit when
classifying the findings. Preserve the running driver's full Ravel 1.1.0
keg and original ending command. The promotion review follows this step.
