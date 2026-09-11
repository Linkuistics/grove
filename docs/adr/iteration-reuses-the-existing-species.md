# Iteration reuses the existing species, and carries no mark

A repeated pass over one subject uses a series node whose children are its
passes. A pass taking one session is a leaf; a pass taking several sessions is
a node, made by `leaf-decompose`. Each node is `NN-k<key>/` with a node file
`_<slug>.md` carrying its brief. No naming token identifies a series.

Position order already visits passes in order and each pass's steps in order.
Terminal leaf names show which steps ran. The series' node-file body declares
the step sequence, exit condition, cap and action at the cap; no filename is
expected to carry those declarations.

Every pass and step uses the series' bare stem. A series that has repeated can
therefore appear as sibling node directories whose **node files share a slug**.
A recursive listing exposes that repetition without reading file contents.
This is an authoring convention, not an invariant: two steps of a review chain
could both decompose and produce the same repetition without being a series.
A series with only one pass has no repetition yet, so its brief carries the
intent a listing cannot establish.

## The trade-off

A marker would label intent before the second pass exists, at the cost of a new
grammar field and a second source for a fact already declared in the brief.
That cost does not buy ordering, routing or a stronger exit condition. The
existing node file reaches every session through the brief chain and carries
all of the series declarations together.

Rejected alternatives and their reopen conditions:

- **A marker in the directory or the slug.** A directory marker adds a field;
  a slug convention can misclassify a subject that naturally uses the same word.
  Reopen if a reader must identify series before a second pass exists without
  opening the brief, or repeated node-file slugs become an unreliable cue.
- **A second metadata file.** It splits the series' context and violates the
  exactly-one node-file rule. Reopen only if separate ownership of that context
  becomes a requirement.
- **A pass number in the slug.** Position already carries order, and insertion
  can make a copied number disagree. Nothing reopens that duplication.
- **An iteration outcome on a leaf.** Another pass is represented by another
  pass existing. Reopen only if the live/done/abandoned partition changes.

A later marker would require a reader to account for every correct unmarked
series before trusting its absence. That is why adding one is a change to the
meaning of the tree, not merely another visible hint.

[`pass-series`](../specs/pass-series.md) owns the declarations and their scope;
[a feedback edge is forward tree growth](a-feedback-edge-is-forward-tree-growth.md)
owns work handed forward inside one pass.
