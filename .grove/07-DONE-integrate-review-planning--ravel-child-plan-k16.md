# ravel-child-plan-k16

**Integrates:** ravel-child-plan-k15

## Goal

Triage the findings of `ravel-child-plan-k15` against the committed plan and
apply the findings that hold before implementation begins.

## Context

- The review's commit, found by its handle, and the producer
  `ravel-child-plan-k4`.
- The root brief's Requirements and Design, and the planned implementation
  leaves `ravel-floor-k6` through `book-specs-k14`.

## Done when

- Each review finding has a recorded disposition supported by the source and
  the plan. Apply the valid findings to the planned subtree and reconcile its
  brief and producer record as needed.
- Each planned boundary has a completion condition that it can meet before
  later siblings run.

## Notes

- Read the findings from the review rather than treating them as an accepted
  work list. This leaf changes the plan; implementation remains with its
  implementation leaves.

## Decisions (running log)

Read from the review's commit `bb117d36` (`06-DONE-review-planning--ravel-child-plan-k15.md`).
The session checked each finding against the source tree. All six are real, and none
needs a redesign, so no new review chain is cut. Each real issue is a fix to a
leaf's completion condition.

- **F1, P1, real issue: the floor leaf omits the overview fragment
  (integration, 2026-10-22).** `docs/walkthroughs/overview/06-standalone-invocations.md:517`
  quotes `dispatch/ravel.rs` whole (lines 1-313). `ravel-floor-k6` now owns the
  chapter, `source-index.md` and `concept-index.md` edit, with the conversion
  first.
- **F2, P1, real issue: the `Argv` bridge breaks the overview book
  (integration, 2026-10-22).** `standalone.rs:141` and `:182` sit inside
  fragments `standalone-launch-context` and `standalone-supervision`.
  `ravel-job-k7` now carries the chapter 06 edit. `grove-run-job-k8` is told
  that the fragments move twice and that a chapter converts once.
- **F3, P1, real issue: two archive consumers pin the old layout
  (integration, 2026-10-22).** The source holds `test_ravel_target_inputs.py:18` and
  `ravel-target-inputs.test.sh:22,28-29,68-71`, and `Taskfile.yml:119` requires
  `ARCHIVES`. The check also found three more 1.0.0 consumers.
  The review did not list them: `scripts/release-smoke.sh` (lines 211, 344, 394), the
  `ravel 1.0.0` assertion at `session_result_installed.rs:157`, and the
  `ravel:result` task that runs it. All moved to `ravel-floor-k6`, with a seeded
  search as the instrument and `ARCHIVES` named for the controls task.
- **F4, P1, real issue: deleting the book leaves dangling links
  (integration, 2026-10-22).** `every_repository_markdown_reference_resolves`
  scans all Markdown. The only inbound links are `docs/ARCHITECTURE.md:45` and
  `docs/specs/grove-loop-book-structure.md:19`. `keyed-launch-removal-k11` now
  fixes both, plus the prose citation at `walkthrough-books.md:978`. The
  producer record no longer says that structure specs have no gate.
- **F5, P2, real issue: current explanations have no owner
  (integration, 2026-10-22).** `lifecycle-job-k9` now owns overview chapters
  03 (line 814) and 04 (line 263) and the `session_result.rs:26-30` comment
  with its book fragment. `book-specs-k14` owns row L12 of
  `user-guide-coverage.md`. A fresh sweep of current records found no further
  unowned file. `prompt.rs:151` ("after the group is gone") and the passages at
  `harness-selection-and-execution.md:294` and `standalone-invocations.md:81`
  describe Ravel's reap, so they stay: noise raised for want of context.
- **F6, P2, real issue: the sweep conditions cannot hold at their
  boundaries (integration, 2026-10-22).** `keyed-launch-removal-k11` and
  `design-records-k12` now sweep only the records they own, and list the
  remainder by file with the owning leaf. `book-specs-k14` holds the one final
  sweep, excludes `.grove/` for a stated reason, covers `survivor`,
  `surviving` and `group`, and carries a seeded control and a broken-pattern
  control.
- **Lifecycle split seam, accepted as a note (integration, 2026-10-22).** The
  review's evidence section found the lease-first split needs an observer
  bridge. `lifecycle-job-k9` Notes now say so, and name which child keeps the
  dependency line.
- **Brief and producer record reconciled (integration, 2026-10-22).**
  `_BRIEF.md` Decomposition now lists this leaf and the shifted numbers, and
  the floor section lists the added consumers. The plan-k4 log carries two
  corrections that point here.
