# finish-transaction-contract-k105

**Kind:** design

## Goal

Settle the recoverable finish-transaction contract and reconcile the durable
specification, ADR, and glossary around its exact commit boundary.

## Context

- Binding constraints: explicit finish confirmation; `.grove/` as the only
  durable workflow state; task-root absence as fresh start; the universal Tree
  access lock; exact handle-named scoped commit proof; preservation of unrelated
  Git/jj work; and no ordered power-loss guarantee.
- The draft uses an in-root `FINISHING-<finish-handle>` evacuation witness so no
  ambiguous pre-commit state can resemble a fresh rootless grove.
- The one permitted in-session fresh reviewer found four substantive defects:
  an untracked generated finish leaf cannot be required in the deletion commit's
  parent; deleting a witness recursively before deleting the root destroys
  recovery state; absence of the exact commit does not prove the starting
  revision is unchanged; and hooks may mutate unrelated working-tree files that
  an index backup cannot restore.
- `post-teardown-restart-k99` owns restart after the exact deletion commit is
  established. This leaf owns validation, evacuation, repository outcome
  classification, rollback, and failure-safe handoff into that boundary.

## Done when

- The contract proves genuine Git/native-jj/colocated-jj teardown commits without
  assuming the generated finish leaf was previously committed.
- Commit classification positively identifies either the exact teardown result
  or an unchanged starting repository state; every other result stays fail
  closed.
- Rollback and forward cleanup each retain a recoverable witness through every
  interruption boundary, including the final atomic namespace transition.
- Hook policy matches the preservation claim rather than assuming index backup
  can reverse arbitrary hook side effects.
- `docs/specs/config-driven-sessions.md`, the minimum coherent ADR set,
  `CONTEXT.md`, `CONTEXT-MAP.md`, and citations describe one consistent design.
- A reviewed implementation chain is cut beneath `finish-failure-recovery-k100`.

## Notes

The in-session reviewer is already spent. This producer must be promoted to a
scheduled design-review chain before it is retired.

## In-session reviewer dispositions

- **Commit-parent finish leaf — valid, fixed.** The manifest records the live
  authorization plus a Git/jj repository anchor and expected tracked deletion
  fingerprint; proof never assumes the working-tree-only finish leaf was in the
  VCS parent.
- **Witness-destroying cleanup — valid, fixed.** After exact commit proof the
  whole task root moves atomically, witness intact, into collision-resistant
  post-commit cleanup quarantine before recursive disposal.
- **Unproven `Not committed` — valid, fixed.** Rollback additionally requires
  Git `HEAD` or jj working-copy topology to match the manifest's start anchor;
  any different revision remains `Recovery pending`.
- **Hook side effects — valid, fixed.** Internal plain-Git finish commits use an
  empty hooks path. Signing and injected repository failures still exercise the
  rollback seam without pretending an index image can restore arbitrary
  working-tree mutations.
