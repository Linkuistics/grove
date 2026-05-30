# tui-multi-repo-and-multiplexer — brief

## Goal

Extend the v1 `grove tui` (per-repo reader + capture) into a multi-repo,
multiplexer-aware TUI. The headline capabilities, all deferred from v1:

1. **Multi-repo "fleet" view** — one process surfacing groves across many
   repos, filterable by repo / workstream / inbox-pending count.
2. **Multiplexer choice** — pick tmux vs zellij vs in-process pty
   (`tui-term` + `portable-pty`). Non-trivial; gets its own grilling thread.
3. **Embedded harness pane** (`d` / launch `grove continue <name>`) — the
   headline feature, unblocked once (2) is decided.
4. **Sync→async refactor** of the event loop (multiplexer control socket,
   N-repo `notify` streams, subprocess output juggling).

## Done when

- The harness pane works: from the dashboard, open a grove's harness session as
  a window in grove's owned tmux session; it persists when the dashboard closes.
- The fleet view spans multiple repos with filtering; `notify` ignores `.git/`
  churn.
- Concern 4 (async) is formally resolved — refactored or recorded as not-needed.

## Decomposition

Settled in 010-plan (D1–D3), then amended: a web-front-end alternative to the
TUI presentation surfaced and now gates the integration decision. Spine:
research the tmux-ownership prior art → compare a web front-end vs the TUI
(presentation-layer fork) → decide the architecture + integration mechanics →
build the harness pane (validates it) → build the fleet view → revisit async.

```
020-research-tmux-ownership   research the owner pattern (claude-squad, iTerm2 -CC, …)  [done]
030-web-frontend-comparison   hybrid web UI vs Ratatui TUI over the same tmux backend → recommendation
040-decide-tmux-integration   architecture (TUI vs web) + socket / control-mode / launch-attach / config → ADR(s)
050-harness-pane              'd' opens a grove-do window (concern 3) — first validator
060-fleet-view                MultiRepoView (concern 1); fs-watch .git filter folds in
070-async-revisit             confirm sync suffices / minimal async (concern 4)
```

Core architecture (010-plan D2/D3): grove **owns** a dedicated tmux session;
each harness is a window grove creates. Chosen over in-process pty for session
persistence + crash isolation — D2 is settled and 020 confirmed it; the web
alternative keeps this tmux backend (hybrid scope). **What 030 re-opens is D3**
(the *presentation* layer — TUI window 0 vs a browser front-end), not D2. The
binding ADR is raised in 040 once 030's comparison lands; 030 may reframe 040
and reshape 050/060 if the recommendation is "go web".

## Pointers

- Seeded from grove `capture-issues-for-later-groves`, leaf
  `020-design-seed-convention/090-tui-server.md`.
- v1 architecture is the inherited starting point — this grove *extends*,
  it does not rewrite.

## Notes

### Inherited v1 architecture (the starting primitives)

- Ships as `grove tui` subcommand (single binary).
- Sync event loop (Ratatui + crossterm); `notify` for filesystem-watch with
  200ms debounce.
- Two screens, master/detail: grove list → grove detail (left: task tree;
  right pane cycles leaf content / inbox / BRIEF).
- Writes via shell-out to existing `grove` verbs only — no direct file edits.
  Walk-away-ability preserved.
- v1's data layer is factored behind a `RepoView`-style abstraction so a
  future `MultiRepoView` is additive, not a rewrite.

### fs-watch `.git/` noise (concrete concern from v1 leaf 030)

`notify::RecommendedWatcher` on `.grove-worktrees/` recursive picks up every
event inside each worktree's `.git/` (pack writes, ref updates, index churn).
v1 swallows it under the 200ms debounce, but a multi-repo view watching N
repos amplifies this N-fold, and fleet-scale worktree trees amplify further.
Two cheap wins:

- **Path-filter `notify` events** — ignore any path component containing
  `/.git/` (or matching `.git$`). Probably enough on its own; can land first.
- **Watch `.grove-worktrees/<name>/.grove/` per-grove** instead of
  `.grove-worktrees/` recursively. More handles, no `.git/` pollution.
  Aligns with the per-repo helper factoring v1 already asked for.

### Why a separate grove, not a v1 stretch goal

The multiplexer decision interlocks with the fleet view and the harness pane:
you can't present multiple repos until you know whether each is a pane in your
own multiplexer-driver or a separate process. Bundling into v1 would have
turned "small UX layer" into "layout manager over a multiplexer."
