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

## Decisions (running log)

- Review boundary: Ravel commit `cba154857d410aa55b0ee180ddccdeab837ea36d`,
  parent `d870d0fc`, against the discovery specification at `822c0ed2`.
  Read the committed capture, cache, descriptor, directory operations, public
  and internal fixtures, documentation diff and producer verification record.
  No test, build, lint, format or reproduction command ran. No Ravel file changed.
- Evidence tier: Verify, with direct-source fallback. The nearest indexed
  Ravel project is the default checkout, generation `2026-11-01T07:59:38Z`.
  The skills query has zero results and coverage reports the feature files
  missing there. It cannot establish feature coverage. Historical `jj file
  show` reads supply the evidence; scratch copies preserve their line numbers.
- Six source-derived findings warrant integration: cache child-directory
  identity, staging name binding, lossy integrity comparisons, symlink
  portability, the concurrent-editor assertion and the kill fixture's
  unobserved execution stage. They are medium priority. These are static
  counterexamples and evidence gaps, not executed reproductions.
- Reconciled the affected paths with current Ravel feature commit
  `aa0e6e88a8dd22cd5c293fa7099244710236ea27`. Bounds and reserved-entry
  alias handling have advanced since the reviewed commit. They do not close
  these findings. The earlier resolver review and its integration own their
  four findings; this review does not repeat them.
- Inserted `ravel-skill-publication-k67` as `integrate-review-impl`
  immediately after this review, before the bounds review. It consumes this
  review's handle and owns triage, fixes and verification. The local-delivery
  parent has live work remaining, so retirement closes no parent node.

## Findings

All locations below refer to Ravel commit `cba154857d41`. P2 means medium
priority. The replacement cases exercise the stated held-identity and cleanup
contracts; they make no claim of authentication against arbitrary code of the
same account.

### 1. P2 — A replaced cache child directory passes the root identity check

Location: `crates/ravel/src/skills/cache.rs:88`, with the held child
directories opened at line 71 and reported paths constructed at
`crates/ravel/src/skills/snapshot.rs:836`.

After `Cache::open` holds its directories, but before the first publication,
rename `v1/snapshots` to `v1/snapshots-old` and create a new private directory
at `v1/snapshots`. Keep `v1` itself unchanged. This can be placed at the
existing prepublication hook; no narrow syscall race is necessary.

Both checks of the root still succeed. Publication writes through the held
descriptor into `snapshots-old`, but returns the stored path under the new,
empty `snapshots`. Descriptor publication then succeeds normally and the final
root check also passes. Resolution reports success with snapshot paths that
do not exist. Replacing `descriptors` instead makes the reported descriptor
path absent. Holding the parent prevents redirected writes, but does not
prove that returned paths reach the held child directories. The existing
replacement fixture changes `.cache` itself and exercises only the root check.

### 2. P2 — Publication and cleanup operate on an unchecked staging name

Location: `crates/ravel/src/skills/snapshot.rs:846`, with cleanup at line 106
and recursive removal at `crates/ravel/src/skills/dir.rs:297`.

Let the capture finish its readback, mandatory checks, manifest write and
sealing. At the prepublication hook, rename its `staging/capture-<random>`
directory aside and put a different directory at that exact name. Leave the
cache root and its three child directories unchanged. The replacement can
contain the original manifest and an altered resource, or no tree at all.

The rename uses the parent descriptor and the name, so it publishes the
replacement. The successful branch never calls `Expected::holds`; its final
chmod acts on the held original staging directory, which is now elsewhere.
The resolver can return a descriptor for bytes it never copied or checked.
Path-based root and mandatory checks earlier in the operation cannot establish
the identity of the entry later moved by name.

The same missing binding affects cleanup independently: after moving the
original staging aside, move another resolver's real staging directory into
that name and cause a refusal. `cleanup` opens and recursively removes that
replacement without comparing it with `self.dir`. It can therefore erase
another resolver's staging. A stable symlink inside staging is not an escape:
the removal routine unlinks it without following it. The defect is a substituted
real directory, despite retaining the original directory descriptor.

### 3. P2 — Lossy UTF-8 conversion can accept a different winner

Location: `crates/ravel/src/skills/snapshot.rs:679`, with the same conversion
for link targets at line 711 and the winner comparison at line 816.

Use a source whose filename is the valid UTF-8 string `s/\uFFFD.md`, where
`\uFFFD` denotes the literal replacement character U+FFFD. Publish it, then
rename that cached file to the raw filename bytes `s/\xFF.md`, preserving its
content and executable bit, and leave the manifest untouched. Both raw names
are possible on Linux. Resolve the unchanged source again.

The manifest name remains valid UTF-8. `read_tree` decodes the invalid cached
byte `FF` to U+FFFD, so its recomputed manifest equals the expected manifest.
The resolver reuses the corrupt object, although the valid source filename is
absent from it. This requires neither a digest collision nor an edit during
verification. An invalid link-target byte can likewise compare equal to a
valid U+FFFD target and leave an accepted dangling link. The comment that a
lossy spelling cannot match a manifest path is false: capture accepts a literal
U+FFFD because it is valid UTF-8 and is not a control character.

### 4. P2 — A source-valid symlink can become dangling on the cache filesystem

Location: `crates/ravel/src/skills/snapshot.rs:530`, with the original link
target retained at line 556 and written-tree comparison at line 198.

On a case-insensitive source filesystem, create `Guide.md` and the relative
link `alias.md -> guide.md`. Put the cache on a case-sensitive filesystem.
Configure roots `["."]` and no mandatory files. This is a quiet source with
one ordinary file and one working internal link; there are no colliding entry
names or reserved entries.

The link walk asks the source filesystem for `guide.md`, which reaches
`Guide.md`, so it admits the link. Capture keeps the actual entry spelling and
the original link bytes. The cache therefore holds `Guide.md` and a link to
the absent `guide.md`. Readback compares the stored link text, without checking
its resolution in the written tree, so every check passes and a dangling link
is published. Neither `check_roots` nor the empty mandatory list sees it.
The contract requires admitted links to work in the retained resources and
dangling links to refuse. The case-sensitive-cache fixture tests distinct
entries, but supplies no link whose target relies on source filesystem aliases.
The later reserved-alias fix is limited to `.git` and `.jj`, so it does not
repair this ordinary-entry case.

### 5. P2 — The concurrent editor fixture assumes atomic file replacement

Location: `crates/ravel/tests/skills_publication.rs:408`, with the successful
capture assertion at line 422.

The editor calls `fs::write`, which truncates the file before writing the new
bytes. A valid schedule pauses that thread after truncation and lets capture
read and verify the empty file before the editor resumes. The entry identity,
size and change times can remain stable throughout all of capture's looks;
the staged manifest and readback then correctly describe an empty file.
This is allowed by the documented non-atomic capture contract.

Nevertheless the fixture permits only the two complete nonempty strings and
fails on this successful capture. Conversely, if the editor is not scheduled
while the six resolutions run, the `refused > 0` assertion at line 445 fails
without a resolver defect. The sleeps do not establish either interleaving.
This can make the full check fail under scheduling changes and asserts a
stronger successful-capture property than the implementation promises. The
internal hook fixtures already supply deterministic change-detection evidence;
the public churn fixture needs an assertion or coordination consistent with
the weaker contract.

### 6. P2 — The killed-publisher fixture can pass without reaching resolution

Location: `crates/ravel/tests/skills_publication.rs:349`, with the sole
interruption coverage assertion at line 361.

The fixture sleeps after spawning, kills the child and counts any
signal-terminated child as an interrupted resolution. A scheduler can keep
every child from reaching capture until after its kill, particularly the
zero-delay child. Then `interrupted > 0` passes, the loops over published
objects inspect nothing, and the final ordinary resolution succeeds. No
killed child needs to create staging, begin publication or reach the
descriptor boundary.

The delays are sampling opportunities, not evidence of which code was
interrupted. The test can therefore pass without exercising the acceptance
case it names, and cannot cover the rename-to-seal window. A measured
execution milestone is needed before crediting a killed run with publication
coverage. This finding concerns test evidence; the no-replace rename itself
still publishes the already materialised directory as a unit.

## Other specific doubts

- **Cache walk, links and `..`:** Under stable Unix ownership and mode bits,
  `passable` rejects other-owned directories and nonsticky directories that
  others can write. `kept` protects the entry through a writable sticky
  parent before following a link or opening a directory. Relative link
  expansion retains its checked parent; absolute expansion starts again at
  `/`; `..` compares the descriptor-reached parent with the reopened spelling
  and checks its mode and owner. The directory open compares identity after
  the first stat and refuses a final symlink. I found no static bypass through
  those segment rules. This is not a claim about hostile root, ACLs or every
  concurrent same-account edit. Findings 1 and 2 concern bindings after that
  walk, which its root-only check does not cover.
- **Change detection:** File identity, type, size, mtime and ctime are checked
  at open and after copying; the final directory pass checks names and entry
  metadata again. A source edit after a subtree's last observation, or a
  same-size rewrite whose timestamps are indistinguishable on the source
  filesystem, can escape detection. Retaining all compared fields also
  defeats this observation method. The copied digest still describes the
  bytes read, not an atomic source state. The module and book explicitly
  acknowledge the limit, and the hook tests describe controlled observations.
  Finding 5 is the public assertion that exceeds that limit.
- **Path collisions and spelling:** With stable staging, exclusive create
  prevents overwriting a name that the cache folds onto an earlier name.
  The recorded destination identity finds the earlier source path for the
  collision message. Whole-tree readback includes directory segments and
  detects a spelling change as `skill_resource_invalid`; unchanged names with
  changed type, bytes or executable bit reach `skill_cache_unavailable`.
  The HFS+ and case fixtures exercise these distinct cases. I found no
  additional quiet-staging diagnostic defect; findings 3 and 4 cover integrity
  or link semantics that comparison of displayed paths and link text misses.
- **Winner checks and sealing:** Existing snapshot objects must have exactly
  `manifest` and `tree`, the expected manifest bytes and a matching recomputed
  tree manifest. Descriptors must have exactly the one expected file and bytes.
  Added, removed or changed ordinary UTF-8 entries therefore refuse without
  repair. Finding 3 is an exact counterexample to the name comparison. Contents
  are sealed before rename, so ordinary publication exposes complete bytes.
  The outer object remains mode `0700` until the post-rename chmod; another
  reader can see it then, and publisher death can leave that mode permanently.
  Reuse does not validate or repair modes. This is a permission-sealing
  limitation, not evidence of partial contents or authentication against the
  same account; no test cited here establishes mode `0555` at first visibility.
- **Cleanup and failure reporting:** With stable entries, each operation stays
  relative to an open directory and a symlink is never traversed by recursive
  removal. Finding 2 is the missing identity precondition for deleting only
  the resolver's own tree. An explicit cleanup failure on the success path is
  `skill_cleanup_failed`, exit 5. On refusal, `after_cleanup` retains the
  original refusal code and appends the cleanup failure. The internal fault
  fixture covers both dispositions. The destructor ignores errors only as an
  unwind fallback; normal paths use explicit cleanup. No new discrepancy was
  found in those error dispositions.
- **Storage remedies:** Destination mkdir, create, write, sync, symlink,
  readback, manifest/descriptor write, seal and rename errors preserve the
  `io::Error` through `unavailable`. Its errno match selects the storage remedy
  for `ENOSPC`/`EDQUOT` and the permissions remedy for
  `EACCES`/`EPERM`/`EROFS`. Source read failures and cleanup failures have
  separate contracts. No additional write path that erases those errno values
  was found in the inspected commit.
- **Barrier and Linux:** Keeping the FIFO writer open until all six children
  finish closes the recorded late-open hang: each shell can consume its line
  even if it reaches the FIFO late. There is no fixture-local deadline on the
  initial FIFO open or child waits, so this is not a general hang guarantee.
  The barrier starts processes together with some probability; it does not
  prove overlap at the publication syscall. Findings 5 and 6 concern the
  remaining timing assumptions. Linux uses `renameat2(RENAME_NOREPLACE)` with
  held parent descriptors and recognises `EEXIST`/`ENOTEMPTY`; the errno reset
  uses the Linux thread-local location. No additional Linux-specific syscall
  defect was found by inspection. Finding 3 has a concrete Linux filename
  input. The Linux path-identity fixture assumes its temporary filesystem
  preserves case and both normalisation forms, and supplies no contrary
  filesystem fixture.

## Verification limits

The producer records macOS checks and mutation controls at the reviewed
commit. Those are historical evidence, not new checks run here. Linux received
cross-target clippy only; this review supplies no Linux execution, ACL
qualification or new confinement measurement. Every counterexample above
requires integration triage and an appropriate regression case at the current
feature tip. The review changes only Grove review and task-tree artifacts.
