# grove.gh-issue-13 — brief

## Goal

Extract grove's tree-on-disk facilities as a reusable, domain-independent Rust
library — `ordinal-fs-tree` — with a CLI in `grove-llm`'s shape, developed
against a formal model that leads the implementation. This is the first step in
deconstructing grove into composable units that can be modelled individually
(Linkuistics/grove issue #13).

**There are two experiments here, not one**, and they are equally the point. The
first is the extraction: does grove's tree machinery survive being made
domain-independent? The second is the method: can requirements and design be
captured in formal models rather than prose reviewed by eye — with the formalism
*chosen per question* rather than fixed in advance, and the implementation then
derived from the checked model. The second experiment's output is a
`linkuistics` skill, and it is a deliverable, not a by-product.

## Done when

- `ordinal-fs-tree` stands alone: an ordered tree of entries on disk, its name
  grammar parameterised by one trait, with a CLI that drives any conforming tree.
- Formal models cover the library — its structure and its operations — and the
  library's behaviour follows them rather than the reverse.
- The method is captured: `docs/formalism-findings.md` carries an entry per
  modelling episode, and a `linkuistics` skill is distilled from it covering
  which formalism suits which question and how a checked model drives an
  implementation.
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

**One formalism is not enough, and the split is deliberate.** Structural
questions — is this shape coherent, can it even represent what is needed — go to
**Alloy**, which finds counterexample *structures*. Behavioural questions — does
this operation preserve the invariant from any reachable state — go to **Quint**,
which finds counterexample *traces*. Both are applied to the *same* design, which
is what makes the comparison evidence about the tools rather than about two
unrelated problems. A single-tool experiment could only ever produce a skill
about that tool, never one about choosing between them.

**Findings accumulate; the skill distils.** The two experiments have opposite
production schedules — the library converges, the method accumulates — so they
must not share an artifact. Every session that reaches for a formalism appends an
entry to `docs/formalism-findings.md` before it retires, against a fixed six-field
format whose load-bearing field is the *counterfactual*: what would have caught
this earlier or more cheaply. A later leaf turns the log into the skill. Writing
the skill continuously would generalise from one data point; writing it only at
the end would find the evidence already gone.

**Prose survives alongside the models, with a demoted job.** A checked model
guarantees consistency with itself, never that the right properties were stated,
so review does not disappear — it relocates, from hundreds of lines of prose to
roughly fifteen invariant statements. And a `.qnt` or `.als` file is not
user-facing documentation. So the architecture document remains the explanation
and the models become the specification.

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

- Implementing the operations, each modelled first. Cannot be leafed until the
  design lands, because the operation set is what the design settles.
- The CLI's own shape, once there is a library for it to expose.
- The grove flip.
- **Distilling `docs/formalism-findings.md` into a `linkuistics` skill.** The
  question is precisely stateable already; its *scope* is not, because it depends
  on how many formalisms were used and what they taught. It also has to run last,
  after every modelling and implementation leaf, and an append lands it before
  work not yet cut. So it stays here until the modelling is done, then earns a
  leaf.
- Whether a checked model can be shown to drive an implementation, or whether
  that stays an article of faith. This is H3 in the findings log and the least
  certain of the three hypotheses; it needs a deliberate test, not an impression
  gathered in passing.
- Whether any of this earns an ADR. The design has now landed in draft, so the
  test is due for re-application at the `architecture-k2` close — in particular
  the single-trait seam, which was intent when the test was last run and is now a
  settled decision with a written rationale.
- Whether `ordinal-fs-tree` becomes a **third bounded context** with its own
  glossary. It has a deliberately separate vocabulary — entry, ordinal, key,
  distinguished child — that shares no term with grove's, which is exactly what
  `CONTEXT-MAP.md` uses to tell contexts apart. Not settled here; the question is
  recorded rather than answered.
- Splitting the crate into separately-modellable units.

Settled since this brief was written: the library does **not** inherit grove's
Unix-only assumption in its *interface* — locking is entirely internal, so the
build is Unix-only today and gaining another platform later changes no signature
and no caller.
