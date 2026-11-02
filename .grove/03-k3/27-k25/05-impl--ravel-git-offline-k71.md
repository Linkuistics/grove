# ravel-git-offline-k71

## Goal

Deliver offline reuse of exact Git resolutions. A checked cache hit of the
acquisition key needs no network, no Git executable and no source repository.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Exact Git acquisition
  and offline reuse", "Publication, concurrency and cleanup" and "Read access
  and delegation" for the origin check of reused snapshots.
- The online acquisition of `ravel-git-acquire-k70`.
- The publication fixtures of `ravel-skill-publication-k56`.

## Done when

- A versioned acquisition key holds the exact locator, the object format, the
  full commit ID and `subdir`. The source ID is not part of it. Two spellings
  of one remote are two keys.
- The key record is published atomically and never replaces an existing
  object. When two publishers of one key race, the loser checks the winner. A
  winner whose content differs is `skill_cache_corrupt`.
- A cache hit checks the key record, the manifest and the content, then reuses
  the snapshot with no network operation and no freshness check. A hit does not
  look up Git on `PATH`.
- A tampered key record or snapshot refuses as `skill_cache_corrupt`, with the
  quarantine-or-remove remedy. Ravel neither accepts nor repairs it.
- Reuse checks the origin of the cached resolution against the protected paths
  of this launch, with its actual `--config` directory. A cached copy of a
  protected file cannot pass where the original would fail.
- Fixtures cover "Offline reuse": warm the exact cache, then remove the remote
  and put no Git on `PATH`. Resolution and launch succeed, and a transport spy
  on `PATH` records zero calls. A cold absent pin beside a cached neighbour
  refuses as `skill_revision_unavailable`.
- Fixtures cover a pin change, a moved remote ref and a removed repository.
  Old readers keep their bytes, and the new pin is a new key. They also cover
  the Git parts of "Concurrent publication" and the tampered snapshot of
  "Missing or corrupt revision".
- The book, `CONTEXT.md` and `CHANGELOG.md` describe offline reuse and manual
  removal, which gives up offline reuse. The affected Ravel checks pass.

## Notes

The cache layout stays internal. A new directory under the `v1` root is
acceptable, because nothing has published that root. Ravel has no collector
and no prune command, and this leaf adds none.
