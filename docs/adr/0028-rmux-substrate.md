# 28. The TUI substrate is rmux: grove owns its draw loop and embeds panes (inversion)

- Status: **proposed — skeleton, finalized in 030-engine/040** (drafted in
  010-draw-loop-pane per D4: the landmark ADR is authored *within* the engine
  leaves, not as a separate leaf)
- Date: 2026-06-08
- Deciders: Antony Blakey (with grove `rmux-substrate` 010-plan + 030-engine)
- Supersedes / will mark dissolved (finalized in 040 once the engine lands): the
  trellis zellij-fork tower — esp. ADR-0015 (owned zellij multiplexer), ADR-0020
  (fork zellij into grove's own framework), ADR-0021 (trellis hosting API),
  ADR-0026 (trellis is the only TUI). The **UX model** those ADRs settled
  survives; only their zellij-fork *realisation* is replaced.
- Builds on / amends: ADR-0013 (presentation boundary — promoted to a literal
  directory wall, see E2; its async-as-web-toll prediction is amended, see E1)
  and ADR-0027 (singleton fleet session name).

## Context

trellis compiled grove *into* a forked zellij server, so grove's own UI lived
inside zellij's pane/buffer model — the root cause of the capture-popup bug and of
the whole ADR-0016–0028 host-surface / host-driver / ScreenInstruction tower, and
it was un-verifiable headlessly. We replace that substrate with **rmux**
(rmux.io, MIT/Apache, v0.5.0): a separate daemon owns the ptys; grove becomes a
plain **ratatui app that owns its own draw loop**, embedding foreign terminal
programs as panes via `ratatui-rmux` (`PaneWidget` over a `PaneSnapshot`) and
driving/observing them with the async `rmux-sdk`.

## The inversion thesis (landmark)

Under trellis the multiplexer owned the loop and grove's surfaces were guests
inside it. **rmux inverts that:** grove owns the loop, and only *foreign* terminal
programs (the harness; later yazi/lazygit/shell) are rmux panes. grove's own
surfaces — nav, per-grove detail, the capture modal, whichkey — are **ordinary
ratatui widgets grove draws itself**. A centered floating modal over live content
becomes trivial (it is just `Clear` + a widget over the pane), and verification
becomes plain headless tests (snapshot → `PaneWidget` → `Buffer`), because the
render path no longer needs a running multiplexer. This reframing is the durable
decision; the specific crates are implementation.

## Decisions settled so far (030-engine grilling — full text in the node brief)

**E1 — Async is confined above the presentation seam.** grove is otherwise 100%
synchronous; the rmux SDK is wholly async and the draw loop is a
`tokio::select!`. Async enters now, but *only* in the TUI module; the sync core
(`RepoView::scan`, shell-out writes, fleet resolution) is called directly from
async context (it is fast, local fs/git), with `spawn_blocking` reserved for
anything slow enough to stall the reactor. The entry builds a multi-threaded
runtime. **This amends ADR-0013's prediction** that async is "the entry toll for
web, paid only if/when web is chosen" — rmux brings async forward now, but the
presentation boundary contains it (the seam doubles as a runtime firewall).

**E2 — The presentation boundary is a literal directory: `src/tui/`.** No
separate `grove-tui` crate. Code under `src/tui/` may import
`ratatui`/`ratatui_rmux`/`rmux_sdk`/`tokio`; code outside it may not. This makes
ADR-0013's "boundary enforced by module placement and review" a directory wall
(a review-time guard, not a compile-time one — the trade-off accepted over a
core/tui lib split that ADR-0013 explicitly declined while no second consumer
exists).

**E3 — One rmux session per `grove tui`; only foreign programs are panes; the
harness pane runs `grove do <name>`; panes are addressed by stable `PaneId`.**
The minimal engine (010) is exactly one session, one pane (the harness), with
nav + capture drawn around/over it. The harness pane runs **`grove do <name>`**
(cwd = worktree), keeping `grove do` the single lifecycle entry point; addressing
uses a stable `PaneId` with a `grove-name → PaneId` map, not a positional slot,
so 030's dynamic open/close/park has stable handles.

## To be finalized in 040

E4 (focus model: leader-gated `Alt-g`, `Harness | Nav | Modal` state machine),
E5 (input coverage), E6 (dependency staging: published 0.5.0 now → rendered-history
fork in 040), the rendered-history capture (D7), and the explicit
mark-dissolved list for the superseded ADR tower (D4).

## Consequences

- The capture-popup bug class is dissolved by construction (grove's modals are
  native widgets, not zellij floating panes).
- The render path is headlessly testable (the migration's whole point).
- grove now depends on the rmux daemon (`connect_or_start` spawns it); bundling
  the daemon binary is 050/060's job. 040 switches the dep to the
  rendered-history fork.
- `crates/trellis/` and `crates/harness-pane/` (removed in 020-rip-out) stay
  gone; the ADR-0013–0028 tower dissolves per D4.
