# 14. Harness backend is an in-process pty embedded in Ratatui

- Status: superseded by ADR-0015 — and the entire substrate tower this began
  (ADR-0014→0026) is dissolved by [ADR-0028](0028-rmux-substrate.md), the rmux
  substrate (2026-06-10, rmux-substrate 070-teardown, D4)
- Date: 2026-05-31
- Deciders: Antony Blakey (with grove spike 050)

> **Superseded by [ADR-0015](0015-harness-substrate-is-zellij-owned-multiplexer.md)
> (2026-05-31).** This ADR rejected the "owned multiplexer" alternative on
> **tmux's** weaknesses, folding zellij into the same bucket without evaluating it
> on its own terms. Leaf 060/020 ran that missing comparison: a grove-owned
> **zellij** substrate (head binary + native dashboard pane + native harness panes,
> driven via `zellij action`) clears the keybinding/chrome blocker and gives copy
> mode, scrollback, search, session persistence, and a web client for free.
> The in-process-pty embed (the `harness-pane` crate) is retained as the documented
> **fallback**, not the live path. The fidelity findings and consumer-wiring lessons
> below remain valid evidence for that fallback.

## Context

v2's headline feature is the **harness pane** — a grove's live, interactive
`grove do <name>` session (claude code / codex) shown beside the dashboard. The
realisation mechanism (decision **D2**) had two candidates:

- **tmux-owner** — grove drives its own private tmux server (`tmux -L grove -f
  grove.conf`), the harness runs in a tmux pane, the dashboard is a tmux window.
  Mechanics worked out in 040 (020 research): dedicated socket, plain scripting
  (not `-CC`), `$TMUX` refuse-and-instruct, shipped `grove.conf`, zoom-toggle vs
  join-pane layout.
- **in-process pty** — grove embeds the harness as a native Ratatui widget via
  `tui-term` (renders a `vt100::Screen` into ratatui cells) over `portable-pty`,
  with grove wiring input itself.

010-plan chose tmux-owner over in-process pty for "session persistence + crash
isolation." 040's **resilience reframe** dissolved that justification:
crash-resilience is the artifacts-over-state model (spine constraint 1) — a grove
restarts and re-derives its next step from the task tree + git — so tmux
persistence is a *convenience*, not a correctness property. That reopened D2, to
be decided **empirically** rather than by the stale "embedding is too hard"
assumption. Spike 050 built a throwaway prototype (`tui-term 0.2` + `portable-pty
0.9` + `vt100 0.15.2`, on v1's **sync** ratatui-0.29/crossterm stack) pointed at a
real `claude` 2.1.158 session, and assessed it headlessly + by human visual pass.

## Decision

We will build the harness backend as an **in-process pty embedded in Ratatui**,
not a tmux-owned server. The held tmux-owner plan (040's Q1–Q4 mechanics and
layout model) is **retired, not ratified** — kept only in 040's archived log as
the path not taken.

The embed will be developed as an **extractable in-repo workspace crate**
(working name `harness-pane`), consumed by grove, publishable later if its API
stabilises — not as an internal-only module and not as a published crate now.

The backend lives **below the core↔presentation seam** (ADR-0013): the crate owns
vt100/pty/input and returns a renderable screen + plain data; only the thin
"render this screen / here is a key event" surface crosses into ratatui.

## Consequences

**Easier / won:**

- Collapses *all* of 040's conditional tmux machinery — private socket, shipped
  `grove.conf`, `$TMUX` refuse-and-instruct, prefix-collision avoidance,
  `join-pane`/`break-pane` vs zoom-toggle choreography — into native Ratatui
  widgets and layout. The single-window dashboard+harness layout is trivial
  splits; switching focus is grove's own state, no multiplexer protocol.
- One binary, no external tmux dependency, no `~/.tmux.conf` interaction surface.
- **Sync suffices.** Under claude's startup burst the sync loop's max backlog was
  **4 chunks/tick** (reader-thread → `mpsc` → `try_recv` drain per pane between
  `event::poll` ticks). 080 stays a *confirm-sync-suffices* leaf, not a refactor.

**Fidelity verdict (050 visual pass against real claude + nvim):** colors, cursor,
alternate-screen, mouse, resize/SIGWINCH, unicode width, OSC title, scrollback,
input latency — **all pass**, after two consumer-wiring fixes. No item was an
emulation failure.

**Harder / cost (honest):**

- **Input is grove's job.** `tui-term` is render-only (and WIP): grove wires the
  full crossterm `KeyEvent`→bytes encoder, SGR mouse, bracketed paste, resize.
- **Cursor** needs care: hide tui-term's drawn cursor (its default overlay style
  renders white-on-grey, unreadable in vim) and position the native hardware
  cursor instead.
- **Mouse capture must be dynamic** — enabled only while the focused app requests
  mouse (vt100 `mouse_protocol_mode != None`), released otherwise so the host
  terminal's native selection/copy still works. Unconditional capture is
  worst-of-both (no app mouse *and* no copy). This couples emulator state (below
  the seam) to a backend control command (above) — a data-up/command-down flow
  tmux would not have.
- **Pane-local copy mode must be built.** The host terminal's native selection
  spans the whole outer grid (dashboard + harness together), not one pane. True
  per-pane drag-select/scroll/copy needs a selection model over vt100 scrollback
  (track selection, render highlight, OSC-52 clipboard) inside the embed crate.
  tmux gets this free; the embed owes it. **Folds into 060**, not a blocker.
- **A real terminal renders anything; vt100 is an emulator.** The fallback if a
  future harness defeats `vt100` is tmux-owner (040's mechanics are recoverable
  from history). The 050 evidence is that claude code — a heavy modern TUI — does
  not defeat it.
- **Host-terminal artifacts pass through unchanged.** 050 hit box-drawing
  misalignment that was the user's iTerm2 dual-font setting, not the embed (bare
  claude misaligns identically). The embed faithfully reproduces the host
  terminal — including its misconfigurations; a tmux backend would too.

**Reshaping:**

- **060 (harness pane)** is pty-shaped: build the `harness-pane` crate (embed +
  input wiring + native cursor + dynamic mouse capture + **pane-local copy
  mode**), grove consumes it, dashboard-as-switcher over native splits.
- **070 (fleet view)** is unaffected by the backend choice (data layer), but its
  harness panes are pty widgets, not tmux panes.
- **080 (async-revisit)** narrows to confirming sync suffices, with the 050
  backlog evidence as its starting point.

## Notes

- Supersedes the held tmux-owner plan in 040's log (D2 REOPENED entry and the
  conditional Q1–Q4 / layout-model entries).
- Builds on **ADR-0013** (presentation boundary): the backend sits below the seam.
- Spike evidence + the consumer-wiring fixes are recorded in the 050 task file's
  "Findings" running log (`.grove/done/050-spike-embed-pty-harness.md` after
  retirement). Prototype was a throwaway scratch crate, discarded after the spike.
- Stack pin worth carrying forward: **`tui-term 0.2.0` requires `vt100 0.15.2`**
  (it re-exports its own vt100 and only impls its `Screen` trait for that
  version); pinning a newer vt100 alongside fails to compile. `vt100 0.15.2` also
  still has `Screen::title()` (removed in 0.16.x), which the OSC-title path uses.
