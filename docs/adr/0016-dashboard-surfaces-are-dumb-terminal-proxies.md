# 16. Dashboard surfaces are dumb-terminal proxies to one controlling process

- Status: **superseded by [ADR-0028](0028-rmux-substrate.md)** (rmux substrate,
  2026-06-10, 070-teardown D4) — the dumb-terminal-proxy / controlling-process /
  socket-seam model dissolves: grove draws its own surfaces as ratatui widgets,
  with no controller↔proxy wire. Premise + mechanism gone.
- Date: 2026-05-31
- Deciders: Antony Blakey (with grove 060 design)
- Refines: ADR-0015 (grove-owned zellij substrate)

## Context

ADR-0015 adopted a grove-owned zellij substrate with a native-pane dashboard
(Strategy 1b). In the 060/020 spike that dashboard pane ran a *standalone*
`grove tui` — its own process, doing its own fs-watch and data access. That makes
the dashboard a full grove instance, not a thin surface.

The required model is the inverse: **every dashboard surface must be only a proxy
to the single controlling process that runs the zellij instance; all functionality
lives in that one process.** Drivers:

- **Single source of truth.** One process owns all state — the multi-repo fleet,
  `notify`/fs-watch streams, shell-out-to-`grove` writes, and the decisions about
  which panes to open/focus. No state duplicated or diverging across surfaces.
- **Uniform presentations.** "component(s)" is plural: there may be several
  dashboard surfaces. They must all be the *same kind* of thin client, so adding
  or switching a surface never relocates logic.
- **Realises ADR-0013 concretely.** ADR-0013 kept core logic below a presentation
  *seam* (module placement) so a second presentation was possible. This turns that
  seam into a runtime **client/server split**: a future web front-end becomes
  another proxy to the same controller, not a parallel logic surface.

A design fork was resolved with the user: *where does rendering run?* Chosen:
**the controller renders; the proxy is a dumb terminal** (not "proxy renders from
pushed state"). The dashboard surface holds no ratatui and no view model.

## Decision

**One persistent controlling process** launches and owns the zellij instance — it
runs zellij as a child and persists for the instance's lifetime (it does *not*
`exec` zellij and vanish). It owns **all** grove functionality: the
`RepoView`/`MultiRepoView` data layer, fs-watch/`notify`, shell-out-to-`grove`
writes, the decisions about which harness panes to open/focus (driving the
substrate via `zellij action`), **and the ratatui rendering of every dashboard
surface**.

**Every dashboard surface is a dumb-terminal proxy** — a thin client (working
name `grove __dash-proxy`) running in a zellij pane that only:

1. reports its terminal size to the controller (and on every SIGWINCH),
2. writes controller-sent output bytes to its stdout, and
3. forwards its stdin (key/mouse) up to the controller.

It contains **no grove state, no business logic, and no ratatui.** The controller
keeps a per-proxy render target sized to that proxy and ships ratatui's rendered
output (diffs) down a local IPC channel (unix domain socket assumed; exact wire
format is a 040 detail). **Input flows up, display flows down.** The controller —
never the proxy — decides what a keypress does and what panes to open or focus.

This governs **all** presentations uniformly: the 1b native-pane dashboard, the
recorded-future 1a WASM plugin, and a future web client are each dumb proxies to
the same controller.

## Consequences

- **Refines ADR-0015's 1b.** The dashboard pane runs the proxy client, not a
  standalone `grove tui`; v1's `grove tui` dashboard logic+rendering moves into the
  controller. `zellij action` driving is the controller's job (it owns the
  decisions), not the pane's.
- **Process model for 040:** controller launches zellij as a child and serves
  proxies until exit; build the dumb proxy client + the IPC protocol (size up,
  frames down, input up) + per-client sizing/SIGWINCH + a ratatui backend that
  renders to the per-proxy channel.
- **Web front-end (ADR-0013, deferred) gets cheap.** It becomes a proxy speaking
  the same protocol (or rendering the controller's frames) — no logic relocation.
- **Cost accepted:** a render-to-IPC backend, input forwarding, and per-client
  sizing, in exchange for single-source-of-truth and uniform clients.
- **Boundary (ADR-0013) reinforced:** the proxy is pure presentation transport;
  all core stays in the controller.
- **Symmetry note:** this inverts the shelved `harness-pane` embed — there grove
  *emulated* someone else's terminal app; here grove's controller *is* the app and
  the zellij pane is the terminal relay.

## Notes

- IPC transport = unix domain socket (assumed); frames = ratatui diff output;
  precise wire format and the proxy launch wiring are 040 build details.
- The harness panes themselves are unaffected: they remain native zellij panes
  running `grove do <name>` (zellij emulates them). This ADR is about the
  *dashboard* surfaces, not the harnesses.
