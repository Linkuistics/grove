# grove.merge-grove-with-skills — brief

## Goal

Merge `Linkuistics/skills` into this repo so grove (CLI + methodology) and the
`linkuistics` / `testanyware` skill plugins live in one tree with one history.
The driver is **coupled change cadence**: most grove changes require a matching
skill change, and two repos made every such change a cross-repo lockstep no
single commit could carry.

## Done when

- `plugins/`, `.claude-plugin/marketplace.json` and `install.sh` live here, with
  the skills repo's 68 commits grafted in — `git blame` on
  `plugins/linkuistics/skills/*` traces past the merge.
- `cargo test` is green and the binary still provisions `content/` unchanged.
- The repo reads as one repo: a root `CONTEXT-MAP.md` over two context
  glossaries, one README, one CHANGELOG, one coherent ADR set.
- Both plugins install and resolve from this repo under their existing names
  (`linkuistics@linkuistics`, `testanyware@linkuistics`), so every
  `linkuistics:<skill>` reference in every project keeps working.
- `Linkuistics/skills` is archived with a pointer, and the move is announced.

## Decomposition

Children are ordered by dependency, not by size — `02` is the only hard
prerequisite; `03`–`05` are mutually independent behind it.

- `02-graft-history-k2` — the merge lands green. Everything else needs the files
  to be here first.
- `03-glossary-partition-k3` — `CONTEXT-MAP.md` plus a rewritten
  `plugins/CONTEXT.md`. Changes what every future session reads at bootstrap, so
  it comes early.
- `04-docs-reconciliation-k4` — README, CHANGELOG and the ADR set read as one
  repo's rather than two.
- `05-plugin-versioning-k5` — stop plugin versions churning on unrelated grove
  commits.
- `06-cutover-k6` — archive, announce, re-point, verify. Last because it depends
  on the merge being pushed.

## Pointers

- ADRs a session here must read: `docs/adr/skills-monorepo.md` (the decision and
  its rejected alternatives), `docs/adr/self-extension-core-and-methodology.md`
  (the distribution model this merge deliberately leaves alone).
- Glossary terms in play: *Global skill provisioning* / *skill precedence*
  (`CONTEXT.md`) — the mechanism that stays unchanged here.
- Reference for the graft mechanics: `linkuistics:using-jujutsu`. This repo is
  jj-native (`.jj/`, no colocated `.git/`); the skills remote is plain git.

## On the horizon

- **Unify provisioning.** grove's `provision.rs` sweeps are stamped, idempotent,
  refuse to clobber a foreign directory, and replace a stale symlink *as a link*;
  `install.sh` plainly symlinks and needs re-running when skills are added or
  removed. Once both trees are here, grove's binary could provision the skills to
  codex and pi directly and `install.sh` could go. Not leaf-shaped yet: the open
  part is which harnesses the sweep covers and how to avoid double-provisioning a
  skill Claude already gets via the plugin (personal beats plugin in precedence,
  and `CONTEXT.md` already treats a shadowed duplicate as dead code).

## Notes

Two findings from the planning grilling that a session here may trip over:

- **Marketplace identity is not the repo URL.** It is the `name` field in
  `marketplace.json` — already `linkuistics` while the repo was `skills`. Skill
  namespaces are therefore repo-independent; only `source.repo` moves.
- **Archiving fails silently.** `autoUpdate: true` keeps *succeeding* against an
  archived repo; the content just freezes with no error. Announcement is the
  mitigation, not a nicety.
