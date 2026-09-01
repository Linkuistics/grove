# pipeline-kinds-k73

**Integrates:** pipeline-kinds-k72

## Goal

Triage the design review of `pipeline-kinds-k27` and apply every finding that
survives scrutiny before `pipeline-skills-k28` authors the four editorial kinds.

## Context

- Read `pipeline-kinds-k72` from its committed review task; its findings and
  citations are the handoff and are not restated here.
- The reviewed artifacts are the two pipeline ADRs, the four-kind work order in
  `pipeline-skills-k28`, and the pipeline shape in `crate-books-k14`'s brief.
- The preregistration and pilot report remain frozen evidence: integration may
  correct what the design says or hands forward, never rewrite the measurement
  to make a finding disappear.
- `pipeline-skills-k28` is the direct consumer and follows this leaf in the walk.

## Done when

- Every review finding is classified against the current artifacts as accepted,
  rejected, or already resolved, with the reasoning recorded in this task's
  running decision log.
- Every accepted finding is integrated into the minimum coherent ADR set, the
  implementation work order, and any directly affected brief or durable handoff.
- The correction-chain procedure specifies one ordered, terminating sequence;
  the review allowance has one owner; and the structure-brief precondition is an
  interface an installed skill can test.
- `bash scripts/check.sh` passes.

## Notes

This leaf was inserted at the first live sibling after the review so no consumer
can silently move its path-and-line citations before triage.
