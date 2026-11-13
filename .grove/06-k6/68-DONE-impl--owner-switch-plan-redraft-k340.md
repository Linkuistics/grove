# owner-switch-plan-redraft-k340

## Goal

Make the prepared switch plan agree with the real HOME again, and repeat the
dry run before finish.

## Context

- owner-final-inventory-k322 found the cause. The section "Final inventory
  before finish" of `docs/owner-skill-delivery-evidence.md` holds the
  evidence.
- The plan `60c34eb7…` names the original identity `46e949c9…` of
  `~/.claude/settings.json`. Mise checkpoint 130 at 2026-11-11T19:54Z then
  changed that file's `"model"` from `"opus[1m]"` to `"fable"`. The current
  identity is `61201db3…`. The switch's preflight stops with "planned
  originals differ from the plan".
- After the dry run, mise moved from 2026.11.4 to 2026.11.7 and Codex from
  0.167.0 to 0.167.2. "When another dry run is necessary" in the evidence
  requires another dry run for each move.
- The sections "Owner switch plan" and "Owner switch rehearsal" of the
  evidence give the draft, prepare, check and dry-run commands.

## Done when

- `task owner:switch-plan:draft` writes the plan from the real HOME.
  `task owner:switch-plan:prepare` and `task owner:switch-plan:check` pass.
- The read-only plan report against the real HOME shows each planned
  original in place, except the documented absent Ravel settings file.
- The dry run passes again with the stand-in prefix. Its result replaces the
  committed and prepared results, and the comparison through the stand-in
  exits 0.
- The evidence records the new plan, program and result digests. The finish
  handoff of finish-handoff-k324 can name them.
- The owner's checklist states this rule: an edit of a write-set file after
  the plan draft needs another draft and dry run. The `"model"` edit of
  2026-11-12 is the example.

## Notes

The comparison of the dry run has no Codex tool row. It also does not
compare whole-file planned originals. The switch's preflight catches the
original, but no earlier command reported it. Decide whether the comparison
or `status` must report each one before finish, and record the decision.

Finish still runs its own dry run after the Grove release and the Ravel
promotion. This leaf does not replace that run.
