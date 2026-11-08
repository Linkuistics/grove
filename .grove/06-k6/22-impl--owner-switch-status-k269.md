# owner-switch-status-k269

## Goal

Move the owner's `mac-check` switch area and the purge report to the
simplified switch. Report delivery health and restore eligibility
separately.

## Context

- The section "Current health and restore eligibility" of
  `docs/specs/owner-skill-delivery.md` owns the status rules. "Purge" owns
  the report. "Test seams" lists the `mac-check` and purge controls.
- owner-switch-restore-k268 supplies the receipt, its delivery assertions
  and its whole-file restore identities.
- `mac-check` runs `~/.local/lib/machine-config/check.py`. Its switch area
  now runs `<state>/prepared/owner_switch.py state --json` and reports six
  states and flagged copies. owner-switch-entries-k92 records that change.
  The machine configuration contract owns the file.
- The fixture suite runs the real `mac-check` with HOME set to each
  disposable HOME.

## Done when

- The program has a read-only status entry for `mac-check`. It evaluates the
  delivery assertions, the current inventory and the sources for health. It
  compares whole files for restore eligibility.
- `mac-check` reports native delivery, a verified switched layout, or a
  layout needing restore or review. Without prepared evidence it reports
  that the switch is not prepared. It names differing files when restore is
  ineligible.
- A receipt alone never shows current success. A partial layout or stale
  evidence requires review. Formula drift keeps its separate area.
- The read-only purge report compares retained identities. It lists the
  conditions that only the owner can establish. A changed retained resource
  or an unverified switch blocks purge.
- Fixture controls cover each case of "Test seams". An unrelated model key
  or hook edit after success keeps health accurate and makes restore
  ineligible.
- The changed `mac-check` follows the machine configuration contract. The
  evidence document records its controls and one mutant that each control
  detects.

## Notes

The old `state` entry stays until owner-switch-contraction-k270 removes it.
The real HOME has no prepared plan yet, so the real `mac-check` reports that
the switch is not prepared.
