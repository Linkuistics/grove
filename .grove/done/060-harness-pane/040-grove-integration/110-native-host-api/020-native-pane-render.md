# 020-native-pane-render

**Kind:** work

## Goal

Add grove's **native surface as a third pane kind** in the trellis server: a host
ratatui buffer rendered to `CharacterChunk`s **server-side**, composited by zellij
as a real pane, and receiving **input + events in-process** (key/mouse/resize/
focus, the crossterm model). Prove it with a trivial self-drawing surface (e.g. a
"hello, counter" ratatui widget that increments on keypress) displayed as a real
trellis pane beside/instead of a terminal pane. This is ADR-0021 seam point (3)
plus the input half of the host API.

## Context

- **The third pane kind (ADR-0021 Evidence #4).** `PaneId` is a closed
  `Terminal(u32) | Plugin(u32)` enum (zellij-server/src/panes/terminal_pane.rs:89);
  `trait Pane` (zellij-server/src/tab/mod.rs:228) renders to `Vec<CharacterChunk>`.
  grove's native surface becomes a **third kind** — a `Pane` impl producing
  `CharacterChunk`s from grove's ratatui buffer instead of from a pty (Terminal)
  or wasm (Plugin). Expect to touch every place that matches on `PaneId` /
  constructs panes (Tab, Screen, the pane containers). This is the meatiest part
  of the whole node — the fat `Pane` trait (~40 methods) and the closed enum mean
  the change fans out across the server's tab/pane layer.
- **Render bridge.** grove draws into a ratatui `Buffer` (off-screen, via a custom
  ratatui backend or `Terminal::draw` over an in-memory buffer); the native pane
  converts that `Buffer`'s cells → zellij `CharacterChunk`s with styles, honouring
  the pane geometry the server assigns (`set_geom`, content x/y/cols/rows). Keep
  the ratatui-buffer→CharacterChunk converter small and testable in isolation
  (feed a synthetic `Buffer`, assert chunks) — mirrors the harness-pane crate's
  source-agnostic testing discipline.
- **Cursor + input.** Implement `cursor_coordinates` (position + visibility) so
  the host cursor lands right, and `adjust_input_to_terminal` (or the equivalent
  routing) so key/mouse bytes reach grove's surface as crossterm
  `KeyCode`/`KeyModifiers` events — replacing the 010-era hand-rolled socket
  decoder (now dead). Resize updates the surface's area; focus changes are
  delivered so the surface knows when it's active.
- **In-process, server-side.** Per ADR-0021 Evidence #3, rendering happens inside
  the server daemon. grove's surface state + draw code is **server-side**; plan it
  there, reachable on the `--server` re-exec path (010's constraint).
- **Boundary (ADR-0013).** The ratatui draw code is *above* the seam; it must not
  reach into `RepoView`/writes here — 020 only needs a self-contained demo
  surface. The real data-backed dashboard is 030.

## Done when

- A trivial grove-native ratatui surface renders as a real trellis pane (correct
  geometry, styles, cursor) and **updates in response to keypresses** — driven
  in-process, no socket, no WASM, no `zellij action`.
- The `Buffer`→`CharacterChunk` converter has unit coverage with synthetic input.
- Mouse + resize + focus reach the surface (at least: resize reflows, focus is
  observable). The crossterm `KeyCode`/mouse model is what the surface consumes.
- `cargo build`/`cargo test` green; grove core below the ADR-0013 seam stays
  `ratatui`-free (the ratatui surface + converter live above it).

## Notes

- Resist generalising the pane API beyond what 030's dashboard needs — it's
  discovered by a real consumer (constraint 4). A single native-pane kind that
  grove can draw into and drive is the target; multi-host registries, the
  observability API, GraphQL are all later/out-of-scope.
- The hardest unknowns are in the server's pane/tab plumbing (how new pane kinds
  are created, sized, focused, composited). Read `Screen`/`Tab` and the existing
  `TerminalPane` / plugin pane impls as the templates to mirror.
- Tab/pane *layout control by direct call* (create/close/focus) is 030's concern —
  here a single statically-placed native pane is enough to prove render+input.
