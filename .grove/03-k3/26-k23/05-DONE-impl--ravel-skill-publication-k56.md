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

## Decisions (running log)

- Entry base. `jj git fetch` changed nothing, and Ravel `main` is still
  `24a545236185`. The line holds k55's `d870d0fc` above k54's `777e609f`, so
  no rebase was necessary at entry.
- Filesystem facts, probed on this Mac. A case-sensitive APFS image keeps `A`
  and `a`. APFS folds NFC and NFD on both variants, so no APFS source holds
  both forms. An HFS+ image stores an NFC name as NFD. The fixtures use a
  case-sensitive APFS image for the case rows and an HFS+ image for the
  spelling row. A Linux run keeps both normalisation forms on its default
  filesystem.
- Held identities. Every cache write goes through an open directory
  descriptor: `mkdirat`, `openat` with `O_NOFOLLOW`, `symlinkat` and
  `renameatx_np(RENAME_EXCL)` or `renameat2(RENAME_NOREPLACE)` between held
  descriptors. Ravel checks before each publication and at the end that the
  lexical cache root still reaches the held identity, else
  `skill_cache_unavailable`.
- Safe cache path. Ravel resolves `$HOME/.cache/ravel/skills/v1` one segment at
  a time and checks each directory that it passes. The owner must be the
  account or root. A directory that others can write must be sticky, and its
  entry on the path must belong to the account or root. A link in such a
  directory must also belong to the account or root. The cache root and its
  three directories must belong to the account and give no group or other
  access.
- Change detection. Capture reads the source through held descriptors and
  looks at each entry twice. The first look is the status at the listing.
  The second look has three parts. First, Ravel reads the status of the open
  file after its copy. Next, it reads the listing and the status of each
  entry again after the directory. Last, it reads the status of the
  directory itself. A difference of identity, type, size, modification
  time or change time refuses as `skill_source_changed`. The access time is
  not compared. A file opens with `O_NOFOLLOW | O_NONBLOCK`, so a link or a
  FIFO that replaced it cannot redirect or hold the read.
- Path identity. A create that meets an existing name (`EEXIST`) is a
  collision. Ravel finds the earlier entry by the identity that it recorded
  and names both source paths. After the walk, Ravel reads the written tree
  back, with every name and every byte, and compares it with the source
  entries. A different spelling refuses as `skill_resource_invalid` and names
  the source path and the stored spelling. Different bytes refuse as
  `skill_cache_unavailable`.
- Winner check. A loser and every reuse read the existing object back in
  full. The object must hold exactly `manifest` and `tree`, with the same
  manifest bytes. A manifest computed again from the tree must also agree. A descriptor object must hold exactly
  `directories.json` with the same bytes. Anything else refuses as
  `skill_cache_corrupt`. The quarantine remedy names the object. Ravel changes
  nothing in it. The check does not compare write permission, because the
  content guarantee does not depend on it.
- Cleanup. `Staging` holds its parent descriptor and its own name, and
  removes only that tree. A failed removal after a successful publication
  refuses as `skill_cleanup_failed`, exit 5. After a refusal, the failure
  joins the original refusal as a report, as k54 established.
- Internal seams. `skills::faults` holds thread-local hooks for the unit tests
  only: `copied`, `listed`, `publishing` and `removing`. The unit tests in
  `skills/publication_tests.rs` use them for controlled edits, a cache link
  replaced before publication, a planted winner and a failed removal.
- Observation for `ravel-skill-bounds-k57`. On macOS, `sync_all` is
  `F_FULLFSYNC` for each file. A resolution of 600 files of 512 bytes takes
  about 3.2 seconds, at 15% CPU, in a debug build. Each later resolution pays
  the same cost, because it captures again. The bounds stage sets the default
  deadline and can decide whether one device flush before the rename can
  replace the flush of each file.
- Firing controls. Each control ran alone and was restored from a copy. The
  digest of the source files was the same before and after. Each made its
  fixture fail. At first three did not fire, because a second layer saw the
  same edit first. Each case now asserts the message of its own layer. A
  case that changes only one file in place reaches the second look at the
  entry alone. Each control removed one of these:
  - the second look at an entry, the second listing and the look after a copy
  - the written-tree check and the naming of a collision
  - the deep winner check and the root identity check
  - the sticky rule, the privacy rule and the storage remedy
  - the restriction of cleanup to its own staging, and the cleanup code
- **The barrier does not depend on a sleep (2026-11-02).** The first full
  `task check` hung for 100 minutes in the barrier fixture. Under the load of
  the full suite, one publisher shell opened the FIFO after the test had
  opened and closed its write end. That shell then waited for a writer that
  never came. The test now writes one line for each publisher and keeps the
  write end open until every publisher ends. A late publisher reads its line
  and does not block. The sleep before the release only makes it probable
  that the publishers wait together. A shell check, with one reader 1 second
  late, showed these semantics. The fixture passed 15 repeated runs.
- **Output (2026-11-02).** The Ravel commit is `cba154857d41` (change
  `tmxylvwk`) on bookmark `skill-directory-delivery`, with parent
  `d870d0fc`. The bookmark is not pushed. The tree did not change during
  the run. On macOS these tasks pass: `check` (683 tests pass, 48 skip),
  `test:cli`, `docs:book:build`, `docs:examples:test`,
  `docs:excerpts:test`, `docs:excerpts`, `docs:examples`, `docs:figures:test`,
  `docs:ste:test`, `docs:breaks:test`, `book:integrity` and `smoke`. No
  Linux run occurred. Only clippy ran for the aarch64 Linux target.
- **A review is earned.** The compiler cannot check the safe path walk, the
  held identities, the checks against a race or the limits of change
  detection. The review `ravel-skills-resolve-k62` reads an earlier commit.
  So a `review-impl` of this commit comes before `ravel-local-handoff-k61`.
