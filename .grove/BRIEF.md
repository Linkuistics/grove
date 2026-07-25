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

Children are ordered by dependency, not by size — `graft-history-k2` is the only
hard prerequisite; the three behind it are mutually independent.

- `graft-history-k2` — the merge lands green. Everything else needs the files to
  be here first.
- `glossary-partition-k3` — `CONTEXT-MAP.md` plus a rewritten
  `plugins/CONTEXT.md`. Changes what every future session reads at bootstrap, so
  it comes early.
- `docs-reconciliation-k4` — README, CHANGELOG and the ADR set read as one
  repo's rather than two.
- `plugin-versioning-k5` — stop plugin versions churning on unrelated grove
  commits. Pinned an explicit semver in both manifests; **reversed by
  `version-bump-guard-k9`**, so the churn is back by choice and the pin is gone.
- `version-bump-guard-k9` — decide whether the bump discipline
  `plugin-versioning-k5` introduced gets a mechanical guard. Grown from that
  leaf, not planned: pinning a version replaced noisy churn with a *silent*
  staleness, and grove's own hazard rule says a silently-failing discipline needs
  either a mitigation or an explicit decision not to build one. Settled by
  removing the discipline rather than guarding it: both plugins ship unversioned,
  so every push delivers and there is nothing to forget
  (`docs/adr/skills-monorepo.md`). A session here should expect
  `claude plugin validate --strict` to fail on both manifests — that is by design.
- `cutover-k6` — archive, announce, re-point, verify. Ran last because it depends
  on the merge being pushed, and it carried the deferred half of
  `plugin-versioning-k5`'s verification. Done: `main` is pushed, the marketplace
  re-points at `Linkuistics/grove` and resolves end to end at SHA `bd9df106d34b`,
  and `Linkuistics/skills` is archived behind a pointer README.
- `sparse-marketplace-checkout-k10` — decide whether the marketplace clone should
  be sparse (`.claude-plugin` + `plugins/` only) rather than dragging grove's whole
  tree. Grown from `cutover-k6`, not planned: re-pointing made the monorepo's first
  *consumer-visible* cost measurable — 2.7M cloned for 172K of plugins. Small
  stakes; an explicit decline is a fine outcome. **Abandoned** — sparse checkout was
  applied to the live clone and measured, then reverted: it saves 1.6M (2.7M → 1.1M,
  a 2.5× cut, not the 16× the leaf assumed) and **zero bandwidth**, because cone mode
  keeps every root-level file and `.git/objects` is untouched by a working-tree
  filter. The recurring fetch `docs/adr/skills-monorepo.md` accepts is therefore
  unchanged, which was the leaf's whole case. No ADR: the choice fails the
  hard-to-reverse test.

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

**For the finish cycle's promote step** (from `docs-reconciliation-k4`, now done):
three parts of `docs/` are **closed historical records, not current-state
documents** — `docs/research/` (ten surveys from two lineages), `docs/superpowers/`
(a dated plan + spec from a past workstream), and the carried-in section at the foot
of `CHANGELOG.md`. They are read for provenance and left alone; only a claim the
merge actually falsified is worth touching. Nothing outside `.grove/` says this
today, so it needs a durable home — a short note in `docs/` is the obvious one — or
it dies with the tree.
