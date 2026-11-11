# ravel-release-promote-k328

**Integrates:** `ravel-release-promote-k326`

## Goal

Triage the promotion review against the release contract, apply the
actionable corrections and verify the resulting tooling before the book
describes it or a public promotion runs.

## Context

- Read the committed findings and source-backed doubt dispositions in
  `ravel-release-promote-k326`.
- The producer is `ravel-release-promote-k306`. Its artifact is on
  `skill-directory-delivery` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
  The review reads current source at `62cccfdbb0d5`, including k327's
  first-versioned-installation correction.
- Ravel `docs/specs/release-channels.md` owns the promotion contract.
  Grove `docs/specs/coordinated-release.md` owns publication order and
  retention of the original prefixes.

## Done when

- Every finding has an evidence-backed disposition. Accepted corrections
  and appropriate fresh verification are recorded in their owning repo.
- No release, tag, public tap change or Homebrew installation is published
  by this leaf.

## Notes

Anchor source coordinates to the reviewed commit while triaging. The
review ran no test or installation command. The actual Homebrew upgrade
rehearsal remains with `promotion-upgrade-rehearsal-k318`. Preserve the
running driver's full original 1.1.0 keg and ending command.
