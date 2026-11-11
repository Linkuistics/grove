# owner-freeze-day-check-k303

## Goal

Check the owner's update freeze at least 24 hours after its baseline. Record
the result for the dry run and the finish handoff.

## Context

- owner-marketplace-freeze-k253 froze automatic plugin updates and held
  Claude Code at 2.1.322 on 2026-11-11. "The update freeze" of
  `docs/owner-cutover/owner-checklist.md` records the setting.
- The baseline is `docs/owner-cutover/update-freeze-baseline.json`, recorded
  at 2026-11-11T03:33:44Z. The check is due at 2026-11-12T03:33:44Z or later.
- `task owner:freeze:check BASELINE=docs/owner-cutover/update-freeze-baseline.json`
  reads the real HOME only. It fails on a changed freeze setting or Claude
  Code version. It also fails on a new plugin version directory, a new orphan
  marker, a changed installation or a marketplace refresh.
- Claude Code removes an orphaned directory 14 days after its marker. The
  check reports that removal as a note and passes.
- The section "Update freeze" of `docs/owner-skill-delivery-evidence.md`
  holds the earlier measurements.

## Done when

- The check ran at least 24 hours after the baseline and reported 0 breaks.
  `claude --version` reported `2.1.322 (Claude Code)`. `claude doctor`
  reported `Auto-updates: disabled (set by env: DISABLE_AUTOUPDATER)`.
- The section "Update freeze" of `docs/owner-skill-delivery-evidence.md`
  holds the time and the result of the check.
- If the check reports a break, the evidence records it, and the owner
  decides the remedy before the dry run. A changed Claude Code version needs
  another dry run, and `task owner:quiet-interval` with that version.

## Notes

If fewer than 24 hours have passed since the baseline, hand the leaf back to
the owner with the time remaining. Keep the leaf live.

Change no setting and pause no tool in this leaf. Inspect read only.
