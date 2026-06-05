# 030-fleet-fs-watch

**Kind:** work

## Goal

Extend the fs-watch to fleet scale (070 Q6): one `notify` watcher over **all** repos'
roots, a **`.git/` path filter** (the OBS-2 cheap win), and **targeted per-repo
re-scan** — an event re-scans only the owning repo's `RepoView`, not the whole fleet.

## Context

- `src/tui.rs:512` — `WatchSet` today holds one `RecommendedWatcher` watching
  `<repo>/.grove-worktrees` + `<repo>/.grove-meta/inboxes` recursively (`:535-536`),
  with a 200ms debounce (`dirty_since`). No `.git/` filter exists.
- **`.git/` filter** (lands regardless, both briefs' OBS-2): drop any event whose
  path contains a `/.git/` component before it can set `dirty_since`. Removes pack/
  ref/index churn the debounce currently only masks — amplified N-fold at fleet scale.
- **Multi-root, one watcher**: call `.watch()` for each repo's two roots on a single
  `RecommendedWatcher` (070 Q6 — rejected per-repo watchers' N threads).
- **Targeted re-scan**: on a (filtered, debounced) event, **prefix-match the event
  path against the known repo roots** to find the owning repo, then call `020`'s
  per-repo re-scan hook for just that `RepoView`. The event path is under exactly one
  root.
- The per-grove watch refinement (watch `…/<name>/.grove/` instead of the worktrees
  root) stays **optional** — the `.git/` filter already removes its target noise;
  only revisit if event volume demands it.

## Done when

- One watcher watches every fleet repo's two roots; adding/removing a repo updates
  the watch set.
- Events whose path contains `/.git/` never mark dirty (covered by a test feeding a
  synthetic `.git/`-internal path).
- A dirty event re-scans only the owning repo's `RepoView` (verified: an event under
  repo A leaves repo B's view object untouched).
- Single-repo behaviour unchanged (N=1).

## Notes

Depends on `010` (repo roots) and `020` (per-repo re-scan hook). The `.git/` filter
satisfies the root brief's "`notify` ignores `.git/` churn" done-criterion.
