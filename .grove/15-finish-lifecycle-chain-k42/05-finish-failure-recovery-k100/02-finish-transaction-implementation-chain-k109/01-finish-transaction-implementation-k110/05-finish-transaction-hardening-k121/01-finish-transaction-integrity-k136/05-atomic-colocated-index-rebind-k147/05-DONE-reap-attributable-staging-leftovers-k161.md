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

## Outcome — the reaping scope is deleted, and the windows narrowed instead

The Notes' alternative is the one that holds. The disposition this leaf was cut
to find does not exist, for a reason internal to the subtree rather than to a
threat model: **Grove's own substitution refusal deliberately produces the state
a namespace sweep would have to destroy.** When `replace_artifact_from`'s
post-copy identity check finds an inode it can no longer identify it declines to
unlink it and reports, leaving a *foreign* regular file at a shape-valid staged
name. An abandoned staged copy is byte-for-byte the same shape. Nothing on disk
separates them, so "attributable" and "provably Grove's" are different claims and
only the first is true of these names.

That also answers the leaf's framing of the precondition argument. Lease
ownership, an invalidated previous epoch, and no matching in-tree witness prove
the entry *abandoned* — no live Grove process can still be publishing it. They
prove nothing about *authorship*, which is the half the removed
`reclaim_unbound_replacement` also lacked. What changed since that removal is only
the forcing function: a drawn name is a collision gate for no later attempt, so
nothing now compels the removal. Doing it anyway would trade a harmless leak for
the exact capability three leaves were spent eliminating.

So: no sweep, for either namespace — the two `*.staging-<nonce>` entries and the
index filter's `GROVE-FINISH-FILTER-<attempt>-*` directory settle the same way,
and Done-when 1 and 2 are answered by the Notes' alternative rather than met.
Recorded in the spec ("Auxiliary Git-index backups or success images"), on ADR
`task-tree-transactions-fail-closed` as a rejected option with its reopen
condition — an entry whose inode can be recorded before it has a name, a portable
`O_TMPFILE`-equivalent — and in the glossary's Finish transaction term.

What is delivered instead is prevention where prevention is available. The filter
staging directory leaked at *every* rebind checkpoint, not just one window,
because its only disposal was a `TempDir` destructor and every boundary there
exists to end the process — an RAII owner is a liveness guarantee, and this
transaction can only rely on ordering. `replace_artifact_from` now takes a
`release_source` callback and runs it once the replacement holds its own staged
copy and before the first publication boundary; the colocated caller releases the
directory there. A failure to release fails the attempt closed, naming the
directory, rather than publishing over a silent leak. Done-when 3 and 4 hold as
written, and the k159 checkpoint matrix is tightened from permitting that residue
to refusing it.

Two things a successor should know. The reaper is untouched, so the three
"left byte-identical" cases are not three branches — the reaper never inspects a
staging name at all, and `reaping_leaves_every_staging_leftover_it_cannot_prove_it_wrote`
locks that as one fact rather than pretending to exercise branches that do not
exist. And a narrow window remains before the release point (the staged copy and
the `git update-index` child); it is the accepted bounded leak, not an oversight.
