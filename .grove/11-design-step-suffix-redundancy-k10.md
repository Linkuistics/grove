# step-suffix-redundancy-k10

## Goal

Settle whether the **step suffix on a composed shape's stem** should go, given
that the filename already carries the session kind as a canonical component:
`<stem>-review` / `<stem>-integrate` beside a `review-…` / `integrate-review-…`
kind, and `<stem>-a` / `<stem>-b` / `<stem>-combine` beside the `research-a`,
`research-b` and `combine-research` kinds.

## Context

Raised by the human during `increments-k4`, verbatim:

> I see that tasks not only have the kind as an initial name component, but also
> as a last component before the id. This is redundant — the prefix is canonical.

**This is an unrelated concern carried in this grove.** It has nothing to do with
mandate-delivered methodology; it is here because grove externalizes a surfaced
concern into the tree rather than absorbing it into the session that found it.

**It is sequenced *before* `classification-k9`, deliberately, and that is a
correction.** `increments-k4` parked it last "so it preempts nothing", claiming
that kept the classification's review chain contiguous with it.
`increments-review-k11` B6 disproved that on two counts.

The contiguity argument was never available: a review leaf is cut lazily by its
producer and lands at the parent's next free position, so when
`classification-k9` was a flat leaf its `review-impl` would have landed *after*
this one. (It is now a node, and its aggregate review lives inside it — which
settles the contiguity question by construction, and leaves position order as the
only thing left to decide.)

What position order has to decide is the corpus. The surface below is
`content/SKILL.md` and `content/TASK-FORMAT.md`: 76,418 of the 139,136 bytes
`classification-k9` classifies. Editing that prose *after* the classification
would leave real units marked over text that then moved, and would leave the
classification reviewer reading a corpus that had since changed. So the whole
concern — this decision **and** any implementation it produces — completes before
the real classification begins.

The redundancy is real and this tree shows it:

```
03-DONE-review-design-mandate-delivery-review-k3.md
04-DONE-integrate-review-design-mandate-delivery-integrate-k5.md
```

The stated position is that the prefix is canonical, so start from removal and
argue *against* it rather than for it. Two arguments in the current methodology
have to be answered, and only one of them is actually about this question.

**Answer this one.** `content/SKILL.md` says: *"Every step keeps the stem so bare
slugs stay unique and surviving commit handles still name their artifact once
`.grove/` is gone."* Two distinct claims, and they do not fall together:

- *Bare-slug uniqueness.* Without the suffix, `grove-llm resolve mandate-delivery`
  matches the producer, its review and its integration. That is not a break —
  `resolve` already answers an ambiguous bare slug by listing each match's key so
  the caller re-queries by key — but it does convert a one-step lookup into two
  for every chain, which is the common case the shape exists to make habitual.
- *Surviving commit handles.* This is the sharper one, because it outlives the
  tree. A commit message names its work item `<slug>-k<key>`, and `.grove/` is
  deleted at the finish cycle. Today `git log` shows `mandate-delivery-k2`,
  `mandate-delivery-review-k3` and `mandate-delivery-integrate-k5`, and the roles
  are legible from the handle alone. Drop the suffix and the historical record
  keeps three commits whose handles differ only by an opaque key, with nothing
  left on disk to say which was the review. Weigh whether the commit *subject*
  already carries that — this repository's convention prefixes it (`review:`,
  `integrate:`, `impl:`, `design:`) — and if it does, say so, because that would
  answer the claim rather than merely trade against it.

**Do not be answered by this one.** The same section says a terminal suffix
*"keeps stem-mates together in a directory listing; `review-<stem>` would sort
every review beside every other review and scatter the chains the naming exists
to reveal."* That is an argument for **suffix over prefix on the stem**, decided
against a different alternative. It says nothing about dropping the marker
entirely, and it must not be allowed to look like a defence of the status quo.

Note also what is *not* at stake: the five-field filename grammar is unchanged
either way. The step suffix is a **convention**, not grammar — grove parses no
relationship between leaves, a `-review` suffix does not make a leaf review its
neighbour, and nothing validates it. So this is a change to guidance plus one
generator, and **no tree migration is implied**: existing leaves keep the slugs
they were created with, and both spellings remain legal filenames.

### This leaf decides; a separate `impl` leaf executes

`increments-review-k11` B6 also caught the kind: this was cut as `design` while
its Done-when required production and test changes to `src/tree_grow.rs` and the
guidance tests. A `design` session's deliverable is a decision — a spec, an ADR
set, or both — and a `design` session that finds itself editing the artifact is
doing an `impl` leaf's work (`content/TASK-FORMAT.md`, *design*, *impl*).

So **write no `content/` prose, no `src/` change and no test edit here.** Decide,
record the reasoning, and hand every resulting edit to an `impl` leaf you cut as
your last act — with the decision and the exact edit list written into its body
verbatim, which is the whole payoff of cutting a step late. That holds in **both**
outcomes: if removal is rejected, the surviving argument still has to be
disentangled from the suffix-vs-prefix one in `content/SKILL.md`, and that prose
repair is execution too.

If the decision genuinely produces no edit anywhere, cut nothing and say so.

**Cut it with `leaf-insert`, not `leaf-add`** — the implementation must also land
before the real classification, and `leaf-add` would append it *after* the
classification node:

```
grove-llm leaf-insert 9 <stem>-<something> --kind impl
```

Key `9` is the `classification-k9` **node directory**, which is the first sibling
entry after this leaf whose subtree still holds live work. Target the node, never
a leaf inside it — that would insert one level down, inside a node whose brief
does not charter this concern.

Whether the decision clears the ADR when-to-write test is a judgement this leaf
makes. If it does not, the decision plus its reasoning living in the `impl` leaf's
body is the whole record, and that is a legitimate outcome — but then the *durable*
form of it is the methodology prose that leaf writes, so make sure the prose says
why and not just what.

### Surface, if removal is chosen

Hand this list to the `impl` leaf; do not work it here.

- `content/SKILL.md` — the *Cut the next step* section: the stem-suffix rule and
  its two arguments.
- `content/TASK-FORMAT.md` — the worked chain and pair examples, and the naming
  discussion around them.
- `src/tree_grow.rs` — `leaf-add-pair` **generates** `<stem>-a`, `<stem>-b`,
  `<stem>-combine`. It is the one place the convention is mechanized, so it is the
  one place the decision has a compile-time consequence. The review chain has no
  generator: each step is a hand-written `leaf-add --kind`, so only guidance
  changes there.
- `CONTEXT.md` — the [[Review chain]] / vendor pair entry.
- Guidance tests that assert the current phrasing — check
  `tests/composition_guidance.rs`, `tests/composition_verbs.rs` and
  `tests/session_kind_guidance.rs` before editing prose, since several pin exact
  wording.

## Done when

- The decision is made, with the bare-slug-uniqueness and surviving-commit-handle
  claims each answered on its own terms, and the suffix-vs-prefix argument
  explicitly barred from standing in for either.
- An ADR exists **only if** the trade-off clears the when-to-write test — a naming
  convention with a stated reason may not, and a paragraph in the methodology may
  be the whole answer.
- No `content/`, `src/`, `CONTEXT.md` or test file is edited by this session.
- Unless the decision produces no edit at all, an `impl` leaf exists, cut with
  `leaf-insert` at the `classification-k9` node so it runs before the real
  classification, carrying verbatim: the decision, its reasoning, and the exact
  edit list from *Surface* above. Its own contract must include —
  - if removal is chosen, that `leaf-add-pair`'s generated slugs and every
    guidance test pinned to the old phrasing move with it, and that the
    methodology says explicitly that both spellings remain legal so no existing
    tree is invalidated;
  - if removal is rejected, that the surviving argument is restated on its own
    terms in the prose.

## Notes

- The human stated a position rather than asked a question. Treat it as the
  default and require the counter-argument to earn its keep; do not stage a
  balanced re-litigation of a call that has already been made.
- Watch for the third redundancy while you are here: `combine-research` is the
  kind and `-combine` is the suffix, but the *pair* also encodes `a`/`b` in both
  places. If the answer differs between the chain and the pair, say why —
  `leaf-add-pair` creating all three eagerly is the asymmetry that might justify
  it.
