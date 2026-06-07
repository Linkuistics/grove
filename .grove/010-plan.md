# 010-plan

**Kind:** planning

## Goal

Grill the rmux-substrate migration into a shared plan and grow the tree. The seed
named seven design areas to walk: working-set layout under rmux; input-mapping
coverage; render path (snapshot-per-frame vs incremental); copy-mode/scrollback/
search (with open-in-editor as interim stand-in); detach/persistence + web path;
daemon lifecycle/bundling; migration order + how `grove tui` launches. Plus two
cross-cutting concerns this session must settle: the **ADR strategy** (which of
0013–0028 are superseded, and whether by new superseding ADRs or one consolidating
ADR) and the **scope of "done"** (full parity vs. interim with stand-ins).

## Context

The migration decision is settled (see root BRIEF + incorporated seed). This is a
*how-to-adopt* planning session, not a *whether-to-adopt* one — but the scrollback
finding is a genuine open risk that may reshape the copy-mode plan. rmux API surface
is grounded in the spike (`~/Development/rmux-spike/src/interactive.rs`).

## Done when

The major design areas are either settled inline (recorded in the running log +
ADRs/CONTEXT) or decomposed into child leaves with briefs, the tree reflects the
plan, and a migration order exists. A PRD is written only if an increment is a
genuine human-facing agreement point.

## Decisions (running log)

**D1 — Done scope: interim parity, but a *usable* open-in-editor is load-bearing.**
The grove's endpoint is: rebuild grove's surfaces (nav/detail/capture/whichkey) +
working set as ratatui widgets on rmux, delete trellis, dissolve the ADR-0013–0028
tower. Real standalone copy-mode / scrollback / search are deferred to follow-up
groves. BUT — user steer — the open-in-editor stand-in must be *genuinely usable*,
which the spike's `strip-ansi(line_stream)` was **not** (raw cursor-addressed bytes
don't linearize). Per the second seed finding, "usable" requires the emulator's
**rendered grid history**, not raw output bytes. So *rendered-history capture* is
IN scope for "done" even though full copy-mode/search is not. This makes the
rendered-history question (next) the load-bearing technical risk of the grove.
Rationale: the swap's value is retiring the fork + unblocking the modal bug now;
coupling it to full copy-mode (Full-parity option) would chain it to an unresolved
upstream dependency, while shipping an unusable stand-in (the spike's regression)
is not acceptable. Interim-with-a-working-stand-in threads the needle.

**D2 — Rendered history = grove ring over `render_stream`, no fork, upstream ask in
parallel; spike-validated first.** Source clean history from `render_stream()`'s
rmux-RENDERED snapshots (NOT raw `line_stream`, NOT a grove vt100 emulator, NOT a
rmux fork on the critical path). grove reconstructs scrolled-off rendered lines into
a bounded per-pane ring; open-in-editor dumps the ring. Rejected: a grove client-side
emulator (re-adds the `harness-pane` burden the migration deletes + double-emulation +
vt100 pin) and patching rmux on the critical path (upstream-timeline / mini-fork risk).
A parallel **upstream feature request** for native rendered-history capture is filed so
the ring can later retire — the daemon already emulates, so `capture_region(start=-N)`
is the correct long-term source. The ring's reliability (scroll reconstruction across
clears / partial scrolls / resize-reflow) is empirically unproven → this area gets its
own leaf that **opens with a validation spike** before committing.

**D3 — Render path = event-driven push (`render_stream` + `PaneDriver`), not
pull-per-frame.** grove runs a tokio loop that `select!`s over {per-pane render
updates, crossterm input, fs-watch ticks} and redraws only when dirty. Each visible
pane has a `PaneDriver` fed by a `render_stream()` task; that one subscription per
pane feeds both the live `PaneState`/`PaneWidget` and the D2 ring. Rejected the
spike's pull-per-frame (idle polling, N round-trips/frame, no sharing with the ring).
A client-side FPS coalescer is deferred until a revision flood is shown to over-draw
(render_stream already debounces 16ms server-side). Durable + surprising + a real
trade-off → ADR-worthy (see ADR strategy).

**D4 — ADR strategy: landmark + focused new ADRs + mark-dissolved.** Write one
consolidating **landmark "rmux substrate" ADR** (the inversion: grove owns the draw
loop, daemon owns ptys, render_stream push, ring history) that supersedes the tower;
a few **focused new ADRs** only where a genuinely new durable decision exists
(rendered-history ring = D2; event-driven render path = D3). For old ADRs that simply
*cease to apply* under rmux (proxy/seam/host-surface/ScreenInstruction — 0016, 0017,
0021, 0023, 0024, …), mark them **Superseded by <landmark>** with a one-line "problem
dissolved under rmux" — no fake 1:1 replacements for problems that no longer exist.
D1 (scope) is process, not ADR-worthy. The landmark + focused ADRs are authored within
the relevant build leaves, not all up front.

**D5 — Migration strategy: replace in place, no coexistence.** No flag, no parallel
trellis path. The rmux TUI becomes the sole substrate; trellis stops being the
`grove tui` path as the engine lands. Chosen (over my recommended coexist-behind-a-flag)
because keeping the vendored zellij fork *compiling* alongside a new rmux path is a real
maintenance tax, a clean replace lets the rmux design be unconstrained by trellis, and
`grove tui` being down mid-migration is *degraded, not blocked* — groves are still
driven via `grove-llm`/`grove status` (as this very session is). Mitigation against the
"no fallback" risk: order the rebuild so a **minimal usable `grove tui` returns fast**
(engine + harness pane + capture + minimal nav as the first milestone), rather than
leaving the TUI down for the whole grove. Open sub-decision → Q6: delete trellis as
literal step one, or fold its removal into the engine landing.

**D6 — Rip trellis out as literal step one (clean slate).** The first leaf deletes
`crates/trellis/` + `crates/harness-pane/` and all their wiring immediately, accepting
that `grove tui` is down until the engine rebuild reaches minimal parity. Maximally
unconstrained by the old substrate; no half-migrated coexistence. The ADR-tower
dissolution + `bugs`-grove retirement + glossary cleanup follow at the teardown leaf
(the landmark ADR is drafted as the engine architecture lands, per D4).

## Notes

### rmux 0.5.0 API facts (grounding for grilling; from crate-source map)

- **Daemon:** `Rmux::builder().connect_or_start()` spawns the daemon if absent;
  `SDK_DAEMON_BINARY_ENV` overrides the daemon binary path (bundling hook).
  `Rmux::shutdown()` requests daemon shutdown. `owned_session(...)` = app-owned
  session guard with cleanup policies.
- **Sessions/panes:** `ensure_session` (CreateOnly/CreateOrReuse/ReuseOnly,
  detached, size, window_name, working_directory, process spec). `session.pane(w,p)`
  (slot, lazy) or `pane_by_id(PaneId)` (stable id). `Session::layout()` declarative
  layout builder. `new_window`, `Pane::split()/split_with()`, `close()`,
  `respawn()/spawn()/shell()`. `Window::panes()`, `info()`.
- **Render:** `Pane::snapshot()` → `PaneSnapshot` (visible grid + cursor + `revision`).
  `PaneSnapshot` carries per-cell glyph/attrs/colors + `visible_lines()/visible_text()`.
  `ratatui-rmux`: `PaneState::from_snapshot` → `PaneWidget` (sync, pure, deterministic).
  `PaneState` also tracks lifecycle (Live/Closed/Exited/Disconnected), paused, lagging.
- **Three streams** (all start `Now` or `Oldest`):
  - `output_stream()` → **raw bytes** (`PaneOutputChunk::Bytes{seq,bytes}` + Lag notices).
  - `line_stream()` → lossy-UTF-8 LF-split lines (what the spike used — ANSI-laden, raw).
  - `render_stream()` → **rmux-RENDERED `PaneSnapshot`s**, debounced 16ms, emitted only
    on revision change (`RenderUpdate{snapshot, lag}`). ← the clean-grid stream.
- **No rendered-history / scrollback capture** (tmux `capture-pane -S -<n>`): ABSENT.
  `snapshot` and `find_text` are visible-screen only. History must be reconstructed
  by the consumer from a stream, OR exposed by an upstream rmux change.
- **Input:** `send_text` (literal), `send_key` (tmux tokens), `mouse().click/move_to`
  (SGR), `resize`. **Observe:** `wait_for(bytes)`, `wait_for_text`, `find_text`,
  `wait_exit`. **Discovery:** `find_panes().title()`, `get_pane_by_title`.
- **Web:** optional daemon `CAPABILITY_WEB_SHARE` → `WebShareBuilder`/`WebShareHandle`
  (HTTP share URLs). Sessions persist across clean daemon restarts (sticky metadata).
