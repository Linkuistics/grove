# implementation-plan-k21


## Goal

Turn the approved architecture into an exact, test-first implementation sequence and refine the implementation subtree before any Rust is changed.



## Context

The existing implementation leaves describe observable slices. Inspect current files and dependencies, map each move to formal claims and documentation, and decompose any leaf too broad for one safe session using canonical `grove-llm` operations.

## Done when

- Every implementation leaf names exact source/manifest/test/doc paths, intended public interface, dependency changes, preservation claims, first failing tests, focused verification, and deletion/contraction steps.
- Wide extractions use expand → migrate consumers → contract old owner; each intermediate state compiles and preserves public behaviour.
- Model-earned ordinal and finish leaves inserted by `formal-synthesis-k16` are ordered at the required seams and have the same implementation detail.
- Task dependencies are reflected by ordinal order or decomposition; no leaf relies on an unstated semantic decision or an artifact produced later.
- Fault-injection and real Git/native-jj/colocated-jj fixtures are assigned to the crate that owns the behaviour, with cross-crate tests retained only at application boundaries.
- Final verification covers model runners, docs/examples, workspace tests/lints, MSRV 1.85, glibc 2.17/release checks, installer/embed paths, and baseline contract comparison.
- The refined `.grove/03-implementation-k3/` tree is internally complete and `grove-llm pick` selects the correct next implementation slice after this phase retires.

## Notes

This leaf may edit the implementation phase briefs/tasks and create decomposition/review nodes; it must not edit Rust product code. Prefer smaller leaves with demonstrable vertical outcomes over file-move batches.
