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

## Findings (build-measure running log)

The prototype lives in a throwaway sibling scratch crate
`../grove-embed-spike/` (outside this repo; `rm -rf` to discard). Stack mirrors
v1: `ratatui 0.29` + `crossterm 0.28` (via `ratatui::crossterm`), **sync** loop,
no async. Embed stack: `portable-pty 0.9` + `tui-term 0.2` + `vt100 0.15.2`.

### Build/integration facts (settled)

- **Version-pin trap (the load-bearing gotcha):** `tui-term 0.2.0` re-exports its
  *own* `vt100` (`pub use vt100;`) and only `impl`s its `Screen` trait for *that*
  vt100's `Screen`. It pins **`vt100 ^0.15.2`**, not the latest 0.16.x. Pinning
  vt100 0.16 alongside resolves two vt100 crates and `PseudoTerminal::new(parser
  .screen())` fails to compile with a type mismatch. **Pin `vt100 = "0.15.2"`** (or
  depend only on `tui_term::vt100`). `Screen::title()` also exists in 0.15.2 but was
  removed in 0.16.x — another reason 0.15.2 is the right floor for this widget.
- **Input is the consumer's job** (tui-term is render-only). The prototype
  implements a full crossterm `KeyEvent`→bytes encoder (ctrl/alt/function/arrows/
  nav), SGR mouse encoding **gated on `screen.mouse_protocol_mode()`** (only forward
  mouse when the app requested it), bracketed paste, and resize→`master.resize()` +
  `parser.set_size()` together. Modelled on tui-term's `nested_shell.rs`/`smux.rs`.
- **Pump pattern:** blocking `try_clone_reader()` on a reader thread →
  `mpsc::channel` → drained via `try_recv()` between `event::poll()` ticks on the
  main thread. No lock on the parser (unlike nested_shell's `Arc<RwLock>`): the sync
  main thread owns the parser, so vt100 mutation never races the render.
- Built **first try, zero errors/warnings** after the version pin. (`cargo build`
  clean; `Compiling vt100 v0.15.2` confirms the pin held.)

### Headless smoke-test evidence (automated half of the assessment)

- **Synthetic ANSI** (`--dump -- bash -lc 'printf …colors… ; OSC title'`): text
  extracted cleanly (SGR codes consumed, not leaked); **OSC title parsed**
  (`title: "my-title"`); alt-screen / cursor / mouse-mode all tracked.
- **Real claude** (`--dump claude`, claude 2.1.158): launched and rendered its
  full startup banner as **legible structure** — box-drawing frame, two-column
  layout, the `▐▛███▜▌` logo unicode, input box, status line — not garbage. Colors
  resolve (`Idx(1)`=red, `Idx(2)`=green+bold). `alternate_screen: false` — claude
  renders **inline**, not on the alt-screen, at least at startup — favourable for
  embedding. **OSC title parsed**: `"✳ Claude Code"` (and `"my-title"` from the
  synthetic OSC probe). Startup output: **5,341 bytes / 15 chunks** over the 2.5 s
  window (262 sync ticks), vs bash's 80 bytes / 2 chunks.
- **Async question — answered concretely:** under claude's startup burst the sync
  loop's **max backlog was 4 chunks/tick** (drained per tick via `try_recv`). The
  existing sync event loop absorbs it trivially. Evidence that N embedded ptys can
  be pumped in the v1 sync loop **without** forcing the 080 async refactor — 080
  stays a confirm-sync-suffices leaf, not a refactor — *pending* the multi-pane
  visual pass holding up under sustained real interaction (a 5 KB startup burst is
  lighter than, e.g., a fast-scrolling build log, which the visual pass should
  stress).

### Human visual pass — fidelity checklist (RESOLVED)

User ran the prototype interactively in iTerm2 against real `claude` and `nvim`.
Result: **fidelity holds.** Every checklist item works after two consumer-wiring
fixes; the one remaining gap is a buildable feature, not an emulation limit.

| Probe | Verdict |
|---|---|
| colors | ✅ correct (matches bare claude) |
| cursor positioning | ✅ after fix (see below) |
| alternate screen | ✅ nvim enters/restores cleanly (`alt:Y`) |
| mouse | ✅ after fix — nvim click/scroll works |
| resize / SIGWINCH | ✅ reflows correctly |
| unicode width | ✅ once the **host-terminal** dual-font issue was fixed (below) |
| OSC (title) | ✅ parsed (`"✳ Claude Code"`) |
| scrollback | ✅ (vt100 scrollback wired) |
| input latency | ✅ feels native |

**Three issues surfaced; all diagnosed and resolved without favouring tmux:**

1. **Box-drawing/unicode misalignment — NOT an embed bug.** Root cause was the
   user's iTerm2 *"Use a different font for non-ASCII text"* setting (ASCII in
   OperatorMono, box/block glyphs in CascadiaCode → mismatched advance widths →
   columns drift). The headless screen-dump already proved vt100 placed every
   cell on a perfect grid; the artifact was the *host terminal's* final paint of
   two fonts. Decisive point: **bare claude in the same iTerm2 misaligns
   identically** — the embed faithfully reproduces the terminal, it does not add
   the glitch. A tmux backend would be subject to the same host-font issue. Fixed
   by disabling the dual-font setting (single Nerd Font has the glyphs).
2. **Cursor white-on-light-grey, unreadable in vim — double-cursor bug, fixed.**
   tui-term draws its *own* cursor cell with a default overlay style that landed
   under the app's fg. Fix: `Cursor::default().hide()` on the widget, position the
   **native hardware cursor** ourselves (`f.set_cursor_position`). One cursor, the
   app's colours.
3. **Mouse dead in claude AND no text-selection — worst-of-both, fixed.** The
   prototype unconditionally enabled `EnableMouseCapture`, so the host stopped
   doing native selection, *and* claude (which requests no mouse,
   `mouse_protocol_mode: None`) got nothing. Fix: **dynamic capture** — enable
   only while the focused app requests mouse (nvim → `ButtonMotion`), release
   otherwise so the host's native selection/copy returns over a claude pane. (The
   `execute!` toggle forced a `Write` bound on the loop's backend — a small
   data-up/command-down coupling across the ADR-0013 seam; tmux wouldn't have it.)

### Outstanding gap → folds into 060 (NOT a D2 blocker)

**Pane-local copy mode.** Even with dynamic capture, the host's *native* selection
selects the whole outer terminal grid — it would grab dashboard chrome + harness
together, not just one pane. Real per-pane "drag to select/scroll *this pane*,
copy *this pane's* text" needs a **selection model built into the embed layer**
over vt100's scrollback (track selection, render highlight, navigate scrollback,
copy via clipboard/OSC-52). tmux gets this free (copy-mode per pane); the embed
must build it. **Confirmed real by the user.** This is a 060 (harness-pane)
feature, not a spike blocker — recorded here, lands in the 060 brief.

### User steer — "this looks like a publishable component in its own right"

The user observed that the embed widget (vt100 + tui-term + the consumer wiring:
key/mouse/paste encoding, native-cursor handling, dynamic mouse capture, and the
forthcoming pane-local copy mode) is a **reusable Rust component** — exactly the
gap tui-term (render-only, WIP) leaves open. Captured as a decision point for the
060 reshape: build the embed as an extractable unit, with packaging (internal
module / in-repo crate / published crate) decided next.
