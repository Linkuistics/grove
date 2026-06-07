# rmux-substrate — brief

## Goal

Replace grove's vendored **trellis** zellij-fork TUI substrate (`crates/trellis/`)
with **rmux** (rmux.io, MIT/Apache, v0.5.0). grove becomes a plain **ratatui app
that owns its own draw loop**, embedding harness/tool panes via `ratatui-rmux`
(`PaneWidget` over a `PaneSnapshot`) and driving/observing them via the async
`rmux-sdk` (`send_text`/`send_key`/`mouse()`, `snapshot`/`wait_for_text`/`find_text`).
A separate rmux daemon owns the ptys.

**Why:** trellis compiles grove *into* a forked zellij server, so grove's own UI
lives inside zellij's pane/buffer model — the root cause of the capture-popup bug
and of the whole ADR-0016–0028 host-surface / host-driver / ScreenInstruction
tower, and it is un-verifiable headlessly. Under rmux, grove's surfaces
(nav/detail/capture/whichkey) become ordinary ratatui widgets → centered floating
modals are trivial and verification is plain headless tests. Retires the fork.

## Done when

(scope settled in 010-plan, D1) **Interim parity:** `grove tui` runs on rmux with the
nav + per-grove detail + capture modal + working set rebuilt as ratatui widgets;
`crates/trellis/` and `crates/harness-pane/` deleted; the ADR-0013–0028 tower dissolved
(landmark + focused ADRs written, dissolved ones marked superseded); the `bugs` grove
retired. **A *usable* open-in-editor stand-in is load-bearing** (D1/D2) — it must dump
clean *rendered* history, not raw bytes. Real standalone copy-mode / scrollback / search
are explicitly **out of scope** (deferred to follow-up groves).

## Decomposition

Settled engine decisions (010-plan): D1 interim-parity scope · D2 rendered-history ring
over `render_stream` (no fork; upstream ask in parallel; spike-validated) · D3
event-driven render path (`render_stream` + `PaneDriver`) · D4 landmark + focused ADRs,
mark-dissolved · D5 replace-in-place, no coexistence · D6 rip trellis out first.

Migration roadmap (leaves materialised lazily — near-term ones exist; the tail is grown
by the 050 planning leaf so the tree never falsely reads "done"):

- **020 rip-out** (work) — delete `crates/trellis/` + `crates/harness-pane/` + wiring;
  `grove tui` disabled until the engine lands. Clean slate.
- **030 engine** (planning) — productionise the spike into a minimal rmux `grove tui`:
  the event-driven draw loop (D3), `connect_or_start` daemon, one harness pane rendered
  via `PaneWidget`, crossterm→tmux input + focus model, minimal nav + capture modal so a
  *usable* `grove tui` returns. Drafts the landmark "rmux substrate" ADR (D4).
- **040 history-ring** (planning) — D2: opens with a validation spike (does
  `render_stream` snapshot-diffing reconstruct usable scrollback across clears / partial
  scrolls / resize-reflow?), then builds the per-pane ring + wires open-in-editor. Files
  the upstream rmux rendered-history feature request.
- **050 plan-rebuild** (planning) — grill + grow the remaining areas: the full surface
  set (nav / per-grove detail / capture / whichkey as widgets), the working set
  (multi-pane layout, aux term/yazi/vcs panes, park-alive via rmux splits/sessions,
  responsive tiers), daemon lifecycle + bundling + `grove tui` launch (vendoring the rmux
  daemon binary, `SDK_DAEMON_BINARY_ENV`, session naming/persistence, fleet/multi-repo),
  the detach + web path (rmux session persistence; rmux web-share vs grove's
  whole-UI-on-web goal), and the **teardown** (dissolve the ADR tower per D4, retire the
  `bugs` grove, glossary cleanup).

## Pointers

- Spike: `~/Development/rmux-spike/` (throwaway, outside the repo). Probes 1–3 pass;
  probe 3 (`probe3-interactive`) embeds a live shell with a centered modal + F3
  open-in-editor. Cargo deps: `rmux-sdk = "0.5"`, `ratatui-rmux = "0.5"`.
- Auto-memory `project_rmux-substrate-evaluation` — full evaluation context.
- Superseded substrate ADRs to dissolve/supersede: 0013–0028 (esp. 0015 owned-zellij,
  0020 trellis-fork, 0021 hosting-API, 0026 trellis-is-the-only-tui).
- `crates/trellis/` (vendored zellij fork) and `crates/harness-pane/` (shelved
  in-process-pty fallback) both become removable.
- The `bugs` grove is superseded — its backlog is trellis-specific and mostly
  evaporates; its branch carries a committed-but-broken trellis floating-pane change
  that is now moot.

## Notes

Two founding observations were incorporated into 010-plan at bootstrap:
the migration decision itself, and the spike finding that **scrollback must come
from the emulator's rendered grid, not raw `line_stream` bytes** — rmux's clean
rendered text (`snapshot`/`capture_region`) is visible-screen-only today, so
rendered *history* capture is an open question (feature-ask-upstream vs. grove
keeps its own rendered-scrollback ring). This blocks real copy-mode/scrollback/
search; open-in-editor is only an interim stand-in and even it needs rendered
history to be useful.
