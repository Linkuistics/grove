# 020-input-focus

**Kind:** work

## Goal

Make the harness pane *interactive*: route crossterm input through grove's leader-gated
focus model into the pane. After this leaf you can type into the harness as if it were a
normal terminal, with `Alt-g` reaching grove.

Build:
- `tui::input`: the crossterm→tmux-token key-map. Take the spike's `forward_key` as the
  baseline and extend to the **modifier matrix on special keys** (`C-Left`, `S-Up`,
  `C-Enter`, …) + correct Shift handling (E5). Plain printables → `send_text`; special/
  modified → `send_key` tokens.
- The **focus state machine** (E4): `Focus = Harness(PaneId) | Nav | Modal(kind)`.
  - **Harness:** forward all keys to the pane *except* the leader; leader → `Nav`.
  - **Nav / Modal** surfaces may be **stubs** this leaf (Nav = a placeholder "grove
    focus" indicator; the real nav is 030, the real modal is 040) — what matters is the
    arbitration + transitions (Esc/return → Harness; Modal captures + restores prior
    focus).
- **Leader = `Alt-g`**, read from config with `Alt-g` as the default (E4); make the
  binding configurable from day one.
- **Bracketed paste** (E5): enable crossterm bracketed paste; on `Event::Paste` with
  Harness focus, forward via `send_text` wrapped in `\e[200~…\e[201~`. In Modal focus,
  insert into the buffer literally.
- **Mouse** (E5): left-click forwards to the harness as a focus/click via
  `pane.mouse().click`. Rich passthrough (drag/wheel/motion) is **explicitly deferred**
  to 050 (likely a fork capability) — leave a clear comment, don't build a lossy
  translator.

## Context

Depends on 010's loop (the `select!` input arm already exists; this leaf fills in what
the arm *does*). The arbitration is grove-owned — there is no zellij locked-mode (E4);
grove sees every key first and decides UI-vs-forward purely by `Focus`. Keep it
headless-testable: the key→token mapping and the focus transition table are pure
functions over `(Focus, KeyEvent) → (Focus', Action)` — unit-test them with no terminal.

## Done when

- Typing into a focused harness works (vim/a shell/claude usable inside the pane);
  multi-line paste arrives as one bracketed paste, not line-by-line.
- `Alt-g` flips focus to a grove surface (even a stub) and Esc returns to the harness;
  the leader is configurable.
- Left-click on the pane focuses/clicks it.
- The focus-transition table + key-map are covered by headless unit tests;
  `cargo build`/`cargo test` green.

## Notes

The key-map is where fidelity is won or lost — bias toward forwarding faithfully. If a
key has no clean tmux token, prefer `send_text` of its bytes over dropping it. Record any
keys that can't round-trip (a known-gaps list) for 050.
</content>
