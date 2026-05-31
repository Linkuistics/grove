# 060-harness-pane — brief

**Kind:** work (backend decided: in-process pty — ADR-0014)

## Goal

Build the **harness pane**: from the dashboard, open a grove's live harness
session (`grove do <name>` → claude code / codex) beside the dashboard,
interact with it, and switch between groves. This is concern 3 — the grove's
headline feature.

Build it as an **extractable in-repo workspace crate** (working name
`harness-pane`) that grove consumes — the reusable embed component the 050 spike
identified (ADR-0014). Publishable later if its API stabilises; no published-crate
overhead now.

## Context

- **Backend decided (050 → ADR-0014): in-process pty**, not tmux. Stack:
  `tui-term 0.2` (render `vt100::Screen` → ratatui) + `portable-pty 0.9` +
  **`vt100 0.15.2`** (pin is load-bearing — tui-term 0.2 re-exports its own vt100
  and only impls `Screen` for 0.15.2; newer fails to compile, and 0.15.2 keeps
  `Screen::title()` for OSC). Sync stack (ratatui 0.29 + crossterm), no async.
- **The 050 prototype is throwaway evidence, not a starting point** — it was a
  scratch crate, now discarded. Rebuild cleanly in `crates/harness-pane`. But its
  *findings* are the spec; carry them forward (see Done-when).
- v1 gives: sync event loop, `RepoView` data layer, shell-out-to-`grove` writes,
  the master/detail dashboard. The harness pane adds a live terminal beside it.
- The dashboard-as-switcher model (040): the user picks the focused grove *in the
  dashboard*; harness panes are alive for parallel work (native ratatui splits,
  no tmux join/break choreography).
- **Boundary (ADR-0013):** the crate owns vt100/pty/input below the seam and
  returns a renderable screen + plain data; only "render this screen / here's a
  key event" crosses into ratatui. No `ratatui` event handler touches pty bytes
  directly — *except* the one deliberate data-up/command-down coupling for mouse
  capture (below), which ADR-0014 records.

## Done when

- A `crates/harness-pane` workspace crate exists, grove consumes it, and from the
  dashboard the user can **launch/attach a harness for the selected grove, see it
  render live, type into it, and switch focus between groves**.
- The **consumer wiring** from the 050 spike is reproduced and solid:
  - reader-thread → `mpsc` → `try_recv` drain per pane between `poll` ticks;
  - full crossterm `KeyEvent`→bytes encoder (ctrl/alt/fn/arrows/nav), SGR mouse,
    bracketed paste, resize → `master.resize()` + `parser.set_size()` together;
  - **native cursor**: hide tui-term's drawn cursor (`Cursor::default().hide()`),
    position the hardware cursor — avoids the white-on-grey unreadable-in-vim bug;
  - **dynamic mouse capture**: enable only while the focused app requests mouse
    (`mouse_protocol_mode != None`), release otherwise so host selection works.
- **Pane-local copy mode** (the 050 outstanding gap, user-confirmed real): a
  selection model over vt100 scrollback so the user can drag-select / scroll
  *within one pane* (not the whole outer grid), with copy to clipboard (OSC-52 or
  platform clipboard). This is the feature tmux would have given free; the embed
  owes it. Scope it as its own sub-step — decompose 060 if it needs >1 session.
- Switching the focused grove re-evaluates mouse-capture state (every focus change,
  not just startup).

## Naming — settled (with user)

Keep **"pane"** meaning a *layout region* (which may hold non-pty content, e.g.
the dashboard). The pty-backed thing inside it gets its own name. Three layers:

- **`TerminalEmulator`** — owns the `vt100` parser/screen, fed *bytes*, renders
  via `tui-term`. Source-agnostic and the testable core (synthetic ANSI, no
  child needed).
- **`PtySession`** — the byte *source*: `portable-pty` master + child + reader
  thread.
- **pane** — grove's layout concern (030) pairing an emulator with input
  routing.

These set the crate's public API and the new CONTEXT.md glossary entries.

## Decomposition (this node)

Base crate → **substrate decision** → feature → integration. The substrate-decision
leaf was inserted after 010 shipped, when a conversation reopened whether zellij (a
named-but-never-evaluated D2 candidate) should be the substrate; it **gates** 030/040.

- **`010-embed-pane`** — scaffold `crates/harness-pane`; build `TerminalEmulator`
  + `PtySession` + input wiring + native cursor + `wants_mouse()`. Proven by
  synthetic-ANSI **and** real-child tests. **[done]**
- **`020-decide-zellij-substrate`** *(planning)* — decide zellij-as-owned-multiplexer
  vs the ADR-0014 in-process embed, on zellij's own terms (the comparison ADR-0014
  folded into "tmux-owner" and skipped). Grill + a throwaway Strategy-1 spike (grove
  dashboard as a zellij plugin rendering ratatui beside native zellij harness panes,
  via `zellij_widgets`) → amend or supersede ADR-0014. **Gates 030/040:** if the
  embed stands they proceed unchanged; if zellij wins, 030 evaporates (copy mode is
  free) and 040 becomes a plugin+layout, not a `src/tui.rs` crate consumer.
- **`030-scrollback-copy`** — pane-local **scrollback (key + mouse)** and a
  selection/copy model over vt100 scrollback → clipboard (OSC-52). The user
  confirmed *both* key- and mouse-driven scrollback are required, with selection
  *within* the pane (not the host terminal's selection). **Conditional on 020:** moot
  if zellij wins.
- **`040-grove-integration`** — grove consumes the crate in `src/tui.rs`:
  launch/attach a harness beside the dashboard, switch focus between groves,
  re-evaluate `EnableMouseCapture` on every focus change. Within-repo only;
  cross-repo fleet is 070. **Reshaped by 020** if zellij wins.

## Notes

The **tmux** backend stays retired (ADR-0014) — do not re-litigate it. What 020
reopens is narrower and was never run: **zellij** as substrate, on its own terms.
Until 020 decides, treat the in-process embed as the working assumption (010 shipped
on it) but hold 030/040 as contingent. The crate boundary is the
deliverable's spine: keep grove's ratatui code calling *into* the crate, never
the reverse (the crate itself legitimately depends on ratatui/tui-term — it *is*
the boundary bridge; the no-ratatui rule constrains grove **core**, not this
crate).
