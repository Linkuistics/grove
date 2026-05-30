# 040-harness-pane

**Kind:** planning (decompose into impl leaves once 030's mechanics are fixed)

## Goal

Build the headline feature: from the TUI dashboard, a `d` action opens the
selected grove's harness session as a window in grove's owned tmux session
(running `grove do <name>` / `grove continue <name>`). This is the first
exerciser of the tmux-owner architecture — it validates 030's decisions.

## Context

- Depends entirely on 030's decided mechanics (socket, launch/attach, window
  lifecycle) and the 020 research (Synthesis Q5/Q6: window lifecycle + crash
  isolation).
- v1 already has a `d` keybinding stub on `Screen::GroveDetail`
  (`src/tui.rs` ~line 797) — check what it currently does and replace/extend.
- Confirm the exact verb the window should run. The grove methodology's sole
  lifecycle entry verb is `grove do <name>`; the seed observation referred to
  `grove continue <name>`. Resolve which is current before wiring.

## Done when

- Pressing `d` on a grove opens/focuses its harness window in grove's tmux
  session; re-pressing focuses the existing window rather than spawning a
  duplicate.
- The dashboard reflects window state (running / exited) per the lifecycle
  mechanism 030 chose.
- Closing the dashboard does not kill harness windows (the D2 persistence
  guarantee), verified manually.

## Notes

Sequenced **before** the fleet view (050) by 010-plan decision: prove the risky
tmux-owner architecture earliest. Likely needs decomposition — treat as planning
until the impl steps are small enough for single sessions.
