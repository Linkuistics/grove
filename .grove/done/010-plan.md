# 010-plan

**Kind:** planning

## Goal

Settle the scope and sequencing of the tui-multi-repo-and-multiplexer grove
and grow the task tree. The grove extends the v1 `grove tui` (per-repo reader
+ capture) toward the four deferred concerns: multi-repo fleet view,
multiplexer choice, embedded harness pane, sync→async refactor. See the root
`BRIEF.md` for the inherited v1 architecture and the seeded concerns.

## Context

- v1 shipped: `src/tui.rs` (~2400 lines, sync Ratatui + crossterm, `notify`
  watch with 200ms debounce) over `src/repo_view.rs` (444 lines, `RepoView`
  keyed on a single `repo_root`; `GroveSummary` / `GroveDetail` / `TaskTree`).
- Writes shell out to `grove` / `grove-llm` verbs only (walk-away preserved).
- The multiplexer choice is the linchpin: it gates the harness pane and shapes
  the fleet view and the async refactor.

## Done when

- The task tree below `010-plan` is grown with ordered leaves/nodes.
- Key scope decisions are recorded in the running log and (where durable)
  promoted to ADRs.

## Decisions (running log)

### D1 — Grove spine: multiplexer research first

The four concerns interlock around the multiplexer choice (it gates the
harness pane, shapes the fleet view and the async refactor). Per the driving
field-guide rule — insert a research leaf ahead of a planning leaf when the
design depends on lessons prior tools learned the hard way — we lead with a
**prior-art research leaf** on the multiplexer options, then a planning leaf
that decides, then build fleet view / harness pane / async outward.

### D2 — Multiplexer: tmux, and grove *owns* it (not a passenger)

User: "tmux is best… I definitely want grove to **own** the process, rather
than just being something that runs in a tmux session." So grove is not a
guest pane inside the user's ambient tmux (passenger model); grove drives tmux
as its own backend engine — it creates and manages a dedicated tmux session
and owns the harness processes' lifecycle inside it. tmux is grove's process
supervisor.

Decisive rationale: **session persistence + crash isolation.** A
`grove continue/do <name>` harness session runs an hour-plus; with tmux owning
it, the session survives the TUI closing or crashing, and gets battle-tested
scrollback / copy-mode / detach for free. In-process pty (tui-term +
portable-pty) was rejected: closing the TUI would kill the live session.

Knock-on: this likely **deflates concern 4 (async refactor)**. The async case
rested on "juggling subprocess output for harness panes"; if tmux owns those
windows, the TUI never touches their ptys — it shells out `tmux` commands and
stays a sync reader. N-repo watch is solvable with v1's existing
`notify` + debounce. Concern 4 may shrink to "confirm sync suffices" or drop.

Prior-art lineage to mine: AI-agent orchestrators that own tmux sessions per
agent (claude-squad / uzi / similar), and iTerm2's tmux control mode
(`tmux -CC`) for the deep frontend-becomes-client pattern.

Consequence for D1: the research leaf **narrows** from a broad 3-way survey
(tmux/zellij/in-process) to a focused study of *how* the tmux-owning
orchestrators structure it — avoids the "pre-baked answer" anti-pattern while
still de-risking the tmux mechanics.

### D3 — TUI placement: dashboard is window 0 of grove's owned session

grove launches/owns a dedicated tmux session; the TUI dashboard runs as
window 0; each harness is another window grove creates. The user switches
between dashboard and harness windows via tmux (or grove-proxied keys). grove
fully owns and lays out one tmux session — the claude-squad / purpose-built
tmux-frontend shape. (Rejected: TUI as a standalone external controller over a
detached server — the in-session window model gives a more unified UX and lets
grove own the status line / keybindings of its session.)

Open mechanics deferred into 020-research / 030-decide (not settled here):
dedicated socket (`tmux -L grove`) vs default server; control-mode vs plain
scripting (`new-window`/`send-keys`); what tmux config/keybindings/status line
grove ships for its owned session; the `grove tui` launch sequence
(detect-or-create session, attach); attach/detach + crash-recovery UX.

## Revised tree (post D1–D3)
```
020-research-tmux-ownership.md   research leaf (narrowed)
030-decide-tmux-integration.md   planning, consumes 020 → ADR(s) + design
040-fleet-view/                  MultiRepoView (concern 1); fs-watch .git folds in
050-harness-pane/                'd' opens a grove-do window (concern 3)
060-async-revisit.md             confirm sync suffices / minimal async (concern 4)
```
Ordering of 040 vs 050 still open — see next grilling question.

## Notes
