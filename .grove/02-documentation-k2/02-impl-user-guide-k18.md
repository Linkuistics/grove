# user-guide-k18


## Goal

Make `docs/USAGE.md` a complete, task-oriented guide for humans and agents using the redesigned Grove.



## Context

The guide must explain actual observable behaviour fixed by the models and preservation ledger, including the approved absence of migration. It should let a user recover safely without reading implementation or formal notation.

## Done when

- A quickstart covers installation/checks, current-format root creation, picking/briefing/kind, driving work, and ordinary retirement.
- Separate but connected examples cover human and agent entry points without duplicating the whole guide.
- Git, native jj, and colocated jj workflows state supported layout, branch/bookmark/workspace expectations, and equivalent abstract outcomes.
- Finish documents explicit confirmation, when it is eligible, progress/diagnostics, the preserve exit, the merge-and-remove-owned exit, interruption/restart, and exact recovery commands.
- `RecoveryPending` and `OwnershipConflict` have symptoms, meaning, safe next steps, and escalation guidance. The guide states that Grove never resets, merges, deletes, or rewrites unproved work.
- Current `session-kinds-v1` roots and fresh initialization are documented; legacy migration is absent and legacy/foreign roots receive fail-closed remediation guidance.
- Every command/output example is checked against the baseline or executable CLI, and configuration detail links to `docs/CONFIGURATION.md` rather than being copied.

## Notes

Write for a user who understands version control but not Grove internals. Use formal terms only where they appear in stable diagnostics, and explain them once in plain language.
