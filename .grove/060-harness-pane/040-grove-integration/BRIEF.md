# 040-grove-integration — brief

**Kind:** node (build, reshaped by 020 onto the [[zellij substrate]], ADR-0015;
dashboard architecture per ADR-0016). Decomposed into ordered build leaves
because the integration is a 3–4 session build, not one focused commit.

## Goal

Wire grove onto its **owned zellij substrate** (Strategy 1b) under the
**controlling-process / dumb-proxy** model (ADR-0016): a persistent **controlling
process** launches zellij (as a child) with grove's bundled config + bars-free
layout (presenting as a single binary) and owns *all* logic, state, and rendering;
the **dashboard** appears in a zellij pane as a [[dashboard proxy]] (dumb terminal)
that the controller renders into; and from the dashboard the user
**launches/attaches a grove's live harness** (`grove do <name>` → claude code /
codex) as a native zellij pane and **switches focus between groves**. This closes
the 060 "Done when" — via zellij, with logic centralised in the controller.

## Context

- **Substrate decided (020 → ADR-0015): grove-owned zellij.** Harness panes are
  native zellij panes; grove does *not* emulate terminals. The in-process-pty
  `harness-pane` crate is the shelved fallback, not consumed here.
- **Dashboard architecture (ADR-0016): controlling process + dumb proxies.** The
  controller renders every dashboard surface and ships frames over a local IPC
  channel (unix socket) to a thin `grove __dash-proxy` running in the pane; the
  proxy only reports its size (+ SIGWINCH), blits controller output to stdout, and
  forwards stdin up. The proxy holds **no** state/logic/ratatui.
- **Boundary (ADR-0013) reinforced (ADR-0016):** all of grove — data layer,
  writes, *and* dashboard ratatui rendering — lives in the controlling process;
  the dashboard proxy is pure presentation transport. The seam becomes a runtime
  client/server split, and a future web client is just another proxy.
- **Free from the substrate (do NOT rebuild):** per-pane copy mode, scrollback,
  search, floating panes, session persistence — all native zellij. (This is why
  030 was retired.)
- **Tamed config knobs (validated on zellij 0.44.3, from the 020 spike):**
  `default_mode "locked"`, `pane_frames false`, `simplified_ui true`,
  `show_release_notes false`, `show_startup_tips false`, `copy_on_select true`,
  `session_serialization true`; a custom top-level `layout { … }` with **no**
  tab/status-bar panes; command panes need `start_suspended false`. The throwaway
  spike at `/tmp/grove-zellij-spike/` is the starting reference.

## Decisions carried into the build (settled when this node was created)

- **Render-over-socket = `CrosstermBackend` over a socket writer.** ratatui already
  renders to any `Backend`/`io::Write`; the controller holds a
  `Terminal<CrosstermBackend<W>>` per proxy where `W` writes framed bytes down the
  socket. The ANSI escape stream *is* the "frames down" wire payload — so the proxy
  stays genuinely dumb (copy socket→stdout). ratatui's cell-diffing already
  minimises the bytes.
- **Input decoder is hand-rolled, no new dep.** crossterm's `event::read()` is
  hard-wired to the process's own stdin/tty and cannot be pointed at a socket, so
  the controller must decode the proxy's raw stdin bytes into
  `KeyCode`/`KeyModifiers` itself. A focused ANSI input parser covering the
  dashboard's key subset (arrows, Enter/Esc/Tab/Backspace, control chars, UTF-8
  chars) is small and aligns with grove's lean-dependency value (ADR-0013). Mouse
  may be deferred for the within-repo cut.
- **Bundled config/layout: embed in the binary, write to a cache dir at launch**
  (e.g. `$XDG_CACHE_HOME/grove/zellij/` or `~/.cache/grove/zellij/`). Keeps the
  single-binary presentation; no reliance on a user-edited config path.
- **Packaging: depend on an installed zellij** for the within-repo cut (task note);
  a bundle/vendor decision can come later (flag if it needs its own leaf/ADR).
- **Unlock key (zellij control seam): `Ctrl-o`** — deliberate, documented,
  low-collision (default `Ctrl-g` collides with nvim "show file info"; user steer).
- **One dashboard proxy for v1**, but the protocol supports N proxies (ADR-0016's
  plural "component(s)"; a future web client is another proxy).

## Done when (acceptance for the whole node)

- `grove`/`grove tui` launches the zellij substrate with grove's bundled config +
  layout and **presents as a single binary** (no visible "you are in zellij":
  bars/frames/branding hidden; dashboard auto-runs in its pane).
- **The dashboard pane is a dumb proxy** (`grove __dash-proxy`): the controlling
  process renders the dashboard and ships frames to it; the proxy carries no grove
  state/logic/ratatui. Resize (SIGWINCH) and input round-trip correctly.
- From the dashboard, selecting a grove makes the **controller open `grove do
  <name>` as a native zellij pane** beside the dashboard (`zellij action
  new-pane`), interactable normally (locked-mode passthrough).
- **Switching focus** between groves works from the dashboard via stable pane-ID
  addressing (`focus-pane-id`); the dashboard remains reachable as the switch
  surface.
- Multiple groves' harnesses can be alive at once; closing one is clean
  (`close-tab-by-id` / `close-pane`).
- The unlock key is `Ctrl-o`, documented.

## Decomposition (this node)

```
010-proxy-protocol   IPC protocol + dumb `grove __dash-proxy` client + input
                     decoder + render-over-socket backend. The controller↔proxy
                     seam, tested against a controller stub. The hard core.
020-controller-loop  Refactor the dashboard event loop into the controlling
                     process: render the existing App to the proxy, feed decoded
                     input to handle_key, run fs-watch + shell-out over the seam.
030-zellij-launch    Head binary: embed config+layout assets, write to cache dir,
                     launch zellij as a child, place the dashboard proxy pane,
                     present-as-single-binary; wire the Ctrl-o unlock key.
040-harness-driving  Controller drives zellij via `zellij action`: open `grove do
                     <name>` panes, focus/switch, close; track stable pane IDs;
                     wire dashboard select→open. Driving layer must not hard-assume
                     one repo (070 reuses it cross-repo).
```

## Notes

- Scope is **within one repo** (the v1 dashboard already lists a repo's groves).
  **Cross-repo** fleet is 070 — but it opens cross-repo harness panes via the same
  `zellij action` driving, so 040's driving layer (leaf 040) must not hard-assume
  one repo.
- Launching `grove do <name>` inside a zellij pane nests grove deliberately (the
  harness *is* a grove session) — intended, not a problem.
- 1a (WASM plugin dashboard via `zellij_widgets` + pipe IPC) stays a recorded
  future refinement; do not pull it into this node (ADR-0015, grove constraint 4).
