# 120-native-nav

**Kind:** work

## Goal

Build grove's **home nav** as a **native in-process surface** (no WASM plugin): the
full-height grove list that *is* the home tab, the [[leader]]-focusable command
surface, which **opens and switches grove [[workspace]] tabs by direct in-process
call**. This is the native realisation of the nav — superseding the WASM nav (070),
the deleted back-channel (080), and the old plugin "self-opens" leaf.

## Context

- **ADR-0020** removes every reason the nav had to be a WASM plugin: a keybind no
  longer needs `LaunchOrFocusPlugin` to reach a focusable surface, and there is no
  out-of-process controller to signal — the nav *is* in the process and calls the
  trellis host API (110) directly. The whole ADR-0018/0019 plugin/permission/
  `cli_pipe_output` apparatus evaporates.
- **The UX model survives** (ADR-0018/0019): grove = tab; home = nav full-height;
  leader (`Ctrl-o`) focuses the nav from any pane; selecting a grove opens its tab
  (first time) or switches to it (already open); global capture (`c`) is a nav key.
  Only the realisation changes (plugin → native call).
- **Command + cwd composition stays in Rust** (it always did); now it is just a
  direct call into the host API to open a tab running `grove do <name>` in the
  grove's cwd — no piped `grove-state`, no layout-as-data round-trip.

## Done when

- The home tab is the native nav (full-height grove list); `Ctrl-o` focuses it from
  any pane via a direct in-process focus call.
- Selecting a **closed** grove opens its [[workspace]] tab (harness running
  `grove do <name>` in the grove's cwd) and focuses it; selecting an **open** grove
  switches to it; no duplicate tabs; no permission prompt anywhere.
- Hints use **sigils** (`⏎`/`⎋`/…) — but the *bar* itself is 140's job; here the
  nav just exposes its keys.
- `cargo build`/`cargo test` green.

## Notes

- Scope: the grove tab opened here can be **harness-only**; native detail is 130,
  the rest of the [[working set]] is 150. Open it with a layout easy to extend.
- Keep grove core `ratatui`-free below the seam; nav rendering is above it.
- Depends on **110** (the native host API the nav drives).
