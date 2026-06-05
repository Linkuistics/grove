# 030-trellis-ignore-user-zellij-config

**Kind:** work

## Goal

Stop the trellis path from inheriting the user's zellij configuration. grove
embeds a vendored fork (trellis), not zellij; its TUI config must be
grove-owned only.

## Context

User steer: "We should not be reading the global zellij config. This is not
zellij." Today the trellis client resolves config via
`Setup::from_cli_args` → `find_default_config_dir()`
(`crates/trellis/zellij-utils/src/home.rs`), which sources
`$ZELLIJ_CONFIG_DIR` / `$XDG_CONFIG_HOME/zellij` / `~/.config/zellij/config.kdl`
as the **base**, then merges grove's `GROVE_TUI_CONFIG` on top
(`src/trellis_host.rs:171-179`, bundled KDL at `:241-256`).

Aligns with the project principle that the fork owes upstream nothing and may be
reshaped to grove's use-case (memory: `project_trellis_no_upstream_compat`).

## Done when

- The trellis config is built from trellis's built-in defaults + grove's
  bundled `GROVE_TUI_CONFIG` only — the user's `~/.config/zellij` is never
  sourced.
- `$ZELLIJ_CONFIG_DIR` / `$XDG_CONFIG_HOME/zellij` / user zellij themes &
  layouts are not read by grove's trellis path. (Confirm whether to neutralise
  at the grove call site in `trellis_host.rs` or deeper in the vendored
  `Setup`/`find_default_config_dir` — prefer the smallest change that fully
  severs the user-config source; a call-site override is cleaner than forking
  more of the vendored crate if it suffices.)
- A user with a populated `~/.config/zellij` sees identical grove behavior to a
  user with none.
- `cargo test` passes; add/adjust a test pinning "user zellij config is ignored"
  if practical.

## Notes

Independent of `020` but conceptually part of the same "grove owns its TUI"
story. The layout is already grove-owned (`GROVE_TUI_LAYOUT`, passed as a
string, `src/trellis_host.rs:211`) — this leaf closes the config-base gap.
