# 050-filter-mode

**Kind:** work

## Goal

Wire the [[filter mode]]'s interaction: `/` enters from Nav, live re-ranking
as the needle/toggles change, Enter accepts / Esc clears, layered Esc in
normal Nav, the whichkey footer's in-mode hints, and the engaged-criteria
summary line.

## Context

- Verdicts: 010-plan Q2 (inline live-filter), Q3 (`/`; Ctrl-i inbox with Tab
  alias; Ctrl-l lifecycle; Ctrl-s sort), Q5 (criteria summary line; Esc
  layers — clear first, pane second; `/` re-enters preserving the needle).
- `src/tui/focus.rs` — the pure `arbitrate` table grows a filter-entry state
  (whether a new `Focus` variant or a Nav sub-state is the session's call;
  keep the table pure and headless-tested either way).
- `src/tui/footer.rs` — the single footer shows the in-mode key hints.
- Engine from 040; nav shapes from 030.

## Done when

- Full key flow headless-tested through `arbitrate` + nav state: enter mode,
  type/backspace needle, flip each toggle, accept, re-enter with needle
  preserved, layered Esc (engaged → cleared → pane).
- The criteria line renders whenever any dimension is engaged; never when
  idle.
- Ctrl-i and Tab both toggle inbox-pending in-mode (legacy-terminal aliasing,
  Q3).

## Notes

Selection semantics while the list re-ranks live: keep the cursor on the
top-ranked row as the needle changes (fzf behavior), not name-sticky.
