# 020-controller-loop

**Kind:** work

## Goal

Move the existing `grove tui` dashboard (App state, render, `handle_key`,
fs-watch, shell-out writes) into the **controlling process**, driving it over the
010 proxy seam instead of the local crossterm tty: render the App to a per-proxy
`Terminal<CrosstermBackend<socket>>`, feed decoded input frames to `handle_key`,
and keep the `notify` fs-watch + `grove-llm` shell-out behaviour. End state: a
controller process serving one connected `grove __dash-proxy` *is* the dashboard —
functionally identical to today's `grove tui`, but rendered remotely.

## Context

- Today (`src/tui.rs`): `run()` does `ratatui::init()` → `live_event_loop` over the
  local terminal; `event::poll`/`event::read` for keys; `WatchSet` for fs-watch;
  `process_pending_action` suspends the terminal to shell out to `$EDITOR`/
  `grove-llm`. All of that is the controller's job now — only the *transport*
  changes (proxy seam, not local tty).
- ADR-0016: the controller owns rendering + all logic; the proxy is dumb. One
  source of truth — `RepoView`/fs-watch/writes/pane-decisions all live here.
- The 010 seam supplies: a per-proxy socket render target (build a
  `Terminal<CrosstermBackend<W>>` over it), an input stream of decoded
  `KeyCode`/`KeyModifiers`, and resize events (re-size the proxy's `Terminal`).
- Per-proxy sizing: each proxy reports its size; the controller keeps that proxy's
  `Terminal` sized to it and re-renders on resize (SIGWINCH-driven from the proxy).

## Done when

- The controller runs an event loop that: accepts a proxy connection, builds its
  render target, and on each tick renders `App` to it and applies queued input/
  resize frames — preserving the current debounced fs-watch rescan and the
  pending-action shell-out flow.
- Shell-out actions (`$EDITOR`, `grove-llm inbox-*`) still work. The terminal
  suspend/restore around them is reconsidered for the proxy world: the editor must
  attach to the proxy's *real* tty, so either the controller hands the proxy a
  "run this command on your tty" instruction, or capture/edit drops are routed
  appropriately. Decide and implement; document the choice. (This is the one place
  the dumb-proxy model meets interactive child processes — flag to an ADR if the
  resolution is non-obvious.)
- `handle_key` and `render` are reused essentially unchanged (the seam emits the
  same crossterm key types) — churn confined to the loop/transport, not the UI.
- The existing `TestBackend` snapshot tests still pass (render is untouched).

## Notes

- Keep `App`, `render`, `handle_key`, `RepoView`, `WatchSet` where they are; this
  leaf rewires the *driver* (`run`/`live_event_loop`) to the seam.
- The interactive-child-on-proxy-tty question (capture modal's `$EDITOR` drop) is
  the subtle risk here — surface it early; it may want its own small ADR.
- Don't launch zellij yet (030) and don't drive harness panes yet (040). This leaf
  can be exercised by manually running `grove __dash-proxy` against the controller
  outside zellij.
