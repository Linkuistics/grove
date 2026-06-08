# 040-detail-triage

**Kind:** work

## Goal

Make the detail inbox view interactive — **grooming** the focused grove's pending
observations from the TUI: **reject** (delete an out-of-scope observation) and
**move/re-route** (send a misfiled note to another grove, then drop it here). Both
are shell-outs below the presentation seam (the ADR-0028 E1 idiom), run under
`spawn_blocking`.

## Context

010 settled detail's interactivity as inbox **grooming**, not the full
incorporate/defer/reject vocabulary: from the TUI you are *watching* a harness,
not executing a task, so "incorporate into a task" and "defer to a new leaf" stay
at the session-bootstrap [[Drain]] (both presuppose an active task). What maps
cleanly to a dashboard is **reject + re-route**.

030 built the read-only detail widget incl. the inbox view. This leaf adds
selection + the two grooming actions:

- **reject** → `grove-llm inbox-drain --for=<grove> --rejected=<path>` (deletes the
  one observation in a commit; pushes when configured).
- **move** → `grove-llm inbox-add --to=<other-grove> --body-file=<path>` (or
  `--body-stdin`), then drop the original (reject it from here). The "pick a target
  grove" step reuses the nav's grove list (a small picker, or the modal-overlay
  pattern).
- Mirror `submit_capture` / `open_in_editor` in `app.rs`: shell out under
  `spawn_blocking` (commit+push must not stall the reactor — E1's firewall),
  surface success/failure as a **toast**, never crash the loop. Refresh the inbox
  view after the action (it shrank).

## Done when

- In `Focus::Detail` with the inbox view, the user can select a pending
  observation and **reject** it (gone after refresh) or **move** it to another
  grove (re-captured there, gone here); both via `grove-llm` below the seam under
  `spawn_blocking`, with a toast on completion.
- Failures surface as toasts and never crash the loop; the inbox view refreshes
  after each action.
- The grooming-key arbitration is pure/unit-tested where it lives in the focus
  table; the shell-out is the impure app layer.

## Notes

- "move" needs a target-grove picker — reuse the nav list or a lightweight modal;
  keep it minimal.
- Reject/move operate on the **focused grove's** inbox (the detail panel's grove).
