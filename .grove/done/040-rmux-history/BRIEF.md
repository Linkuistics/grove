# 040-rmux-history — brief

## Goal

Make open-in-editor *usable* (010-plan D1) — a leader-triggered dump of the focused
harness pane's **rendered** history into `$EDITOR`. The scoping spike (below) overturned
D7's premise: the rendered-history capability already ships, complete, in published
**rmux 0.5.0**, so there is **no fork**. grove reaches it by shelling out to the stock
`rmux capture-pane` CLI — the same shell-out-below-the-seam idiom as the capture write
(ADR-0028 E1). Decision recorded in **ADR-0029**.

## Scoping spike findings (2026-06-08) — premise overturned

Spike question: *does the rmux daemon emulator already retain a rendered scrollback buffer
(expose it) or must we add one (build it)?* Answer: **neither — published rmux 0.5.0 already
exposes complete rendered-history capture end-to-end (emulator → proto → CLI).** Evidence
(`~/.cargo/registry/.../rmux-*-0.5.0`):

1. **Emulator retains a rendered scrollback grid.** `rmux-core/src/grid.rs:68` — "Absolute
   grid storage split into history and visible rows", `history: VecDeque<GridLine>`,
   `history_enabled: true` by default, bounded by `history-limit`. Real cells, not a raw
   byte backlog.
2. **Capture renders from that grid.** `rmux-core/src/screen/capture.rs::capture_transcript`
   → `grid.render_absolute_line(...)`; `transcript.rs` resolves tmux ranges over
   `history_size` (negative offsets relative to history; `-` = absolute history start).
3. **The proto carries it.** `rmux-proto` `CapturePaneRequest` (`request/buffer.rs:107`):
   `start/end: Option<i64>`, `start_is_absolute`/`end_is_absolute` (`-S -`/`-E -`), `print`
   (`-p`), `escape_ansi` (`-e`), `join_wrapped` (`-J`), `alternate` (saved pre-alt grid).
4. **The stock `rmux` CLI exposes it** (`rmux-0.5.0/src/cli_args/history.rs`,
   `src/cli/capture_pane.rs`), connecting to the *running* daemon via `rmux_client::Connection`
   — the same daemon grove's TUI spawns via `connect_or_start`. Pane targeting uses the tmux
   `%N` convention (`rmux-core/src/target_find/syntax.rs:34`), and `PaneId` (re-exported from
   `rmux-proto`) is the stable id grove already holds per E3.

**Only gap:** the `rmux-sdk` `Pane` has no in-process capture-history method
(`capture.rs`/`screenshot` are visible-snapshot-only) and `transport::request` is
`pub(crate)` with no escape hatch — so an *in-process* path would need a (tiny) SDK fork,
whereas the CLI shell-out needs none. We chose the shell-out (ADR-0029).

## Decisions (running log)

- **D-A — Reach via shell-out, no fork** (user, grilling): open-in-editor runs the stock
  `rmux capture-pane` CLI; grove ships stock rmux. Rejects D7's daemon fork (capability
  already upstream; no sunk-cost). Dissolves D7's two caveats — no forked daemon to bundle
  (060 ships stock rmux), no rebase-onto-upstream burden. ADR-0029.
- **D-B — Dump = plain text, full history** (user): `rmux capture-pane -p -S - -J -t %<id>`
  — full retained scrollback + visible, soft-wrapped lines rejoined (`-J`), trailing space
  trimmed, **no `-e`** (editors don't render ANSI). Alt-screen apps (vim/htop) reflect the
  current screen, acceptable per the task caveat.
- **D-C — Trigger = leader → `e` on the focused harness** (user): mirrors the capture
  modal's leader→`c` (ADR-0028 E4). Leader-gated, so no collision with harness F-keys.
- **D-D — Binary resolution reuses the SDK's daemon resolution** (recommended): resolve the
  `rmux` binary the same way `connect_or_start` does (honouring `SDK_DAEMON_BINARY_ENV`), not
  bare `rmux` on PATH — so dev builds and the 060-bundled binary both work, and the CLI talks
  to the daemon grove started.
- **D-E — Editor flow suspends the loop and pauses the input reader** (recommended): leave
  alt-screen + disable raw mode (+ DisableMouseCapture/BracketedPaste), **pause
  `spawn_input_reader`** so the editor child owns stdin (no thread race), run `$EDITOR`
  blocking (inherit stdio), then restore terminal state + force a full redraw. `$EDITOR`
  resolution: `$VISUAL` → `$EDITOR` → `vi`. The capture itself goes through `spawn_blocking`
  so the reactor is not stalled, matching E1.
- **D-F — No upstream PR required** (amends D7): there is no fork to upstream. *Optional,
  fire-and-forget:* file an upstream feature suggestion that `rmux-sdk` add a
  `Pane::capture_history()` convenience (the one real SDK gap) — nice-to-have, never a
  blocker, and grove does not depend on it landing.

## Done when

- `010-wire-open-in-editor` is complete: leader→`e` dumps the focused harness pane's full
  rendered history to a temp file and opens `$EDITOR`, restoring the TUI cleanly on exit;
  the dump is genuinely readable (the D1 bar). Headlessly tested where the seam allows
  (command construction is a pure, unit-testable function; the editor drop is integration).
- ADR-0029 is accepted (drafted this session) and the root brief + glossary reflect the
  no-fork reality.

## Decomposition

- **010 wire-open-in-editor** (work) — the only remaining work; the heavy fork that D7
  anticipated evaporated. Implements D-A…D-F.

## Pointers

- ADR-0029 (`docs/adr/0029-rendered-history-via-stock-rmux-capture-pane.md`) — the decision +
  full spike rationale.
- ADR-0028 (rmux substrate) E1 — the shell-out-below-the-seam idiom this mirrors; E3 — stable
  `PaneId` addressing; E4 — leader-gating.
- The capture modal (`src/tui/capture.rs`, `src/tui/app.rs::submit_capture`) — the existing
  `spawn_blocking` shell-out + leader-gated overlay to pattern-match.
