# 29. Rendered history via stock `rmux capture-pane` (D7 fork abandoned)

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record.
- Date: 2026-06-08
- Deciders: Antony Blakey (with grove `rmux-substrate` 040-rmux-history)
- Amends: ADR-0028 (rmux substrate) — extends E1's shell-out-below-the-seam idiom to
  rendered-history capture, and **retracts the root brief's D7 amendment** (fork rmux to add
  native rendered-history capture sourced from the daemon emulator's scrollback).

## Context

Open-in-editor is load-bearing for the interim-parity milestone (010-plan D1): a *usable*
stand-in must dump clean **rendered** history (not raw `line_stream` ANSI bytes — the spike's
strip-ANSI dump was unusable). The root brief's D7 amendment planned to get that by **forking
rmux**: add a rendered-history capture API sourced from the daemon emulator's scrollback,
depend on the fork (SDK *and* daemon binary), and carry an upstream PR until merged.

040 opened with the scoping spike D7 mandated — *the size of the rmux change is the risk, not
the wiring.* The spike read the published rmux 0.5.0 source directly.

## The spike finding (premise overturned)

**Published rmux 0.5.0 already exposes complete rendered-history capture, end-to-end.** There
is nothing to add to the emulator or the proto:

1. The emulator retains a **rendered scrollback grid** — `rmux-core/src/grid.rs:68`,
   "Absolute grid storage split into history and visible rows", `history: VecDeque<GridLine>`,
   `history_enabled: true` by default, bounded by `history-limit`. Cells, not a raw byte
   backlog.
2. Capture **renders from that grid** — `rmux-core/src/screen/capture.rs::capture_transcript`
   → `grid.render_absolute_line(...)`; `rmux-core/src/transcript.rs` resolves tmux-style
   ranges over `history_size` (negative offsets relative to history; `-` = absolute history
   start). This *is* the `capture_region(start = -N)` / `capture-pane -S` D7 set out to add.
3. The **proto carries it** — `rmux-proto::CapturePaneRequest` (`request/buffer.rs:107`):
   `start/end: Option<i64>`, `start_is_absolute`/`end_is_absolute` (the `-S -`/`-E -`
   full-history sentinels), `print` (`-p`), `escape_ansi` (`-e`), `join_wrapped` (`-J`),
   `alternate` (saved pre-alt-screen grid for vim/htop).
4. The **stock `rmux` CLI exposes it** — `rmux-0.5.0/src/cli_args/history.rs`,
   `src/cli/capture_pane.rs` — connecting to the *running* daemon via
   `rmux_client::Connection`: the same daemon grove's TUI spawns with `connect_or_start`.
   Pane targeting uses the tmux `%N` convention (`rmux-core/src/target_find/syntax.rs:34`),
   and `PaneId` (re-exported by `rmux-proto`) is the stable id grove already holds (ADR-0028
   E3).

The **only** genuine gap is SDK ergonomics: `rmux-sdk`'s `Pane` has no in-process
capture-history method (`capture.rs`/`screenshot` operate on the *visible* `PaneSnapshot`
only), and `transport::request` is `pub(crate)` with no public escape hatch (`RmuxCommandKind`
is an inert DTO — `command.rs` states it "does not send IPC"). So reaching capture *in
process* would require a fork of the SDK crate; reaching it by **shelling out to the `rmux`
CLI** requires no fork at all.

## Decision

**grove reaches rendered history by shelling out to the stock `rmux capture-pane` CLI; the
D7 fork is abandoned.** Open-in-editor runs

```
rmux capture-pane -p -S - -J -t %<paneid>
```

against the daemon grove already started, writes stdout to a temp file, and drops to
`$EDITOR`. This is the same shell-out-below-the-seam idiom as grove's capture write (ADR-0028
E1: `grove-llm inbox-add` under `spawn_blocking`).

Sub-decisions (040 grilling): plain text, full retained history, soft-wrap joined (`-J`),
**no `-e`** — editors don't render ANSI (D-B); triggered by **leader → `e`** on the focused
harness, mirroring the capture modal's leader → `c` (D-C); the `rmux` binary resolved like the
SDK daemon, honouring `SDK_DAEMON_BINARY_ENV` (D-D); the editor flow suspends the loop and
**pauses the crossterm input-reader thread** so the editor child owns stdin (D-E).

## Why (and why not the alternatives)

- **No sunk cost.** D7 was a reasonable plan under the belief that rmux lacked the capability.
  The spike disproved the belief; the decision is re-made on current merit.
- **It restores the grove's thesis.** The whole `rmux-substrate` grove exists to *retire a
  fork* (the deep zellij hard-fork). D7 had quietly reintroduced one (a forked rmux daemon).
  Shelling out to stock rmux returns grove to **zero forks**.
- **It de-escalates the dependency model.** D7's caveat — "grove now depends on the forked
  rmux daemon; 060 bundling ships our build" — **dissolves**: grove bundles the *stock* rmux
  daemon, and there is no patch to rebase onto each upstream release.
- **Rejected — tiny SDK-only fork** (add `Pane::capture_history()` sugar; daemon stays
  stock): more "in-process" purity, but reintroduces a fork (however small) and an upstream-PR
  carry for ergonomic sugar that the shell-out gets for free. Not worth it.
- **Rejected — the original D7 medium fork** (emulator scrollback + new proto verb):
  redundant; the capability already ships.

## Consequences

- 040 collapses to one small work leaf (`010-wire-open-in-editor`); the anticipated heavy
  fork work evaporates.
- **060 bundling ships stock rmux** (daemon + CLI are the one `rmux` binary). No fork to
  vendor, track, or rebase.
- The deferred real copy-mode / scrollback / search groves (010-plan D1) build on the **same
  stock `capture-pane`** capability — they too get simpler (no grove fork to depend on).
- Optional, fire-and-forget (D-F): an upstream suggestion that `rmux-sdk` add a
  `Pane::capture_history()` convenience would close the one real SDK gap, but grove does not
  depend on it.
- The root brief's D7 amendment, "Done when", "Pointers" dependency-model note, and the 040
  decomposition entry are corrected to the no-fork reality (this session).
