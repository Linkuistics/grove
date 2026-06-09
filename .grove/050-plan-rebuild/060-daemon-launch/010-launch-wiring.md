# 010-launch-wiring

**Kind:** work

## Goal

Make `grove tui` find and use the bundled rmux daemon with zero config: resolve the
`rmux` binary shipped beside grove's own executable and point both the SDK
daemon-spawn and the ADR-0029 `capture-pane` shell-out at it via
`RMUX_SDK_DAEMON_BINARY`. Implements ADR-0030 §3.

## Context

Both `Rmux::connect_or_start` (daemon-spawn) and `src/tui/editor.rs::rmux_binary`
(`capture-pane`) resolve one binary the same way: `$RMUX_SDK_DAEMON_BINARY` else bare
`rmux` on `PATH` (`rmux-sdk-0.5.0/src/handles/rmux/connect.rs:177`, `editor.rs:73`).
Setting that one var once at startup redirects both. The bundled `rmux` is installed
into the same `bin/` as `grove`, so it is `current_exe().parent()/rmux`.

`run_app` (`src/tui/app.rs:182`) is already async — the input-reader thread and the
tokio runtime are up by the time it runs. Rust 2024 makes `std::env::set_var` unsound
once threads exist, so the resolve-and-set must run **earlier**, in the synchronous
`grove tui` entry that builds the runtime (find where `run_app` is spawned — the
`grove tui` subcommand handler in `src/cli`/`main`).

## Done when

- A pure-ish resolver picks the daemon binary with precedence **user override
  (`RMUX_SDK_DAEMON_BINARY` already set) → sibling-of-exe (`current_exe().parent()/
  rmux`, only if the file exists) → leave unset so the SDK falls back to bare `rmux`
  on `PATH`**. The pure part (given current-exe dir + an "already set?" flag + an
  existence check → the path to set, or none) is headlessly unit-tested; the
  effectful `current_exe()` / `set_var` wrapper is thin.
- The set happens in the **synchronous entry, before the tokio runtime + reader
  thread spawn** (not inside `run_app`'s async body). Verify no `set_var` runs after
  threads exist.
- First-run failure is friendly: if `connect_or_start` errors because no `rmux`
  resolves anywhere, surface "rmux not found — it ships with grove; reinstall, or set
  `RMUX_SDK_DAEMON_BINARY`" rather than the raw transport error. (Map the error at the
  `run_app` `connect_or_start` `.context(...)` site, or just before.)
- Works against a dev `rmux` on `PATH` today (no bundled binary yet — that is 020);
  `cargo test` green; a manual `grove tui` against a `PATH` rmux still launches.

## Notes

- Respect a user-set var verbatim (the testing escape hatch — ADR-0030 §3).
- Setting the var process-wide is intended: a nested `grove`/`grove-llm` child
  inherits the same bundled daemon.
- No new dependency; this is pure std (`env`, `current_exe`).
