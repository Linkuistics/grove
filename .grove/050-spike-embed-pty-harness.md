# 050-spike-embed-pty-harness

**Kind:** planning (a build-measure-decide spike: prototype → assess → decide D2 → backend ADR)

## Goal

Decide **D2 — the harness backend** — empirically: does grove embed the live
harness (claude code / codex) **in-process** as a Ratatui widget (`tui-term` +
`portable-pty`), or drive an **owned tmux server** (the 010-plan model)? Build a
throwaway prototype, assess it against real harness sessions, then make the call
and write the binding **backend ADR** (the held ADR-0014).

This gates everything downstream: it determines whether 060 (harness pane) and
070 (fleet view) are tmux-shaped or pty-shaped, and whether 080 (async) reopens.

## Context

- **Why this leaf exists** — 040 ratified D3 (TUI presentation + the
  core↔presentation boundary, ADR-0013) but **reopened D2**. 010-plan chose
  tmux-owner over in-process pty for "persistence + crash isolation"; 040's
  resilience reframe demoted tmux persistence to a *convenience* (resilience is
  the artifacts-over-state model, spine constraint 1), so the embed alternative is
  live again. Full reasoning: `.grove/done/040-decide-tmux-integration.md`
  running log, "D2 REOPENED" entry.
- **The load-bearing question is fidelity**, not feasibility — embedding is known
  to work (see prior art) but is not turnkey. The spike resolves whether the
  fidelity/input/resize quality is good enough for a heavy modern TUI like claude
  code.
- **Prior art to study first (don't start from scratch):**
  - **maestro-tui** (lib.rs/crates/maestro-tui) — a Ratatui dual-pane multiplexer
    that puts a shell and **Claude Code side-by-side** via in-process PTY. Almost
    exactly grove's feature without tmux; read its `terminal.rs` PTY handling.
  - **`tui-term`** (a-kenji/tui-term) + **`portable-pty`** (wezterm) — the embed
    stack. `tui-term` renders a `vt100::Screen` into Ratatui cells; it's WIP, the
    consumer wires input, `vt100` is the only emulation backend.
  - **Turborepo** drives `tui-term`'s `vt100` path in production (vercel/turborepo
    PR #9123 is a perf pass) — evidence the emulator scales.
- v1 stack to respect: sync event loop, `ratatui` 0.29 + crossterm, no async deps
  (`Cargo.toml`); the presentation boundary (ADR-0013) — keep pty management
  *below* the seam, not entangled with ratatui widgets above it.

## Done when

- A throwaway prototype embeds a **real** harness session (`grove do <name>`
  running claude code/codex) in a Ratatui pane and is **driven interactively**
  (keystrokes in, live output out), assessed against a fidelity checklist:
  **colors, cursor positioning, alternate screen, mouse, resize/SIGWINCH, unicode
  width, OSC sequences (titles/clipboard), scrollback.** Each item: works /
  glitches-how / unsupported.
- The **async question** is answered concretely: can N embedded ptys be pumped in
  the existing sync loop, or does this force the 080 refactor? (Feeds 080.)
- **D2 is decided** — in-process pty *or* tmux-owner — with the evidence, and
  **confirmed with the user** before ratifying (the verdict reshapes 060/070).
- The **backend ADR (ADR-0014)** is written: either *in-process-pty backend* or
  *tmux-owner backend* (`tmux -L grove -f grove.conf`, plain scripting, Q1–Q4 as
  worked out in 040's log). Cite the spike findings + prior art in its rationale.
- **060/070/080 are reshaped** to match the verdict (sharpen briefs; tmux Q1–Q4
  promote from 040's conditional log into ADR-0014 only if tmux wins).

## Notes

This is a spike — the prototype is **disposable evidence**, not the start of the
implementation; resist polishing it. The decision + ADR are the deliverable. If
the embed's fidelity is clearly good (maestro-tui suggests it can be), in-process
pty collapses most of 040's tmux complexity (no socket/config/`$TMUX`/layout
choreography) into native Ratatui widgets — that simplicity is a real thumb on the
scale, but **only if fidelity holds for claude code specifically**. If the embed
glitches on claude code's rendering in ways `vt100` can't fix, tmux-owner is the
safe fallback (a real terminal renders anything), and 040's mechanics are ready.
