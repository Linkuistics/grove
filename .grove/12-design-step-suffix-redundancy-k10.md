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

**This is an unrelated concern parked in this grove.** It has nothing to do with
mandate-delivered methodology; it is here because grove externalizes a surfaced
concern into the tree rather than absorbing it into the session that found it. It
is sequenced last so it preempts nothing.

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

### Surface, if removal is chosen

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

- The decision is made and recorded where it binds: `content/SKILL.md` and
  `content/TASK-FORMAT.md` for the convention, and an ADR **only if** the
  trade-off clears the when-to-write test — a naming convention with a stated
  reason may not, and a paragraph in the methodology may be the whole answer.
- If removal is chosen, `leaf-add-pair`'s generated slugs and every guidance test
  pinned to the old phrasing move with it, and the methodology says explicitly
  that both spellings remain legal so no existing tree is invalidated.
- If removal is rejected, the *surviving* argument is stated on its own terms and
  the suffix-vs-prefix argument stops standing in for it.

## Notes

- The human stated a position rather than asked a question. Treat it as the
  default and require the counter-argument to earn its keep; do not stage a
  balanced re-litigation of a call that has already been made.
- Watch for the third redundancy while you are here: `combine-research` is the
  kind and `-combine` is the suffix, but the *pair* also encodes `a`/`b` in both
  places. If the answer differs between the chain and the pair, say why —
  `leaf-add-pair` creating all three eagerly is the asymmetry that might justify
  it.
