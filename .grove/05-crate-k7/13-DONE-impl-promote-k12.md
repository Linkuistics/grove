# promote-k12

## Goal

`promote`: turn a leaf into a node, with the node's parts supplied by the
caller, moving the leaf's bytes verbatim into the new node's distinguished child
and preserving the entry's ordinal and key. Optionally creating a first child in
the same unit.

This is the operation with the most that can go wrong, and the only one by which
the library can damage a tree it was handed. It gets a leaf to itself for that
reason.

## Context

Beyond the brief chain:

- `ARCHITECTURE.md` — the `promote` row, *Promotion is not atomic against the
  invariants*, *When rollback fails*, the `promote` refusals, and the invariant
  *Identity preservation under promotion*.
- `operations.qnt` — the `no_distinguished` and `rollback_fails` instances.
  `rollback_fails` is the only instance that does not claim key uniqueness at
  rest, because this operation is what breaks it.
- `docs/formalism-findings.md` entry 003 — the transient duplicate, the
  failed-rollback duplicate key (reached in 0.07% of traces and needing its own
  sample budget), and the reissue restatement. All three are about this
  operation.

## Done when

- `promote` works: the node carries the promoted leaf's own ordinal and key, the
  leaf's bytes become the distinguished child's, and an optional first child
  lands in the same unit or not at all.
- Its refusals are implemented and tested: a node, and a distinguished child,
  are both refused; a domain whose `distinguished()` is `None` is refused
  outright rather than guessed at, because the leaf's content would have nowhere
  to go; and supplied parts that do not imply species `Node` are refused.
- The transient invariant break is documented where the caller meets it, not
  only in the architecture document. Between its two effects both the leaf and
  the node are on disk, sharing an ordinal and a key. There is no ordering that
  avoids it, and the invariants therefore hold of **quiescent** trees.
- The failed-rollback recovery is stated to the consumer in the crate's own
  error text or docs, mechanically: *a node and a leaf sharing an ordinal and a
  key, with the node holding no distinguished child, is an interrupted
  promotion; removing either half resolves it.* A library that can leave a tree
  in that state and does not say how to get out of it has told the consumer
  nothing useful.
- Each test names the model claim it discharges, or says it has none.
- An entry in `docs/formalism-findings.md`.

## Notes

**Identity preservation is about the entity, not the file.** The node is a new
directory and the leaf's own file survives inside it as the distinguished child:
the content keeps its identity, the container acquires a new one. A consumer
holding a path is stale either way; one holding a key is not, which is the whole
reason the key exists.

**The parts come from the caller for a reason the first draft got wrong.**
Species follows from parts, `Parts` is opaque with bounds `Clone + Eq`, and every
`Parts` value the library can reach belongs to some entry already in the tree —
so the library cannot name the promoted node. A trait method mapping a leaf's
parts to a node's would widen the seam to serve one operation and force every
domain to declare a canonical mapping when the honest one is often lossy. If
implementing this makes that trait method look attractive again, that is a
finding, and it argues with `docs/adr/entry-name-is-the-only-seam.md` rather
than around it.

## Decisions (running log)

**`promote` takes a bare `Key`, not a `Target`.** Every other mutation takes a
`Target` because its target is a *level* something goes into, and the tree root
is one. A promotion's target is an **entry** that has to be a leaf, and the root
is neither an entry nor a leaf — so `Target::Root` would name a call refused by
construction. `operations.qnt` splits them the same way, `TagPromote` carrying a
key where `TagInsert` carries a target, and `ARCHITECTURE.md`'s *Operations*
preamble was rewritten to say which operations take the root variant and why the
other two do not.

**The four refusals are transcribed in the model's own order.** `planPromote` is
a chain — missing, not a leaf, no distinguished child, parts not a node — and the
order is observable: a node in a domain with no distinguished child has two true
refusals and reports the first. Transcribed rather than reinvented, and covered
by a test that exists for no other reason.

**The document's `promote` refusal named a case no argument can reach, and the
bullet was corrected.** *A node is already a node, and a distinguished child has
no ordinal to carry across; both are refused* — but a target is named by key and
a distinguished child carries none, so `by_key` cannot answer with one and
neither can the model's `idsWithKey`. Not a model disagreement: the model
declines to support the second half **silently**, by having no witness for it.
`Refusal::PromoteNotLeaf` carries the species it actually found, the bullet now
says which case arrives, and `docs/formalism-findings.md` entry 014 carries the
generalisation.

**The transient invariant break is documented on `WriteGuard::promote`, where
the caller meets it.** `ARCHITECTURE.md` already carried it; a consumer reading
rustdoc would not. The operation's own doc states that between its two effects
both the leaf and the node are on disk sharing an ordinal and a key, that no
ordering avoids it, and that the invariants therefore hold of quiescent trees.

**The failed-rollback recovery was already in `Error::FailedPartiallyRolledBack`'s
`Display`, and what this leaf added is the test that holds it there.** Entry
013's habit — name the fact behind each clause of a message — becomes a test
shape: drive the state with the internal fault seam, assert each of the three
clauses **as a fact about the directory**, then follow the advice and assert the
tree reads cleanly again.

**The first child takes `freshKey`, not `freshKey + 1`.** The node carries the
promoted leaf's own key and allocates nothing, which is the whole content of
`inv_freshKeysAreFresh`'s comment about allocation versus creation. A promotion
that had spent a key on the node would leave a permanent hole in the key
sequence and break no invariant, because a skipped key is not a duplicate.
Mutation (d) is the only thing in the crate that fails on it.

**Six mutation controls, all caught**: the node taking a fresh key (10 tests),
the node taking the level's next ordinal (8), dropping the parts-imply-a-node
refusal (2), the child's key stepping past a key the node did not take (3),
never moving the leaf in (10), and reporting the domain refusal before the
not-a-leaf one (1).

**A review chain is cut, and inserted adjacent rather than appended.** This is
the operation with the most that can go wrong and the only path by which the
library damages a tree it was handed; `rewrite-k13` edits the same two files, so
a review appended at the node's end would reconcile a historical diff against a
tree that had moved.
