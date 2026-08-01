# retire-flowchart-confirmation-k18

**Kind:** impl

## Goal

Reconcile the Grove loop flowchart's Retire branch with the current
verify-and-report node-close contract.

## Context

`content/SKILL.md` still labels the parent-chain branch `Ask user`, while the
Retire prose and ADR `confirmation-boundary` say no node species has a routine
confirmation gate. This surfaced during `composition-guidance-k17` and is kept
separate because it is not part of doubt/Grove review composition.

## Done when

- The flowchart says the session verifies/promotes/reports a closing
  brief-carrying node without implying a human gate.
- A focused contradiction assertion prevents the stale `Ask user` label from
  returning while leaving pruning and the finish-cycle confirmations intact.

## Notes

Rework the existing diagram in place; do not broaden this leaf into a general
retirement rewrite.
