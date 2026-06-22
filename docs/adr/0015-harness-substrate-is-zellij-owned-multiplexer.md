# 15. Harness/presentation substrate is a grove-owned zellij multiplexer

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record. (Prior status: superseded by
  [ADR-0028](0028-rmux-substrate.md), rmux substrate, 2026-06-10, 070-teardown D4 — the
  grove-owned zellij multiplexer was replaced wholesale: under rmux a separate daemon
  owns the ptys and grove owns its own ratatui draw loop. Premise + mechanism gone.)
- Date: 2026-05-31
- Deciders: Antony Blakey (with grove spike 060/020)
- Supersedes: ADR-0014 (in-process pty embed)
- Refined by: ADR-0016 (the 1b dashboard is a dumb-terminal proxy to a persistent
  controlling process that owns all logic + rendering — not a standalone `grove tui`)

## Context

ADR-0014 chose an **in-process pty embed** (`tui-term` + `portable-pty` + `vt100`,
in the `harness-pane` crate) as the harness backend (decision **D2**), rejecting
the "owned multiplexer" alternative. But that rejection evaluated only **tmux**:
020-research studied tmux-ownership and ADR-0014 framed the call as tmux-owner vs
embed, folding **zellij** into the generic "owned multiplexer" bucket and
rejecting it on *tmux's* weaknesses (prefix-collision tax, external dependency,
`~/.tmux.conf` interaction). The root brief had named D2 as "tmux **vs zellij** vs
in-process pty"; zellij — a materially different, plugin-first, ratatui-renderable,
MIT-licensed multiplexer — was never evaluated on its own terms. Leaf 060/020
reopened exactly that comparison.

The reframe that reopened it: grove's "single binary / walk-away" identity is **not**
threatened by zellij, because a **head binary** (`grove`/`grove tui`) can launch
zellij with grove's own bundled config + layout — it *looks* like a single binary
regardless of process count. The concern was never packaging; it was **UX/DX**.

A throwaway **Strategy-1b spike** (zellij 0.44.3) tested the only blocker common
to every zellij strategy — **keybinding/control collision + chrome tameability** —
with grove's real v1 `grove tui` as a native dashboard pane beside a real harness
shell, under a tamed grove config. Headless probes all passed (config well-defined;
bars-free layout drops tab/status bars; startup float suppressed; external
`zellij action` control proven incl. stable-ID pane addressing). The human visual
pass — does `default_mode "locked"` let claude/nvim run exactly as bare, does the
composite read as grove not zellij — returned **"all ok."**

The decision burden was pre-committed: **zellij is preferred; the spike hunts for a
blocker; ambiguous-or-better resolves toward zellij.** (Prior investment in the
010 embed crate was explicitly excluded as a factor.) No blocker surfaced.

## Decision

grove's v2 harness/presentation substrate is a **grove-owned zellij multiplexer**,
not an in-process pty embed.

- A **head binary** launches zellij with grove's bundled config and a bars-free
  KDL layout, presenting as a single binary.
- The **dashboard is a native zellij terminal pane** running grove's own ratatui
  (the evolving v1 `grove tui`) — **Strategy 1b**. It drives the layout from
  *outside* by shelling out to `zellij action` (`new-pane`, `focus-pane-id`,
  `close-tab-by-id`), which is grove's existing shell-out-to-`grove` write idiom.
- **Harnesses are native zellij panes** running `grove do <name>` — zellij emulates
  them; grove does not.
- zellij runs in **`default_mode "locked"`** so every key passes through to the
  focused app; the single control seam is the (remappable) unlock key. Chrome is
  hidden via `pane_frames false` + `simplified_ui true` + a bars-free layout +
  `show_release_notes/startup_tips false`. Command panes use `start_suspended
  false` so the dashboard auto-runs.
- **The WASM-plugin dashboard (Strategy 1a — `zellij_widgets` + zellij pipe IPC to
  the head binary) is a recorded future refinement, not a v2 commitment** — to be
  revisited only if CLI-driving chafes (grove constraint 4: lazy/optional).

ADR-0013's core↔presentation boundary is **unchanged and reinforced**: the
dashboard ratatui stays *above* the seam, `RepoView`/`MultiRepoView` + shell-out
writes stay *below* it; the substrate (now zellij) sits below/around the
presentation exactly as ADR-0013 anticipated for D2. ADR-0013's deferral of a
*grove-built* web front-end also stands — zellij's bundled web client is a
property of the substrate, not a grove web server, and does not reopen that axis.

## Consequences

**Won (for free, native on every pane):** copy mode, scrollback, search, floating
panes, session persistence (detach/reattach), and a web client — the bundle the
embed would have had to build or forgo. Consistent across dashboard + harness
panes.

**The tmux prefix tax does not apply.** zellij is modal: in locked mode it is a
transparent passthrough, so claude/codex get their Ctrl/Alt chords and vim gets
Esc + everything. grove needs none of zellij's interactive keybindings.

**Reshaping of the 060 tree:**
- **030-scrollback-copy evaporates** — copy mode, scrollback, and search are
  native zellij. Retired as moot.
- **040-grove-integration is rewritten** around the head-binary + bundled
  config/layout + `zellij action` driving model (1b), replacing "grove consumes
  the `harness-pane` crate in `src/tui.rs`."
- **070-fleet-view** is unaffected at the data layer (`MultiRepoView`); its
  harness panes are native zellij panes, opened cross-repo via `zellij action`.
- **080-async-revisit** is softened further: pty output juggling is no longer
  grove's problem (zellij owns emulation), so the residual async question shrinks.

**The `harness-pane` crate (010, ADR-0014) is shelved as a recoverable fallback.**
It is not on the v2 path and not built on further (030's copy mode, which it owed,
is now moot). It stays in-repo and in history as proven evidence that the embed
works — the fallback if a future requirement defeats the zellij substrate. Its
glossary terms are re-scoped to "fallback," not "the mechanism."

**New costs accepted:** an external **zellij** runtime dependency (bundled/launched
by the head binary), grove tracking zellij's config/layout/`action` CLI surface
across versions, and the head-binary launch/teardown choreography (040's work).
These are judged smaller than building copy mode + scrollback + search + a web
client by hand and owning a bespoke emulator's fidelity edge cases.

## Notes

- Spike config knobs worth carrying into 040 (validated on zellij 0.44.3):
  `default_mode "locked"`, `pane_frames false`, `simplified_ui true`,
  `show_release_notes false`, `show_startup_tips false`, `copy_on_select true`,
  `session_serialization true`; a custom top-level `layout { … }` with no tab/status
  bar panes; command panes need `start_suspended false`; pane control via
  `zellij action new-pane/focus-pane-id/close-tab-by-id`; the spike head binary was
  a `zellij --config … --new-session-with-layout … --session …` launcher.
- Builds on **ADR-0013** (presentation boundary), which stands. Supersedes
  **ADR-0014** (the embed remains the documented fallback, not the live path).
- The throwaway spike (`/tmp/grove-zellij-spike/`) is disposable like the 050
  prototype; its findings live in 060/020's running log and this ADR.
- Resist re-litigating **tmux** — that path stays retired (ADR-0014). What 020
  resolved was the narrower, never-run **zellij** comparison.
