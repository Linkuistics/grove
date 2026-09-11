# node-grammar-k3

## Goal

Cut the `impl` leaves that deliver the node grammar `NN-k<key>/_<slug>.md`
(root `_BRIEF.md`) end to end — the library, grove's grammar and verbs, the
skills and docs, the books, the cutover and release, the migration — each a
vertical slice that lands green on its own, ordered so that this repo's own
tree is cut over in the same session as the release that can read it, and the
other groves are migrated after that release is installed.

## Context

- The design at `node-grammar-k2`, reviewed if that session cut a chain.
- The root brief's *Notes*: the meta-grove's sequencing hazard and the list
  of five groves to migrate.
- `plan-k1` decision 9 — the agreed test seams; every leaf names which of
  them it is tested through.
- `docs/specs/walkthrough-books.md`, *An accepted source change*: a source
  edit and its book's fragments and prose land in one commit with
  `scripts/check.sh` green. A leaf that touches a book's root owes that book
  its update in the same commit — no leaf may leave a book red for a sibling.
- `docs/RELEASING.md` — the release procedure. The cutover leaf ends with a
  human: stop the loop, cut and install the release, update the plugin, rename
  this tree, re-run `grove`.
- This is a canonical-grammar cutover with no dual-grammar reader (`plan-k1`
  decision 7), so it is not an expand → migrate → contract refactor: each leaf
  lands the new spelling for its own surface, and the tree that the loop
  reads changes only at the cutover leaf.

## Done when

- Every `impl` leaf exists with a body that names its surface, the seams it is
  tested through, and the book (if any) it owes.
- The positions encode the order: library → grove grammar and verbs → skills
  and docs → cutover and release (the last code leaf, HITL) → migration of
  the four other groves.
- The cutover leaf's body states the human sequence above and says that the
  session ends by stopping the loop rather than by `grove-llm complete` into
  a tree the running driver can no longer read.
- The migration leaf's body lists the five trees, the check (open each with
  the new binary), and what to do with `grove.gh-issue-12/.grove/FORMAT`.

## Notes

- No naming history in anything a leaf writes (`plan-k1` decision 6).
- A leaf that cannot be demoed on its own is a horizontal layer; redraw it.
