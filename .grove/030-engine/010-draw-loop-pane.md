# 010-draw-loop-pane

**Kind:** work

## Goal

Stand up the engine core: `grove tui` opens an async ratatui app that owns its own draw
loop and renders **one live harness pane** embedded via `ratatui-rmux` `PaneWidget`
(read-only this leaf — input lands in 020). The first time a harness renders *inside
grove's own loop* rather than inside a zellij fork.

Build:
- The `src/tui/` module tree (E2): `tui::app` (the loop), `tui::pane`/`tui::driver`
  (per-pane rmux glue). Add the async runtime + rmux deps to `Cargo.toml`
  (`rmux-sdk = "0.5"`, `ratatui-rmux = "0.5"`, `tokio` multi-thread — E1/E6, **published
  crate**, not the fork).
- The `grove tui` entry: a multi-threaded `#[tokio::main]`-style runtime (E1), wired
  into `src/cli.rs`'s `Command::Tui` arm (replacing the disabled stub at `cli.rs:317`).
- `Rmux::builder().connect_or_start()`; `ensure_session(CreateOrReuse, detached, sized)`
  named deterministically (E3); open **one** harness pane running `grove do <name>`
  (cwd = worktree), held by stable `PaneId` in a `grove-name → PaneId` map (E3).
- The **D3 event-driven loop**: a `PaneDriver` per visible pane fed by a `render_stream()`
  task; `tokio::select!` over {per-pane `RenderUpdate`, crossterm input events, fs-watch
  ticks}; redraw only when dirty. Render via `PaneState::from_snapshot` → `PaneWidget`,
  placing the **hardware cursor** from `snapshot.cursor` (read before `from_snapshot`
  consumes it — see the spike). Resize forwarded to the pane.

## Context

Productionises `~/Development/rmux-spike/src/interactive.rs`, but replaces its
pull-per-frame `pane.snapshot()` loop with the D3 push model (010-plan D3). Input
handling beyond resize is deliberately **out of scope here** (020); this leaf forwards
nothing — it proves the *render* path. The sync core stays below the seam (E1): any
`RepoView`/launch call is a direct sync call from async context.

The bootstrap of crossterm raw-mode / alt-screen / mouse-capture + clean teardown
(the spike's `main`) is part of this leaf. Keep the loop **headless-testable**: the
render path (snapshot → `PaneState` → buffer) must be exercisable in a unit test with no
real terminal (the spike's probes ran headless — that testability is the migration's
whole point).

Begin **drafting the landmark "rmux substrate" ADR** here (D4) as the loop/daemon/pane
architecture and the E1 async-firewall decision settle — skeleton + the inversion thesis
+ E1/E2/E3; 040 finalizes it.

## Done when

- `grove tui` opens, connects/starts the daemon, and renders a **live** harness pane
  (run something inside it — output streams, cursor tracks) via the D3 push loop.
- `src/tui/` exists with the async entry wired into `cli.rs`; `cargo build`/`cargo test`
  green; below-seam code imports no `tokio`/`ratatui`/`rmux`.
- A headless unit test exercises snapshot → `PaneWidget` render into a `Buffer`.
- Landmark ADR skeleton committed.

## Notes

Watch: `connect_or_start` spawns whatever daemon is on PATH / `SDK_DAEMON_BINARY_ENV`
(this leaf uses the published daemon; bundling is 050/060). The fs-watch arm reuses the
existing `notify`-based helpers in `src/fleet.rs` (one watcher) but is consumed by the
async loop — keep the watcher below the seam, surface events into the `select!` via a
channel. Don't over-build nav/multi-pane: exactly one pane this leaf.
</content>
