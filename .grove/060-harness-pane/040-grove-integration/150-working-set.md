# 150-working-set

**Kind:** work

## Goal

Flesh out each grove tab's [[working set]] — harness + a plain terminal + yazi
(files) + lazygit (→ lazyjj) — embedding the aux tools via the **[[trellis
framework]]'s TUI-embedding capability**, with **individual show/hide toggles** and
a **responsive layout** that packs everything on a large display (5K2K) and
degrades gracefully to a MacBook Pro screen.

## Context

- **This is where trellis's headline capability (ADR-0020 D-fork-3) gets
  exercised:** the terminal / yazi / lazygit panes are *embedded TUI apps*, run via
  the framework's seamless embedding (zellij's emulation under the hood) — the same
  mechanism the harness pane uses. 110 proved grove's *own* native surfaces; this
  proves **embedding other TUI apps**.
- ADR-0018/0019 (UX) stand: harness + native detail (130) + terminal + yazi +
  lazygit, per-pane toggles driven from the nav, responsive defaults per screen
  size. Each aux pane runs in the grove's worktree cwd.
- A *specific*-pane toggle is open/close (or float/embed) of that one pane, driven
  in-process via the host API (110) — no native zellij float-toggle limitation to
  work around now that grove owns the layout code.

## Done when

- A grove tab shows harness + terminal + yazi + lazygit laid out sensibly for the
  current screen size; the default-visible set adapts to terminal size.
- Each pane toggles individually from the nav; aux panes run in the grove's
  worktree; the embedded tools behave exactly as they do bare (focus, input, copy,
  resize, cursor — the trellis embedding promise).
- lazygit works; the vcs pane is not hard-wired to git (lazyjj later).
- `cargo build`/`cargo test` green.

## Notes

- Keep it the smallest legible thing (constraint 4) — not a general tiling-config
  system. Responsive defaults/breakpoints are this leaf's design call.
- Embedded-tool **observability** (ADR-0020 D-fork-5) beyond plain rendering is a
  *later/lazy* framework concern — this leaf only needs the tools embedded and
  usable, not introspected.
- Depends on **110/120/130** (host API; nav-opened tab; native detail in it).
