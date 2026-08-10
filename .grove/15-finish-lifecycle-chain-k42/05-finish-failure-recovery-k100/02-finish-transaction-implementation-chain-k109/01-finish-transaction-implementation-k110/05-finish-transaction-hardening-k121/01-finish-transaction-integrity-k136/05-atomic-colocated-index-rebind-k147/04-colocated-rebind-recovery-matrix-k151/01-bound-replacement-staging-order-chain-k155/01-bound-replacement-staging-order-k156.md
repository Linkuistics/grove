# bound-replacement-staging-order-k156

**Kind:** impl

## Goal

Publish the replacement's owning state document before the deterministic
replacement name exists, so Grove never creates an entry nothing describes and
nothing has to delete an entry it did not prove it wrote.

## Context

- `replace_artifact_from` currently creates the deterministic replacement name
  first and publishes the `.replacing` state document second. Everything wrong
  here follows from that order: the window leaves an unowned copy, the unowned
  copy blocks the same-attempt collision gate, and `reclaim_unbound_replacement`
  exists only to unblock it by unlinking whatever regular file sits at that name.
- A substitution inside the post-copy identity check makes that entry foreign,
  so the reclamation deletes external bytes on a path whose own contract
  forbids it.
- Refusing instead of unlinking is not available: the attempt identity is the
  per-launch signal nonce, so a second `finish-commit` in one launch reuses it.
- The inverted order is recoverable because the temporary staging name is drawn
  from the existing reserved random-nonce namespace and its identity is recorded
  in the state document before the deterministic name is claimed, so recovery
  proves ownership by inode rather than by reasoning about a name.

## Done when

- A failing test first shows the reachable substitution deleting a foreign entry
  at the deterministic replacement name.
- The replacement is copied into a reserved randomly named staging entry, its
  identity is recorded in the published state document, and only then is the
  deterministic replacement name claimed by a non-replacing rename.
- Recovery settles the pre-claim state forward by proving the recorded staging
  identity, and refuses to move or unlink anything at either name that does not
  match the recorded identity.
- `reclaim_unbound_replacement` is gone; a foreign entry at the deterministic
  replacement name fails closed with an actionable diagnostic and is left
  byte-identical, inode intact.
- A forged state document cannot name a staging entry outside the reserved
  namespace.
- Unit tests cover the new pre-claim boundary, its interruption, and the foreign
  entry at both names. `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Keep the primitive generic to auxiliary markers. Do not widen the change into
the abort and disposal paths beyond what a failing test demonstrates; the
process matrices in this node's later leaves own those.
