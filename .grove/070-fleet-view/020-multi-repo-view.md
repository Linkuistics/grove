# 020-multi-repo-view

**Kind:** work

## Goal

Build `MultiRepoView` — the fleet data layer that wraps **N** `RepoView`s, scanned
concurrently, exposing the **grouped** (repo → groves) structure the nav renders.
Single-repo is the N=1 case (070 Q4 "subsume"). Below the presentation boundary
(ADR-0013) — no `ratatui`.

## Context

- `src/repo_view.rs:26` — `RepoView::scan(repo_root) -> Result<Self>` already exists
  and is unchanged here; the comment at `:9-12` anticipates this exact wrap
  ("the deferred multi-repo evolution wraps this type rather than rewriting it").
- Input: the `Vec<repo root>` from leaf `010`.
- Shape: `MultiRepoView { repos: Vec<RepoView> }` (a *collection*, mirroring the
  grouped UI — 070 Q2). Accessors return per-repo groups, not a flattened list.
  Each `RepoView` already carries `repo_root`, so repo attribution is structural.
- **Concurrent scan** — one `scan()` per repo on its own thread (scans are
  independent I/O; fleet may be a dozen+ repos). A failed scan drops that repo
  (070 Q3 silent-skip); collect successes, never fail the whole fleet.
- **Repo ordering** (070 Q5 sort default): current-repo first, then explicit `repos`
  in manifest order, then scanned repos alphabetically. Groves *within* a repo keep
  the existing `RepoView` order (lifecycle then numeric prefix).
- Construct the single-repo case as `MultiRepoView` over a one-element list so callers
  collapse to one code path.

## Done when

- `MultiRepoView::scan(repo_roots) -> Self` scans each repo concurrently, skipping
  failures, in the defined repo order.
- Grouped accessors expose `(repo, &[GroveSummary])` per repo and reach a specific
  `GroveDetail` by `(repo, grove name)`.
- A one-element `MultiRepoView` reproduces today's single-repo data exactly.
- Targeted re-scan hook exists: re-scan *one* repo's `RepoView` in place by repo
  root (consumed by `030`).
- Tests: multi-repo grouping/order, one-repo equivalence, a failing repo is skipped.
- No `ratatui` import.

## Notes

Depends on `010`. Feeds `030` (watch→targeted re-scan), `040` (nav rendering), and
the cross-repo `GroveDetail` lookup `050` uses. Keep the public surface presentation-
agnostic so a future web front-end reuses it.
