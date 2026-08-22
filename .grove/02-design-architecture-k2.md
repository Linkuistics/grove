# architecture-k2

## Goal

Settle the architecture of `ordinal-fs-tree` **interactively with the human**,
and deliver it as **standalone user-facing documentation with diagrams** —
readable on its own terms by someone who has never heard of grove, and free of
task handles, decision-record citations, and grove's domain vocabulary.

This is a design session, not an implementation one. Nothing is built here.

## Context

The root brief carries the settled decisions and the vocabulary; this task adds
only what is specific to designing against them.

**Run it interactively.** The human asked for the architecture to be worked out
as a conversation, not presented as a finished proposal. Put design questions one
at a time and wait, the way the requirements session did. This kind is usually
driven alone; here it is not.

**Read before designing.** The current implementation is the reference for
*behaviour*, not for structure: `tree_id` (the name grammar and its three
orthogonal parts), `tree_read` (walk, ancestor chain, reference resolution),
`tree_grow` (append, insert-with-shift), `tree_lifecycle` (promotion, attribute
rewrite), `tree_rename` (the version-control-aware move), `tree_access` (the lock
guards). Migration is out of scope — do not read it as a requirement.

**The trait is the hard part.** All genericity in one trait's associated types,
with the entry name a type wrapping a string that owns its own parsing,
validation and formatting. The open design question is what else the trait must
carry once the surface includes locking and reserved names, and whether that
stays honest to "all genericity in associated types" or quietly grows into the
callbacks the human ruled out.

**Settle the operation set.** Implementation leaves cannot be cut until this
session names the operations, because the operation set is what the Quint model
is written against, one operation at a time.

**Quint leads.** The model is authoritative over the working implementation. This
session does not write the model, but the design has to be *shaped so it can be*
— explicit state, total operations, invariants that can be stated. If part of the
design resists being modelled, that is a design finding, not a modelling problem
to defer.

## Done when

- A standalone architecture document exists, with diagrams, explaining what
  `ordinal-fs-tree` is, the model it implements, its trait seam, and its
  operations — comprehensible without grove as background. Not in `docs/adr/`;
  where it does live is this session's call with the human.
- The trait is specified: its associated types, what the name type is responsible
  for, and how reserved names and lock scope are expressed through types rather
  than hooks.
- The operation set is named and described behaviourally, ready for one Quint
  model per operation.
- The invariants the model will state are written down in prose, at least: key
  uniqueness, ordinal gaplessness, subtree preservation under sibling shift, and
  no task-shaped name ever silently skipped.
- The human agrees the architecture is right. This is an interactive session; it
  ends on agreement, not on a document being finished.

## Notes

Where the design departs from the current implementation's structure, say so and
why — that is signal for the flip increment later, not noise.

Anything that turns out to need its own session gets a leaf rather than being
absorbed here.
