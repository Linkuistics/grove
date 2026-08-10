# reap-attributable-staging-leftovers-k161

**Kind:** impl

## Goal

Let a lease-owning driver reap the attributable staging entries a process death
leaves behind, without reintroducing a path that removes bytes Grove did not
write.

## Context

- Surfaced as R4 of `bound-replacement-staging-order-review-k157` and answered in
  part by `bound-replacement-staging-order-integrate-k158`: both staged entries
  are now named inside the auxiliary's reserved role-and-attempt namespace, so a
  death before the state document is durable leaves an entry whose owner is
  readable from its name. That satisfies this node's parseable-ownership
  requirement; it does not reap the entry.
- Two windows produce one: `replace_artifact_from_with` creates and fills the
  staged artifact, and `publish_marker_replacement` creates and writes the staged
  marker, each before `publish_state` makes a document that names it. Neither
  window is closable by reordering — a document cannot record an entry's inode
  before that entry exists — so reaping, not prevention, is the disposition.
- `auxiliary_marker_paths` matches only `GROVE-FINISH-AUXILIARY-*` names ending
  `.json`, so staging entries are invisible to `reap_orphaned`. They collide with
  nothing: `prepare_auxiliary` and `ensure_auxiliary_available` probe exact
  canonical names only, so a leftover blocks no later attempt.
- The hard part is the disposition, not the discovery. Unlinking a staging entry
  is exactly the shape this subtree removed with `reclaim_unbound_replacement`,
  so reaping needs an argument that the reaper's own preconditions — lease
  ownership, an invalidated previous epoch, and no matching in-tree witness —
  make the entry provably abandoned Grove state, or a disposition that does not
  unlink at all.
- The same acceptance already covers `finish-transaction-git-k118`'s
  `GROVE-FINISH-FILTER-<attempt>-*` staging directory; settle both here or say
  why they differ.
- `colocated-rebind-checkpoint-matrix-k159` turned both leftovers from conjecture
  into observed process behavior. A death at `before-state-publication` leaves
  the staged artifact, and a death at *any* rebind checkpoint also leaves the
  filter directory — its only disposal is a `TempDir` destructor that
  `process::exit` never runs. Its assertions currently permit exactly that
  residue, so tightening them is part of this leaf's acceptance.

## Done when

- A failing test first shows an attributable staging leftover surviving a
  lease-owned reap that has no matching in-tree witness.
- Reaping removes it, and a recorded argument or an enforced precondition
  establishes that what it removes is abandoned Grove state rather than an entry
  whose identity it never proved.
- A leftover belonging to a live witness's attempt, a foreign entry outside the
  reserved namespaces, and a symlink at a namespaced name are all left
  byte-identical.
- The reaper still never turns cleanup bytes into a finish receipt or a
  lifecycle input.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Fix only what a failing test demonstrates. If the honest answer is that these
entries must stay, say so in the spec and delete this leaf's reaping scope rather
than weakening the no-unproven-removal contract.
