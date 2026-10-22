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
