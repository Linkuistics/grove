# grove.gh-issue-13 — brief

## Goal

Extract grove's tree-on-disk facilities as a reusable, domain-independent Rust
library — `ordinal-fs-tree` — with a CLI in `grove-llm`'s shape, developed
against a Quint model that leads the implementation. This is the first step in
deconstructing grove into composable units that can be modelled individually
(Linkuistics/grove issue #13).

## Done when

- `ordinal-fs-tree` stands alone: an ordered tree of entries on disk, its name
  grammar parameterised by one trait, with a CLI that drives any conforming tree.
- A Quint model covers the library's operations, and the library's behaviour
  follows the model rather than the reverse.
- grove is flipped onto the library: its tree modules are gone, grove supplies a
  domain impl, and trees in flight are unaffected.

## The model, in the library's own vocabulary

grove's own words for these things are grove's *domain vocabulary* and must not
appear in the library.

- An **entry** is a **leaf** (a regular file) or a **node** (a directory of
  children).
- Every entry's name encodes an **ordinal** — its per-level position, mutable,
  and the sole sort input within one level; a **permanent key** — assigned once,
  unique tree-wide, never rewritten; a **label**; and some **domain attributes**.
- A node may hold one distinguished child that is the node's own content rather
  than one of its children.

## Decomposition

Two increments, in this order:

1. **The library and its CLI, standing alone.** No grove changes. A design step
   comes first, because the architecture is the thing under experiment and has to
   be settled interactively before anything is built.
2. **The flip.** grove's tree modules deleted, replaced by a domain impl.

The drift window between them is accepted deliberately: the two increments fail
in different ways and are cheaper to debug apart.

## Decisions settled

**One crate for now.** Splitting into separately-modellable units is a later
workstream, deferred to keep this step small. The algebra stays free of
`std::fs` so that split remains mechanical, and a test enforces it rather than
convention — inside one crate, a seam the compiler does not enforce is a seam
nothing measures.

**All genericity in one trait's associated types.** The entry name is a **type
wrapping a string** that owns its own parsing, validation and formatting;
reserved names are variants of that type. No domain callbacks or hooks anywhere.

**Surface: everything grove's tree modules do except migration** — the name
grammar, the tree algebra (walk, resolve, ancestor chain, append,
insert-with-sibling-shift, leaf-to-node promotion, attribute rewrite), its
filesystem realisation, the version-control-aware move primitive, and the shared
and exclusive lock guards. Migration is out entirely.

**The model leads.** Quint, written per operation, before that operation is
implemented. Where the model and a test disagree, the model wins and the test
changes. The working implementation stops being the reference — which makes every
disagreement between the two a finding, and the catalogue of those findings a
deliverable in its own right rather than a by-product.

**The flip is a pure refactor.** No on-disk name changes, so trees in flight need
no migration by construction.

## Pointers

Established by reading the current implementation; the extraction must preserve
each of these:

- The lock is `flock` on the working tree's directory — the *parent* of the tree
  root, not the root. Lock scope is therefore itself a domain decision.
- Paths are deliberately never canonicalised: locking follows inode identity
  through the descriptor, output preserves the caller's spelling. On macOS `/var`
  and `/private/var` name the same inode, so canonicalising would make the mere
  presence of a lock observably rewrite every path the read verbs return.
- Reserved-name refusal carries recovery advice, not just detection. A domain
  contributes both, or the library's errors are useless to whoever hits them.
- A task-shaped name a walk silently skips is lost work — and a whole subtree is
  lost when the skipped name is a directory. That is why the grammar refuses
  unknown attribute tokens rather than ignoring them.
- `libc::flock` is Unix-only. grove may assume that; a library published for
  reuse has to decide whether it inherits the assumption.

Existing coverage that adapts — roughly 130 tests, overwhelmingly CLI-contract
tests through `assert_cmd` rather than unit tests: `leaf`, `session_kind_tree`,
`composition_verbs`, `leaf_ops`, `kind`, `jj_tree_verbs`, `resolve`, `pick`,
`root_init`, `brief_chain`, `tree_access`. Out of scope: everything driver,
methodology, session-configuration and migration related. Note that
`lifecycle_invariants` concerns methodology-corpus invariants despite its name,
and has nothing to do with tree invariants.

## On the horizon

- Implementing the operations, each with its Quint model first. Cannot be leafed
  until the design lands, because the operation set is what the design settles.
- The CLI's own shape, once there is a library for it to expose.
- The grove flip.
- Whether any of this earns an ADR. Nothing has yet: the candidate decisions each
  fail the *hard to reverse* limb of the three-part test, and the trait design is
  intent rather than a landed decision until the design step settles it. Re-apply
  the test when the design lands.
- Whether the library inherits grove's Unix-only assumption.
- Splitting the crate into separately-modellable units.
