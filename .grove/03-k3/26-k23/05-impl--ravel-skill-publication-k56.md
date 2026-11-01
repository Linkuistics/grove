# ravel-skill-publication-k56

## Goal

Prove that local capture and publication never pair a manifest with bytes that
Ravel did not copy, and never expose a partial object. The proofs run at both
public seams, `skills resolve` and `start-session`.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Local freshness" and
  "Publication, concurrency and cleanup".
- The resolver of `ravel-skills-resolve-k54` and the launch of
  `ravel-skill-launch-k55`.

## Done when

- Capture detects an observable change of entry, type, identity, size or
  content during its scan. It then refuses as `skill_source_changed` with a
  retry remedy. Digests before and after capture and controlled concurrent
  edits prove it. No partial descriptor stays.
- Ravel checks the written tree against the source manifest, with every
  directory segment. Two source paths that the cache filesystem cannot keep
  apart refuse as `skill_resource_invalid` and name both paths. That includes
  collisions of case and of Unicode normalisation. A filesystem that keeps
  both paths keeps both. No entry is written over another or normalised.
- Publishers behind a barrier and an existing reader see no partial object.
  The losing publisher checks the winner before it removes its copy. A winner
  with different content refuses as `skill_cache_corrupt`. A killed publisher
  cannot publish a partial descriptor.
- A tampered published snapshot refuses as `skill_cache_corrupt`, with the
  quarantine remedy. Ravel does not repair it under its readers.
- A cache root that a replaced or unsafe symlink reaches refuses as
  `skill_cache_unavailable`. So do a permission failure and full storage.
  Cache writes use held directory identities and staging that only the owner
  can open.
- Ordinary cleanup removes only the staging of its own resolver. A cleanup
  failure refuses as `skill_cleanup_failed`, exit 5.
- Public fixtures cover the acceptance rows "Path identity", "Concurrent
  publication" and the controlled-edit part of "Local freshness". They also
  cover the tampered-snapshot case of "Missing or corrupt revision". Fault
  injection stays behind internal seams. At least one fixture uses a
  case-insensitive filesystem of the release platform.
- The affected book pages and `CHANGELOG.md` agree. The affected Ravel checks
  pass. The bookmark advances by jj, and nobody pushes it.

## Notes

Capture is not atomic against every concurrent writer. A successful retry does
not prove atomicity. State that limit in the tests and the documentation.
