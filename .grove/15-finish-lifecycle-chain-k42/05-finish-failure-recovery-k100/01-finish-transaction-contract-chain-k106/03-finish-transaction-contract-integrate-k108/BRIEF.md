# finish-transaction-contract-integrate-k108 — brief

**Kind:** integrate-review-design
**Integrates:** finish-transaction-contract-review-k107

## Goal

Apply the verified findings from `finish-transaction-contract-review-k107`
while preserving the reviewed artifact's contract.

## Context

- Verify every `finish-transaction-contract-review-k107` finding against the
  binding artifacts before changing them.
- Preserve explicit finish confirmation, rootless=fresh, artifact-only workflow
  state, manifest-anchored commit proof, positive unchanged-topology proof before
  rollback, hook-free internal Git commit, and atomic post-commit quarantine.
- This is design integration only. Implementation belongs to
  `finish-transaction-implementation-k110`.

## Done when

- Every review finding has a recorded disposition and each valid issue is fixed
  in the minimum coherent spec/ADR/glossary set.
- Git, native jj, and colocated jj share one transaction boundary without
  weakening unrelated-work preservation or the post-teardown restart contract.
- The implementation task remains accurate after all design changes.

## Decomposition

- `recovery-proof-hardening-k115`: address the narrow integration review's
  repository handoff race, exact jj start identity, launch-bound rootless proof,
  and symlink-root failure.
- `recovery-contract-finalize-k116`: re-read both reviews and reconcile the
  final disposition ledger, minimum coherent artifact set, and implementation
  handoff after hardening lands.

## Narrow integration review

The one permitted in-session reviewer found five substantive gaps after the
first integration pass: repository topology can change between a returned
disposition and its tree handoff; jj predicates need exact-result absence and an
exact preflight commit identity; the documented jj operator rollback target is
otherwise not identifiable; a reused handle can satisfy rootless proof after a
non-cooperating reset/removal unless proof is launch-bound; and `.grove/` itself
must be a descriptor-bound real directory, not a followed symlink. These are
owned by `recovery-proof-hardening-k115`; no second in-session reviewer remains.

## Final review dispositions

### Scheduled review

- **F1 — valid, fixed.** The spec's `Recovery pending` procedure and ADR
  `task-tree-transactions-fail-closed` name both operator-restorable topologies,
  require the diagnostic to show recorded and observed state, and prohibit Grove
  from rewriting divergent history.
- **F2 — valid contract omission, fixed.** The spec's pre-commit and plain-Git
  rules require the witness to remain untracked and absent from every candidate
  result. A broad commit that tracks it enters the F1 recovery procedure rather
  than satisfying success.
- **F3 — valid, fixed.** The native-jj contract records exact preflight commit
  `C0`, change `W`, and parents `P`; success is the attempt-bound parent of the
  successor, while rollback requires the current working-copy commit itself to
  remain `W` at `P` and to reproduce `C0`. Mere repository presence is
  insufficient.
- **F4 — valid, fixed after hardening.** `recovery-proof-hardening-k115` binds
  the manifest, deletion-commit message, and rootless retry to the active
  session epoch's launch nonce. The spec's crash/retry contract and ADR
  `one-live-driver-per-working-tree` therefore reject an older teardown with a
  reused handle, including after an external reset and task-root removal.
- **F5 — valid diagnostic/preflight gap, fixed.** The transaction requires a
  non-empty tracked deletion fingerprint before evacuation. A wholly untracked
  or ignored task tree is refused unchanged with instructions to record it;
  Grove never fabricates an empty finish commit.
- **F6 — safe contract, separate usability concern.** The reviewed transaction
  refuses a cross-device quarantine before mutation. Earlier workspace-layout
  validation remains live as `workspace-layout-preflight-k113` and is not part
  of this transaction contract.
- **F7 — valid repository-wide asymmetry, separate concern.** Finish's hook
  suppression is coherent within this transaction. Applying the same policy to
  the independent migration transaction remains live as
  `migration-hook-suppression-k114`.
- **F8 — valid, fixed.** The colocated-jj contract copies the user's Git index
  before any preflight snapshot/export, records the jj anchor afterward, and
  restores the pre-snapshot image before reporting an uncommitted result.
- **F9 — valid, fixed.** The spec, transaction ADR, and glossary all require
  descriptor-rooted no-follow quarantine disposal: symlinks are unlinked as
  entries and their targets are never traversed.
- **F10 — valid operational gap, fixed.** `finish-commit` owns immediate
  best-effort cleanup; a later lease-owning driver reaps validated orphaned
  auxiliaries and quarantines without consulting them as lifecycle receipts.
- **F11 — valid, fixed.** The manifest digest is a canonical no-follow Merkle
  SHA-256 over length-delimited path, type, mode, file-byte, symlink-target, and
  raw-name-ordered directory records; unsupported special entries are refused
  before mutation.

### Narrow integration review

- **H1 — valid, fixed.** The repository seam retains outcome context through
  the filesystem handoff and revalidates before and after rollback or quarantine
  rename. A changed result keeps or atomically restores the blocking witness.
- **H2 — contract ambiguity, fixed.** Jujutsu `Not committed` requires both the
  current `W`-at-`P` working-copy identity and absence of the exact attempt-bound
  teardown result, keeping it disjoint from `Committed` after `jj edit` or a
  rewrite.
- **H3 — valid, fixed.** The jj manifest and operator diagnostic name exact
  preflight commit `C0`; rollback must reproduce `C0` before witness removal.
- **H4 — valid, fixed.** The active launch nonce is the finish-attempt identity
  in the manifest, commit message, and rootless retry proof. A replacement epoch
  cannot accept a prior attempt even when the stable handle is reused.
- **H5 — valid, fixed.** Preflight opens `.grove/` no-follow, verifies that it is
  a real directory with the identity named by the locked working-tree root, and
  keeps transaction operations descriptor-relative.

## Artifact reconciliation

- `docs/specs/config-driven-sessions.md` remains the single detailed description
  of the Git, native-jj, and colocated-jj transaction and its observable test
  seams.
- ADR `task-tree-transactions-fail-closed` owns fail-closed tree mutation and
  recovery; ADR `one-live-driver-per-working-tree` owns the launch epoch and the
  narrow same-launch retry. Their overlap is the explicit handoff between those
  two decisions, not a duplicate transaction contract.
- `CONTEXT.md` gives the same backend-neutral contract in the `Complete finish
  cycle`, `Finish transaction`, `Session epoch`, and `Tree access lock` terms.
  `CONTEXT-MAP.md` assigns both ADRs and the spec to the Grove bounded context,
  with no missing or dangling record.
- `finish-transaction-implementation-k110` carries every final obligation:
  guarded repository handoff, exact jj start/result proof, attempt-bound retry,
  no-follow task-root identity, operator recovery, cleanup ownership, and the
  corresponding Git/native-jj/colocated-jj regressions.
