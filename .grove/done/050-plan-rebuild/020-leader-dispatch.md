# 020-leader-dispatch

**Kind:** work

## Goal

Refactor the focus state machine to the **leader-dispatch** model settled in 010:
generalise `Harness` → `Pane` (any focused foreign rmux pane), add a transient
`LeaderPending` state and a `Detail` focus peer, and render the **whichkey
footer** (the live leader menu when pending; the focused surface's hint line
otherwise). This is the focus spine the composed working-set layout (050) and the
detail surface (030/040) build on.

## Context

Today `src/tui/focus.rs` is `arbitrate(Focus, Leader, Event) → (Focus, Action)`
with `Focus::{Harness, Nav, Modal}`; the leader flips `Harness → Nav` **directly**,
and Nav doubles as the command menu (`c`/`e`/`q`/`Enter`). 010 settled the
**composed layout** (harness + detail coexist on screen), which needs lateral
focus movement and a uniform command gate rather than a single full-screen flip.

The new model (010):

- **`Focus = Pane | Detail | Nav | Modal` + transient `LeaderPending`.**
- **`Pane`** is today's `Harness` arbitration *verbatim* (forward every key but
  the leader). It generalises to *any* foreign pane because the aux term/yazi/vcs
  panes (050) are also rmux panes; `self.focused` (the `name → PaneEntry` map key
  in `src/tui/app.rs`) says which pane is focused.
- **Leader = dispatch gate.** Leader → `LeaderPending`; the *next* key dispatches:
  `g`→Nav, `d`→Detail, `c`→Capture (Modal), `e`→OpenEditor, `q`→Quit, `Esc`→cancel
  back to the prior pane. (Aux-pane keys `t`/`y`/`v` are **050** — leave room in
  the menu; do not wire panes that don't exist yet.)
- **Whichkey** (010 verdict): **not a surface** — the `App` draws a footer line.
  When `LeaderPending`, it is the live leader menu
  (`g nav · d detail · c capture · e editor · q quit · ⎋ cancel`); otherwise it is
  the focused surface's own hint line (the `footer_line` concept). The ADR-0019
  single-hint-owner rule holds **by construction** (one draw loop, one footer) —
  no publish/subscribe, no injected pane, no host-driver seam. Whichkey collapsed
  far enough that it earns no leaf of its own; it lives here.

## Done when

- `Focus` is `Pane | Detail | Nav | Modal` + `LeaderPending`; `arbitrate`
  implements the dispatch gate; the harness-forwarding behaviour is preserved
  under `Pane`. Nav's in-surface keys (`j`/`k`/`Enter`) stay; the leader-prefixed
  actions (`c`/`e`/`q`) move onto the dispatch gate. Every transition is pure and
  unit-tested in the existing `focus.rs` style.
- The whichkey footer renders in `App::draw`: the leader menu when pending, the
  focused surface's hint line otherwise — headless buffer-tested.
- `grove tui` keeps parity through the new gate: leader→g reaches nav, leader→c
  the capture modal, leader→e the editor, leader→q quits.
- `leader → d` wires the `Focus::Detail` transition; the panel itself may be a
  stub until 030 builds the widget (no-op render is fine).

## Notes

- Decide whether to literally rename `Harness`→`Pane` or keep `Harness` as the
  sole pane variant for now. The *requirement* is that the dispatch table must not
  assume a single pane. Recommend the rename now (cheap; 050 adds aux panes).
- `ModalKind`/capture mechanics and the `OpenEditor`/`Quit`/capture `Action`s are
  unchanged — only the *route to them* moves onto the gate.
