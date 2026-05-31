# 010-embed-pane

**Kind:** work

## Goal

Stand up the **`crates/harness-pane`** workspace crate and build the
source-agnostic terminal embed it exists for — the base pane that renders a
live child process and takes input, proven both headless and against a real
child. Scrollback/copy (020) and grove wiring (030) build on this; neither is
in scope here.

End state: a self-contained crate that can spawn a real harness, render its
screen via `tui-term`, and feed it keystrokes — with the consumer wiring the
050 spike found `tui-term` leaves open already in place.

## Naming (settled with user)

Three layers, named to keep "pane" meaning a *layout region* (which may hold
non-pty content), distinct from the pty-backed thing inside it:

- **`TerminalEmulator`** — owns the `vt100::Parser`/`Screen`, is **fed bytes**
  (`process(&[u8])`), and renders via `tui-term`. Source-agnostic: a pty,
  synthetic ANSI, or a recording can all feed it. This is the testable core.
- **`PtySession`** — the byte *source*: `portable-pty` master + child + the
  reader thread. Produces bytes, accepts input/resize, reports child status.
- **pane** — a layout region (grove's concern, 030) that pairs an emulator with
  input routing. Not a crate type.

This split is also the main testing lever: `TerminalEmulator` is driven by
synthetic ANSI with no child process (mirrors the spike's `--dump`).

## Context

- **Stack pins (load-bearing, from ADR-0014 / 050):** `tui-term 0.2` +
  `portable-pty 0.9` + **`vt100 0.15.2`**. tui-term 0.2 re-exports its own
  `vt100` and only impls `Screen` for 0.15.2 — pinning a newer vt100 alongside
  fails to compile; 0.15.2 also keeps `Screen::title()` for OSC. Sync only
  (no async/tokio) — the 050 backlog evidence (4 chunks/tick) says sync
  suffices; async is 080's call, not ours.
- **Boundary (ADR-0013):** this crate *is* the presentation-boundary bridge —
  it legitimately depends on `ratatui`/`tui-term` because its job is to return
  a renderable widget + plain data. The "no `ratatui` below the seam" rule
  constrains **grove's core** (`repo_view` etc.), not this crate. grove's TUI
  (above the seam, 030) consumes it; grove core never imports it.
- **Pump pattern (050):** blocking `try_clone_reader()` on a reader thread →
  `mpsc::channel` → drained via `try_recv()` between event-loop ticks. The sync
  owner of the parser is the only mutator, so no lock on the parser.
- **The 050 prototype is discarded** — rebuild cleanly; its *findings* are the
  spec (see the 060 BRIEF and `done/050-spike-embed-pty-harness.md` Findings).

## Done when

- A `crates/harness-pane` crate exists in a cargo **workspace** (root
  `Cargo.toml` gains `[workspace]`); `cargo build` and `cargo test` are clean
  with the version pins held (`Compiling vt100 v0.15.2`).
- **`TerminalEmulator`**: construct with `(rows, cols)`; `process(&[u8])` feeds
  vt100; renders via tui-term's `PseudoTerminal` widget; `resize(rows, cols)`
  calls `parser.set_size`; exposes cursor (row/col/visible) and
  `screen().title()`; **hides tui-term's drawn cursor** so the host positions
  the native hardware cursor (the white-on-grey-in-vim fix).
- **`PtySession`**: `spawn(argv, cwd, env, rows, cols)` → `portable-pty` pair +
  child; reader thread → `mpsc`; `drain()`/`try_recv` of output chunks;
  `write_input(&[u8])` to the master; `resize` that updates **`master.resize`
  and the emulator together**; child status via `try_wait`.
- **Input encoder**: crossterm `KeyEvent`→bytes (ctrl/alt/function/arrows/nav),
  SGR mouse **gated on `screen.mouse_protocol_mode()`**, bracketed paste.
- **Dynamic mouse capture (the data-up half)**: expose `wants_mouse()`
  (`mouse_protocol_mode() != None`) so 030's host can toggle `EnableMouseCapture`
  per focus. The crate never calls `execute!` itself — it only reports intent.
- **Tests, both halves of the 050 assessment:**
  - *headless / synthetic ANSI* — feed colors + cursor moves + an OSC title to a
    `TerminalEmulator`, assert cell contents, cursor position, and parsed title
    (no child process);
  - *real child* — `PtySession::spawn` a deterministic command (e.g.
    `sh -c 'printf ...'`), drain output, assert the grid contains the expected
    text and the child exits.

## Notes

- Keep the crate **generic/publishable** — no grove-specific types, no
  knowledge of `grove do`. That belongs in 030.
- Out of scope: scrollback navigation, selection, clipboard (→ 020); launching
  the harness from the dashboard, focus switching, real `EnableMouseCapture`
  toggling (→ 030).
- Isolate the clipboard/terminal side-effects behind small seams now if cheap,
  so 020 and 030 can test against them — but don't build 020's model here.
