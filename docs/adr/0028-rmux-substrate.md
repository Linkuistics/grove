# 28. The TUI substrate is rmux: grove owns its draw loop and embeds panes (inversion)

- Status: **accepted** (drafted in 030-engine/010-draw-loop-pane, finalized in
  030-engine/040-capture-modal per D4: the landmark ADR is authored *within* the
  engine leaves, not as a separate leaf). Accepted now because the engine
  milestone is demonstrated end-to-end — a harness pane renders + takes input, a
  minimal nav lists/opens groves, and the **centered capture modal works over the
  live pane** (the motivating bug, fixed; verified headlessly).
- Date: 2026-06-08
- Deciders: Antony Blakey (with grove `rmux-substrate` 010-plan + 030-engine)
- Supersedes (at the thesis level — the per-ADR `Superseded` marking sweep is the
  050 teardown, D4): the trellis zellij-fork tower, esp. ADR-0015 (owned zellij
  multiplexer), ADR-0020 (fork zellij into grove's own framework), ADR-0021
  (trellis hosting API), ADR-0026 (trellis is the only TUI), and the
  ADR-0016–0024 host-surface / host-driver / ScreenInstruction sub-tower they
  anchor. The **UX model** those ADRs settled survives; only their zellij-fork
  *realisation* is replaced.
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

## Decisions (030-engine grilling — full text in the node brief)

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

**E4 — Focus is a leader-gated `Harness | Nav | Modal` state machine; leader =
`Alt-g` (configurable).** grove owns the loop, so there is no zellij locked-mode —
grove is the arbiter by construction and sees every crossterm key first.
Arbitration is **leader-gated**: while the harness is focused, grove forwards
*everything* to the pane except the single leader key (maximising harness key
fidelity — vim/claude lean on F-keys + Ctrl-chords). The transition table is a
**pure function** `(Focus, Leader, Event) → (Focus, Action)`, so the whole focus
model is headlessly unit-tested; the app applies the returned side-effecting
`Action`. **Harness:** forward all keys but the leader. **Nav:** grove handles
keys (navigate, select → open/focus a harness, open the modal). **Modal:** a focus
overlay capturing all keys, `Esc` cancels and `Enter` submits, both restoring the
prior focus. Leader `Alt-g` chosen for lowest collision across readline/vim/claude.

**E5 — Input coverage: extended crossterm→tmux key-map; bracketed paste forwarded
wrapped; mouse drives grove surfaces + click-to-focus.** The key-map covers the
modifier matrix on special keys (`C-Left`, `S-Up`, `C-Enter`, …). A multi-line
paste arrives as one `Event::Paste` forwarded **wrapped in `\e[200~…\e[201~`** so
it does not execute line-by-line (claude multi-line / vim paste-mode); in the
modal, paste inserts into the buffer literally. Mouse drives grove's own
nav/modal, with a basic left-click forwarded to the harness as focus/click. **Rich
mouse passthrough (drag/wheel/motion) is deferred to 050 and flagged as a likely
rmux-*fork* raw-mouse capability** — no lossy automation-call translator is built.

**E6 — Dependency staging: the engine builds on published `rmux 0.5.0`; the
rendered-history fork is a *separate, later* leaf.** The whole 030 engine — render,
input, session, nav, the capture modal — needs only what published `rmux 0.5.0`
offers, **not** the rendered-history capture the fork (D7) exists for. So grove
still depends on the published SDK/daemon as of this milestone; forking rmux,
adding `capture_region`, and switching the dep is the root-level `040-rmux-history`
leaf (a clean superset → mechanical dep swap). The capture modal here writes its
observation via `grove-llm inbox-add` (E1's shell-out write), needing no
rendered-history capability.

## The capture write (E1, this leaf)

The capture modal's submit performs grove's **capture write below the seam** by
shelling out to `grove-llm inbox-add --to=<grove> --repo=<root> --body=…` — the
same idiom every grove capture uses, not a new in-process path. The target is the
**focused pane's grove** (capturing over a harness leaves a note for that grove's
next session). Because the write commits + best-effort pushes, the async loop runs
it under `spawn_blocking` so a slow push cannot stall the reactor (E1's firewall).

## Consequences

- The capture-popup bug class is **dissolved by construction**, and now
  demonstrated: the capture modal renders as a centered `Clear`+widget overlay
  *over the live harness pane*, proven by a headless overlay-over-pane buffer
  test. grove's modals are native widgets, not zellij floating panes.
- The render path is headlessly testable (the migration's whole point): pane,
  nav, focus, input, and the capture overlay all run as pure
  snapshot/event → `Buffer` unit tests with no daemon and no terminal.
- Async entered the codebase, contained to `src/tui/` (E1/E2): the multi-threaded
  tokio runtime + `tokio::select!` loop live behind the presentation seam; the
  sync core is called directly, with `spawn_blocking` for the capture write.
- grove now depends on the rmux daemon (`connect_or_start` spawns it); bundling
  the daemon binary is 050/060's job. The dep is still **published `rmux 0.5.0`**
  here — the root-level `040-rmux-history` leaf forks rmux and switches to it (D7).
- `crates/trellis/` and `crates/harness-pane/` (removed in 020-rip-out) stay
  gone. The full **mark-`Superseded` sweep across the ADR-0015–0026 tower is the
  050 teardown** (D4); this ADR records the supersession at the thesis level so
  the decision is durable now, with the per-ADR edits to follow.

## Remaining engine-adjacent work (pointers, not blockers)

- **Root `040-rmux-history` (D7):** fork rmux, add native rendered-history capture
  (`capture_region`), repoint the dep, wire open-in-editor, file the upstream PR.
- **Root `050-plan-rebuild`:** the full surface set (per-grove detail / whichkey),
  the working set (multi-pane layout, aux panes, park-alive, responsive tiers),
  daemon bundling/launch, the detach+web path, and the ADR-tower teardown + `bugs`
  grove retirement + glossary cleanup.
