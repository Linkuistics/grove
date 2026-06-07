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
*rendered* history, sourced from native rmux rendered-history capture added in a grove
fork of rmux (D7). Real standalone copy-mode / scrollback / search are explicitly **out of
scope** (deferred to follow-up groves — they will build on the same D7 capability).

## Decomposition

Settled engine decisions (010-plan): D1 interim-parity scope · ~~D2 rendered-history ring
over `render_stream`~~ (**superseded by D7**) · D3 event-driven render path
(`render_stream` + `PaneDriver`) · D4 landmark + focused ADRs, mark-dissolved · D5
replace-in-place, no coexistence · D6 rip trellis out first.

**D7 (post-010-plan amendment) — rendered history via a rmux *fork* with native capture,
superseding D2's grove-ring.** Do the work here: fork rmux, add a rendered-history capture
API (`capture_region(start = -N)` / tmux `capture-pane -S` equivalent) sourced from the
daemon emulator's scrollback, depend on the fork, and call it directly. Drop the
snapshot-diff ring entirely (no heuristic reconstruction). File an upstream PR from the
fork but **don't block on the merge** ("leave the upstreaming") — carry the fork until it
lands, then drop back to crates.io. Rationale: correct data source instead of a heuristic
reconstruction, and the same capability is what the deferred real copy-mode/scrollback/
search (D1) will build on. Caveats: (1) a fork we want merged must *track* upstream
(rebase the one patch onto new rmux releases — a real but bounded cost, unlike the frozen
zellij fork); (2) escalates the dependency model — grove now depends on the **forked rmux
daemon** (`connect_or_start` spawns our build), so 030-engine targets the forked daemon
and the 060 bundling work ships our build. The fork-vs-"retire-the-fork" tension is
accepted: a single-feature fork with a live PR is qualitatively different from the deep
zellij hard-fork.

Migration roadmap (leaves materialised lazily — near-term ones exist; the tail is grown
by the 050 planning leaf so the tree never falsely reads "done"):

- **020 rip-out** (work) — delete `crates/trellis/` + `crates/harness-pane/` + wiring;
  `grove tui` disabled until the engine lands. Clean slate.
- **030 engine** (planning) — productionise the spike into a minimal rmux `grove tui`:
  the event-driven draw loop (D3), `connect_or_start` daemon, one harness pane rendered
  via `PaneWidget`, crossterm→tmux input + focus model, minimal nav + capture modal so a
  *usable* `grove tui` returns. Drafts the landmark "rmux substrate" ADR (D4).
- **040 rmux-history** (planning) — D7: opens with a scoping spike (does the rmux daemon
  emulator already retain a rendered scrollback buffer to expose, or must we add one?),
  then forks rmux, adds the native rendered-history capture API, points grove's dep at the
  fork, wires open-in-editor to it, and opens an upstream PR (fire-and-forget). No grove
  ring.
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
- Dependency model (D7): grove depends on a **fork of rmux** (`Helvesec/rmux`,
  MIT/Apache) carrying the rendered-history capture patch — for both the SDK crates and
  the daemon binary `connect_or_start` spawns. Upstream PR filed, not blocked on. Fork
  tracks upstream (rebase the patch onto new releases until merged, then drop to crates.io).
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
