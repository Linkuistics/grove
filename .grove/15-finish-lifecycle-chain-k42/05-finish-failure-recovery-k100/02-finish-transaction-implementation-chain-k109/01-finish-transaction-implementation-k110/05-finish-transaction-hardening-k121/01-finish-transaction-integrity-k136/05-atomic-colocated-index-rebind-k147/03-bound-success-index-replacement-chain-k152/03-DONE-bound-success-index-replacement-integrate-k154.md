# bound-success-index-replacement-integrate-k154

**Kind:** integrate-review-impl
**Integrates:** bound-success-index-replacement-review-k153

## Goal

Apply the verified findings from `bound-success-index-replacement-review-k153` while preserving the reviewed artifact's contract.

## Context

- Integrates F1–F6. F1 took both dispositions the review asked for; F4 took the
  attempt-scoped name the review offered as its alternative disposition rather
  than a new reaping class.
- F1 and F5 share one cause: the replacement state document was the only
  authority for the two names it exchanged, and nothing bound either of them to
  a name Grove derives. The write side now refuses any replacement outside the
  deterministic replacement name, and the read side pins both
  `artifact_name` and `staged_artifact_name` exactly as `canonical_name` was
  already pinned.
- F1's second half is separable and survives a later relaxation of the name
  binding: the artifact exchange is the transaction's first mutation, and the
  marker-side classifier cannot run before it, because
  `validate_bound_artifact` expects the replacement inode to be at the canonical
  artifact name already. A weaker gate that *can* run early — the recorded
  marker pair parses, binds, and describes exactly the recorded exchange —
  restores "prove, then move".
- F2, F3 and F4 are all "what an abort leaves behind". The deterministic
  replacement name is the only one of the three that blocks a same-attempt
  retry, so it is the one that gained a reclamation path; the other two are
  bounded by the per-launch attempt identity.
- The marker-side machinery from `recoverable-marker-replacement-k149` is
  unchanged: a redirected `staged_name` still has to parse and bind as a Grove
  auxiliary marker and match a recorded snapshot, which is why the review
  located the hole on the artifact side only.

## Done when

- A replacement state document can no longer redirect the artifact exchange:
  binding refuses a non-deterministic replacement name and recovery refuses a
  state document whose artifact names are not the derived pair, both with the
  victim's inode and bytes intact.
- No artifact is exchanged before the recorded marker pair is proven to be
  Grove's own and to describe that exact exchange; the substituted-marker path
  refuses with both artifact inodes unchanged.
- A replacement copy left unowned by an interrupted publication is reclaimed by
  recovery, disposal and activation, so a same-attempt retry no longer collides
  and no unowned copy of the user's index survives. A symlink or non-regular
  file at that name still fails closed without touching its target.
- A failed index filter discards its own marked success auxiliary, leaving the
  same attempt available.
- The index-filter staging directory carries its finish attempt, so a leftover
  is attributable rather than anonymous.
- The diagnostic on a failed bound-artifact adoption no longer claims the
  replacement was left untouched on a path where an exchange may already have
  landed.
- `cargo fmt --check` and `cargo test --locked` pass from the final tree.

## Notes

Every fix is first demonstrated by a test that fails against the reviewed
producer commit; the six new auxiliary tests and the two new
`repo::finish_commit` tests were each run against that commit to confirm it.

`reclaim_unbound_replacement` unlinks a regular file it has not proven Grove
wrote, resting on the collision gate plus the attempt-scoped name. That is the
one deviation from `atomic-colocated-index-rebind-k147`'s no-deletion promise,
and the reachable counterexample is a substitution landing in
`replace_artifact_from`'s post-copy identity window. Rather than duplicate the
concern as a new leaf, it is recorded in `colocated-rebind-recovery-matrix-k151`,
whose `Done when` already owns substitution without deleting external bytes; that
leaf must either disprove the deletion or change the disposition. Reclamation was
also moved after each caller's own artifact validation, so nothing is unlinked
before the auxiliary proves its own integrity.

Diagnostics that were reachable after a mutation landed no longer claim anything
was left untouched; the strong claim survives only at the new pre-mutation gate,
where it holds.

F6 was pure signature churn and carries no test.

`docs/specs/config-driven-sessions.md` describes success-index preparation one
grain coarser than the auxiliary publication protocol, so nothing durable
changed; methodology and doc reconciliation stay with
`finish-transaction-docs-acceptance-k122`.
