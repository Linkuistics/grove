# docs-reconciliation-k4 — brief

## Goal

Make the README, the CHANGELOG and the ADR set read as one repo's documentation
rather than two repos' stapled together.

## Context

`graft-history-k2` deliberately resolved these three minimally — this node is
where the real reconciliation happens.

**Research docs.** Ten now sit in `docs/research/` from two lineages. They are
historical surveys, not current-state documents — leave them be unless one makes
a claim the merge has falsified. The same applies to `docs/superpowers/` (a dated
plan + spec from a past workstream) and to the carried-in section at the foot of
`CHANGELOG.md`: closed records, not current-state documents.

## Decomposition

Split at the seam the original leaf named. The two children touch disjoint files
and neither depends on the other; `01` runs first because a cold reader's entry
point matters more than record hygiene, and because `02` carries an open decision
that `01` does not need resolved.

- `01-prose-reconciliation-k7` — the human-facing prose: `README.md`,
  `CHANGELOG.md`, and a sweep of `docs/` for statements the merge invalidated.
  Editorial throughout; no open decisions.
- `02-adr-set-k8` — the ADR set as a **minimum coherent set**, plus the open
  **placement** question `glossary-partition-k3` raised (one flat root set vs a
  per-context split). Carries a decision, so it is not editorial.

## Pointers

- ADRs a session here must read: `docs/adr/skills-monorepo.md` (why the two trees
  are one repo), `docs/adr/self-extension-core-and-methodology.md` (the
  distribution model the merge leaves alone).
- `content/ADR-FORMAT.md` — grove's ADR *placement* rule, and the subject of
  `02`'s open question.
- `linkuistics:decision-records` — the minimum-coherent-set discipline `02`
  applies: rework in place (merge / split / delete), never append a superseding
  record.

## Notes

- `content/SKILL.md` and `README.md` both name the `linkuistics` plugin as an
  external **prerequisite**. That framing is now half-true — the skills are
  in-repo, but a *user* still installs them separately via the marketplace since
  grove's binary does not provision them. Say exactly that; do not simply delete
  the prerequisite note. (Owned by `01`.)
