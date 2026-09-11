# The entry name is the only seam

Every point at which `ordinal-fs-tree` is parameterised by a consumer belongs
to `EntryName`. The name type owns parsing, formatting, species and validation
of distinguished names at a level. There is no separate `Domain` object,
registration mechanism or filesystem callback. The library owns locking,
traversal and the mutation algebra; the consumer owns vocabulary.

Positioned names decompose into ordinal, key and opaque parts. Distinguished
names are supplied as values: promotion takes one destination name, and root
initialization takes an optional name-and-bytes pair. The library does not
extract a label or invent a distinguished name from node parts.

`validate_distinguished` receives the containing node name, or root, and the
complete distinguished-name set at that level. The domain can require a file
and distinguish its root marker from other names. The method is a deterministic
judgment over names, not a callback into I/O. The reader invokes it before
exposing entries; the planner invokes it over projected final levels before
effects. Independently, the library rejects more than one distinguished child.
A domain can issue its own grammar error for competing names before that generic
check, so strict consumers' diagnostics carry their canonical form.

`EntryNameExt` is blanket-implemented and sealed. It derives the positioned
triple, species and name identity, so a consumer cannot override the comparisons
the algebra depends on. Positioned identity compares both view and species:
`Parts: Eq` may compare a leaf's parts equal to a node's even when their species
differ. Distinguished identity compares canonical rendered filenames; the unit
`Distinguished` view classifies a name and carries no unique identity.

## The trade-off

The algebra stores names and triples and never interprets text. Canonical
renderings are observed only for distinguished-name equality, path-component
validation, filesystem paths and diagnostics. The models can represent these
names as opaque atoms: no string grammar, label parser or content model crosses
the seam. Byte-level canonicity and rendering stability remain obligations of
the consumer and are sampled by the conformance kit.

A distinct associated type for distinguished-name payloads could provide their
equality, but adds another consumer type and another equality law when canonical
rendering already supplies the required identity. Comparing only the unit view
would instead merge different filenames. Reopen the extra type if rendering for
this comparison is measurably too expensive or cannot remain deterministic.

Level validation must see a containing name and all its distinguished names:
per-entry parsing alone cannot detect a missing file or judge root-only
placement. Keeping that judgment on `EntryName` avoids a second domain seam.
The library still owns the enforcement points, including bare node creation
through append or insert. A consumer unable to omit its own file must use an
operation that creates it in the same plan.

A sibling shift remains `compose(new_ordinal, key, parts)`. A renamed node
carries its subtree, including its own file, without copying a label into parts.
The filesystem layer checks every rendered name is one path component before
joining it to the tree; grammar canonicity alone cannot prevent a path escape.

Name and level errors carry the consumer's error value with the relevant path.
Algebraic refusals stay in the library's vocabulary. Consumers impose their own
stronger task preconditions before invoking the algebra; level grammar is
validated inside the shared store so every reader and writer meets it.

## Considered options

- **A `Domain` trait carrying lock and move callbacks.** Rejected because the
  containing-directory lock and filesystem moves have one meaning for all
  consumers. Reopen if a real consumer needs a different lock scope.
- **A second validation layer outside the store.** Rejected because another
  read or mutation path could bypass the same grammar rule. Nothing reopens it
  for a rule governing a tree the store exposes.
- **Copy a label into node parts to manufacture the distinguished name.**
  Rejected because a name parsed from a slugless directory cannot supply that
  value. Hydrating it from another entry would break the positioned-name
  isomorphism and give one title two owners. Nothing reopens the duplication.
- **Parse names inside the library from a grammar description.** Rejected
  because vocabulary and recovery advice would move into a layer with no domain
  words. Nothing reopens that alternative.

The trait is a public contract for every consumer, and the models and
conformance kit are stated in its terms. Changing it therefore changes more
than a caller signature. The [architecture](../ordinal-fs-tree/ARCHITECTURE.md)
owns the detailed obligations; this record owns the placement of the seam.
