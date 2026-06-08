# 010-wire-open-in-editor

**Kind:** work

## Goal

Wire **leader → `e`** so it dumps the focused harness pane's full **rendered** history into
`$EDITOR`, restoring the TUI cleanly on exit. Pure shell-out to the stock `rmux capture-pane`
CLI — no fork, no SDK change, no proto change (ADR-0029, D-A).

## Context

The 040 brief's scoping spike found published rmux 0.5.0 already exposes rendered-history
capture via `rmux capture-pane`. This leaf is the wiring only. Mirror the existing capture
write (`src/tui/app.rs::submit_capture` + `src/tui/capture.rs::write_capture`): a pure
command-builder below the seam, invoked via `spawn_blocking`, leader-gated above it.

Key engine facts (from 030-engine):
- The TUI owns the loop (`src/tui/app.rs`); terminal setup/teardown already does
  raw-mode + alt-screen + mouse + bracketed-paste enter/leave.
- `spawn_input_reader` is a **separate OS thread** polling crossterm — it will race the
  editor for stdin unless paused (D-E).
- Panes are addressed by stable `PaneId` (`src/tui/driver.rs::id()`, E3). The harness session
  is `grove-fleet`; each grove a window. rmux target syntax accepts `%N` pane ids
  (`rmux-core/src/target_find/syntax.rs:34`).
- Focus arbitration is the pure `arbitrate` table in `src/tui/focus.rs` (E4); the leader-→`c`
  path that opens the capture modal is the template for leader-→`e`.

## Done when

- Pressing the leader then `e` while a **harness** pane is focused: leaves the alt screen,
  runs `$EDITOR <tmpfile>` where the tmpfile holds the output of
  `rmux capture-pane -p -S - -J -t %<paneid>` (full retained history + visible, soft-wrap
  joined, plain text — D-B), then restores the TUI and forces a full redraw. No-op (or a
  toast) on the bare `shell` pane / when no harness is focused.
- **Input-reader race handled (D-E):** the reader thread is paused for the duration of the
  editor so the child owns stdin; resumed and the screen fully repainted afterwards.
- **`rmux` binary resolved like the SDK daemon (D-D):** honour `SDK_DAEMON_BINARY_ENV` /
  the same resolution `connect_or_start` uses, not bare `rmux` — so dev + 060-bundled both
  hit the daemon grove started. (Confirm the exact env/const name the SDK uses when
  implementing; fall back to `rmux` on PATH only if unresolved.)
- **Tests:** the `capture-pane` argv builder is a pure function with a unit test
  (target id → correct flag vector, incl. `-S -` rendered as two args `["-S","-"]`). `$EDITOR`
  resolution (`$VISUAL`→`$EDITOR`→`vi`) unit-tested. The editor drop itself (suspend/restore)
  is exercised by hand / a thin integration test — the visual confirm is the user's
  (cf. memory: grove tui blank under headless tmux; leave the live visual check to the user).
- Capture errors (daemon down, empty history) surface as a toast, never crash the loop
  (mirror `submit_capture`'s `CaptureOutcome`).

## Notes

- Add `Action::OpenEditor` (or similar) to `src/tui/focus.rs` and the `arbitrate` table on the
  leader-→`e` transition from `Focus::Harness`; carry it out in `App::handle_event` like
  `Action::ModalSubmit`. The `e` token must only be intercepted *after* the leader (harness
  forwards everything else, E4).
- The editor flow is necessarily synchronous from the loop's view (the UI is fully
  suspended); run it with the input thread paused and the reactor free (`spawn_blocking` for
  the capture, blocking `.status()` for the editor child since nothing else should draw).
- Optional, fire-and-forget (D-F): file an upstream suggestion for a `Pane::capture_history()`
  SDK convenience. Not required for this leaf.
