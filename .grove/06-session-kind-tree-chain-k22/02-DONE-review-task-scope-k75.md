# review-task-scope-k75

**Kind:** impl

## Goal

Make Grove review sessions inspection-only so the standard
implementation/review/integrate chain does not pay for the implementation's
test cycle twice.

## Context

- The reviewer reviews only the implementation task's changes against its
  requirements and verification evidence. It does not re-run project tests,
  modify production code, or redo the implementation.
- Concrete findings remain review output for the existing integrate task;
  fixes and post-fix verification belong there.
- Apply the rule both to the Grove skill content and to every still-live review
  task in this grove, including `session-kind-tree-review-k24`.
- Preserve the ability to inspect diffs, source, specifications, and already
  captured command output. "No re-test" means no test/build/lint/format runs in
  the review session.

## Done when

- The Grove skill's review-stage instructions explicitly prohibit re-testing
  and implementation edits, and define review output as findings-only.
- Each live current-grove `review-*` task carries the same inspection-only
  constraint before it can be picked.
- Review instructions direct any required fixes to the paired integrate task
  and rely on the implementation task's recorded verification evidence.
- Relevant skill-content checks pass without running this project's test suite.

## Notes

This is a throughput correction: the consistent impl/review/integrate
breakdown remains, but review no longer duplicates the expensive producer test
pass.
