# system-k13


## Goal

Model Grove's end-to-end lifecycle in Quint at component boundaries.



## Context

Compose task-tree observations with finish/recovery observations in `models/system/`. Build before reading Alloy's system model and avoid duplicating component internals.

## Done when

- State/actions connect confirmation intent, session completion, selectable work, exhaustion, finish entry, interruption, restart, blocked recovery, ownership conflict, preserve/merge success, and root absence.
- Explicit refusals prevent early finish and ordinary work during correlated recovery; interruption remains possible at every transient boundary.
- Invariants and temporal/scenario checks cover safe absence, stable terminal outcomes, recovery availability under stated assumptions, and absence of unowned mutation.
- Tests, simulations, seeds, trace limits, backend limitations, and witnesses for normal and hostile paths are documented and run by the common model command.
- Cross-component findings are appended to Experiment 2.

## Notes

Do not claim liveness from a few successful simulations. State exactly which temporal claim, scheduler/fairness assumption, and finite limit were checked.
