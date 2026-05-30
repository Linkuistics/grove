# 020-research-tmux-ownership

**Kind:** work (research → produces a docs/research/ survey)

## Goal

Survey how existing tools **own and drive a tmux session as a backend** — the
"dashboard window 0 + one window per long-running child process" pattern grove
committed to (root BRIEF + 010-plan D2/D3). Output: a post-mortem-framed
research doc that de-risks the 030 tmux-integration design and the 040 harness
pane. Do **not** survey in-process pty or zellij as candidates — that fork is
already decided (tmux owner). The job is *how*, not *whether*.

## Context

- Decisions this consumes: 010-plan D2 (tmux owner, not passenger) and D3
  (TUI dashboard is window 0 of grove's owned session).
- v1 code that will be extended: `src/tui.rs`, `src/repo_view.rs`.
- The two downstream consumers are leaf 030 (decide integration mechanics) and
  leaf 040 (build the harness pane). Structure the doc around their questions.

## Done when

A doc at `docs/research/tmux-owning-frontends.md` exists that, for each surveyed
tool, gives: the ownership model, a **walk-away check** (with the tool
uninstalled / its process gone, what survives — do the user's tmux sessions and
the harness processes persist?), and **primary-source citations** (repo, issue,
or docs URL) for every failure-mode or design-rationale claim. Plus a final
**Synthesis** section answering the per-leaf questions below. Absence of a
source is itself recorded ("no primary source found"), not glossed.

### Systems to survey (at least)

- **claude-squad** / **uzi** (and similar AI-agent orchestrators) — they spawn
  each agent into its own tmux session/window and own it. The closest prior art.
- **iTerm2 tmux control mode (`tmux -CC`)** — the deep "frontend becomes the
  tmux client, renders windows as native UI" pattern; what it buys and costs.
- **sesh / tmuxinator / smug** — session managers that create+own sessions
  declaratively; the lighter-weight scripting (`new-session`/`new-window`/
  `send-keys`) end of the spectrum.
- **lazygit / gh-dash** (if they shell to tmux at all) — how a Ratatui-class TUI
  hands off to / spawns sibling panes; note if they *don't*, that's a finding.

### Downstream questions the Synthesis must answer

**For 030 (decide integration):**
1. **Dedicated socket vs default server.** Should grove run `tmux -L grove` on
   its own socket (isolated from the user's tmux, own config) or share the
   default server? What do the orchestrators do, and what broke when they got
   this wrong?
2. **Control mode (`-CC`) vs plain scripting.** Is control mode worth the
   complexity for grove's needs, or do `new-window` / `list-windows` /
   `send-keys` / `display-message -p` suffice? Cite a tool that regretted either
   choice.
3. **Launch sequence.** How does a tool detect-or-create its owned session and
   attach the user (and re-attach on a second invocation without nesting tmux
   inside tmux — the `$TMUX` already-set pitfall)? What's the failure mode when
   launched from inside an existing tmux?
4. **Owned config / keybindings / status line.** How much custom tmux config do
   these tools ship for their session, and how do they avoid clobbering or
   depending on the user's `~/.tmux.conf`?

**For 040 (harness pane):**
5. **Window lifecycle.** How is a per-child window created, named, and torn
   down? How does the dashboard learn a child process exited (window-closed
   hooks? polling `list-windows`?)?
6. **Crash isolation + persistence.** Confirm the load-bearing claim behind D2:
   with the dashboard/TUI killed, do the harness windows and their processes
   survive, and can the user re-attach? Cite the mechanism.

### Search bias

Frame each system as a post-mortem: "after real multi-session use, what went
wrong with owning tmux this way?" Broad "how to use tmux" tutorials are not the
target. Demand a citation per failure-mode claim; record silences explicitly.

## Notes

This leaf may itself prove too big for one session; if so it stays a single
research doc but the executor scopes depth to the six questions above rather
than an exhaustive tmux survey.
