# 030-scrollback-copy

**Kind:** work

## Goal

Build **pane-local scrollback and copy** into `crates/harness-pane`: the user
scrolls back through a pane's history and selects text **within that one pane**
— by keyboard *and* mouse — then copies it, without using the host terminal's
own selection (which spans the whole outer grid, dashboard included). This is
the feature tmux gives free via per-pane copy-mode; the in-process-pty embed
owes it (ADR-0014 / 050 outstanding gap, user-confirmed real).

## Context

- **User requirement (explicit):** scrollback must be **both key- and
  mouse-driven**, and selection happens *over that scrollback* — the point is
  selecting multiple lines within a pane, not the host terminal's selection.
- **vt100 0.15.2** carries the scrollback buffer; `set_scrollback(n)` chooses
  how many lines back the rendered `Screen` is offset. The selection/extraction
  model is *ours* to build over that buffer — vt100 stores the lines, it does
  not track a selection.
- **Why the host's selection is insufficient (050):** even with dynamic mouse
  capture released, the host terminal selects the whole outer grid (dashboard
  chrome + harness together), so dragging can't isolate one pane's text.
- Depends on **010** (`TerminalEmulator` is the surface this extends). Builds on
  its source-agnostic design: the selection/extraction logic is testable by
  feeding synthetic lines, no child needed.

## Done when

- **Scrollback navigation** works by both inputs, mapped to vt100
  `set_scrollback`:
  - keyboard (e.g. PageUp/PageDown, line up/down, jump to top/bottom);
  - mouse wheel — the pane intercepts wheel events for scrollback **when the
    focused app is not itself requesting mouse** (`mouse_protocol_mode == None`);
    when the app wants mouse, wheel passes through to it.
- **Selection model** over emulator coordinates **including scrolled-back
  lines**: an anchor+cursor range, settable by mouse drag and by keyboard;
  rendered as a **highlight overlay** on the visible cells.
- **Copy** extracts the selected text from the vt100 grid + scrollback and
  places it on the clipboard via **OSC-52** (portable default; note a
  platform-clipboard fallback as a follow-up, behind the 010 side-effect seam).
- A **copy-mode interaction model**: how the user enters/exits scrollback-select
  mode vs. passing input through to the app, and what's bound. Settle the
  bindings as you build; record the chosen model in the crate docs + CONTEXT.
- **Tests (headless):** feed more than one screenful, `set_scrollback`, assert
  the visible window; build a selection over known coordinates spanning
  scrollback, assert the extracted text; assert the OSC-52 payload is
  well-formed (base64, correct envelope).

## Notes

- Keep selection + extraction **in the crate and unit-tested headlessly**;
  isolate the actual clipboard write behind a trait (reuse 010's side-effect
  seam) so tests don't touch the real clipboard.
- The interaction model (modal copy-mode vs. always-on wheel scroll) is the one
  real design choice here — small, settle it by building; don't over-spec.
- Out of scope: grove dashboard wiring (→ 030).
