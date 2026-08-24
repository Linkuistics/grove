# system-k9


## Goal

Model the temporal lifecycle from ordinary Grove work through completion, finish, interruption, recovery, and root absence.



## Context

This is a cross-component model in `models/system/`. It composes the task-tree and finish contracts at their observations rather than copying their internal state.

## Done when

- The model represents user confirmation/intent, session completion, remaining selectable work, tree exhaustion, finish eligibility, an in-flight attempt, blocked recovery, terminal preservation/merge outcomes, and absent root.
- Assertions prevent finish before explicit eligibility/intent, prevent ordinary work while correlated recovery is pending, prevent successful absence without the finish obligations, and preserve a safe recovery path after every modelled interruption.
- Temporal properties and fairness assumptions are stated separately; non-actions/refusals are modelled so liveness is not manufactured by omitting hostile choices.
- Normal, preserve, merge, interrupted, recovery, and ownership-conflict traces have witnesses within documented bounds.
- Cross-component gaps or new tests are logged in Experiment 2.

## Notes

Keep this model small enough to explain. Detailed filesystem and VCS operations remain in the finish model; detailed entry mutation remains in the task-tree model.
