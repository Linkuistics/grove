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

## Review dispositions

- **F1 — valid, fixed.** `Recovery pending` now has a documented operator exit:
  preserve divergent work, restore either the exact manifest start or exact
  teardown result, and retry. Diagnostics expose both recorded and observed
  topology; Grove never guesses by rewriting history or editing the witness.
- **F2 — valid contract omission, fixed.** The witness is explicitly required
  to remain untracked and absent from every candidate result. A direct broad
  commit that tracks it is `Recovery pending`, not success; the F1 procedure
  restores a provable topology before any retry can commit or roll back.
- **F3 — valid, fixed.** Jujutsu proof now models partial `jj commit`: the
  recorded working-copy change becomes the exact teardown parent of a new
  successor, while `Not committed` requires that the current working-copy
  commit itself still be the recorded change at the recorded parents. Repository
  presence alone is insufficient.
- **F4 — reopened by narrow review.** Root presence excludes an older commit in
  the cooperating path, but an external reset plus root removal can recreate the
  old rootless proof. `recovery-proof-hardening-k115` binds the deletion commit
  and retry proof to the current launch attempt.
- **F5 — valid diagnostic/preflight gap, fixed.** A non-empty tracked deletion
  fingerprint is now a pre-evacuation requirement. A wholly untracked task tree,
  including one ignored before it was recorded, is refused unchanged with
  instructions to record it; Grove does not fabricate an empty finish commit.
- **F6 — safe contract, separate usability concern.** Finish already refuses a
  cross-device quarantine before mutation. Deciding how much earlier to reject
  an unsupported workspace layout is externalized as
  `workspace-layout-preflight-k113`.
- **F7 — valid repository-wide asymmetry, separate concern.** Migration is not
  part of this finish transaction. Applying the same hook suppression to its
  internal Git commit is externalized as `migration-hook-suppression-k114`.
- **F8 — valid, fixed.** Colocated jj now copies the user's Git index before any
  preflight snapshot/export, records the jj anchor afterward, and restores the
  pre-snapshot image on an uncommitted result.
- **F9 — valid, fixed.** Quarantine disposal is descriptor-rooted and no-follow;
  a symlink is unlinked as an entry and its target is never traversed.
- **F10 — valid operational gap, fixed.** `finish-commit` owns immediate
  best-effort cleanup and later lease-owning drivers reap orphaned internal
  auxiliaries/quarantines without consulting them as lifecycle receipts.
- **F11 — valid, fixed.** Manifest digests are canonical no-follow Merkle
  SHA-256 digests over length-delimited ordered child records, raw paths, entry
  types, mode bits, regular-file bytes, and symlink-target bytes; unsupported
  special entry types are refused before mutation.
