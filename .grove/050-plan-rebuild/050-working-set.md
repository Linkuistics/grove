# 050-working-set

**Kind:** planning

## Goal

Settle and grow the **working set** — the multi-pane layout shown for one grove:
the harness pane plus aux panes (plain terminal / yazi files / lazygit-or-lazyjj
vcs), with **park-alive** (off-screen panes stay live) and **responsive tiers**
(the ~220-col breakpoint). The biggest rebuild area; depends on the surface +
focus model from 010 (built in 020-leader-dispatch / 030-detail-widget /
040-detail-triage — the `Pane | Detail | Nav | Modal` + leader-dispatch gate) and
the session/pane model from the root 030-engine node (E3).

## Context

030 gives exactly one rmux pane per grove (the harness), addressed by a stable
`PaneId` via the `name → PaneDriver` map (ADR-0028 E3). The working set generalises
that to several panes per grove and several groves' sets coexisting off-screen.
Two trellis-era machineries are candidates to **evaporate** under rmux:

- **Park-alive.** ADR-0023 built suppress/restore + `replace_pane` to keep a
  non-displayed harness pty alive inside zellij. rmux keeps panes alive in
  detached windows/sessions **natively** — a deselected grove's panes simply are
  not drawn, but the daemon keeps them running. Does this delete the ADR-0023
  machinery wholesale?
- **Responsive tiers / harness-dominant layout.** The ADR-0022 "wide vs laptop"
  two-tier model was a zellij-layout concern. Under rmux grove draws every rect
  itself, so tiers become pure ratatui layout math over the content region width.

## Areas to grill (questions, not answers)

- **Pane model.** Are aux tools (term/yazi/vcs) separate rmux panes grove embeds
  via `PaneWidget` (like the harness), or something lighter? One rmux session with
  N panes vs per-grove sessions (ties to 030's session question).
- **Park-alive mechanism.** Confirm rmux's native keep-alive (detached
  windows/panes) replaces ADR-0023; what, if anything, grove must still track
  (the `name → PaneDriver` map already holds live handles for off-screen panes).
- **Layout + responsive tiers.** Constant nav + content region (from 010) holding
  harness-dominant + stacked aux column; the ~220-col breakpoint; per-pane toggle
  UX. Pure ratatui layout now — what survives of ADR-0022's two-tier rule?
- **Aux pane lifecycle.** How term/yazi/vcs launch (cwd = worktree), when they
  spawn (eager vs on-toggle), and how they relate to detail (010): is detail a
  member of the working set or chrome beside it?
- **Detail placement.** Resolve the seam flagged by 010 — where per-grove detail
  sits among the aux panes.

## Done when

The pane model, park-alive verdict, layout/responsive-tier model, and aux-pane
lifecycle are settled; work leaves are grown; the ADR-0022/0023 "what survives"
verdict is recorded (feeds the 040 teardown).

## Notes
