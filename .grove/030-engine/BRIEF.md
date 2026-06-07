# 030-engine — brief

Productionise the spike (`~/Development/rmux-spike/src/interactive.rs`) into a minimal
but *usable* rmux-backed `grove tui` — the milestone that brings the TUI back after the
020 rip-out. This node's grilling settled the engine design (decisions E1–E6 below) and
decomposed the build into four work leaves.

## Goal

`grove tui` runs on rmux as a **ratatui app that owns its own draw loop**, embedding one
**harness pane** via `ratatui-rmux` `PaneWidget` and driving it via the async `rmux-sdk`.
Below the presentation boundary (ADR-0013) the `RepoView`/`MultiRepoView` core is
unchanged — this node is the new presentation over it.

## Done when

`grove tui` runs on rmux: a harness pane renders + takes input, a centered capture modal
works over the live pane, and a minimal nav exists. Build + **headless** tests green (the
testability win — the spike's probes ran headless). The **landmark "rmux substrate" ADR**
is written (drafted in 010, finalized in 040, per D4).

## Decomposition

Four work leaves, each an independently demoable increment, in order:

- **010-draw-loop-pane** — async `grove tui` entry, `Rmux::connect_or_start` +
  `ensure_session`, one harness pane (`grove do <name>`, addressed by `PaneId`), the D3
  event loop (`render_stream` + `PaneDriver` + `tokio::select!` over
  {render-updates, crossterm input, fs-watch}), `PaneWidget` render with the hardware
  cursor placed from `snapshot.cursor`. Establishes the `src/tui/` module tree (E2) on
  **published rmux 0.5.0** (E6). Drafts the landmark ADR skeleton. → *harness renders live
  (read-only)*.
- **020-input-focus** — crossterm→tmux key-map (extends the spike's `forward_key` to the
  modifier matrix), the `Harness | Nav | Modal` focus state machine (E4), `Alt-g`
  leader-gating, bracketed-paste forwarding (wrapped), click-to-focus mouse (E5).
  → *you can type into the harness*.
- **030-nav** — the minimal `Nav` surface: list groves from `RepoView`/`MultiRepoView`,
  navigate + select → open/focus the harness pane. → *nav lists & opens groves*.
- **040-capture-modal** — the centered capture modal over the live pane (the motivating
  bug-fix proof point), a native ratatui focus-overlay wired to grove's capture write.
  Finalizes the landmark ADR. → *the modal bug is fixed*.

The landmark ADR is authored **within** these leaves (D4: not a separate leaf). Full
surface set (per-grove detail / whichkey), the working set, daemon bundling/launch, and
the ADR-tower teardown remain **050**'s job; the rendered-history fork is **040**.

## Decisions (running log)

**E1 — Async blast radius: confine async to the TUI presentation layer; sync core
stays sync; multi-thread tokio runtime.** grove is 100% synchronous today (zero
`tokio`); the rmux SDK is wholly async and D3 already makes the draw loop a
`tokio::select!`. So async enters now, via this node — but *only above/at the
presentation seam* (ADR-0013). The async draw loop + all rmux glue live in the new
TUI module; the sync core (`RepoView::scan`, shell-out writes, fleet resolution) is
called directly from async context (it is fast, local fs/git), with `spawn_blocking`
reserved for any call slow enough to stall the reactor. Below-boundary code never
imports `tokio`. The `grove tui` entry builds the **default multi-threaded**
`#[tokio::main]` runtime, so each visible pane's `render_stream` task can run on its
own worker. Rejected: async-ifying the core (huge churn, no payoff for local ops,
dirties the seam) and a sync loop with per-frame `block_on` (contradicts D3's
event-driven push). **Amends ADR-0013's consequence** that async is "the entry toll
for web, paid only if/when web is chosen" — rmux brings async forward now, but the
presentation boundary contains it (the seam doubles as a runtime firewall). Durable +
surprising (reverses a recorded ADR-0013 prediction) + a real trade-off → folds into
the landmark ADR (or a short focused amendment to 0013).

**E2 — Crate structure: a new `src/tui/` module tree in the existing binary; the
presentation boundary is the directory wall.** No separate `grove-tui` crate. Code
under `src/tui/` may import `ratatui`/`rmux`/`tokio`; code outside it may not — this
makes ADR-0013's "boundary enforced by module placement and review" a literal
directory. Submodules: the async draw loop (`app`), per-pane rmux glue
(`pane`/`driver` — `PaneDriver` + `render_stream` task), `input` (crossterm→tmux
token mapping + routing), `nav`, `capture`. Matches grove's deliberate single-package
workspace (`Cargo.toml` `members = []`) and constraint 6 (single walk-away binary).
The forked rmux **dependency** (per E6/D7) is a normal Cargo dep; a *vendored* fork
source (if 040 vendors) would be a path member — a separate axis from where grove's
TUI code lives. Rejected the `crates/grove-tui` split: it buys a compiler-enforced
seam but needs a core/tui lib split ADR-0013 explicitly declined ("no second consumer
exists yet"); reconsider when web arrives. Trade-off accepted: the module wall is a
review-time guard, not a compile-time one.

**E3 — Session/pane model: one rmux session per `grove tui`; only foreign programs
are panes; harness pane runs `grove do <name>`; address panes by stable `PaneId`.**
The inversion frame (landmark thesis, from the brief): grove's *own* surfaces
(nav, capture) are native ratatui widgets grove draws; only foreign terminal programs
(the harness now; yazi/lazygit/shell in 050) are rmux panes. 030 minimal = **one
session, exactly one rmux pane** (the harness), nav+capture drawn around/over it.
- *Session:* one rmux session per `grove tui` process, named deterministically
  (historically the singleton `grove-fleet`, ADR-0027), `ensure_session(CreateOrReuse,
  detached)`. 050 revisits whether park-alive uses hidden panes here vs separate
  detached sessions.
- *Process spec:* the harness pane runs **`grove do <name>`** (cwd = worktree), not the
  bare harness binary — keeps `grove do` the single lifecycle entry point so its setup
  (worktree attach, drain, skill-stamp) runs before the harness execs. Confirmed safe:
  `exec_harness` (`src/launch.rs:147`) execs the harness *directly*, no nested
  multiplexer, so no recursion.
- *Addressing:* stable **`PaneId`** with a `grove-name → PaneId` map, not positional
  slot `(w,p)`. The spike used slot `pane(0,0)`; slots are positional and 050's dynamic
  open/close/park would have to unwind them. Establish PaneId now though 030 has one pane.

**E4 — Focus model: leader-gated arbitration; leader = `Alt-g` (configurable);
focus is a `Harness | Nav | Modal` state machine.** grove owns the loop, so there is
no zellij locked-mode — grove is the arbiter by construction and sees every crossterm
key first. Arbitration is **leader-gated**: while the harness pane is focused, grove
forwards *everything* to the pane except the single leader key, maximising harness key
fidelity (claude/vim lean on F-keys + Ctrl-chords; every stolen key is a fidelity
loss). Rejected always-arbitrate (the spike's global-hotkey allowlist — steals F2/F3
etc. from the harness) and pure hybrid.
- *Focus state machine:* `Focus = Harness(PaneId) | Nav | Modal(kind)`. **Harness:**
  forward all keys except leader (leader → Nav). **Nav:** grove handles keys
  (navigate, select → open/focus a harness, open Modal); Esc/return → Harness.
  **Modal:** focus overlay capturing all keys (text into the capture buffer), Esc
  cancels, Enter submits, both restoring prior focus. Native ratatui — the
  centered-modal-over-live-pane bug-fix proof point.
- *Leader = `Alt-g`* (mnemonic: g = grove), configurable from day one. Chosen over
  `Ctrl-b`/`Ctrl-a`/`Ctrl-o`/`Ctrl-Space`: `M-g` is largely unbound across readline /
  vim / claude (lowest collision), crossterm reports it cleanly as `Char('g')` +
  `ALT`, encoding-safe enough. Caveat: Alt-detection relies on the terminal sending
  ESC-prefixed meta sequences (modern default) with a negligible Esc-then-`g`
  ambiguity — acceptable since it's remappable.

**E5 — Input coverage: mouse drives grove surfaces + click-to-focus (rich passthrough
deferred + flagged as a fork capability); bracketed paste forwarded wrapped; key-map
extends the spike.**
- *Mouse:* rmux's mouse API is automation-style (`mouse().click/move_to`), not raw
  passthrough — faithful drag/wheel/motion/button-up forwarding is architecturally
  limited (same shape as 040's history gap). For 030: mouse drives grove's own
  nav/modal, and a basic left-click forwards to the harness as focus/click. **Rich
  mouse passthrough is deferred to 050 and flagged as a likely rmux-*fork* capability
  (raw-mouse passthrough)** — do not build a lossy automation-call translator now.
- *Bracketed paste:* enable crossterm bracketed paste; on `Event::Paste` with the
  harness focused, forward via `send_text` **wrapped in `\e[200~…\e[201~`** so
  multi-line pastes don't execute line-by-line (claude multi-line input / vim
  paste-mode depend on it). In `Modal` focus, paste inserts into the buffer literally.
- *Key-map:* the spike's `forward_key` is the baseline; extend to the modifier matrix
  on special keys (`C-Left`, `S-Up`, `C-Enter`, …) + correct Shift handling. Build
  detail, recorded so "minimal" ≠ "spike-verbatim".

**E6 — Dependency staging: 030 builds on published `rmux 0.5.0`; 040 forks and
switches the dep.** 030 needs only render/input/session, *not* the rendered-history
capture the fork exists for, so forking up front would pull 040's substantial work
(create `Helvesec/rmux`, add the capture API, build+vendor the daemon binary, repoint
deps) earlier for zero 030 benefit. The fork is a clean superset (adds
`capture_region`), so 040's mid-stream dep swap is mechanical. **Amends the brief's D7
"030 targets the forked daemon"** — 030 targets the *published* daemon/SDK; 040 does
the switch. Accepted consequence: 030's published daemon and 040's forked daemon are
different builds momentarily, but they render/drive identically and bundling our build
is 050/060's job regardless.
</content>
</invoke>
