# chain-contiguity-k6

## Goal

Give the "is this chain's order load-bearing?" judgement call the test it
currently lacks, and narrow the blanket **"a chain is not contiguous by
construction, and that is accepted"** position where it is wrong: an
`integrate-review-*` step should run **immediately after** the review it
integrates, and departing from that needs an argument rather than a shrug.

## Context

Surfaced by the human while `clippy-baseline-k4` was being bootstrapped, from a
live instance in this grove's own tree:

```
02-DONE-impl-flat-lazy-review-k2.md                        producer
03-DONE-review-impl-flat-lazy-review-review-k3.md          its review
04-impl-clippy-baseline-k4.md                              unrelated, appended by k2
05-integrate-review-impl-flat-lazy-review-integrate-k5.md  the integration
```

**Nothing malfunctioned, and the trade is already documented** — that was
checked before this leaf was written, and it is why the leaf is scoped the way
it is. `flat-lazy-review-k2` externalized the clippy concern with `leaf-add`
(position 04, correct per Decompose). `flat-lazy-review-review-k3` then cut its
integration with `leaf-add` — exactly as the new methodology instructs — and
`leaf-add` appends at the next gapless position, which was 05. Both
`content/SKILL.md:332` and `CHANGELOG.md:99` already state that chains are not
contiguous and already name `leaf-insert`.

So this is **not** a missing-documentation leaf. The documentation is present,
and it is the documentation that is wrong — or rather, under-specified in a way
that reliably produces the wrong default.

### What is actually wrong

`content/SKILL.md:332-338` closes with *"Use `leaf-insert` when the order
genuinely matters."* That hands the session a judgement call and supplies **no
test for making it**. The first session ever to face that call — `k3` — reached
for plain `leaf-add` and split its own chain. A rule whose first application
fails is not a rule, it is a hope.

The missing test is concrete and mechanical: **a review's findings are anchored
to a commit and to line numbers.** This grove's review cites `src/tree_grow.rs:166`,
`src/tree_grow.rs:175`, `src/leaf.rs:258`, `src/leaf.rs:292`,
`docs/ARCHITECTURE.md:378`, `CHANGELOG.md:67`, and more. Every one of those
anchors is invalidated by an intervening edit to the same file, and the drift is
**silent** — nothing errors, the finding simply points somewhere slightly wrong
and the integrating session has to re-derive what the reviewer meant from a
codebase the reviewer never saw.

For the `review → integrate` hop, therefore, the order does not merely
"sometimes matter": it matters by default, and the exception is what must be
justified. That is a narrowing of the current blanket position, not a
contradiction of it — the flat-and-lazy decision itself stands, and the
`producer → review` hop is genuinely fine, because a producer cuts its review as
its own last act and nothing can intervene.

### The cost this grove actually paid

`clippy-baseline-k4` was allowed to keep its slot only because the interference
was checkable and near-nil: it and `flat-lazy-review-integrate-k5` target
disjoint files. With one exception that proves the point — `k4` had to log a
`CHANGELOG.md` entry while finding 6 cites `CHANGELOG.md:67`, so it had to place
its entry below that line *on purpose* and verify it by hand. Being obliged to
perform that check, and being lucky enough that it was performable, is exactly
the cost adjacency removes. It will not be checkable in general.

## Done when

- `content/SKILL.md:332-338` no longer leaves the call to unaided judgement. It
  keeps the flat-and-lazy design and the "grove validates no cross-leaf grammar"
  framing, but states the rule for the `review → integrate` hop and the reason
  (findings are commit- and line-anchored), and gives the exception its test:
  depart only when the intervening work provably touches no file the findings
  cite.
- The distinction between the two hops is explicit — `producer → review` cannot
  be split, `review → integrate` can and routinely will be — so the blanket
  claim is narrowed rather than merely softened.
- `leaf-insert` is presented as the **default** for cutting an integration when
  any leaf already holds the next slot, with plain `leaf-add` correct only when
  nothing intervenes.
- Every other statement of the accepted-cost position is reconciled to match,
  not just the one in `content/SKILL.md` — at minimum `CHANGELOG.md:99-103`
  ("that cost is accepted rather than defended against") and whatever
  `docs/specs/doubt-grove-review-mechanics.md` carries after
  `flat-lazy-review-integrate-k5` lands.
- If a guidance test pins chain-step instructions (finding 4 of the review
  touches this area), it covers the verb choice and the hop distinction too.

## Notes

Out of scope: reinstating the chain node, or any mechanism that enforces
contiguity. `flat-lazy-review-k2` decided flat and lazy and that decision is not
reopened here — this leaf changes what the methodology *tells a session to do*,
not what the verbs do. If the rule turns out to want enforcement rather than
guidance, that is a separate decision with its own ADR.

Whether this rises to an ADR is a judgement for the session: the decision
"integration is adjacent to its review by default" is arguably a trade worth
recording against the flat-and-lazy ADR set rather than only prose in
`content/SKILL.md`. Apply the when-to-write test rather than defaulting either
way.

Sequenced after `flat-lazy-review-integrate-k5` deliberately: that leaf edits
`docs/ARCHITECTURE.md`, `docs/specs/doubt-grove-review-mechanics.md` and the
`doubt-driven-development` skill, so the exact text this leaf must reconcile is
not settled until it lands. This ordering is itself an application of the rule
above — and unlike `k4`'s slot, it was chosen rather than inherited.
