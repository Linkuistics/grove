# 030-decide-tmux-integration

**Kind:** planning

## Goal

Consume the 020 research and **decide** grove's tmux-integration mechanics, then
grow the implementation tree for the harness pane. Settle the open mechanics
deferred from 010-plan D3: dedicated socket, control-mode vs scripting, the
`grove tui` launch/attach sequence, and the tmux config grove ships for its
owned session. Raise ADR(s) for the durable, hard-to-reverse choices.

## Context

- Inputs: 010-plan D2/D3 (the owner model + TUI-as-window-0 decision) and
  `docs/research/tmux-owning-frontends.md` (from 020).
- v1 launch path to change: `src/tui.rs::run` (currently just sets up a terminal
  and renders). Under the owner model it must detect-or-create grove's tmux
  session and attach.
- Likely-ADR candidates: socket isolation strategy; control-mode vs scripting;
  whether `grove tui` becomes a tmux launcher (a surprising identity shift worth
  recording).

## Done when

- The mechanics questions (020 Synthesis Q1–Q4) are decided and recorded in a
  running log here, with ADR(s) for the durable ones (cite the 020 research by
  primary source in each ADR's rationale).
- A short design spec exists (e.g. `docs/specs/tmux-integration-design.md`) if
  the design is large enough to warrant one.
- The harness-pane leaf (040) is decomposed if 030 reveals it needs >1 session,
  or its brief is sharpened with the decided mechanics if it fits in one.

## Notes

This is the planning leaf that turns research into a binding design. Keep
`CONTEXT.md` updated inline if new terms resolve (e.g. a name for grove's owned
session / socket).
