# extract-finish-baseline-k26


## Goal

Extract the existing, load-bearing finish/recovery behaviour into `grove-finish` before applying any model-proven simplification.



## Context

This is a behavioural move, not the simplification step. Preserve witness, evacuation, external ticket, quarantine, journal/store, recovery, cleanup, fault injection, and lane-specific mechanisms unless the formal synthesis inserted a later leaf that replaces one under stronger checked claims.

## Done when

- Contract/fault tests are written first for the shared protocol and all three VCS lanes, including failure and restart at every durable boundary, ownership loss, both success exits, and exact `RecoveryPending`/`OwnershipConflict` classification.
- A deep entry point—conceptually `FinishService::new(validated_workspace)` plus execute/recover operations—hides protocol, journal/store, repository adapters, cleanup, platform, and fault mechanics.
- `execute` attempts the requested finish from current state; `recover` resumes only a correlated persisted artifact set. Neither mutates unrelated or ambiguous state.
- The crate depends on `grove-workspace` and the minimum task-tree observation contract, not on CLI rendering, session driving, or application globals.
- Root callers migrate with identical observable behaviour; old finish modules and duplicated tests are removed only after real Git/native-jj/colocated-jj and model-derived tests pass.

## Notes

Keep environmental operations explicit inside the crate even if the public interface is small. The abstraction is deep because it hides a conservative protocol, not because it ignores failure states.
