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

## Decisions (running log)

- Entry base. `jj git fetch` changed nothing, and Ravel `main` is still
  `24a545236185`. The line holds k57's `52d856978b24`, so no rebase was
  necessary at entry.
- Fixture file. The fixtures of this leaf go into a new public test binary,
  `crates/ravel/tests/skills_lifetime.rs`. The rows "Lifetime", "Nested
  sessions" and "Directory recovery" then have one owner file. The existing
  stale-value and disabled-delivery checks in `skills_launch.rs` stay where
  they are, and the recovery fixture repeats them for the managed process.
- A detached reader leaves the process group with `setsid` through
  `/usr/bin/perl`. macOS has no `setsid` command, and the Linux image of the
  native checks installs `perl`. A shell with job control was not usable,
  because the managed process has no terminal in the fixtures.
- Manual removal of a local source. A local source has no offline reuse:
  each resolution captures again. After a quiet removal of the whole cache,
  the next resolution of a present source captures and publishes new objects.
  After the source and its snapshot go, the next resolution refuses as
  `skill_source_missing`, and a neighbour snapshot of the same source ID stays
  unused. Partial removal inside an object is `skill_cache_corrupt`, which
  `skills_publication.rs` already proves. The Git half of "Quiet manual
  removal loses offline reuse" belongs to `ravel-pinned-delivery-k25`.
- Fixtures. `skills_lifetime.rs` holds five public fixtures. (1) An
  independent reader that `skills resolve` started and a reader that left the
  group with `setsid` read the same descriptor, mandatory file and skill
  files. They read after the session ended, after a settings change with a
  new resolution and after the removal of the source. An ordinary background
  descendant is gone when `start-session` returns. (2) Quiet removal of the
  whole cache, then of one snapshot and its descriptor with the source gone.
  (3) Four readers of one snapshot while three resolutions publish changed
  sources. (4) A nested `start-session` in the managed process. (5) Recovery
  through the variable, a child that lost both channels and disabled
  delivery. Its agent counts the environment values that name a cache path.
  The recovered case counts 1, which is the positive control of that count.
- A finding about the fixtures, not a defect. The JSON notice of an outer
  session comes after the text notice of a nested session on standard error,
  because Ravel writes the JSON notice when its job has ended. The nested
  fixture therefore takes the first line that starts with `{`.
- No product defect. The five fixtures passed against k57's code with no
  change to `src`.
- Firing controls, each run alone and restored from a copy. The digest of the
  Rust sources under `crates` was the same before and after
  (`97953d1f6717`). M1: the session removes its descriptor when its job ends.
  The lifetime and nested fixtures failed. M2: an inherited value stays when
  delivery is enabled. The recovery and nested fixtures failed. M3: disabled
  delivery keeps the inherited name. The recovery fixture failed. C1: the
  reader does not call `setsid`. The lifetime fixture failed. C2: a
  publisher writes the first snapshot in place. The concurrent-reader fixture
  failed. Two checks have no control of their own. The neighbour check of the
  removal fixture has no product mutation, because no code path substitutes
  objects. The reap of the ordinary descendant relies on the existing group
  fixtures in `tests/cli.rs`.
- Documentation. Chapter 5 states the nested resolution and the outer
  session's retained paths, the loss of both path channels and the lifetime
  of the resources. The reference states the removal outcome and the nested
  value of the variable. The changelog entries of `skills resolve` and of
  delivery in `start-session` state the same rules. The strict STE check
  passes on chapters 5 and 6. `breaks.sh --strict` reports two prose runs
  over 450 words in chapter 6. The committed parent has both runs, with the
  same word counts, so this leaf did not cause them.
- No review is earned. The leaf changes no product code, and each fixture
  has a firing control except the two checks named above.
- **Output (2026-11-02).** The Ravel commit is `4b0f54c575a7` (change
  `lwzqpnxv`) on bookmark `skill-directory-delivery`, with parent
  `52d856978b24`. The bookmark is not pushed. On macOS, one script on a tree
  whose digest was the same before and after ran `check`, `test:cli`,
  `docs:book:build`, `docs:examples:test`, `docs:excerpts:test`,
  `docs:figures:test`, `docs:ste:test`, `docs:breaks:test`,
  `book:integrity` and `smoke`, and all passed. `check` ran 706 tests: 706
  passed and 48 were skipped. No Linux run occurred.
