# 010-data-layer

**Kind:** work

## Goal

Build the read-only data layer the TUI consumes. Scan a repo for its
groves, count what each grove contains, list inbox observations, and
expose it all through a single small API surface (`RepoView`-style
abstraction) so the future `MultiRepoView` (per the
`tui-multi-repo-and-multiplexer` seed) is additive rather than a
refactor.

No UI work in this leaf — no Ratatui, no crossterm. The data layer must
be exercisable from a plain test binary or unit tests.

## Context

The TUI's situational-awareness story (parent BRIEF) needs four facts
per grove:

- Name (the worktree's branch, equivalently `.grove-worktrees/<name>/`).
- Live task tree shape — leaves and nodes under `.grove/`, with
  `done/` segregated so retired material is visible but distinguishable.
- Inbox pending count — number of observation files in
  `<repo>/.grove-meta/inboxes/<name>/` (excluding `.gitkeep`).
- Lifecycle state — live (worktree exists), seed (inbox exists, no
  worktree), finished (no inbox, no worktree — appears only in git
  history, may be out of scope for v1).

For a *single* selected grove, the detail screen also needs:

- Per-leaf content (lazy read; just the absolute path is enough at
  scan time).
- Per-inbox-observation content (same — path now, body on demand).
- The `BRIEF.md` for each node on the path to a selected leaf.

The existing `grove status` and `grove list` subcommands already cover
some of this; check whether their internals are shaped for reuse before
writing fresh scanning code. If `repo.rs` / `status.rs` / `list.rs`
expose enough, the data layer can be a thin orchestrator.

## Done when

- A `RepoView` struct (name is the implementor's call — `RepoView`,
  `RepoSnapshot`, `GroveIndex`, whatever fits) is the single entry
  point: `RepoView::scan(&repo_root)` returns a populated view.
- The view exposes:
  - `groves(&self) -> &[GroveSummary]` with name, lifecycle state,
    live/retired leaf counts, inbox pending count.
  - `grove(&self, name: &str) -> Option<&GroveDetail>` returning the
    task-tree shape (a tree of nodes/leaves with absolute paths,
    `done/` flagged) and inbox observation paths.
  - Methods or free functions to read a leaf's body and an inbox
    observation's body on demand (returning `Result<String>`).
- The scan is cheap enough to be re-run on filesystem-watch events
  (see leaf 030) — no expensive parsing of every leaf body at scan
  time.
- Unit tests against a `tempfile`-built fixture repo cover: zero
  groves, one live grove with mixed live/retired leaves, one seed (no
  worktree), one grove with pending inbox observations.
- Nothing in this leaf references Ratatui or crossterm. The TUI binary
  itself doesn't exist yet.

## Notes

- **Lifecycle state for finished groves is out of v1 unless cheap.** A
  grove is "finished" only by examining git history of the
  `grove-meta` branch for a removed inbox. If that's cheap (one
  `git log --diff-filter=D` invocation), include it; otherwise stop at
  live + seed and leave finished for the future grove.
- **The `RepoView` name is judgement.** The parent BRIEF used it as a
  proxy for "the data abstraction". Pick whatever reads cleanest with
  the actual fields — the contract that matters is "this layer is
  single-repo today, an additive `MultiRepoView` wrapper tomorrow",
  not the literal name.
- **No memoisation in v1.** Re-scan on every refresh; the trees are
  small. If profiling later shows it matters, the future grove adds
  caching.
