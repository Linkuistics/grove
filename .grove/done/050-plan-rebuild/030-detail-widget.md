# 030-detail-widget

**Kind:** work

## Goal

Build the per-grove **detail** surface as a grove-drawn ratatui widget (not a
proxy): the focused grove's **task tree** (live leaves + `done/`), **brief chain**,
and **inbox view** (pending count + list), drawn purely from `RepoView`. Focusable
via the leader-dispatch gate (`leader → d`), `j`/`k` scroll, `Esc` → back to the
pane. Place it as a panel **beside** the harness (minimal split; the full
responsive layout is 050).

## Context

010 settled the composed layout (harness pane + detail panel coexist) and that
detail is a **widget grove draws from `RepoView`**, dissolving the ADR-0016
dumb-proxy / `grove __dash-proxy` / socket-seam mechanism. Detail is the *only*
grove-drawn panel besides nav/whichkey/modal; the aux term/yazi/vcs panes (050)
are foreign rmux panes.

020-leader-dispatch added `Focus::Detail` and the `leader → d` transition (stub).
This leaf builds the panel:

- **Source.** The presentation-agnostic core already exposes the data —
  `RepoView`/`MultiRepoView` (grove list, lifecycle, inbox-pending counts, the
  `.grove/` task-tree walk; the nav already projects these). Brief chain follows
  the `grove-llm brief-chain` walk semantics (ancestor `BRIEF.md`s root→leaf).
  Build detail as a **pure** `&RepoView/&GroveView → Buffer` widget
  (`src/tui/detail.rs`), mirroring `nav.rs`'s headless-tested style.
- **Placement.** A minimal two-panel split of the content region (harness the
  dominant share, detail a side column) — just enough to prove coexistence +
  lateral focus. The **responsive tiers + aux-pane placement are 050** — the seam
  010 flagged; `050-working-set` "Detail placement" resolves where detail sits
  among the aux panes.

## Done when

- `src/tui/detail.rs` renders the focused grove's task tree (live + `done/`),
  brief chain, and inbox view as a pure snapshot→Buffer widget, headless-tested.
- `App::draw` shows detail beside the harness in the composed layout;
  `Focus::Detail` highlights the detail panel and routes `j`/`k` scroll;
  `Esc`/leader-gate returns to the pane.
- Detail tracks the **focused grove** (the `self.focused` pane's grove); switching
  groves via nav re-points detail. The bare-shell pane (no grove) shows an
  empty/"no grove" detail.
- fs-watch ticks refresh detail (task tree / inbox changing) like they refresh
  the nav.

## Notes

- Capture entry already exists (leader→c writes to the focused pane's grove);
  detail's "[c] capture" is a hint, not new wiring.
- Triage actions are the next leaf (040). This leaf is read-only render + focus +
  scroll.
