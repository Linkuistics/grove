# 050-working-set — brief

Grilled and decomposed (this node carries the settled model; the three child
leaves build it). The **working set** is the multi-pane layout shown for one
grove: the harness pane + a side column of aux tools (plain term / yazi /
lazygit-or-lazyjj vcs) and the per-grove detail widget, with **park-alive**
(off-screen panes stay live) and a **responsive breakpoint**. Depends on the
surface + focus model from 010 (the `Pane | Detail | Nav | Modal` +
leader-dispatch gate) and the session/pane model from 030-engine (E3).

## The settled model

A grove's working set is a **layout grove draws itself** (the inversion,
ADR-0028) — no rmux splits, no zellij park machinery:

- **Layout.** Harness pane (dominant left) + a **side column** stacking the
  *visible* members vertically + the footer row. The side column is a
  **heterogeneous stack**: the **detail** widget (always-present top member,
  grove-drawn) + zero-or-more **aux panes** below it in fixed order
  (term → yazi → vcs). Each `Rect` grove fills; the member kind only selects the
  render fn (`Detail::render` vs `render_pane`).
- **Pane model.** Aux panes are foreign rmux panes embedded exactly like the
  harness — one **detached rmux window** each, keyed by composite **(grove,
  role)**, rendered into a grove-chosen `Rect` via `PaneWidget`.
- **Park-alive is native + free.** A detached-window pane stays alive in the
  daemon whether or not grove draws it → ADR-0023/0024 evaporate wholesale.
- **Lifecycle.** Aux panes **lazy-spawn on first toggle** (cwd = worktree),
  **hide-not-close** on toggle-off, close only at TUI exit. **Visibility is
  per-grove**, ephemeral per session.
- **Responsive = geometry only.** A single ~220-col breakpoint sets side-column
  width + per-member min-height; ADR-0022's tier-as-membership rule dissolves.
- **Keys.** Leader gate: `t`/`y`/`v` toggle term/yazi/vcs (focus-follow);
  `d`→detail; Esc stays with the embedded tool.

Full rationale + the per-question evidence is in the running log below.

## Decomposition (three ordered work leaves)

- **010-pane-keying** — the addressing refactor: `name → PaneEntry` → composite
  **(grove, role)** (map, `self.focused`, capture target). The foundation.
- **020-side-column-layout** — generalise `composed_layout` to harness + a side
  column stacking detail + N aux slots, with the breakpoint geometry. Pure
  ratatui, headless-tested.
- **030-aux-panes** — lazy-spawn term/yazi/vcs, wire `t`/`y`/`v` into the gate +
  footer, per-grove visibility, hide-not-close. Ties it together.

## Done when

All three leaves are built + retired; `grove tui` shows a usable per-grove
working set (harness + detail + toggleable aux panes) with native park-alive and
a responsive side column. The ADR-0022/0023/0024 "what survives" verdicts are
recorded (above) for the 070 teardown to apply.

## Decisions (running log)

**Q1 — Pane model: aux tools are ordinary entries in the same pane map as the
harness (this session).** Each aux tool (plain term / yazi / vcs) is a foreign
rmux pane embedded exactly like the harness — one **detached rmux window**,
addressed by stable `PaneId`, rendered into a **grove-chosen `Rect`** via
`PaneWidget`. Identical mechanism; only the argv and the `Rect` differ.
- *Why not rmux splits:* grove draws every `Rect` itself (`composed_layout`,
  `app.rs`), so rmux's own split geometry is never displayed — splits would leave
  the daemon tracking a layout grove ignores, and `capture-pane`-by-`PaneId`
  (open-in-editor) stays cleaner per separate window. Splits earn their keep only
  if rmux owned the layout (it doesn't) or for the deferred web path (out of
  scope → `rmux-web`).
- *The refactor it implies:* the `name → PaneEntry` map key (today the grove
  name, one pane per grove) becomes a **composite (grove, role)** —
  `harness`/`term`/`yazi`/`vcs`; `self.focused` and the per-pane capture target
  widen from "grove name" to "(grove, role)". Contained but real — the honest
  shape of several panes per grove. `Focus::Pane` already generalises to "any
  focused foreign pane" (`focus.rs`), so no new focus variant is needed.
- User confirmed: "your recommendation."

**Q2 — Park-alive verdict: ADR-0023 + ADR-0024 evaporate wholesale (this
session).** The suppress/restore + `replace_pane`-into-`suppressed_panes`
machinery (ADR-0023) and the suppress/restore exit-observability (ADR-0024) have
**no analogue** under the inversion and are marked **Superseded
(mechanism-dissolved)** in the 070 teardown.
- *Park = do nothing.* A pane in a detached window stays alive in the daemon
  regardless of whether grove draws it; `render_surface` just iterates the panes
  it wants this frame. No suppress step, no restore step. grove tracks nothing
  new beyond the `name → PaneEntry` map it already holds.
- *ADR-0024's concern survives via the SDK, not suppress/restore:* "did the
  embedded tool exit?" is answered by `render_stream` closing (the render task
  ends, `driver.rs`) or `Pane::wait_exit()`. UX-survives / mechanism-dissolves,
  same marking shape as ADR-0019.
- *Residuals deferred to lifecycle/layout (Q3/Q4), not park machinery:*
  (a) **off-screen resize** — resize-on-show (a hidden pane has no `Rect` this
  frame; resize it when it next becomes visible) rather than keeping hidden panes
  sized; (b) **close/cleanup** — handled by lazy spawn (Q3), not a deliberate
  park/unpark step.
- *Resource-reclaim pushback resolved:* the "many groves × 4 panes = many live
  ptys" concern is addressed by **lazy spawn** (aux panes spawn on first toggle,
  Q3), not by retaining a park/kill step — so wholesale evaporation stands.
- User confirmed: "your recommendation."

**Q3 — Aux pane lifecycle: lazy spawn, hide-not-close (this session).**
- **Lazy spawn on first toggle.** An aux pane's window is created the first time
  the user toggles it on (leader `t`/`y`/`v`), not eagerly on grove select — a
  grove you glance at costs one harness pane, not four (the Q2 resource story).
- **cwd = the grove's worktree** (`grove_worktree(repo_root, name)`, as the
  harness uses).
- **argv:** `term` = `$SHELL`; `yazi` = `yazi`; `vcs` = **lazygit hardcoded
  now**, with a `// jj detection is a follow-up` seam (grove worktrees are git;
  lazyjj-vs-lazygit detection is a rabbit hole that doesn't block this leaf's
  layout work). Resolved below the seam, configurable later.
- **Toggle-off = hide, not close.** Toggling off stops drawing the pane (its
  render pushes stop triggering redraws, like a deselected grove); the pty stays
  warm so toggle-on is instant — park-alive applied *within* a working set.
- **Close only at TUI exit.** Spawned aux panes live until `grove tui` quits; no
  per-grove teardown (consistent with harness panes — deselecting never closes).
- **Missing tool binaries:** spawn the pane anyway and let the process fail
  visibly *inside* it (`command not found`); grove does **not** pre-check PATH or
  toast — same posture as the harness (grove doesn't validate `grove do`).
- *Cost accepted:* first toggle of each tool pays spawn latency once per session;
  cheaper than eager spawn × every grove × every tool.
- User confirmed: "your recommendations."

**Q4 — Layout + responsive tiers: one shape, breakpoint governs geometry only
(this session).** The working set is **one layout** — harness pane (dominant
left) + a **side column** stacking the *visible* members vertically (detail
widget + each toggled-on aux pane) + the footer row. Generalises today's
`composed_layout` (harness + single detail column) to a side column of N members.
- **ADR-0022's two-tier *membership* rule dissolves.** The zellij tier was
  evaluated at mount to decide *which panes to open*; under the inversion
  membership is user-driven (Q3 lazy-toggle) and detail is always shown — so a
  width threshold has no membership job left.
- **What survives: a single ~220-col breakpoint governing side-column geometry**
  (column width + per-member min-height), pure ratatui math over `content.width`.
  Wide (≥~220): comfortable column width (raise today's 48 cap), generous member
  height. Laptop (<~220): narrower column (≈today), members still stack.
- **Overflow: let it tile.** Too many members toggled on to stack legibly is a
  **known degradation the user manages via toggles** (equal vertical shares); no
  overflow/scroll machinery this leaf. (The "one-aux-at-a-time below the
  breakpoint" alternative was offered and declined — let-it-tile + direct toggle
  control preferred.)
- **Split ratios/caps are build-time tuning** — settle "harness keeps the
  dominant share, column wide enough for one foreign TUI"; exact numbers left to
  the work leaf.
- **070 marking for ADR-0022:** UX intent survives (harness-dominant, aux side
  column, width breakpoint); two mechanisms dissolve — the nav-region pin
  (already retired in 010-surfaces) and tier-as-mount-time-membership (here).
- User confirmed: "your recommendations."

**Q5 — Detail placement: always-present top of the side column (this session).**
Detail is the **always-present top member** of the side column; toggled-on aux
panes stack **below** it in fixed order (term → yazi → vcs).
- Detail is qualitatively different from the aux trio — grove-drawn (no pty,
  spawn cost, or PATH dep), grove-specific info (task tree / brief chain /
  inbox), already a first-class `Focus::Detail` peer. Always-on + top gives the
  side column a **stable anchor** (the user always knows where "this grove's
  status" is). Detail does **not** toggle off (uniformity-with-aux alternative
  declined in favour of the stable anchor).
- The side column is a **heterogeneous stack**: one widget `Rect` (detail, via
  `Detail::render`) + zero-or-more pane `Rect`s (aux, via `render_pane`). Both are
  just `Rect`s grove fills; the kind only selects which render fn runs — the
  honest realisation of the inversion.
- **Focus between side members is via the leader gate**, not a separate cycle
  key: `d`→detail, `t`/`y`/`v`→that aux pane. No Tab-cycle.
- User confirmed: "recommendations are good."

**Q6 — Toggle UX: toggle-with-focus-follow, per-grove visibility (this
session).**
- **`t`/`y`/`v` toggle visibility with focus-follow.** Hidden → lazy-spawn (if
  first time) + show + focus; visible → hide + return focus to harness. Common
  path ("show me vcs") is one gesture; same key again dismisses.
- **Visibility state is per-grove**, part of the working set, ephemeral per
  session (constraint 1). Switching groves restores *that* grove's toggled-on set
  — a natural consequence of the working set being per-grove.
- **Esc stays with the embedded tool** (`Focus::Pane` forwards all but the
  leader) — you leave an aux pane via the leader, preserving key fidelity.
- **Footer gains the aux keys:** the `LeaderPending` menu extends to
  `g nav · d detail · c capture · t term · y yazi · v vcs · e editor · q quit · ⎋`
  (toggled-on marker a nice-to-have; build detail).
- **Accepted gap:** with simple toggle, leader→`t` on a *visible-but-unfocused*
  term hides rather than focuses it. Accepted for now; the real fix is
  **click-to-focus hit-testing across side-column rects** — a **follow-up**
  (seeded), out of this leaf's scope (today's click only forwards into the
  already-focused pane). The "double-tap-to-dismiss" alternative (gate behaviour
  depends on app state, not the pure `Focus` type) was declined.
- User confirmed: "recommendations are good."

**Decomposition (this session).** Three ordered work leaves — **addressing →
layout → lifecycle/toggle**:
- **010-pane-keying** (work) — refactor the `name → PaneEntry` addressing from
  grove-name to **composite (grove, role)**: the map, `self.focused`, and the
  per-pane capture target. Generalises harness handling; no new panes yet. The
  foundation everything else builds on; de-risks the rest.
- **020-side-column-layout** (work) — generalise `composed_layout` to harness
  (dominant left) + a **side column stacking detail + N aux slots** + footer,
  with the ~220-col breakpoint governing column width + per-member min-height
  (geometry only). Pure ratatui `Rect` math, headless-tested; buildable before
  aux panes spawn (detail is already a member).
- **030-aux-panes** (work) — lazy-spawn term/yazi/vcs on toggle, wire `t`/`y`/`v`
  into the leader gate + footer, per-grove visibility state, hide-not-close,
  cwd = worktree, lazygit hardcoded (jj-detection seam). Ties it together into a
  usable working set.
- **No new ADR:** the model is a direct corollary of ADR-0028's inversion thesis;
  the running log is the durable build record. ADR-0022/0023/0024 markings are
  the **070 teardown's** job (verdicts recorded above).
- **Seeded follow-up:** click-to-focus hit-testing across side-column rects → the
  same TUI grove (captured this session).

## Notes
