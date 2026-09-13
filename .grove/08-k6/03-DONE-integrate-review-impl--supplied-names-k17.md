# supplied-names-k17

**Integrates:** supplied-names-k15

## Goal

Triage the findings of the `review-impl` leaf `supplied-names-k15` against the
supplied-name API boundary delivered at `supplied-names-k14`, apply the ones
that are real, and leave the boundary and its books current for
`valid-levels-k16` to consume.

## Context

- The findings are in the review's own file, found by its handle; read them
  from there rather than from this body. They are anchored to commit
  `kyymvozo` (`9d09fb67`) and to `path:line` coordinates in that tree. The
  review's own insert of this leaf shifted `valid-levels-k16` from position 03
  to 04; handles are unchanged.
- The producer's contract is `01-DONE-impl--supplied-names-k14.md` and the
  parent brief in this directory. The reader and projected-level enforcement
  and the expected-level conformance samples belong to `valid-levels-k16`, and
  no finding asks for them here.
- The artifact is code, tests and the two books whose roots this boundary
  changed: `ordinal-fs-tree` and `grove-loop` under `docs/walkthroughs/`, plus
  `CHANGELOG.md`. Each touched source root lands with its fragments, ledgers,
  indexes and prose in this commit; `bash scripts/check.sh` passes on
  unchanged tracked inputs.

## Done when

- Every finding is triaged as applied, rejected with the reason, or a visible
  accepted trade-off, in this file's running log.
- Where a finding names an API decision the enforcement child inherits, the
  decision is settled and recorded here so `valid-levels-k16` builds on it
  rather than reopening it.
- `valid-levels-k16` still follows this leaf; positions and keys are otherwise
  untouched. The installed driver, plugin and live-tree grammar remain
  untouched.

## Notes

- Substantial redesign is not this leaf's: externalise it as a new producer
  review chain beside the leaf being integrated.
- One narrow in-session reviewer is available for a non-mechanical fix; a
  second need is the signal to cut a `review-impl` leaf instead.

## Decisions (running log)

1. F1, F2 and F6 are real documentation issues: the cited book text and ledger
   still name a removed factory, Unreleased is empty, and chapter 4's ownership
   total disagrees with its ledger. Correct these against the supplied values
   at the lifecycle call sites and the current ledger.
2. F3 is a real regression gap, not a demonstrated escape: `names_are_one_component`
   checks effect names before effects, including before root creation. Add
   supplied-name adversaries through both public operations, checking the error,
   unchanged files and bytes, and absent destinations. Bypassing that validation
   for distinguished effect names must fail both tests.
3. F4 is an accepted diagnostic refinement of the model's coarse outcome.
   Keep `SuppliedNameNotDistinguished` for wrong species. The enforcement child
   must report rejected level policy separately, retaining the domain error and
   containing-level path in `Error<N>`; it must not flatten that error into this
   non-generic `Refusal`. Both cases correspond to `RefusedInvalidDistinguished`
   where the model represents them. Restore the refusal witness and supplied-name
   invariant citations; byte preservation remains outside the model.
4. F5 is an accepted sampling limit. An empty name slice remains untested, even
   for a domain that admits no distinguished names: the API carries no evidence
   of that universal claim. Expected-level samples in k16 test those levels;
   they do not establish absence of every possible distinguished value. State
   this in the kit diagnostic and book; do not add an absence declaration or
   silently count empty samples as passing.
5. F7 is minor but real: the lifecycle caller supplies `TaskName::Brief`, and
   the kit's combined diagnostic must include wrong species as well as a failed
   canonical round trip. Correct the wording and synchronized fragments.
6. Graph discovery fell back to targeted source reads after `list_projects`
   refused startup due to an incompatible active generation. No graph coverage
   claim is made. The installed driver and graph sessions are untouched.
7. F1, F2, F3, F6 and F7 are applied. F4 keeps the diagnostic refinement with
   model citations restored; F5 keeps the sampling limit with explicit advice.
   Neither requires a redesign. No in-session reviewer was used: the changes
   are documentation, visible trade-offs and tests checked by a concrete mutant.

## Verification

- The four confinement tests pass. Temporarily bypassing
  `names_are_one_component` for distinguished effect names makes both new
  supplied-name tests fail: initialization reaches filesystem creation and
  promotion incorrectly succeeds. The bypass was restored before final checks.
- `bash scripts/check.sh` exits 0: all eight principal checks pass, including
  the locked workspace tests and final reconstruction of all six books.
  SHA-256 inventories of the repository's non-ignored files, including hidden
  files and sources, fixtures, scripts, manifests and books, compare identical
  before and after the run. This evidence and tree bookkeeping follow that run.
- The removed factory reference is absent from the grove-loop book; the same
  literal search finds the pre-change chapter and the historical review. The
  corrected ownership count agrees with the ledger and closing arithmetic.
- `valid-levels-k16` remains the next live sibling. The parent has outstanding
  enforcement work, so no node closes in this retirement.
