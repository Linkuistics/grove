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

- The CLI's own shape, once there is a library for it to expose. Whether it
  belongs to increment 1's decomposition or its own is `library-k6`'s to decide.
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
- Splitting the crate into separately-modellable units.

## Settled since this brief was written

**The library does not inherit grove's Unix-only assumption in its
*interface*.** Locking is entirely internal, so the build is Unix-only today and
gaining another platform later changes no signature and no caller.

**The architecture has landed and is checked.** `architecture-k2` closed with
`docs/ordinal-fs-tree/ARCHITECTURE.md` reconciled against two models that both
run green. Three facts a later session needs and cannot get from the document
alone:

- **The models are re-runnable, and that is deliberate.** `models/run-alloy.sh`
  and `models/run-quint.sh` each report pass/fail per claim. Both exist because
  both tools report *found nothing* with exit code 0 — a model whose result
  cannot be read is not a checked model, and this cost was paid twice.
- **The models lead, so read their misses before trusting them.** Each records
  what it does *not* establish, in its own file and in `docs/formalism-findings.md`.
  Two matter most downstream: a rename carrying its subtree is assumed rather
  than checked, and walk *order* is unmodelled, so `by_key`'s tie-break on a
  duplicate-key tree rests on prose.
- **The ADR test was re-applied and two decisions now pass it** — the
  single-trait seam and no-removal. Filing them is `records-k5`, blocked only on
  which context maintains them.

**The operation set is fixed**, which is what the implementation leaves were
waiting for. `library-k6` cuts them.

**`ordinal-fs-tree` is a third bounded context, and the two records are filed.**
`records-k5` closed the question this brief had left on the horizon. The evidence
this brief offered for it was wrong and the real evidence is stronger: the
vocabularies do not merely differ, they **collide** — grove's `CONTEXT.md`
defines **Leaf** and **Node directory** in session terms while the library's
*leaf* and *node* are any regular file and any directory of children, and
`CONTEXT-FORMAT.md` forbids one term living in two glossaries. Three facts a
later session needs:

- **Placement and ownership were two questions, and only one was open.**
  `docs/adr/` stays flat, exactly as `ADR-FORMAT.md`'s split rule requires while
  grove occupies the repo root; the third context changes who *maintains* two
  records, not where any record lives.
- **The context is declared on vocabulary, not delivery.** It has a glossary at
  `docs/ordinal-fs-tree/CONTEXT.md` and no crate. `CONTEXT-MAP.md` now says a
  context is a language boundary rather than a shipping path, so `library-k6`'s
  answer about where the crate lands changes a path in that map and no ownership.
- **The rejected alternatives were unfiled and `.grove/` is deleted at the
  finish.** `docs/ordinal-fs-tree/ARCHITECTURE.md` names rejected shapes for the
  plan/interpreter split and for nothing else; the `Domain` trait, the two-trait
  split by layer and the non-derived key source lived only in
  `02-architecture-k2/BRIEF.md`. That, and the reversal cost, is what the two
  records carry that the document does not.
