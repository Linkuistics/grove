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
