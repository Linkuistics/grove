# harness-selection-and-execution-k42

**Reviews:** harness-selection-and-execution-k6

## Goal

Review the implementation tree that `harness-selection-and-execution-k6` cut
for `harness-dispatch`'s first release, before any increment runs. Produce
findings, not fixes.

## Context

Read the planning leaf's running log and its commit, which grew the tree. Read
the root brief's `Implementation plan`, every node brief, and every leaf body
under the ten increments. Judge them against
`docs/specs/harness-selection-and-execution.md`, its ADRs and the root brief's
accepted requirements. The design is reviewed and settled. Challenge the
decomposition, not the design.

## Done when

The findings are committed, and each is classed as blocking or advisory. The
doubts the producer could not settle alone are these:

- **Independence.** Can each increment be demonstrated at its boundary without a
  later sibling? Is any leaf a horizontal layer that sits dead until its
  successor lands? Suspects are `choice-and-refusals-k15`, which bundles
  diagnostics, and `dispatch-documentation-k41`.
- **Order.** Delivery runs third to surface the supported-target floor early.
  Is the cost worth it? The archive assertions and smoke churn with each later
  shipped file. Does computed-before-records, or records-before-boundary,
  hide a dependency the log does not state?
- **Coverage.** Repeat the coverage walk. Every cell of the spec's `#test-seams`
  table, every root acceptance case, and the planning leaf's `Done when` should
  map to a leaf that owns it. Look for obligations no leaf names: a
  documentation claim, a Taskfile task, an archive assertion, a book update.
- **Size.** Which leaves will not fit one session? `routed-inspection-k13`
  carries the crate, the worker, the protocol, authority and inspection.
  `ambient-authority-k30` carries five hostile classes with probe builds.
- **Review placement.** Five node reviews are expected. Is that too many or too
  few, and is each one's stated doubt the right one?
- **Methodology fit.** Each increment is a node of this grove, not a separately
  created grove. Does that reading of the planning rule hold, given the
  repository's finish-and-release sequence?

## Notes

A `review-planning` session spends no in-session reviewer. If the findings
warrant action, cut `integrate-review-planning` with the bare stem
`harness-selection-and-execution`. Place it where `pick` reaches it next,
which is before `grove-task-slots-k11`.
