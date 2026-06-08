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

## Decomposition (the five areas → four leaves + one deferred grove)

- **010-surfaces** (planning) — nav / per-grove detail / capture / whichkey as
  plain ratatui widgets. What survives of the ADR-0019 "A′" UX (constant nav +
  swapped content)? Nav + capture exist (030); detail + whichkey are the new
  build. First, because it is the natural next functional increment and sharpens
  the surface/focus model the working set depends on.
- **020-working-set** (planning) — multi-pane layout, aux panes (plain term /
  yazi / lazygit-lazyjj), park-alive (do rmux splits/sessions replace the
  ADR-0023 suppress/restore + `replace_pane` machinery?), responsive tiers
  (~220-col breakpoint). The biggest area; depends on the surface/session model.
- **030-daemon-launch** (planning) — bundling/shipping the **stock** rmux daemon
  binary (ADR-0029 — *no fork*; grove ships published rmux 0.5.0's daemon+CLI)
  via `SDK_DAEMON_BINARY_ENV` / `connect_or_start`, session naming/persistence,
  how `grove tui` launches, fleet singleton + multi-repo (ADR-0025/0027) under
  rmux. Makes `grove tui` a walk-away binary. Independent enough to pull earlier
  if a shippable artifact is wanted sooner.
- **040-teardown** (work) — dissolve the ADR-0013–0028 tower (D4 mark-Superseded
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
030-daemon-launch leaf inherits the corrected framing.
