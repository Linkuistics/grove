# 21. trellis hosting-API is library-you-link (grove owns `main`)

- Status: **superseded by [ADR-0028](0028-rmux-substrate.md)** (rmux substrate,
  2026-06-10, 070-teardown D4) — there is no trellis hosting API to link: grove
  owns `main` and its own ratatui draw loop, and rmux (a separate daemon) owns
  the ptys. Premise + mechanism gone.
- Date: 2026-06-03
- Deciders: Antony Blakey (with grove 060/040/100/030 spike)
- Builds on: ADR-0020 (deep hard fork; this resolves the hosting-API shape that
  ADR-0020 §Notes deliberately left build-discovered)

## Context

ADR-0020 forked zellij into the [[trellis framework]] but **deferred the
hosting-API shape** to a build-discovery spike against the real vendored
internals (per this tree's precedent — leaf 080 surfaced the `cli_pipe_output`
constraint empirically). The two candidate shapes:

- **library-you-link** — grove owns `main`, *links* the framework crates, and
  calls a `run(app)`-style entry. ADR-0020 flagged this as *preferred* but
  empirical.
- **runtime-you-plug-into** — trellis owns `main`; grove registers itself
  (a callback/app/plugin) and trellis drives it.

This ADR records the spike's finding. The spike was a **code-level trace of
both directions through the real zellij 0.44.3 source** vendored at
`crates/trellis/` — no throwaway demo was needed because the decision is forced
by a structural fact, and zellij's own binary is a compiled existence proof of
the winning shape (see Evidence #5).

## Evidence (vendored zellij 0.44.3 internals)

1. **zellij is already library-structured.** `crates/trellis/src/main.rs` is a
   thin CLI dispatcher; the real work lives in `pub fn
   zellij_client::start_client(...)` and `pub fn zellij_server::start_server(...)`.
   Both accept an **injectable `Box<dyn ClientOsApi>` / `Box<dyn ServerOsApi>`** —
   the OS/terminal seam a host substitutes at.

2. **The server is a re-exec'd separate process** *(load-bearing)*.
   `spawn_server()` (zellij-client/src/lib.rs:353) runs
   `Command::new(current_exe()).arg("--server")`; the server double-forks /
   daemonizes and talks to the client over a unix socket. **The same binary
   plays both roles** — foreground client and re-exec'd server daemon.

3. **Rendering is server-side; the client is a thin blitter.** The client's main
   loop receives `ClientInstruction::Render(String)` and writes the bytes to
   stdout (zellij-client/src/lib.rs:1152). The server renders panes to abstract
   `CharacterChunk`s; the compositor turns them into the ANSI string the client
   blits. (This nuances ADR-0020's "in-process rendering": grove's surfaces
   render *inside the server daemon* — in-process to that process, the same
   binary, not to the foreground client.)

4. **Panes are a trait with exactly two kinds.** `PaneId::Terminal | Plugin`;
   `trait Pane` (zellij-server/src/tab/mod.rs:228) renders to
   `Vec<CharacterChunk>`. grove's native surfaces become a **third pane kind** —
   a native `Pane` impl producing `CharacterChunk`s from grove's ratatui buffer
   instead of from a pty or a WASM plugin.

5. **The only app-extensibility surface is WASM.** The plugin host is `wasmi`
   (zellij-server/src/plugins/plugin_loader.rs:15); there is **no native
   app-registration trait**. So "runtime-you-plug-into" can *only* mean a WASM
   plugin (the rejected ADR-0018 premise) or a bespoke native callback.

## Decision

**The trellis hosting API is library-you-link: grove owns `main`, links the
framework crates (`zellij-client` / `zellij-server` / `zellij-utils`), and
dispatches the client/server roles itself.**

- No `--server` arg → grove runs the **client** UI path.
- `--server <socket>` arg → grove runs the **trellis server**, augmented with
  grove's native surfaces.

This is exactly the shape zellij's own `main.rs` already implements — grove
replaces that thin dispatcher with its own, keeping role-dispatch but swapping
in grove's config/layout/panes.

**Why runtime-you-plug-into loses, forced by Evidence #2:** because the server
is re-exec'd from `current_exe()`, grove's pane-rendering code must be compiled
into the binary and re-dispatched on the `--server` path *regardless* of model
— grove's surfaces render server-side. A "trellis owns `main`, grove registers
an app, trellis calls `run()`" wrapper therefore either (a) registers grove as a
**WASM plugin** — the sandboxed, no-native-data, protobuf-host-call model
ADR-0020 forked to escape — or (b) registers a native callback that **does not
survive the re-exec into the daemon**, forcing grove to re-establish it on the
`--server` path anyway. The runtime wrapper buys nothing and re-introduces a
serialize-the-app-across-a-process-boundary indirection. The architecture itself
is the evidence; no demo can make the runtime model fit.

## The seam 110 builds the MVP host API on

1. **grove owns `main` and dispatches roles** (client vs `--server`), replacing
   `trellis/src/main.rs`'s dispatcher.
2. **grove substitutes/extends `ClientOsApi` / `ServerOsApi`** as the OS/terminal
   injection seam (already abstracted as trait objects).
3. **grove adds a native `Pane` impl** (the third pane kind) for its in-process
   surfaces — dashboard, nav, detail, whichkey — rendering ratatui output as
   `CharacterChunk`s server-side. This subsumes the ADR-0016 proxy seam
   (010/020) and the ADR-0015/040 `zellij action` driving: native calls replace
   the socket frames and the external CLI.

## Consequences

- **110's seam is concrete** (the three points above), not abstract.
- **The one-way crate seam (ADR-0020 §4) is honoured naturally:** grove depends
  on the `zellij-*` crates; they never depend on grove. The current
  `trellis = { package = "zellij-utils", ..., optional }` seam in grove's
  `Cargo.toml` widens to `zellij-client` + `zellij-server` when 110 makes it real.
- **grove's logic lives in the server daemon, not the foreground client.** Plan
  grove's state/render code as server-side; the client stays the thin blitter
  zellij already ships. (Watch the re-exec: `current_exe()` must remain the grove
  binary; anything grove needs in the server must be reachable on the `--server`
  path, not just set up pre-spawn in the client.)
- **No throwaway spike code retained** — consistent with the leaf's mandate
  ("the decision is the artifact"). The seam already compiles
  (`cargo build --features trellis-seam`, proven at leaf 010).

## Notes

- The **public brand** for the framework and the eventual `run(app)` signature's
  exact ergonomics are still deferred (ADR-0020 §Notes); this ADR fixes the
  *shape* (library-you-link, role-dispatch, native-pane seam), not the final API
  surface — that is 110's to design against real grove rendering.
