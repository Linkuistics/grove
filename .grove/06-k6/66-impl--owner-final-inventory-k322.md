# owner-final-inventory-k322

## Goal

Take the final inventory of the owner's installed skill set before finish,
and reconcile each entry with accepted Ravel delivery.

## Context

- `docs/specs/owner-skill-delivery.md`, "Classes and dispositions" and
  "Cutover contract".
- `docs/owner-skill-delivery-evidence.md`, "Re-inventory before the
  cutover", and the corrected instrument of owner-inventory-exceptions-k264.
- `docs/owner-cutover/switch-plan-draft.json` and the dry run of
  owner-switch-rehearsal-k255.

## Done when

- `task owner:inventory` runs on the real HOME, and
  `task owner:inventory:compare` compares the result with the inventory of
  2026-11-07.
- Each entry, TestAnyware's included, has a disposition: delivered by an
  accepted Ravel source, retained for a named reader, or a recorded exception.
  No entry stays unassigned. Tool, hook and plugin dependencies are named.
- A difference that changes the switch plan's inputs is carried as live work.
  That work rewrites the plan and needs another dry run before finish.
- A committed evidence section records the inventory, the comparison and the
  dispositions.

## Notes

Run this leaf after every other Homebrew change of the rollout. Finish's
own dry run against the real prefix still runs after promotion.

paused-grove-handoffs-k320 read the owner's `~/.config/ravel/config.rhai` on
2026-11-13. Its SHA-256 was `21ca2e515718ef638b6a5fecea4e4b099e93c7f4c4a50b6e4b7a0434a4336a6d`,
modified 2026-11-11 14:44. No record names that digest. The staging HOME
copied `0ccd5cdaa954…` of 2026-11-08, and the switch plan names no
`config.rhai`. Record the current digest, and decide whether the change
affects a staging verdict or the dry run.
