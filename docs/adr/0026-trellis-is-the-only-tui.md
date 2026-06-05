# 26. trellis is the only TUI — local dashboard and `trellis-seam` gate removed

- Status: accepted
- Date: 2026-06-05
- Deciders: Antony Blakey (grove `grove-always-starts-in-local-mode`)
- Amends: ADR-0020 / ADR-0021 (trellis as the native, supported TUI) — by
  removing the `--local` escape hatch and the build-feature gate those left in
  place, and by deleting the now-obsolete `grove-nav` WASM plugin.

## Context

ADR-0020/0021 forked zellij into the [[trellis framework]] and made grove's
dashboard a native, in-process trellis surface. But the landing kept two pieces
of pre-fork scaffolding:

1. **A `trellis-seam` cargo feature**, *off by default*, gating the `trellis`
   crate dependencies. The intent was to keep a default `cargo build` fast while
   the native host API was built out (ADR-0021 deferred flipping it always-on to
   "a later 110 leaf").
2. **A `grove tui --local` flag** selecting the legacy in-terminal ratatui
   dashboard (`tui::run`) — the pre-fork rendering, with no trellis and no
   embedding — as a dev/debug escape.

This combination shipped a bug the grove's own name records. Nothing in the
release path (`release.toml`, `release-build.sh`) passes `--features
trellis-seam`, so the **shipped binary built with the feature off** and therefore
took the `#[cfg(not(feature = "trellis-seam"))]` branch: `grove tui` → `tui::run`
→ the **single-repo local dashboard that ignores `~/.config/grove/fleet.toml`**.
The entire v6 fleet view was unreachable in the released artifact. An off-by-
default feature whose "on" path is the only supported one is a latent foot-gun:
the default *is* the accident.

Separately, the `crates/grove-nav` WASM plugin (ADR-0018) was already superseded
by the native nav surface (ADR-0020, leaf `120-native-nav`): nothing
`include_bytes!`es it anymore and `GROVE_NAV_WASM` is consumed nowhere. It
survived only as a crate plus a `build.rs` step that compiled it on every build —
dead weight that *also* broke `cargo build` inside a nested grove worktree (cargo
binds the workspace-excluded crate to the parent repo's workspace).

`grove tui` is convenience-only — real work is `grove do`, and the
non-interactive view is `grove status` — so no in-terminal fallback is owed. The
trellis fork owes upstream nothing, so reshaping the build to grove's single
use-case is in-bounds.

## Decision

**trellis is the one, unconditional TUI.**

1. **Remove the `trellis-seam` feature.** The `trellis`, `trellis-client`, and
   `trellis-server` crates become unconditional (non-optional) dependencies, so a
   default `cargo build` is trellis-capable and surfaces `fleet.toml`. No
   `#[cfg(feature = "trellis-seam")]` / `#[cfg(not(...))]` remains anywhere.
2. **Delete the local dashboard.** `tui::run` (the standalone crossterm event
   loop) and everything that became dead with it (`live_event_loop`,
   `process_pending_action`, `suspended`, the tty `$EDITOR` helpers, and the
   `WatchSet` fs-watch debounce the loop polled) are removed. The reusable,
   transport-agnostic core — `App`, `render`, `handle_key`, the shell-out
   writers — is kept; it is what the native host surface (`dashboard_surface`)
   already drives.
3. **Remove the `--local` flag.** `TuiArgs.local` is gone; `grove tui --local`
   is now an unknown-flag error. The `tui` dispatch reduces to
   `trellis_host::run_client(&repo_args)` and the `--server` re-exec intercept
   becomes unconditional.
4. **Delete the obsolete `grove-nav` WASM plugin** — the `crates/grove-nav`
   crate, the root `build.rs` that compiled it, the `build =` line, the workspace
   `exclude` entry, and the `wasm32-wasip1` release-doctor prerequisite. The
   `"grove-nav"` name survives only as the *native* nav pane in the layout.

## Consequences

- **The fleet view actually ships.** A default build links trellis and reads
  `fleet.toml`; there is no off-path to fall into.
- **One coherent binary, one rendering path.** The cost is build time and binary
  size — every build compiles the ~100k-LOC forked server (terminal emulation,
  PTY, the wasmi plugin host). Accepted deliberately for a single always-trellis
  binary; `grove tui` is convenience-only, so the lost in-terminal fallback costs
  nothing real.
- **No `wasm32-wasip1` toolchain needed to build grove**, and `cargo build`
  works inside nested worktrees again.
- **The `native_chrome=false` rendering is now test-only.** Every production
  surface sets `native_chrome=true`; the footer-drawing, whichkey-less path
  survives purely as a unit-test fixture of the `App` core.
- **Trellis dead-code warnings (`ASYNC_RUNTIME`/`async_runtime`) are now visible**
  in every build, since the fork always compiles. They are the fork's concern,
  not grove's, and are left to the trellis maintenance surface.
