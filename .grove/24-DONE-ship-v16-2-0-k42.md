# ship-v16-2-0-k42

**Kind:** impl

## Goal

Cut **v16.2.0**, install it locally, and prove the *installed* binary carries
the work — closing the in-the-tree-not-in-the-binary gap that five of this
grove's leaves each ended on, and that this grove has been working around all
along by running `./target/debug/grove-llm`.

## Context

The `### Added` / `### Changed` / `### Fixed` entries above the first `## v`
heading are written and **not cut**. Everything in them is a normal feature or
fix — nothing breaking, `work` → `impl` already landed in v16.0.0 — so this is a
**minor**, not a major.

Five leaves ended explicitly "in the tree, not in the installed binary", and
each one names a real behaviour real groves do not have yet:

- **compose-task-chains-k29** — the cutting-time guidance in the three surfaces
  a session actually reads while cutting leaves.
- **agent-hint-k33 / agent-hint-observe-k34** — `HERDR_AGENT`, which also
  repairs `fallback_state` on grove panes.
- **codex-grant-refused-k35** — the codex sandbox pre-flight. Until it ships, a
  codex `grove do` in an untrusted tree still dies at startup on a mute exit.
  This is the one with a **live victim**: the `UIAnyware` grove.
- **chain-construction-k38 → k40** — `leaf-add-chain` and `leaf-add-pair`.
- **routed-leaf-diagnostic-k41** — the launch line naming the routed leaf.

`main` is pushed to origin at `retire: routed-leaf-diagnostic-k41` (2026-07-29),
so the tree to release is already published.

## Done when

- The v16.2.0 section is cut over the unreleased entries; `Cargo.toml` is off
  16.1.0; tagged; tarballs for all three targets; GitHub Release; tap formula
  updated; `brew upgrade` clean.
- The **installed** binary is proved to carry the work, not just assumed —
  cheapest honest checks: `grove-llm leaf-add-chain --help` succeeds (k38–k40),
  `strings` on the resolved binary shows `HERDR_AGENT` (k33), and a scratch
  grove's launch line shows the `— <slug>-k<key> (<kind>)` tail (k41).
- Work in this repo stops needing `./target/debug/grove-llm`.
- The tags on `origin` are reconciled — see Notes; decide whether that is this
  leaf's job or a follow-up, but do not leave it undecided.

## Notes

**This working tree cannot cut the release.** It is a jj-native *secondary*
workspace with no `.git`, and the release machinery is git-shaped (`cargo
release` commits and tags through git; `release-build.sh` demands `git describe
--tags --exact-match HEAD`). Cut from the **colocated default workspace**,
`jj workspace root --name default` → `~/Development/grove`. This is
`ship-release-k25`'s finding, unchanged; *symmetric-vcs-rule* still binds — jj
performs every mutation, git is read-only — with tag creation the standing
exception, since no jj verb makes one.

**`main@git` is 32 commits behind `main`.** The colocated `.git`'s
`refs/heads/main` still reads v16.1.0's ship commit, and `cargo release` drives
*that* view. Reconcile before cutting, or the release commit lands on the wrong
base. Afterwards the path is unremarkable: git makes the detached release commit
and tag, jj imports it, and `jj bookmark set main -r <release-change>` puts the
bookmark on it (root brief, Notes).

**`jj git push` refuses to push tags** — observed 2026-07-29: `Refusing to
create new remote tag v16.1.0@origin`. So **v16.0.0 and v16.1.0 are tagged
locally and absent from `origin`**, and v16.2.0 will be too unless pushed
through git from the colocated workspace. This is not cosmetic: a GitHub Release
is cut against a tag, so the release step already depends on it.

**The two remembered workarounds are gone** (`release-doctor-toolchain-gap-k27`,
shipped in the tree): plain `cargo release <level> --execute` and
`scripts/release-build.sh` now work as written — `release.toml` carries
`allow-branch = ["*", "HEAD"]` and the build pins its own toolchain. If either
still needs a flag, that is a **regression in k27**, not a workaround to
re-learn.

**Installing mid-loop stops the loop, by design.** The driver's per-session
version-skew guard compares its compiled-in version against the `grove-llm` the
agent would invoke, so a `brew upgrade` under a live `grove do` parks the pane
`blocked` and stops before the next session (observed,
`observe-mid-turn-live-k31`). Expect this session to end that way rather than by
signalling; re-running `grove do` continues on the new binary
(restart ≡ continuation). Do the install **last**, after everything else is
committed.

**Nothing follows this leaf.** After it retires, `pick` is empty and the next
session proposes the complete finish cycle. That cycle's promotion step has
exactly one identified homeless finding — macOS discloses no environment for
SIP-protected platform binaries (`kern_procargs2` is what herdr reads; a copied
signed binary also fails codesign on Apple Silicon, so compile a throwaway) —
whose natural home is `docs/specs/herdr-fork-maintenance.md` § *Verifying a
rebase*. Everything else in the root brief's Notes already has an ADR, spec or
glossary home (checked 2026-07-29).
