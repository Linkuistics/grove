# 050-plan-rebuild — brief

Plan + build the rest of the rmux rebuild past the minimal-usable engine (030)
and open-in-editor (040). This node carries the cross-cutting rebuild context;
each area below is grilled (or built) in its own child-leaf session.

## Goal

Reach the brief's **interim-parity** scope (root `BRIEF.md` "Done when") on the
rmux substrate: the full surface set (nav + per-grove detail + capture + whichkey
as ratatui widgets), the working set (multi-pane layout, aux panes, park-alive,
responsive tiers), a shippable `grove tui` (daemon bundling + launch), and the
teardown (dissolve the ADR-0013–0028 tower, retire the `bugs` grove, glossary
cleanup). The detach + web path is **out of this grove's scope** — seeded as its
own follow-up grove.

## Context

010-plan settled the cross-cutting engine decisions (D1–D6); 030-engine landed a
*usable* rmux `grove tui` (harness pane, leader-gated focus, nav surface, centered
capture modal — all headlessly testable) and 040 wired open-in-editor over stock
`rmux capture-pane` (ADR-0029, no fork). With the engine in hand the remaining
areas sharpen: the focus model, the `name → PaneDriver` addressing (E3), and the
"grove draws its own surfaces" inversion (ADR-0028) are now concrete, not
hypothetical.

The recurring theme across these areas: under the inversion most of the old
trellis-era machinery **evaporates rather than ports**. Per-grove detail and
whichkey were dumb proxies / injected host panes *because grove lived inside
zellij*; now they are widgets grove draws. Park-alive was the ADR-0023
suppress/restore + `replace_pane` dance; rmux keeps panes alive off-screen
natively. Each area's grilling should separate the **UX intent that survives**
(mostly the ADR-0019 "A′" model) from the **mechanism that dissolves**.

## Decomposition (four areas → seven leaves + one deferred grove)

- **010-surfaces** (planning, **settled**) — what shape the surface set takes
  under the inversion. Verdict (see decision log): a **composed layout** (harness
  pane + detail panel coexist), a **`Pane | Detail | Nav | Modal` + leader-dispatch**
  focus model, **whichkey collapses to an `App` footer**, and **detail is a widget
  grove draws from `RepoView`**. Grew the three work leaves below.
- **020-leader-dispatch** (work) — the focus spine: generalise `Harness`→`Pane`,
  add `LeaderPending` + a `Detail` peer, the leader-dispatch gate, and the
  whichkey footer (leader menu when pending / surface hints otherwise). Whichkey
  earned no leaf of its own — folded here.
- **030-detail-widget** (work) — the per-grove detail surface as a ratatui widget
  (task tree + brief chain + inbox view) drawn from `RepoView`, focusable + scroll,
  placed beside the harness (minimal split; full layout is 050).
- **040-detail-triage** (work) — interactive inbox **grooming** in detail: reject
  + move/re-route, shell-outs below the seam (E1) under `spawn_blocking`.
- **050-working-set** (planning) — multi-pane layout, aux panes (plain term /
  yazi / lazygit-lazyjj), park-alive (do rmux splits/sessions replace the
  ADR-0023 suppress/restore + `replace_pane` machinery?), responsive tiers
  (~220-col breakpoint). The biggest area; depends on the surface/session model.
  Resolves the **detail-placement seam** 010 flagged (where detail sits among the
  aux panes) and wires the aux panes into the leader-dispatch gate (`t`/`y`/`v`).
- **060-daemon-launch** (planning) — bundling/shipping the **stock** rmux daemon
  binary (ADR-0029 — *no fork*; grove ships published rmux 0.5.0's daemon+CLI)
  via `SDK_DAEMON_BINARY_ENV` / `connect_or_start`, session naming/persistence,
  how `grove tui` launches, fleet singleton + multi-repo (ADR-0025/0027) under
  rmux. Makes `grove tui` a walk-away binary. Independent enough to pull earlier
  if a shippable artifact is wanted sooner.
- **070-teardown** (work) — dissolve the ADR-0013–0028 tower (D4 mark-Superseded
  sweep + finalise the landmark/focused ADRs), retire the `bugs` grove, clean up
  `CONTEXT.md` (the superseded trellis/proxy/host-surface entries). Near the end,
  so we are not marking ADRs superseded while still discovering what survives —
  though glossary cleanup MAY be folded in incrementally as each area resolves
  its terms.

**Deferred to its own grove — Detach + web path.** rmux session
persistence/detach and rmux `web-share` (HTTP share URLs) vs grove's
whole-UI-on-web goal. Genuinely open-ended and research-first, and **not** on the
interim-parity critical path the root brief scopes. Seeded to the **`rmux-web`**
grove via `grove-llm inbox-add` rather than carried as a leaf here.

## Done when

The four leaves are settled (each grilled/built in its own session and retired);
the detach+web path is seeded elsewhere; interim parity is reached and the ADR
tower + `bugs` grove + glossary are torn down. A PRD is written only if an
increment is a genuine human-facing agreement point.

## Working-set teardown inputs (for 070, promoted on 050-working-set retirement)

The 050-working-set node is **built and retired** (keying → side-column layout →
aux panes, all on rmux's native park-alive). The full rationale lives in the
archived node brief (`.grove/done/050-plan-rebuild/050-working-set/BRIEF.md`);
the **ADR markings 070-teardown must apply** are lifted here so they stay on the
live path:

- **ADR-0022 — UX survives, two mechanisms dissolve.** Survives: harness-dominant
  layout + an aux side column + a width breakpoint (now a single ~220-col
  breakpoint governing column *geometry only*). Dissolves: the constant-nav
  *region* pin (already retired in 010-surfaces) and *tier-as-mount-time-
  membership* (membership is now user-driven lazy-toggle, Q3/Q4). Mark **amended /
  mechanism-superseded, UX survives** — not blanked.
- **ADR-0023 + ADR-0024 — evaporate wholesale (mechanism-dissolved).** The
  suppress/restore + `replace_pane`-into-`suppressed_panes` machinery (0023) and
  the suppress/restore exit-observability (0024) have **no analogue** under the
  inversion: a pane in a detached rmux window stays alive whether or not grove
  draws it (park = do nothing). ADR-0024's *concern* survives via the SDK
  (`render_stream` closing / `Pane::wait_exit()`), not suppress/restore. Mark
  both **Superseded (mechanism-dissolved)**.

## Decisions (running log)

**Sequencing + decomposition (050 grilling, this session).** Decompose 050 into
this node with four ordered child leaves — **Surfaces → Working set →
Daemon/launch → Teardown** — each grilled in its own session rather than settling
all five areas inline (they are large and mostly independent; inline-everything
is the mega-session anti-pattern, driving.md). Surfaces first: the next useful
increment, and it sharpens the surface/focus model the working set builds on.
Teardown last (decks-clearing once what-survives is known), with glossary cleanup
allowed to trickle in earlier. **Detach + web is deferred to its own `rmux-web`
grove**, not a leaf here — it is open-ended, research-first, and off the
interim-parity path. (User confirmed: "your recommendation.")

**Corrected stale note.** The pre-decomposition leaf text said daemon bundling
ships grove's *forked* rmux build (the original D7). **ADR-0029 superseded D7** —
there is no fork; grove ships the *stock* published rmux 0.5.0 daemon+CLI. The
060-daemon-launch leaf inherits the corrected framing.

**Surfaces (010-surfaces grilling, this session).** Settled the surface set + focus
model under the inversion, and grew three work leaves (020/030/040). User confirmed
each decision:

- **Composed layout, not flip-to.** A grove's view is a *layout* grove draws — the
  harness pane and the **detail** panel coexist on screen; focus moves laterally
  between visible panels. Nav stays a flip-to full surface, Modal an overlay.
  Owning the draw loop *upgrades* ADR-0019's "A′": detail is a coexisting panel,
  not a flip-to tab and not a dumb proxy. The aux term/yazi/vcs panes the user
  reaffirmed are foreign rmux panes in the same working set — their layout +
  membership is **050** (placement deferred per the leaf).
- **Focus = `Pane | Detail | Nav | Modal` + transient `LeaderPending`; leader =
  dispatch gate.** `Pane` generalises today's `Harness` (any focused foreign rmux
  pane; `self.focused` says which — so aux panes need no new focus variant).
  Leader → pending → next key dispatches `g`→Nav, `d`→Detail, `c`→Capture,
  `e`→Editor, `q`→Quit (`t`/`y`/`v`→aux in 050), `Esc`→cancel. Costs one keystroke
  to reach nav vs 030's direct flip; bought back in uniformity + discoverability.
- **Whichkey is a footer, not a surface.** The `App` draws one footer line: the
  live leader menu when `LeaderPending`, the focused surface's hints otherwise.
  ADR-0019's single-hint-owner holds **by construction** (one draw loop, one
  footer) — the publish/subscribe host-driver seam + injected `grove-whichkey`
  pane fully dissolve. It collapsed too far to earn a leaf; folded into
  020-leader-dispatch.
- **Detail = ratatui widget from `RepoView`** (task tree + brief chain + inbox
  view), pure snapshot→Buffer, headless-tested. The ADR-0016 dumb-proxy /
  `grove __dash-proxy` / socket seam / `RunEditor` frame all dissolve.
- **Detail triage = grooming.** Interactive **reject** + **move/re-route** only
  (shell-outs below the seam, E1). `incorporate` / `defer-to-a-leaf` have no honest
  TUI meaning (no active task) and stay at the session-bootstrap [[Drain]].

**A′-survival verdict (feeds the 070 teardown's ADR-0019 marking).** The ADR-0019
*UX intent survives* — nav always reachable (now via the leader, not a pinned
region), detail scoped per-grove. The *mechanisms dissolve*: N dumb proxies /
`grove __dash-proxy` / the controller socket seam / the `RunEditor` frame (→ detail
is a grove-drawn widget); the "constant nav **region**" pin (a zellij-era artifact,
ADR-0022 — "always reachable via leader" replaces it); whichkey-as-injected-pane +
the publish/subscribe host-driver (→ an `App` footer). Mark ADR-0019 **amended /
mechanism-superseded, UX survives**, not blanked.

**Whichkey earned no leaf (deviation from the leaf's "grow leaves for detail and
whichkey").** The grilling *finding* is that under the inversion whichkey is one
footer the `App` draws, intrinsically tied to the `LeaderPending` dispatch state —
building it apart from the dispatch table would be artificial. So it folds into
020-leader-dispatch (grove constraint 4: an artifact earns its place). Detail, by
contrast, splits into two work leaves (widget + triage).
