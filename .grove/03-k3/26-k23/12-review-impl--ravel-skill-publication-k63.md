# ravel-skill-publication-k63

**Reviews:** ravel-skill-publication-k56

## Goal

Read the capture and publication code of Ravel commit `cba154857d41`
adversarially, against `docs/specs/skill-discovery.md` at `822c0ed2`.
Produce findings, not fixes. The parent `d870d0fc` is the base of the diff.

## Specific doubts

- `crates/ravel/src/skills/cache.rs` walks to the cache root one segment at
  a time and follows each link by hand. Consider a link, a `..` segment and
  a directory that another account can write. Can one of them bring the walk
  to a root that the checks do not cover? Does the order of `kept`, `passable` and the
  identity check leave a gap?
- `Cache::check` compares the identity of the held root with the lexical
  path and the real path just before each rename. The staging checks
  `check_roots` and `check_mandatory` in `skills/mod.rs` read the staging
  by path strings, not by descriptors. Can the same account replace a
  directory between those checks and the rename, so that the manifest
  describes bytes that Ravel did not copy?
- `Walk::directory` in `skills/snapshot.rs` takes a first and a second look
  at each entry, by device, inode, type, size, mtime and ctime. Which edits
  can it miss? Consider an edit within the timestamp resolution of the
  filesystem, and a replacement that keeps all six fields. The book states
  that capture is not atomic. Does any test or message claim more?
- `read_tree` compares the written tree with the source entries. It reports
  a missing path as `skill_resource_invalid` and other differences as
  `skill_cache_unavailable`. Does that division hold for each case and
  normalisation behaviour of "Path identity"? Does the collision message
  always name both source paths?
- `publish` treats `EEXIST` and `ENOTEMPTY` from the rename as a lost race.
  It then reads the winner back with `Expected::holds`. Can a winner that
  differs from the staging pass that check? Can a reader see an object
  before its root is sealed at 0555?
- `Staging` removes its tree when it drops. `Dir::remove_tree` restores mode
  0700 on each directory. Can cleanup remove anything outside its own
  staging, for example through a link inside the staging? Does a cleanup
  failure always give `skill_cleanup_failed` with exit 5, and keep the code
  of an earlier refusal?
- `unavailable` in `cache.rs` chooses the remedy from the errno. Do full
  storage and a permission failure reach that function from each write
  path, or does some path report the general remedy?
- The barrier fixture in `crates/ravel/tests/skills_publication.rs` first
  depended on a sleep and hung under the full suite. Consider the other
  fixtures, such as the killed publisher and the controlled edits. Can their
  timing make them hang, or pass without proof?
- No Linux run occurred. Only clippy ran for the aarch64 Linux target. Read
  the Linux paths of `dir.rs`, such as `renameat2` with `RENAME_NOREPLACE`
  and `__errno_location`. Read the Linux arm of each fixture too. Look for
  an error that only a run would show.

## Done when

- Each doubt has a finding or a reasoned statement that the code holds.
- The findings cite file and line at `cba154857d41`.
