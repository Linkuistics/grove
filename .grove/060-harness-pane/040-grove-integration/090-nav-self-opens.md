# 090-nav-self-opens

**Kind:** work

## Goal

Make the [[nav plugin]] open **and** switch [[workspace]]s **itself** (ADR-0019),
and **delete the 080 back-channel**. Selecting a grove in the nav switches to its
tab if open (`switch_tab_to`) or first-opens its tab if closed
(`new_tabs_with_layout` / `open_command_pane_in_new_tab`) — no round-trip to the
controller. This replaces the 080 `cli_pipe_output` intent path, which the live
investigation proved unworkable (reply-only; can't push from a keypress).

## Context

- **ADR-0019** is the spine. Confirmed live this session: keys reach the focused
  nav in locked mode; the controller→nav forward pipe works; a plugin can open
  tabs (`open_command_pane_in_new_tab(CommandToRun, ctx) -> (tab_id, pane_id)`,
  `new_tabs_with_layout(layout) -> tab_ids`).
- **Controller → nav (forward only):** extend the `grove-state` push so each grove
  carries `name`, `inbox_pending`, **cwd**, and the **exact `grove do <name>`
  command** (composed in Rust — composition stays in Rust, just transmitted). The
  nav drops those into a layout / `CommandToRun` to first-open.
- **Delete:** `src/nav_pipe.rs` (`IntentReader`, the return-path codec), the
  controller's intent drain + the channel wiring in `tui.rs`, the plugin's
  `intent_pipe` / `cli_pipe_output` / `ReadCliPipes`. Recoverable from history
  (the 080 commit). The forward `grove-state` push *stays* (it works) but its
  payload grows (cwd + command).
- **Permission/grant UX:** opening a tab needs a zellij permission
  (`OpenTerminalsOrPlugins` and/or `RunCommands`). To avoid a first-run permission
  prompt inside grove's chrome-less locked config, **grove pre-writes the grant
  into zellij's `permissions.kdl`** at launch (it owns the bundled plugin's cache
  path). Prove this out here — a fresh launch must reach a working nav with **no**
  permission prompt.

## Done when

- Selecting a **closed** grove in the nav opens its [[workspace]] tab (harness
  running `grove do <name>` in the grove's cwd) and focuses it — driven entirely
  by the plugin, no controller intent channel.
- Selecting an **open** grove switches to its tab; `h` jumps home; no duplicate
  tabs.
- The 080 back-channel code is gone; `grove-state` carries cwd + command; the
  build is green and the bundled config still passes `zellij setup --check`.
- A fresh `grove tui` launch grants the nav's tab-open permission with no prompt
  (pre-seeded `permissions.kdl`).
- Nav hint footer uses **sigils** (`⏎`/`⎋`/…) not words (a small UX item from the
  same live-test feedback).

## Notes

- Scope: the grove tab here can be **harness-only** (the [[detail proxy]] pane is
  the next leaf, 100; the rest of the [[working set]] is 120). Open it with a
  layout that's easy to extend pane-by-pane.
- Keep grove **core** `ratatui`-free below the [[presentation boundary]]; the
  command/cwd composition is controller-side, transmitted as data.
- The single grant write must be idempotent and not clobber a user's other plugin
  grants in `permissions.kdl`.
