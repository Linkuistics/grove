# 010-boot-role-dispatch

**Kind:** work

## Goal

Make **grove own `main`** and dispatch the trellis client/server roles itself,
so that running grove brings up a **live trellis session from the grove binary** —
a stock zellij terminal pane, rendered by the re-exec'd server daemon, displayed
by grove's own client path. This is ADR-0021's seam point (1) and the foundation
the native pane (020) and dashboard port (030) stack on. No grove surfaces yet —
just prove the binary can play both roles.

## Context

- **ADR-0021 (library-you-link).** grove links `zellij-client` / `zellij-server`
  / `zellij-utils` and dispatches roles: no `--server` arg → client UI path;
  `--server <socket>` → the trellis server. This replaces `trellis/src/main.rs`'s
  thin dispatcher with grove's own.
- **The re-exec is load-bearing (ADR-0021 Evidence #2).** `spawn_server`
  (zellij-client/src/lib.rs:353) runs `Command::new(current_exe()).arg("--server")`
  and daemonizes. So **`current_exe()` must be the grove binary** and grove must
  re-dispatch the `--server` path — anything the server needs must be reachable
  there, not just set up pre-spawn in the client.
- **Widen the seam.** Today `Cargo.toml` has `trellis = { package =
  "zellij-utils", optional }` behind `trellis-seam`. This leaf broadens it to add
  `zellij-client` + `zellij-server` path deps. Decide here whether `trellis-seam`
  stays a feature or becomes always-on (ADR-0021 says 110 makes the link
  non-optional — likely flip it on this leaf, but keep grove's non-TUI verbs
  building without dragging in the server if that proves cheap).
- **Entry points to reuse, not reinvent:** `zellij_client::start_client(...)`
  (lib.rs:720) and the server start path both take an injectable
  `Box<dyn ClientOsApi>` / `Box<dyn ServerOsApi>` — use the stock impls here;
  020 is where grove substitutes/extends them.
- Config/layout: bring up with a minimal grove-owned config + a bars-free single
  terminal-pane layout (the tamed knobs validated on 0.44.3 — locked mode, no
  bars — carry forward as the starting config; they are no longer chrome
  *gymnastics*, just our config now that we own the build).

## Done when

- `grove tui` (or the chosen entry) launches a working trellis session **from the
  grove binary itself** — a stock terminal pane the daemon renders and the client
  blits; no installed/external zellij, no `zellij::launch`.
- The `--server` re-exec resolves back to the grove binary (`current_exe()`) and
  grove's dispatcher routes it to the trellis server start path.
- `cargo build` green with the broadened seam; grove's existing CLI verbs still
  build and run. grove core stays `ratatui`-free below the ADR-0013 boundary.
- The superseded launch path (`zellij::launch`, the dumb proxy/controller entry)
  is left in place but no longer the live `grove tui` path (its removal can wait
  for 030, when the native dashboard fully replaces it — note what's now dead).

## Notes

- This is the spike-y foundation: expect to read zellij's `main.rs` /
  `commands.rs` / `start_client` / `spawn_server` closely and mirror the
  role-dispatch. The decision is "grove's `main` does what zellij's `main` did,
  swapping in grove's config" — keep it that thin.
- Don't build the native pane or input model here — that's 020. A stock terminal
  pane is the proof; if it shows a shell, the re-exec + role-dispatch works.
- Watch for grove's *other* binary (`grove-llm`) and the non-TUI verbs: linking
  the server crate must not bloat or break them. If the server link is heavy,
  consider gating the TUI entry so plain `grove status` etc. stay light.
