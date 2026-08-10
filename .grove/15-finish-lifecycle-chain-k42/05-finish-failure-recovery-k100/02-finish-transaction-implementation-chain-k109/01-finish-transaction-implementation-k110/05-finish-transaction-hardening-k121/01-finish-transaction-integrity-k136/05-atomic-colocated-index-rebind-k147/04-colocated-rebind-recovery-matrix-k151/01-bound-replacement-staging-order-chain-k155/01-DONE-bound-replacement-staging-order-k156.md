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

- A failing test first shows disposal unlinking a foreign entry at the
  derivable replacement name.
- The replacement is copied into an entry whose name is **drawn**, not derived —
  role, attempt and a fresh 128-bit nonce — and the state document records that
  name together with its inode before anything else can reach it. No name a
  reader can derive from the role and attempt is ever claimed.
- The artifact exchange runs against that drawn name directly, so the settled
  transition still leaves no staging entry behind.
- `reclaim_unbound_replacement` is gone, along with the derivable name it
  guarded and that name's collision-gate and recovery probes. Nothing unlinks an
  entry whose identity it has not proven, on any path.
- A forged state document cannot name a replacement outside this role and
  attempt's staging namespace.
- Unit tests cover the drawn-name publication, interruption on either side of the
  state document, a clean same-attempt retry after interruption, and a foreign
  regular file and symlink at the derivable name. `cargo fmt --check` and
  `cargo test --locked` pass.

## Notes

Keep the primitive generic to auxiliary markers. Do not widen the change into
the abort and disposal paths beyond what a failing test demonstrates; the
process matrices in this node's later leaves own those.

The design settled one step further than this leaf's Context anticipated. The
Context assumed the derivable name would survive and be claimed by a rename once
its owner was durable; exchanging against the drawn name instead removes the
derivable name from the protocol altogether, which is why the state machine and
its phases are unchanged.

Accepted cost: a process death between the staging copy and the state document
leaves one copy of the user's index under a role-and-attempt-named entry that
nothing reaps — the same acceptance `finish-transaction-git-k118`'s index-filter
staging directory already carries. It is attributable, blocks nothing, and is
never adopted or deleted. A returned error still unwinds its own staging entry.
