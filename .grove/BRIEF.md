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

- The harness pane works: from the dashboard, open a grove's **live harness
  session beside the dashboard** (claude code / codex), interact with it, and
  switch between groves. The backend mechanism (tmux-owner vs in-process pty) is
  decided by the 050 spike; crash-resilience rests on the artifacts-over-state
  model, so live-session survival across a dashboard restart is a *convenience*,
  not a guarantee.
- The fleet view spans multiple repos with filtering; `notify` ignores `.git/`
  churn.
- Concern 4 (async) is formally resolved — refactored or recorded as not-needed.

## Decomposition

Settled in 010-plan (D1–D3), then twice amended: (a) a web-front-end alternative
to the TUI presentation surfaced and gated the architecture decision; (b) in 040,
the resilience reframe (crash-resilience is the artifacts-over-state model, not
multiplexer persistence) **reopened D2** — the harness *backend* — between a
tmux-owned server and an in-process pty embedded in Ratatui. Spine: research the
tmux-ownership prior art → compare web vs TUI presentation → ratify TUI + the
core↔presentation boundary (D3) → **spike the embed to decide the backend (D2)** →
build the harness pane → build the fleet view → revisit async.

```
020-research-tmux-ownership   owner pattern prior art (claude-squad, iTerm2 -CC, …)            [done]
030-web-frontend-comparison   web UI vs Ratatui TUI → recommend TUI                            [done]
040-decide-tmux-integration   D3=TUI + core↔presentation boundary (ADR-0013); D2 reopened      [done]
050-spike-embed-pty-harness   decide D2 empirically: tui-term+portable-pty embed vs tmux-owner → backend ADR
060-harness-pane              live harness beside the dashboard (concern 3) — shape depends on 050's verdict
070-fleet-view                MultiRepoView (concern 1); fs-watch .git filter folds in
080-async-revisit             confirm sync suffices / minimal async (concern 4)
```

Architecture status after 040: **D3 is settled** — v2 presentation is the Ratatui
TUI behind a core↔presentation boundary, web deferred (ADR-0013). **D2 is
reopened** — 010-plan chose a tmux-owned session over in-process pty for "session
persistence + crash isolation," but 040 demoted tmux persistence to a convenience
(resilience is the artifact model), so the embed alternative is live again. 020's
tmux mechanics (socket / scripting / launch / config) are worked out and held as
the plan *if* the 050 spike picks tmux; the binding backend ADR (and whether
060/070 are tmux- or pty-shaped) waits on that spike.

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
