# ravel-skill-lifetime-k58

## Goal

Prove that retained resources outlive their sessions. Prove that nested
sessions keep their own directories and result channels, and that the recovery
environment behaves as the contract states. Repair any defect that these
fixtures expose.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`:
  "`RAVEL_SKILL_DIRECTORIES`", "Publication, concurrency and cleanup" and the
  nested-session rules under "Read access and delegation".
- Ravel `docs/adr/skill-resources-outlive-sessions.md` and
  `docs/adr/ravel-supervises-the-session.md`.

## Done when

- Independent and detached readers keep their descriptor and resources after
  the parent exits, after the source is removed and after the settings change.
  Ordinary managed descendants are reaped with the group.
- A manual removal of a quiet cache loses reuse and refuses clearly at the
  next resolution. It never substitutes a neighbour object.
- Concurrent readers of one snapshot read the same bytes while a new resolution
  publishes a changed source.
- An explicit nested `start-session` in a managed process resolves current
  sources and returns only its own result. The parent keeps its descriptor
  path, its directories and its channel. Only the nested managed process
  receives the new `RAVEL_SKILL_DIRECTORIES` value.
- A stale inherited `RAVEL_SKILL_DIRECTORIES` is replaced, and disabled
  delivery removes it. A fixture loses the prompt path and recovers the same
  descriptor through the variable. A fixture without both path channels shows
  that nothing supplies a guessed path.
- Public fixtures cover the acceptance row "Lifetime". They also cover the
  unconfined part of "Nested sessions" and the fixture part of "Directory
  recovery". Agent behaviour after compaction belongs to
  `common-loading-acceptance-k29`.
- The book pages on sessions and lifetime and `CHANGELOG.md` agree. The
  affected Ravel checks pass. The bookmark advances by jj, and nobody pushes it.

## Notes

Ravel has no collector and no prune command. Do not add either to make a
fixture convenient.
