# 010-proxy-protocol

**Kind:** work

## Goal

Build the controller↔proxy seam (ADR-0016): the **IPC protocol**, the dumb
**`grove __dash-proxy`** client, the **input decoder** (raw stdin bytes →
`KeyCode`/`KeyModifiers`), and the **render-over-socket backend** the controller
uses to draw into a proxy. End state: a controller *stub* can render a frame to a
real `grove __dash-proxy` running in a terminal and receive its decoded keystrokes
and resizes — proving the seam before the real dashboard loop (020) plugs in.

## Context

- ADR-0016: the proxy only (1) reports its size to the controller + on SIGWINCH,
  (2) blits controller-sent output to its stdout, (3) forwards stdin up. No state,
  no logic, no ratatui.
- **Render = `CrosstermBackend` over a socket writer** (node-brief decision). The
  controller holds `Terminal<CrosstermBackend<W>>` where `W: io::Write` frames
  bytes down the socket; ratatui's cell-diff minimises them; the ANSI stream *is*
  the down-wire payload, so the proxy is a pure `socket → stdout` copy.
- **Wire shape (proposed; refine in build):** one unix-domain socket, bidirectional.
  - *Down (controller→proxy):* raw output bytes only — one message kind, no framing
    needed; proxy writes whatever arrives to stdout.
  - *Up (proxy→controller):* needs framing to distinguish **resize** from **input**.
    Length-prefixed, type-tagged frames — e.g. `S`=resize (`cols:u16, rows:u16`),
    `I`=input (`len:u32` + raw bytes). Keep it a tiny hand-rolled codec; no serde
    needed for a 2-variant frame.
- **Terminal mode ownership:** the *proxy* owns the real tty — it sets raw mode +
  alternate screen + hides cursor locally on startup and restores on exit (termios
  raw mode can't travel over a socket; this is transport, not ratatui/logic). The
  controller's backend must NOT also toggle alt-screen (construct `Terminal::new`
  directly, not `ratatui::init()`), to avoid double-toggling.
- **Input decoder** (the hard part): crossterm's `event::read()` reads the
  process's own stdin and can't be pointed at a socket, so the controller decodes
  the proxy's raw bytes itself. Cover the dashboard's key subset: printable UTF-8
  chars, Enter/Esc/Tab/Backspace, the CSI arrow keys, and control chars
  (Ctrl-S/E/C, etc., as `Char` + `KeyModifiers::CONTROL`). Mouse may be deferred.
  Emit the same `crossterm::event::{KeyCode, KeyModifiers}` the existing
  `handle_key` already consumes, so 020 wires in with no churn.

## Done when

- A `grove __dash-proxy --socket <path>` subcommand exists (hidden from human
  `--help`, like the other `__`/`grove-llm` surfaces): connects, sets up its tty,
  sends initial size, then runs two pumps (socket→stdout, stdin→socket) and a
  SIGWINCH→resize-frame handler; restores the tty cleanly on exit/EOF.
- The protocol codec (encode/decode of the up-frames; the down direction is raw)
  has unit tests for round-tripping resize and input frames, including partial/
  split reads on the socket.
- The input decoder has unit tests mapping representative byte sequences to the
  right `KeyCode`/`KeyModifiers` (arrows via CSI, Enter, Esc-alone, Ctrl-S, a
  multi-byte UTF-8 char, a paste-ish run of chars).
- The render-over-socket backend renders a ratatui frame to bytes on the socket
  that a real proxy displays correctly (manual check with a stub controller is
  acceptable; the automated check is the codec/decoder tests).

## Notes

- Below the ADR-0013 seam? The proxy client and codec are transport; the *decoder*
  produces presentation input. Keep the codec/proxy free of `RepoView`/ratatui.
  The decoder lives with the controller (it feeds the dashboard), above the data
  layer but it imports no `RepoView`.
- Don't build the real dashboard loop here — only the stub needed to exercise the
  seam. The loop is 020.
- A future web client reuses this protocol (ADR-0016) — keep the frame definitions
  presentation-neutral (bytes + size), not ratatui-specific.
