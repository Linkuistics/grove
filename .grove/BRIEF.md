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
retired. **A *usable* open-in-editor stand-in is load-bearing** (D1) — it must dump clean
*rendered* history, sourced from **stock `rmux capture-pane` via shell-out** (D7-as-revised;
the 040 scoping spike found rmux 0.5.0 already ships rendered-history capture, so the planned
fork is abandoned — ADR-0029). Real standalone copy-mode / scrollback / search are explicitly
**out of scope** (deferred to follow-up groves — they build on the same stock capability).

## Decomposition

Settled engine decisions (010-plan): D1 interim-parity scope · ~~D2 rendered-history ring
over `render_stream`~~ (**superseded by D7**) · D3 event-driven render path
(`render_stream` + `PaneDriver`) · D4 landmark + focused ADRs, mark-dissolved · D5
replace-in-place, no coexistence · D6 rip trellis out first.

**D7 (post-010-plan amendment, then REVISED by the 040 scoping spike — ADR-0029) — rendered
history via stock `rmux capture-pane`, no fork.** D7 originally planned to *fork* rmux and add
a rendered-history capture API sourced from the daemon emulator's scrollback. The 040 scoping
spike found this is **unnecessary**: published rmux 0.5.0 already ships complete
rendered-history capture end-to-end (emulator grid → proto `CapturePaneRequest` → stock `rmux
capture-pane` CLI). So **the fork is abandoned** and grove reaches the capability by
**shelling out to the stock `rmux capture-pane` CLI** (the ADR-0028 E1 idiom). Drop the
snapshot-diff ring entirely (no heuristic reconstruction — moot, the data source is the
emulator's real rendered grid). Consequences vs the original D7: the **dependency-model
escalation dissolves** — grove ships the *stock* rmux daemon+CLI (060 bundling unchanged, no
forked build), there is **no patch to rebase** onto new releases, and there is **no upstream
PR to carry** (optional fire-and-forget: suggest a `Pane::capture_history()` SDK convenience,
the one real SDK gap). The grove's "retire the fork" thesis is fully honoured — zero forks.

Migration roadmap (leaves materialised lazily — near-term ones exist; the tail is grown
by the 050 planning leaf so the tree never falsely reads "done"):

- **020 rip-out** (work) — delete `crates/trellis/` + `crates/harness-pane/` + wiring;
  `grove tui` disabled until the engine lands. Clean slate.
- **030 engine** (planning) — productionise the spike into a minimal rmux `grove tui`:
  the event-driven draw loop (D3), `connect_or_start` daemon, one harness pane rendered
  via `PaneWidget`, crossterm→tmux input + focus model, minimal nav + capture modal so a
  *usable* `grove tui` returns. Drafts the landmark "rmux substrate" ADR (D4).
- **040 rmux-history** (planning, DONE-via-spike) — D7: the scoping spike answered the sizing
  question decisively (rmux 0.5.0 *already* retains a rendered scrollback grid and exposes it
  via `capture-pane`), so the fork is abandoned (ADR-0029). Decomposed to a single work leaf
  `040/010-wire-open-in-editor`: shell out to stock `rmux capture-pane -p -S - -J` and dump to
  `$EDITOR`. No fork, no grove ring, no upstream PR.
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
- Dependency model (D7-as-revised, ADR-0029): grove depends on **stock published rmux 0.5.0**
  — `rmux-sdk`/`ratatui-rmux` crates and the stock `rmux` daemon+CLI binary. **No fork.**
  Rendered history comes from shelling out to the stock `rmux capture-pane` CLI. (The earlier
  plan to depend on a `Helvesec/rmux` fork is dropped — the capability already ships upstream.)
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
from the emulator's rendered grid, not raw `line_stream` bytes**. The 010-plan
spike noted that the SDK's `snapshot`/`capture_region` are visible-screen-only and
treated rendered *history* capture as an open question. **The 040 scoping spike
resolved it (ADR-0029):** rmux 0.5.0's *daemon* already keeps a rendered scrollback
grid and exposes it via `capture-pane` (the SDK helpers are visible-only, but the
proto and stock CLI are not). So rendered history comes from the emulator's real
rendered grid via the stock `rmux capture-pane` CLI — no upstream ask, no grove ring,
no fork. This unblocks open-in-editor (040) and the deferred real
copy-mode/scrollback/search groves, which build on the same stock capability.
