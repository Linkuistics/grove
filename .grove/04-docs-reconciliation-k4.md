# docs-reconciliation-k4

**Kind:** work

## Goal

Make the README, the CHANGELOG and the ADR set read as one repo's documentation
rather than two repos' stapled together.

## Context

`graft-history-k2` deliberately resolved these three minimally — this leaf is
where the real reconciliation happens.

**ADR set.** The merge brings `docs/adr/symmetric-vcs-rule.md` alongside this
repo's existing slugs (`cli-binary-split`, `codex-gitdir-grant`,
`do-is-sole-lifecycle-verb`, `fresh-grove-start-contract`,
`in-session-finish-cycle`, `model-per-task-kind`, `pruning`, `self-driving-loop`,
`self-extension-core-and-methodology`, `task-kind-taxonomy`, `task-tree-scheme`,
`user-owned-worktrees`) plus `skills-monorepo` from planning. Use
`linkuistics:decision-records`: the set must be the **minimum coherent set
describing the current design**, reworked in place — merge, split or delete,
never append a superseding record.

`self-extension-core-and-methodology` is the one most likely to need editing: it
describes the distribution model, and while this merge does not *change* that
model, it changes what else lives in the repo alongside it.

**Research docs.** Ten now sit in `docs/research/` from two lineages. They are
historical surveys, not current-state documents — leave them be unless one makes
a claim the merge has falsified.

## Done when

- `README.md` describes a repo with two components: the grove CLI + methodology
  (brew, binary-provisioned to three harnesses) and the skill plugins (installed
  via the marketplace, or `install.sh` for codex/pi). A reader arriving cold can
  tell which install path they want.
- The skills README's skill table survives somewhere sensible — its per-skill
  "loads when" column is genuinely useful and has no equivalent here.
- The stale line in the skills README — *"Looking for grove? It moved to its own
  repo"* — is gone. It is now actively false.
- `CHANGELOG.md` is one file with one entry style, and a stated policy for how a
  skills-only change is logged alongside a `v<N>.0.0` grove release.
- The ADR set passes the minimum-coherent-set test: no two ADRs describing the
  same decision, no dangling citation. Grep every ADR slug across `docs/`,
  `content/`, `README.md`, `plugins/` and `.grove/` and fix what the rework
  breaks.
- `docs/workflows/` and `docs/superpowers/` are checked for statements the merge
  invalidated.

## Notes

- **ADR *placement* is now an open question, raised by `glossary-partition-k3`.**
  `content/ADR-FORMAT.md` says that in a multi-context repo — one carrying a root
  `CONTEXT-MAP.md`, which this repo now does — system-wide decisions stay in the
  root `docs/adr/` while **context-specific** ones go under that context's own
  `docs/adr/`. By that rule `symmetric-vcs-rule` belongs at
  `plugins/docs/adr/symmetric-vcs-rule.md`, and `skills-monorepo` is arguably
  system-wide. Decide it explicitly — split per the rule, or keep one flat set and
  amend `content/ADR-FORMAT.md` to say when a flat set is right. Either way
  `CONTEXT-MAP.md`'s third relationship ("both sit in the root `docs/adr/` today")
  and every citation of a moved slug need updating. This is the single biggest
  argument for `leaf-decompose`-ing this leaf.
- `content/SKILL.md` and `README.md` both name the `linkuistics` plugin as an
  external **prerequisite**. That framing is now half-true — the skills are
  in-repo, but a *user* still installs them separately via the marketplace since
  grove's binary does not provision them. Say exactly that; do not simply delete
  the prerequisite note.
- This leaf may prove bigger than one session — the ADR-set pass alone can. If
  so, `leaf-decompose` it rather than running long; the natural seam is
  README/CHANGELOG in one child and the ADR set in another.
