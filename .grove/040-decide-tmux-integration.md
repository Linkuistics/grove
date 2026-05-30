# 040-decide-tmux-integration

**Kind:** planning

## Goal

Consume the 020 research and **decide** grove's tmux-integration mechanics, then
grow the implementation tree for the harness pane. Settle the open mechanics
deferred from 010-plan D3: dedicated socket, control-mode vs scripting, the
`grove tui` launch/attach sequence, and the tmux config grove ships for its
owned session. Raise ADR(s) for the durable, hard-to-reverse choices.

## Context

- Inputs: 010-plan D2/D3 (the owner model + TUI-as-window-0 decision) and
  `docs/research/tmux-owning-frontends.md` (from 020).
- v1 launch path to change: `src/tui.rs::run` (currently just sets up a terminal
  and renders). Under the owner model it must detect-or-create grove's tmux
  session and attach.
- Likely-ADR candidates: socket isolation strategy; control-mode vs scripting;
  whether `grove tui` becomes a tmux launcher (a surprising identity shift worth
  recording).

## Done when

- The mechanics questions (020 Synthesis Q1–Q4) are decided and recorded in a
  running log here, with ADR(s) for the durable ones (cite the 020 research by
  primary source in each ADR's rationale).
- A short design spec exists (e.g. `docs/specs/tmux-integration-design.md`) if
  the design is large enough to warrant one.
- The harness-pane leaf (040) is decomposed if 030 reveals it needs >1 session,
  or its brief is sharpened with the decided mechanics if it fits in one.

## Notes

This is the planning leaf that turns research into a binding design. Keep
`CONTEXT.md` updated inline if new terms resolve (e.g. a name for grove's owned
session / socket).

## Decisions (running log)

### D3 — presentation layer: TUI now, web later (ratified)

**Decision.** v2 is built as the **Ratatui TUI** on the existing sync stack
(fleet view + harness pane as planned), with all grove logic kept below an
explicit **core↔presentation boundary** so a future web server is a *second
presentation over the same core*, not a rewrite. Web is a deferred,
deliberately-preserved option — not rejected.

**Why.** Follows 030's comparison
(`docs/research/web-frontend-vs-tui-presentation.md`). Web's two primary draws
are real but narrow: richness lands in dashboard *chrome*, not the harness panes
(terminal emulation in both — the only realistic web bridge, pty-wrapper (C),
re-derives the TUI's own rendering model), and the tmux backend is already
ssh/mosh-attachable in both models. Against that, web imposes decisive costs on
four load-bearing grove values — walk-away-ability (one binary vs server +
browser + build + port), persistence simplicity (a new long-lived server is
where the prior-art failure surface concentrates, §3.2), dependency budget
(forces `tokio`/async — the 070 refactor — as a *precondition*, not additive),
and near-term effort (TUI extends v1; web adds a whole parallel surface). The one
clear web win on a primary driver is the fleet view (§2.6), buildable in a TUI
(k9s/gh-dash), not enough to overturn the rest.

**Consequences.** 050 (harness-pane) and 060 (fleet-view) keep their TUI shape on
the sync stack. 070 narrows to "sync suffices for the TUI; web ⇒ async is the
entry toll" and defers. Raises an ADR for D3 + the boundary (see below).

### Boundary — principle + module placement (not a speculative trait)

**Decision.** The core↔presentation boundary is recorded as a binding *principle*
in the D3 ADR, enforced cheaply by **module placement**: tmux driving lives in
its own module (`src/tmux.rs`, working name) that takes and returns plain data
and never imports `ratatui` types; `src/repo_view.rs` is already
presentation-agnostic; `src/tui.rs` sits *above* the seam and calls down. No
`GroveBackend` trait or other web-shaped abstraction is introduced in v2.

**Why.** The only v2 consumer is the TUI, so a formal backend trait would be
designed against a non-existent web consumer and almost certainly over-fit the
TUI's needs (grove constraint 4: lazy/optional; driving-guide "runaway tree"
anti-pattern). Module placement is the cheapest rule that still bites — it keeps
tmux calls out of ratatui event handlers (the drift failure mode) and is
*validated by 050 actually using the module*, not by a hypothetical second
caller. The web-shaped interface is deferred until web is real, when its true
needs are known.

**Consequences.** 050 introduces `src/tmux.rs` as plain-data-in/out. Reviews
check the no-`ratatui`-below-the-seam rule. "Web later" becomes a second
presentation calling the same modules, not a rewrite — the boundary is proven the
day that second caller appears.

### tmux mechanics (020 Synthesis Q1–Q4)

> **Numbering note.** The "tmux mechanics" and "layout model" entries below were
> written before the 050 spike was inserted; their bare leaf numbers use the
> *old* numbering (harness-pane = 050, fleet = 060, async = 070). Post-insert the
> harness-pane leaf is **060**, fleet **070**, async **080**, and the new spike is
> **050**. These entries are also **conditional on the spike picking tmux** — see
> the "D2 REOPENED" entry below, which is the controlling conclusion.

**Q1 — dedicated socket: `tmux -L grove` (settled by evidence).** Not the default
server. Default-server reuse produced 020's worst bugs: env-var non-inheritance
(cs #277, whose own proposed fix is "use a dedicated socket"), `kill-server`
collateral damage on the user's whole tmux (cs `clean_hard.sh`), and user-config
bleed-through breaking window targeting (uzi #11). A private socket isolates
kills, guarantees a known base-index/config, and never touches `~/.tmux.conf`.
*Residual for 050:* a fresh server captures the launching process's env at start;
later windows still face tmux's `update-environment` allowlist — pass needed env
explicitly at `new-window` time (`-e`) rather than relying on inheritance.

**Q2 — plain scripting, not control mode `-CC` (settled by evidence).** None of
the surveyed orchestrators/managers use `-CC`; only iTerm2 does, and pays heavily
(stateful `%begin/%end/%output` stream parser, no version-negotiation handshake,
octal decode, ~50% window-launch failures, TTY-on-stdin hostility to servers).
grove's dashboard *is* a tmux window (D3), so grove needs none of what `-CC` buys
(native rendering of tmux windows) — tmux renders them. The one real benefit
(pushed window-close events) is obtainable far more cheaply by polling
`list-windows` (a 050/Q5 concern).

**Q4 — ship a minimal `grove.conf`, launch `tmux -L grove -f grove.conf`
(settled by evidence).** Socket + config travel together: the private socket only
pays off with a private config. `grove.conf` pins `base-index`/`pane-base-index`
(kills uzi #11 by construction), sets grove's own status line and a **prefix that
does not collide with the user's `C-b`** (working choice TBD in 050), and never
reads/clobbers `~/.tmux.conf`.

**Q3 — `$TMUX` launch handling: refuse-and-instruct for the MVP (open decision,
now settled).** The managers' `$TMUX`-set → `switch-client` answer does **not**
transfer: `switch-client` cannot move a client across sockets, and grove's
`-L grove` is a different server. Chosen among refuse / nest / hybrid:
- **Outside tmux** (`$TMUX` empty): `tmux -L grove attach` (or `new-session` on
  first run). Clean, no decision.
- **Inside the user's tmux** (`$TMUX` set): **refuse and instruct** — print a
  clear message to detach and re-run from outside (or `tmux -L grove attach`
  manually), exit non-zero. Chosen for MVP simplicity: zero nesting edge cases
  for 050 (the first validator), and refuse→nest is purely *additive*, so this is
  the low-regret default. *Known cost:* a papercut for users who live inside tmux
  permanently; relax to deliberate nesting later if it proves annoying (grove's
  owned prefix already neutralizes nesting's functional objection, leaving only
  visual nesting).

### Resilience reframe — artifacts, not tmux persistence (corrects D2's framing)

**Clarification (user steer).** grove's crash-resilience comes from the
**artifacts-over-state model** (spine constraint 1): you can always restart a
grove and it works out the next step from the task tree + git — a developer
taking over someone else's work with no handover. The tmux-server ownership does
**not** earn its place as a *durability* guarantee.

**What this changes.** 010-plan D2 / 020 Q6 framed "harness survives the
dashboard dying" as the headline justification for grove owning a tmux server.
Demote that: tmux persistence is a **convenience** — it spares re-establishing
the *live interactive* session (scrollback, an in-flight claude/codex REPL) — not
a correctness property. A harness pane dying is recoverable by `grove do <name>`,
which re-derives state from artifacts. The tmux-backend ADR (0014) states the
persistence claim at this strength, not stronger.

**Consequences.** (1) The layout mechanism is a pure **ergonomics** decision, not
constrained by persistence-granularity. (2) `join-pane`/`break-pane` fragility is
acceptable — an orphaned harness just resumes via `grove do`. (3) Running N
harnesses at once is a *parallel-work* feature (watch several agents), distinct
from resilience. (4) 070 (async-revisit) keeps its lean shape: nothing here adds
event-loop pressure.

### Layout model — single window, dashboard-as-switcher + harness panes

**Decision (intent locked; concrete mechanism is a 050 spike).** The harness pane
layout is **single-window**, not separate full-screen windows:

- **One tmux window** in grove's session holds a persistent **dashboard pane**
  plus the **harness panes** (one per active grove, all alive for parallel work).
- The **dashboard pane is the navigation/switch surface** — the user picks which
  working grove to focus *in the dashboard*, not via tmux's own window switching.
  It must stay reachable to switch (so "zoom a harness forever" is not the model;
  you return to the dashboard to choose the next grove).
- The active grove's harness is shown **alongside / focused** from the dashboard.
  "harness pane" is therefore the **correct** term — a tmux *pane* (split) showing
  the live `grove do <name>` session, not a separate window.

**Key tmux finding driving the mechanism (for 050).** A tmux client shows one
window at a time; the only ways to *hide* a pane are zoom (`resize-pane -Z`, which
fills the whole window and hides the dashboard too) or move it out via
`break-pane`. So "dashboard + one *full-size* harness, others hidden" is **not**
plain zoom — it needs `join-pane`/`break-pane` choreography (park inactive
harnesses in hidden windows, join the selected one beside the dashboard). Two
candidate mechanisms, to be **prototyped as 050's first spike**:
  1. *Zoom-toggle (simplest MVP):* overview = dashboard pane + tiled harness
     panes (navigate/switch here); zoom one harness to work; unzoom to return to
     the dashboard. Pure tmux, no choreography. Cost: harness tiles are cramped in
     overview when many groves are active; dashboard + a *full-size* harness are
     not simultaneous.
  2. *join-pane/break-pane (matches the drawn model):* dashboard + exactly one
     harness always visible, inactive harnesses parked in hidden windows. Cost:
     join/break-pane is the fiddly part of tmux (focus/resize churn) — acceptable
     now that resilience is the artifact model, not pane survival.

**Recommendation for 050:** start with (1) zoom-toggle; escalate to (2) only if
the cramped-overview tiles prove inadequate in practice. The intent above is
binding; the mechanism is 050's to settle by building.

### D2 REOPENED — harness backend (tmux vs in-process pty) goes to a spike

**This entry recontextualises everything above it.** The resilience reframe (that
tmux persistence is a *convenience*, not the resilience mechanism) removed the
headline justification 010-plan D2 used to choose tmux-owner *over* in-process
pty. The user observed: "if we could reliably use ratatui with an embedded claude
code/codex session I wouldn't be suggesting tmux — but I thought that was too
difficult." That assumption is the load-bearing one, and it is **testable and
possibly stale**:

- **maestro-tui** (lib.rs/crates/maestro-tui) is "a dual-pane terminal
  multiplexer… that puts shell and Claude Code side-by-side" — built on Ratatui
  with in-process PTY management. Almost exactly grove's headline feature, without
  tmux.
- **Turborepo** uses `tui-term`'s `vt100` render path in its production UI
  (vercel/turborepo PR #9123 is a perf pass on it) — real mileage on the emulator.
- But `tui-term` is WIP (its docs: "active development", consumer wires input
  itself, historically oneshot-oriented), `vt100` is its only emulation backend,
  and embedding vim in Ratatui has documented input-hang failures. So: viable, not
  turnkey.

**Decision.** D2 is genuinely reopened and resolved **empirically**, not by
assumption. A spike leaf `050-spike-embed-pty-harness` (inserted ahead of the
harness-pane leaf, now 060) builds a minimal `tui-term` + `portable-pty` embed
pointed at a real harness session, evaluates fidelity (colors / cursor /
alt-screen / mouse / resize / unicode width / OSC), input wiring, and resize;
studies maestro-tui; and decides D2 — writing the **backend ADR** (the held
ADR-0014: either *tmux-owner backend* or *in-process-pty backend*) on the result.

**Status of the tmux mechanics above.** D3 + the core↔presentation boundary are
ratified now (**ADR-0013**, written). Everything in this log under *tmux mechanics
(Q1–Q4)* and *layout model* is **conditional on the spike picking tmux** — it is
the worked-out plan *if* tmux wins, held here, deliberately **not** ratified.
**ADR-0014 is HELD** until the 050 spike decides. If the spike picks in-process
pty: Q1–Q4 (socket, `-CC`, `$TMUX`, `grove.conf`) become moot; the single-window
layout becomes trivial native Ratatui widgets (no zoom/join-pane choreography, no
`$TMUX` nesting); and async (080) reopens on the pty-output-juggling grounds the
tmux model had deflated — the 050 spike flags that.

## Outcome (what 040 produced)

- **ADR-0013** — v2 presentation = TUI behind a core↔presentation boundary; web
  deferred. (Written; mechanism-agnostic, robust to the D2 spike.)
- **CONTEXT.md** — added `Dashboard`, `Harness pane` (mechanism-neutral),
  `Presentation boundary`. tmux-specific terms (`grove socket` / `grove session`)
  deliberately **not** added — they only exist if the spike picks tmux.
- **Tree grown** — inserted `050-spike-embed-pty-harness` (decides D2, writes the
  backend ADR) ahead of the harness-pane leaf (renumbered 050→060; fleet 060→070;
  async 070→080); sharpened the harness-pane brief (060) to depend on 050's
  verdict.
- **Held** — ADR-0014 (backend mechanics), pending the 050 spike.
- **No design spec** — folded into ADR-0013 + this log + the 050/060 briefs
  (grove ethos: lazy/optional artifacts).
