# 080-tui-capture-multiline-paste

**Kind:** work

## Goal

Stop the capture modal's body field from submitting on every newline, so a
multi-line observation can be typed or pasted without `Enter` accidentally
firing submit.

## Context

Deferred from an inbox observation (2026-05-29), verbatim:

> Pasting multi-line input into the capture body text area causes the
> 'enter=submit' to trigger, but it shouldn't. submit should be harder to
> trigger so it doesn't happen by accident, and also so that multiline input
> can be entered.

Today the modal is a one-line editor by design (`src/tui.rs`, `CaptureModal`
— "a multi-line editor is the wrong tool for a one-line observation, and
anything longer should drop to `$EDITOR` via Ctrl-E"). But `Enter` is bound
to submit, so a pasted newline (or a deliberate one) submits prematurely and
truncates the observation.

Pointers:
- `src/tui.rs` — `CaptureModal { open, field, target, body }`,
  `render_capture_modal` (~line 729), and the `handle_key` path that maps
  `Enter` to `PendingAction::Submit`.
- `src/tui.rs` — `PendingAction::EditBody` already routes longer input to
  `$EDITOR`; that remains the escape hatch for genuinely long bodies.

## Done when

- In the body field, `Enter` inserts a newline (or is otherwise non-
  submitting); submit requires a distinct, deliberate gesture (e.g.
  Ctrl-Enter / a confirm key) that cannot fire from a paste.
- Pasting a multi-line string into the body does not submit and does not
  truncate the observation eventually sent via `grove-llm inbox-add`.
- The target field keeps its current single-line Enter-advances behaviour
  (the accidental-submit problem is specific to the body field).
- A `handle_key`-level test covers: newline in body does not submit; the
  deliberate submit gesture does.

## Notes

- Decide whether the body becomes truly multi-line (stored with `\n`) or
  whether newlines are normalised; the observation asks for multi-line to be
  *enterable*, which argues for keeping the newlines.
- Relates to [[090-tui-inbox-triage]] — same capture/inbox surface.
