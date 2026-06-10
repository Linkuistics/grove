# 020-recency-field

**Kind:** work

## Goal

Add a per-grove **recency** field to the core data layer: the last-commit
timestamp on the grove's branch, computed during the (already concurrent)
`RepoView` scan. This is the new datum the filter mode's recency sort order
(010-plan Q4) needs; everything else the mode ranks on (name, lifecycle,
inbox-pending count) already exists.

## Context

- `src/repo_view.rs` — `GroveDetail` carries no timestamp today; the scan
  walks `.grove-worktrees/`. Seeds have no branch, so recency is `Option`.
- Below the [[presentation boundary]]: no `ratatui`, headless tests only.
- mtime-based recency was explicitly rejected (Q4): background file touches
  must not reorder the list.

## Done when

- `GroveDetail` (or the per-grove struct the scan fills) exposes an optional
  last-commit timestamp for live groves; `None` for seeds and for groves
  whose branch lookup fails (a failure never blocks the scan — same
  silently-skip posture as repo scan failures).
- Headless tests cover: a grove with commits, a seed, a lookup failure.
- Scan stays concurrent; no measurable serial git bottleneck at fleet scale.

## Notes

Implementation choice (git2 vs shelling to `git log -1 --format=%ct`) is the
session's call; bias toward whatever the crate already depends on.
