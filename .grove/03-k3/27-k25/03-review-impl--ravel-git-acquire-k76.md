# ravel-git-acquire-k76

**Reviews:** ravel-git-acquire-k70

## Goal

Read the online Git acquisition of Ravel commit `1898dc8603a0` adversarially.
Read it against `docs/specs/skill-discovery.md` at `822c0ed2`, "Exact Git
acquisition and offline reuse" without the cache hit, "Directory descriptor
and loading contract" and "Failures". Produce findings, not fixes. The parent
`1d50dae5d29e` is the base of the diff.

## Context

- The work is in the jj workspace `Ravel.skill-directory-delivery`, on the
  unpushed bookmark `skill-directory-delivery`. The new code is
  `crates/ravel/src/skills/git.rs`. The resolver changes are in
  `skills/mod.rs` and `skills/snapshot.rs`.
- The public fixtures are `crates/ravel/tests/skills_git.rs`. The running log
  of `ravel-git-acquire-k70` records the observed Git behaviour, the nine
  mutation controls and the checks on macOS and Linux arm64.
- Offline reuse is `ravel-git-offline-k71`. Deadlines, cancellation, child
  reaping and bounded redacted diagnostics are `ravel-git-bounds-k72`. Report a
  defect in those areas only when this commit makes a later leaf harder or
  unsafe.

## Specific doubts

- The system and global Git configuration stay in effect for the owner's
  credentials. Can a key there still run a command or a prompt? Examples are
  `url.<base>.insteadOf`, `include.path`, `includeIf`, `core.sshCommand`,
  `uploadpack.packObjectsHook` for a local repository and `fetch.bundleURI`.
  Does `protocol.allow=never` still hold after an `insteadOf` rewrite?
- `transfer.fsckObjects` is off. A crafted tree can hold a duplicate name,
  `.`, `..`, `.git`, a name with `/` or an odd mode. `Acquisition::name`,
  the reserved-name skip and the collision check must refuse or skip each
  one. Does a case exist that writes outside the extraction directory?
- `fetch` classifies a format mismatch by the text `mismatched algorithms`
  under `LC_ALL=C`. Other failures are transport failures. Can a missing
  commit, a refused want or a server error receive the wrong remedy?
- The direct fetch uses `--depth=1`. Its failure is followed by a full fetch
  of `+refs/*:refs/fetched/*` into the same repository. Can a shallow file
  from a partial first fetch hide an object from `cat-file`?
- `Git::run` lists the inherited descriptors before the spawn and marks each
  one above 2 close-on-exec in `pre_exec`. A thread of Ravel could open a
  descriptor between the list and the fork. Is that reachable, and does it
  matter here?
- Refusals carry up to 1000 bytes of Git's standard error, unredacted, until
  `ravel-git-bounds-k72`. Settings refuse a URL with credentials. Can the
  excerpt still show a secret, such as a header or a token in a redirect?
- The protected-overlap check of a local repository percent-decodes a
  `file://` path and canonicalises it. Git can resolve the same locator to a
  different path. Find a locator that reaches a protected path and passes.
- No control proves the new session without a controlling terminal, because
  the test process has no terminal. Judge whether `setsid` alone is
  sufficient for SSH and for Git's credential prompt.

## Done when

- Each doubt above and each other defect found has a finding with evidence,
  or a recorded reason why it does not hold.
- The findings are committed in this leaf. Cut an integration only if a
  finding is worth acting on.

## Notes

Insert any integration before `ravel-git-offline-k71`, which edits
`git.rs` next.
