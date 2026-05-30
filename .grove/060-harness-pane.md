# 060-harness-pane

**Kind:** planning (decompose into impl leaves once the 050 spike fixes the backend)

## Goal

Build the headline feature: from the TUI dashboard, open a grove's **live harness
session beside the dashboard** (claude code / codex via `grove do <name>`),
interact with it, and switch between active groves. First real exerciser of the
v2 harness architecture.

## Context

- **Backend is decided by 050, not assumed here.** 050 (the embed spike) decides
  D2 — in-process pty (`tui-term` + `portable-pty`, harness = a Ratatui widget) vs
  tmux-owner (harness = a pane on `tmux -L grove`). This brief is written
  mechanism-neutral; **re-sharpen it from 050's verdict + ADR-0014** before
  decomposing. If pty: layout is native Ratatui widgets, no tmux. If tmux: apply
  040's worked-out Q1–Q4 mechanics (socket/config/`$TMUX`-refuse/plain-scripting)
  and the pane-layout spike below.
- **Layout intent (locked in 040, mechanism-independent):** *single window* —
  a persistent **dashboard pane** that is the **navigation/switch surface** (the
  user picks which working grove to focus *in the dashboard*) plus the active
  grove's **harness pane** shown alongside; N harnesses alive for parallel work.
  Not separate full-screen windows. See `CONTEXT.md` → `Dashboard`, `Harness
  pane`.
- **If the verdict is tmux**, the layout has a known wrinkle to spike first: a
  tmux client shows one window at a time and `resize-pane -Z` zooms a pane to the
  *whole* window (hiding the dashboard too), so "dashboard + one full-size harness,
  others hidden" needs `join-pane`/`break-pane` choreography, not zoom. 040's log
  recommends starting with the zoom-toggle (overview ⟷ focus) and escalating to
  join-pane only if needed. **If the verdict is pty**, this wrinkle evaporates —
  it's just ratatui layout.
- **Verb:** the window/pane runs **`grove do <name>`** — confirmed the sole
  lifecycle entry verb (`grove continue` is internal-only; `cli.rs`/`launch.rs`).
  The old seed's `grove continue` reference is stale.
- v1 has a `d` keybinding stub on `Screen::GroveDetail` (`src/tui.rs` ~line 797) —
  check and replace/extend.
- Keep harness/pty (or tmux) driving **below** the presentation boundary
  (ADR-0013); ratatui rendering above.

## Done when

- Selecting a grove opens/focuses its live harness beside the dashboard;
  re-selecting focuses the existing one rather than spawning a duplicate.
- The dashboard reflects harness state (running / exited) and is the switch
  surface between active harnesses.
- Closing/restarting the dashboard does not lose work — recovery is `grove do
  <name>` re-deriving state from artifacts (resilience = artifact model, not
  live-session durability; live-session survival is a convenience the chosen
  backend may or may not provide).

## Notes

Sequenced before the fleet view (070) by 010-plan: prove the risky harness
architecture earliest. Likely needs decomposition — treat as planning until the
impl steps are small enough for single sessions, and only after 050's verdict has
re-sharpened this brief.
