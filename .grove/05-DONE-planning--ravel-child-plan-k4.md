# ravel-child-plan-k4

## Goal

Cut the implementation leaves that carry out the design of `ravel-child-k2`.
Find the smallest increments that are useful on their own and keep
`bash scripts/check.sh` green.

## Context

- The root brief: *Requirements*, *Design* and *Notes*. *Design* holds the
  shared function, the new homes, the removal list, the records to rework, the
  tests and the residues.
- The caller contract `docs/specs/harness-selection-and-execution.md`, and the
  ADRs `docs/adr/the-launched-child-is-a-job.md` and
  `docs/adr/dispatch-supervises-the-harness.md`.
- The review `ravel-child-k3`, and any integration that it cut.

## Done when

- Each item of *Design* in the root brief belongs to exactly one leaf.
- Each leaf keeps the repository gate green on its own. A book chapter lands
  with the code that it quotes, and the crate goes only after no caller uses it.
- Each leaf that changes the job, the resolver or the interpretation names the
  Ravel tasks that it must pass with `RAVEL` set.

## Notes

- The change fits an expand, migrate and contract sequence. Add the shared
  function beside the crate. Then move the two callers and the tests of the
  driver lease. Then remove the crate, its tasks and its book.
- The 1.1.0 floor of the resolver does not depend on the rest, so it can land
  first. The same leaf moves the release inputs to Ravel 1.1.0, as the root
  brief lists in *The 1.1.0 floor in the release inputs*. A release on the old
  pins fails, because Grove refuses their Ravel.
- Each edited record needs a conversion first, as its own change. The root
  brief, in *Notes*, says which records already pass the checker.

## Decisions (running log)

- **Nine leaves in expand, migrate and contract order (planning, 2026-10-22).**
  The floor comes first (`ravel-floor-k6`). It touches none of the code that
  later leaves change, and the shared function relies on the corrected failure
  paths of Ravel 1.1.0. The shared function follows beside the crate
  (`ravel-job-k7`). `grove run` moves next (`grove-run-job-k8`), then the loop
  driver with the driver lease (`lifecycle-job-k9`), then the rule that an error
  keeps a caught signal (`interrupted-errors-k10`). The crate goes only after
  that (`keyed-launch-removal-k11`). Three leaves of records close the grove.
- **The book gate sets the leaf boundaries (planning, 2026-10-22).** The books
  of `grove-loop` and `overview` take every `src` file as corpus and
  reproduce it exactly. A new module and each changed line need their chapter
  edit in the same leaf. So a new module cannot be a leaf without its chapter.
  Chapters land with the code that they quote, and records that no test
  gates land after the code.
- **`grove run` moves before the lifecycle (planning, 2026-10-22).** It has no
  lease and no interpretation, so it proves the detached shape cheaply. The two
  latches never meet in one process, because `grove run` and `grove do` are
  separate invocations.
- **`Argv` moves in the expand leaf (planning, 2026-10-22).** The callers
  convert the new value at their one call site until they move. This bridge is
  small and each migration leaf removes one conversion.
- **The scrub list merges across two leaves (planning, 2026-10-22).** The two
  copies sit in `standalone.rs` and `loop_driver.rs`. The new module owns the
  one list. `grove-run-job-k8` drops the first copy and `lifecycle-job-k9`
  drops the second.
- **The rule on errors is its own leaf (planning, 2026-10-22).** The migration
  keeps the behaviour of the errors. The addition has its own unit seam and
  stays easy to review. The shared function already returns an error that
  carries the interruption.
- **The structure specs wait for the last leaf (planning, 2026-10-22).** The
  specs of the books are not gated by a content test, except the row in
  `walkthrough-books.md`, which goes with the crate. Each spec converts once,
  in `book-specs-k14`. *Corrected by `ravel-child-plan-k16`:* a Markdown link
  to a removed file is gated by `every_repository_markdown_reference_resolves`,
  so `keyed-launch-removal-k11` also fixes the two inbound links.
- **Mapping of the work to leaves (planning, 2026-10-22).**
  - Shared function, `Argv`, `LaunchEvent`, `take_interrupt`, `reraise` and the
    scrub list: `ravel-job-k7`.
  - `regular_file_at`, the standalone scrub list and the standalone dependency
    line: `grove-run-job-k8`.
  - `Channel::discard_abandoned`, the loop scrub list, the lease tests, the
    interpretation tests and the dependency line of `grove-loop`:
    `lifecycle-job-k9`.
  - The signal rule for errors: `interrupted-errors-k10`.
  - The removal list, the row of the book inventory and the four test comments:
    `keyed-launch-removal-k11`.
  - The design records, the diagram panel: `design-records-k12`.
  - `RELEASING.md`, `USAGE.md` and `driver.md`: `user-records-k13`.
  - The structure specs and `user-guide-coverage.md`: `book-specs-k14`.
  - The 1.1.0 floor with the release inputs and the resolver cases:
    `ravel-floor-k6`. *Added by `ravel-child-plan-k16`:* the two target-input
    controls, `release-smoke.sh`, the installed-Ravel version assertion and
    the overview fragment of the resolver file.
- **A review of the plan runs first (planning, 2026-10-22).** Nine sessions
  build on this decomposition, and the gate order is new content. The
  `review-planning` leaf sits ahead of the first leaf of code. The producers
  `ravel-job-k7` and `lifecycle-job-k9` cut a `review-impl` leaf at their end.
- **No provider receipt (planning, 2026-10-22).** The environment of this
  session has no `GROVE_PROVIDER_EVIDENCE`, so the review carries no
  `**Creator:**` line.
- **No in-session reviewer (planning, 2026-10-22).** The scheduled
  `review-planning` leaf is the adversarial read.
