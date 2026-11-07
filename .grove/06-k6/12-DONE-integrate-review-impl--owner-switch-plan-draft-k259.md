# owner-switch-plan-draft-k259

**Integrates:** owner-switch-plan-draft-k258

## Goal

Triage the review of the owner's switch plan draft and integrate the
actionable findings before its staging consumers run.

## Context

Read the findings from the committed review `owner-switch-plan-draft-k258`.
Its durable report is the section "Owner switch plan draft review" of
`docs/owner-skill-delivery-evidence.md`. The producer is
`owner-switch-plan-draft-k247`; `docs/specs/owner-skill-delivery.md` owns
the contract.

## Done when

- Each finding has a recorded disposition supported by source and the
  owning contract.
- Accepted findings are integrated, and the resulting artifact has the
  verification required by the integration procedure.
- Any resulting handoffs reach the leaves that consume the draft.

## Notes

The review ran no executable checks. The real HOME remains read only in
this integration; real activation belongs to finish. Use disposable
layouts for verification.

## Decisions (running log)

- Each of D1 to D5 was real against the current design. Repairs for all
  five were prototyped and passed their checks in disposable layouts.
- The human stated that this grove is the only grove that runs, and that
  the human is the only user of Grove and Ravel. The human chose to drop
  the repairs and redesign the switch. The human chose four
  simplifications: the process and job exclusion, the concurrent-writer
  detection, the recovery state machine and the rehearsal admission.
- This leaf discarded every repair. It cut
  owner-switch-simplification-k260, k261 and k262 before the staging
  leaves. k260 carries D1, D3 and D5, which stay relevant to any design.
- The section "Owner switch plan draft integration" of
  `docs/owner-skill-delivery-evidence.md` records each disposition.
