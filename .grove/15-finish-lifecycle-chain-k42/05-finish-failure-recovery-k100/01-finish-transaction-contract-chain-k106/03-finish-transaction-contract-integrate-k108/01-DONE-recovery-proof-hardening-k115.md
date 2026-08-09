# recovery-proof-hardening-k115

**Kind:** integrate-review-design

## Goal

Harden the finish transaction so repository classification stays true through
its tree handoff, jj recovery names exact revisions, rootless proof belongs to
this launch, and the task root cannot redirect mutation through a symlink.

## Context

- First child of `finish-transaction-contract-integrate-k108`, created when its
  one permitted narrow reviewer found five substantive gaps after the original
  eleven review findings were integrated.
- Binding artifacts: `docs/specs/config-driven-sessions.md` finish transaction,
  crash/retry, repository seam, and test sections; ADR
  `task-tree-transactions-fail-closed`; glossary terms Complete finish cycle and
  Finish transaction; implementation handoff
  `finish-transaction-implementation-k110`.
- Preserve explicit confirmation, artifact-only workflow state, rootless=fresh
  for later drivers, unrelated-work preservation, no automatic history rewrite,
  and process-interruption rather than power-loss guarantees.
- The integration leaf's reviewer allowance is spent. Apply and verify this
  bounded slice without another fresh reviewer.

## Done when

- Both rollback and forward cleanup revalidate repository topology before and
  after their filesystem handoff, retaining or atomically restoring the witness
  when a disposition changes before exposure.
- The jj manifest records exact preflight commit identity in addition to change
  and parents; `Not committed` requires exact teardown-result absence and the
  post-restore snapshot must reproduce the recorded start before witness
  removal.
- The exact deletion commit carries the active session epoch's opaque launch
  nonce as its finish-attempt identity, and rootless retry requires the same attempt so
  an older reused-handle teardown cannot match a new launch.
- Finish preflight opens and identity-revalidates `.grove/` itself as a real
  no-follow directory; a symlink or other non-directory root is refused before
  mutation and all transaction paths remain descriptor-relative.
- The spec, ADR, glossary, test seams, and implementation task state one
  coherent contract for Git, native jj, and colocated jj.

## Notes

Direct repository/filesystem mutations after the final guarded revalidation are
outside cooperating Grove guarantees; mutations observed at either gate become
`Recovery pending` rather than being acted on through a stale disposition.

## Narrow-review dispositions

- **H1 — valid, fixed.** Repository outcome context now spans the filesystem
  handoff with pre/post revalidation. Rollback retains the witness until the
  exact start is reproduced; forward cleanup can atomically restore quarantine
  to a blocking root when proof changes.
- **H2 — contract ambiguity, fixed.** `Not committed` requires the exact
  attempt-bound teardown result to be absent as well as the current jj working
  copy to retain the recorded change/parents. `jj edit` cannot satisfy both
  dispositions.
- **H3 — valid, fixed.** The jj anchor now includes exact preflight commit ID
  `C0`; ordinary rollback must reproduce it, and the operator procedure names it
  rather than an under-specified change ID.
- **H4 — valid, fixed.** The internal deletion commit and rootless retry carry
  the active launch nonce as an opaque identity. A new epoch cannot
  accept an older reused-handle attempt even after external reset/root removal.
- **H5 — valid, fixed.** Finish opens `.grove/` with no-follow semantics,
  identity-revalidates the descriptor against the locked root entry, rejects a
  symlink/non-directory before mutation, and keeps child operations
  descriptor-relative.
