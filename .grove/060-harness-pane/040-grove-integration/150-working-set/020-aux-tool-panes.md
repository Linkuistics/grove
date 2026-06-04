# 020-aux-tool-panes

**Kind:** work (grove side — composes the real working set)

## Goal

Spawn the aux tools — a **plain terminal**, **yazi** (files), and a **vcs** TUI
(lazygit now, lazyjj later) — as embedded command panes in the **grove's worktree
cwd**, and compose them into the working set beside the harness + detail (using the
010 mechanism). This is where ADR-0020's headline "embed *other* TUI apps" gets
exercised for real.

## Context

- 010 gives the variable-membership working set + per-member show/hide. This leaf
  fills the set with real `Run::Command` panes — exactly the harness-spawn path
  (`swap_content`'s primary is already a `grove do <name>` command pane), so the
  aux tools reuse that spawn mechanism, just more of them.
- Each aux pane's cwd is the grove's worktree (`.grove-worktrees/<name>/`), the
  same cwd the harness runs in — so yazi/lazygit operate on the grove's tree.
- **vcs is not hard-wired to git** (brief decision): route the vcs command through
  a single `vcs_tool(worktree)` helper that probes for a jj repo (`.jj/` present,
  or `jj root` succeeds) → `lazyjj`, else `lazygit`. **Default lazygit now**; the
  probe is the seam so lazyjj lands later without re-touching the spawn path.

## Done when

- Selecting a grove mounts a working set that includes harness + detail +
  terminal + yazi + vcs (all members present in the set; visibility per 040's
  responsive default — for this leaf, having them mountable + correct is enough,
  e.g. all visible on a wide test terminal).
- Each aux tool runs in the grove's worktree cwd and behaves exactly as it does
  bare — focus, input, copy, resize, cursor (the trellis embedding promise);
  lazygit drives the worktree's git.
- `vcs_tool()` returns lazygit for a git worktree and is structured so a jj
  worktree would route to lazyjj (lazyjj not required to be installed/working yet).
- Switching groves parks the whole set (incl. aux panes) alive and restores it.
- `cargo build` / `cargo test` green (grove + trellis suites).

## Notes

- Resolve tool binaries on `PATH` (`terminal` = `$SHELL` or the user's shell);
  surface a graceful in-pane message if a tool isn't installed rather than failing
  the whole working-set mount (yazi/lazygit are not guaranteed present).
- Toggle *keys* and responsive *defaults* are 030/040 — this leaf only needs the
  panes spawnable and composed into the set.
- Keep grove core (`RepoView`/writes) `ratatui`-free below the ADR-0013 seam; the
  spawn composition lives in the `trellis-seam`-gated `mod native` like the rest.
