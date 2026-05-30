# 070-fleet-view

**Kind:** planning (decompose into impl leaves)

## Goal

Extend the single-repo reader into a multi-repo **fleet view** (concern 1): one
process surfacing groves across many repos, filterable by repo / workstream /
inbox-pending count. Architecturally additive — `RepoView` → `MultiRepoView` —
per the v1 brief's deliberate factoring.

## Context

- v1 data layer: `src/repo_view.rs` — `RepoView::scan(repo_root)`,
  `GroveSummary`, `GroveDetail`. The seam to generalise is the single
  `repo_root`. Aim for `MultiRepoView` as a collection of `RepoView`s rather
  than a rewrite.
- **Repo discovery** is an open design question for this leaf's grilling: how
  does grove learn which repos to span (config file? a registry on the
  `grove-meta` branch? a scan root? explicit `grove tui --repo` flags)?
- **fs-watch `.git/` noise (OBS 2) folds in here.** Watching N repos amplifies
  the `.git/` event noise N-fold. Land the two cheap wins as part of this leaf:
  (a) path-filter `notify` events whose path contains `/.git/`; (b) optionally
  watch `.grove-worktrees/<name>/.grove/` per-grove instead of the worktrees
  root recursively. See root BRIEF "fs-watch .git/ noise" note for detail.

## Done when

- The TUI lists groves spanning multiple repos with repo attribution.
- Filtering by repo / workstream / inbox-pending works.
- `notify` no longer marks dirty on `.git/`-internal churn (path filter landed).

## Notes

Sequenced after the harness pane (060). The harness model 060 settles (per the
050 backend spike) must compose with the fleet list — selecting any grove in any
repo opens its harness beside the dashboard. Likely needs decomposition.
